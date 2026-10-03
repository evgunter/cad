//! Reviewer probe (reach-dual3985-r1): every result edge sampled against an
//! INDEPENDENT closed-form signed function of the result solid, kernel volume
//! against a Monte Carlo of that function, point_in_solid against it, and a
//! bit digest of every edge's samples for the base/head differential.
//! Output: one line per run to $PROBE_OUT (append).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Affine3, Band, Point2, Point3, Tol, Vec3};
use std::hash::{Hash, Hasher};
use std::io::Write;
use sweep::Revolution;
use sweep::test_support::revolved_about_y;
use topo::Body;

/// Closed-form solid: f(p) < 0 inside, > 0 outside, 0 on the boundary.
#[derive(Clone)]
enum S {
    Ball(Vec3<f64>, f64),
    /// box in a frame: centre, rotation (columns = local axes), half sizes
    Boxr(Vec3<f64>, [Vec3<f64>; 3], [f64; 3]),
    /// z-cylinder of radius r about (x,y) on z in [z0,z1]
    Cyl(f64, f64, f64, f64, f64),
    Union(Box<S>, Box<S>),
    Inter(Box<S>, Box<S>),
    Sub(Box<S>, Box<S>),
    Half(Vec3<f64>, Vec3<f64>), // origin, normal: inside where (p-o).n < 0... used for split parts
}
fn f(s: &S, p: Vec3<f64>) -> f64 {
    match s {
        S::Ball(c, r) => (p - *c).norm() - r,
        S::Boxr(c, ax, h) => {
            let d = p - *c;
            (0..3).map(|i| d.dot(ax[i]).abs() - h[i]).fold(f64::MIN, f64::max)
        }
        S::Cyl(x, y, r, z0, z1) => {
            let rr = ((p.x - x).powi(2) + (p.y - y).powi(2)).sqrt() - r;
            rr.max(z0 - p.z).max(p.z - z1)
        }
        S::Union(a, b) => f(a, p).min(f(b, p)),
        S::Inter(a, b) => f(a, p).max(f(b, p)),
        S::Sub(a, b) => f(a, p).max(-f(b, p)),
        S::Half(o, n) => (p - *o).dot(*n) / n.norm(),
    }
}
fn bbox(s: &S) -> (Vec3<f64>, Vec3<f64>) {
    match s {
        S::Ball(c, r) => (*c - Vec3::new(*r, *r, *r), *c + Vec3::new(*r, *r, *r)),
        S::Boxr(c, ax, h) => {
            let e = Vec3::new(
                (0..3).map(|i| (ax[i].x * h[i]).abs()).sum(),
                (0..3).map(|i| (ax[i].y * h[i]).abs()).sum(),
                (0..3).map(|i| (ax[i].z * h[i]).abs()).sum(),
            );
            (*c - e, *c + e)
        }
        S::Cyl(x, y, r, z0, z1) => (Vec3::new(x - r, y - r, *z0), Vec3::new(x + r, y + r, *z1)),
        S::Union(a, b) => {
            let (a0, a1) = bbox(a);
            let (b0, b1) = bbox(b);
            (
                Vec3::new(a0.x.min(b0.x), a0.y.min(b0.y), a0.z.min(b0.z)),
                Vec3::new(a1.x.max(b1.x), a1.y.max(b1.y), a1.z.max(b1.z)),
            )
        }
        S::Inter(a, _) | S::Sub(a, _) => bbox(a),
        S::Half(..) => panic!("half space has no box"),
    }
}

fn tol() -> Tol {
    Tol::witness()
}
fn ball(r: f64, c: Vec3<f64>) -> Body<f64> {
    let b = revolved_about_y(
        vec![(Point2::new(0.0, -r), 1.0), (Point2::new(0.0, r), 0.0)],
        Revolution::Full,
        tol(),
    );
    topo::transform_rigid(&b, &Affine3::translation(c), tol()).unwrap()
}
/// ball whose pole axis is turned by `ang` about `k` (about its centre)
fn ball_turned(r: f64, c: Vec3<f64>, k: Vec3<f64>, ang: f64) -> Option<Body<f64>> {
    let b = ball(r, c);
    let t = Affine3::rotation_about_axis(Point3::new(c.x, c.y, c.z), k, ang);
    topo::transform_rigid(&b, &t, tol()).ok()
}
/// axis-aligned brick, then turned by `ang` about axis `k` through the origin
fn brick_turned(
    x: (f64, f64),
    y: (f64, f64),
    z: (f64, f64),
    k: Vec3<f64>,
    ang: f64,
) -> (Body<f64>, S) {
    let b = sweep::test_support::brick(x, y, z, tol());
    let c = Vec3::new((x.0 + x.1) / 2.0, (y.0 + y.1) / 2.0, (z.0 + z.1) / 2.0);
    let h = [(x.1 - x.0) / 2.0, (y.1 - y.0) / 2.0, (z.1 - z.0) / 2.0];
    if ang == 0.0 {
        return (b, S::Boxr(c, [Vec3::unit_x(), Vec3::unit_y(), Vec3::unit_z()], h));
    }
    let t = Affine3::rotation_about_axis(Point3::origin(), k, ang);
    let Ok(body) = topo::transform_rigid(&b, &t, tol()) else {
        // the fixture's own rigid map refused (seen at eps 1e-12): an
        // empty body stands in and every op on it is recorded as refused
        return (topo::Body::new(), S::Ball(Vec3::new(0.0, 0.0, 0.0), 0.0));
    };
    let rot = |v: Vec3<f64>| -> Vec3<f64> {
        // Rodrigues
        let k = k / k.norm();
        v * ang.cos() + k.cross(v) * ang.sin() + k * (k.dot(v) * (1.0 - ang.cos()))
    };
    (
        body,
        S::Boxr(rot(c), [rot(Vec3::unit_x()), rot(Vec3::unit_y()), rot(Vec3::unit_z())], h),
    )
}
fn cyl(r: f64, x: f64, y: f64, z0: f64, h: f64) -> (Body<f64>, S) {
    (
        sweep::test_support::prism_at(
            vec![(Point2::new(x - r, y), 1.0), (Point2::new(x + r, y), 1.0)],
            z0,
            h,
            tol(),
        ),
        S::Cyl(x, y, r, z0, z0 + h),
    )
}

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> f64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        (self.0 >> 11) as f64 / (1u64 << 53) as f64
    }
}

#[derive(Default)]
struct Tally {
    runs: usize,
    built: usize,
    refused: usize,
    bad_edge: usize,
    bad_vol: usize,
    pis_disagree: usize,
}

/// Check one result body against its closed form; returns a line.
fn check(name: &str, body: &Body<f64>, oracle: &S, scale: f64, t: &mut Tally) -> String {
    t.built += 1;
    let mut problems = Vec::new();
    // 1. every edge's samples on the closed-form boundary
    let mut digest: Vec<String> = Vec::new();
    let mut worst = 0.0f64;
    for (_, e) in body.edges() {
        let Some(c) = body.get_curve_geom(e.curve).and_then(|g| g.certified()) else {
            problems.push("uncertified edge".to_string());
            continue;
        };
        let (t0, t1) = c.params();
        let mut pts = Vec::new();
        for k in 0..=8 {
            let s = t0 + (t1 - t0) * f64::from(k) / 8.0;
            let p = c.carrier().eval(s);
            let v = Vec3::new(p.x, p.y, p.z);
            worst = worst.max(f(oracle, v).abs());
            pts.push(format!("{:?},{:?},{:?}", p.x, p.y, p.z));
        }
        digest.push(pts.join(";"));
    }
    if worst > 1e-7 * scale {
        problems.push(format!("EDGE_OFF_BOUNDARY {worst:.3e}"));
        t.bad_edge += 1;
    }
    digest.sort();
    let mut h = std::collections::hash_map::DefaultHasher::new();
    digest.hash(&mut h);
    let hash = h.finish();
    // 2. volume: kernel against Monte Carlo of the closed form
    let vol = topo::mass_properties(body, tol()).map(|p| p.volume);
    let (lo, hi) = bbox(oracle);
    let span = hi - lo;
    let bv = span.x * span.y * span.z;
    let mut rng = Rng(0x9E3779B97F4A7C15 ^ (name.len() as u64 * 7919));
    let n = 60_000;
    let mut inside = 0usize;
    for _ in 0..n {
        let p = lo + Vec3::new(span.x * rng.next(), span.y * rng.next(), span.z * rng.next());
        if f(oracle, p) < 0.0 {
            inside += 1;
        }
    }
    let frac = inside as f64 / n as f64;
    let mc = frac * bv;
    let sigma = (frac * (1.0 - frac) / n as f64).sqrt() * bv;
    let vol_s = match &vol {
        Ok(v) => {
            if (v - mc).abs() > 5.0 * sigma + 1e-12 * bv {
                problems.push(format!("VOLUME {v:.6e} vs MC {mc:.6e}±{sigma:.1e}"));
                t.bad_vol += 1;
            }
            format!("{v:?}")
        }
        Err(e) => format!("massprops-err:{e:?}").chars().take(60).collect(),
    };
    // 3. point_in_solid against the closed form
    let band = Band::linear(tol()).unwrap();
    let (mut agree, mut dis, mut refu) = (0, 0, 0);
    let mut first_ref = String::new();
    for _ in 0..40 {
        let p = lo - span * 0.1 + Vec3::new(span.x * 1.2 * rng.next(), span.y * 1.2 * rng.next(), span.z * 1.2 * rng.next());
        let fv = f(oracle, p);
        if fv.abs() < 1e-3 * scale {
            continue;
        }
        match topo::point_in_solid(body, Point3::new(p.x, p.y, p.z), band, tol()) {
            Ok(topo::SolidContainment::In) if fv < 0.0 => agree += 1,
            Ok(topo::SolidContainment::Out) if fv > 0.0 => agree += 1,
            Ok(other) => {
                dis += 1;
                problems.push(format!("PIS {other:?} at {p:?} f={fv:.3e}"));
            }
            Err(e) => {
                refu += 1;
                if first_ref.is_empty() {
                    first_ref = format!("{e:?}").chars().take(40).collect();
                }
            }
        }
    }
    if dis > 0 {
        t.pis_disagree += 1;
    }
    format!(
        "{name}\tBUILT\thash={hash:016x}\tvol={vol_s}\tedges={}\tworst={worst:.2e}\tpis={agree}/{dis}/{refu} {first_ref}\t{}",
        digest.len(),
        if problems.is_empty() { "ok".to_string() } else { problems.join(" | ") }
    )
}

#[derive(Clone, Copy, Debug)]
enum Op {
    U,
    I,
    D,
}
fn run(op: Op, a: &Body<f64>, b: &Body<f64>) -> Result<Option<Body<f64>>, String> {
    let r = match op {
        Op::U => topo::boolean::union(a, b, tol()),
        Op::I => topo::boolean::intersect(a, b, tol()),
        Op::D => topo::boolean::subtract(a, b, tol()),
    };
    match r {
        Ok(out) => Ok(out.body().map(|bb| bb.body.clone())),
        Err(e) => Err(format!("{e:?}")),
    }
}

/// every op, both orders
fn all_ops(
    out: &mut Vec<String>,
    t: &mut Tally,
    name: &str,
    a: &Body<f64>,
    sa: &S,
    b: &Body<f64>,
    sb: &S,
    scale: f64,
    keep: &mut Vec<(String, Body<f64>, S)>,
) {
    for (tag, op, x, y, sx, sy) in [
        ("A∪B", Op::U, a, b, sa, sb),
        ("B∪A", Op::U, b, a, sb, sa),
        ("A∩B", Op::I, a, b, sa, sb),
        ("B∩A", Op::I, b, a, sb, sa),
        ("A∖B", Op::D, a, b, sa, sb),
        ("B∖A", Op::D, b, a, sb, sa),
    ] {
        t.runs += 1;
        let oracle = match op {
            Op::U => S::Union(Box::new(sx.clone()), Box::new(sy.clone())),
            Op::I => S::Inter(Box::new(sx.clone()), Box::new(sy.clone())),
            Op::D => S::Sub(Box::new(sx.clone()), Box::new(sy.clone())),
        };
        let label = format!("{name} {tag}");
        match run(op, x, y) {
            Ok(Some(body)) => {
                out.push(check(&label, &body, &oracle, scale, t));
                if keep.len() < 400 {
                    keep.push((label, body, oracle));
                }
            }
            Ok(None) => out.push(format!("{label}\tEMPTY")),
            Err(e) => {
                t.refused += 1;
                let short: String = e.chars().take(140).collect();
                out.push(format!("{label}\tREFUSED\t{short}"));
            }
        }
    }
}

fn write_out(file: &str, lines: &[String], t: &Tally) {
    let path = std::env::var("PROBE_OUT").unwrap_or_else(|_| "/tmp/probe_out".into());
    let path = format!("{path}.{file}");
    let mut fh = std::fs::File::create(&path).unwrap();
    for l in lines {
        writeln!(fh, "{l}").unwrap();
    }
    writeln!(
        fh,
        "TALLY runs={} built={} refused={} bad_edge={} bad_vol={} pis_disagree={}",
        t.runs, t.built, t.refused, t.bad_edge, t.bad_vol, t.pis_disagree
    )
    .unwrap();
}

/// Tilted sphere pairs, pierce rings, turned poles, three scales.
#[test]
fn zz_probe_sphere_pairs() {
    let mut out = Vec::new();
    let mut t = Tally::default();
    let mut keep = Vec::new();
    for s in [1e-3, 1.0, 1e3] {
        let base = Vec3::new(2.0, 2.0, 0.5) * s;
        for (pose, r, off, turn) in [
            ("eq-x", 1.0, Vec3::new(1.4, 0.0, 0.0), 0.0),
            ("eq-xz", 1.0, Vec3::new(1.3, 0.0, 0.2), 0.0),
            ("eq-xyz", 1.0, Vec3::new(1.0, 0.7, -0.5), 0.0),
            ("r.6-xyz", 0.6, Vec3::new(0.9, 0.3, 0.6), 0.0),
            ("r.35-in", 0.35, Vec3::new(0.85, -0.2, 0.3), 0.0),
            ("r1.7-big", 1.7, Vec3::new(1.2, 0.5, 0.4), 0.0),
            ("eq-x-turnedB", 1.0, Vec3::new(1.4, 0.0, 0.0), 0.7),
            ("r.6-xyz-turnedB", 0.6, Vec3::new(0.9, 0.3, 0.6), 2.1),
        ] {
            let a = ball(s, base);
            let cb = base + off * s;
            let b = if turn == 0.0 {
                ball(r * s, cb)
            } else {
                match ball_turned(r * s, cb, Vec3::new(0.3, 0.5, 0.8), turn) {
                    Some(b) => b,
                    None => {
                        out.push(format!("sph s={s:e} {pose}\tFIXTURE-REFUSED"));
                        continue;
                    }
                }
            };
            all_ops(
                &mut out,
                &mut t,
                &format!("sph s={s:e} {pose}"),
                &a,
                &S::Ball(base, s),
                &b,
                &S::Ball(cb, r * s),
                s,
                &mut keep,
            );
        }
    }
    write_out("sph", &out, &t);
}

/// Planes tilted against the ball's chart: box faces across a ball, the
/// four-crossing slab (0°,100°,200°,300°-shaped), rotated boxes, the bar
/// through a ball (reflex notch), at three scales.
#[test]
fn zz_probe_plane_sphere() {
    let mut out = Vec::new();
    let mut t = Tally::default();
    let mut keep = Vec::new();
    for s in [1e-3, 1.0, 1e3] {
        let c0 = Vec3::new(0.0, 0.0, 0.0);
        let mut cases: Vec<(String, (Body<f64>, S))> = Vec::new();
        for (x0, z) in [(0.5, (0.0, 2.0)), (0.3, (-2.0, 0.4)), (-0.4, (-0.7, 0.2)), (0.5, (-2.0, 2.0))] {
            cases.push((
                format!("box x>{x0} z{z:?}"),
                brick_turned((x0 * s, 3.0 * s), (-2.0 * s, 2.0 * s), (z.0 * s, z.1 * s), Vec3::unit_z(), 0.0),
            ));
        }
        // the four-crossing slab: plane x = c, circle radius ρ = √(1−c²);
        // strip a<z<b cuts it at asin(a/ρ), asin(b/ρ), π−asin(b/ρ), π−asin(a/ρ)
        for (c, a_deg, b_deg) in [(0.3f64, -40.0f64, 60.0f64), (0.0, -10.0, 80.0), (0.5, 10.0, 70.0)] {
            let rho = (1.0 - c * c).sqrt();
            let (za, zb) = (rho * a_deg.to_radians().sin(), rho * b_deg.to_radians().sin());
            cases.push((
                format!("slab4 x>{c} θ({a_deg},{b_deg})"),
                brick_turned((c * s, 3.0 * s), (-3.0 * s, 3.0 * s), (za * s, zb * s), Vec3::unit_z(), 0.0),
            ));
        }
        for (ang, k) in [(0.4, Vec3::new(0.0, 1.0, 0.0)), (1.1, Vec3::new(1.0, 1.0, 0.3)), (2.5, Vec3::new(0.2, -0.4, 1.0))] {
            cases.push((
                format!("box-turned {ang} {k:?}"),
                brick_turned((0.4 * s, 3.0 * s), (-0.5 * s, 0.6 * s), (-0.3 * s, 2.0 * s), k, ang),
            ));
        }
        for (h, y0) in [(0.3, 0.0), (0.2, 0.15), (0.45, -0.1)] {
            cases.push((
                format!("bar h={h} y0={y0}"),
                brick_turned((0.5 * s, 2.0 * s), ((y0 - h) * s, (y0 + h) * s), (-h * s, h * s), Vec3::unit_z(), 0.0),
            ));
        }
        // the bar again, turned off the axes
        cases.push((
            "bar-turned".into(),
            brick_turned((0.5 * s, 2.0 * s), (-0.25 * s, 0.25 * s), (-0.25 * s, 0.25 * s), Vec3::new(0.2, 0.3, 1.0), 0.8),
        ));
        for (name, (bx, sbx)) in cases {
            for (pole, bl) in [
                ("pole-y", Some(ball(s, c0))),
                ("pole-turned", ball_turned(s, c0, Vec3::new(0.4, 0.1, 0.9), 1.3)),
            ] {
                let Some(bl) = bl else {
                    out.push(format!("ps s={s:e} {name} {pole}\tFIXTURE-REFUSED"));
                    continue;
                };
                all_ops(&mut out, &mut t, &format!("ps s={s:e} {name} {pole}"), &bx, &sbx, &bl, &S::Ball(c0, s), s, &mut keep);
            }
        }
    }
    write_out("ps", &out, &t);
}

/// Cylinders: tilted box cuts (the window rule's old home), splits of
/// cylinders by tilted planes, and results reused as operands.
#[test]
fn zz_probe_cylinders_and_splits() {
    let mut out = Vec::new();
    let mut t = Tally::default();
    let mut keep = Vec::new();
    for s in [1e-3, 1.0, 1e3] {
        let (cb, cs) = cyl(1.0 * s, 0.0, 0.0, 0.0, 2.0 * s);
        for (ang, k, x) in [
            (0.3, Vec3::new(0.0, 1.0, 0.0), (0.2, 3.0)),
            (0.9, Vec3::new(1.0, 0.0, 0.2), (-0.5, 0.4)),
            (2.2, Vec3::new(0.3, 0.7, 0.5), (0.1, 0.6)),
        ] {
            let (bx, sbx) = brick_turned(
                (x.0 * s, x.1 * s),
                (-0.4 * s, 0.7 * s),
                (0.6 * s, 1.5 * s),
                k,
                ang,
            );
            all_ops(&mut out, &mut t, &format!("cyl s={s:e} box {ang}"), &cb, &cs, &bx, &sbx, s, &mut keep);
        }
        // a ball against a cylinder (curved×curved, likely refused; recorded)
        let bl = ball(0.6 * s, Vec3::new(0.9, 0.2, 1.0) * s);
        all_ops(&mut out, &mut t, &format!("cyl s={s:e} ball"), &cb, &cs, &bl, &S::Ball(Vec3::new(0.9, 0.2, 1.0) * s, 0.6 * s), s, &mut keep);
        // splits of the cylinder by tilted planes (the split lane's walk)
        for (o, n) in [
            (Vec3::new(0.0, 0.0, 1.0), Vec3::new(0.3, 0.2, 1.0)),
            (Vec3::new(0.2, 0.1, 1.0), Vec3::new(1.0, 0.3, 0.6)),
            (Vec3::new(0.0, 0.0, 0.9), Vec3::new(0.9, -0.8, 0.5)),
        ] {
            let o = o * s;
            let plane = topo::test_support::split_plane(Point3::new(o.x, o.y, o.z), n, tol());
            t.runs += 1;
            match topo::split(&cb, &plane, tol()) {
                Ok(r) => {
                    for (side, part, sgn) in [("above", &r.above, -1.0), ("below", &r.below, 1.0)] {
                        if let Some(b) = part.body() {
                            let or = S::Inter(Box::new(cs.clone()), Box::new(S::Half(o, n * sgn)));
                            out.push(check(&format!("split s={s:e} {n:?} {side}"), b, &or, s, &mut t));
                        }
                    }
                }
                Err(e) => {
                    t.refused += 1;
                    out.push(format!("split s={s:e} {n:?}\tREFUSED\t{}", format!("{e:?}").chars().take(140).collect::<String>()));
                }
            }
        }
    }
    // results reused as operands
    let (cb, cs) = cyl(1.0, 0.0, 0.0, 0.0, 2.0);
    let (bx, sbx) = brick_turned((0.2, 3.0), (-0.4, 0.7), (0.6, 1.5), Vec3::new(0.0, 1.0, 0.0), 0.3);
    if let Ok(Some(r1)) = run(Op::D, &cb, &bx) {
        let s1 = S::Sub(Box::new(cs.clone()), Box::new(sbx.clone()));
        let (bx2, sbx2) = brick_turned((-3.0, -0.3), (-0.2, 0.9), (0.2, 1.8), Vec3::new(0.4, 0.0, 1.0), 0.5);
        all_ops(&mut out, &mut t, "reuse cyl∖box vs box2", &r1, &s1, &bx2, &sbx2, 1.0, &mut keep);
    }
    let a = ball(1.0, Vec3::new(2.0, 2.0, 0.5));
    let b = ball(1.0, Vec3::new(3.4, 2.0, 0.5));
    if let Ok(Some(lens)) = run(Op::I, &a, &b) {
        let sl = S::Inter(Box::new(S::Ball(Vec3::new(2.0, 2.0, 0.5), 1.0)), Box::new(S::Ball(Vec3::new(3.4, 2.0, 0.5), 1.0)));
        let (bx3, sbx3) = brick_turned((2.6, 4.0), (1.0, 3.0), (0.4, 2.0), Vec3::unit_z(), 0.0);
        all_ops(&mut out, &mut t, "reuse lens vs box", &lens, &sl, &bx3, &sbx3, 1.0, &mut keep);
        let c = ball(0.5, Vec3::new(2.7, 2.6, 0.7));
        all_ops(&mut out, &mut t, "reuse lens vs ball", &lens, &sl, &c, &S::Ball(Vec3::new(2.7, 2.6, 0.7), 0.5), 1.0, &mut keep);
    }
    write_out("cyl", &out, &t);
}

/// The wedge/collar matrix (the PR's own poses plus scaled and turned
/// ones at ×1e-3 and ×1e3): a bit digest and the volume, for the
/// base/head differential. (The PR's own row holds the head's volumes
/// to the annular-sector closed form.)
#[test]
fn zz_probe_wedge_collar() {
    let mut out = Vec::new();
    let mut t = Tally::default();
    let rect = |r0: f64, r1: f64, y0: f64, y1: f64| {
        vec![
            (Point2::new(r0, y0), 0.0),
            (Point2::new(r1, y0), 0.0),
            (Point2::new(r1, y1), 0.0),
            (Point2::new(r0, y1), 0.0),
        ]
    };
    for s in [1e-3, 1.0, 1e3] {
        let collar = revolved_about_y(rect(0.5 * s, 1.5 * s, 1.0 * s, 2.0 * s), Revolution::Full, tol());
        for (r0, r1) in [(0.3, 0.9), (0.2, 0.7), (0.4, 1.2), (0.3, 1.8)] {
            for (y0, y1) in [(1.2, 1.8), (0.5, 1.5), (1.5, 2.5), (0.5, 2.5)] {
                for angle in [core::f64::consts::FRAC_PI_2, 1.0, 4.0] {
                    let wedge = revolved_about_y(
                        rect(r0 * s, r1 * s, y0 * s, y1 * s),
                        Revolution::Partial(angle),
                        tol(),
                    );
                    for (tag, op, x, y) in [
                        ("∪", Op::U, &collar, &wedge),
                        ("∩", Op::I, &collar, &wedge),
                        ("c∖w", Op::D, &collar, &wedge),
                        ("w∖c", Op::D, &wedge, &collar),
                    ] {
                        t.runs += 1;
                        let label = format!("wc s={s:e} ρ({r0},{r1}) y({y0},{y1}) θ{angle:.3} {tag}");
                        match run(op, x, y) {
                            Ok(Some(body)) => {
                                t.built += 1;
                                let mut digest: Vec<String> = Vec::new();
                                for (_, e) in body.edges() {
                                    if let Some(c) = body.get_curve_geom(e.curve).and_then(|g| g.certified()) {
                                        let (t0, t1) = c.params();
                                        let pts: Vec<String> = (0..=8)
                                            .map(|k| {
                                                let p = c.carrier().eval(t0 + (t1 - t0) * f64::from(k) / 8.0);
                                                format!("{:?},{:?},{:?}", p.x, p.y, p.z)
                                            })
                                            .collect();
                                        digest.push(pts.join(";"));
                                    }
                                }
                                digest.sort();
                                let mut h = std::collections::hash_map::DefaultHasher::new();
                                digest.hash(&mut h);
                                let v = topo::mass_properties(&body, tol()).map(|p| p.volume);
                                out.push(format!("{label}\tBUILT\thash={:016x}\tvol={v:?}", h.finish()));
                            }
                            Ok(None) => out.push(format!("{label}\tEMPTY")),
                            Err(e) => {
                                t.refused += 1;
                                out.push(format!("{label}\tREFUSED\t{}", e.chars().take(140).collect::<String>()));
                            }
                        }
                    }
                }
            }
        }
    }
    write_out("wc", &out, &t);
}

/// Splits on the cone (the window walk's third chart) and on boolean
/// results, by planes tilted every way, at three scales: each part's
/// edges on the closed form of (solid ∩ half-space).
#[test]
fn zz_probe_cone_and_reused_splits() {
    let mut out = Vec::new();
    let mut t = Tally::default();
    for s in [1e-3, 1.0, 1e3] {
        let (r, h) = (1.0 * s, 1.5 * s);
        let cone = revolved_about_y(
            vec![(Point2::new(0.0, 0.0), 0.0), (Point2::new(r, 0.0), 0.0), (Point2::new(0.0, h), 0.0)],
            Revolution::Full,
            tol(),
        );
        let frustum = revolved_about_y(
            vec![
                (Point2::new(0.0, 0.0), 0.0),
                (Point2::new(r, 0.0), 0.0),
                (Point2::new(0.4 * r, h), 0.0),
                (Point2::new(0.0, h), 0.0),
            ],
            Revolution::Full,
            tol(),
        );
        // an implicit for each: y in [0,h], ρ ≤ profile(y)
        let cone_f = move |p: Vec3<f64>, top: f64| {
            let rho = (p.x * p.x + p.z * p.z).sqrt();
            let prof = r + (top - r) * p.y / h;
            // scale the radial term by the slant so it is a distance
            let slant = ((r - top).powi(2) + h * h).sqrt() / h;
            ((rho - prof) / slant).max(-p.y).max(p.y - h)
        };
        for (name, body, top) in [("cone", &cone, 0.0), ("frustum", &frustum, 0.4 * r)] {
            for (o, n) in [
                (Vec3::new(0.0, 0.6, 0.0), Vec3::new(0.3, 1.0, 0.2)),
                (Vec3::new(0.2, 0.5, 0.1), Vec3::new(1.0, 0.4, 0.3)),
                (Vec3::new(0.1, 0.3, 0.0), Vec3::new(0.9, -0.2, 0.7)),
                (Vec3::new(0.0, 0.2, 0.3), Vec3::new(0.0, 0.3, 1.0)),
            ] {
                let o = o * s;
                let plane = topo::test_support::split_plane(Point3::new(o.x, o.y, o.z), n, tol());
                t.runs += 1;
                match topo::split(body, &plane, tol()) {
                    Ok(res) => {
                        for (side, part, sgn) in [("above", &res.above, 1.0), ("below", &res.below, -1.0)] {
                            let Some(b) = part.body() else { continue };
                            // edges against max(cone, half-space) directly
                            let mut worst = 0.0f64;
                            let mut digest = Vec::new();
                            for (_, e) in b.edges() {
                                let Some(c) = b.get_curve_geom(e.curve).and_then(|g| g.certified()) else { continue };
                                let (t0, t1) = c.params();
                                for k in 0..=8 {
                                    let p = c.carrier().eval(t0 + (t1 - t0) * f64::from(k) / 8.0);
                                    let v = Vec3::new(p.x, p.y, p.z);
                                    let half = -sgn * (v - o).dot(n) / n.norm();
                                    worst = worst.max(cone_f(v, top).max(half).abs());
                                    digest.push(format!("{:?},{:?},{:?}", p.x, p.y, p.z));
                                }
                            }
                            digest.sort();
                            let mut hh = std::collections::hash_map::DefaultHasher::new();
                            digest.hash(&mut hh);
                            t.built += 1;
                            let bad = worst > 1e-7 * s;
                            if bad {
                                t.bad_edge += 1;
                            }
                            let v = topo::mass_properties(b, tol()).map(|p| p.volume);
                            out.push(format!(
                                "split {name} s={s:e} {n:?} {side}\tBUILT\thash={:016x}\tvol={v:?}\tworst={worst:.2e}\t{}",
                                hh.finish(),
                                if bad { "EDGE_OFF_BOUNDARY" } else { "ok" }
                            ));
                        }
                    }
                    Err(e) => {
                        t.refused += 1;
                        out.push(format!("split {name} s={s:e} {n:?}\tREFUSED\t{}", format!("{e:?}").chars().take(140).collect::<String>()));
                    }
                }
            }
        }
        // a boolean result split again: cylinder less a turned box
        let (cb, cs) = cyl(1.0 * s, 0.0, 0.0, 0.0, 2.0 * s);
        let (bx, sbx) = brick_turned((0.2 * s, 3.0 * s), (-0.4 * s, 0.7 * s), (0.6 * s, 1.5 * s), Vec3::new(0.0, 1.0, 0.0), 0.3);
        if let Ok(Some(r1)) = run(Op::D, &cb, &bx) {
            let s1 = S::Sub(Box::new(cs.clone()), Box::new(sbx.clone()));
            for (o, n) in [(Vec3::new(0.0, 0.0, 1.0), Vec3::new(0.5, 0.2, 1.0)), (Vec3::new(0.3, 0.0, 1.2), Vec3::new(1.0, 1.0, 0.4))] {
                let o = o * s;
                let plane = topo::test_support::split_plane(Point3::new(o.x, o.y, o.z), n, tol());
                t.runs += 1;
                match topo::split(&r1, &plane, tol()) {
                    Ok(res) => {
                        for (side, part, sgn) in [("above", &res.above, -1.0), ("below", &res.below, 1.0)] {
                            if let Some(b) = part.body() {
                                let or = S::Inter(Box::new(s1.clone()), Box::new(S::Half(o, n * sgn)));
                                out.push(check(&format!("split cyl∖box s={s:e} {n:?} {side}"), b, &or, s, &mut t));
                            }
                        }
                    }
                    Err(e) => {
                        t.refused += 1;
                        out.push(format!("split cyl∖box s={s:e} {n:?}\tREFUSED\t{}", format!("{e:?}").chars().take(140).collect::<String>()));
                    }
                }
            }
        }
    }
    write_out("cone", &out, &t);
}

/// The bodies behind the phase-1 cross-check's DISAGREE rows that build
/// on the base: the round boss ∪ slab (the intermediate of
/// `reach_slab_cut_sector_side`'s order [boss, slab, plate]) and the
/// cylinder less a pierced ball (`r1_diag_cylinder_pierces`), against the
/// closed form, every op, both orders.
#[test]
fn zz_probe_disagree_bodies() {
    let mut out = Vec::new();
    let mut t = Tally::default();
    let mut keep = Vec::new();
    let (r, cx, cy) = (0.6, 1.5, 1.0);
    let b = (core::f64::consts::PI / 6.0).tan(); // a third of a turn per arc
    let pts: Vec<(Point2<f64>, f64)> = (0..3)
        .map(|k| {
            let a = f64::from(k) * 2.0 * core::f64::consts::PI / 3.0;
            (Point2::new(cx + r * a.cos(), cy + r * a.sin()), b)
        })
        .collect();
    let boss = sweep::test_support::prism_at(pts, 0.44, 1.8, tol());
    let sboss = S::Cyl(cx, cy, r, 0.44, 2.24);
    let (slab, sslab) = brick_turned((1.4, 1.6), (-1.0, 3.0), (0.5, 2.0), Vec3::unit_z(), 0.0);
    all_ops(&mut out, &mut t, "boss/slab", &boss, &sboss, &slab, &sslab, 1.0, &mut keep);
    let (cylb, scyl) = cyl(1.0, 0.0, 0.0, 0.0, 1.0);
    let phi = 33.75f64.to_radians();
    for (name, ball, c) in [
        ("z-ball on axis", sweep::test_support::ball_poled_z(0.16, Vec3::new(0.0, 0.0, 1.0), tol()), Vec3::new(0.0, 0.0, 1.0)),
        ("z-ball 0.75@33.75", sweep::test_support::ball_poled_z(0.16, Vec3::new(0.75 * phi.cos(), 0.75 * phi.sin(), 1.0), tol()), Vec3::new(0.75 * phi.cos(), 0.75 * phi.sin(), 1.0)),
        ("z-ball 0.5@0", sweep::test_support::ball_poled_z(0.16, Vec3::new(0.5, 0.0, 1.0), tol()), Vec3::new(0.5, 0.0, 1.0)),
        ("z-ball 0.5@90", sweep::test_support::ball_poled_z(0.16, Vec3::new(0.0, 0.5, 1.0), tol()), Vec3::new(0.0, 0.5, 1.0)),
        ("y-ball 0.75@33.75", sweep::test_support::ball_poled_y(0.16, Vec3::new(0.75 * phi.cos(), 0.75 * phi.sin(), 1.0), tol()), Vec3::new(0.75 * phi.cos(), 0.75 * phi.sin(), 1.0)),
        ("z-ball rim", sweep::test_support::ball_poled_z(0.16, Vec3::new(1.0, 0.0, 1.0), tol()), Vec3::new(1.0, 0.0, 1.0)),
        ("z-ball wall", sweep::test_support::ball_poled_z(0.16, Vec3::new(0.0, 1.0, 0.5), tol()), Vec3::new(0.0, 1.0, 0.5)),
    ] {
        all_ops(&mut out, &mut t, &format!("cyl/{name}"), &cylb, &scyl, &ball, &S::Ball(c, 0.16), 1.0, &mut keep);
    }
    write_out("dis", &out, &t);
}
