//! Reviewer probes for PR #4044 (lane `reach-dual4044-r1`): the meridian
//! cut-in of a trimmed sphere face, widened beyond the PR's own poses.
//! Oracles are independent: closed-form cap/ball volumes, and an
//! analytic signed-distance membership against `point_in_solid`.
//! The survey rows PRINT a table and never assert per pose; the
//! assertions are the final tallies (a wrong body or a panic fails).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;
use std::panic::{AssertUnwindSafe, catch_unwind};

use geom_core::{Affine3, Band, Point2, Point3, Tol, Vec3};
use sweep::Revolution;
use sweep::test_support::{finished, revolved_about_y};
use topo::{AtRestBody, Body, BooleanOp, SolidContainment};

/// A signed function, positive inside.
type F = Box<dyn Fn(Point3<f64>) -> f64>;
/// A union of intersections of half-spaces/balls.
struct Shape(Vec<Vec<F>>);
impl Shape {
    fn eval(&self, p: Point3<f64>) -> f64 {
        self.0
            .iter()
            .map(|part| part.iter().map(|f| f(p)).fold(f64::INFINITY, f64::min))
            .fold(f64::NEG_INFINITY, f64::max)
    }
}
fn ball_f(c: Point3<f64>, r: f64) -> F {
    Box::new(move |p| r - (p - c).norm())
}
fn half_f(n: Vec3<f64>, s: f64) -> F {
    // n·p ≥ s inside
    Box::new(move |p| n.dot(p - Point3::origin()) - s)
}
fn box_fs(lo: [f64; 3], hi: [f64; 3]) -> Vec<F> {
    let mut v: Vec<F> = Vec::new();
    for i in 0..3 {
        let mut e = [0.0; 3];
        e[i] = 1.0;
        let n = Vec3::new(e[0], e[1], e[2]);
        v.push(half_f(n, lo[i]));
        v.push(half_f(-n, -hi[i]));
    }
    v
}

fn ball(r: f64, y: f64) -> AtRestBody<f64> {
    let b = revolved_about_y(
        vec![(Point2::new(0.0, y - r), 1.0), (Point2::new(0.0, y + r), 0.0)],
        Revolution::Full,
        Tol::witness(),
    );
    finished("ball", b, Tol::witness())
}
fn brick(lo: [f64; 3], hi: [f64; 3]) -> AtRestBody<f64> {
    finished(
        "brick",
        sweep::test_support::brick(
            (lo[0], hi[0]),
            (lo[1], hi[1]),
            (lo[2], hi[2]),
            Tol::witness(),
        ),
        Tol::witness(),
    )
}

/// The rotation taking +y to `n` (unit).
fn rot_y_to(n: Vec3<f64>) -> Affine3<f64> {
    let y = Vec3::new(0.0, 1.0, 0.0);
    let ax = y.cross(n);
    if ax.norm() < 1e-14 {
        if n.y > 0.0 {
            return Affine3::identity();
        }
        return Affine3::rotation_about_axis(Point3::origin(), Vec3::new(1.0, 0.0, 0.0), PI);
    }
    Affine3::rotation_about_axis(Point3::origin(), ax / ax.norm(), ax.norm().atan2(y.dot(n)))
}

/// A `w × t × w` slab whose near face is the plane `n·p = s` and whose
/// material is `n·p ∈ [s, s+t]`, centred on the foot `n s + off`.
fn slab(n: Vec3<f64>, s: f64, w: f64, t: f64, off: Vec3<f64>) -> (AtRestBody<f64>, Vec<F>) {
    let n = n / n.norm();
    let rot = rot_y_to(n);
    let b: Body<f64> = sweep::test_support::brick(
        (-w / 2.0, w / 2.0),
        (0.0, t),
        (-w / 2.0, w / 2.0),
        Tol::witness(),
    );
    let place = Affine3::translation(n * s + off) * rot;
    let b = topo::transform_rigid(&b, &place, Tol::witness()).unwrap();
    let ex = rot.transform_vec(Vec3::new(1.0, 0.0, 0.0));
    let ez = rot.transform_vec(Vec3::new(0.0, 0.0, 1.0));
    let o = Point3::origin() + n * s + off;
    let fs: Vec<F> = vec![
        Box::new(move |p| n.dot(p - o)),
        Box::new(move |p| t - n.dot(p - o)),
        Box::new(move |p| w / 2.0 - ex.dot(p - o)),
        Box::new(move |p| w / 2.0 + ex.dot(p - o)),
        Box::new(move |p| w / 2.0 - ez.dot(p - o)),
        Box::new(move |p| w / 2.0 + ez.dot(p - o)),
    ];
    (finished("slab", b, Tol::witness()), fs)
}

fn op_run(op: BooleanOp, a: &AtRestBody<f64>, b: &AtRestBody<f64>) -> Result<Option<AtRestBody<f64>>, String> {
    let t = Tol::witness();
    let r = match op {
        BooleanOp::Union => topo::boolean::union(a, b, t),
        BooleanOp::Intersect => topo::boolean::intersect(a, b, t),
        BooleanOp::Subtract => topo::boolean::subtract(a, b, t),
    };
    match r {
        Ok(out) => Ok(out.body().map(|b| b.body.clone())),
        Err(e) => Err(format!("{e:?}")),
    }
}

#[derive(Default, Debug)]
struct Tally {
    ok: usize,
    refused: usize,
    wrong: usize,
    panicked: usize,
}

/// Deterministic LCG.
struct Rng(u64);
impl Rng {
    fn next(&mut self) -> f64 {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        ((self.0 >> 11) as f64) / ((1u64 << 53) as f64)
    }
}

/// Checks one built body: three tiers, volume, point classification.
fn check(
    label: &str,
    body: &Body<f64>,
    expected: Option<f64>,
    truth: &dyn Fn(Point3<f64>) -> f64,
    focus: (Point3<f64>, f64),
    scale: f64,
) -> Result<String, String> {
    let t = Tol::witness();
    topo::validate(body).map_err(|e| format!("validate {e:?}"))?;
    topo::validate_closed(body).map_err(|e| format!("validate_closed {e:?}"))?;
    topo::validate_geometric(body, t).map_err(|e| format!("validate_geometric {e:?}"))?;
    let p = topo::mass_properties(body, t).map_err(|e| format!("props {e:?}"))?;
    let mut note = format!("V={:.15e}", p.volume);
    if let Some(want) = expected {
        let floor = scale.powi(3);
        let err = (p.volume - want).abs() / want.abs().max(floor);
        note += &format!(" rel={err:.1e}");
        if err > 1e-8 {
            return Err(format!("{label}: volume {} vs oracle {want} (rel {err:.2e})", p.volume));
        }
    }
    // point classification: half near the focus (the cap), half wide.
    let mut rng = Rng(0x4044_0001 ^ (label.len() as u64) * 7919);
    let (c, rad) = focus;
    let band = Band::linear(t).unwrap();
    let mut bad = 0usize;
    let mut asked = 0usize;
    let mut first_bad = String::new();
    let mut pis_err = 0usize;
    let mut first_err = String::new();
    for i in 0..240 {
        let r = if i % 2 == 0 { rad } else { 2.2 * scale };
        let c0 = if i % 2 == 0 { c } else { Point3::origin() + Vec3::new(0.0, 0.3 * scale, 0.0) };
        let q = c0 + Vec3::new(rng.next() - 0.5, rng.next() - 0.5, rng.next() - 0.5) * (2.0 * r);
        let v = truth(q);
        if v.abs() < 1e-6 * scale {
            continue;
        }
        asked += 1;
        match topo::boolean::point_in_solid(body, q, band, t) {
            Ok(SolidContainment::In) if v > 0.0 => {}
            Ok(SolidContainment::Out) if v < 0.0 => {}
            Err(e) => {
                pis_err += 1;
                if first_err.is_empty() {
                    first_err = format!("{e:?}");
                }
            }
            got => {
                bad += 1;
                if first_bad.is_empty() {
                    first_bad = format!("q={q:?} truth={v:.3e} got={got:?}");
                }
            }
        }
    }
    if bad > 0 {
        return Err(format!("{label}: point_in_solid {bad}/{asked} wrong; first {first_bad}"));
    }
    Ok(note + &format!(" pis ok {}/{asked} refused {pis_err} {}", asked - pis_err, &first_err[..first_err.len().min(60)]))
}

/// Runs every op both orders of `x` (truth `xs`, volume `vx`) against the
/// slab `y` (truth `ys`, volume `vy`), with `cap` the volume of x∩y when
/// known. Prints one line per op.
#[allow(clippy::too_many_arguments)]
fn survey(
    tally: &mut Tally,
    label: &str,
    x: &AtRestBody<f64>,
    xs: &Shape,
    vx: Option<f64>,
    y: &AtRestBody<f64>,
    ys: &Shape,
    vy: Option<f64>,
    cap: Option<f64>,
    focus: (Point3<f64>, f64),
    scale: f64,
) -> Vec<AtRestBody<f64>> {
    let mut built = Vec::new();
    let vol = |a: Option<f64>, b: Option<f64>, f: &dyn Fn(f64, f64, f64) -> f64| match (a, b, cap) {
        (Some(a), Some(b), Some(c)) => Some(f(a, b, c)),
        _ => None,
    };
    for (name, op, swap) in [
        ("x∪y", BooleanOp::Union, false),
        ("y∪x", BooleanOp::Union, true),
        ("x∩y", BooleanOp::Intersect, false),
        ("y∩x", BooleanOp::Intersect, true),
        ("x∖y", BooleanOp::Subtract, false),
        ("y∖x", BooleanOp::Subtract, true),
    ] {
        let (l, r) = if swap { (y, x) } else { (x, y) };
        let (ls, rs) = if swap { (ys, xs) } else { (xs, ys) };
        let want = match (op, swap) {
            (BooleanOp::Union, _) => vol(vx, vy, &|a, b, c| a + b - c),
            (BooleanOp::Intersect, _) => cap,
            (BooleanOp::Subtract, false) => vol(vx, vy, &|a, _, c| a - c),
            (BooleanOp::Subtract, true) => vol(vx, vy, &|_, b, c| b - c),
        };
        let truth = |p: Point3<f64>| {
            let (a, b) = (ls.eval(p), rs.eval(p));
            match op {
                BooleanOp::Union => a.max(b),
                BooleanOp::Intersect => a.min(b),
                BooleanOp::Subtract => a.min(-b),
            }
        };
        let full = format!("{label} {name}");
        let res = catch_unwind(AssertUnwindSafe(|| op_run(op, l, r)));
        match res {
            Err(p) => {
                tally.panicked += 1;
                let msg = p
                    .downcast_ref::<String>()
                    .cloned()
                    .or_else(|| p.downcast_ref::<&str>().map(|s| s.to_string()))
                    .unwrap_or_default();
                println!("PANIC   {full}: {msg}");
            }
            Ok(Err(e)) => {
                tally.refused += 1;
                println!("REFUSED {full}: {}", &e[..e.len().min(260)]);
            }
            Ok(Ok(None)) => {
                if want.is_some_and(|w| w.abs() > 1e-12 * scale.powi(3)) {
                    tally.wrong += 1;
                    println!("WRONG   {full}: empty, oracle {want:?}");
                } else {
                    tally.ok += 1;
                    println!("OK      {full}: empty");
                }
            }
            Ok(Ok(Some(b))) => match catch_unwind(AssertUnwindSafe(|| check(&full, &b, want, &truth, focus, scale))) {
                Ok(Ok(note)) => {
                    tally.ok += 1;
                    println!("OK      {full}: {note}");
                    built.push(b);
                }
                Ok(Err(e)) => {
                    tally.wrong += 1;
                    println!("WRONG   {full}: {e}");
                }
                Err(_) => {
                    tally.panicked += 1;
                    println!("PANIC   {full}: in check");
                }
            },
        }
    }
    built
}

fn cap_volume(r: f64, h: f64) -> f64 {
    PI * h * h * (3.0 * r - h) / 3.0
}
fn lens_volume(r1: f64, r2: f64, d: f64) -> f64 {
    let x = (d * d + r1 * r1 - r2 * r2) / (2.0 * d);
    cap_volume(r1, r1 - x) + cap_volume(r2, r2 - (d - x))
}
/// Unit vector at latitude `lat` (from the xz-plane toward +y) and
/// azimuth `az` measured from +x toward +z.
fn dir(lat_deg: f64, az_deg: f64) -> Vec3<f64> {
    let (la, az) = (lat_deg.to_radians(), az_deg.to_radians());
    Vec3::new(la.cos() * az.cos(), la.sin(), la.cos() * az.sin())
}

fn slab_cap(
    tally: &mut Tally,
    label: &str,
    body: &AtRestBody<f64>,
    shape: &Shape,
    v: f64,
    c: Point3<f64>,
    r: f64,
    n: Vec3<f64>,
    h: f64,
    scale: f64,
) -> Vec<AtRestBody<f64>> {
    slab_cap_with(tally, label, body, shape, v, c, r, n, h, scale, cap_volume(r, h))
}

/// [`slab_cap`] with the x∩slab volume given by an outside oracle
/// (`probes/*.py`) where the plain cap formula does not apply.
#[allow(clippy::too_many_arguments)]
fn slab_cap_with(
    tally: &mut Tally,
    label: &str,
    body: &AtRestBody<f64>,
    shape: &Shape,
    v: f64,
    c: Point3<f64>,
    r: f64,
    n: Vec3<f64>,
    h: f64,
    scale: f64,
    cap: f64,
) -> Vec<AtRestBody<f64>> {
    let n = n / n.norm();
    let s = n.dot(c - Point3::origin()) + r - h;
    let w = 6.0 * scale;
    let (sl, fs) = slab(n, s, w, scale, Vec3::new(0.0, 0.0, 0.0));
    let ys = Shape(vec![fs]);
    let rho = (h * (2.0 * r - h)).sqrt();
    let focus = (c + n * (r - h), (rho * 1.5).max(h * 4.0));
    survey(tally, label, body, shape, Some(v), &sl, &ys, Some(w * w * scale), Some(cap), focus, scale)
}

fn lens_shape(k: f64, r1: f64, r2: f64, d: f64) -> Shape {
    Shape(vec![vec![
        ball_f(Point3::origin(), r1 * k),
        ball_f(Point3::new(0.0, d * k, 0.0), r2 * k),
    ]])
}

fn report(t: &Tally) {
    println!("TALLY {t:?}");
    assert_eq!(t.wrong, 0, "a wrong body");
    assert_eq!(t.panicked, 0, "a panic");
}

/// The lens's unit-sphere cap face (pole at y=1, rim at y≈0.8286,
/// angular radius ≈34°), cut at many tilts, depths, azimuths and scales.
#[test]
fn probe_lens_caps() {
    let mut t = Tally::default();
    for k in [1.0, 1e-3, 1e3] {
        let lens_b = {
            let a = ball(k, 0.0);
            let b = ball(0.8 * k, 1.4 * k);
            op_run(BooleanOp::Intersect, &a, &b).unwrap().unwrap()
        };
        let shape = lens_shape(k, 1.0, 0.8, 1.4);
        let v = lens_volume(k, 0.8 * k, 1.4 * k);
        for (tilt, az, h) in [
            (20.0, 90.0, 0.015),
            (20.0, 45.0, 0.015),
            (20.0, 200.0, 0.015),
            (12.0, 90.0, 0.015),   // pole 2° outside the circle
            (9.0, 90.0, 0.015),    // circle holds the pole
            (20.0, 90.0, 1e-6),
            (20.0, 90.0, 1e-4),
            (25.0, 90.0, 0.012),   // near the rim (25+8.9 = 33.9 < 34.06)
            (26.0, 90.0, 0.015),   // crosses the rim (26+9.9 > 34)
            (17.0, 5.0, 0.015),    // close to the seam at azimuth 0
            (17.0, 1.0, 1e-4),     // closer still, tiny circle
        ] {
            if k != 1.0 && !(tilt == 20.0 && az == 90.0 && h == 0.015) && !(tilt == 9.0) {
                continue;
            }
            let label = format!("lens k={k} tilt={tilt} az={az} h={h}");
            // tilt 26: the circle crosses the rim; slice oracle (probes/slice_oracle.py)
            let cap = if tilt == 26.0 { 6.919291021145407e-4 } else { cap_volume(k, h * k) };
            slab_cap_with(&mut t, &label, &lens_b, &shape, v, Point3::origin(), k, dir(90.0 - tilt, az), h * k, k, cap);
        }
    }
    report(&t);
}

/// The lens's OTHER face (the 0.8 sphere's lower cap, pole y=0.6), and an
/// unequal lens.
#[test]
fn probe_lens_lower_and_unequal() {
    let mut t = Tally::default();
    let lens_b = op_run(BooleanOp::Intersect, &ball(1.0, 0.0), &ball(0.8, 1.4)).unwrap().unwrap();
    let shape = lens_shape(1.0, 1.0, 0.8, 1.4);
    let v = lens_volume(1.0, 0.8, 1.4);
    let c2 = Point3::new(0.0, 1.4, 0.0);
    for (tilt, az, h) in [(20.0, 90.0, 0.01), (25.0, 30.0, 0.008)] {
        let label = format!("lens lower tilt={tilt} az={az} h={h}");
        slab_cap(&mut t, &label, &lens_b, &shape, v, c2, 0.8, dir(-(90.0 - tilt), az), h, 1.0);
    }
    // unequal lens ball(1) ∩ ball(0.5, 1.2): radical plane y = (1.44+1-.25)/2.4
    let ul = op_run(BooleanOp::Intersect, &ball(1.0, 0.0), &ball(0.5, 1.2)).unwrap().unwrap();
    let us = lens_shape(1.0, 1.0, 0.5, 1.2);
    let uv = lens_volume(1.0, 0.5, 1.2);
    for (tilt, az, h) in [(12.0, 90.0, 0.01), (18.0, 135.0, 0.003)] {
        let label = format!("unequal lens tilt={tilt} az={az} h={h}");
        slab_cap(&mut t, &label, &ul, &us, uv, Point3::origin(), 1.0, dir(90.0 - tilt, az), h, 1.0);
    }
    report(&t);
}

/// Banded ball (|y| ≤ 0.6) at several latitudes and azimuths, results
/// reused as operands, and a second cap on a cut face.
#[test]
fn probe_band() {
    let mut t = Tally::default();
    let band_box = brick([-2.0, -0.6, -2.0], [2.0, 0.6, 2.0]);
    let banded = op_run(BooleanOp::Intersect, &ball(1.0, 0.0), &band_box).unwrap().unwrap();
    let mut shape_parts = vec![ball_f(Point3::origin(), 1.0)];
    shape_parts.extend(box_fs([-2.0, -0.6, -2.0], [2.0, 0.6, 2.0]));
    let shape = Shape(vec![shape_parts]);
    let v = PI * (1.2 - 2.0 * 0.6f64.powi(3) / 3.0);
    for (lat, az, h) in [
        (10.0, 90.0, 0.015),
        (0.0, 60.0, 0.015),
        (-20.0, 120.0, 0.02),
        (25.0, 270.0, 0.01),
        (10.0, 3.0, 1e-4),
        (0.0, 90.0, 1e-6),
        (0.0, 90.0, 0.15), // circle 31.8° across: reaches the rims (lat ±36.9)? 31.8 < 36.9
    ] {
        let label = format!("band lat={lat} az={az} h={h}");
        slab_cap(&mut t, &label, &banded, &shape, v, Point3::origin(), 1.0, dir(lat, az), h, 1.0);
    }
    report(&t);
}

/// A quarter-ball face spanning pole to pole: ball ∩ {x ≥ 0} has its
/// two faces (z>0, z<0) bounded by the x=0 meridian and the seam at +x;
/// a cap at azimuth 45° gets a cut from the south pole to the north
/// pole, a span of exactly π.
#[test]
fn probe_pole_to_pole() {
    let mut t = Tally::default();
    let half = brick([0.0, -2.0, -2.0], [2.0, 2.0, 2.0]);
    let hemi = op_run(BooleanOp::Intersect, &ball(1.0, 0.0), &half).unwrap().unwrap();
    let mut parts = vec![ball_f(Point3::origin(), 1.0)];
    parts.extend(box_fs([0.0, -2.0, -2.0], [2.0, 2.0, 2.0]));
    let shape = Shape(vec![parts]);
    let v = 2.0 * PI / 3.0;
    for (lat, az, h) in [
        (0.0, 45.0, 0.015),
        (30.0, 45.0, 0.015),
        (-40.0, 60.0, 0.01),
        (0.0, -45.0, 0.015),
    ] {
        let label = format!("hemi lat={lat} az={az} h={h}");
        let built = slab_cap(&mut t, &label, &hemi, &shape, v, Point3::origin(), 1.0, dir(lat, az), h, 1.0);
        let _ = built;
    }
    report(&t);
}

/// Two caps in one face in ONE op (two cut-ins on one face key), and in
/// sequence (a cut result reused as an operand).
#[test]
fn probe_two_caps_one_face() {
    let mut t = Tally::default();
    let half = brick([0.0, -2.0, -2.0], [2.0, 2.0, 2.0]);
    let hemi = op_run(BooleanOp::Intersect, &ball(1.0, 0.0), &half).unwrap().unwrap();
    let mut parts = vec![ball_f(Point3::origin(), 1.0)];
    parts.extend(box_fs([0.0, -2.0, -2.0], [2.0, 2.0, 2.0]));
    let shape = Shape(vec![parts]);
    let v = 2.0 * PI / 3.0;
    let h = 0.015;
    let mk = |n: Vec3<f64>| {
        let n = n / n.norm();
        slab(n, 1.0 - h, 0.5, 0.3, Vec3::new(0.0, 0.0, 0.0))
    };
    let (n1, n2) = (dir(25.0, 45.0), dir(-25.0, 45.0));
    let (b1, f1) = mk(n1);
    let (b2, _f2) = mk(n2);
    let s1 = Shape(vec![f1]);
    let vb = 0.5 * 0.5 * 0.3;
    let cap = cap_volume(1.0, h);
    let focus = (Point3::origin() + n1 * (1.0 - h), 0.3);
    // sequence: hemi ∖ b1, then ∖ b2
    let firsts = survey(&mut t, "hemi vs brick1", &hemi, &shape, Some(v), &b1, &s1, Some(vb), Some(cap), focus, 1.0);
    let _ = firsts;
    match op_run(BooleanOp::Subtract, &hemi, &b1) {
        Ok(Some(r1)) => {
            let mut p1 = vec![ball_f(Point3::origin(), 1.0)];
            p1.extend(box_fs([0.0, -2.0, -2.0], [2.0, 2.0, 2.0]));
            // hemi ∖ b1 is not an intersection-of-positives; use truth via closure in a Shape of one F
            let (_, g1) = mk(n1);
            let g1s = Shape(vec![g1]);
            let hemi_s = Shape(vec![p1]);
            let r1_shape = Shape(vec![vec![Box::new(move |p| hemi_s.eval(p).min(-g1s.eval(p))) as F]]);
            let (_, f2b) = mk(n2);
            let s2 = Shape(vec![f2b]);
            let focus2 = (Point3::origin() + n2 * (1.0 - h), 0.3);
            survey(&mut t, "(hemi∖b1) vs brick2", &r1, &r1_shape, Some(v - cap), &b2, &s2, Some(vb), Some(cap), focus2, 1.0);
        }
        other => println!("SKIP sequence: {:?}", other.map(|o| o.is_some())),
    }
    // one op: the two bricks' union as one operand
    match op_run(BooleanOp::Union, &b1, &b2) {
        Ok(Some(u)) => {
            let (_, g1) = mk(n1);
            let (_, g2) = mk(n2);
            let us = Shape(vec![g1, g2]);
            survey(&mut t, "hemi vs (b1∪b2)", &hemi, &shape, Some(v), &u, &us, Some(2.0 * vb), Some(2.0 * cap), focus, 1.0);
        }
        other => println!("SKIP union: {:?}", other.map(|o| o.is_some())),
    }
    report(&t);
}

/// A ball wedge: the half-disc lamina revolved by `theta` about y, then
/// moved by `off`. Its sphere face spans pole to pole between two
/// meridian arcs, so a cut through it runs pole to pole (span π).
fn wedge(theta: f64, off: Vec3<f64>) -> (AtRestBody<f64>, Shape) {
    let b = revolved_about_y(
        vec![(Point2::new(0.0, -1.0), 1.0), (Point2::new(0.0, 1.0), 0.0)],
        Revolution::Partial(theta),
        Tol::witness(),
    );
    let b = topo::transform_rigid(&b, &Affine3::translation(off), Tol::witness()).unwrap();
    let c = Point3::origin() + off;
    let sector: F = Box::new(move |p| {
        let q = p - c;
        let rho = (q.x * q.x + q.z * q.z).sqrt();
        let mut psi = (-q.z).atan2(q.x);
        if psi < 0.0 {
            psi += 2.0 * PI;
        }
        if psi <= theta {
            rho * (psi.min(theta - psi)).min(PI / 2.0).sin()
        } else {
            -rho * ((psi - theta).min(2.0 * PI - psi)).min(PI / 2.0).sin()
        }
    });
    (finished("wedge", b, Tol::witness()), Shape(vec![vec![ball_f(c, 1.0), sector]]))
}

#[test]
fn probe_wedge_pole_to_pole() {
    let mut t = Tally::default();
    for (theta, off) in [
        (PI, Vec3::new(0.0, 0.0, 0.0)),
        (PI, Vec3::new(0.3, -0.2, 0.1)),
        (2.0, Vec3::new(0.0, 0.0, 0.0)),
        (4.0, Vec3::new(0.0, 0.0, 0.0)),
        (4.0, Vec3::new(0.3, -0.2, 0.1)),
    ] {
        let (w, ws) = wedge(theta, off);
        let v = theta / (2.0 * PI) * 4.0 / 3.0 * PI;
        let c = Point3::origin() + off;
        // the face's azimuths in dir()'s convention run from 0 to -theta
        for (lat, frac) in [(0.0, 0.5), (35.0, 0.3), (-20.0, 0.7)] {
            let az = -(theta * frac).to_degrees();
            let label = format!("wedge θ={theta:.3} off={:?} lat={lat} az={az:.1}", (off.x, off.y, off.z));
            slab_cap(&mut t, &label, &w, &ws, v, c, 1.0, dir(lat, az), 0.015, 1.0);
        }
    }
    report(&t);
}

/// Does point classification read the lens itself, and the PR's own
/// lens-cut result?
#[test]
fn probe_pis_on_lens_and_cut() {
    let t = Tol::witness();
    let band = Band::linear(t).unwrap();
    let lens = op_run(BooleanOp::Intersect, &ball(1.0, 0.0), &ball(0.8, 1.4)).unwrap().unwrap();
    let q = Point3::new(0.0, 0.9, 0.0);
    println!("PIS lens itself: {:?}", topo::boolean::point_in_solid(&lens, q, band, t));
    let (sl, _) = slab(dir(70.0, 90.0), 0.985, 6.0, 1.0, Vec3::new(0.0, 0.0, 0.0));
    for op in [BooleanOp::Union, BooleanOp::Intersect, BooleanOp::Subtract] {
        let r = op_run(op, &lens, &sl).unwrap().unwrap();
        println!("PIS lens {op:?} slab20: {:?}", topo::boolean::point_in_solid(&r, q, band, t));
    }
    let (slz, _) = slab(dir(70.0, 0.0), 0.985, 6.0, 1.0, Vec3::new(0.0, 0.0, 0.0));
    for op in [BooleanOp::Union, BooleanOp::Intersect, BooleanOp::Subtract] {
        let r = op_run(op, &lens, &slz).unwrap().unwrap();
        println!("PIS lens {op:?} slab-z20: {:?}", topo::boolean::point_in_solid(&r, q, band, t));
    }
}

fn census(b: &Body<f64>) -> String {
    let (mut sph, mut pl, mut other) = (0, 0, 0);
    for (_, f) in b.faces() {
        match b.get_surface(f.surface) {
            Some(geom::Surface::Sphere { .. }) => sph += 1,
            Some(geom::Surface::Plane { .. }) => pl += 1,
            _ => other += 1,
        }
    }
    format!("faces sphere={sph} plane={pl} other={other} edges={}", b.edges().count())
}

/// Topology of the cut results (x-tilt: the meridian cut) against the
/// z-tilt (the seams already cross the circle) and the bare operands.
#[test]
fn probe_census() {
    let lens = op_run(BooleanOp::Intersect, &ball(1.0, 0.0), &ball(0.8, 1.4)).unwrap().unwrap();
    println!("CENSUS lens: {}", census(&lens));
    for (name, n) in [("x-tilt(cut)", dir(70.0, 90.0)), ("z-tilt(seam)", dir(70.0, 0.0))] {
        let (sl, _) = slab(n, 0.985, 6.0, 1.0, Vec3::new(0.0, 0.0, 0.0));
        for (on, op, swap) in [("∪", BooleanOp::Union, false), ("∩", BooleanOp::Intersect, false), ("lens∖slab", BooleanOp::Subtract, false), ("slab∖lens", BooleanOp::Subtract, true)] {
            let r = if swap { op_run(op, &sl, &lens) } else { op_run(op, &lens, &sl) };
            let r = r.unwrap().unwrap();
            println!("CENSUS {name} {on}: {}", census(&r));
        }
    }
}

/// A cut result reused as an operand: a second cap off the same lens
/// face, and the same face cut twice by consecutive ops.
#[test]
fn probe_reuse() {
    let mut t = Tally::default();
    let lens = op_run(BooleanOp::Intersect, &ball(1.0, 0.0), &ball(0.8, 1.4)).unwrap().unwrap();
    let ls = lens_shape(1.0, 1.0, 0.8, 1.4);
    let v = lens_volume(1.0, 0.8, 1.4);
    let h = 0.015;
    let n1 = dir(70.0, 90.0);
    let (s1, f1) = slab(n1, 1.0 - h, 6.0, 1.0, Vec3::new(0.0, 0.0, 0.0));
    let r1 = op_run(BooleanOp::Subtract, &lens, &s1).unwrap().unwrap();
    let g1 = Shape(vec![f1]);
    let r1s = Shape(vec![vec![Box::new(move |p| ls.eval(p).min(-g1.eval(p))) as F]]);
    let cap = cap_volume(1.0, h);
    // overlapping second caps: probes/two_cap_oracle.py
    for (lab, n, cap2) in [
        ("az 210", dir(70.0, 210.0), cap),
        ("az 150", dir(72.0, 150.0), 7.024532425084538e-4),
        ("az 30", dir(75.0, 30.0), 6.989406182349951e-4),
    ] {
        let label = format!("(lens∖cap) reused, second cap {lab}");
        slab_cap_with(&mut t, &label, &r1, &r1s, v - cap, Point3::origin(), 1.0, n, h, 1.0, cap2);
    }
    report(&t);
}

/// The PR's pole-strut carve, `ball_poled_y(0.5) ∖ [−1,0.25]×[−1,1]×[−1,0]`,
/// against slabs at 0.4925 (h = 0.0075); closed-form oracle: the carve is
/// the ball less half the ball's z<0 half beyond... i.e. the ball less
/// (z≤0 half ball less half of the x>0.25 cap). Builds only with #4046.
#[test]
fn probe_strut() {
    let mut t = Tally::default();
    let tol = Tol::witness();
    let ball = finished("ball", sweep::test_support::ball_poled_y(0.5, Vec3::new(0.0, 0.0, 0.0), tol), tol);
    let cutter = brick([-1.0, -1.0, -1.0], [0.25, 1.0, 0.0]);
    let carve = match op_run(BooleanOp::Subtract, &ball, &cutter) {
        Ok(Some(c)) => c,
        other => panic!("carve: {other:?}"),
    };
    let r = 0.5;
    let boxed = 0.5 * (4.0 / 3.0 * PI * r * r * r) - 0.5 * cap_volume(r, 0.25);
    let v = 4.0 / 3.0 * PI * r * r * r - boxed;
    let mut parts_ball = vec![ball_f(Point3::origin(), r)];
    let bx = box_fs([-1.0, -1.0, -1.0], [0.25, 1.0, 0.0]);
    let not_box: F = Box::new(move |p| -bx.iter().map(|f| f(p)).fold(f64::INFINITY, f64::min));
    parts_ball.push(not_box);
    let shape = Shape(vec![parts_ball]);
    let tt = 10f64.to_radians();
    for (lab, n) in [
        ("lat10", Vec3::new(0.0, tt.sin(), tt.cos())),
        ("x-face", Vec3::new(0.866, 0.1, -0.5)),
        ("x-face2", Vec3::new(0.9, -0.2, -0.35)),
        ("lat-30", Vec3::new(0.3, -0.5, 0.8)),
    ] {
        slab_cap(&mut t, &format!("strut {lab}"), &carve, &shape, v, Point3::origin(), r, n, 0.0075, 1.0);
    }
    report(&t);
}

/// Two circles inside ONE lens face (the z>0 half of the unit sphere's
/// cap, azimuths 50° and 130°), cut in by ONE op whose operand is the
/// union of two small bricks: two `SphereCutIn`s on one face key.
#[test]
fn probe_two_cuts_one_lens_face() {
    let mut t = Tally::default();
    let lens = op_run(BooleanOp::Intersect, &ball(1.0, 0.0), &ball(0.8, 1.4)).unwrap().unwrap();
    let ls = lens_shape(1.0, 1.0, 0.8, 1.4);
    let v = lens_volume(1.0, 0.8, 1.4);
    let h = 0.015;
    let mk = |n: Vec3<f64>| slab(n, 1.0 - h, 0.4, 0.3, Vec3::new(0.0, 0.0, 0.0));
    for (a1, a2) in [(50.0, 130.0), (130.0, 50.0), (20.0, 160.0), (160.0, 20.0), (210.0, 330.0), (330.0, 210.0)] {
        let (n1, n2) = (dir(68.0, a1), dir(68.0, a2));
        let (b1, _) = mk(n1);
        let (b2, _) = mk(n2);
        let u = match op_run(BooleanOp::Union, &b1, &b2) {
            Ok(Some(u)) => u,
            other => {
                println!("SKIP union {a1}/{a2}: {:?}", other.map(|o| o.is_some()));
                continue;
            }
        };
        let (_, g1) = mk(n1);
        let (_, g2) = mk(n2);
        let us = Shape(vec![g1, g2]);
        let vb = 2.0 * 0.4 * 0.4 * 0.3;
        let cap = cap_volume(1.0, h);
        let focus = (Point3::origin() + n1 * (1.0 - h), 0.3);
        survey(&mut t, &format!("lens vs two bricks {a1}/{a2}"), &lens, &ls, Some(v), &u, &us, Some(vb), Some(2.0 * cap), focus, 1.0);
    }
    report(&t);
}
