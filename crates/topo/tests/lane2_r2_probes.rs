//! **The declared straddle seat through the four tier-3′ doors at
//! `f64`**, as a user runs them: the two certified doors certify the
//! declared seat; the two `_structural` doors, holding no region door
//! at ANY scalar, refuse the declared pair as `CensusLaneUnsupported`
//! on the same body and leave the two crossings the declaration backs
//! as hard findings — and the refusal's `Display` tells the `f64`
//! caller to replay the body at `f64`
//! (`work/atrest/census-lane-unsupported-display-names-the-scalar-not-the-door.md`).
//! The undeclared seat through the `_structural` door carries no lane
//! refusal: no declaration, no consult. The same seat at `Dual64`
//! through the `_structural` door is `mate9_crossing_rung`'s row.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use geom_core::Tol;
use topo::{CensusContact, ContactRecords, FaceKey, PatchContact, ValidationError};

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

#[test]
fn the_declared_seat_through_the_four_doors_at_f64() {
    let tol = Tol::witness();
    let seat = common::straddle_seat(tol);
    let pair = (seat.post_top, seat.shelf_bottom);
    let records = declared(pair);

    let certified = topo::validate_pseudomanifold(&seat.body, &records, tol);
    assert!(
        certified.is_ok(),
        "the certified door certifies the declared seat: {certified:?}"
    );
    let certified_cert = topo::validate_pseudomanifold_certificate(&seat.body, &records, tol);
    assert!(certified_cert.is_ok(), "{certified_cert:?}");

    let structural = topo::validate_pseudomanifold_structural(&seat.body, &records, tol);
    let structural_cert =
        topo::validate_pseudomanifold_certificate_structural(&seat.body, &records, tol);
    assert_eq!(
        structural.clone().map(|()| ()),
        structural_cert.map(|_| ()),
        "the two structural forms agree"
    );
    let errors = structural.expect_err(
        "the `_structural` door at f64 holds no region door and refuses the declared pair",
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
        topo::validate_pseudomanifold_structural(&seat.body, &ContactRecords::default(), tol)
            .expect_err("the undeclared seat's crossings are hard");
    assert!(
        lane_refusals(&bare).is_empty(),
        "no declaration, no consult: {bare:?}"
    );
    assert_eq!(crossings(&bare), 2);
}
