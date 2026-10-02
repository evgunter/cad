//! Reviewer probes, PR #3843 (lane reach-dual3843-r1). Not for merge.
//!
//! Oracles are closed forms in the body's canonical frame: the body is
//! a column field `y ∈ [lo(ρ), hi(ρ)]` over the (x, z) disk, the
//! unarmed face is sampled analytically, and volumes are a midpoint
//! integral of the column clipped by the plane.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::revolve_common::{axis_y, validated};
use geom_core::{Affine3, Band, Point2, Point3, Tol, UnitVec3, Vec3};
use profile::{ArcSweep, RawLoop, bulge_from_center, test_support::bulge_loop};
use sweep::{Revolution, revolve};
use topo::splitting::{SplitError, SplitPart, SplitPlane, SplitReduceError, split};
use topo::{Body, DATUM_UNIT_NORM, SolidContainment, validate, validate_closed, validate_geometric};

thread_local! { static WEDGE: std::cell::Cell<Option<f64>> = const { std::cell::Cell::new(None) }; }

/// Inside the revolve's wedge: the profile at azimuth 0 swept by +θ
/// about +y takes (x, 0, 0) to (x cos a, 0, −x sin a).
fn in_wedge(x: f64, z: f64) -> bool {
    WEDGE.with(|w| match w.get() {
        None => true,
        Some(th) => {
            let a = (-z).atan2(x).rem_euclid(std::f64::consts::TAU);
            a <= th
        }
    })
}

fn revolved(chain: Vec<(Point2<f64>, f64)>, tj: Vec<usize>) -> Body<f64> {
    revolve(
        &validated(vec![bulge_loop(chain).with_tangent_joints(tj)]),
        axis_y(),
        WEDGE.with(|w| w.get()).map_or(Revolution::Full, Revolution::Partial),
        Tol::witness(),
    )
    .unwrap_or_else(|e| panic!("FIXTURE-BUILD {e:?}"))
    .body
}

#[derive(Clone, Copy, Debug)]
enum Shape {
    /// Cylinder r=1, y∈[0,1], spherical cap of radius R above y=1.
    /// `bulge`: the cap is the larger (centre above the rim).
    Cap { r_sph: f64, bulge: bool },
    /// Cylinder r=1 with top rim rounded by tube radius t.
    Round { t: f64 },
    /// Ball of radius 5/4 about (0, 1/4), flat at y = 1.
    Trunc,
}

impl Shape {
    fn cap_center(self) -> f64 {
        match self {
            Shape::Cap { r_sph, bulge } => {
                let k = (r_sph * r_sph - 1.0).sqrt();
                if bulge { 1.0 + k } else { 1.0 - k }
            }
            Shape::Trunc => 0.25,
            Shape::Round { .. } => unreachable!(),
        }
    }
    fn build(self, s: f64) -> Body<f64> {
        let p = |x: f64, y: f64| Point2::new(x * s, y * s);
        match self {
            Shape::Cap { r_sph, .. } => {
                let c = self.cap_center();
                let (a, b) = (p(1.0, 1.0), p(0.0, c + r_sph));
                let bl = bulge_from_center(a, b, p(0.0, c), ArcSweep::Ccw);
                revolved(
                    vec![(p(0.0, 0.0), 0.0), (p(1.0, 0.0), 0.0), (a, bl), (b, 0.0)],
                    if (r_sph - 1.0).abs() < 1e-12 { vec![2] } else { vec![] },
                )
            }
            Shape::Round { t } => {
                let (a, b) = (p(1.0, 1.0), p(1.0 - t, 1.0 + t));
                let bl = bulge_from_center(a, b, p(1.0 - t, 1.0), ArcSweep::Ccw);
                revolved(
                    vec![(p(0.0, 0.0), 0.0), (p(1.0, 0.0), 0.0), (a, bl), (b, 0.0), (p(0.0, 1.0 + t), 0.0)],
                    vec![2, 3],
                )
            }
            Shape::Trunc => {
                let (a, b) = (p(0.0, -1.0), p(1.0, 1.0));
                let bl = bulge_from_center(a, b, p(0.0, 0.25), ArcSweep::Ccw);
                revolved(vec![(a, bl), (b, 0.0), (p(0.0, 1.0), 0.0)], vec![])
            }
        }
    }
    fn rmax(self) -> f64 {
        match self {
            Shape::Cap { r_sph, bulge: true } => r_sph,
            Shape::Trunc => 1.25,
            _ => 1.0,
        }
    }
    /// The body's column at radius ρ (unit scale), or None outside.
    fn column(self, rho: f64) -> Option<(f64, f64)> {
        match self {
            Shape::Cap { r_sph, bulge } => {
                let c = self.cap_center();
                if rho <= 1.0 {
                    Some((0.0, c + (r_sph * r_sph - rho * rho).sqrt()))
                } else if bulge && rho <= r_sph {
                    let k = (r_sph * r_sph - rho * rho).sqrt();
                    Some((c - k, c + k))
                } else {
                    None
                }
            }
            Shape::Round { t } => {
                if rho <= 1.0 - t {
                    Some((0.0, 1.0 + t))
                } else if rho <= 1.0 {
                    let d = rho - (1.0 - t);
                    Some((0.0, 1.0 + (t * t - d * d).sqrt()))
                } else {
                    None
                }
            }
            Shape::Trunc => {
                if rho > 1.25 {
                    return None;
                }
                let k = (1.5625 - rho * rho).sqrt();
                Some((0.25 - k, (0.25 + k).min(1.0)))
            }
        }
    }
    /// Analytic samples of the unarmed face (unit scale).
    fn face_samples(self) -> Vec<[f64; 3]> {
        let mut out = Vec::new();
        let na = 96;
        let nl = 64;
        for i in 0..na {
            let az = std::f64::consts::TAU * i as f64 / na as f64;
            let (sa, ca) = az.sin_cos();
            for j in 0..=nl {
                let f = j as f64 / nl as f64;
                let (rho, y) = match self {
                    Shape::Cap { .. } | Shape::Trunc => {
                        let c = self.cap_center();
                        let r = match self { Shape::Cap { r_sph, .. } => r_sph, _ => 1.25 };
                        // latitude from the rim y=1 to the pole at c+R (cap) or c-R (trunc)
                        let h0 = 1.0 - c;
                        let h1 = if matches!(self, Shape::Trunc) { -r } else { r };
                        let h = h0 + (h1 - h0) * f;
                        (((r * r - h * h).max(0.0)).sqrt(), c + h)
                    }
                    Shape::Round { t } => {
                        let th = std::f64::consts::FRAC_PI_2 * f;
                        (1.0 - t + t * th.cos(), 1.0 + t * th.sin())
                    }
                };
                if in_wedge(rho * ca, rho * sa) {
                    out.push([rho * ca, y, rho * sa]);
                }
            }
        }
        out
    }
    fn kind(self) -> geom::SurfaceKind {
        match self {
            Shape::Round { .. } => geom::SurfaceKind::Torus,
            _ => geom::SurfaceKind::Sphere,
        }
    }
}

/// Volume of the body (unit scale) on the side n·p < d.
fn vol_below(shape: Shape, n: [f64; 3], d: f64) -> f64 {
    let rm = shape.rmax();
    let m = 1400;
    let hstep = 2.0 * rm / m as f64;
    let mut v = 0.0;
    for i in 0..m {
        let x = -rm + (i as f64 + 0.5) * hstep;
        for k in 0..m {
            let z = -rm + (k as f64 + 0.5) * hstep;
            if !in_wedge(x, z) {
                continue;
            }
            let Some((lo, hi)) = shape.column((x * x + z * z).sqrt()) else { continue };
            let len = if n[1].abs() < 1e-14 {
                if n[0] * x + n[2] * z < d { hi - lo } else { 0.0 }
            } else {
                let yp = (d - n[0] * x - n[2] * z) / n[1];
                if n[1] > 0.0 {
                    (yp.min(hi) - lo).max(0.0)
                } else {
                    (hi - yp.max(lo)).max(0.0)
                }
            };
            v += len;
        }
    }
    v * hstep * hstep
}

fn total(shape: Shape) -> f64 {
    vol_below(shape, [0.0, 1.0, 0.0], 100.0)
}

struct Lcg(u64);
impl Lcg {
    fn f(&mut self) -> f64 {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        ((self.0 >> 11) as f64) / ((1u64 << 53) as f64)
    }
    fn dir(&mut self) -> [f64; 3] {
        loop {
            let v = [2.0 * self.f() - 1.0, 2.0 * self.f() - 1.0, 2.0 * self.f() - 1.0];
            let l = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
            if l > 0.2 && l <= 1.0 {
                return [v[0] / l, v[1] / l, v[2] / l];
            }
        }
    }
}

fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

#[derive(Default, Debug)]
struct Tally {
    ok: usize,
    refused_gate: usize,
    refused_other: usize,
    unsound: Vec<String>,
    wrong: Vec<String>,
    pis_bad: Vec<String>,
}

/// Every Ok split against the oracle; returns the tally.
fn sweep_shape(shape: Shape, s: f64, posed: bool, planes: &[([f64; 3], f64)], t: &mut Tally) {
    let Ok(canon) = std::panic::catch_unwind(|| shape.build(s)) else {
        eprintln!("   SKIP {shape:?} s={s}: the fixture does not build at this eps");
        return;
    };
    let pose = Affine3::rotation_about_axis(
        Point3::new(0.3 * s, -0.2 * s, 0.7 * s),
        Vec3::new(0.48, 0.6, 0.64),
        1.1,
    );
    let body = if posed {
        topo::transform_rigid(&canon, &pose, Tol::witness()).unwrap()
    } else {
        canon
    };
    let map_p = |p: [f64; 3]| {
        let q = Point3::new(p[0] * s, p[1] * s, p[2] * s);
        if posed { pose.transform_point(q) } else { q }
    };
    let samples = shape.face_samples();
    let vt = total(shape);
    let band = Band::linear(Tol::witness()).unwrap();
    for &(n, d) in planes {
        let o = [n[0] * d, n[1] * d, n[2] * d];
        let o_w = map_p(o);
        let tip = map_p([o[0] + n[0], o[1] + n[1], o[2] + n[2]]);
        let nw = Vec3::new((tip.x - o_w.x) / s, (tip.y - o_w.y) / s, (tip.z - o_w.z) / s);
        let normal = UnitVec3::new(nw, DATUM_UNIT_NORM, band).unwrap();
        let plane = SplitPlane { origin: o_w, normal };
        let what = format!("{shape:?} s={s} posed={posed} n={n:?} d={d}");
        let (mn, mx) = samples.iter().fold((f64::MAX, f64::MIN), |(a, b), p| {
            let v = dot(*p, n) - d;
            (a.min(v), b.max(v))
        });
        match split(&body, &plane, Tol::witness()) {
            Err(SplitError::Reduce(SplitReduceError::CurvedBooleanUnsupported { kind, .. }))
                if kind == shape.kind() =>
            {
                t.refused_gate += 1
            }
            Err(e) => {
                t.refused_other += 1;
                let _ = e;
            }
            Ok(r) => {
                t.ok += 1;
                if mn < -1e-9 && mx > 1e-9 {
                    t.unsound.push(format!("{what}: face straddles [{mn:e}, {mx:e}]"));
                }
                let vb = vol_below(shape, n, d) * s * s * s;
                let va = vt * s * s * s - vb;
                let mut sum = 0.0;
                let mut props_failed = false;
                let mut pads = 0.0;
                for (part, want, side) in [(&r.above, va, "above"), (&r.below, vb, "below")] {
                    match part {
                        SplitPart::Empty => {
                            if want > 1e-4 * s * s * s {
                                t.wrong.push(format!("{what}: {side} Empty, oracle {want}"));
                            }
                        }
                        SplitPart::Body(b) => {
                            // tier 3's +V backstop rides the same quadrature
                            // as props; an escalation there is counted under
                            // PROPS-ERR (pre-existing on a plain wedge).
                            if validate(b).is_err()
                                || validate_closed(b).is_err()
                                || validate_geometric(b, Tol::witness()).is_err_and(|es| {
                                    !es.iter().all(|e| format!("{e:?}").starts_with("VolumeUncomputable"))
                                })
                            {
                                t.wrong.push(format!("{what}: {side} fails a tier"));
                            }
                            let pr = match topo::props::mass_properties(b, Tol::witness()) {
                                Ok(pr) => pr,
                                Err(e) => {
                                    eprintln!("   PROPS-ERR {what}: {side}: {e:?}");
                                    props_failed = true;
                                    continue;
                                }
                            };
                            sum += pr.volume;
                            pads += pr.volume_pad;
                            if (pr.volume - want).abs() > pr.volume_pad + 2e-4 * s * s * s {
                                t.wrong.push(format!(
                                    "{what}: {side} {} ± {} vs oracle {want}",
                                    pr.volume, pr.volume_pad
                                ));
                            }
                            // point_in_solid against the column oracle.
                            let mut g = Lcg(7);
                            for _ in 0..12 {
                                let rm = shape.rmax();
                                let p = [
                                    (2.0 * g.f() - 1.0) * rm,
                                    -1.2 + 3.4 * g.f(),
                                    (2.0 * g.f() - 1.0) * rm,
                                ];
                                let rho = (p[0] * p[0] + p[2] * p[2]).sqrt();
                                let sd = dot(p, n) - d;
                                if sd.abs() < 1e-3 {
                                    continue;
                                }
                                if WEDGE.with(|w| w.get()).is_some() {
                    continue; // wedge walls: skip the membership oracle
                }
                let inside = match shape.column(rho) {
                                    Some((lo, hi)) => {
                                        if (p[1] - lo).abs() < 1e-3 || (p[1] - hi).abs() < 1e-3 {
                                            continue;
                                        }
                                        p[1] > lo && p[1] < hi
                                    }
                                    None => false,
                                };
                                if (rho - 1.0).abs() < 1e-3 || (rho - shape.rmax()).abs() < 1e-3 {
                                    continue;
                                }
                                let want_in = inside && ((side == "below") == (sd < 0.0));
                                let q = map_p(p);
                                match topo::point_in_solid(b, q, band, Tol::witness()) {
                                    Ok(SolidContainment::In) if !want_in => {
                                        t.pis_bad.push(format!("{what}: {side} {p:?} In, oracle Out"))
                                    }
                                    Ok(SolidContainment::Out) if want_in => {
                                        t.pis_bad.push(format!("{what}: {side} {p:?} Out, oracle In"))
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                }
                if !props_failed && (sum - vt * s * s * s).abs() > pads + 4e-4 * s * s * s {
                    t.wrong.push(format!("{what}: halves sum {sum} vs {}", vt * s * s * s));
                }
            }
        }
    }
}

fn planes_for(shape: Shape, seed: u64) -> Vec<([f64; 3], f64)> {
    let mut g = Lcg(seed);
    let samples = shape.face_samples();
    let mut out = Vec::new();
    for _ in 0..14 {
        let n = g.dir();
        let (mn, mx) = samples.iter().fold((f64::MAX, f64::MIN), |(a, b), p| {
            let v = dot(*p, n);
            (a.min(v), b.max(v))
        });
        // just meeting, in the pad, just clear, both ends; and a random one.
        for d in [mx - 1e-3, mx - 1e-7, mx + 1e-7, mx + 1e-3, mx + 5e-2, mn + 1e-3, mn - 1e-3, mn - 5e-2] {
            out.push((n, d));
        }
        out.push((n, mn + (mx - mn) * g.f()));
    }
    // Planes crossing the cylinder only, at tilts.
    for k in 0..10 {
        let phi = 0.05 * k as f64;
        let n = [phi.sin(), phi.cos(), 0.0];
        out.push((n, 0.5 * phi.cos()));
    }
    out
}

#[test]
fn r1_gate_soundness_sweep() {
    let shapes = [
        Shape::Cap { r_sph: 1.25, bulge: false },
        Shape::Cap { r_sph: 1.0, bulge: false },
        Shape::Cap { r_sph: 3.0, bulge: false },
        Shape::Cap { r_sph: 1.25, bulge: true },
        Shape::Round { t: 0.25 },
        Shape::Round { t: 0.4 },
        Shape::Round { t: 0.1 },
        Shape::Trunc,
    ];
    let mut all = Tally::default();
    let from: usize = std::env::var("R1_FROM").ok().and_then(|v| v.parse().ok()).unwrap_or(0);
    for (i, shape) in shapes.into_iter().enumerate().skip(from) {
        for s in [1e-3, 1.0, 1e3] {
            for posed in [false, true] {
                let planes = planes_for(shape, 11 + i as u64);
                let mut t = Tally::default();
                sweep_shape(shape, s, posed, &planes, &mut t);
                eprintln!(
                    "{shape:?} s={s} posed={posed}: ok {} gate {} other {} unsound {} wrong {} pis {}",
                    t.ok, t.refused_gate, t.refused_other, t.unsound.len(), t.wrong.len(), t.pis_bad.len()
                );
                for m in t.unsound.iter().chain(&t.wrong).chain(&t.pis_bad).take(4) {
                    eprintln!("   {m}");
                }
                all.ok += t.ok;
                all.refused_gate += t.refused_gate;
                all.refused_other += t.refused_other;
                all.unsound.extend(t.unsound);
                all.wrong.extend(t.wrong);
                all.pis_bad.extend(t.pis_bad);
            }
        }
    }
    eprintln!(
        "TOTAL ok {} gate {} other {} unsound {} wrong {} pis {}",
        all.ok, all.refused_gate, all.refused_other, all.unsound.len(), all.wrong.len(), all.pis_bad.len()
    );
    assert!(all.unsound.is_empty() && all.wrong.is_empty() && all.pis_bad.is_empty());
}

/// Partial revolutions: sphere and torus faces with an azimuth window.
#[test]
fn r1_gate_soundness_wedges() {
    let mut all = Tally::default();
    for th in [1.0, 2.5, 4.5] {
        WEDGE.with(|w| w.set(Some(th)));
        for (i, shape) in [
            Shape::Cap { r_sph: 1.25, bulge: false },
            Shape::Cap { r_sph: 1.25, bulge: true },
            Shape::Round { t: 0.25 },
            Shape::Trunc,
        ]
        .into_iter()
        .enumerate()
        {
            let planes = planes_for(shape, 101 + i as u64);
            let mut t = Tally::default();
            sweep_shape(shape, 1.0, false, &planes, &mut t);
            eprintln!(
                "wedge {th} {shape:?}: ok {} gate {} other {} unsound {} wrong {}",
                t.ok, t.refused_gate, t.refused_other, t.unsound.len(), t.wrong.len()
            );
            for m in t.unsound.iter().chain(&t.wrong).take(4) {
                eprintln!("   {m}");
            }
            all.unsound.extend(t.unsound);
            all.wrong.extend(t.wrong);
        }
    }
    WEDGE.with(|w| w.set(None));
    assert!(all.unsound.is_empty() && all.wrong.is_empty());
}

/// Liveness under rotation: the PR's own fixture and cut (φ = 0 through
/// mid-height) with body and plane turned together about z by θ.
#[test]
fn r1_rotated_fixture_liveness() {
    let shape = Shape::Cap { r_sph: 1.25, bulge: false };
    let canon = shape.build(1.0);
    let band = Band::linear(Tol::witness()).unwrap();
    for th in [0.0, 0.01, 0.05, 0.1, 0.2, 0.3, 0.5, 1.0, std::f64::consts::FRAC_PI_2] {
        let pose = Affine3::rotation_about_axis(Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0), th);
        let body = topo::transform_rigid(&canon, &pose, Tol::witness()).unwrap();
        let o = pose.transform_point(Point3::new(0.0, 0.5, 0.0));
        let tip = pose.transform_point(Point3::new(0.0, 1.5, 0.0));
        let n = UnitVec3::new(tip - o, DATUM_UNIT_NORM, band).unwrap();
        let r = split(&body, &SplitPlane { origin: o, normal: n }, Tol::witness());
        eprintln!("theta {th}: {}", match &r { Ok(_) => "splits".to_string(), Err(e) => format!("{e:?}") });
    }
}

/// Results reused as operands: the upper half (cap whole) re-split at
/// y = 0.8, tilted, then the lower piece of THAT split again.
#[test]
fn r1_resplit_halves() {
    let shape = Shape::Cap { r_sph: 1.25, bulge: false };
    let body = shape.build(1.0);
    let band = Band::linear(Tol::witness()).unwrap();
    let pl = |phi: f64, y: f64| SplitPlane {
        origin: Point3::new(0.0, y, 0.0),
        normal: UnitVec3::new(Vec3::new(phi.sin(), phi.cos(), 0.0), DATUM_UNIT_NORM, band).unwrap(),
    };
    let r = split(&body, &pl(0.0, 0.4), Tol::witness()).unwrap();
    let SplitPart::Body(up) = r.above else { panic!() };
    let r2 = split(&up, &pl(0.1, 0.75), Tol::witness()).unwrap_or_else(|e| panic!("{e:?}"));
    let (SplitPart::Body(a2), SplitPart::Body(b2)) = (&r2.above, &r2.below) else { panic!() };
    let v = |b: &Body<f64>| topo::props::mass_properties(b, Tol::witness()).unwrap();
    for b in [a2, b2] {
        validate(b).unwrap();
        validate_closed(b).unwrap();
        validate_geometric(b, Tol::witness()).unwrap();
    }
    // Oracle: the plane through (0, .75) tilted by 0.1 stays in the
    // cylinder (the zone box starts at y = 1), so the piece below it and above
    // y = 0.4 is π·(0.75 − 0.4).
    let want_b = std::f64::consts::PI * 0.35;
    let (pa, pb) = (v(a2), v(b2));
    eprintln!("resplit below {} ± {} oracle {want_b}; above {} ± {}", pb.volume, pb.volume_pad, pa.volume, pa.volume_pad);
    assert!((pb.volume - want_b).abs() <= pb.volume_pad + 1e-9);
    let want_a = total(shape) - std::f64::consts::PI * 0.75;
    assert!((pa.volume - want_a).abs() <= pa.volume_pad + 2e-4, "{} vs {want_a}", pa.volume);
    // and the cap-carrying piece once more, at a plane through the cap: refuses.
    let e = split(a2, &pl(0.0, 1.2), Tol::witness()).unwrap_err();
    assert!(matches!(e, SplitError::Reduce(SplitReduceError::CurvedBooleanUnsupported { .. })), "{e:?}");
}

/// Isolation: the wedge θ = 1 tilted split, with and without the cap.
#[test]
fn r1_wedge_tier_isolation() {
    let band = Band::linear(Tol::witness()).unwrap();
    let phi: f64 = 0.1;
    let plane = SplitPlane {
        origin: Point3::new(0.0, 0.5, 0.0),
        normal: UnitVec3::new(Vec3::new(phi.sin(), phi.cos(), 0.0), DATUM_UNIT_NORM, band).unwrap(),
    };
    for th in [1.0, 2.5] {
        WEDGE.with(|w| w.set(Some(th)));
        let p = |x: f64, y: f64| Point2::new(x, y);
        let plain = revolved(vec![(p(0.0, 0.0), 0.0), (p(1.0, 0.0), 0.0), (p(1.0, 1.0), 0.0), (p(0.0, 1.0), 0.0)], vec![]);
        let capped = Shape::Cap { r_sph: 1.25, bulge: false }.build(1.0);
        for (name, b) in [("plain", plain), ("capped", capped)] {
            match split(&b, &plane, Tol::witness()) {
                Err(e) => eprintln!("wedge {th} {name}: refused {e:?}"),
                Ok(r) => {
                    for (side, part) in [("above", &r.above), ("below", &r.below)] {
                        if let SplitPart::Body(h) = part {
                            eprintln!(
                                "wedge {th} {name} {side}: t1 {:?} t2 {:?} t3 {:?} props {:?}",
                                validate(h).is_ok(),
                                validate_closed(h).is_ok(),
                                validate_geometric(h, Tol::witness()).map_err(|e| format!("{e:?}").chars().take(300).collect::<String>()),
                                topo::props::mass_properties(h, Tol::witness()).map(|m| (m.volume, m.volume_pad)).map_err(|e| format!("{e:?}").chars().take(200).collect::<String>())
                            );
                        }
                    }
                }
            }
        }
    }
    WEDGE.with(|w| w.set(None));
}
