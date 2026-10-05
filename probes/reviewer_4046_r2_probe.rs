//! Reviewer probe (PR #4046, lane reach-dual4046-r2). Not a permanent row.
//!
//! Every carved body is built through the public API and read against an
//! INDEPENDENT oracle: a CSG signed-depth function over the balls and
//! (rotated) boxes it was built from — a conservative distance bound, so
//! `|f| > thr` means the point really is that far from the boundary — and
//! volumes against an exact-interval slice integral along z-lines.
//! Run: `cargo nextest run -p sweep --test all reviewer_4046 --no-capture`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, dead_code)]

use geom_core::{Affine3, Band, Point2, Point3, Tol, Vec3};
use sweep::Revolution;
use sweep::test_support::{brick, finished, revolved_about_y};
use topo::{AtRestBody, BooleanOp, SolidContainment, point_in_solid};

#[derive(Clone, Debug)]
enum Csg {
    Ball(Vec3<f64>, f64),
    /// Axis box `lo..hi` in its own frame, placed by `place` (rigid).
    Box(Vec3<f64>, Vec3<f64>, Affine3<f64>),
    U(Box<Csg>, Box<Csg>),
    I(Box<Csg>, Box<Csg>),
    S(Box<Csg>, Box<Csg>),
}

fn p3(v: Vec3<f64>) -> Point3<f64> {
    Point3::new(v.x, v.y, v.z)
}

impl Csg {
    fn f(&self, q: Point3<f64>) -> f64 {
        match self {
            Csg::Ball(c, r) => r - (q - p3(*c)).norm(),
            Csg::Box(lo, hi, place) => {
                let l = place.inverse().transform_point(q);
                (l.x - lo.x)
                    .min(hi.x - l.x)
                    .min(l.y - lo.y)
                    .min(hi.y - l.y)
                    .min(l.z - lo.z)
                    .min(hi.z - l.z)
            }
            Csg::U(a, b) => a.f(q).max(b.f(q)),
            Csg::I(a, b) => a.f(q).min(b.f(q)),
            Csg::S(a, b) => a.f(q).min(-b.f(q)),
        }
    }

    /// The material on the z-line through `(x, y)`, as sorted disjoint intervals.
    fn line(&self, x: f64, y: f64) -> Vec<(f64, f64)> {
        match self {
            Csg::Ball(c, r) => {
                let rho2 = (x - c.x).powi(2) + (y - c.y).powi(2);
                if rho2 >= r * r {
                    vec![]
                } else {
                    let h = (r * r - rho2).sqrt();
                    vec![(c.z - h, c.z + h)]
                }
            }
            Csg::Box(lo, hi, place) => {
                let inv = place.inverse();
                let o = inv.transform_point(Point3::new(x, y, 0.0));
                let d = inv.transform_vec(Vec3::new(0.0, 0.0, 1.0));
                let (mut t0, mut t1) = (f64::NEG_INFINITY, f64::INFINITY);
                for (oi, di, l, h) in [
                    (o.x, d.x, lo.x, hi.x),
                    (o.y, d.y, lo.y, hi.y),
                    (o.z, d.z, lo.z, hi.z),
                ] {
                    if di.abs() < 1e-300 {
                        if oi < l || oi > h {
                            return vec![];
                        }
                    } else {
                        let (a, b) = ((l - oi) / di, (h - oi) / di);
                        t0 = t0.max(a.min(b));
                        t1 = t1.min(a.max(b));
                    }
                }
                if t0 < t1 { vec![(t0, t1)] } else { vec![] }
            }
            Csg::U(a, b) => combine(&a.line(x, y), &b.line(x, y), |p, q| p || q),
            Csg::I(a, b) => combine(&a.line(x, y), &b.line(x, y), |p, q| p && q),
            Csg::S(a, b) => combine(&a.line(x, y), &b.line(x, y), |p, q| p && !q),
        }
    }
}

fn combine(a: &[(f64, f64)], b: &[(f64, f64)], op: impl Fn(bool, bool) -> bool) -> Vec<(f64, f64)> {
    let mut cuts: Vec<f64> = a.iter().chain(b).flat_map(|&(l, h)| [l, h]).collect();
    cuts.sort_by(f64::total_cmp);
    let inside = |s: &[(f64, f64)], t: f64| s.iter().any(|&(l, h)| l < t && t < h);
    let mut out: Vec<(f64, f64)> = vec![];
    for w in cuts.windows(2) {
        if w[1] <= w[0] {
            continue;
        }
        let m = 0.5 * (w[0] + w[1]);
        if op(inside(a, m), inside(b, m)) {
            match out.last_mut() {
                Some(last) if last.1 == w[0] => last.1 = w[1],
                _ => out.push((w[0], w[1])),
            }
        }
    }
    out
}

/// Slice integral of the CSG's volume over the box `lo..hi` (x, y), `n`² midpoint cells.
fn oracle_volume(c: &Csg, lo: (f64, f64), hi: (f64, f64), n: usize) -> f64 {
    let (dx, dy) = ((hi.0 - lo.0) / n as f64, (hi.1 - lo.1) / n as f64);
    let mut v = 0.0;
    for i in 0..n {
        let x = lo.0 + (i as f64 + 0.5) * dx;
        for j in 0..n {
            let y = lo.1 + (j as f64 + 0.5) * dy;
            v += c.line(x, y).iter().map(|(l, h)| h - l).sum::<f64>();
        }
    }
    v * dx * dy
}

fn ball(r: f64, c: Vec3<f64>) -> AtRestBody<f64> {
    let b = revolved_about_y(
        vec![(Point2::new(0.0, -r), 1.0), (Point2::new(0.0, r), 0.0)],
        Revolution::Full,
        Tol::witness(),
    );
    let b = topo::transform_rigid(&b, &Affine3::translation(c), Tol::witness()).unwrap();
    finished("ball", b, Tol::witness())
}

fn boxb(lo: Vec3<f64>, hi: Vec3<f64>, place: Affine3<f64>) -> AtRestBody<f64> {
    let b = brick((lo.x, hi.x), (lo.y, hi.y), (lo.z, hi.z), Tol::witness());
    let b = topo::transform_rigid(&b, &place, Tol::witness()).unwrap();
    finished("box", b, Tol::witness())
}

fn run(op: BooleanOp, a: &AtRestBody<f64>, b: &AtRestBody<f64>) -> Result<Option<AtRestBody<f64>>, String> {
    let t = Tol::witness();
    let r = match op {
        BooleanOp::Union => topo::boolean::union(a, b, t),
        BooleanOp::Intersect => topo::boolean::intersect(a, b, t),
        BooleanOp::Subtract => topo::boolean::subtract(a, b, t),
    }
    .map_err(|e| format!("{e:?}"))?;
    Ok(r.body().map(|bb| bb.body.clone()))
}

fn csg_op(op: BooleanOp, a: &Csg, b: &Csg) -> Csg {
    let (a, b) = (Box::new(a.clone()), Box::new(b.clone()));
    match op {
        BooleanOp::Union => Csg::U(a, b),
        BooleanOp::Intersect => Csg::I(a, b),
        BooleanOp::Subtract => Csg::S(a, b),
    }
}

/// A tiny deterministic generator.
struct Rng(u64);
impl Rng {
    fn next(&mut self) -> f64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        (self.0 >> 11) as f64 / (1u64 << 53) as f64
    }
    fn unit(&mut self) -> Vec3<f64> {
        loop {
            let v = Vec3::new(2.0 * self.next() - 1.0, 2.0 * self.next() - 1.0, 2.0 * self.next() - 1.0);
            let n = v.norm();
            if n > 0.1 && n < 1.0 {
                return v / n;
            }
        }
    }
}

#[derive(Default, Debug)]
struct Tally {
    checked: usize,
    wrong: Vec<String>,
    refused: Vec<String>,
    refused_far: Vec<String>,
    on_far: Vec<String>,
}

fn eps() -> f64 {
    Tol::witness().eps()
}

/// Classify `q` and hold it to the oracle. `scale` sets the thresholds.
fn check(t: &mut Tally, label: &str, body: &AtRestBody<f64>, csg: &Csg, q: Point3<f64>, scale: f64) {
    let tol = Tol::witness();
    let band = Band::linear(tol).unwrap();
    let f = csg.f(q);
    // Tolerances in this kernel are absolute metres; the scale only sets where we look.
    let decisive = 1000.0 * eps();
    let _ = scale;
    match point_in_solid(body, q, band, tol) {
        Err(e) => {
            if f.abs() > decisive {
                t.refused_far.push(format!("{label} {q:?} f={f:.3e}: {e:?}"));
            }
            t.refused.push(format!("{label} {q:?} f={f:.3e}: {e:?}"))
        }
        Ok(got) => {
            t.checked += 1;
            let want = if f > 0.0 { SolidContainment::In } else { SolidContainment::Out };
            if f.abs() > decisive {
                if got == SolidContainment::OnBoundary {
                    t.on_far.push(format!("{label} {q:?} f={f:.3e}: OnBoundary"));
                } else if got != want {
                    t.wrong.push(format!("{label} {q:?} f={f:.3e}: got {got:?}"));
                }
            } else if f.abs() > 20.0 * eps() && got != want && got != SolidContainment::OnBoundary {
                t.wrong.push(format!("{label} {q:?} f={f:.3e} (near): got {got:?}"));
            }
        }
    }
}

/// Points to probe near: body vertices, every ball's poles (y) and the
/// antipodes of every vertex on every ball.
fn specials(body: &AtRestBody<f64>, balls: &[(Vec3<f64>, f64)]) -> Vec<Point3<f64>> {
    let verts: Vec<Point3<f64>> = body.vertex_points().map(|(_, p)| p).collect();
    let mut out = verts.clone();
    for &(c, r) in balls {
        out.push(p3(c) + Vec3::new(0.0, r, 0.0));
        out.push(p3(c) - Vec3::new(0.0, r, 0.0));
        for v in &verts {
            let w = *v - p3(c);
            if (w.norm() - r).abs() < 1e-9 * r.max(1.0) {
                out.push(p3(c) - w);
            }
        }
    }
    out
}

/// Random lattice points plus near-special probes, on and off the spheres.
fn probe_body(label: &str, body: &AtRestBody<f64>, csg: &Csg, balls: &[(Vec3<f64>, f64)], lo: Vec3<f64>, hi: Vec3<f64>, scale: f64) -> Tally {
    let mut t = Tally::default();
    let mut rng = Rng(0x9E37_79B9_7F4A_7C15);
    for _ in 0..300 {
        let q = Point3::new(
            lo.x + (hi.x - lo.x) * rng.next(),
            lo.y + (hi.y - lo.y) * rng.next(),
            lo.z + (hi.z - lo.z) * rng.next(),
        );
        check(&mut t, label, body, csg, q, scale);
    }
    let sp = specials(body, balls);
    for s in &sp {
        for d in [1e-3, 1e-5, 1e-7, 1e-9, 1e-11, 1e-12] {
            for _ in 0..3 {
                let u = rng.unit();
                check(&mut t, &format!("{label} near {s:?} d={d:e}"), body, csg, *s + u * (d * scale), scale);
                // On a sphere through s: slide tangentially and project back.
                for &(c, r) in balls {
                    let w = *s - p3(c);
                    if (w.norm() - r).abs() > 1e-9 * r.max(1.0) {
                        continue;
                    }
                    let n = w / w.norm();
                    let tan = u - n * n.dot(u);
                    let tan = tan / tan.norm();
                    let moved = w + tan * (d * scale);
                    let q = p3(c) + moved * (r / moved.norm());
                    check(&mut t, &format!("{label} on-sphere near {s:?} d={d:e}"), body, csg, q, scale);
                }
            }
        }
    }
    // Points on each sphere, just off it, random over the whole sphere (face interiors, arcs).
    for &(c, r) in balls {
        for _ in 0..200 {
            let u = rng.unit();
            for off in [0.0, 1e-6 * scale, -1e-6 * scale] {
                check(&mut t, &format!("{label} sphere r{r}"), body, csg, p3(c) + u * (r + off), scale);
            }
        }
    }
    t
}

fn report(label: &str, t: &Tally) -> usize {
    eprintln!(
        "[{label}] eps {:e}: checked {}, WRONG {}, refused {} (decisive-distance {}), OnBoundary-far {}",
        eps(),
        t.checked,
        t.wrong.len(),
        t.refused.len(),
        t.refused_far.len(),
        t.on_far.len()
    );
    for w in t.wrong.iter().take(6) {
        eprintln!("   WRONG {w}");
    }
    for w in t.on_far.iter().take(3) {
        eprintln!("   ONFAR {w}");
    }
    for w in t.refused_far.iter().take(4) {
        eprintln!("   REFUSED {w}");
    }
    t.wrong.len()
}

struct Fixture {
    label: String,
    body: AtRestBody<f64>,
    csg: Csg,
    balls: Vec<(Vec3<f64>, f64)>,
    lo: Vec3<f64>,
    hi: Vec3<f64>,
    scale: f64,
}

fn rot(axis: Vec3<f64>, angle: f64, about: Vec3<f64>) -> Affine3<f64> {
    Affine3::rotation_about_axis(p3(about), axis / axis.norm(), angle)
}

fn fixtures(s: f64) -> Vec<Fixture> {
    let mut out = vec![];
    let v = |x: f64, y: f64, z: f64| Vec3::new(x * s, y * s, z * s);
    let mut push = |label: String, r: Result<Option<AtRestBody<f64>>, String>, csg: Csg, balls: Vec<(Vec3<f64>, f64)>, lo: Vec3<f64>, hi: Vec3<f64>| match r {
        Ok(Some(body)) => out.push(Fixture { label, body, csg, balls, lo, hi, scale: s }),
        Ok(None) => eprintln!("[{label}] EMPTY"),
        Err(e) => eprintln!("[{label}] BUILD REFUSED: {e}"),
    };
    // Lens unions at several separations and radii, along x and through A's pole.
    let a = v(2.0, 2.0, 0.5);
    for (sep_dir, sep, rb) in [
        (Vec3::new(1.0, 0.0, 0.0), 1.4, 1.0),
        (Vec3::new(1.0, 0.0, 0.0), 0.3, 1.0),
        (Vec3::new(1.0, 0.0, 0.0), 1.95, 1.0),
        (Vec3::new(0.6, 0.0, 0.8), 1.2, 0.6),
        (Vec3::new(0.51f64.sqrt(), 0.7, 0.0), 1.4, 1.0), // radical circle through A's +y pole
        (Vec3::new(0.3, 0.9, 0.3), 1.0, 0.5),
    ] {
        let d = sep_dir / sep_dir.norm();
        let b = a + d * (sep * s);
        for op in [BooleanOp::Union, BooleanOp::Intersect, BooleanOp::Subtract] {
            let csg = csg_op(op, &Csg::Ball(a, s), &Csg::Ball(b, rb * s));
            let r = run(op, &ball(s, a), &ball(rb * s, b));
            push(
                format!("s{s:e} {op:?} sep{sep} dir{:.2},{:.2},{:.2} rb{rb}", d.x, d.y, d.z),
                r,
                csg,
                vec![(a, s), (b, rb * s)],
                a - v(1.2, 1.2, 1.2),
                a + v(3.2, 3.2, 3.2),
            );
        }
    }
    // Three balls in a row: the middle sphere keeps two loops (outer + ring).
    {
        let (b0, b1, b2) = (v(0.0, 0.0, 0.0), v(1.5, 0.1, 0.0), v(-1.5, 0.0, 0.2));
        let u1 = run(BooleanOp::Union, &ball(s, b0), &ball(0.8 * s, b1)).unwrap().unwrap();
        let csg1 = Csg::U(Box::new(Csg::Ball(b0, s)), Box::new(Csg::Ball(b1, 0.8 * s)));
        for op in [BooleanOp::Union, BooleanOp::Subtract] {
            let r = run(op, &u1, &ball(0.8 * s, b2));
            push(format!("s{s:e} row3 {op:?}"), r, csg_op(op, &csg1, &Csg::Ball(b2, 0.8 * s)), vec![(b0, s), (b1, 0.8 * s), (b2, 0.8 * s)], v(-2.5, -1.2, -1.2), v(2.5, 1.2, 1.2));
        }
        // Triangle of balls: triple points (vertices on the spheres).
        let b3 = v(0.7, 0.5, 0.9);
        let csg3 = Csg::U(Box::new(csg1.clone()), Box::new(Csg::Ball(b3, 0.8 * s)));
        let r = run(BooleanOp::Union, &u1, &ball(0.8 * s, b3));
        push(format!("s{s:e} tri-union"), r, csg3, vec![(b0, s), (b1, 0.8 * s), (b3, 0.8 * s)], v(-1.2, -1.2, -1.2), v(2.5, 2.0, 2.0));
        let r = run(BooleanOp::Subtract, &u1, &ball(0.8 * s, b3));
        let csg4 = Csg::S(Box::new(csg1.clone()), Box::new(Csg::Ball(b3, 0.8 * s)));
        push(format!("s{s:e} tri-sub"), r, csg4, vec![(b0, s), (b1, 0.8 * s), (b3, 0.8 * s)], v(-1.2, -1.2, -1.2), v(2.5, 2.0, 2.0));
    }
    // A ball against rotated boxes: reflex corners, lunes, tilted cuts.
    let o = v(0.0, 0.0, 0.0);
    for (name, lo, hi, place) in [
        ("corner-in", v(0.2, 0.1, -0.3), v(2.0, 2.0, 2.0), rot(Vec3::new(1.0, 2.0, 3.0), 0.7, o)),
        ("wedge-through-centre", v(0.0, -2.0, 0.0), v(2.0, 2.0, 2.0), rot(Vec3::new(1.0, 0.0, 0.0), 0.5, o)),
        ("wedge-oblique", v(0.0, -2.0, 0.0), v(2.0, 2.0, 2.0), rot(Vec3::new(0.3, 1.0, 0.2), 1.1, o)),
        ("slab-near-tangent", v(0.999, -2.0, -2.0), v(2.0, 2.0, 2.0), rot(Vec3::new(0.0, 0.0, 1.0), 0.4, o)),
        ("pole-corner", v(-0.3, 0.6, -0.3), v(0.3, 2.0, 0.3), Affine3::identity()),
        ("thin-sliver", v(0.0, -2.0, 0.0), v(2.0, 2.0, 2.0), rot(Vec3::new(1.0, 0.2, 0.0), 0.02, o)),
    ] {
        let lo = lo;
        let hi = hi;
        let place_s = place; // translation part is zero (rotation about origin), scale-safe
        for op in [BooleanOp::Union, BooleanOp::Intersect, BooleanOp::Subtract] {
            let csg = csg_op(op, &Csg::Ball(o, s), &Csg::Box(lo, hi, place_s));
            let r = run(op, &ball(s, o), &boxb(lo, hi, place_s));
            push(format!("s{s:e} ball {op:?} box[{name}]"), r, csg, vec![(o, s)], v(-1.3, -1.3, -1.3), v(1.3, 1.3, 1.3));
        }
        // and box ∖ ball
        let csg = Csg::S(Box::new(Csg::Box(lo, hi, place_s)), Box::new(Csg::Ball(o, s)));
        let r = run(BooleanOp::Subtract, &boxb(lo, hi, place_s), &ball(s, o));
        push(format!("s{s:e} box[{name}] - ball"), r, csg, vec![(o, s)], v(-2.5, -2.5, -2.5), v(2.5, 2.5, 2.5));
    }
    // A bowl: the cavity face of a crossing subtraction has sense false.
    {
        let c2 = v(0.8, 0.3, 0.1);
        let csg = Csg::S(Box::new(Csg::Ball(o, s)), Box::new(Csg::Ball(c2, 0.6 * s)));
        let r = run(BooleanOp::Subtract, &ball(s, o), &ball(0.6 * s, c2));
        push(format!("s{s:e} bowl"), r, csg, vec![(o, s), (c2, 0.6 * s)], v(-1.2, -1.2, -1.2), v(1.5, 1.2, 1.2));
    }
    out
}

#[test]
fn reviewer_4046_point_classification() {
    let mut wrong = 0;
    for s in [1.0, 1e-3, 1e3] {
        for fx in fixtures(s) {
            let t = probe_body(&fx.label, &fx.body, &fx.csg, &fx.balls, fx.lo, fx.hi, fx.scale);
            wrong += report(&fx.label, &t);
        }
    }
    assert_eq!(wrong, 0, "wrong classifications");
}

/// Each fixture reused as an operand against a third ball (nested,
/// disjoint, crossing), every op, both orders; volume against the slice
/// integral, and the result's own points against the oracle.
#[test]
fn reviewer_4046_reuse_volumes() {
    let mut bad = vec![];
    let tol = Tol::witness();
    for s in [1.0] {
        for fx in fixtures(s) {
            let c = fx.lo + (fx.hi - fx.lo) * 0.5;
            // the third balls: one at the body's own interior point near centre, one far, one crossing
            let mut rng = Rng(7);
            let mut thirds = vec![(Vec3::new(4.2 * s, 4.2 * s, 4.2 * s).max(fx.hi + Vec3::new(0.5 * s, 0.5 * s, 0.5 * s)), 0.3 * s)];
            for _ in 0..200 {
                let q = fx.lo + (fx.hi - fx.lo) * 0.0 + Vec3::new(
                    (fx.hi.x - fx.lo.x) * rng.next(),
                    (fx.hi.y - fx.lo.y) * rng.next(),
                    (fx.hi.z - fx.lo.z) * rng.next(),
                );
                let f = fx.csg.f(p3(q));
                if f > 0.25 * s && thirds.len() < 2 {
                    thirds.push((q, 0.8 * f)); // nested
                }
            }
            thirds.push((c, 0.45 * s)); // likely crossing
            for (tc, tr) in thirds {
                let third = ball(tr, tc);
                let tcsg = Csg::Ball(tc, tr);
                for (name, op, swap) in [
                    ("F∪T", BooleanOp::Union, false),
                    ("T∪F", BooleanOp::Union, true),
                    ("F∩T", BooleanOp::Intersect, false),
                    ("T∩F", BooleanOp::Intersect, true),
                    ("F∖T", BooleanOp::Subtract, false),
                    ("T∖F", BooleanOp::Subtract, true),
                ] {
                    let label = format!("{} | {name} third r{tr:.3} at {:.3},{:.3},{:.3}", fx.label, tc.x, tc.y, tc.z);
                    let (r, csg) = if swap {
                        (run(op, &third, &fx.body), csg_op(op, &tcsg, &fx.csg))
                    } else {
                        (run(op, &fx.body, &third), csg_op(op, &fx.csg, &tcsg))
                    };
                    // Rotated boxes reach |corner| = 2·√3 < 3.6 from the origin.
                    let lo = Vec3::new(fx.lo.x.min(tc.x - tr).min(-3.6 * s), fx.lo.y.min(tc.y - tr).min(-3.6 * s), 0.0);
                    let hi = Vec3::new(fx.hi.x.max(tc.x + tr).max(3.6 * s), fx.hi.y.max(tc.y + tr).max(3.6 * s), 0.0);
                    let want = oracle_volume(&csg, (lo.x, lo.y), (hi.x, hi.y), 1000);
                    match r {
                        Err(e) => eprintln!("[{label}] REFUSED {e}"),
                        Ok(None) => {
                            if want > 1e-3 * s.powi(3) {
                                bad.push(format!("{label}: EMPTY, oracle {want:.6}"));
                            }
                        }
                        Ok(Some(b)) => {
                            let v = topo::mass_properties(&b, tol).map(|m| m.volume);
                            let tiers = topo::validate_closed(&b)
                                .map_err(|e| format!("{e:?}"))
                                .and_then(|_| topo::validate_geometric_certificate(&b, tol).map_err(|e| format!("{e:?}")));
                            match v {
                                Ok(v) if (v - want).abs() <= 2e-3 * want.max(1e-2 * s.powi(3)) => {}
                                Ok(v) => bad.push(format!("{label}: volume {v:.6} oracle {want:.6}")),
                                Err(e) => bad.push(format!("{label}: mass refused {e:?}")),
                            }
                            if let Err(e) = tiers {
                                bad.push(format!("{label}: tiers {e}"));
                            }
                            // Points of the result against the oracle, and reuse once more.
                            let mut t = Tally::default();
                            let mut rng2 = Rng(11);
                            for _ in 0..60 {
                                let q = Point3::new(
                                    lo.x + (hi.x - lo.x) * rng2.next(),
                                    lo.y + (hi.y - lo.y) * rng2.next(),
                                    fx.lo.z.min(tc.z - tr) + (fx.hi.z.max(tc.z + tr) - fx.lo.z.min(tc.z - tr)) * rng2.next(),
                                );
                                check(&mut t, &label, &b, &csg, q, s);
                            }
                            if !t.wrong.is_empty() {
                                bad.push(format!("{label}: {} wrong points, e.g. {}", t.wrong.len(), t.wrong[0]));
                            }
                            if !t.refused.is_empty() {
                                eprintln!("[{label}] {} point refusals e.g. {}", t.refused.len(), t.refused[0]);
                            }
                        }
                    }
                }
            }
        }
    }
    for b in &bad {
        eprintln!("BAD {b}");
    }
    assert!(bad.is_empty(), "{} bad", bad.len());
}

/// Build-outcome census: every fixture and every reuse op, Ok(volume) or the
/// refusal's variant, for a differential against the merge-base.
#[test]
fn reviewer_4046_build_census() {
    let tol = Tol::witness();
    for s in [1.0, 1e-3, 1e3] {
        for fx in fixtures(s) {
            let c = fx.lo + (fx.hi - fx.lo) * 0.5;
            for (tc, tr) in [(c, 0.45 * s), (Vec3::new(4.2 * s, 4.2 * s, 4.2 * s).max(fx.hi + Vec3::new(0.5 * s, 0.5 * s, 0.5 * s)), 0.3 * s)] {
                let third = ball(tr, tc);
                for (name, op, swap) in [
                    ("F∪T", BooleanOp::Union, false), ("T∪F", BooleanOp::Union, true),
                    ("F∩T", BooleanOp::Intersect, false), ("T∩F", BooleanOp::Intersect, true),
                    ("F∖T", BooleanOp::Subtract, false), ("T∖F", BooleanOp::Subtract, true),
                ] {
                    let r = if swap { run(op, &third, &fx.body) } else { run(op, &fx.body, &third) };
                    let out = match r {
                        Ok(Some(b)) => format!("ok {:.9e}", topo::mass_properties(&b, tol).map(|m| m.volume).unwrap_or(f64::NAN)),
                        Ok(None) => "empty".into(),
                        Err(e) => format!("err {}", e.split(['{', '(']).next().unwrap_or("").trim()),
                    };
                    eprintln!("CENSUS {} | {name} r{tr:.3e} => {out}", fx.label);
                }
            }
        }
    }
}

/// Liveness gap (MINOR): a point on sphere A 1e-9 from its +y pole, 0.063
/// deep inside B (so In the union), refuses `bool_sphere_region_span`: face
/// 2v1 has a vertex at A's −y pole (two meridian edges meet there), so every
/// forward ray meets that antipodal vertex first at s = π, and the rays
/// aimed at the meridians run along them. Asserts the refusal (flip to
/// `Ok(In)` once read).
#[test]
fn reviewer_4046_antipodal_vertex_refuses() {
    let a = Vec3::new(2.0, 2.0, 0.5);
    let d = Vec3::new(0.3, 0.9, 0.3);
    let b = a + d / d.norm();
    let u = run(BooleanOp::Union, &ball(1.0, a), &ball(0.5, b)).unwrap().unwrap();
    let q = Point3::new(1.999_999_999_083_450_5, 3.0, 0.499_999_999_600_078_9);
    assert!(Csg::U(Box::new(Csg::Ball(a, 1.0)), Box::new(Csg::Ball(b, 0.5))).f(q) > 0.06);
    let tol = Tol::witness();
    let got = point_in_solid(&u, q, Band::linear(tol).unwrap(), tol);
    eprintln!("antipodal-vertex probe at eps {:e}: {got:?}", eps());
    if (eps() - 1e-9).abs() < 1e-15 {
        assert!(got.is_err(), "now answers: {got:?}");
    }
}
