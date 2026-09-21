//! **LANE-2 review probes (R2)** — the tier-3′ doors run as a user
//! would run them on a body with a declared straddling face pair, at
//! `f64` and at `Dual64`, the verdicts and `Display` texts printed
//! beside the assertions (`--no-capture` to read them).
//!
//! What the rows pin, on the head of `scalar/lane-2`:
//!
//! 1. the certified door at `f64` certifies the declared seat;
//! 2. the `_structural` door at `f64` on the SAME declared seat refuses
//!    the declared pair as `CensusLaneUnsupported` and leaves the two
//!    crossings hard — the pass holds no region door at any scalar,
//!    `f64` included — and the refusal's `Display` tells the `f64`
//!    caller to "replay the body at f64";
//! 3. the `_structural` door at `Dual64` says the same, shape for
//!    shape, as the unit's own row states;
//! 4. the undeclared seat through the `_structural` door at `f64`
//!    carries no lane refusal (no declaration, no consult).
//!
//! Row 2 is the one that reads differently at the merge base, where
//! `f64`'s `ChartRegionLane` impl handed the `_structural` twin the
//! doors through `AtRestPolicy`'s supertrait and the declared seat
//! certified through it.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use geom_core::{Dual64, Tol};
use topo::{Body, CensusContact, ContactRecords, FaceKey, PatchContact, ValidationError};

fn declared(pair: (FaceKey, FaceKey)) -> ContactRecords {
    ContactRecords {
        patches: vec![PatchContact {
            face_a: pair.0,
            face_b: pair.1,
        }],
        ..ContactRecords::default()
    }
}

fn lane_refusals(errors: &[ValidationError]) -> Vec<&ValidationError> {
    errors
        .iter()
        .filter(|e| matches!(e, ValidationError::CensusLaneUnsupported { .. }))
        .collect()
}

fn crossings(errors: &[ValidationError]) -> usize {
    errors
        .iter()
        .filter(|e| {
            matches!(
                e,
                ValidationError::UndeclaredContact {
                    contact: CensusContact::EdgeEdgeCross { .. },
                    ..
                }
            )
        })
        .count()
}

fn show(label: &str, r: &Result<(), Vec<ValidationError>>) {
    match r {
        Ok(()) => println!("LANE2R2|{label}|Ok"),
        Err(errors) => {
            for e in errors {
                println!("LANE2R2|{label}|Err|{e:?}|{e}");
            }
        }
    }
}

/// The straddle seat at `Dual64`, keyed as the `f64` seat is (one
/// arena order), as the unit's own `Dual64` row builds it.
fn dual_seat(tol: Tol) -> (Body<Dual64>, (FaceKey, FaceKey)) {
    let post: common::Prism<Dual64> = common::prism_z(
        &[(0.30, 0.20), (0.60, 0.20), (0.60, 0.42), (0.30, 0.42)],
        0.0,
        0.5,
        tol,
    );
    let shelf: common::Prism<Dual64> = common::prism_z(
        &[(0.0, 0.0), (0.9, 0.0), (0.9, 0.30), (0.0, 0.30)],
        0.5,
        0.54,
        tol,
    );
    let mut body = post.body;
    let keys = topo::graft_disjoint_all_keyed(&mut body, &shelf.body, tol).unwrap();
    (body, (post.top_face, keys.face(shelf.bottom_face).unwrap()))
}

#[test]
fn the_declared_seat_through_the_four_doors_at_f64_and_the_structural_door_at_a_dual() {
    let tol = Tol::witness();
    let seat = common::straddle_seat(tol);
    let pair = (seat.post_top, seat.shelf_bottom);
    let records = declared(pair);

    let certified = topo::validate_pseudomanifold(&seat.body, &records, tol);
    show("f64|validate_pseudomanifold|declared", &certified);
    assert!(
        certified.is_ok(),
        "the certified door certifies the declared seat: {certified:?}"
    );
    let certified_cert = topo::validate_pseudomanifold_certificate(&seat.body, &records, tol);
    assert!(certified_cert.is_ok(), "{certified_cert:?}");

    let structural = topo::validate_pseudomanifold_structural(&seat.body, &records, tol);
    show(
        "f64|validate_pseudomanifold_structural|declared",
        &structural,
    );
    let structural_cert =
        topo::validate_pseudomanifold_certificate_structural(&seat.body, &records, tol);
    assert_eq!(
        structural.clone().map(|()| ()),
        structural_cert.map(|_| ()),
        "the two structural forms agree"
    );
    let errors = structural.expect_err(
        "on the head the `_structural` door at f64 holds no region door and refuses the declared pair",
    );
    let refusals = lane_refusals(&errors);
    assert_eq!(refusals.len(), 1, "{errors:?}");
    assert_eq!(
        *refusals[0],
        ValidationError::CensusLaneUnsupported {
            subject: topo::CensusSubject::FacePair(pair.0, pair.1),
        }
    );
    let text = refusals[0].to_string();
    assert!(
        text.contains("this scalar has no certified chart-overlap lane")
            && text.contains("Replay the body at f64"),
        "the f64 caller of the structural door is told to replay at f64: {text}"
    );
    assert_eq!(
        crossings(&errors),
        2,
        "the declared pair backs neither crossing: {errors:?}"
    );

    let bare =
        topo::validate_pseudomanifold_structural(&seat.body, &ContactRecords::default(), tol);
    show("f64|validate_pseudomanifold_structural|undeclared", &bare);
    let bare = bare.expect_err("the undeclared seat's crossings are hard");
    assert!(
        lane_refusals(&bare).is_empty(),
        "no declaration, no consult: {bare:?}"
    );
    assert_eq!(crossings(&bare), 2);

    let (dual, dual_pair) = dual_seat(tol);
    assert_eq!(dual_pair, pair, "one arena order at both scalars");
    let at_dual = topo::validate_pseudomanifold_structural(&dual, &declared(dual_pair), tol);
    show(
        "dual64|validate_pseudomanifold_structural|declared",
        &at_dual,
    );
    let at_dual = at_dual.expect_err("a dual holds no region door");
    assert_eq!(lane_refusals(&at_dual).len(), 1, "{at_dual:?}");
    assert_eq!(crossings(&at_dual), 2);
    let shape = |errors: &[ValidationError]| -> Vec<String> {
        errors
            .iter()
            .map(|e| match e {
                ValidationError::UndeclaredContact { contact, .. } => format!("{contact:?}"),
                other => format!("{other:?}"),
            })
            .collect()
    };
    assert_eq!(
        shape(&at_dual),
        shape(&errors),
        "the same verdict shape at f64 and at the dual"
    );
}
