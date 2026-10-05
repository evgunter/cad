//! Review probe for PR #4042 (reach-full4042): the conic × quadric door
//! against an INDEPENDENT oracle — the true Euclidean distance to the
//! quadric, sampled densely along the carrier, every local extremum
//! golden-section refined, every sign change bisected to the bit.
//!
//! Installed as `crates/topo/src/boolean/conic_quadric/probe_review.rs`
//! on head (door = `conic_quadric_roots`), or as
//! `crates/topo/src/boolean/probe_review.rs` on main with `door_call`
//! swapped for the old per-kind doors (`probes/install.sh`). Writes one
//! CSV line per pose to `$PROBE_OUT`, same poses in the same order on
//! both trees, and panics on nothing: wrong answers are counted.

#![allow(clippy::unwrap_used, clippy::panic, clippy::too_many_lines)]

use std::fmt::Write as _;

use geom_core::{Band, Point3, Tol, Vec3};

use super::BooleanError;
use super::circle_roots::CircleRoots;

// DOOR_CALL_BEGIN
fn door_call(
    c: &geom::Curve3<f64>,
    t0: f64,
    t1: f64,
    s: &geom::Surface<f64>,
    band: Band,
) -> Result<CircleRoots<f64>, BooleanError> {
    super::conic_quadric_roots(c, t0, t1, s, band)
}
// DOOR_CALL_END

type V = [f64; 3];
fn add(a: V, b: V) -> V {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}
fn sub(a: V, b: V) -> V {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}
fn mul(a: V, k: f64) -> V {
    [a[0] * k, a[1] * k, a[2] * k]
}
fn dot(a: V, b: V) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}
fn cross(a: V, b: V) -> V {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}
fn unit(a: V) -> V {
    mul(a, 1.0 / dot(a, a).sqrt())
}
/// `a` rotated by `ang` about unit `k` (Rodrigues).
fn rot(a: V, k: V, ang: f64) -> V {
    let (s, c) = ang.sin_cos();
    add(
        add(mul(a, c), mul(cross(k, a), s)),
        mul(k, dot(k, a) * (1.0 - c)),
    )
}


/// Double-double arithmetic for the oracle's distance: the inputs are
/// exact f64s, so every product and sum is carried to ~106 bits.
#[derive(Clone, Copy, Debug)]
struct Dd(f64, f64);
fn two_sum(a: f64, b: f64) -> Dd {
    let s = a + b;
    let bb = s - a;
    Dd(s, (a - (s - bb)) + (b - bb))
}
fn two_prod(a: f64, b: f64) -> Dd {
    let p = a * b;
    Dd(p, a.mul_add(b, -p))
}
impl Dd {
    fn f(x: f64) -> Dd {
        Dd(x, 0.0)
    }
    fn add(self, o: Dd) -> Dd {
        let s = two_sum(self.0, o.0);
        let t = two_sum(self.1, o.1);
        let s = two_sum(s.0, s.1 + t.0);
        two_sum(s.0, s.1 + t.1)
    }
    fn neg(self) -> Dd {
        Dd(-self.0, -self.1)
    }
    fn sub(self, o: Dd) -> Dd {
        self.add(o.neg())
    }
    fn mul(self, o: Dd) -> Dd {
        let p = two_prod(self.0, o.0);
        two_sum(p.0, p.1 + (self.0 * o.1 + self.1 * o.0))
    }
    fn div(self, o: Dd) -> Dd {
        let q1 = self.0 / o.0;
        let r = self.sub(o.mul(Dd::f(q1)));
        let q2 = r.0 / o.0;
        let r = r.sub(o.mul(Dd::f(q2)));
        let q3 = r.0 / o.0;
        two_sum(q1, q2).add(Dd::f(q3))
    }
    fn sqrt(self) -> Dd {
        if self.0 <= 0.0 {
            return Dd::f(0.0);
        }
        let x = Dd::f(self.0.sqrt());
        // Two Newton steps: x ← (x + a/x)/2.
        let x = x.add(self.div(x)).mul(Dd::f(0.5));
        x.add(self.div(x)).mul(Dd::f(0.5))
    }
    fn v(self) -> f64 {
        self.0 + self.1
    }
}
type Dv = [Dd; 3];
fn dv(a: V) -> Dv {
    [Dd::f(a[0]), Dd::f(a[1]), Dd::f(a[2])]
}
fn dadd(a: Dv, b: Dv) -> Dv {
    [a[0].add(b[0]), a[1].add(b[1]), a[2].add(b[2])]
}
fn dsub(a: Dv, b: Dv) -> Dv {
    [a[0].sub(b[0]), a[1].sub(b[1]), a[2].sub(b[2])]
}
fn dscale(a: Dv, k: Dd) -> Dv {
    [a[0].mul(k), a[1].mul(k), a[2].mul(k)]
}
fn ddot(a: Dv, b: Dv) -> Dd {
    a[0].mul(b[0]).add(a[1].mul(b[1])).add(a[2].mul(b[2]))
}
fn dcross(a: Dv, b: Dv) -> Dv {
    [
        a[1].mul(b[2]).sub(a[2].mul(b[1])),
        a[2].mul(b[0]).sub(a[0].mul(b[2])),
        a[0].mul(b[1]).sub(a[1].mul(b[0])),
    ]
}

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> f64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        (self.0 >> 11) as f64 / (1u64 << 53) as f64
    }
    fn pick<T: Copy>(&mut self, xs: &[T]) -> T {
        xs[((self.next() * xs.len() as f64) as usize).min(xs.len() - 1)]
    }
    fn dir(&mut self) -> V {
        loop {
            let v = [
                2.0 * self.next() - 1.0,
                2.0 * self.next() - 1.0,
                2.0 * self.next() - 1.0,
            ];
            let n = dot(v, v);
            if n > 0.01 && n < 1.0 {
                return unit(v);
            }
        }
    }
}

fn perp(n: V, r: &mut Rng) -> V {
    unit(cross(n, r.dir()))
}

#[derive(Clone, Copy, Debug)]
struct Conic {
    c: V,
    n: V,
    u: V,
    a: f64,
    b: f64,
    ellipse: bool,
}
impl Conic {
    fn at(&self, t: f64) -> V {
        let (s, co) = t.sin_cos();
        let v = cross(self.n, self.u);
        add(self.c, add(mul(self.u, self.a * co), mul(v, self.b * s)))
    }
    /// The point in double-double: `(cos, sin)` renormalized onto the
    /// unit circle, so the only error left is a ~1e-16 shift in `t`.
    fn at_dd(&self, t: f64) -> Dv {
        let (s, co) = t.sin_cos();
        let (s, co) = (Dd::f(s), Dd::f(co));
        let h = s.mul(s).add(co.mul(co)).sqrt();
        let (s, co) = (s.div(h), co.div(h));
        let (n, u) = (dv(self.n), dv(self.u));
        let v = dcross(n, u);
        dadd(
            dv(self.c),
            dadd(dscale(u, Dd::f(self.a).mul(co)), dscale(v, Dd::f(self.b).mul(s))),
        )
    }
    fn speed(&self, t: f64) -> f64 {
        let (s, co) = t.sin_cos();
        (self.a * s).hypot(self.b * co)
    }
    fn curve(&self) -> geom::Curve3<f64> {
        let p = Point3::new(self.c[0], self.c[1], self.c[2]);
        let vv = |x: V| Vec3::new(x[0], x[1], x[2]);
        if self.ellipse {
            geom::Curve3::Ellipse {
                center: p,
                axis: vv(self.n),
                major: self.a,
                minor: self.b,
                u_ref: vv(self.u),
            }
        } else {
            geom::Curve3::Circle {
                center: p,
                axis: vv(self.n),
                radius: self.a,
                u_ref: vv(self.u),
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Quad {
    Wall { o: V, k: V, r: f64 },
    Sphere { o: V, r: f64 },
}
impl Quad {
    /// The true signed distance (outside positive).
    fn dist(&self, p: V) -> f64 {
        match *self {
            Quad::Wall { o, k, r } => {
                let d = sub(p, o);
                let q = sub(d, mul(k, dot(d, k)));
                dot(q, q).sqrt() - r
            }
            Quad::Sphere { o, r } => {
                let d = sub(p, o);
                dot(d, d).sqrt() - r
            }
        }
    }
    fn dist_dd(&self, p: Dv) -> f64 {
        match *self {
            Quad::Wall { o, k, r } => {
                let d = dsub(p, dv(o));
                let k = dv(k);
                // `k` is unit only to f64 rounding: project with `k/|k|²`.
                let kk = ddot(k, k);
                let q = dsub(d, dscale(k, ddot(d, k).div(kk)));
                ddot(q, q).sqrt().sub(Dd::f(r)).v()
            }
            Quad::Sphere { o, r } => {
                let d = dsub(p, dv(o));
                ddot(d, d).sqrt().sub(Dd::f(r)).v()
            }
        }
    }
    fn normal(&self, p: V) -> V {
        match *self {
            Quad::Wall { o, k, .. } => {
                let d = sub(p, o);
                unit(sub(d, mul(k, dot(d, k))))
            }
            Quad::Sphere { o, .. } => unit(sub(p, o)),
        }
    }
    fn r(&self) -> f64 {
        match *self {
            Quad::Wall { r, .. } | Quad::Sphere { r, .. } => r,
        }
    }
    fn surface(&self, rng_u: V) -> geom::Surface<f64> {
        let vv = |x: V| Vec3::new(x[0], x[1], x[2]);
        match *self {
            Quad::Wall { o, k, r } => geom::Surface::Cylinder {
                origin: Point3::new(o[0], o[1], o[2]),
                axis: vv(k),
                radius: r,
                u_ref: vv(unit(cross(k, rng_u))),
            },
            Quad::Sphere { o, r } => geom::Surface::Sphere {
                center: Point3::new(o[0], o[1], o[2]),
                radius: r,
                axis: vv([0.0, 0.0, 1.0]),
                u_ref: vv([1.0, 0.0, 0.0]),
            },
        }
    }
}

/// The second harmonic's amplitude `A₂` of `(|⊥(C − o)|² − r²)/2r`, by
/// hand: `⊥(C − o) = P₀ + P₁ cos θ + P₂ sin θ`, so the cos 2θ and
/// sin 2θ coefficients are `(|P₁|² − |P₂|²)/4r` and `P₁·P₂/2r`.
fn a2_of(cn: &Conic, q: &Quad) -> f64 {
    let v = cross(cn.n, cn.u);
    let (p1, p2) = (mul(cn.u, cn.a), mul(v, cn.b));
    let (p1, p2, r) = match *q {
        Quad::Wall { k, r, .. } => (sub(p1, mul(k, dot(p1, k))), sub(p2, mul(k, dot(p2, k))), r),
        Quad::Sphere { r, .. } => (p1, p2, r),
    };
    ((dot(p1, p1) - dot(p2, p2)) / (4.0 * r)).hypot(dot(p1, p2) / (2.0 * r))
}

struct Oracle {
    roots: Vec<f64>,
    dmin: (f64, f64),
    dmax: (f64, f64),
}

fn golden(f: &dyn Fn(f64) -> f64, mut a: f64, mut b: f64, max: bool) -> f64 {
    let g = (5f64.sqrt() - 1.0) / 2.0;
    let s = if max { -1.0 } else { 1.0 };
    for _ in 0..90 {
        let x1 = b - g * (b - a);
        let x2 = a + g * (b - a);
        if s * f(x1) < s * f(x2) {
            b = x2;
        } else {
            a = x1;
        }
    }
    (a + b) / 2.0
}

/// The oracle on the closed turn `[t0, t0 + 2π)`.
fn oracle(cn: &Conic, q: &Quad, t0: f64, n: usize) -> Oracle {
    let f = |t: f64| q.dist_dd(cn.at_dd(t));
    let h = std::f64::consts::TAU / n as f64;
    let ts: Vec<f64> = (0..n).map(|k| t0 + h * k as f64).collect();
    let fs: Vec<f64> = ts.iter().map(|&t| f(t)).collect();
    let mut pts: Vec<(f64, f64)> = ts.iter().copied().zip(fs.iter().copied()).collect();
    let (mut dmin, mut dmax) = ((0.0, f64::INFINITY), (0.0, f64::NEG_INFINITY));
    let tau = std::f64::consts::TAU;
    for i in 0..n {
        let (p, c, nx) = (fs[(i + n - 1) % n], fs[i], fs[(i + 1) % n]);
        for max in [false, true] {
            let ext = if max { c >= p && c >= nx } else { c <= p && c <= nx };
            if ext {
                let t = golden(&f, ts[i] - h, ts[i] + h, max);
                let v = f(t);
                let t = t0 + (t - t0).rem_euclid(tau);
                pts.push((t, v));
                if !max && v < dmin.1 {
                    dmin = (t, v);
                }
                if max && v > dmax.1 {
                    dmax = (t, v);
                }
            }
        }
    }
    pts.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    // The closed turn: the last point pairs with the first, one turn on.
    let m = pts.len();
    let mut roots = Vec::new();
    for i in 0..m {
        let (mut a, fa) = pts[i];
        let (mut b, fb) = pts[(i + 1) % m];
        if i + 1 == m {
            b += tau;
        }
        if fa == 0.0 {
            roots.push(a);
            continue;
        }
        if fb == 0.0 || fa.signum() == fb.signum() {
            continue;
        }
        for _ in 0..200 {
            let mid = (a + b) / 2.0;
            if mid <= a || mid >= b {
                break;
            }
            if f(mid).signum() == fa.signum() {
                a = mid;
            } else {
                b = mid;
            }
        }
        roots.push(t0 + ((a + b) / 2.0 - t0).rem_euclid(tau));
    }
    Oracle { roots, dmin, dmax }
}

fn wrap(x: f64) -> f64 {
    let t = std::f64::consts::TAU;
    let y = x.rem_euclid(t);
    if y > t / 2.0 { y - t } else { y }
}

struct Pose {
    fam: &'static str,
    cn: Conic,
    q: Quad,
    note: String,
}

/// Translate the carrier so its (min|max) distance hits `target`.
fn regraze(cn: &mut Conic, q: &Quad, target: f64, use_max: bool) {
    for _ in 0..4 {
        let o = oracle(cn, q, 0.0, 20_000);
        let (t, d) = if use_max { o.dmax } else { o.dmin };
        let nrm = q.normal(cn.at(t));
        if !nrm.iter().all(|x| x.is_finite()) {
            return;
        }
        cn.c = add(cn.c, mul(nrm, target - d));
    }
}

fn make_pose(rng: &mut Rng, eps: f64) -> Pose {
    let r = rng.pick(&[1e-3, 1e-2, 1.0, 7.0, 100.0]);
    let o = mul(rng.dir(), r * rng.pick(&[0.0, 1.0, 10.0]));
    let rd = rng.dir();
    let k = rng.pick(&[[0.0, 0.0, 1.0], rd]);
    let reach = 2.0 * (r * eps).sqrt();
    let fam_ix = (rng.next() * 7.0) as usize;
    let graze = rng.pick(&[0usize, 0, 1, 2]);
    let depth = rng.pick(&[0.0, 0.5, -0.5, 2.0, -2.0, 20.0, -20.0, 1e3, -1e3, 1e-3]) * eps;
    let wall = Quad::Wall { o, k, r };
    let sph = Quad::Sphere { o, r };
    let mut note = String::new();
    let (fam, mut cn, q) = match fam_ix {
        // A circle near square to the wall's axis: tilt `ρ sin α` across
        // both bands (tilt band ε, A₂ reach 2√(rε)).
        0 => {
            let t = rng.pick(&[
                0.0, 1e-3, 0.1, 0.5, 0.9, 0.99, 1.0, 1.01, 1.1, 1.5, 2.0, 10.0, 100.0,
            ]);
            let tilt = rng.pick(&[t * reach, t * eps, t * 10.0 * eps]);
            let rho = r * rng.pick(&[1.0, 0.5, 0.999, 1.5, 0.2, 3.0]);
            let off = r * rng.pick(&[0.0, 1e-9, 1e-6, 0.3, 0.6]);
            let p = perp(k, rng);
            let alpha = (tilt / rho).min(1.0).asin();
            let n = rot(k, perp(k, rng), alpha);
            let c = add(o, add(mul(k, r * (rng.next() - 0.5)), mul(p, off)));
            let _ = write!(note, "t/reach={:.3e} tilt/eps={:.3e}", tilt / reach, tilt / eps);
            (
                "cw_tilt",
                Conic { c, n, u: perp(n, rng), a: rho, b: rho, ellipse: false },
                wall,
            )
        }
        // An ellipse near a section of a (coaxial) wall: projection off the
        // axis ~ a circle.
        1 => {
            let r0 = r * rng.pick(&[1.0, 1.0, 0.7, 1.3]);
            let phi = rng.pick(&[0.05, 0.3, 0.7, 1.2]);
            let p = perp(k, rng);
            let n0 = rot(k, p, phi);
            let u = unit(cross(p, n0));
            let extra = rng.pick(&[0.0, 0.1, 0.5, 1.0, 1.1, 2.0, 10.0]) * reach;
            let n = rot(n0, u, extra / r0);
            let off = r * rng.pick(&[0.0, 0.0, 1e-9, 0.3, 0.6]);
            let c = add(o, add(mul(k, r * (rng.next() - 0.5)), mul(perp(k, rng), off)));
            let (a, b) = (r0 / phi.cos(), r0);
            let _ = write!(note, "phi={phi} extra/reach={:.2e}", extra / reach);
            ("ew_section", Conic { c, n, u: rot(u, n, 0.0), a, b, ellipse: true }, wall)
        }
        2 => {
            let a = r * rng.pick(&[0.3, 1.0, 2.0]);
            let b = a * rng.pick(&[0.2, 0.6, 0.95]);
            let n = rng.dir();
            let c = add(o, mul(rng.dir(), r * rng.next()));
            ("ew_gen", Conic { c, n, u: perp(n, rng), a, b, ellipse: true }, wall)
        }
        // An ellipse against a sphere, semi-axes near equal:
        // A₂ = |a² − b²|/4r about t·ε.
        3 => {
            let b = r * rng.pick(&[0.5, 0.9, 1.0, 1.5]);
            let t = rng.pick(&[1e-3, 0.5, 0.99, 1.01, 2.0, 10.0, 1e3]);
            let a = (b * b + 4.0 * r * t * eps).sqrt();
            let n = rng.dir();
            let c = add(o, mul(rng.dir(), r * rng.pick(&[0.0, 0.1, 0.5])));
            let _ = write!(note, "A2/eps={t}");
            ("es_near", Conic { c, n, u: perp(n, rng), a, b, ellipse: a != b }, sph)
        }
        4 => {
            let a = r * rng.pick(&[0.3, 1.0, 2.0]);
            let b = a * rng.pick(&[0.2, 0.6, 0.95]);
            let n = rng.dir();
            let c = add(o, mul(rng.dir(), r * rng.next()));
            ("es_gen", Conic { c, n, u: perp(n, rng), a, b, ellipse: true }, sph)
        }
        5 => {
            let a = r * rng.pick(&[0.3, 1.0, 2.0]);
            let n = rng.dir();
            let c = add(o, mul(rng.dir(), r * rng.next()));
            ("cs_gen", Conic { c, n, u: perp(n, rng), a, b: a, ellipse: false }, sph)
        }
        _ => {
            let a = r * rng.pick(&[0.3, 1.0, 2.0]);
            let n = rng.dir();
            let c = add(o, mul(rng.dir(), r * rng.next()));
            ("cw_gen", Conic { c, n, u: perp(n, rng), a, b: a, ellipse: false }, wall)
        }
    };
    if graze > 0 {
        regraze(&mut cn, &q, depth, graze == 2);
        let _ = write!(note, " graze{}={:.1e}", if graze == 2 { "In" } else { "Out" }, depth / eps);
    }
    let _ = write!(note, " r={r}");
    Pose { fam, cn, q, note }
}

#[test]
fn review_door_fuzz() {
    let eps = Tol::witness().get().eps;
    let band = Band::linear(Tol::witness()).unwrap();
    let n_poses: usize = std::env::var("PROBE_N").ok().and_then(|s| s.parse().ok()).unwrap_or(3000);
    let samples: usize = std::env::var("PROBE_S").ok().and_then(|s| s.parse().ok()).unwrap_or(100_000);
    let mut rng = Rng(0x9e37_79b9_7f4a_7c15 ^ (eps.to_bits()));
    let mut out = String::new();
    let (mut wrong, mut amb, mut slack) = (0usize, 0usize, 0usize);
    let mut tally = std::collections::BTreeMap::<String, usize>::new();
    for i in 0..n_poses {
        let pose = make_pose(&mut rng, eps);
        let (cn, q) = (pose.cn, pose.q);
        let curve = cn.curve();
        // The convention check: my point formula is the kernel's eval.
        for t in [0.3, 2.0] {
            let p = curve.eval(t);
            let m = cn.at(t);
            let e = (p.x - m[0]).abs() + (p.y - m[1]).abs() + (p.z - m[2]).abs();
            assert!(e <= 1e-12 * (1.0 + dot(cn.c, cn.c).sqrt() + cn.a), "convention: {e} {cn:?}");
        }
        let (t0, t1) = (0.0, std::f64::consts::TAU);
        let surf = q.surface(rng.dir());
        let got = door_call(&curve, t0, t1, &surf, band);
        let o = oracle(&cn, &q, t0, samples);
        let scale = 1e-28 * (q.r() + cn.a + dot(cn.c, cn.c).sqrt());
        let maxabs = o.dmin.1.abs().max(o.dmax.1.abs());
        let (verdict, mut flag) = match &got {
            Ok(CircleRoots::OnSurface) => {
                let bad = maxabs > eps * (1.0 + 1e-9) + scale;
                ("OnSurface".to_string(), if bad { format!("WRONG on-surface max|d|/eps={:.4}", maxabs / eps) } else { String::new() })
            }
            Ok(CircleRoots::Miss) => {
                let bad = !o.roots.is_empty();
                let ambig = bad && o.dmin.1.abs().min(o.dmax.1.abs()) < scale * 10.0;
                (
                    "Miss".to_string(),
                    if ambig {
                        "AMBIG miss vs sub-rounding crossing".into()
                    } else if bad {
                        format!("WRONG miss through {} crossings dmin/eps={:.3e} dmax/eps={:.3e}", o.roots.len(), o.dmin.1 / eps, o.dmax.1 / eps)
                    } else {
                        String::new()
                    },
                )
            }
            Ok(CircleRoots::Certified { count, thetas }) => {
                let mut f = String::new();
                if *count != o.roots.len() {
                    let ambig = o.dmin.1.abs().min(o.dmax.1.abs()) < scale * 10.0;
                    f = format!(
                        "{} count {} vs oracle {} dmin/eps={:.3e} dmax/eps={:.3e}",
                        if ambig { "AMBIG" } else { "WRONG" },
                        count,
                        o.roots.len(),
                        o.dmin.1 / eps,
                        o.dmax.1 / eps
                    );
                } else {
                    let mut worst = 0.0f64;
                    let mut worst_d = 0.0f64;
                    for &th in &thetas[..*count] {
                        let best = o
                            .roots
                            .iter()
                            .map(|&rt| (wrap(th - rt).abs() * cn.speed(rt), rt))
                            .fold((f64::INFINITY, 0.0), |a, b| if b.0 < a.0 { b } else { a });
                        worst = worst.max(best.0);
                        worst_d = worst_d.max(q.dist_dd(cn.at_dd(th)).abs());
                        if !(th >= t0 - 1e-12 && th <= t1 + 1e-12) {
                            f = format!("WRONG root {th} outside the reporting window");
                        }
                    }
                    if worst_d > eps && f.is_empty() {
                        f = format!("WRONG root off the surface by {:.4}·eps", worst_d / eps);
                    } else if worst > eps * 1.000_001 && f.is_empty() {
                        f = format!("SLACK root off along the carrier by {:.4}·eps (arc), on the surface to {:.2e}·eps", worst / eps, worst_d / eps);
                    }
                    let _ = write!(f, "{}worst_arc/eps={:.3e} worst_d/eps={:.3e}", if f.is_empty() { "" } else { " " }, worst / eps, worst_d / eps);
                }
                (format!("Certified{count}"), f)
            }
            Ok(CircleRoots::Uncertain) => ("Uncertain".into(), String::new()),
            Ok(CircleRoots::CountDisagrees) => ("CountDisagrees".into(), String::new()),
            Err(BooleanError::Escalated { .. }) => ("Escalated".into(), String::new()),
            Err(e) => ("Err".into(), format!("WRONG error {e:?}")),
        };
        if std::env::var("PROBE_ONLY").ok().and_then(|s| s.parse::<usize>().ok()) == Some(i) {
            eprintln!("ORACLE roots {:?}", o.roots);
            for &rt in &o.roots { eprintln!("  root {rt} d={:e} d(-1e-9)={:e} d(+1e-9)={:e}", q.dist_dd(cn.at_dd(rt)), q.dist_dd(cn.at_dd(rt-1e-9)), q.dist_dd(cn.at_dd(rt+1e-9))); }
            if let Ok(CircleRoots::Certified { count, thetas }) = &got { for &th in &thetas[..*count] { eprintln!("  door {th} d={:e}", q.dist_dd(cn.at_dd(th))); } }
            let o2 = oracle(&cn, &q, t0 + 0.123, samples * 10);
            eprintln!("ORACLE2 roots {:?} dmin {:?} dmax {:?}", o2.roots, o2.dmin, o2.dmax);
        }
        if flag.starts_with("WRONG") {
            wrong += 1;
            eprintln!("[{i}] {} {} {verdict} {flag} {:?} {:?}", pose.fam, pose.note, cn, q);
        } else if flag.starts_with("SLACK") {
            slack += 1;
            eprintln!("[{i}] {} {} {verdict} {flag} A2/eps={:.3e}", pose.fam, pose.note, a2_of(&cn, &q) / eps);
        } else if flag.starts_with("AMBIG") {
            amb += 1;
        }
        if flag.is_empty() {
            flag.push('-');
        }
        let short = verdict.split(':').next().unwrap().to_string();
        *tally.entry(format!("{}/{}", pose.fam, short)).or_default() += 1;
        let _ = writeln!(
            out,
            "{i},{},{:.4e},{verdict},{},{},{:.6e},{:.6e},\"{}\",\"{}\"",
            pose.fam,
            a2_of(&cn, &q) / eps,
            o.roots.len(),
            flag,
            o.dmin.1 / eps,
            o.dmax.1 / eps,
            pose.note,
            match &got {
                Ok(CircleRoots::Certified { count, thetas }) => format!("{:?}", &thetas[..*count]),
                _ => String::new(),
            }
        );
    }
    if let Ok(path) = std::env::var("PROBE_OUT") {
        std::fs::write(path, out).unwrap();
    }
    for (k, v) in &tally {
        eprintln!("{k}: {v}");
    }
    eprintln!("eps={eps} poses={n_poses} WRONG={wrong} SLACK={slack} AMBIG={amb}");
}

/// Claim 2, at the band's edge: `A₂` just inside the zero band (so the
/// dropped harmonic is charged at its largest) and shallow crossings
/// just past the extremes' escalation threshold, where the charge is the
/// largest share of the slope.
#[test]
fn review_arm_edge() {
    let eps = Tol::witness().get().eps;
    let band = Band::linear(Tol::witness()).unwrap();
    let mut rng = Rng(0x1234_5678_9abc_def1 ^ eps.to_bits());
    let (mut n, mut cert, mut wrong, mut worst) = (0usize, 0usize, 0usize, 0.0f64);
    let mut tally = std::collections::BTreeMap::<String, usize>::new();
    for &r in &[1e-3, 1.0, 100.0] {
        for &s in &[0.5, 0.8, 0.95, 0.99, 0.999] {
            for &depth in &[10.5, 11.0, 12.0, 15.0, 20.0, 50.0, 100.0, 1e3] {
                for inside in [false, true] {
                    for _ in 0..12 {
                        let wall_like = rng.next() < 0.6;
                        let k = rng.dir();
                        let o = mul(rng.dir(), r * rng.next());
                        let (mut cn, q) = if wall_like {
                            // A circle of radius ρ, tilt from square with
                            // `A₂ = (ρ sin α)²/4r = s·ε`.
                            let rho = r * rng.pick(&[1.0, 0.5, 2.0]);
                            let tilt = 2.0 * (r * s * eps).sqrt();
                            let n = rot(k, perp(k, &mut rng), (tilt / rho).asin());
                            let c = add(o, mul(perp(k, &mut rng), r * rng.pick(&[0.0, 0.2, 0.5])));
                            (Conic { c, n, u: perp(n, &mut rng), a: rho, b: rho, ellipse: false }, Quad::Wall { o, k, r })
                        } else {
                            // An ellipse against a sphere, `A₂ = (a² − b²)/4r = s·ε`.
                            let b = r * rng.pick(&[0.5, 1.0, 1.5]);
                            let a = (b * b + 4.0 * r * s * eps).sqrt();
                            let n = rng.dir();
                            let c = add(o, mul(rng.dir(), r * rng.pick(&[0.1, 0.5])));
                            (Conic { c, n, u: perp(n, &mut rng), a, b, ellipse: true }, Quad::Sphere { o, r })
                        };
                        regraze(&mut cn, &q, if inside { depth * eps } else { -depth * eps }, inside);
                        if !cn.c.iter().all(|x| x.is_finite()) {
                            continue;
                        }
                        n += 1;
                        if std::env::var("REVIEW_ARM").is_ok() {
                            eprintln!("POSE");
                        }
                        let got = door_call(&cn.curve(), 0.0, std::f64::consts::TAU, &q.surface(rng.dir()), band);
                        let o2 = oracle(&cn, &q, 0.0, 20_000);
                        let key = match &got {
                            Ok(CircleRoots::Certified { count, thetas }) => {
                                cert += 1;
                                if *count != o2.roots.len() {
                                    wrong += 1;
                                    eprintln!("WRONG count {count} vs {} {cn:?} {q:?}", o2.roots.len());
                                } else {
                                    for &th in &thetas[..*count] {
                                        let e = o2.roots.iter().map(|&rt| wrap(th - rt).abs() * cn.speed(rt)).fold(f64::INFINITY, f64::min);
                                        let d = q.dist_dd(cn.at_dd(th)).abs();
                                        worst = worst.max(e / eps);
                                        if e > eps {
                                            eprintln!("EDGE-SLACK arc/eps={:.3e} d/eps={:.3e} s={s} r={r} depth={depth} inside={inside} wall={wall_like}", e / eps, d / eps);
                                        }
                                        if d > eps {
                                            wrong += 1;
                                            eprintln!("WRONG off surface {d:e} {cn:?} {q:?}");
                                        }
                                    }
                                }
                                "Certified"
                            }
                            Ok(CircleRoots::Miss) => {
                                if !o2.roots.is_empty() {
                                    wrong += 1;
                                    eprintln!("WRONG miss {cn:?} {q:?}");
                                }
                                "Miss"
                            }
                            Ok(CircleRoots::OnSurface) => {
                                if o2.dmin.1.abs().max(o2.dmax.1.abs()) > eps {
                                    wrong += 1;
                                }
                                "OnSurface"
                            }
                            Ok(CircleRoots::Uncertain) => "Uncertain",
                            Ok(CircleRoots::CountDisagrees) => "CountDisagrees",
                            Err(BooleanError::Escalated { .. }) => "Escalated",
                            Err(_) => {
                                wrong += 1;
                                "Err"
                            }
                        };
                        *tally.entry(format!("{}/{key}", if wall_like { "circle-wall" } else { "ellipse-sphere" })).or_default() += 1;
                    }
                }
            }
        }
    }
    for (k, v) in &tally {
        eprintln!("{k}: {v}");
    }
    eprintln!("EDGE eps={eps} poses={n} certified={cert} WRONG={wrong} worst_arc/eps={worst:.4}");
}
