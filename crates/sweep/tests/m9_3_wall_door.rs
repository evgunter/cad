//! M9-3 PR-A — the wall and the door (CONTACT-DESIGN C8 at the
//! boolean): touching whose margins decide one carrier glues, a `Rest`
//! pair opening the cosurface reduction rung and the carrier lump at
//! both wall sites; a tangency the DEV-1 witness lane verifies descends
//! to the second-order sector trilean, save a union whose ruling runs
//! through a plane face's interior, which refuses until the reduction
//! records a face–face curve touch.
//!
//! The canonical reachability fixture is the two-peg kernel shape's
//! core: a bored plate (through-hole subtract) and an exactly-filling
//! peg, both walls authored as three-arc cylinders on one carrier
//! (`demos/README.md` two-peg; CONTACT-DESIGN C7).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common::operands::{slab as plate, three_arc_cylinder};
use crate::common::three_arc;
use geom_core::k_stats::Bracket;
use geom_core::{Point2, Point3, Sign, Tol};
use profile::SketchPlane;
use sweep::test_support::{extruded, finished};
use topo::{AtRestBody, BooleanError, BooleanResult};

/// A radius-`r` three-arc cylinder at (2, 2), z ∈ [z0, z0 + h] (the
/// boss_union authorship: three 120° arcs on ONE cylinder surface).
fn cyl(z0: f64, h: f64, r: f64) -> AtRestBody<f64> {
    let cyl = three_arc_cylinder(Point2::new(2.0, 2.0), r, z0, h, 0.0);
    finished("the cylinder", cyl, Tol::witness())
}

/// The bored plate: a through-hole subtract (the shipped transverse
/// lane) leaving three bore-wall faces on one cylinder carrier.
fn bored_plate() -> AtRestBody<f64> {
    let plate = finished("the plate", plate(), Tol::witness());
    match topo::subtract(&plate, &cyl(-0.2, 1.4, 0.5), Tol::witness())
        .expect("the through-hole subtract is the shipped transverse lane")
    {
        BooleanResult::Body(b) => b.body,
        BooleanResult::Empty => panic!("a bored plate cannot be empty"),
    }
}

/// The opened wall: with the nine bore×peg wall pairs glued `Rest`,
/// classification RUNS — the reduction rung produces the v-v record
/// family, and the v-v lane's carrier lump consumes the glued pairs (the recl wall,
/// previously an untyped `ClassificationInvariant`, is gone). The op
/// then either succeeds with the closed-form volume or refuses typed
/// strictly DOWNSTREAM of classification.
#[test]
fn rest_two_peg_reaches_downstream_of_classification() {
    let bored = bored_plate();
    let peg = cyl(0.0, 1.0, 0.5);
    let bracket = Bracket::open();
    let out = topo::union(&bored, &peg, Tol::witness());
    let log = bracket.finish();
    let v = log.verdicts;
    // The carrier ladder's cylinder reading ran — the glued descent
    // executed rather than being skipped past (telemetry from birth) —
    // and read every glued wall pair coincident: the peg
    // fills the bore exactly, so each reach decides zero, and none
    // escalates.
    let reach: Vec<_> = v
        .iter()
        .filter(|x| x.predicate == "carrier_cyl_reach")
        .collect();
    assert!(
        !reach.is_empty(),
        "carrier_cyl_reach never reached the funnel — the glued descent did not run"
    );
    assert!(
        reach.iter().all(|x| x.sign == geom_core::Sign::Zero),
        "every glued wall pair reads coincident: {reach:?}"
    );
    assert!(
        !log.escalations
            .iter()
            .any(|e| e.source.predicate == Some("carrier_cyl_reach")),
        "no glued wall pair's reach escalates: {:?}",
        log.escalations
    );
    match out {
        Ok(BooleanResult::Body(b)) => {
            // Exactly additive against the closed-form oracle: the peg
            // fills the bore exactly, so the union is the unbored
            // plate's volume, exactly (the C7-lane statement).
            let vol =
                topo::test_support::pinned_volume("the measured body", &b.body, Tol::witness())
                    .volume;
            assert_eq!(vol, 16.0, "exactly-additive volume (closed form)");
            // The surviving rim arcs sit between two COPLANAR planar
            // faces (plate cap × peg cap): the surfaces under-
            // determine the locus, so the D6 pass re-describes the
            // stale intersection citations CONVENTIONALLY on the
            // unchanged circle carriers — the arrival the curved
            // smooth-seam `JoinDesync` door demands (the red half of
            // this row was the measured refusal before the
            // conventional-arc lane existed).
            //
            // The peg's ends are flush with the plate's faces —
            // continuations the union merges — so no rim
            // circle survives between two coplanar faces.
            let rims = b
                .body
                .edges()
                .filter(|(_, e)| {
                    b.body
                        .get_curve_geom(e.curve)
                        .and_then(|g| g.certified())
                        .is_some_and(|c| matches!(c.carrier(), geom::Curve3::Circle { .. }))
                })
                .count();
            assert_eq!(rims, 0, "the flush peg ends merge into the plate's faces");
        }
        Ok(BooleanResult::Empty) => panic!("a filled plate cannot be empty"),
        // PR-B: the zip's band closure landed — the union SUCCEEDS
        // (the Ok arm above carries the closed-form volume oracle);
        // any refusal is a regression of the opened lane.
        Err(err) => panic!("the exactly-filling union must succeed: {err:?}"),
    }
}

/// A horizontal three-arc cylinder (axis +y at height `zc`, radius
/// 0.5) with a meridian SEAM on its lowest ruling (profile vertices
/// at 60°/180°/300° in sketch coordinates), spanning y ∈ [0.5, 3.5].
fn lying_cyl(zc: f64) -> AtRestBody<f64> {
    // Sketch frame: sketch x → world z, sketch y → world x, normal
    // (extrusion) +y. Disc centre at world (x = 2, z = zc).
    let plane =
        SketchPlane::from_frame(geom_core::RigidMotion::axes_zx(Point3::new(0.0, 0.5, 0.0)));
    let cyl = extruded(
        plane,
        vec![three_arc(Point2::new(zc, 2.0), 0.5, 60.0)],
        3.0,
        Tol::witness(),
    );
    finished("the lying cylinder", cyl, Tol::witness())
}

/// The Tangent door on the witness lane's own rows: an in-band gap
/// ESCALATES, and the touching pair VERIFIES, and its union is the two
/// solids touching along the cylinder's edge on the ruling, which lies
/// in the plate top's interior: an edge–face record, no face–face one.
#[test]
fn tangent_door_escalates_and_verifies() {
    let a = finished("the plate", plate(), Tol::witness());
    // An in-band gap (the geometric mean of the run's band, so the
    // row holds at every sampled ε): ESCALATES — the sliver is a
    // sliver (C4's must-verify-DEFINITE list is never bridged).
    let band = geom_core::Band::linear(Tol::witness()).unwrap();
    let grazing = lying_cyl(1.5 + (band.zero() * band.escalate()).sqrt());
    let err =
        topo::union(&a, &grazing, Tol::witness()).expect_err("an in-band tangency gap escalates");
    assert!(matches!(err, BooleanError::Escalated { .. }), "{err:?}");
    // The genuinely-touching pair verifies.
    let resting = lying_cyl(1.5);
    let bracket = Bracket::open();
    let out = topo::union(&a, &resting, Tol::witness());
    let v = bracket.finish().verdicts;
    // A verdict names its row, not its caller: any `pc_parallel_gap` Zero
    // in the bracket satisfies this, the witness lane's among them.
    assert!(
        v.iter()
            .any(|x| x.predicate == "pc_parallel_gap" && x.sign == Sign::Zero),
        "the witness lane must have derived the ruling from the section's tangent gap"
    );
    match out {
        Ok(topo::BooleanResult::Body(r)) => {
            assert_eq!(r.body.solids().count(), 2, "two solids");
            assert_eq!(r.contacts.edge_face().count(), 1, "the edge on the ruling");
            assert_eq!(
                r.contacts.face_face().count(),
                0,
                "no face holds the ruling inside"
            );
        }
        other => panic!("the verified tangent union: {:?}", other.map(|_| ())),
    }
}
