//! R2 review probe (PR 6345): cone apex against a vertex, every op both
//! orders, against an independent radial-integral oracle. Prints a table;
//! asserts nothing but "no panic" counts at the end.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, dead_code)]

use crate::common::wrap_tip::{NORTH, one_wrap_edge};
use core::f64::consts::PI;
use geom_core::{Affine3, Band, Point2, Point3, Tol, Vec3};
use std::panic::{AssertUnwindSafe, catch_unwind};
use sweep::Revolution;
use sweep::test_support::{finished, revolved_about_y};
use topo::{AtRestBody, BooleanResult, SolidContainment, intersect, point_in_solid, subtract, union};

fn tol() -> Tol {
    Tol::witness()
}

#[derive(Clone, Debug)]
enum Shape {
    /// apex, unit nappe axis, half-angle, height.
    Cone(Point3<f64>, Vec3<f64>, f64, f64),
    /// half-spaces (p - o)·n <= c, n unit, o the apex.
    Planes(Point3<f64>, Vec<(Vec3<f64>, f64)>),
    /// apex o on the sphere, centre c, radius r.
    Sphere(Point3<f64>, Point3<f64>, f64),
}

impl Shape {
    fn reach(&self, d: Vec3<f64>) -> f64 {
        match self {
            Shape::Cone(_, a, al, h) => {
                let c = d.dot(*a);
                if c > al.cos() { h / c } else { 0.0 }
            }
            Shape::Planes(_, ps) => {
                let mut r = f64::INFINITY;
                for &(n, c) in ps {
                    let dn = d.dot(n);
                    if c.abs() < 1e-12 {
                        if dn > 0.0 {
                            return 0.0;
                        }
                    } else if dn > 0.0 {
                        r = r.min(c / dn);
                    }
                }
                r
            }
            Shape::Sphere(o, c, _) => (2.0 * d.dot(*c - *o)).max(0.0),
        }
    }
    fn margin(&self, p: Point3<f64>) -> f64 {
        match self {
            Shape::Cone(o, a, al, h) => {
                let v = p - *o;
                let z = v.dot(*a);
                let rho = (v - *a * z).norm();
                (z * al.sin() - rho * al.cos()).min(h - z)
            }
            Shape::Planes(o, ps) => {
                let v = p - *o;
                ps.iter().map(|&(n, c)| c - v.dot(n)).fold(f64::INFINITY, f64::min)
            }
            Shape::Sphere(_, c, r) => r - (p - *c).norm(),
        }
    }
}

/// ∫ over cone A's cap of ⅓ min(rA, rB)³ dΩ.
fn shared(a: &Shape, b: &Shape, n: usize) -> f64 {
    let Shape::Cone(_, ax, al, _) = a else { panic!() };
    let (ax, al) = (*ax, *al);
    let u = if ax.x.abs() < 0.9 { Vec3::new(1.0, 0.0, 0.0) } else { Vec3::new(0.0, 1.0, 0.0) };
    let u = (u - ax * u.dot(ax)).normalize();
    let v = ax.cross(u);
    let (dt, dp) = (al / n as f64, 2.0 * PI / n as f64);
    let mut sum = 0.0;
    for i in 0..n {
        let t = (i as f64 + 0.5) * dt;
        let (st, ct) = t.sin_cos();
        for j in 0..n {
            let p = (j as f64 + 0.5) * dp;
            let d = ax * ct + (u * p.cos() + v * p.sin()) * st;
            let r = a.reach(d).min(b.reach(d));
            if r > 0.0 {
                sum += r.powi(3) / 3.0 * st * dt * dp;
            }
        }
    }
    sum
}

fn cone_body(alpha: f64, tipped: bool) -> topo::Body<f64> {
    if tipped {
        let r = alpha.to_radians().tan();
        one_wrap_edge(
            revolved_about_y(
                vec![
                    (Point2::new(0.0, 0.0), 0.0),
                    (Point2::new(r, 0.0), 0.0),
                    (Point2::new(0.0, 1.0), 0.0),
                ],
                Revolution::Full,
                tol(),
            ),
            true,
        )
        .0
    } else {
        crate::revolve_common::needle(alpha, None, tol()).into_body()
    }
}

fn moved(body: &topo::Body<f64>, m: Affine3<f64>) -> AtRestBody<f64> {
    finished(
        "moved",
        topo::transform_rigid(body, &geom_core::test_support::rigid(&m, tol()), tol()).unwrap(),
        tol(),
    )
}

fn cone_shape(alpha: f64, m: Affine3<f64>) -> Shape {
    Shape::Cone(
        NORTH,
        m.transform_vec(Vec3::new(0.0, -1.0, 0.0)).normalize(),
        alpha.to_radians(),
        1.0,
    )
}

fn cone_vol(alpha: f64) -> f64 {
    PI * alpha.to_radians().tan().powi(2) / 3.0
}

fn name(e: &impl core::fmt::Debug) -> String {
    let s = format!("{e:?}");
    s.split(|c| c == ' ' || c == '{' || c == '(').next().unwrap().to_string()
}

#[derive(Default)]
struct Tally {
    ok: usize,
    bad: usize,
    refused: usize,
    panics: usize,
    lines: Vec<String>,
}

/// Runs six ops; `a` is the cone (oracle cap), `b` anything.
fn six(
    t: &mut Tally,
    what: &str,
    (a, sa, va): (&AtRestBody<f64>, &Shape, f64),
    (b, sb, vb): (&AtRestBody<f64>, &Shape, f64),
    lattice: (Point3<f64>, Point3<f64>),
) {
    let i = shared(sa, sb, 1200);
    let ra = |p| sa.margin(p);
    let rb = |p| sb.margin(p);
    type R = Box<dyn Fn(Point3<f64>) -> f64>;
    let (sa1, sb1) = (sa.clone(), sb.clone());
    let both: R = Box::new(move |p| sa1.margin(p).min(sb1.margin(p)));
    let (sa2, sb2) = (sa.clone(), sb.clone());
    let either: R = Box::new(move |p| sa2.margin(p).max(sb2.margin(p)));
    let (sa3, sb3) = (sa.clone(), sb.clone());
    let aless: R = Box::new(move |p| sa3.margin(p).min(-sb3.margin(p)));
    let (sa4, sb4) = (sa.clone(), sb.clone());
    let bless: R = Box::new(move |p| sb4.margin(p).min(-sa4.margin(p)));
    let _ = (ra, rb);
    let cases: Vec<(&str, Box<dyn Fn() -> Result<BooleanResult<f64>, topo::BooleanError>>, f64, &R)> = vec![
        ("a∩b", Box::new(|| intersect(a, b, tol())), i, &both),
        ("b∩a", Box::new(|| intersect(b, a, tol())), i, &both),
        ("a∪b", Box::new(|| union(a, b, tol())), va + vb - i, &either),
        ("b∪a", Box::new(|| union(b, a, tol())), va + vb - i, &either),
        ("a∖b", Box::new(|| subtract(a, b, tol())), va - i, &aless),
        ("b∖a", Box::new(|| subtract(b, a, tol())), vb - i, &bless),
    ];
    let band = Band::linear(tol()).unwrap();
    for (op, f, want, region) in cases {
        let res = catch_unwind(AssertUnwindSafe(|| {
            match f() {
                Ok(BooleanResult::Body(body)) => {
                    let mp = topo::test_support::pinned_volume("v", &body.body, tol());
                    let slack = mp.volume_pad + 4e-3 * va.max(vb).min(8.0);
                    let mut msg = String::new();
                    if (mp.volume - want).abs() > slack {
                        msg += &format!(" VOL {} want {want:.6} pad {:e}", mp.volume, mp.volume_pad);
                    }
                    if let Err(e) = topo::validate_geometric(&body.body, tol()) {
                        msg += &format!(" TIER3 {}", name(&e));
                    }
                    let (lo, hi) = lattice;
                    let n = 6;
                    let span = hi - lo;
                    let mut wrong = 0;
                    let mut errs = 0;
                    for ii in 0..n {
                        for jj in 0..n {
                            for kk in 0..n {
                                let at = |q: usize, o: f64, s: f64, w: f64| s + w * (q as f64 + o) / (n as f64 - 0.63);
                                let p = Point3::new(at(ii, 0.13, lo.x, span.x), at(jj, 0.29, lo.y, span.y), at(kk, 0.41, lo.z, span.z));
                                let s = region(p);
                                if s.abs() < 1e-2 {
                                    continue;
                                }
                                match point_in_solid(&body.body, p, band, tol()) {
                                    Ok(c) => {
                                        let want = if s > 0.0 { SolidContainment::In } else { SolidContainment::Out };
                                        if c != want {
                                            wrong += 1;
                                        }
                                    }
                                    Err(_) => errs += 1,
                                }
                            }
                        }
                    }
                    if wrong > 0 {
                        msg += &format!(" LATTICE {wrong} wrong");
                    }
                    if errs > 0 {
                        msg += &format!(" (pis errs {errs})");
                    }
                    if msg.is_empty() || msg.starts_with(" (") {
                        (0, format!("ok {:.5}/{want:.5}{msg}", mp.volume))
                    } else {
                        (1, format!("BAD{msg}"))
                    }
                }
                Ok(BooleanResult::Empty) => {
                    if want.abs() < 1e-4 {
                        (0, "ok empty".into())
                    } else {
                        (1, format!("BAD empty want {want}"))
                    }
                }
                Err(e) => (2, format!("refuse {}", name(&e))),
            }
        }));
        let (k, msg) = match res {
            Ok(x) => x,
            Err(p) => {
                let s = p
                    .downcast_ref::<String>()
                    .cloned()
                    .or_else(|| p.downcast_ref::<&str>().map(|s| s.to_string()))
                    .unwrap_or_default();
                (3, format!("PANIC {}", s.lines().next().unwrap_or("")))
            }
        };
        match k {
            0 => t.ok += 1,
            1 => t.bad += 1,
            2 => t.refused += 1,
            _ => t.panics += 1,
        }
        t.lines.push(format!("{what} | {op} | {msg}"));
    }
}

fn report(name: &str, t: &Tally) {
    let eps = tol().eps();
    let mut out = format!("== {name} eps {eps:e}: ok {} bad {} refused {} panic {}\n", t.ok, t.bad, t.refused, t.panics);
    for l in &t.lines {
        out += l;
        out += "\n";
    }
    let dir = std::env::var("R2_OUT").unwrap_or_else(|_| "/tmp".into());
    std::fs::write(format!("{dir}/{name}-{eps:e}.txt"), &out).unwrap();
    println!("{out}");
}

fn cone_pairs(pairs: &[(f64, f64)], tag: &str) {
    let mut t = Tally::default();
    let z = Vec3::new(0.0, 0.0, 1.0);
    let skew = Vec3::new(0.6, 0.3, 0.74).normalize();
    for &(a1, a2) in pairs {
        let s12 = (a1 + a2).to_radians();
        let d12 = (a1 - a2).abs().to_radians();
        let mut taus: Vec<(Vec3<f64>, f64, &str)> = vec![
            (z, 0.0, "coax"),
            (z, 0.02, "near-coax"),
            (z, 0.5 * s12, "cross-mid"),
            (z, s12 - 0.01, "near-touch-in"),
            (z, s12 + 0.01, "near-touch-out"),
            (z, PI - 0.3, "opp-tilt"),
            (z, PI, "hourglass"),
            (skew, 0.6 * s12, "skew-cross"),
            (skew, PI - 0.1, "skew-opp"),
        ];
        if d12 > 0.1 {
            taus.push((z, 0.5 * d12, "nested-tilt"));
            taus.push((skew, 0.5 * d12, "skew-nested"));
        }
        for (form_a, form_b) in [(true, true), (false, false), (true, false)] {
            let ca = finished("a", cone_body(a1, form_a), tol());
            let cb0 = cone_body(a2, form_b);
            let sa = cone_shape(a1, Affine3::identity());
            for &(axis, tau, label) in &taus {
                let m = Affine3::rotation_about_axis(NORTH, axis, tau);
                let cb = moved(&cb0, m);
                let sb = cone_shape(a2, m);
                let reach = a1.to_radians().tan().max(a2.to_radians().tan()) + 1.2;
                six(
                    &mut t,
                    &format!("{a1}/{a2} {label} τ={tau:.3} forms {form_a}/{form_b}"),
                    (&ca, &sa, cone_vol(a1)),
                    (&cb, &sb, cone_vol(a2)),
                    (
                        Point3::new(-reach, -0.3, -reach),
                        Point3::new(reach, 2.3, reach),
                    ),
                );
            }
        }
    }
    report(tag, &t);
}

#[test]
fn r2_cone_pairs_equal() {
    cone_pairs(&[(10.0, 10.0), (30.0, 30.0), (45.0, 45.0), (60.0, 60.0), (80.0, 80.0)], "cones-equal");
}

#[test]
fn r2_cone_pairs_unequal() {
    cone_pairs(&[(10.0, 80.0), (80.0, 10.0), (30.0, 60.0), (45.0, 30.0), (20.0, 50.0)], "cones-unequal");
}

/// The box `[0,l]³` in its own frame, corner at the origin, moved so its
/// corner sits at NORTH, turned by `rot` about the corner.
fn boxes(tag: &str, alphas: &[f64]) {
    let mut t = Tally::default();
    let rots: Vec<(Vec3<f64>, f64, &str)> = vec![
        (Vec3::new(0.0, 1.0, 0.0), 0.0, "rest"),
        (Vec3::new(0.0, 1.0, 0.0), 0.4, "y0.4"),
        (Vec3::new(1.0, 0.0, 0.0), 0.3, "x0.3"),
        (Vec3::new(1.0, 0.0, 1.0), -0.6155, "diag-down"), // a body diagonal along -y
        (Vec3::new(1.0, 0.0, 1.0), -0.3, "diag-part"),
        (Vec3::new(0.3, 0.8, -0.5), 1.1, "skew1"),
        (Vec3::new(-0.7, 0.2, 0.4), 2.3, "skew2"),
        (Vec3::new(1.0, 0.0, 0.0), PI / 2.0, "x90 (box up)"),
        (Vec3::new(1.0, 0.0, 0.0), -PI / 2.0 + 0.2, "x-70"),
    ];
    for &alpha in alphas {
        for tipped in [true, false] {
            let c = finished("c", cone_body(alpha, tipped), tol());
            let sc = cone_shape(alpha, Affine3::identity());
            for &l in &[0.3f64, 2.0, 5.0] {
                // box occupying x∈[0,l], y∈[-l,0], z∈[0,l] about the origin.
                let block = topo::test_support::brick((0.0, l), (-l, 0.0), (0.0, l), tol());
                for &(axis, ang, label) in &rots {
                    let m = Affine3::translation(Vec3::new(0.0, 1.0, 0.0))
                        * Affine3::rotation_about_axis(Point3::origin(), axis, ang);
                    let bx = moved(&block, m);
                    let ns = [
                        (Vec3::new(-1.0, 0.0, 0.0), 0.0),
                        (Vec3::new(1.0, 0.0, 0.0), l),
                        (Vec3::new(0.0, 1.0, 0.0), 0.0),
                        (Vec3::new(0.0, -1.0, 0.0), l),
                        (Vec3::new(0.0, 0.0, -1.0), 0.0),
                        (Vec3::new(0.0, 0.0, 1.0), l),
                    ]
                    .map(|(n, c)| (m.transform_vec(n).normalize(), c));
                    let sb = Shape::Planes(NORTH, ns.to_vec());
                    let reach = alpha.to_radians().tan().max(l * 1.8) + 0.3;
                    six(
                        &mut t,
                        &format!("{alpha}° tipped={tipped} box {l} {label}"),
                        (&c, &sc, cone_vol(alpha)),
                        (&bx, &sb, l.powi(3)),
                        (
                            Point3::new(-reach, 1.0 - reach, -reach),
                            Point3::new(reach, 1.0 + reach, reach),
                        ),
                    );
                }
            }
        }
    }
    report(tag, &t);
}

#[test]
fn r2_box_corners_a() {
    boxes("boxes-a", &[30.0, 45.0]);
}

#[test]
fn r2_box_corners_b() {
    boxes("boxes-b", &[10.0, 60.0, 80.0]);
}

/// A square pyramid with its tip at NORTH, axis along `m`·(−y), side
/// half-width `w` at depth `h`.
#[test]
fn r2_pyramid_tips() {
    use topo::test_support::meeting::{Pose, posed_pyramid};
    let mut t = Tally::default();
    for &alpha in &[30.0f64, 45.0, 60.0] {
        for tipped in [true, false] {
            let c = finished("c", cone_body(alpha, tipped), tol());
            let sc = cone_shape(alpha, Affine3::identity());
            for &(w, h) in &[(0.2f64, 1.0f64), (1.0, 0.5), (3.0, 1.0), (0.6, 0.6)] {
                for &(axis, ang, label) in &[
                    (Vec3::new(0.0, 0.0, 1.0), 0.0, "coax"),
                    (Vec3::new(0.0, 1.0, 0.0), 0.3, "turn0.3"),
                    (Vec3::new(0.0, 0.0, 1.0), 0.4, "tilt0.4"),
                    (Vec3::new(1.0, 0.2, 0.5), 1.0, "skew1.0"),
                    (Vec3::new(0.0, 0.0, 1.0), PI, "opposite"),
                ] {
                    let m = Affine3::rotation_about_axis(NORTH, axis, ang);
                    let loc = [[w, -h, w], [-w, -h, w], [-w, -h, -w], [w, -h, -w]];
                    let world = |q: [f64; 3]| {
                        let p = m.transform_point(Point3::new(q[0], q[1] + 1.0, q[2]));
                        [p.x, p.y, p.z]
                    };
                    let mut base: Vec<[f64; 3]> = loc.iter().map(|&q| world(q)).collect();
                    let tip = [0.0, 1.0, 0.0];
                    let pose = Pose::rest();
                    let pyr = match catch_unwind(AssertUnwindSafe(|| posed_pyramid(&base, tip, &pose, tol()))) {
                        Ok(p) => p,
                        Err(_) => {
                            base.reverse();
                            posed_pyramid(&base, tip, &pose, tol())
                        }
                    };
                    // half-spaces: four side faces through the tip, base plane.
                    let o = NORTH;
                    let mut ps = Vec::new();
                    let bp: Vec<Point3<f64>> = base.iter().map(|q| Point3::new(q[0], q[1], q[2])).collect();
                    let centroid = bp.iter().fold(Vec3::new(0.0, 0.0, 0.0), |s, p| s + (*p - o)) * 0.25;
                    for k in 0..4 {
                        let (p1, p2) = (bp[k] - o, bp[(k + 1) % 4] - o);
                        let mut n = p1.cross(p2).normalize();
                        if n.dot(centroid) > 0.0 {
                            n = -n;
                        }
                        ps.push((n, 0.0));
                    }
                    let mut nb = m.transform_vec(Vec3::new(0.0, -1.0, 0.0)).normalize();
                    let cb = (bp[0] - o).dot(nb);
                    if cb < 0.0 {
                        nb = -nb;
                    }
                    ps.push((nb, cb.abs()));
                    let sp = Shape::Planes(o, ps);
                    let vol = (2.0 * w).powi(2) * h / 3.0;
                    let reach = alpha.to_radians().tan().max(w * 1.5).max(h) + 0.6;
                    six(
                        &mut t,
                        &format!("{alpha}° tipped={tipped} pyr w{w} h{h} {label}"),
                        (&c, &sc, cone_vol(alpha)),
                        (&pyr, &sp, vol),
                        (
                            Point3::new(-reach, 1.0 - reach, -reach),
                            Point3::new(reach, 1.0 + reach, reach),
                        ),
                    );
                }
            }
        }
    }
    report("pyramids", &t);
}

#[test]
fn r2_sphere_poles() {
    let mut t = Tally::default();
    for &alpha in &[30.0f64, 45.0, 60.0] {
        for tipped in [true, false] {
            let c = finished("c", cone_body(alpha, tipped), tol());
            let sc = cone_shape(alpha, Affine3::identity());
            for &r in &[0.2f64, 0.6, 1.5] {
                for &(u, label) in &[
                    (Vec3::new(0.0, 1.0, 0.0), "above"),
                    (Vec3::new(0.0, -1.0, 0.0), "below"),
                    (Vec3::new(0.5, -1.0, 0.0), "below-tilt"),
                    (Vec3::new(1.0, -0.2, 0.3), "side"),
                    (Vec3::new(1.0, 0.4, 0.0), "side-up"),
                ] {
                    let u = u.normalize();
                    let centre = NORTH + u * r;
                    let ball = finished(
                        "ball",
                        sweep::test_support::ball_poled(r, centre - Point3::origin(), u, tol()),
                        tol(),
                    );
                    let ss = Shape::Sphere(NORTH, centre, r);
                    let reach = alpha.to_radians().tan().max(2.0 * r) + 0.4;
                    six(
                        &mut t,
                        &format!("{alpha}° tipped={tipped} sphere r{r} {label}"),
                        (&c, &sc, cone_vol(alpha)),
                        (&ball, &ss, 4.0 / 3.0 * PI * r.powi(3)),
                        (
                            Point3::new(-reach, 1.0 - reach, -reach),
                            Point3::new(reach, 1.0 + reach, reach),
                        ),
                    );
                }
            }
        }
    }
    report("spheres", &t);
}

fn box_at(l: f64, axis: Vec3<f64>, ang: f64) -> AtRestBody<f64> {
    let block = topo::test_support::brick((0.0, l), (-l, 0.0), (0.0, l), tol());
    let m = Affine3::translation(Vec3::new(0.0, 1.0, 0.0))
        * Affine3::rotation_about_axis(Point3::origin(), axis, ang);
    moved(&block, m)
}

#[test]
fn r2_detail() {
    let show = |what: &str, r: Result<BooleanResult<f64>, topo::BooleanError>| match r {
        Ok(BooleanResult::Body(b)) => println!("{what}: body {}", topo::test_support::pinned_volume("v", &b.body, tol()).volume),
        Ok(BooleanResult::Empty) => println!("{what}: empty"),
        Err(e) => println!("{what}: {e:?}"),
    };
    let x = Vec3::new(1.0, 0.0, 0.0);
    let c80 = finished("c", cone_body(80.0, true), tol());
    let b5 = box_at(5.0, x, PI / 2.0);
    show("80 box5 x90 b∩a", intersect(&b5, &c80, tol()));
    show("80 box5 x90 a∩b", intersect(&c80, &b5, tol()));
    let c30 = finished("c", cone_body(30.0, true), tol());
    let b03 = box_at(0.3, x, PI / 2.0);
    show("30 box0.3 x90 a∪b", union(&c30, &b03, tol()));
    show("30 box0.3 x90 b∪a", union(&b03, &c30, tol()));
    let c10 = finished("c", cone_body(10.0, true), tol());
    let skew = Vec3::new(0.6, 0.3, 0.74).normalize();
    let m = Affine3::rotation_about_axis(NORTH, skew, 0.6 * 20f64.to_radians());
    let cb = moved(&cone_body(10.0, true), m);
    show("10/10 skew-cross a∩b", intersect(&c10, &cb, tol()));
    let c60 = finished("c", cone_body(60.0, true), tol());
    let m = Affine3::rotation_about_axis(NORTH, Vec3::new(0.0, 0.0, 1.0), 60f64.to_radians());
    let cb = moved(&cone_body(60.0, true), m);
    show("60/60 cross-mid a∩b", intersect(&c60, &cb, tol()));
    show("60/60 cross-mid b∩a", intersect(&cb, &c60, tol()));
    let ball = finished("ball", sweep::test_support::ball_poled(0.6, Vec3::new(0.0, 1.6, 0.0), Vec3::new(0.0, 1.0, 0.0), tol()), tol());
    let eps = tol().eps();
    let block = topo::test_support::brick((0.0, 2.0), (-2.0, 0.0), (0.0, 2.0), tol());
    let bx = moved(&block, Affine3::translation(Vec3::new(0.0, 1.0 - 0.3 * eps, 0.0)));
    show("30 box 0.3eps down b∩a", intersect(&bx, &c30, tol()));
    show("30 box 0.3eps down a∩b", intersect(&c30, &bx, tol()));
    show("45 sphere above", intersect(&finished("c", cone_body(45.0, true), tol()), &ball, tol()));
}

/// Claim 4: a box corner within / just outside the band of the apex, and
/// a box edge running through the apex.
#[test]
fn r2_near_apex_ends() {
    let mut t = Tally::default();
    let eps = tol().eps();
    for &alpha in &[30.0f64, 45.0] {
        for tipped in [true, false] {
            let c = finished("c", cone_body(alpha, tipped), tol());
            let sc = cone_shape(alpha, Affine3::identity());
            let mut offs: Vec<(Vec3<f64>, String)> = Vec::new();
            for k in [0.3f64, 0.9, 3.0, 30.0] {
                for (d, dl) in [
                    (Vec3::new(0.0, -1.0, 0.0), "down"),
                    (Vec3::new(0.0, 1.0, 0.0), "up"),
                    (Vec3::new(1.0, 0.0, 1.0).normalize(), "side-in"),
                    (Vec3::new(-1.0, 0.0, -1.0).normalize(), "side-out"),
                ] {
                    offs.push((d * (k * eps), format!("{k}ε {dl}")));
                }
            }
            offs.push((Vec3::new(-0.2, 0.0, 0.0), "edge through apex x".into()));
            offs.push((Vec3::new(-0.2, 0.0, -0.2), "face through apex".into()));
            offs.push((Vec3::new(0.0, 0.2, 0.0), "y-edge through apex".into()));
            for (rot_label, axis, ang) in [("rest", Vec3::new(0.0, 1.0, 0.0), 0.0), ("skew1", Vec3::new(0.3, 0.8, -0.5), 1.1)] {
                let rot = Affine3::rotation_about_axis(Point3::origin(), axis, ang);
                for (off, label) in &offs {
                    let l = 2.0;
                    let block = topo::test_support::brick((0.0, l), (-l, 0.0), (0.0, l), tol());
                    let m = Affine3::translation(Vec3::new(0.0, 1.0, 0.0) + rot.transform_vec(*off)) * rot;
                    let bx = moved(&block, m);
                    let corner_off = rot.transform_vec(*off);
                    let ns = [
                        (Vec3::new(-1.0, 0.0, 0.0), 0.0),
                        (Vec3::new(1.0, 0.0, 0.0), l),
                        (Vec3::new(0.0, 1.0, 0.0), 0.0),
                        (Vec3::new(0.0, -1.0, 0.0), l),
                        (Vec3::new(0.0, 0.0, -1.0), 0.0),
                        (Vec3::new(0.0, 0.0, 1.0), l),
                    ]
                    .map(|(n, c)| {
                        let n = rot.transform_vec(n).normalize();
                        let c = c + corner_off.dot(n);
                        (n, if c.abs() < 1e-6 { 0.0 } else { c })
                    });
                    let sb = Shape::Planes(NORTH, ns.to_vec());
                    six(
                        &mut t,
                        &format!("{alpha}° tipped={tipped} {rot_label} {label}"),
                        (&c, &sc, cone_vol(alpha)),
                        (&bx, &sb, l.powi(3)),
                        (Point3::new(-2.5, -1.25, -2.5), Point3::new(2.5, 1.25 + 2.0, 2.5)),
                    );
                }
            }
        }
    }
    report("near-apex", &t);
}
