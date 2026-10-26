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
/// flush sides, which share interior facing the same way
/// (`SameSideInterference`). Recording those touches would erase that
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
            .any(|e| matches!(e, ValidationError::SameSideInterference { .. })),
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

/// The census over two blocks, each its own group, recorded
/// (`record_at_rest`), and the flush pairs between them
/// (`AtRestPolicy::flush_at_rest`).
fn at_rest(body: &Body<f64>) -> (topo::RecordedAtRest, topo::FlushAtRest) {
    let groups = by_solid(body);
    let recorded = topo::record_at_rest(
        body,
        &ContactRecords::default(),
        census(body),
        &|s| groups.get(&s).copied(),
        0,
    );
    let band = geom_core::Band::linear(Tol::witness()).unwrap();
    let flush =
        <f64 as topo::AtRestPolicy>::flush_at_rest(body, &|s| groups.get(&s).copied(), band);
    (recorded, flush)
}

/// The face each of a row's two cells is, `None` where one is not a face.
fn faces_of(row: &topo::Coincidence) -> Option<[topo::FaceKey; 2]> {
    let face = |cell: topo::RowCell| match cell {
        topo::RowCell::Result {
            cell: topo::Cell::Face(f),
        } => Some(f),
        _ => None,
    };
    Some([face(row.cells[0])?, face(row.cells[1])?])
}

/// The outward normal of a face of a block: the carrier's normal, turned
/// by the face's sense.
fn outward(body: &Body<f64>, face: topo::FaceKey) -> geom_core::Vec3<f64> {
    let f = body.get_face(face).unwrap();
    let geom::Surface::Plane { normal, .. } = body.get_surface(f.surface).unwrap() else {
        panic!("a block's faces are planes");
    };
    let n = *normal;
    if f.sense { n } else { -n }
}

/// **An area contact is one face pair, and a flush side is its own row**
/// (`[assertions]`). A block seated on a bigger block, flush with it on
/// one, two and three sides: the census records the seat as ONE
/// face-pair row (the block's bottom and the base's top, one carrier,
/// opposed) and one record citing it, which answers for the edges and
/// vertices bounding them; each flush side is one row of two faces on
/// one carrier facing the same way, which no record cites. Red if the
/// seat is recorded as its edge and vertex events, if a flush side is
/// missing or counted twice, or if any finding stands.
#[test]
fn a_seat_flush_on_n_sides_is_one_face_pair_and_n_flush_rows() {
    let base = ((0.0, 2.0), (0.0, 2.0), (0.0, 1.0));
    for (sides, top) in [
        (1, ((0.0, 1.0), (0.5, 1.5), (1.0, 2.0))),
        (2, ((0.0, 1.0), (0.0, 1.0), (1.0, 2.0))),
        (3, ((0.0, 2.0), (0.0, 1.0), (1.0, 2.0))),
    ] {
        let body = two_blocks(base, top);
        let (recorded, flush) = at_rest(&body);
        assert!(
            recorded.findings.is_empty() && flush.findings.is_empty(),
            "{sides} flush sides: {:?} {:?}",
            recorded.findings,
            flush.findings
        );
        assert_eq!(recorded.rows.len(), 1, "{sides}: {:?}", recorded.rows);
        let seat = recorded.rows[0];
        assert_eq!(seat.relation, topo::Relation::SameOpposite, "{sides}");
        let [a, b] = faces_of(&seat).expect("the seat's row names two faces");
        assert!(
            (outward(&body, a).z.abs() - 1.0).abs() < 1e-12
                && outward(&body, a).z == -outward(&body, b).z,
            "{sides}: the seat is the block's bottom on the base's top"
        );
        assert_eq!(recorded.records.rows().len(), 1, "{sides}: one record");
        assert_eq!(
            topo::validate_pseudomanifold(&body, &recorded.records, Tol::witness()),
            Ok(()),
            "{sides}: the record confirms"
        );
        assert_eq!(flush.rows.len(), sides, "{sides}: {:?}", flush.rows);
        for row in &flush.rows {
            assert_eq!(row.relation, topo::Relation::SameOriented, "{sides}");
            let [a, b] = faces_of(row).expect("a flush row names two faces");
            assert!(
                (outward(&body, a) - outward(&body, b)).norm() < 1e-12
                    && outward(&body, a).z.abs() < 1e-12,
                "{sides}: a flush row is two side walls facing one way"
            );
        }
    }
}

/// **An overhang is one face pair.** A block hanging over the base's
/// edge: its bottom and the base's top meet over part of each, their
/// boundary edges crossing where the base's top ends. One face-pair row,
/// no flush row, nothing else. Red if a crossing at the overhang is
/// recorded or found.
#[test]
fn an_overhang_is_one_face_pair() {
    let body = two_blocks(
        ((0.0, 2.0), (0.0, 2.0), (0.0, 1.0)),
        ((1.0, 3.0), (0.5, 1.5), (1.0, 2.0)),
    );
    let (recorded, flush) = at_rest(&body);
    assert!(recorded.findings.is_empty(), "{:?}", recorded.findings);
    assert_eq!(recorded.rows.len(), 1, "{:?}", recorded.rows);
    assert_eq!(recorded.rows[0].relation, topo::Relation::SameOpposite);
    assert!(
        flush.rows.is_empty() && flush.findings.is_empty(),
        "{flush:?}"
    );
}

/// **Two coplanar faces meeting only along a line are no face pair.** A
/// block beside the base, its bottom on the base's top plane and the two
/// meeting only along the base's edge: no area, so no face-pair row; the
/// line they share is the census's edge row, and the vertices bounding
/// it. Their fronts lie on one plane facing one way and meet at one
/// point, no segment, so they are no flush pair. Red if a face pair or a
/// flush row is recorded, or the line is not.
#[test]
fn coplanar_faces_meeting_only_along_a_line_are_no_face_pair() {
    let body = two_blocks(
        ((0.0, 2.0), (0.0, 1.0), (0.0, 1.0)),
        ((2.0, 3.0), (0.0, 1.0), (1.0, 2.0)),
    );
    let (recorded, flush) = at_rest(&body);
    assert!(recorded.findings.is_empty(), "{:?}", recorded.findings);
    assert!(
        recorded.rows.iter().all(|r| faces_of(r).is_none()),
        "no row names two faces: {:?}",
        recorded.rows
    );
    assert!(
        recorded.rows.iter().any(|r| matches!(
            r.cells,
            [
                topo::RowCell::Result {
                    cell: topo::Cell::Edge(_)
                },
                topo::RowCell::Result {
                    cell: topo::Cell::Edge(_)
                }
            ]
        )),
        "the shared line is an edge row: {:?}",
        recorded.rows
    );
    assert!(
        flush.rows.is_empty() && flush.findings.is_empty(),
        "{flush:?}"
    );
}

/// **A near-flush side: the seat is one face pair, and the side's own
/// pair is the flush door's finding.** A block on the base, its x = 0
/// side standing off the base's by a distance inside the band. The seat
/// is confirmed, one face pair, which answers for the cells bounding it:
/// the bottom edge along x = off, in band of the base's top
/// edge, no longer escalates. The x = 0 side's carrier reads in band, so
/// it is no flush row, and the flush door's finding names that pair.
///
/// What stays: the census still reads the in-band side walls' own
/// boundary cells against each other and escalates them, so the seat is
/// not recorded (`work/intent/an-in-band-face-pair-at-rest-is-not-one-finding.md`).
/// Red if the seat's pair is not found, if the near-flush pair is
/// decided, or if its finding names another pair.
#[test]
fn a_near_flush_side_is_its_pairs_finding_and_the_seat_is_one_pair() {
    let band = geom_core::Band::linear(Tol::witness()).unwrap();
    let off = 0.5 * (band.zero() + band.escalate());
    let body = two_blocks(
        ((0.0, 2.0), (0.0, 2.0), (0.0, 1.0)),
        ((off, 1.0), (0.5, 1.5), (1.0, 2.0)),
    );
    let (recorded, flush) = at_rest(&body);
    let seats: Vec<_> = recorded
        .findings
        .iter()
        .filter_map(|e| match e {
            ValidationError::UnrecordedContact {
                contact: topo::CensusContact::ConformalPatch { finding },
                ..
            } => Some([finding.pair.a, finding.pair.b]),
            _ => None,
        })
        .collect();
    let [[a, b]] = seats[..] else {
        panic!("one face pair: {:?}", recorded.findings);
    };
    assert!(
        outward(&body, a).z.abs() == 1.0 && outward(&body, a).z == -outward(&body, b).z,
        "the seat is the block's bottom on the base's top"
    );
    assert!(flush.rows.is_empty(), "{:?}", flush.rows);
    let [
        ValidationError::CensusUndecidable {
            a: topo::EntityId::Face(a),
            b: topo::EntityId::Face(b),
            ..
        },
    ] = flush.findings.as_slice()
    else {
        panic!("one finding, the near-flush pair's: {:?}", flush.findings);
    };
    for face in [*a, *b] {
        assert!(
            (outward(&body, face).x + 1.0).abs() < 1e-12,
            "the finding names the two x = 0 sides"
        );
    }
}
