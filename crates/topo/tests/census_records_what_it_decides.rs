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
    // Each record cites exactly the rows that decided its own pair, and
    // each row's margin is the Zero the census decided for that touch.
    for record in recorded.records.rows() {
        let cells = record.cells();
        for backing in record.cites.iter() {
            let topo::Backing::Decided(k) = backing else {
                panic!("a minted record cites a row: {backing:?}");
            };
            let row = &recorded.rows[(k - first) as usize];
            let mut row_cells = row.cells.map(|c| match c {
                topo::RowCell::Result { cell } => cell,
                other => panic!("a census row's cell is the body's: {other:?}"),
            });
            row_cells.sort();
            assert_eq!(row_cells, cells, "record {cells:?} cites row {k}");
        }
    }
    let bare_rows: Vec<topo::Coincidence> =
        bare.iter().filter_map(topo::census_contact_row).collect();
    for row in &recorded.rows {
        assert!(
            bare_rows.contains(row),
            "each row is the bare census's own reading of its touch: {row:?}"
        );
    }
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

/// Two blocks one unit wide, grafted as two solids of one body.
fn two_blocks(
    (ax, ay, az): ((f64, f64), (f64, f64), (f64, f64)),
    (bx, by, bz): ((f64, f64), (f64, f64), (f64, f64)),
) -> Body<f64> {
    let block = |(x0, x1): (f64, f64), (y0, y1): (f64, f64), (z0, z1): (f64, f64)| {
        common::prism_z::<f64>(
            &[(x0, y0), (x1, y0), (x1, y1), (x0, y1)],
            z0,
            z1,
            Tol::witness(),
        )
        .body
    };
    let mut body = block(ax, ay, az);
    topo::graft_disjoint_all_keyed(&mut body, &block(bx, by, bz)).unwrap();
    body
}

/// **An overlap with flush sides records nothing.** Two blocks of two
/// groups overlapping by half their height, their side faces flush:
/// no edge pierces a face, and the census reads the overlap off the
/// touches between them (`CensusUndecidable`, "one passes into the
/// other where they touch"). Recording those touches would erase that
/// evidence and certify the pair with its volume counted twice, so the
/// pair has nothing recorded and its findings stay. Red if the
/// overlapping pair records a touch or comes back clean.
#[test]
fn an_overlap_with_flush_sides_records_nothing_and_stays_loud() {
    let body = two_blocks(
        ((0.0, 1.0), (0.0, 1.0), (0.0, 1.0)),
        ((0.0, 1.0), (0.0, 1.0), (0.5, 1.5)),
    );
    let groups = by_solid(&body);
    let recorded = topo::record_at_rest(
        &body,
        &ContactRecords::default(),
        census(&body),
        &|s| groups.get(&s).copied(),
        0,
    );
    assert!(
        recorded.records.is_empty() && recorded.rows.is_empty(),
        "{:?}",
        recorded.records
    );
    assert!(
        recorded
            .findings
            .iter()
            .any(|e| matches!(e, ValidationError::CensusUndecidable { .. })),
        "{:?}",
        recorded.findings
    );
}

/// **Stacked blocks certify.** The same two blocks one on the other,
/// meeting over the face between them and nowhere inside: every touch
/// is recorded, and the records confirm.
#[test]
fn stacked_blocks_certify_with_their_touches_recorded() {
    let body = two_blocks(
        ((0.0, 1.0), (0.0, 1.0), (0.0, 1.0)),
        ((0.0, 1.0), (0.0, 1.0), (1.0, 2.0)),
    );
    let groups = by_solid(&body);
    let recorded = topo::record_at_rest(
        &body,
        &ContactRecords::default(),
        census(&body),
        &|s| groups.get(&s).copied(),
        0,
    );
    assert!(recorded.findings.is_empty(), "{:?}", recorded.findings);
    assert!(!recorded.records.is_empty());
}

/// **Interpenetration is not a coincidence**: a post pushed up into the
/// shelf pierces it, the pierce stays a finding, and nothing else
/// between the two groups is recorded either. Red if a pierce, or any
/// touch of the pierced pair, is recorded.
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
        recorded.records.is_empty() && recorded.rows.is_empty(),
        "nothing between interpenetrating groups is recorded: {:?}",
        recorded.records
    );
}
