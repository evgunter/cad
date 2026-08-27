//! **R2 review probes, ADOPTED** (PCURVE P-1a fix pass). Authored by
//! the R2 reviewer on `pcurve/r2-probes` and taken here by
//! cherry-pick, so the measurement and its authorship travel
//! together. Two things changed in adoption, both recorded rather
//! than silent:
//!
//! 1. the probes reported by `panic!`, which is right for a probe and
//!    wrong for a merged row — each is now a PINNED assertion, so the
//!    measurement becomes a tripwire instead of a one-shot reading;
//! 2. the fixtures perturbed by `eps() * 0.25`, which makes the row a
//!    function of the run's ε — the exact defect R2's own MAJOR-1
//!    named on the `d2` row. The drift is now a fixed metre value and
//!    the band is built from it, so the rows read the same at every ε
//!    point in the matrix.
//!
//! The claim they attack stands: the seam classes this file measures
//! move, and one of them moves in QUANTITY. See
//! `certify.rs`'s Chart arm for the disposition.
//!
//! Claim-1 attack: the D2 bit-diff row measures a cylinder seam at
//! r = 2 (0 ULP) and a plane iso (1907 ULP). These probes measure the
//! SAME legacy-vs-collapsed delta on the seam classes the row does
//! not name: a sphere seam, a cone seam, and a cylinder seam at a
//! radius where the quadratic term of the implicit form does NOT
//! round away.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::{Curve3, Surface};
use geom_brep::{
    CERT_SAMPLES, EdgeCurve, EdgeCurveSpec, EdgeDescriptionSpec, implicit_residual, sample_param,
};
use geom_core::{Band, Point3, Vec3};

fn table(
    surfs: Vec<Surface<f64>>,
) -> (
    Vec<geom_brep::SurfaceKey>,
    impl Fn(geom_brep::SurfaceKey) -> Option<Surface<f64>>,
) {
    let mut map: slotmap::SlotMap<geom_brep::SurfaceKey, Surface<f64>> =
        slotmap::SlotMap::with_key();
    let keys: Vec<geom_brep::SurfaceKey> = surfs.into_iter().map(|s| map.insert(s)).collect();
    (keys, move |k| map.get(k).cloned())
}

/// The probes' fixed drift, and the band built from it (adoption note
/// 2 in the module docs). Four times the drift is the same fraction
/// of the band R2's `eps() * 0.25` had, without the ε coupling.
const DRIFT: f64 = 2.5e-10;

fn band() -> Band {
    Band::new(4.0 * DRIFT, 40.0 * DRIFT).expect("the probes' own band")
}

/// R2's measured rows, pinned at the adoption drift (module docs).
/// Sphere: a bit move. Cone: a QUANTITY move, `sec α`. Small
/// cylinder: a bit move in the other direction, the legacy form's own
/// `d²/(2r)` term.
/// Sphere seam, r = 2: a BIT move, upward — 2.35e9 ULP.
const R2_SPHERE_ULPS: i64 = 2_349_518_964;
const R2_SPHERE_LEGACY: f64 = 2.500_000_206_850_927_5e-10;
const R2_SPHERE_NOW: f64 = 2.500_001_421_523_762_4e-10;
/// Cone seam, α = 0.5 rad: a QUANTITY move — `sec α` = 1.139494,
/// asserted as a ratio below, not just as a delta. 6.75e14 ULP.
const R2_CONE_ULPS: i64 = 674_550_383_640_576;
const R2_CONE_LEGACY: f64 = 2.500_001_317_073_952e-10;
const R2_CONE_NOW: f64 = 2.848_735_691_785_009_3e-10;
/// Cylinder seam, r = 1e-4: a bit move DOWNWARD — the legacy
/// normalized implicit form's own `d²/(2r)` term, which at this
/// radius no longer rounds away. 6.04e9 ULP.
const R2_SMALL_CYL_ULPS: i64 = 6_044_665_344;
const R2_SMALL_CYL_LEGACY: f64 = 2.500_003_125_058_102e-10;
const R2_SMALL_CYL_NOW: f64 = 2.500_000_000_039_363e-10;

fn ulps(a: f64, b: f64) -> i64 {
    let (x, y) = (a.to_bits() as i64, b.to_bits() as i64);
    (x - y).abs()
}

/// The pre-collapse SEAM meter, verbatim (main's check 4 seam arm):
/// endpoint pins, then per sample the implicit residual and the two
/// seam predicates. `implicit_residual` is called through the crate's
/// own export, so the legacy value is bitwise the shipped one.
fn legacy_seam_max(
    spec: &EdgeCurveSpec<f64>,
    surface: &Surface<f64>,
    anchor: Point3<f64>,
    axis: Vec3<f64>,
    u_ref: Vec3<f64>,
    start: Point3<f64>,
    end: Point3<f64>,
) -> f64 {
    let (t0, t1) = (spec.param_start, spec.param_end);
    let mut m = spec.carrier.eval(t0).distance(start);
    m = m.max(spec.carrier.eval(t1).distance(end));
    let v_ref = axis.cross(u_ref);
    for i in 0..CERT_SAMPLES {
        let p = spec.carrier.eval(sample_param(t0, t1, i));
        m = m.max(implicit_residual(surface, p).abs());
        // seam_frame, replicated: w is the radial component.
        let q = p - anchor;
        let w = q - axis * q.dot(axis);
        m = m.max(w.dot(v_ref).abs());
        m = m.max((0.0 - w.dot(u_ref)).max(0.0).abs());
    }
    m
}

fn seam_delta(
    surface: Surface<f64>,
    anchor: Point3<f64>,
    axis: Vec3<f64>,
    u_ref: Vec3<f64>,
    carrier: Curve3<f64>,
    t0: f64,
    t1: f64,
) -> (i64, f64, f64) {
    let (keys, lookup) = table(vec![surface.clone()]);
    let p0 = carrier.eval(t0);
    let p1 = carrier.eval(t1);
    let spec = EdgeCurveSpec {
        description: EdgeDescriptionSpec::seam(keys[0]),
        carrier,
        param_start: t0,
        param_end: t1,
    };
    let certified = EdgeCurve::certify(spec.clone(), p0, p1, &lookup, band())
        .expect("this seam class certified on main and must still certify");
    let cert = *certified.certificate();
    let legacy = legacy_seam_max(&spec, &surface, anchor, axis, u_ref, p0, p1);
    (ulps(cert.max_residual, legacy), legacy, cert.max_residual)
}

/// Sphere seam meridian, in-band radial drift 0.25 eps — the D2 row's
/// own perturbation posture, on a chart class the row does not name.
#[test]
fn r2_probe_sphere_seam_bit_move() {
    let r = 2.0;
    let d = DRIFT;
    let sphere = Surface::Sphere {
        center: Point3::origin(),
        radius: r,
        axis: Vec3::unit_z(),
        u_ref: Vec3::unit_x(),
    };
    // Meridian in the x-z halfplane (x > 0): a circle about -y so the
    // traversal runs south-to-north, radius r + d (radial drift).
    let carrier = Curve3::Circle {
        center: Point3::origin(),
        axis: Vec3::new(0.0, -1.0, 0.0),
        radius: r + d,
        u_ref: Vec3::unit_x(),
    };
    let (delta, legacy, now) = seam_delta(
        sphere,
        Point3::origin(),
        Vec3::unit_z(),
        Vec3::unit_x(),
        carrier,
        -1.2,
        1.3,
    );
    assert_eq!(
        (delta, legacy, now),
        (R2_SPHERE_ULPS, R2_SPHERE_LEGACY, R2_SPHERE_NOW),
        "R2 sphere-seam row moved: legacy {legacy:e} m, now {now:e} m, delta {delta} ULP"
    );
}

/// Cone seam meridian, in-band normal drift 0.25 eps.
#[test]
fn r2_probe_cone_seam_bit_move() {
    let half_angle = 0.5_f64;
    let d = DRIFT;
    let cone = Surface::Cone {
        apex: Point3::origin(),
        axis: Vec3::unit_z(),
        half_angle,
        u_ref: Vec3::unit_x(),
    };
    let (s, c) = half_angle.sin_cos();
    // Ruling direction in the seam halfplane; drift along the outward
    // normal (c, 0, -s).
    let dir = Vec3::new(s, 0.0, c);
    let normal = Vec3::new(c, 0.0, -s);
    let carrier = Curve3::Line {
        origin: Point3::origin() + normal * d,
        dir,
    };
    let (delta, legacy, now) = seam_delta(
        cone,
        Point3::origin(),
        Vec3::unit_z(),
        Vec3::unit_x(),
        carrier,
        0.5,
        2.0,
    );
    assert_eq!(
        (delta, legacy, now),
        (R2_CONE_ULPS, R2_CONE_LEGACY, R2_CONE_NOW),
        "R2 cone-seam row moved: legacy {legacy:e} m, now {now:e} m, delta {delta} ULP"
    );
    // The cone row is the one that moved in QUANTITY, not in bits, and
    // the ratio is the geometry: the chart image carries the carrier's
    // own azimuth and axial height, so `C − S(P)` is purely radial
    // where `implicit_residual` reads the perpendicular.
    let sec_alpha = 1.0 / half_angle.cos();
    assert!(
        ((now / legacy) / sec_alpha - 1.0).abs() < 1e-6,
        "the cone re-baseline is sec α = {sec_alpha}, measured {}",
        now / legacy
    );
}

/// Cylinder seam at r = 1e-4: same fixture SHAPE as the D2 row's
/// cylinder-seam, different radius — the quadratic term d^2/(2r) of
/// the legacy implicit form no longer rounds away against r^2's ULP.
#[test]
fn r2_probe_small_cylinder_seam_bit_move() {
    let r = 1e-4;
    let d = DRIFT;
    let cyl = Surface::Cylinder {
        origin: Point3::origin(),
        axis: Vec3::unit_z(),
        radius: r,
        u_ref: Vec3::unit_x(),
    };
    let carrier = Curve3::Line {
        origin: Point3::new(r + d, 0.0, 0.0),
        dir: Vec3::unit_z(),
    };
    let (delta, legacy, now) = seam_delta(
        cyl,
        Point3::origin(),
        Vec3::unit_z(),
        Vec3::unit_x(),
        carrier,
        0.0,
        3.0,
    );
    assert_eq!(
        (delta, legacy, now),
        (R2_SMALL_CYL_ULPS, R2_SMALL_CYL_LEGACY, R2_SMALL_CYL_NOW),
        "R2 small-cylinder-seam row moved: legacy {legacy:e} m, now {now:e} m, \
         delta {delta} ULP"
    );
    // Here the collapsed meter reads LOWER than the legacy one, and
    // that direction matters: `implicit_residual`'s cylinder arm is a
    // normalized implicit form carrying a `d²/(2r)` term, which at
    // r = 1e-4 no longer rounds away, while `|C − S(P)|` is the
    // distance itself. Neither meter dominates the other in general —
    // see the Chart arm's disposition — and this row is the
    // counter-example to any claim that one does.
    assert!(
        now < legacy,
        "at r = 1e-4 the legacy normalized implicit form over-reads: {now:e} vs {legacy:e}"
    );
}
