//! Adversarial review of `topo::transform_rigid` (M4 PR 2, R1):
//! certificate honesty under rotation composition, the rigidity door's
//! band behavior (including non-finite translations, which the linear
//! rigidity checks cannot see), bit-level key/topology stability, and
//! determinism.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod common;

use std::f64::consts::{FRAC_PI_2, FRAC_PI_3};

use common::{describe_as_intersections, prism_z};
use geom_core::Tol;
use geom_core::{Affine3, Mat3, Vec3};
use topo::{
    Body, TransformError, mass_properties, transform_rigid, validate, validate_closed,
    validate_geometric,
};

fn brick(x: (f64, f64), y: (f64, f64), z: (f64, f64)) -> Body<f64> {
    prism_z::<f64>(&[(x.0, y.0), (x.1, y.0), (x.1, y.1), (x.0, y.1)], z.0, z.1).body
}

fn tiers_ok(b: &Body<f64>) {
    assert_eq!(validate(b), Ok(()));
    assert_eq!(validate_closed(b), Ok(()));
    assert_eq!(validate_geometric(b, Tol::witness()), Ok(()));
}

/// R1b: a NaN or infinite TRANSLATION passes the linear rigidity door
/// untouched (check_rigid reads only `map.linear`); the op must still
/// refuse typed — never return Ok with poisoned points.
#[test]
fn non_finite_translation_is_refused_not_laundered() {
    let b = brick((0.0, 1.0), (0.0, 1.0), (0.0, 1.0));
    for t in [
        Vec3::new(f64::NAN, 0.0, 0.0),
        Vec3::new(0.0, f64::INFINITY, 0.0),
        Vec3::new(0.0, 0.0, f64::NEG_INFINITY),
    ] {
        match transform_rigid(&b, &Affine3::translation(t), Tol::witness()) {
            Ok(out) => {
                // If it slipped through, the result MUST still be a
                // valid body — a NaN point cloud here is a laundering
                // bug (silent corruption).
                panic!(
                    "non-finite translation {t:?} returned Ok; first point {:?}",
                    out.points().next()
                );
            }
            Err(e) => {
                // Fix-pass tightening: the door now refuses with the
                // named finiteness predicate, not the oblique
                // certification path.
                assert!(
                    format!("{e:?}").contains("NonFiniteMap"),
                    "expected the door-side finiteness refusal, got {e:?}"
                );
            }
        }
    }
}

/// R1b: near-rigid maps around the band edges, with probes derived
/// from the AMBIENT tolerance (valid on every gate ε row; the first
/// draft hard-coded the 1e-9 default). Rounding-level defects (≤
/// zero) are decided Zero and pass; in-band and definite defects
/// refuse typed.
#[test]
fn near_rigid_door_band_behavior() {
    let tol = geom_core::Tol::witness().get();
    let (eps, kesc) = (tol.eps, tol.eps * tol.k);
    let b = brick((0.0, 1.0), (0.0, 1.0), (0.0, 1.0));
    let with_c0x = |s: f64| {
        Affine3::from_parts(
            Mat3::from_cols(
                Vec3::new(s, 0.0, 0.0),
                Vec3::new(0.0, 1.0, 0.0),
                Vec3::new(0.0, 0.0, 1.0),
            ),
            Vec3::zero(),
        )
    };
    // Rounding-level defect: col0 norm² off well below the zero band
    // — decided Zero, must PASS (a real rotation matrix carries this
    // much noise). eps*1e-4 stays sub-band on every row while ≫ ulp.
    assert!(
        transform_rigid(&b, &with_c0x(1.0 + eps * 1e-4), Tol::witness()).is_ok(),
        "rounding-level orthonormality noise must not refuse"
    );
    // In-band defect (norm² between zero and escalate, geometric
    // mean of the edges): maybe-rigid is not rigid — typed NotRigid.
    let in_band = (eps * kesc).sqrt();
    match transform_rigid(&b, &with_c0x(1.0 + in_band / 2.0), Tol::witness()) {
        Err(TransformError::NotRigid { .. }) => {}
        other => panic!("in-band defect: expected NotRigid, got {other:?}"),
    }
    // Past escalate: definite, refused.
    match transform_rigid(&b, &with_c0x(1.0 + kesc * 2.0), Tol::witness()) {
        Err(TransformError::NotRigid { .. }) => {}
        other => panic!("definite defect: expected NotRigid, got {other:?}"),
    }
    // Improper rotation built from an honest axis-angle then negated
    // column: det −1 with perfect columns.
    let r = Mat3::rotation_about(Vec3::new(0.0, 0.0, 1.0), FRAC_PI_3);
    let refl = Mat3::from_cols(r.c0, r.c1, Vec3::zero() - r.c2);
    match transform_rigid(&b, &Affine3::from_parts(refl, Vec3::zero()), Tol::witness()) {
        Err(TransformError::NotRigid { check }) => {
            assert_eq!(check, "transform_rigid_det_plus_one");
        }
        other => panic!("reflection: expected NotRigid(det), got {other:?}"),
    }
}

/// R1d: the same transform applied twice to the same body is
/// bit-identical — every point, every arena Debug line.
#[test]
fn transform_is_bit_deterministic() {
    let mut b = brick((0.0, 2.0), (0.0, 1.0), (0.0, 0.5));
    describe_as_intersections(&mut b);
    let map = Affine3::from_parts(
        Mat3::rotation_about(Vec3::new(1.0, 2.0, 3.0).normalize(), FRAC_PI_3),
        Vec3::new(0.1, -0.2, 0.3),
    );
    let t1 = transform_rigid(&b, &map, Tol::witness()).unwrap();
    let t2 = transform_rigid(&b, &map, Tol::witness()).unwrap();
    assert_eq!(
        arena_dump(&t1),
        arena_dump(&t2),
        "same map twice must be bit-identical"
    );
}

/// R1a: certificate honesty under composition. 32 successive
/// rotations by π/3-ish angles about mixed axes (worst-case rounding),
/// on a body whose edges carry Intersection descriptions with real
/// witnesses. After every step the op must either refuse typed or
/// yield a body whose witnesses ACTUALLY lie on both surfaces at
/// far-below-ε residuals — measured directly, not via the returned
/// certificate.
#[test]
fn rotation_composition_keeps_witness_residuals_honest() {
    let mut b = brick((0.0, 2.0), (0.0, 1.0), (0.0, 0.5));
    describe_as_intersections(&mut b);
    let axes = [
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        Vec3::new(1.0, 1.0, 1.0).normalize(),
        Vec3::new(-0.3, 0.7, 0.2).normalize(),
    ];
    let mut cur = b;
    let mut worst = 0.0f64;
    for i in 0..32 {
        let map = Affine3::from_parts(
            Mat3::rotation_about(axes[i % 4], FRAC_PI_3 + 0.01 * i as f64),
            Vec3::new(0.25, -0.5, 0.125),
        );
        cur = match transform_rigid(&cur, &map, Tol::witness()) {
            Ok(t) => t,
            Err(e) => panic!("step {i}: typed refusal {e:?} — fail-loud but surprising"),
        };
        let r = max_witness_residual(&cur);
        worst = worst.max(r);
    }
    tiers_ok(&cur);
    eprintln!("composition worst witness residual: {worst:e}");
    // 32 compositions of ~1-ulp rotations on unit-scale geometry:
    // residuals must sit orders of magnitude below ε=1e-9. If this
    // approaches ε the certificate is being carried by slack, not
    // honesty.
    assert!(worst < 1e-12, "witness residual degraded to {worst:e}");
}

/// R1a: near-ε margins — a long thin feature (10^7 aspect, 10^6
/// coordinates) rotated by π/3. Rounding at 1e6 scale is ~1e-10,
/// within 10x of the 1e-9 band: the op must either refuse typed or
/// produce a body that re-validates geometrically. Silence with a
/// degraded body is the only failure mode.
#[test]
fn long_thin_feature_transform_is_honest_either_way() {
    let mut b = brick((0.0, 1.0e6), (0.0, 0.1), (0.0, 0.1));
    describe_as_intersections(&mut b);
    let map = Affine3::from_parts(
        Mat3::rotation_about(Vec3::new(0.0, 0.0, 1.0), FRAC_PI_3),
        Vec3::zero(),
    );
    match transform_rigid(&b, &map, Tol::witness()) {
        Ok(t) => {
            tiers_ok(&t);
            let m0 = mass_properties(&b, Tol::witness()).unwrap();
            let m1 = mass_properties(&t, Tol::witness()).unwrap();
            let rel = ((m1.volume - m0.volume) / m0.volume).abs();
            eprintln!("long-thin took the Ok path; rel volume drift {rel:e}");
            assert!(rel < 1e-9, "volume drifted rel {rel:e} under a rigid map");
        }
        Err(e) => {
            // Typed refusal at scale is acceptable (fail-loud), but it
            // must be the certification door, not corruption.
            match e {
                TransformError::Certify { .. } => {}
                other => panic!("expected Certify refusal at scale, got {other:?}"),
            }
        }
    }
}

/// R1a escalation: at 1e9 coordinates the rotation rounding (~1e-7)
/// exceeds the escalate threshold (1e-8) — the door must FIRE typed
/// (Certify or a decided refusal), or, if it passes, the result must
/// still re-validate; silent degradation (Ok + tier-3 failure) is the
/// falsifier.
#[test]
fn extreme_scale_fires_the_certify_door_or_stays_valid() {
    let mut b = brick((1.0e9, 1.0e9 + 1.0), (0.0, 1.0), (0.0, 1.0));
    describe_as_intersections(&mut b);
    let map = Affine3::from_parts(
        Mat3::rotation_about(Vec3::new(0.0, 0.0, 1.0), FRAC_PI_3),
        Vec3::zero(),
    );
    match transform_rigid(&b, &map, Tol::witness()) {
        Ok(t) => {
            eprintln!("1e9 scale took the Ok path");
            tiers_ok(&t);
        }
        Err(e) => {
            eprintln!("1e9 scale refused: {e:?}");
            assert!(matches!(e, TransformError::Certify { .. }));
        }
    }
}

/// The worst distance of any Intersection-edge witness from either of
/// its two (plane) surfaces — the direct residual measurement.
fn max_witness_residual(b: &Body<f64>) -> f64 {
    use geom::Surface;
    use geom_brep::EdgeDescription;
    let mut worst = 0.0f64;
    for (_k, e) in b.edges() {
        let geom = match b.get_curve_geom(e.curve) {
            Some(topo::CurveGeom::Certified(ec)) => ec,
            _ => continue,
        };
        if let EdgeDescription::Intersection { s1, s2, witness } = geom.description() {
            for sk in [s1, s2] {
                if let Some(Surface::Plane { origin, normal, .. }) = b.get_surface(*sk) {
                    let d = (*witness - *origin).dot(normal.normalize()).abs();
                    worst = worst.max(d);
                }
            }
        }
    }
    worst
}

/// R1c: arena keys AND topology are untouched bit-level — every key
/// set identical, every edge/face/half-edge record identical except
/// the geometry the keys point at.
#[test]
fn keys_and_topology_are_bit_stable_under_rotation() {
    let mut b = brick((0.0, 2.0), (0.0, 1.0), (0.0, 0.5));
    describe_as_intersections(&mut b);
    let map = Affine3::from_parts(
        Mat3::rotation_about(Vec3::new(0.0, 1.0, 0.0), FRAC_PI_2),
        Vec3::new(-3.0, 7.0, 0.5),
    );
    let t = transform_rigid(&b, &map, Tol::witness()).unwrap();
    let keys = |bb: &Body<f64>| {
        (
            bb.points()
                .map(|(k, _)| format!("{k:?}"))
                .collect::<Vec<_>>(),
            bb.curves()
                .map(|(k, _)| format!("{k:?}"))
                .collect::<Vec<_>>(),
            bb.surfaces()
                .map(|(k, _)| format!("{k:?}"))
                .collect::<Vec<_>>(),
            bb.edges()
                .map(|(k, _)| format!("{k:?}"))
                .collect::<Vec<_>>(),
            bb.faces()
                .map(|(k, _)| format!("{k:?}"))
                .collect::<Vec<_>>(),
        )
    };
    assert_eq!(keys(&b), keys(&t), "arena key sets must be identical");
    // Topology records: edges carry (curve key, half-edge keys) — the
    // Debug forms must match EXCEPT nothing, since edge records hold
    // keys only. Same for faces (surface key + loops).
    let topo_dump = |bb: &Body<f64>| {
        use std::fmt::Write as _;
        let mut s = String::new();
        for (k, e) in bb.edges() {
            let _ = writeln!(
                s,
                "E {k:?} curve={:?} hp={:?} hm={:?}",
                e.curve, e.he_plus, e.he_minus
            );
        }
        for (k, f) in bb.faces() {
            let _ = writeln!(s, "F {k:?} {f:?}");
        }
        s
    };
    assert_eq!(
        topo_dump(&b),
        topo_dump(&t),
        "topology records must be bit-stable"
    );
}

/// Full bit-level dump of every arena (geometry AND topology).
fn arena_dump(b: &Body<f64>) -> String {
    use std::fmt::Write as _;
    let mut s = String::new();
    for (k, p) in b.points() {
        let _ = writeln!(
            s,
            "P {k:?} {:016x} {:016x} {:016x}",
            p.x.to_bits(),
            p.y.to_bits(),
            p.z.to_bits()
        );
    }
    for (k, c) in b.curves() {
        let _ = writeln!(s, "C {k:?} {c:?}");
    }
    for (k, sf) in b.surfaces() {
        let _ = writeln!(s, "S {k:?} {sf:?}");
    }
    for (k, e) in b.edges() {
        let _ = writeln!(s, "E {k:?} {e:?}");
    }
    for (k, f) in b.faces() {
        let _ = writeln!(s, "F {k:?} {f:?}");
    }
    s
}
