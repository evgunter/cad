//! **The census records what it decides** (`topo::record_at_rest`;
//! D1 (ii), (iii); D10): a touch between two groups of a body at rest
//! is a row of the census's own Zero and a record citing it, which the
//! census's next pass confirms; a touch inside one group stays a
//! finding, and so does interpenetration.
//!
//! The subject is the straddle seat (`common::straddle_seat`): a post
//! whose cap overhangs the edge of a shelf it rests under, two solids
//! meeting in vertex-on-face, edge-in-face and edge-crossing events.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use geom_core::Tol;
use std::collections::BTreeMap;
use topo::{Body, ContactRecords, SolidKey, ValidationError};

fn census(body: &Body<f64>) -> impl FnMut(&ContactRecords) -> Vec<ValidationError> + '_ {
    |records| {
        topo::validate_pseudomanifold(body, records, Tol::witness())
            .err()
            .unwrap_or_default()
    }
}

/// Each solid its own group.
fn by_solid(body: &Body<f64>) -> BTreeMap<SolidKey, u32> {
    body.solids().map(|(s, _)| s).zip(0..).collect()
}

/// **Two groups**: every touch the bare census finds between the post
/// and the shelf is recorded, each record citing its own row from the
/// first row number the caller gave, each row the census's Zero at
/// rest, and the records confirm. Red if a touch is left a finding, a
/// record cites no row of its own, or a record refuses on the census's
/// next pass.
#[test]
fn a_touch_between_two_groups_is_a_row_and_a_record_citing_it() {
    let seat = common::straddle_seat(Tol::witness());
    let groups = by_solid(&seat.body);
    let bare = census(&seat.body)(&ContactRecords::default());
    assert!(
        !bare.is_empty(),
        "the premise: the bare seat's touches are findings"
    );
    let first = 7;
    let recorded = topo::record_at_rest(
        &seat.body,
        &ContactRecords::default(),
        census(&seat.body),
        &|s| groups.get(&s).copied(),
        first,
    );
    assert!(recorded.findings.is_empty(), "{:?}", recorded.findings);
    assert!(!recorded.rows.is_empty());
    assert!(
        recorded
            .rows
            .iter()
            .all(|r| r.site == topo::DecisionSite::CensusAtRest),
        "{:?}",
        recorded.rows
    );
    let rows = u32::try_from(recorded.rows.len()).unwrap();
    let cited = recorded.records.decided();
    assert!(
        cited.iter().all(|k| (first..first + rows).contains(k)),
        "each record cites a row of this door's: {cited:?}"
    );
    assert_eq!(
        topo::validate_pseudomanifold(&seat.body, &recorded.records, Tol::witness()),
        Ok(()),
        "the records confirm"
    );
}

/// **One group**: the same seat with both solids in one group records
/// nothing, and the census's findings come back as it found them — the
/// op's finished-body gate. Red if a touch inside one group is
/// recorded.
#[test]
fn a_touch_inside_one_group_stays_a_finding() {
    let seat = common::straddle_seat(Tol::witness());
    let bare = census(&seat.body)(&ContactRecords::default());
    let recorded = topo::record_at_rest(
        &seat.body,
        &ContactRecords::default(),
        census(&seat.body),
        &|_| Some(0),
        0,
    );
    assert!(recorded.rows.is_empty() && recorded.records.is_empty());
    assert_eq!(recorded.findings, bare);
}

/// **Interpenetration is not a coincidence**: a post pushed up into the
/// shelf pierces it, and the pierce stays a finding however the solids
/// are grouped. Red if a pierce is recorded.
#[test]
fn a_pierce_between_two_groups_stays_a_finding() {
    let tol = Tol::witness();
    let post: common::Prism<f64> = common::prism_z(
        &[(0.30, 0.10), (0.60, 0.10), (0.60, 0.20), (0.30, 0.20)],
        0.0,
        0.52,
        tol,
    );
    let shelf: common::Prism<f64> = common::prism_z(
        &[(0.0, 0.0), (0.9, 0.0), (0.9, 0.30), (0.0, 0.30)],
        0.5,
        0.54,
        tol,
    );
    let mut body = post.body;
    topo::graft_disjoint_all_keyed(&mut body, &shelf.body).unwrap();
    let groups = by_solid(&body);
    let recorded = topo::record_at_rest(
        &body,
        &ContactRecords::default(),
        census(&body),
        &|s| groups.get(&s).copied(),
        0,
    );
    let pierces: Vec<(topo::Cell, topo::Cell)> = recorded
        .findings
        .iter()
        .filter_map(|e| match e {
            ValidationError::UnrecordedContact {
                contact: topo::CensusContact::EdgeFacePierce { edge, face },
                margin: None,
                ..
            } => Some((topo::Cell::Edge(*edge), topo::Cell::Face(*face))),
            _ => None,
        })
        .collect();
    assert!(!pierces.is_empty(), "{:?}", recorded.findings);
    assert!(
        recorded
            .records
            .cell_pairs()
            .all(|pair| !pierces.contains(&pair)),
        "no pierce is recorded: {:?}",
        recorded.records
    );
}
