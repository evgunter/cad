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

/// **A near-flush side: the seat is one face pair, and the side is one
/// finding.** A block on the base, its x = 0 side standing off the
/// base's by a distance inside the band. The seat is confirmed, one face
/// pair, recorded and confirming. The x = 0 sides' carriers read in band,
/// so that pair is one could-not-look finding naming the two faces
/// (`[assertions]`), its cells' escalations its own: none stands. The two
/// sides face the same way but share no area thicker than the band, so
/// the finding is no evidence the two blocks overlap, and the seat is
/// still recorded. Red if any escalation stands, if the in-band pair is
/// reported more than once or names another pair, or if the seat is not
/// recorded.
#[test]
fn a_near_flush_side_is_its_pairs_finding_and_the_seat_is_one_pair() {
    let band = geom_core::Band::linear(Tol::witness()).unwrap();
    let off = 0.5 * (band.zero() + band.escalate());
    let body = two_blocks(
        ((0.0, 2.0), (0.0, 2.0), (0.0, 1.0)),
        ((off, 1.0), (0.5, 1.5), (1.0, 2.0)),
    );
    let (recorded, flush) = at_rest(&body);
    let [
        ValidationError::CensusUndecidable {
            a: topo::EntityId::Face(a),
            b: topo::EntityId::Face(b),
            reason: topo::Undecidable::ThinInBand,
            ..
        },
    ] = recorded.findings.as_slice()
    else {
        panic!(
            "one finding, the near-flush pair's: {:?}",
            recorded.findings
        );
    };
    for face in [*a, *b] {
        assert!(
            (outward(&body, face).x + 1.0).abs() < 1e-12,
            "the finding names the two x = 0 sides"
        );
    }
    let [seat] = recorded.rows[..] else {
        panic!("one row, the seat's: {:?}", recorded.rows);
    };
    assert_eq!(seat.relation, topo::Relation::SameOpposite);
    let [a, b] = faces_of(&seat).expect("the seat's row names two faces");
    assert!(
        outward(&body, a).z.abs() == 1.0 && outward(&body, a).z == -outward(&body, b).z,
        "the seat is the block's bottom on the base's top"
    );
    assert_eq!(recorded.records.rows().len(), 1, "one record, the seat's");
    assert_eq!(
        topo::validate_pseudomanifold(&body, &recorded.records, Tol::witness()).err(),
        Some(recorded.findings.clone()),
        "the seat's record confirms, and the in-band pair's finding is all that stands"
    );
    assert!(
        flush.rows.is_empty() && flush.findings.is_empty(),
        "the in-band pair is the census's, not the flush door's: {flush:?}"
    );
}

/// **Two blocks overlapping past the band, with aligned faces in band,
/// are never cleared.** The second block is lifted by a distance inside
/// the band, so its top lies in band of the first's top, both facing up,
/// and the two tops share area. Both materials lie behind both faces, so
/// such a pair hides an overlap rather than ruling one out: each such
/// pair is a finding that may hide an overlap (`Undecidable::MayOverlap`),
/// and nothing between the two blocks is recorded. Three poses: a corner
/// overlap (both caps in band), one nested inside the other's footprint
/// (both caps in band), and a corner overlap with only the tops in band.
/// Red if an aligned pair in band reads as one hiding no overlap, or a
/// touch between the blocks is recorded.
#[test]
fn an_overlap_whose_aligned_faces_are_in_band_is_never_cleared() {
    let band = geom_core::Band::linear(Tol::witness()).unwrap();
    let off = 0.5 * (band.zero() + band.escalate());
    let base = ((0.0, 2.0), (0.0, 2.0), (0.0, 1.0));
    for (name, top, unread) in [
        ("corner", ((1.0, 3.0), (1.0, 3.0), (off, 1.0 + off)), 2),
        ("nested", ((0.5, 1.5), (0.5, 1.5), (off, 1.0 + off)), 2),
        (
            "corner, tops only",
            ((1.0, 3.0), (1.0, 3.0), (0.5, 1.0 + off)),
            1,
        ),
    ] {
        let body = two_blocks(base, top);
        let (recorded, _) = at_rest(&body);
        let reasons: Vec<topo::Undecidable> = recorded
            .findings
            .iter()
            .filter_map(|e| match e {
                ValidationError::CensusUndecidable {
                    a: topo::EntityId::Face(_),
                    b: topo::EntityId::Face(_),
                    reason,
                    ..
                } => Some(*reason),
                _ => None,
            })
            .collect();
        assert_eq!(
            reasons,
            vec![topo::Undecidable::MayOverlap; unread],
            "{name}: {:?}",
            recorded.findings
        );
        assert!(
            recorded
                .findings
                .iter()
                .any(|e| topo::census_uncleared_solids(&body, e).is_some()),
            "{name}: the blocks are not cleared"
        );
        assert!(
            recorded.records.is_empty() && recorded.rows.is_empty(),
            "{name}: nothing between overlapping blocks is recorded: {:?}",
            recorded.records
        );
    }
}

/// A base `[0, 3] × [0, base_y1] × [−1, 0]` and a keel along y over it:
/// an inverted triangle prism whose bottom edge runs at z = off, in band
/// of the base's top, from y = off to y = keel_y1. Its end face at
/// y = off stands in band of the base's front, both facing −y.
fn keel_over_a_base(base_y1: f64, keel_y1: f64) -> Body<f64> {
    let band = geom_core::Band::linear(Tol::witness()).unwrap();
    let off = 0.5 * (band.zero() + band.escalate());
    let mut body = common::prism_z::<f64>(
        &[(0.0, 0.0), (3.0, 0.0), (3.0, base_y1), (0.0, base_y1)],
        -1.0,
        0.0,
        Tol::witness(),
    )
    .body;
    let mut keel = Body::<f64>::new();
    common::prism_ops(
        &mut keel,
        &[(1.4, off), (1.6, 0.5), (1.2, 0.5)],
        (-keel_y1, -off),
        |a, b, c| geom_core::Point3::new(a, -c, b),
        common::FaceGeometry::Certified,
        Tol::witness(),
    );
    common::describe_as_intersections(&mut keel, Tol::witness());
    topo::graft_disjoint_all_keyed(&mut body, &keel).unwrap();
    body
}

/// The census's escalations whose predicate is `predicate`.
fn escalated(findings: &[ValidationError], predicate: &str) -> usize {
    findings
        .iter()
        .filter(|e| {
            matches!(
                e,
                ValidationError::CensusEscalated { cause } if cause.predicate == Some(predicate)
            )
        })
        .count()
}

/// **An edge lying in band along a face is not a shadow of an in-band
/// pair's corner** (`drop_shadows`). A keel's bottom edge runs in band
/// over a base's top; its end faces stand in band of the base's front
/// and back, unread pairs whose corners touch the base's top edges.
/// The bottom edge read against the base's top holds those corners in
/// its closure, but lies along the top with no one meeting point, so
/// its escalation may lie anywhere along it: it stands beside the end
/// pairs' findings, and the blocks are not cleared. Three poses: the
/// keel a band short of both ends of the base, running past its back,
/// and ending over it. Red if the edge's escalation is dropped or a
/// touch is recorded.
#[test]
fn an_edge_in_band_along_a_face_stands_beside_the_unread_ends() {
    let band = geom_core::Band::linear(Tol::witness()).unwrap();
    let off = 0.5 * (band.zero() + band.escalate());
    for (name, base_y1, keel_y1) in [
        ("between two in-band ends", 1.0, 1.0 - off),
        ("past the base", 0.5, 1.0),
        ("over the base", 1.5, 1.0),
    ] {
        let body = keel_over_a_base(base_y1, keel_y1);
        let (recorded, _) = at_rest(&body);
        assert!(
            escalated(&recorded.findings, "pm_census_ef_residual") > 0,
            "{name}: the bottom edge's escalation stands: {:?}",
            recorded.findings
        );
        assert!(
            recorded
                .findings
                .iter()
                .any(|e| topo::census_uncleared_solids(&body, e).is_some()),
            "{name}: the blocks are not cleared"
        );
        assert!(
            recorded.records.is_empty(),
            "{name}: {:?}",
            recorded.records
        );
    }
}

/// **The control: the same keel with no unread pair.** The base runs
/// past both of the keel's ends, so no face of the keel stands in band
/// of one of the base's, and the bottom edge's escalations are what the
/// census reads with no unread pair at all; the poses above add the
/// end pairs' findings and drop none of them. Red if the control has an
/// unread pair, or its edge escalates under another predicate.
#[test]
fn the_keel_with_no_unread_pair_escalates_on_its_edge_alone() {
    let mut body = common::prism_z::<f64>(
        &[(0.0, -0.5), (3.0, -0.5), (3.0, 1.5), (0.0, 1.5)],
        -1.0,
        0.0,
        Tol::witness(),
    )
    .body;
    let band = geom_core::Band::linear(Tol::witness()).unwrap();
    let off = 0.5 * (band.zero() + band.escalate());
    let mut kb = Body::<f64>::new();
    common::prism_ops(
        &mut kb,
        &[(1.4, off), (1.6, 0.5), (1.2, 0.5)],
        (-1.0, -off),
        |a, b, c| geom_core::Point3::new(a, -c, b),
        common::FaceGeometry::Certified,
        Tol::witness(),
    );
    common::describe_as_intersections(&mut kb, Tol::witness());
    topo::graft_disjoint_all_keyed(&mut body, &kb).unwrap();
    let (recorded, _) = at_rest(&body);
    assert!(
        !recorded.findings.iter().any(|e| matches!(
            e,
            ValidationError::CensusUndecidable {
                a: topo::EntityId::Face(_),
                ..
            }
        )),
        "no unread pair: {:?}",
        recorded.findings
    );
    assert!(
        escalated(&recorded.findings, "pm_census_ef_residual") > 0,
        "the bottom edge escalates along the top: {:?}",
        recorded.findings
    );
}
