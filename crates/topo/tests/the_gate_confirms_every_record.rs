//! **A finished body holds no record its gate did not confirm**
//! (`[topology.tiers]`). A deciding door's gate reads each record it is
//! handed where it lies, and a reading that is not the record's
//! confirmation refuses, whatever it is: in the band, on a boundary the
//! region door declines, contradicted. A `⊔`'s gate takes arrivals and
//! reads their records off them; one that does not confirm on the graft
//! refuses there. A move carries the records it holds.
#![allow(clippy::panic, clippy::expect_used, clippy::unwrap_used)]

use crate::common;
use geom::SurfaceKind;
use geom_core::{Point3, RigidMotion, Tol, Vec3};
use topo::{
    Assembled, AtRestBody, Body, BooleanResult, Built, Cell, Cited, Cites, Contact, ContactRecords,
    FaceKey, FinishedBody, Isometry, LeafBody, ValidationError, finish, finish_assembled, union,
};

fn tol() -> Tol {
    Tol::witness()
}

fn cube(x: f64, y: f64, z: f64) -> AtRestBody<f64> {
    common::finished(
        "cube",
        common::brick::<f64>((x, x + 1.0), (y, y + 1.0), (z, z + 1.0), tol()),
        tol(),
    )
}

/// The planar face of `body` whose every vertex has `x` at `at` and `y`
/// in the unit span from `corner`.
fn face_at_x(body: &Body<f64>, at: f64, corner: f64) -> FaceKey {
    let points: std::collections::BTreeMap<_, _> = body.vertex_points().collect();
    body.faces()
        .map(|(f, _)| f)
        .find(|&f| {
            let mut starts = body
                .half_edges()
                .filter(|(he, _)| body.face_of_half_edge(*he) == Some(f))
                .map(|(_, h)| points[&h.start])
                .peekable();
            body.face_surface(f).map(geom::Surface::kind) == Some(SurfaceKind::Plane)
                && starts.peek().is_some()
                && starts.all(|p| p.x == at && p.y >= corner - 1e-6 && p.y <= corner + 1.0 + 1e-6)
        })
        .unwrap_or_else(|| panic!("a face at x = {at}"))
}

fn face_face(body: &Body<f64>, a: FaceKey, b: FaceKey) -> ContactRecords {
    [Cited::new(
        Contact::new(body, Cell::Face(a), Cell::Face(b)).unwrap(),
        Cites::decided(0),
    )]
    .into_iter()
    .collect()
}

/// Two cubes as one body of two solids, `gap` apart along x, the second
/// raised `lift` along y.
fn two_cubes(gap: f64, lift: f64) -> AtRestBody<f64> {
    let mut joined = Assembled::<f64>::new();
    joined
        .graft_finished(&cube(0.0, 0.0, 0.0), None, tol())
        .unwrap();
    joined
        .graft_finished(&cube(1.0 + gap, lift, 0.0), None, tol())
        .unwrap();
    finish_assembled(joined, tol()).unwrap().body
}

/// Two faces the band's midpoint apart, recorded as touching: the
/// census reads their carriers' offset in the band, at whichever ε and
/// ambiguity factor the suite runs, and escalates, which is not a
/// confirmation, and the gate refuses. Red on a gate that refuses only
/// the stale and the contradicted: it held the record.
#[test]
fn a_record_the_census_reads_in_the_band_refuses() {
    let band = geom_core::Band::linear(tol()).unwrap();
    let gap = 0.5 * (band.zero() + band.escalate());
    let two = two_cubes(gap, 0.0);
    let record = face_face(
        &two,
        face_at_x(&two, 1.0, 0.0),
        face_at_x(&two, 1.0 + gap, 0.0),
    );
    let got = finish(Built::of(Body::clone(&two)).recording(record), tol());
    let findings = got.expect_err("a record read in the band is not confirmed");
    assert!(
        matches!(
            findings.as_slice(),
            [ValidationError::CensusEscalated { .. }]
        ),
        "{findings:?}"
    );
}

/// Two faces on one plane, facing each other, that meet at an edge
/// alone, recorded as touching: the region door declines the pair
/// (their boundaries touch), which is not a confirmation, and the gate
/// refuses. Red on a gate that refuses only the stale and the
/// contradicted.
#[test]
fn a_record_the_region_door_declines_refuses() {
    let two = two_cubes(0.0, 1.0);
    let record = face_face(&two, face_at_x(&two, 1.0, 0.0), face_at_x(&two, 1.0, 1.0));
    let got = finish(Built::of(Body::clone(&two)).recording(record), tol());
    let findings = got.expect_err("a pair the region door declines is not confirmed");
    assert!(
        matches!(
            findings.as_slice(),
            [ValidationError::CensusUnsupported { .. }]
        ),
        "{findings:?}"
    );
}

/// An arrival holding a record of two faces that do not meet: the
/// `⊔`'s gate reads what its arrivals carried in, and refuses it. Red on
/// a gate that holds every carried record and reports the contradiction
/// beside the finished body.
#[test]
fn a_joins_gate_refuses_a_record_an_arrival_carried_that_does_not_confirm() {
    let kiss = common::unconfirmed::kiss_holding_a_false_record(tol());
    let mut joined = Assembled::<f64>::new();
    joined.graft_finished(&kiss, None, tol()).unwrap();
    joined
        .graft_finished(&cube(10.0, 10.0, 10.0), None, tol())
        .unwrap();
    let findings = finish_assembled(joined, tol())
        .map(|censused| censused.body.records().len())
        .expect_err("the false record refuses at the join's gate");
    assert!(
        common::unconfirmed::the_false_record_alone(&findings),
        "{findings:?}"
    );
}

/// A corner kiss and its record.
fn kiss() -> AtRestBody<f64> {
    match union(&cube(0.0, 0.0, 0.0), &cube(1.0, 1.0, 1.0), tol()).unwrap() {
        BooleanResult::Body(kiss) => kiss.body,
        BooleanResult::Empty => panic!("the kiss is empty"),
    }
}

/// A finished body moved, or mirrored, holds the records it held, on
/// the cells it held them on: an isometry re-mints no key. As a leaf at
/// `f64` the image is finished again, and its gate confirms them. Red
/// on a move that drops the records.
#[test]
fn a_move_and_a_mirror_keep_the_records() {
    let kiss = kiss();
    let held: Vec<_> = kiss.records().kinds().collect();
    assert_eq!(held.len(), 1, "the kiss holds its one record: {held:?}");
    let slide = Isometry::Proper(RigidMotion::axes_xy(Point3::new(3.0, -2.0, 0.5)));
    let flip = Isometry::Improper(
        topo::Reflection::across(Point3::new(0.25, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0), tol())
            .unwrap(),
    );
    for (what, iso) in [("moved", &slide), ("mirrored", &flip)] {
        let image = FinishedBody::moved_by(&kiss, iso, tol()).unwrap();
        assert_eq!(
            image.records().kinds().collect::<Vec<_>>(),
            held,
            "{what}: a finished body"
        );
        let image = LeafBody::moved_by(&kiss, iso, tol()).unwrap();
        assert_eq!(
            LeafBody::records(&image).kinds().collect::<Vec<_>>(),
            held,
            "{what}: a leaf"
        );
    }
}

/// **A box scalar's taker finishes its leaf with the leaf's records.**
/// At `Interval` a leaf is tier 2 alone and holds its records unread;
/// the taker's gate is the one that reads them. A cube's leaf handed a
/// record of two opposite faces, which do not meet, is taken: the gate
/// refuses the record. Red on a taker that finishes the leaf's body
/// without its records: the take succeeds.
#[test]
fn a_box_scalars_taker_reads_the_records_its_leaf_holds() {
    use geom_core::Interval;
    use topo::AtRestPolicy;
    let body = common::brick::<Interval>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol());
    let corners = |f: FaceKey| -> std::collections::BTreeSet<_> {
        body.half_edges()
            .filter(|(he, _)| body.face_of_half_edge(*he) == Some(f))
            .map(|(_, h)| h.start)
            .collect()
    };
    let faces: Vec<FaceKey> = body.faces().map(|(f, _)| f).collect();
    let a = faces[0];
    let b = *faces
        .iter()
        .find(|&&f| corners(f).is_disjoint(&corners(a)))
        .expect("a cube's face has an opposite");
    let record: ContactRecords = [Cited::new(
        Contact::new(&body, Cell::Face(a), Cell::Face(b)).unwrap(),
        Cites::decided(0),
    )]
    .into_iter()
    .collect();
    let leaf = <Interval as AtRestPolicy>::leaf(Built::of(body).recording(record), tol())
        .expect("a leaf at a box scalar is tier 2 alone");
    assert_eq!(
        LeafBody::records(&leaf).len(),
        1,
        "the leaf holds it unread"
    );
    let taken = topo::take::<Interval>(&std::sync::Arc::new(leaf), tol());
    let findings = taken
        .map(|finished| finished.records().len())
        .expect_err("the taker's gate reads the leaf's record");
    assert!(
        matches!(
            findings.as_slice(),
            [ValidationError::ContactContradicted { .. }]
        ),
        "{findings:?}"
    );
}

/// **A finished body names the bar its gate paid on its records**
/// (`[topology.tiers]`), at each door that makes one. A brick holds no
/// record: tier 3. A kiss holds its record, confirmed: tier 3′ over its
/// records, and so do its image under a move, as a finished body and as
/// a leaf, the side of a split that keeps the kiss, and its shell. A
/// `⊔` of two cubes apart, whose census decided between them and found
/// nothing, is tier 3′, and stays so moved as a leaf, though it holds no
/// record; a `⊔` of one brick, which had nothing to decide, is tier 3.
/// Red where a leaf's move reads the bar off its image's records alone,
/// or a `⊔` claims tier 3′ for a census that had one arrival.
#[test]
fn a_finished_body_names_the_bar_its_gate_paid() {
    use topo::Bar;
    assert_eq!(cube(0.0, 0.0, 0.0).bar(), Bar::Tier3, "a brick");
    let kiss = kiss();
    assert_eq!(kiss.bar(), Bar::Tier3Prime, "a kiss");
    let slide = Isometry::Proper(RigidMotion::axes_xy(Point3::new(3.0, -2.0, 0.5)));
    assert_eq!(
        FinishedBody::moved_by(&kiss, &slide, tol()).unwrap().bar(),
        Bar::Tier3Prime,
        "a kiss, moved"
    );
    assert_eq!(
        LeafBody::moved_by(&kiss, &slide, tol()).unwrap().bar(),
        Bar::Tier3Prime,
        "a kiss, moved as a leaf"
    );
    let plane = common::split_plane(Point3::new(0.0, 0.0, -5.0), Vec3::new(0.0, 0.0, 1.0), tol());
    let split = topo::split(&kiss, &plane, tol()).unwrap();
    let side = [&split.above, &split.below]
        .into_iter()
        .find_map(|part| part.body())
        .expect("the side that keeps the kiss");
    assert_eq!(side.bar(), Bar::Tier3Prime, "the split's side");
    assert_eq!(
        topo::shell(&kiss, 0.1, tol()).unwrap().body.bar(),
        Bar::Tier3Prime,
        "the kiss's shell"
    );
    let apart = two_cubes(5.0, 0.0);
    assert!(apart.records().is_empty(), "two cubes apart touch nowhere");
    assert_eq!(apart.bar(), Bar::Tier3Prime, "a join of two arrivals");
    assert_eq!(
        LeafBody::moved_by(&apart, &slide, tol()).unwrap().bar(),
        Bar::Tier3Prime,
        "a join of two arrivals, moved as a leaf"
    );
    let mut alone = Assembled::<f64>::new();
    alone
        .graft_finished(&cube(0.0, 0.0, 0.0), None, tol())
        .unwrap();
    assert_eq!(
        finish_assembled(alone, tol()).unwrap().body.bar(),
        Bar::Tier3,
        "a join of one arrival"
    );
}

/// No `pub fn` written in an inherent `impl … Assembled<T>` block of
/// `crates/topo/src/finish.rs` names a record type in its signature
/// (`ContactRecords`, `OperandRecords`, `Cited`, `Contact`, `Backing`,
/// `Cites`). That is all this reads: a line scan of that one file. It
/// does not see an `impl` in another file, a trait impl, a `pub` field
/// (the type has none: its fields are `pub(crate)`), or a free function
/// that takes an `Assembled`; the type's own doc pins `new`'s arity by
/// `compile_fail`. It goes red on the likeliest regression, a
/// constructor or method added beside the others with a list in hand.
#[test]
fn no_pub_fn_of_assembled_in_finish_rs_names_a_record_type() {
    let source = include_str!("../src/finish.rs");
    let mut inside = false;
    let mut declared = 0;
    let mut signatures: Vec<String> = Vec::new();
    let mut open: Option<String> = None;
    for line in source.lines() {
        if line.starts_with("impl") && line.contains("Assembled<T>") && !line.contains(" for ") {
            inside = true;
        } else if line == "}" {
            inside = false;
        }
        if !inside {
            continue;
        }
        if line.trim_start().starts_with("pub fn ") {
            declared += 1;
            open = Some(String::new());
        }
        if let Some(signature) = open.as_mut() {
            signature.push_str(line.trim());
            signature.push(' ');
            if line.trim_end().ends_with('{') {
                signatures.extend(open.take());
            }
        }
    }
    assert!(declared > 0, "the scan found the type's impl blocks");
    assert_eq!(
        signatures.len(),
        declared,
        "every `pub fn` the scan met was read to its body: {signatures:#?}"
    );
    for signature in &signatures {
        assert!(
            ![
                "ContactRecords",
                "OperandRecords",
                "Cited",
                "Contact",
                "Backing",
                "Cites"
            ]
            .iter()
            .any(|record| signature.contains(record)),
            "a `pub fn` of a join names a record type: {signature}"
        );
    }
}

/// A join names tier 3′ only where its census decided every meeting
/// between two arrivals (`Bar::Tier3Prime`'s own sentence). RED on
/// `bbb6955172`: two cubes whose facing faces stand in the band apart
/// (its midpoint, at whichever ε the suite runs), each an arrival; the
/// census hands back `CensusUndecidable` on that pair and the body was
/// named `Tier3Prime`.
#[test]
fn a_joins_bar_is_not_tier_3_prime_where_its_census_left_a_meeting_undecided() {
    let band = geom_core::Band::linear(tol()).unwrap();
    let gap = 0.5 * (band.zero() + band.escalate());
    let mut join = Assembled::<f64>::new();
    join.graft_finished(&cube(0.0, 0.0, 0.0), None, tol())
        .unwrap();
    join.graft_finished(&cube(1.0 + gap, 0.0, 0.0), None, tol())
        .unwrap();
    let done = finish_assembled(join, tol()).unwrap();
    assert!(
        done.findings
            .iter()
            .any(|f| matches!(f, ValidationError::CensusUndecidable { .. })),
        "the census leaves the pair undecided: {:?}",
        done.findings
    );
    assert_eq!(
        done.body.bar(),
        topo::Bar::Tier3,
        "named tier 3' beside {:?}",
        done.findings
    );
}
