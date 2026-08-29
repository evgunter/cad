//! Issue #86: a legal double-subtract (crossing slots — the second
//! subtract's operand is itself a boolean result) must SUCCEED, not
//! panic in `set_edge_curve`'s tier-1 postcondition (OrphanGeometry).
//!
//! Shape: a 3×3×1 plate; slot 1 cut along y (full run in y, upper
//! half in z), slot 2 cut along x (crossing slot 1). Where the slots
//! cross, the second boolean's coplanar-face merge absorbs faces of
//! the first result whose surfaces stay alive only through first-
//! boolean `Intersection` descriptions; `describe_minted_edges` then
//! re-describes those edges through `set_edge_curve`, dropping the
//! last references — the orphan-hygiene cascade (curve → description
//! surfaces) is what this fixture pins. Both lanes (f64 and interval)
//! run the same full soundness check: tiers 1/2/3′ plus a rigid-
//! transform shake-out.
//!
//! The companion `kef` test pins the cascade door's REPORT: when the
//! killed curve's description holds the last reference to the dying
//! face's surface, the cascade (not `kef`'s explicit check) reaps it,
//! and `KefResult::killed_surface` must still name it.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod common;

use common::{flush_declarations, geometric_cube, prism_z};
use geom_core::Decide;
use geom_core::Tol;
use topo::{
    Body, BooleanBody, BooleanResult, subtract_with, validate, validate_closed,
    validate_pseudomanifold,
};

fn brick<T: Decide>(x: (f64, f64), y: (f64, f64), z: (f64, f64)) -> Body<T> {
    prism_z::<T>(&[(x.0, y.0), (x.1, y.0), (x.1, y.1), (x.0, y.1)], z.0, z.1).body
}

fn double_subtract_crossing_slots<T: Decide + geom_core::Bounds + geom_brep::PcurveFittedLane>() -> BooleanBody<T> {
    let a = brick::<T>((0.0, 3.0), (0.0, 3.0), (0.0, 1.0));
    let b1 = brick::<T>((1.0, 2.0), (-1.0, 4.0), (0.5, 1.5));
    let BooleanResult::Body(s1) =
        subtract_with(&a, &b1, &flush_declarations(&a, &b1), Tol::witness())
            .expect("first subtract succeeds")
    else {
        panic!("first subtract yields a body");
    };
    let b2 = brick::<T>((-1.0, 4.0), (1.0, 2.0), (0.5, 1.5));
    let BooleanResult::Body(s2) = subtract_with(
        &s1.body,
        &b2,
        &flush_declarations(&s1.body, &b2),
        Tol::witness(),
    )
    .expect("second subtract succeeds (issue #86)") else {
        panic!("second subtract yields a body");
    };
    s2
}

/// The full soundness check, generic over the scalar lane: tiers 1/2,
/// tier 3′ with the op's own contact records, and a rigid-transform
/// shake-out (downstream consumers re-certify the result cleanly).
fn assert_result_sound<T: Decide + topo::PropsQuadLane>(out: &BooleanBody<T>) {
    assert_eq!(validate(&out.body), Ok(()), "tier 1");
    assert_eq!(validate_closed(&out.body), Ok(()), "tier 2");
    assert_eq!(
        validate_pseudomanifold(&out.body, &out.contacts, Tol::witness()),
        Ok(()),
        "tier 3′"
    );
    let f = T::from_f64;
    let moved = topo::transform_rigid(
        &out.body,
        &geom_core::Affine3::translation(geom_core::Vec3::new(f(1.0), f(2.0), f(3.0))),
        Tol::witness(),
    )
    .expect("transform of the double-subtract result");
    assert_eq!(validate(&moved), Ok(()), "tier 1 after transform");
    assert_eq!(validate_closed(&moved), Ok(()), "tier 2 after transform");
}

#[test]
fn double_subtract_crossing_slots_succeeds() {
    assert_result_sound(&double_subtract_crossing_slots::<f64>());
}

#[cfg(feature = "interval")]
mod interval {
    use super::*;

    #[test]
    fn double_subtract_crossing_slots_interval() {
        assert_result_sound(&double_subtract_crossing_slots::<geom_core::Interval>());
    }
}

/// Cascade-door report (adapted from the PR 90 review probe): an edge
/// whose `Intersection` description holds the LAST reference to the
/// dying face's surface — the curve-removal cascade (not `kef`'s
/// explicit check, which runs after it) reaps the surface, and
/// `KefResult::killed_surface` must still report it. `geometric_cube`
/// leaves edges on their conventional chords, so the one upgraded
/// description is the surface's only non-face reference.
#[test]
fn kef_cascade_reports_killed_surface() {
    let cube = geometric_cube::<f64>();
    let mut body: Body<f64> = cube.body;

    // Pick any edge; derive its two faces and their surfaces.
    let (edge_key, edge) = body.edges().next().map(|(k, e)| (k, e.clone())).unwrap();
    let face_of = |body: &Body<f64>, he| {
        let l = body.get_half_edge(he).unwrap().parent_loop;
        body.get_loop(l).unwrap().face
    };
    let f_plus = face_of(&body, edge.he_plus);
    let s_plus = body.get_face(f_plus).unwrap().surface;
    let s_minus = body
        .get_face(face_of(&body, edge.he_minus))
        .unwrap()
        .surface;
    assert_ne!(s_plus, s_minus, "cube faces carry distinct planes");

    // Upgrade just this edge to an honest Intersection description:
    // its two surfaces are now description-referenced by this curve.
    let start = body.get_half_edge(edge.he_plus).unwrap().start;
    let end = body.half_edge_end(edge.he_plus).unwrap();
    let p0 = *body
        .get_point(body.get_vertex(start).unwrap().point)
        .unwrap();
    let p1 = *body.get_point(body.get_vertex(end).unwrap().point).unwrap();
    let mut spec = geom_brep::EdgeCurveSpec::line_between(p0, p1);
    spec.description = geom_brep::EdgeDescriptionSpec::Intersection {
        s1: s_plus,
        s2: s_minus,
        witness: p0.lerp(p1, 0.5),
    };
    body.set_edge_curve(edge_key, spec, Tol::witness()).unwrap();

    // Kill the plus side's face through this edge. The dying face's
    // surface is referenced only by (a) the face itself and (b) the
    // doomed curve's description — post-#86 the cascade is what
    // actually reaps it, and the report must come through that door.
    let result = body.kef(edge.he_plus).unwrap();
    assert_eq!(result.killed_face, f_plus);
    assert!(
        body.get_surface(s_plus).is_none(),
        "dying face's surface must be reaped (tier 1 forbids orphans)"
    );
    assert_eq!(
        result.killed_surface,
        Some(s_plus),
        "killed_surface must report the cascade-reaped surface"
    );
    assert!(
        body.get_surface(s_minus).is_some(),
        "the surviving face's surface stays"
    );
    assert_eq!(validate(&body), Ok(()), "tier 1 after kef");
}
