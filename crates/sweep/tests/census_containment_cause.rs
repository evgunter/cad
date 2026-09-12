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
//! **RED-FIRST.** Before the repair, `no_fabricated_containment_margin`
//! fails with the minted diagnostic in hand and
//! `the_arc_loop_refusal_reaches_the_user_as_itself` fails for want of
//! any `CensusUnsupported` at all.
//!
//! **Only the arc-loop arm has a fixture.** `RayExhausted` and
//! `Corrupt` are routed by the same match and pinned by the
//! `CensusUnsupportedCause` rows in `editor-core`'s attribution suite,
//! but neither has an executed body here: an exhausted parity schedule
//! needs a configuration every one of sixteen directions grazes, and
//! unwalkable topology cannot arrive through the public door at all —
//! `validate_pseudomanifold` runs referential integrity before the
//! census. That is disclosed, not claimed as coverage.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Affine3, Point2, Tol, Vec3};
use profile::{Profile, ProfileLoop, ProfileVertex, RawLoop, SketchPlane};
use sweep::{Extrusion, extrude};
use topo::{Body, ContactRecords, ValidationError};

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

/// A LENS extruded z0..z0+h: a profile loop of TWO vertices joined by
/// two arcs of different circles. Its caps are planar and their outer
/// loop is `LoopShape::NoWalk` — every walk the kernel has either
/// needs three vertices for a polygon with area, or needs one circle.
fn lens(z0: f64, h: f64) -> Body<f64> {
    let tol = Tol::witness();
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(p2(-1.0, 0.0), 0.6),
        ProfileVertex::new(p2(1.0, 0.0), 0.6),
    ]);
    let plane = SketchPlane::new(Affine3::translation(Vec3::new(0.0, 0.0, z0)));
    let profile = Profile::new(plane, vec![lp]).validate(tol).unwrap();
    extrude(&profile, Extrusion::Distance(h), tol).unwrap().body
}

fn boxx(x0: f64, x1: f64, y0: f64, y1: f64, z0: f64, z1: f64) -> Body<f64> {
    let tol = Tol::witness();
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(p2(x0, y0), 0.0),
        ProfileVertex::new(p2(x1, y0), 0.0),
        ProfileVertex::new(p2(x1, y1), 0.0),
        ProfileVertex::new(p2(x0, y1), 0.0),
    ]);
    let plane = SketchPlane::new(Affine3::translation(Vec3::new(0.0, 0.0, z0)));
    let profile = Profile::new(plane, vec![lp]).validate(tol).unwrap();
    extrude(&profile, Extrusion::Distance(z1 - z0), tol)
        .unwrap()
        .body
}

/// A lens solid in z ∈ [0, 1] and a small box standing on its top cap,
/// as ONE two-solid body. The box's four bottom vertices sit in the
/// cap's plane and well inside its outline, so the census's v-on-f
/// sweep passes the plane residual and then asks the cap whether it
/// contains the point — the question the cap's loop has no walk for.
fn lens_under_a_box() -> Body<f64> {
    let tol = Tol::witness();
    let mut body = lens(0.0, 1.0);
    let b = boxx(-0.2, 0.2, -0.1, 0.1, 1.0, 2.0);
    topo::graft_disjoint(&mut body, &b, tol).expect("two disjoint solids in one body");
    body
}

fn census_findings(body: &Body<f64>) -> Vec<ValidationError> {
    match topo::validate_pseudomanifold(body, &ContactRecords::default(), Tol::witness()) {
        Ok(()) => panic!(
            "the box stands on a cap whose region no walk expresses — the census must \
             refuse, not certify"
        ),
        Err(errors) => errors,
    }
}

/// INVARIANT: no census finding carries an `Indeterminate` the census
/// invented. An escalation is what a predicate says when it MEASURED
/// and could not decide; a door that measured nothing has no margin to
/// report and must not be given one.
///
/// The falsifier is the literal tag the old site minted, matched on
/// the payload rather than on message prose: a rename of the tag must
/// not make this row pass.
#[test]
fn no_fabricated_containment_margin() {
    let findings = census_findings(&lens_under_a_box());
    let fabricated: Vec<_> = findings
        .iter()
        .filter(|e| {
            matches!(
                e,
                ValidationError::CensusEscalated { cause }
                    if cause.predicate == Some("pm_census_containment")
            )
        })
        .collect();
    assert!(
        fabricated.is_empty(),
        "the census minted a margin nothing metred: {fabricated:?} (all findings: {findings:?})"
    );
}

/// INVARIANT: the point-in-face door's refusal reaches the user AS
/// ITSELF — the arm, the loop it names, and the repair that moves it.
///
/// The subject is the FACE and not a pair: what refused is this face's
/// region, and the second entity differs at each of the helper's call
/// sites.
#[test]
fn the_arc_loop_refusal_reaches_the_user_as_itself() {
    let findings = census_findings(&lens_under_a_box());
    let carried: Vec<_> = findings
        .iter()
        .filter(|e| {
            matches!(
                e,
                ValidationError::CensusUnsupported {
                    subject: topo::CensusSubject::Entity(topo::EntityId::Face(_)),
                    cause: topo::CensusUnsupportedCause::Containment(
                        topo::ContainError::ArcLoopUnsupported { .. }
                    ),
                }
            )
        })
        .collect();
    assert!(
        !carried.is_empty(),
        "the cap's loop has no walk and the census must say so in the door's own words: \
         {findings:?}"
    );
    // The rendered sentence names the mechanism and a repair, so a
    // reader is not sent to the tolerance levers for a modelling fact.
    let shown = carried[0].to_string();
    for want in [
        "fewer than three vertices",
        "no available walk",
        "split an arc",
    ] {
        assert!(
            shown.contains(want),
            "the message must name {want:?} — the mechanism and its repair: {shown}"
        );
    }
    assert!(
        !shown.contains("lower the tolerance"),
        "an unexpressible loop is not an ill-conditioned one; the tolerance levers are \
         the wrong repair here: {shown}"
    );
}
