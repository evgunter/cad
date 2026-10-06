//! Review probe for PR #4159, end to end through the public boolean
//! API: torus × {plane, ball, parallel wall} touching at an elliptic
//! point, widened beyond the PR's poses — other radii, three scales,
//! touches off the outer equator, on the seam, near and on the edge of
//! a 270° torus's missing quarter, nested (a ball inside the tube, a
//! 270° torus inside a rod whose wall touches it), near misses at
//! ±1, ±10, ±100 ε, every op in both orders, results reused as
//! operands, and `point_in_solid` sampled against an implicit oracle.
//!
//! Every expected volume is a closed form from the radii and lengths.
//! Nothing here reads the kernel's margins.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{FRAC_PI_2, PI, TAU};

use geom_core::{Affine3, Band, Point2, Point3, Tol, Vec3};
use sweep::Revolution;
use topo::{Body, BooleanError, BooleanOp, SolidContainment};

use super::extent_scan_off_face_tangency::{ball, boolean, rod_z};

fn tol() -> Tol {
    Tol::witness()
}

fn torus(big_r: f64, r: f64, sweep_angle: Option<f64>) -> Body<f64> {
    sweep::test_support::revolved_about_y(
        vec![
            (Point2::new(big_r, -r), 1.0),
            (Point2::new(big_r, r), 1.0),
        ],
        sweep_angle.map_or(Revolution::Full, Revolution::Partial),
        tol(),
    )
}

fn brick(x: (f64, f64), y: (f64, f64), z: (f64, f64)) -> Body<f64> {
    sweep::test_support::brick(x, y, z, tol())
}

fn rod_y(r: f64, (x, z): (f64, f64), (y0, y1): (f64, f64)) -> Body<f64> {
    let turn = Affine3::rotation_about_axis(Point3::origin(), Vec3::unit_x(), -PI / 2.0);
    topo::transform_rigid(&rod_z(r, (x, -z), (y0, y1)), &turn, tol()).unwrap()
}

fn cap(r: f64, h: f64) -> f64 {
    PI * h * h * (3.0 * r - h) / 3.0
}

/// The torus about y: `e(φ) = (cos φ, 0, −sin φ)`, the revolve's own
/// sense, so a partial revolve by θ covers `φ ∈ [0, θ]`.
#[derive(Clone, Copy, Debug)]
struct Tor {
    big_r: f64,
    r: f64,
    sweep: Option<f64>,
}

impl Tor {
    fn volume(&self) -> f64 {
        TAU * PI * self.big_r * self.r * self.r * self.sweep.map_or(1.0, |t| t / TAU)
    }
    fn point(&self, phi: f64, v: f64) -> (Point3<f64>, Vec3<f64>) {
        let e = Vec3::new(phi.cos(), 0.0, -phi.sin());
        let n = e * v.cos() + Vec3::unit_y() * v.sin();
        let p = Point3::origin() + e * (self.big_r + self.r * v.cos()) + Vec3::unit_y() * (self.r * v.sin());
        (p, n)
    }
    fn contains(&self, q: Point3<f64>) -> Option<bool> {
        let rho = q.x.hypot(q.z);
        let m = (rho - self.big_r).hypot(q.y) - self.r;
        let mut phi = (-q.z).atan2(q.x);
        if phi < 0.0 {
            phi += TAU;
        }
        if let Some(t) = self.sweep {
            let edge = phi.min((t - phi).abs()).min(TAU - phi) * rho;
            if edge < 1e-6 * self.big_r {
                return None;
            }
            if phi > t {
                return Some(false);
            }
        }
        if m.abs() < 1e-6 * self.r {
            return None;
        }
        Some(m < 0.0)
    }
}

/// The rigid frame taking the local touch frame (touch at the origin,
/// the torus on `−z`) to `p` with `z ↦ n`.
#[derive(Clone, Copy)]
struct Frame {
    p: Point3<f64>,
    axis: Vec3<f64>,
    angle: f64,
}

impl Frame {
    fn new(p: Point3<f64>, n: Vec3<f64>) -> Self {
        let k = Vec3::unit_z().cross(n);
        let s = k.norm();
        let (axis, angle) = if s < 1e-14 {
            if n.z > 0.0 { (Vec3::unit_x(), 0.0) } else { (Vec3::unit_x(), PI) }
        } else {
            (k / s, n.z.clamp(-1.0, 1.0).acos())
        };
        Self { p, axis, angle }
    }
    fn place(&self, b: &Body<f64>) -> Body<f64> {
        let rot = Affine3::rotation_about_axis(Point3::origin(), self.axis, self.angle);
        let b = topo::transform_rigid(b, &rot, tol()).unwrap();
        topo::transform_rigid(&b, &Affine3::translation(self.p - Point3::origin()), tol()).unwrap()
    }
    /// A global point in local coordinates.
    fn local(&self, q: Point3<f64>) -> Point3<f64> {
        let w = q - self.p;
        // Rotate by −angle about axis (Rodrigues).
        let (k, a) = (self.axis, -self.angle);
        let w = w * a.cos() + k.cross(w) * a.sin() + k * (k.dot(w) * (1.0 - a.cos()));
        Point3::origin() + w
    }
}

#[derive(Clone, Copy, Debug)]
enum Rel {
    Apart,
    BInA,
    AInB,
}

#[derive(Clone, Copy, Debug)]
enum Partner {
    /// Bottom face on the tangent plane at height δ; `off`: the face
    /// stands beside the touch.
    Brick { off: bool },
    /// A ball outside, tangent at the touch; `trimmed`: its cap about
    /// the touch is cut off below `z = δ + 0.2ρ`.
    Ball { rho: f64, trimmed: bool },
    /// A ball inside the tube, tangent at the touch, its cap above
    /// `z = δ − 0.2ρ` cut off.
    BallIn { rho: f64 },
    /// A rod along the torus axis (local y at an equator touch);
    /// `short`: it starts past the touch.
    Rod { rho: f64, short: bool },
    /// A rod along the axis about the whole torus, its wall tangent.
    RodAbout,
}

struct Case {
    label: String,
    a: Body<f64>,
    b: Body<f64>,
    va: f64,
    vb: f64,
    rel: Rel,
    /// The expected answer is a refusal (the boundaries touch on both
    /// faces).
    touching: bool,
    inside_b: Box<dyn Fn(Point3<f64>) -> Option<bool>>,
    t: Tor,
}

fn make(t: Tor, l: f64, phi: f64, v: f64, partner: Partner, delta: f64, touching: bool) -> Case {
    let (p, n) = t.point(phi, v);
    let f = Frame::new(p, n);
    let label = format!("{t:?} φ={phi:.9} v={v} {partner:?} δ={delta:e}");
    type In = Box<dyn Fn(Point3<f64>) -> Option<bool>>;
    let near = move |m: f64| m.abs() < 1e-6 * l;
    let (b, vb, rel, inside): (Body<f64>, f64, Rel, In) = match partner {
        Partner::Brick { off } => {
            let x = if off { (0.4 * l, 1.4 * l) } else { (-0.5 * l, 0.5 * l) };
            let (y, z) = ((-0.5 * l, 0.5 * l), (delta, l));
            let vb = (x.1 - x.0) * (y.1 - y.0) * (z.1 - z.0);
            let inside: In = Box::new(move |q| {
                let q = f.local(q);
                let m = [x.0 - q.x, q.x - x.1, y.0 - q.y, q.y - y.1, z.0 - q.z, q.z - z.1]
                    .into_iter()
                    .fold(f64::MIN, f64::max);
                (!near(m)).then_some(m < 0.0)
            });
            (f.place(&brick(x, y, z)), vb, Rel::Apart, inside)
        }
        Partner::Ball { rho, trimmed } => {
            let c = Vec3::new(0.0, 0.0, rho + delta);
            let cut = delta + 0.2 * rho;
            let full = ball(rho, c);
            let (body, vb) = if trimmed {
                let bk = brick((-2.0 * rho, 2.0 * rho), (-2.0 * rho, 2.0 * rho), (delta - rho, cut));
                let body = super::extent_scan_off_face_tangency::built(BooleanOp::Subtract, &full, &bk)
                    .expect("trimmed ball");
                (body, 4.0 / 3.0 * PI * rho.powi(3) - cap(rho, 0.2 * rho))
            } else {
                (full, 4.0 / 3.0 * PI * rho.powi(3))
            };
            let inside: In = Box::new(move |q| {
                let q = f.local(q);
                let m = (q - (Point3::origin() + c)).norm() - rho;
                let m = if trimmed { m.max(cut - q.z) } else { m };
                (!near(m)).then_some(m < 0.0)
            });
            (f.place(&body), vb, Rel::Apart, inside)
        }
        Partner::BallIn { rho } => {
            let c = Vec3::new(0.0, 0.0, -rho + delta);
            let cut = delta - 0.2 * rho;
            let full = ball(rho, c);
            let bk = brick((-2.0 * rho, 2.0 * rho), (-2.0 * rho, 2.0 * rho), (cut, delta + rho));
            let body = super::extent_scan_off_face_tangency::built(BooleanOp::Subtract, &full, &bk)
                .expect("trimmed inner ball");
            let vb = 4.0 / 3.0 * PI * rho.powi(3) - cap(rho, 0.2 * rho);
            let inside: In = Box::new(move |q| {
                let q = f.local(q);
                let m = ((q - (Point3::origin() + c)).norm() - rho).max(q.z - cut);
                (!near(m)).then_some(m < 0.0)
            });
            (f.place(&body), vb, Rel::BInA, inside)
        }
        Partner::Rod { rho, short } => {
            let y = if short { (0.2 * l + t.r, 2.0 * l) } else { (-2.0 * l, 2.0 * l) };
            let vb = PI * rho * rho * (y.1 - y.0);
            let zc = rho + delta;
            let inside: In = Box::new(move |q| {
                let q = f.local(q);
                let m = (q.x.hypot(q.z - zc) - rho).max(y.0 - q.y).max(q.y - y.1);
                (!near(m)).then_some(m < 0.0)
            });
            (f.place(&rod_y(rho, (0.0, zc), y)), vb, Rel::Apart, inside)
        }
        Partner::RodAbout => {
            let rc = 1.5 * (t.big_r + t.r);
            let zc = -rc + delta;
            let y = (-2.0 * t.r, 2.0 * t.r);
            let vb = PI * rc * rc * (y.1 - y.0);
            let inside: In = Box::new(move |q| {
                let q = f.local(q);
                let m = (q.x.hypot(q.z - zc) - rc).max(y.0 - q.y).max(q.y - y.1);
                (!near(m)).then_some(m < 0.0)
            });
            (f.place(&rod_y(rc, (0.0, zc), y)), vb, Rel::AInB, inside)
        }
    };
    Case {
        label,
        a: torus(t.big_r, t.r, t.sweep),
        b,
        va: t.volume(),
        vb,
        rel,
        touching,
        inside_b: inside,
        t,
    }
}

#[derive(Default)]
struct Log {
    built_ok: usize,
    refused: Vec<String>,
    bad: Vec<String>,
    pis: usize,
}

fn volume(b: &Body<f64>) -> Result<f64, String> {
    topo::validate(b).map_err(|e| format!("validate {e:?}"))?;
    topo::validate_closed(b).map_err(|e| format!("validate_closed {e:?}"))?;
    topo::validate_geometric(b, tol()).map_err(|e| format!("validate_geometric {e:?}"))?;
    let p = topo::mass_properties(b, tol()).map_err(|e| format!("mass {e:?}"))?;
    Ok(p.volume)
}

fn check(case: &Case, log: &mut Log, reuse: bool) {
    let (va, vb) = (case.va, case.vb);
    let (union, meet) = match case.rel {
        Rel::Apart => (va + vb, 0.0),
        Rel::BInA => (va, vb),
        Rel::AInB => (vb, va),
    };
    let scale = va.max(vb);
    for (op, swap, want) in [
        (BooleanOp::Union, false, union),
        (BooleanOp::Union, true, union),
        (BooleanOp::Intersect, false, meet),
        (BooleanOp::Intersect, true, meet),
        (BooleanOp::Subtract, false, va - meet),
        (BooleanOp::Subtract, true, vb - meet),
    ] {
        let (x, y) = if swap { (&case.b, &case.a) } else { (&case.a, &case.b) };
        let name = format!("{} {op:?} swap={swap}", case.label);
        match boolean(op, x, y) {
            Err(e) => {
                let tangent = matches!(&e, BooleanError::FallbackExtentUnsupported { what, .. } if what.contains("tangent"));
                if !case.touching || !tangent {
                    log.refused.push(format!("{name}: {e:?}"));
                }
            }
            Ok(res) => {
                if case.touching {
                    log.bad.push(format!("{name}: BUILT across a touch on both faces"));
                }
                let got = match res.body() {
                    None => Ok(0.0),
                    Some(b) => volume(&b.body),
                };
                match got {
                    Ok(v) if (v - want).abs() <= 1e-7 * scale => {
                        log.built_ok += 1;
                        eprintln!("OK {name}");
                        if reuse && op == BooleanOp::Union && !swap {
                            if let Some(b) = res.body() {
                                pis(case, &b.body, log, &name);
                                reuse_union(case, &b.body, log, &name);
                            }
                        }
                    }
                    Ok(v) => log.bad.push(format!("{name}: volume {v} vs closed form {want} (rel {:e})", (v - want) / scale)),
                    Err(e) => log.bad.push(format!("{name}: {e}")),
                }
            }
        }
    }
}

/// `point_in_solid` on the union against the implicit oracle.
fn pis(case: &Case, u: &Body<f64>, log: &mut Log, name: &str) {
    let band = Band::linear(tol()).unwrap();
    let ext = case.t.big_r + case.t.r;
    let mut seed = 0x9e3779b97f4a7c15u64;
    let mut rnd = || {
        seed ^= seed << 13;
        seed ^= seed >> 7;
        seed ^= seed << 17;
        (seed >> 11) as f64 / (1u64 << 53) as f64
    };
    for _ in 0..60 {
        let q = Point3::new(
            (2.0 * rnd() - 1.0) * 1.6 * ext,
            (2.0 * rnd() - 1.0) * 1.6 * ext,
            (2.0 * rnd() - 1.0) * 1.6 * ext,
        );
        let (Some(ia), Some(ib)) = (case.t.contains(q), (case.inside_b)(q)) else { continue };
        let want = ia || ib;
        match topo::point_in_solid(u, q, band, tol()) {
            Ok(SolidContainment::In) if want => log.pis += 1,
            Ok(SolidContainment::Out) if !want => log.pis += 1,
            other => log.bad.push(format!("{name}: point_in_solid {q:?} = {other:?}, oracle in={want}")),
        }
    }
}

/// The union reused: `U ∪ A` and `U ∩ B` as operands again.
fn reuse_union(case: &Case, u: &Body<f64>, log: &mut Log, name: &str) {
    let (va, vb) = (case.va, case.vb);
    let u_vol = match case.rel {
        Rel::Apart => va + vb,
        Rel::BInA => va,
        Rel::AInB => vb,
    };
    for (op, other, want, what) in [
        (BooleanOp::Intersect, &case.b, vb, "U ∩ B"),
        (BooleanOp::Subtract, &case.a, u_vol - va, "U − A"),
    ] {
        match boolean(op, u, other) {
            Err(e) => log.refused.push(format!("{name} reuse {what}: {e:?}")),
            Ok(res) => {
                let got = match res.body() {
                    None => Ok(0.0),
                    Some(b) => volume(&b.body),
                };
                match got {
                    Ok(v) if (v - want).abs() <= 1e-7 * u_vol => log.built_ok += 1,
                    Ok(v) => log.bad.push(format!("{name} reuse {what}: volume {v} vs {want}")),
                    Err(e) => log.bad.push(format!("{name} reuse {what}: {e}")),
                }
            }
        }
    }
}

fn report(tag: &str, log: &Log) {
    eprintln!("== {tag}: built-and-right {} pis-agree {} refused(unexpected) {} bad {}", log.built_ok, log.pis, log.refused.len(), log.bad.len());
    for r in &log.refused {
        eprintln!("REFUSED {r}");
    }
    for b in &log.bad {
        eprintln!("BAD {b}");
    }
}

const SHAPES: [(f64, f64); 3] = [(2.0, 0.5), (3.0, 1.4), (1.0, 0.1)];

fn full_donut_cases(l: f64, deltas: &[f64], shapes: &[(f64, f64)]) -> Log {
    let mut log = Log::default();
    for &(big_r, r) in shapes {
        let t = Tor { big_r: big_r * l, r: r * l, sweep: None };
        for &delta in deltas {
            let reuse = delta == 0.0;
            for (phi, v) in [(0.9, 0.0), (0.9, 0.6), (2.0, -1.2), (0.0, 0.3), (0.9, 1.5)] {
                let mut partners = vec![
                    (Partner::Brick { off: true }, false),
                    (Partner::Ball { rho: 0.4 * l, trimmed: true }, false),
                    (Partner::BallIn { rho: 0.6 * t.r }, false),
                ];
                if delta == 0.0 {
                    partners.push((Partner::Brick { off: false }, true));
                    partners.push((Partner::Ball { rho: 0.4 * l, trimmed: false }, true));
                }
                if v == 0.0 {
                    partners.push((Partner::Rod { rho: 0.3 * l, short: true }, false));
                    if delta == 0.0 {
                        partners.push((Partner::Rod { rho: 0.3 * l, short: false }, true));
                    }
                }
                for (p, touching) in partners {
                    // A crossing near miss on both faces is not a touch.
                    let c = make(t, l, phi, v, p, delta, touching);
                    check(&c, &mut log, reuse);
                }
            }
        }
    }
    log
}

fn partial_cases(l: f64, deltas: &[f64], shapes: &[(f64, f64)]) -> Log {
    let mut log = Log::default();
    let theta = 1.5 * PI;
    for &(big_r, r) in shapes {
        let t = Tor { big_r: big_r * l, r: r * l, sweep: Some(theta) };
        for &delta in deltas {
            for (phi, touching) in [
                (1.75 * PI, false),
                (theta + 1e-3, false),
                (theta + 1e-6, false),
            ] {
                for v in [0.0, 0.6] {
                    let mut partners = vec![
                        Partner::Brick { off: false },
                        Partner::Ball { rho: 0.4 * l, trimmed: false },
                    ];
                    if v == 0.0 {
                        partners.push(Partner::Rod { rho: 0.3 * l, short: false });
                        partners.push(Partner::RodAbout);
                    }
                    for p in partners {
                        let c = make(t, l, phi, v, p, delta, touching);
                        check(&c, &mut log, delta == 0.0);
                    }
                }
            }
        }
    }
    log
}

/// The touch ON the 270° torus's edge circle, or just inside its face:
/// the boundaries touch, so a refusal is right; a build must still be
/// right.
fn edge_cases(l: f64) -> Log {
    let mut log = Log::default();
    let theta = 1.5 * PI;
    let t = Tor { big_r: 2.0 * l, r: 0.5 * l, sweep: Some(theta) };
    for phi in [theta, theta - 1e-6, theta - 1e-3] {
        for v in [0.0, 0.6] {
            for p in [Partner::Brick { off: false }, Partner::Ball { rho: 0.4 * l, trimmed: false }] {
                let c = make(t, l, phi, v, p, 0.0, true);
                check(&c, &mut log, false);
            }
        }
    }
    log
}

fn eps() -> f64 {
    tol().eps()
}

#[test]
fn probe_full_unit_scale() {
    let log = full_donut_cases(1.0, &[0.0], &SHAPES);
    report("full ×1", &log);
    assert!(log.bad.is_empty());
}

#[test]
fn probe_full_small_scale() {
    let log = full_donut_cases(1e-3, &[0.0], &SHAPES);
    report("full ×1e-3", &log);
    assert!(log.bad.is_empty());
}

#[test]
fn probe_full_large_scale() {
    let log = full_donut_cases(1e3, &[0.0], &SHAPES);
    report("full ×1e3", &log);
    assert!(log.bad.is_empty());
}

#[test]
fn probe_full_near_misses() {
    let e = eps();
    let log = full_donut_cases(1.0, &[e, -e, 10.0 * e, -10.0 * e, 100.0 * e, -100.0 * e], &SHAPES[..1]);
    report("full near misses", &log);
    assert!(log.bad.is_empty());
}

#[test]
fn probe_partial_all_scales() {
    let mut bad = 0;
    for l in [1e-3, 1.0, 1e3] {
        let log = partial_cases(l, &[0.0], &SHAPES);
        report(&format!("partial ×{l}"), &log);
        bad += log.bad.len();
    }
    assert_eq!(bad, 0);
}

#[test]
fn probe_partial_near_misses() {
    let e = eps();
    let log = partial_cases(1.0, &[e, -e, 10.0 * e, -10.0 * e, 100.0 * e, -100.0 * e], &SHAPES[..1]);
    report("partial near misses", &log);
    assert!(log.bad.is_empty());
}

#[test]
fn probe_edge_touches() {
    let mut bad = 0;
    for l in [1e-3, 1.0, 1e3] {
        let log = edge_cases(l);
        report(&format!("edge ×{l}"), &log);
        bad += log.bad.len();
    }
    assert_eq!(bad, 0);
}

/// The near-top plane: tilted `s` off normal to the axis, touching the
/// tube near its top circle, the brick's face beside the touch.
#[test]
fn probe_near_top_plane() {
    let mut log = Log::default();
    for l in [1.0, 1e3] {
        let t = Tor { big_r: 2.0 * l, r: 0.5 * l, sweep: None };
        for s in [1e-2, 1e-4, 1e-6, 1e-7] {
            let v = FRAC_PI_2 - s;
            for (p, touching) in [(Partner::Brick { off: true }, false), (Partner::Brick { off: false }, true)] {
                let c = make(t, l, 0.9, v, p, 0.0, touching);
                check(&c, &mut log, false);
            }
        }
    }
    report("near top", &log);
    assert!(log.bad.is_empty());
}
