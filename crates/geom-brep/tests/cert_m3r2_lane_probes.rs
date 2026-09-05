//! CERT-M3 R2 adversarial probes at the certify door.
//!
//! What the two lane-free at-rest doors (`validate_pseudomanifold`,
//! `contact_marks`, and `validate_pseudomanifold_certificate` through
//! `tier3_local_checks`) now SKIP: `validate.rs` check 2 computes
//! `claimable = nurbs_lane.is_some() || !needs_nurbs_lane(..)` and calls
//! `recertify_via` only when `claimable`. So for an M7-8 edge the skip
//! is TOTAL — every check-2 verdict on that edge, not only the plane ×
//! NURBS limbs. These rows show a defect that has nothing to do with the
//! lane (an endpoint that drifted) reported through the lane-injected
//! call and unreachable through the lane-free one.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::shared::fixture::{quarter_cylinder_wall, segment, transverse_plane};
use crate::shared::tol::band;
use geom::Curve3;
use geom::{NurbsCurve3, NurbsSurface, Surface};
use geom_brep::keys::SurfaceKey;
use geom_brep::{CertifyError, EdgeCurve, EdgeCurveSpec, EdgeDescriptionSpec};
use geom_core::Point3;
use slotmap::SlotMap;

fn door_spec(
    plane: Surface<f64>,
    wall: NurbsSurface<f64>,
    carrier: NurbsCurve3<f64>,
) -> (
    impl Fn(SurfaceKey) -> Option<Surface<f64>>,
    EdgeCurveSpec<f64>,
) {
    let mut arena: SlotMap<SurfaceKey, Surface<f64>> = SlotMap::with_key();
    let s1 = arena.insert(plane);
    let s2 = arena.insert(Surface::Nurbs(std::sync::Arc::new(wall)));
    let carrier = Curve3::Nurbs(std::sync::Arc::new(carrier));
    let witness = carrier.eval(0.5);
    (
        move |k| arena.get(k).cloned(),
        EdgeCurveSpec {
            description: EdgeDescriptionSpec::Intersection { s1, s2, witness },
            carrier,
            param_start: 0.0,
            param_end: 1.0,
        },
    )
}

/// A non-lane defect on an M7-8 edge: the lane-injected door reports it,
/// the lane-free door cannot even reach it, and `needs_nurbs_lane` — the
/// predicate check 2 gates on — says yes either way, so check 2 skips
/// the edge whole.
#[test]
fn m3r2_a_non_lane_defect_on_an_m7_8_edge_is_unreachable_without_the_lane() {
    let carrier = segment(Point3::new(1.0, 0.0, 0.0), Point3::new(1.0, 0.0, 1.0));
    let ends = (carrier.eval(0.0), carrier.eval(1.0));
    let (arena, spec) = door_spec(transverse_plane(), quarter_cylinder_wall(), carrier);
    let edge = EdgeCurve::certify_nurbs_lane(spec, ends.0, ends.1, &arena, band())
        .expect("the stated carrier certifies through the attach door");

    // A drifted endpoint: nothing to do with the plane x NURBS lane.
    let drifted = Point3::new(1.0, 0.0, -0.25);

    println!("M3R2|needs_nurbs_lane={}", edge.needs_nurbs_lane(&arena));
    let with_lane = edge.recertify_via(
        drifted,
        ends.1,
        &arena,
        band(),
        Some(&geom_brep::plane_nurbs_limbs::<f64>),
    );
    println!("M3R2|with_lane   -> {with_lane:?}");
    let without = edge.recertify_via(drifted, ends.1, &arena, band(), None);
    println!("M3R2|without_lane -> {without:?}");

    assert!(edge.needs_nurbs_lane(&arena));
    assert!(
        with_lane.is_err(),
        "the endpoint drift is a real check-2 finding"
    );
    assert!(
        !matches!(with_lane, Err(CertifyError::Unimplemented)),
        "and it is NOT the class's standing refusal: {with_lane:?}"
    );
    assert!(
        matches!(without, Err(CertifyError::Unimplemented)),
        "without the lane the drift is indistinguishable from the class refusal: {without:?}"
    );
    // ...and `claimable` is false in validate.rs check 2 for this edge,
    // so the lane-free at-rest doors push NO error for it at all.
}
