//! **The census's point-in-face refusal carries its own cause.**
//!
//! `contfp` has four typed refusals. One of them —
//! `ContainError::Escalated` — carries an `Indeterminate` a predicate
//! really metred. The other three carry no quantity at all: an
//! arc-bearing loop no walk expresses, an exhausted parity schedule,
//! unwalkable topology. The tier-3′ census used to answer all three
//! with `CensusEscalated` over an `Indeterminate` it MINTED —
//! predicate `pm_census_containment`, margin `Invalid` — which claims
//! a named predicate was posed and came back poisoned, in the one
//! field a reader uses to judge how close the call was. Nothing was
//! posed and nothing was poisoned.
//!
//! **The refusal arm's fixture is crate-internal.** A lens cap — two
//! arcs of two circles over two vertices — is walked on its carriers,
//! so a box standing on it is a contact the census DECIDES, which the
//! row below pins. The arm that is left refuses a loop with a spiric or
//! spline edge where every ray from the point could meet it, and no body
//! that reaches the census through the public door has one: the
//! sectioned torus vessel's cavity, whose moved section cap is bounded
//! by a spiric rim, stops at its volume first. So the carriage and the
//! "no fabricated margin" invariant are executed on a hand-built spiric
//! cap in `topo`'s census unit tests
//! (`a_spiric_caps_refusal_reaches_the_census_as_itself`).
//! `RayExhausted` needs a configuration every one of sixteen directions
//! grazes, and unwalkable topology cannot arrive through the public door
//! at all — `validate_pseudomanifold` runs referential integrity before
//! the census; both are routed by the same match and pinned by the
//! `CensusUnsupportedCause` rows in `editor-core`'s attribution suite,
//! which is disclosed, not claimed as executed coverage.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Affine3, Point2, Tol, Vec3};
use profile::{Profile, SketchPlane, test_support::bulge_loop};
use sweep::{Extrusion, extrude};
use topo::{Body, ContactRecords, ValidationError};

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

/// A LENS extruded z0..z0+h: a profile loop of TWO vertices joined by
/// two arcs of different circles. Its caps are planar, and the polygon
/// through their two vertices has no area.
fn lens(z0: f64, h: f64) -> Body<f64> {
    let tol = Tol::witness();
    let lp = bulge_loop(vec![(p2(-1.0, 0.0), 0.6), (p2(1.0, 0.0), 0.6)]);
    let plane = SketchPlane::new(Affine3::translation(Vec3::new(0.0, 0.0, z0)));
    let profile = Profile::new(plane, vec![lp]).validate(tol).unwrap();
    extrude(&profile, Extrusion::Distance(h), tol).unwrap().body
}

/// A lens solid in z ∈ [0, 1] and a small box standing on its top cap,
/// as ONE two-solid body. The box's four bottom vertices sit in the
/// cap's plane and well inside its outline, so the census's v-on-f
/// sweep passes the plane residual and then asks the cap whether it
/// contains the point.
fn lens_under_a_box() -> Body<f64> {
    let tol = Tol::witness();
    let mut body = lens(0.0, 1.0);
    let b = sweep::test_support::brick((-0.2, 0.2), (-0.1, 0.1), (1.0, 2.0), tol);
    topo::graft_disjoint(&mut body, &b, tol).expect("two disjoint solids in one body");
    body
}

fn census_findings(body: &Body<f64>) -> Vec<ValidationError> {
    match topo::validate_pseudomanifold(body, &ContactRecords::default(), Tol::witness()) {
        Ok(()) => panic!("a solid stands on a cap — the census must not certify the body"),
        Err(errors) => errors,
    }
}

/// INVARIANT: a lens cap is read as a region. Its two arcs are walked
/// on their own circles, so the box's bottom vertices — in the cap's
/// plane and inside its outline — are decided inside the cap, and the
/// census reports the contact rather than refusing the question.
#[test]
fn the_lens_cap_is_read_as_a_region() {
    let findings = census_findings(&lens_under_a_box());
    let refused: Vec<_> = findings
        .iter()
        .filter(|e| {
            matches!(
                e,
                ValidationError::CensusUnsupported {
                    cause: topo::CensusUnsupportedCause::Containment(_),
                    ..
                }
            )
        })
        .collect();
    assert!(
        refused.is_empty(),
        "the cap's loop is walked on its carriers; nothing is refused: {refused:?}"
    );
    let inside = findings
        .iter()
        .filter(|e| {
            matches!(
                e,
                ValidationError::UndeclaredContact {
                    contact: topo::CensusContact::VertexOnFace { .. },
                    ..
                }
            )
        })
        .count();
    assert_eq!(
        inside, 4,
        "each of the box's four bottom vertices lies inside the cap: {findings:?}"
    );
}
