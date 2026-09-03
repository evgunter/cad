//! Re-mint-at-transform pins (ruled with Ev, PR #83, 2026-07-24):
//! `transform_rigid` re-mints Intersection witnesses construction-
//! fresh from the MAPPED carrier at the pinned mid parameter, instead
//! of mapping the stored witness. These tests pin the mechanism on
//! the real code path.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use common::prism_z;
use geom_brep::{CERT_SAMPLES, EdgeDescription};
use geom_core::Tol;
use geom_core::{Affine3, Mat3, Vec3};
use topo::transform_rigid;

fn rot_trans() -> Affine3<f64> {
    // A rounding-hostile map: rotation by 1 radian about a skew axis
    // plus a non-dyadic translation.
    let axis = Vec3::new(1.0, 2.0, 3.0).normalize();
    Affine3::from_parts(Mat3::rotation_about(axis, 1.0), Vec3::new(0.1, 0.2, 0.3))
}

/// THE PIN: after transform, every Intersection witness is BIT-EQUAL
/// to the mapped carrier evaluated at the pinned mid parameter — the
/// construction-fresh property. (The old path — mapping the stored
/// witness — agrees only to float noise; bit-equality distinguishes
/// the mechanisms.)
#[test]
fn witnesses_are_construction_fresh_bits() {
    // An L-shaped prism union: guarantees boolean-minted Intersection
    // edges with real seam geometry.
    let a = prism_z::<f64>(&[(0.0, 0.0), (2.0, 0.0), (2.0, 1.0), (0.0, 1.0)], 0.0, 1.0).body;
    let b = prism_z::<f64>(
        &[(1.0, 0.5), (3.0, 0.5), (3.0, 1.5), (1.0, 1.5)],
        0.25,
        0.75,
    )
    .body;
    let topo::BooleanResult::Body(bb) = topo::union(&a, &b, Tol::witness()).unwrap() else {
        panic!("overlapping union is a body")
    };
    let out = transform_rigid(&bb.body, &rot_trans(), Tol::witness()).unwrap();
    let mid = (CERT_SAMPLES - 1) / 2;
    let mut checked = 0usize;
    for (_k, e) in out.edges() {
        let Some(topo::CurveGeom::Certified(ec)) = out.get_curve_geom(e.curve) else {
            continue;
        };
        if let EdgeDescription::Intersection { witness, .. } = ec.description() {
            let fresh = ec.carrier().eval(ec.sample_param(mid));
            assert_eq!(
                (
                    witness.x.to_bits(),
                    witness.y.to_bits(),
                    witness.z.to_bits()
                ),
                (fresh.x.to_bits(), fresh.y.to_bits(), fresh.z.to_bits()),
                "witness must be construction-fresh (carrier'(mid)), not mapped"
            );
            checked += 1;
        }
    }
    assert!(
        checked >= 4,
        "expected several Intersection edges, got {checked}"
    );
}

/// Chains don't ratchet: five successive transforms all succeed and
/// the final body's witnesses are still construction-fresh bits.
#[test]
fn transform_chain_stays_fresh() {
    let a = prism_z::<f64>(&[(0.0, 0.0), (2.0, 0.0), (2.0, 1.0), (0.0, 1.0)], 0.0, 1.0).body;
    let b = prism_z::<f64>(
        &[(1.0, 0.5), (3.0, 0.5), (3.0, 1.5), (1.0, 1.5)],
        0.25,
        0.75,
    )
    .body;
    let topo::BooleanResult::Body(bb) = topo::union(&a, &b, Tol::witness()).unwrap() else {
        panic!("overlapping union is a body")
    };
    let mut cur = bb.body;
    for i in 1..=5 {
        cur = transform_rigid(&cur, &rot_trans(), Tol::witness())
            .unwrap_or_else(|e| panic!("chain step {i} refused: {e:?}"));
    }
    let mid = (CERT_SAMPLES - 1) / 2;
    for (_k, e) in cur.edges() {
        let Some(topo::CurveGeom::Certified(ec)) = cur.get_curve_geom(e.curve) else {
            continue;
        };
        if let EdgeDescription::Intersection { witness, .. } = ec.description() {
            let fresh = ec.carrier().eval(ec.sample_param(mid));
            assert_eq!(witness.x.to_bits(), fresh.x.to_bits());
            assert_eq!(witness.y.to_bits(), fresh.y.to_bits());
            assert_eq!(witness.z.to_bits(), fresh.z.to_bits());
        }
    }
}

/// Determinism unchanged: the same transform twice gives bit-identical
/// bodies (re-mint is a pure function of the mapped carrier).
#[test]
fn remint_is_deterministic() {
    let a = prism_z::<f64>(&[(0.0, 0.0), (2.0, 0.0), (2.0, 1.0), (0.0, 1.0)], 0.0, 1.0).body;
    let b = prism_z::<f64>(
        &[(1.0, 0.5), (3.0, 0.5), (3.0, 1.5), (1.0, 1.5)],
        0.25,
        0.75,
    )
    .body;
    let topo::BooleanResult::Body(bb) = topo::union(&a, &b, Tol::witness()).unwrap() else {
        panic!("overlapping union is a body")
    };
    let o1 = transform_rigid(&bb.body, &rot_trans(), Tol::witness()).unwrap();
    let o2 = transform_rigid(&bb.body, &rot_trans(), Tol::witness()).unwrap();
    assert_eq!(format!("{o1:?}"), format!("{o2:?}"));
}
