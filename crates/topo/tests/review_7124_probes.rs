//! Review probes for PR 7124 (the finished body holds its records).
#![allow(clippy::panic, clippy::expect_used, clippy::unwrap_used)]

use crate::common;
use geom_core::Tol;
use std::sync::Arc;
use topo::{
    Assembled, Body, BooleanResult, Built, Cell, Cited, Cites, Contact, ContactRecords, FaceKey,
    FinishedBody, Surface, finish, finish_assembled, union, validate_pseudomanifold,
};

fn tol() -> Tol {
    Tol::witness()
}

/// A unit cube at the origin and one moved by `d`, in one body.
fn two_cubes(d: (f64, f64, f64)) -> Body<f64> {
    let mut body = common::mapped_cube(common::identity_map::<f64>, tol());
    common::cube_into(
        &mut body,
        |x, y, z| common::identity_map::<f64>(x + d.0, y + d.1, z + d.2),
        tol(),
    );
    body
}

/// The face of solid number `solid` (in `solids()` order) on the plane x = `x`.
fn face_at_x(body: &Body<f64>, solid: usize, x: f64) -> FaceKey {
    let (key, _) = body.solids().nth(solid).unwrap();
    let found: Vec<FaceKey> = body
        .faces_of_solid(key)
        .unwrap()
        .into_iter()
        .filter(|f| {
            matches!(
                body.face_surface(*f),
                Some(Surface::Plane { frame })
                    if frame.axis().x.abs() == 1.0 && frame.origin().x == x
            )
        })
        .collect();
    assert_eq!(found.len(), 1, "one face at x = {x} on solid {solid}");
    found[0]
}

fn one(body: &Body<f64>, x: Cell, y: Cell) -> ContactRecords {
    let mut records = ContactRecords::default();
    records.extend([Cited::new(Contact::new(body, x, y).unwrap(), Cites::decided(0))]);
    records
}

fn names(findings: &[topo::ValidationError]) -> Vec<String> {
    findings
        .iter()
        .map(|f| {
            let s = format!("{f:?}");
            s.split(|c: char| c == ' ' || c == '{' || c == '(')
                .next()
                .unwrap()
                .to_owned()
        })
        .collect()
}

/// P-A: a `⊔`'s gate handed a record with no touch behind it.
#[test]
fn probe_a_assembled_holds_a_false_carried_record() {
    let body = two_cubes((3.0, 0.0, 0.0));
    let records = one(
        &body,
        Cell::Face(face_at_x(&body, 0, 0.0)),
        Cell::Face(face_at_x(&body, 1, 4.0)),
    );
    let plain = finish(Built::of(body.clone()).recording(records.clone()), tol());
    eprintln!("P-A plain finish: {:?}", plain.as_ref().map(|_| "Ok").map_err(|e| names(e)));
    for (what, arrivals) in [
        ("one arrival", Arc::new(|_| Some(0u32)) as topo::Arrivals),
        ("no arrival", Arc::new(|_| None) as topo::Arrivals),
        ("two arrivals", {
            let first = body.solids().next().unwrap().0;
            Arc::new(move |s| Some(u32::from(s != first))) as topo::Arrivals
        }),
    ] {
        match finish_assembled(Assembled::new(body.clone(), records.clone(), arrivals), tol()) {
            Ok(c) => eprintln!(
                "P-A {what}: Ok, holds {} record(s), bar {:?}, findings {:?}, rows {}",
                c.body.records().rows().len(),
                c.body.bar(),
                names(&c.findings),
                c.rows.len()
            ),
            Err(e) => eprintln!("P-A {what}: refused {:?}", names(&e)),
        }
    }
}

/// P-B: records whose confirmation the census neither confirms nor
/// calls stale or contradicted.
#[test]
fn probe_b_plain_gate_and_unconfirmable_records() {
    for gap in [0.0, 1e-14, 1e-12, 1e-11, 1e-10, 1e-9, 1e-8, 1e-7, 1e-6, 1e-3] {
        let body = two_cubes((1.0 + gap, 0.0, 0.0));
        let fa = face_at_x(&body, 0, 1.0);
        let fb = face_at_x(&body, 1, 1.0 + gap);
        let records = one(&body, Cell::Face(fa), Cell::Face(fb));
        let census = validate_pseudomanifold(&body, &records, tol());
        let gate = finish(Built::of(body.clone()).recording(records), tol());
        eprintln!(
            "P-B gap {gap:e}: census {:?}; gate {:?}",
            census.as_ref().map_err(|e| names(e)),
            gate.as_ref()
                .map(|b| (b.records().rows().len(), b.bar()))
                .map_err(|e| names(e))
        );
    }
    // Overlapping cubes, a record of two faces that are not the overlap.
    let body = two_cubes((0.5, 0.5, 0.5));
    let records = one(
        &body,
        Cell::Face(face_at_x(&body, 0, 0.0)),
        Cell::Face(face_at_x(&body, 1, 1.5)),
    );
    let census = validate_pseudomanifold(&body, &records, tol());
    let gate = finish(Built::of(body.clone()).recording(records), tol());
    eprintln!(
        "P-B overlap: census {:?}; gate {:?}",
        census.as_ref().map_err(|e| names(e)),
        gate.as_ref().map(|b| (b.records().rows().len(), b.bar())).map_err(|e| names(e))
    );
}

/// P-C: a finished body's records through the public trait's
/// `try_recite`.
#[test]
fn probe_c_try_recite_from_outside() {
    let at_rest = |b| finish::<f64>(b, tol()).unwrap();
    let a = at_rest(common::brick::<f64>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol()));
    let b = at_rest(common::brick::<f64>((1.0, 2.0), (1.0, 2.0), (1.0, 2.0), tol()));
    let BooleanResult::Body(kiss) = union(&a, &b, tol()).unwrap() else {
        panic!("empty");
    };
    let mut body = kiss.body.clone();
    eprintln!("P-C before: {} record(s), bar {:?}", body.records().rows().len(), body.bar());
    let refused: Result<(), ()> = FinishedBody::try_recite(&mut body, |_| Err(()), |_| ());
    eprintln!(
        "P-C after a refused recite ({refused:?}): {} record(s), bar {:?}",
        body.records().rows().len(),
        body.bar()
    );
    // And re-citing to a row that does not exist.
    let mut body = kiss.body.clone();
    let ok: Result<(), ()> = FinishedBody::try_recite(
        &mut body,
        |_| Ok(vec![topo::Backing::Decided(4_000_000)]),
        |_| (),
    );
    eprintln!("P-C recite to a made-up row ({ok:?}): {:?}", body.records());
    // The stripped body, reused as an operand.
    let mut stripped = kiss.body.clone();
    let _: Result<(), ()> = FinishedBody::try_recite(&mut stripped, |_| Err(()), |_| ());
    let c = at_rest(common::brick::<f64>((5.0, 6.0), (0.0, 1.0), (0.0, 1.0), tol()));
    match union(&stripped, &c, tol()) {
        Ok(BooleanResult::Body(r)) => eprintln!(
            "P-C stripped kiss ∪ far cube: Ok, {} record(s); tier 3' over it: {:?}",
            r.contacts().rows().len(),
            validate_pseudomanifold(&r.body, r.contacts(), tol()).map_err(|e| names(&e))
        ),
        other => eprintln!("P-C stripped kiss ∪ far cube: {other:?}"),
    }
}

/// P-D: a `⊔`'s leaf holding a carried record with no touch behind it,
/// then moved as a leaf.
#[test]
fn probe_d_assembled_leaf_then_moved() {
    use topo::{AtRestPolicy, LeafBody};
    let body = two_cubes((3.0, 0.0, 0.0));
    let records = one(
        &body,
        Cell::Face(face_at_x(&body, 0, 0.0)),
        Cell::Face(face_at_x(&body, 1, 4.0)),
    );
    let first = body.solids().next().unwrap().0;
    let leaf = <f64 as AtRestPolicy>::leaf_assembled(
        Assembled::new(
            body,
            records,
            Arc::new(move |s| Some(u32::from(s != first))),
        ),
        tol(),
    )
    .map(|c| c.leaf);
    match leaf {
        Ok(leaf) => {
            eprintln!("P-D leaf: Ok, {} record(s), bar {:?}", leaf.records().rows().len(), leaf.bar());
            let motion =
                geom_core::RigidMotion::translation(geom_core::Vec3::new(10.0, 0.0, 0.0)).unwrap();
            let as_leaf = LeafBody::moved(&leaf, &motion, tol());
            eprintln!(
                "P-D moved as a leaf: {:?}",
                as_leaf.as_ref().map(|b| (b.records().rows().len(), b.bar())).map_err(|e| names(e))
            );
            let as_finished = FinishedBody::moved(&leaf, &motion, tol());
            eprintln!(
                "P-D moved as a finished body: {:?}",
                as_finished.as_ref().map(|b| (b.records().rows().len(), b.bar())).map_err(|e| names(e))
            );
        }
        Err(e) => eprintln!("P-D leaf refused: {:?}", names(&e)),
    }
    // A true ⊔: two cubes kissing at a corner, each its own arrival, moved as a leaf.
    let body = two_cubes((1.0, 1.0, 1.0));
    let first = body.solids().next().unwrap().0;
    let c = <f64 as AtRestPolicy>::leaf_assembled(
        Assembled::new(
            body,
            ContactRecords::default(),
            Arc::new(move |s| Some(u32::from(s != first))),
        ),
        tol(),
    )
    .unwrap();
    eprintln!(
        "P-D kiss leaf: {} record(s), bar {:?}, findings {:?}",
        c.leaf.records().rows().len(),
        c.leaf.bar(),
        names(&c.findings)
    );
    let motion = geom_core::RigidMotion::translation(geom_core::Vec3::new(10.0, 0.0, 0.0)).unwrap();
    let moved = LeafBody::moved(&c.leaf, &motion, tol());
    eprintln!(
        "P-D kiss leaf moved as a leaf: {:?}",
        moved.as_ref().map(|b| (b.records().rows().len(), b.bar())).map_err(|e| names(e))
    );
}

/// P-E: the plain gate over a record it cannot decide, a duplicate, a
/// record on a key of another body, and what a kiss reads with its
/// record given up.
#[test]
fn probe_e_more_records_at_the_plain_gate() {
    for gap in [1e-9, 1e-8] {
        let body = two_cubes((1.0 + gap, 0.0, 0.0));
        let records = one(
            &body,
            Cell::Face(face_at_x(&body, 0, 1.0)),
            Cell::Face(face_at_x(&body, 1, 1.0 + gap)),
        );
        let census = validate_pseudomanifold(&body, &records, tol());
        let mut kinds = census.as_ref().map_err(|e| names(e)).err().unwrap_or_default();
        kinds.sort();
        kinds.dedup();
        let gate = finish(Built::of(body.clone()).recording(records), tol());
        eprintln!(
            "P-E gap {gap:e}: census kinds {kinds:?}; gate {:?}",
            gate.as_ref().map(|b| (b.records().rows().len(), b.bar())).map_err(|e| names(e))
        );
        if let Err(findings) = &census {
            eprintln!("P-E gap {gap:e}: first finding: {:?}", findings[0]);
        }
    }
    // A duplicate of a true record.
    let body = two_cubes((1.0, 0.0, 0.0));
    let mut records = one(
        &body,
        Cell::Face(face_at_x(&body, 0, 1.0)),
        Cell::Face(face_at_x(&body, 1, 1.0)),
    );
    let again = records.rows()[0].clone();
    records.extend([again]);
    let gate = finish(Built::of(body.clone()).recording(records), tol());
    eprintln!(
        "P-E duplicate: gate {:?}",
        gate.as_ref().map(|b| (b.records().rows().len(), b.bar())).map_err(|e| names(e))
    );
    // A record keyed in another body's keys (a fresh-keyed brick's faces).
    let other = common::brick_with_fresh_keys::<f64>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol());
    let foreign: Vec<FaceKey> = other.faces().map(|(f, _)| f).take(2).collect();
    let mut records = ContactRecords::default();
    match Contact::new(&body, Cell::Face(foreign[0]), Cell::Face(foreign[1])) {
        Ok(c) => {
            records.extend([Cited::new(c, Cites::decided(0))]);
            let gate = finish(Built::of(body.clone()).recording(records), tol());
            eprintln!(
                "P-E foreign keys: gate {:?}",
                gate.as_ref().map(|b| b.records().rows().len()).map_err(|e| names(e))
            );
        }
        Err(e) => eprintln!("P-E foreign keys: Contact::new refused {e:?}"),
    }
    // The kiss, its record given up, read at tier 3'.
    let at_rest = |b| finish::<f64>(b, tol()).unwrap();
    let a = at_rest(common::brick::<f64>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol()));
    let b = at_rest(common::brick::<f64>((1.0, 2.0), (1.0, 2.0), (1.0, 2.0), tol()));
    let BooleanResult::Body(kiss) = union(&a, &b, tol()).unwrap() else {
        panic!("empty");
    };
    eprintln!(
        "P-E kiss: {} solid(s), {} vertices, records {:?}; tier 3' with none: {:?}",
        kiss.body.solids().count(),
        kiss.body.vertices().count(),
        kiss.contacts(),
        validate_pseudomanifold(&kiss.body, &ContactRecords::default(), tol()).map_err(|e| names(&e))
    );
    // `boolean_op_with` handed nothing where the operand holds a record.
    let c = at_rest(common::brick::<f64>((5.0, 6.0), (0.0, 1.0), (0.0, 1.0), tol()));
    let with = topo::boolean_op_with(
        topo::BooleanOp::Union,
        &kiss.body,
        &c,
        &topo::BooleanInputs::default(),
        topo::SweepStrategy::Realized,
        tol(),
    );
    match with {
        Ok(BooleanResult::Body(r)) => eprintln!(
            "P-E boolean_op_with(default) over a kiss holding 1 record: Ok, result holds {}, bar {:?}; \
             tier 3' with what it holds: {:?}",
            r.contacts().rows().len(),
            r.body.bar(),
            validate_pseudomanifold(&r.body, r.contacts(), tol()).map_err(|e| names(&e))
        ),
        other => eprintln!("P-E boolean_op_with(default): {:?}", other.map(|_| ())),
    }
    let plain = union(&kiss.body, &c, tol());
    if let Ok(BooleanResult::Body(r)) = plain {
        eprintln!("P-E plain union over the same: result holds {}", r.contacts().rows().len());
    }
}

/// P-F: end to end, a corner kiss whose touch a later cut removes.
#[test]
fn probe_f_a_cut_that_removes_a_recorded_touch() {
    let at_rest = |b| finish::<f64>(b, tol()).unwrap();
    let a = at_rest(common::brick::<f64>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol()));
    let b = at_rest(common::brick::<f64>((1.0, 2.0), (1.0, 2.0), (1.0, 2.0), tol()));
    let BooleanResult::Body(kiss) = union(&a, &b, tol()).unwrap() else {
        panic!("empty");
    };
    for (what, tool) in [
        ("the corner of b at the kiss", ((1.0, 1.5), (1.0, 1.5), (1.0, 1.5))),
        ("a block across b's corner and beyond", ((0.5, 1.5), (0.5, 1.5), (1.25, 1.75))),
        ("a block clear of the kiss", ((1.5, 2.5), (1.5, 2.5), (1.5, 2.5))),
        ("the corner of a at the kiss", ((0.5, 1.0), (0.5, 1.0), (0.5, 1.0))),
        ("both corners", ((0.5, 1.5), (0.5, 1.5), (0.5, 1.5))),
    ] {
        let t = at_rest(common::brick::<f64>(tool.0, tool.1, tool.2, tol()));
        match topo::subtract(&kiss.body, &t, tol()) {
            Ok(BooleanResult::Body(r)) => eprintln!(
                "P-F kiss ∖ {what}: Ok, {} record(s), tier 3' {:?}",
                r.contacts().rows().len(),
                validate_pseudomanifold(&r.body, r.contacts(), tol()).map_err(|e| names(&e))
            ),
            Ok(_) => eprintln!("P-F kiss ∖ {what}: empty"),
            Err(e) => eprintln!("P-F kiss ∖ {what}: refused: {e}"),
        }
    }
    // The gate's text for the two record findings, as a user would read it.
    let body = two_cubes((3.0, 0.0, 0.0));
    let records = one(
        &body,
        Cell::Face(face_at_x(&body, 0, 0.0)),
        Cell::Face(face_at_x(&body, 1, 4.0)),
    );
    if let Err(e) = finish(Built::of(body).recording(records), tol()) {
        for f in e.as_slice() {
            eprintln!("P-F text: {f}");
        }
    }
    let body = two_cubes((3.0, 0.0, 0.0));
    let v: Vec<_> = body.vertices().map(|(k, _)| k).collect();
    let records = one(&body, Cell::Vertex(v[0]), Cell::Vertex(v[v.len() - 1]));
    if let Err(e) = finish(Built::of(body).recording(records), tol()) {
        for f in e.as_slice() {
            eprintln!("P-F text: {f}");
        }
    }
}

// ---------------------------------------------------------------------
// The four demonstrations as rows: each states what the PR claims, and
// is RED on `18c956f37a`.
// ---------------------------------------------------------------------

/// A `⊔`'s gate handed a record its own census contradicts does not
/// hand back a finished body holding it. RED: `finish_assembled`
/// returns the body with the record, `Bar::ArrivalsDecided`, the
/// contradiction only in the findings beside it.
#[test]
fn red_a_joins_gate_holds_no_record_its_census_contradicts() {
    let body = two_cubes((3.0, 0.0, 0.0));
    let records = one(
        &body,
        Cell::Face(face_at_x(&body, 0, 0.0)),
        Cell::Face(face_at_x(&body, 1, 4.0)),
    );
    assert!(
        finish(Built::of(body.clone()).recording(records.clone()), tol()).is_err(),
        "the plain gate refuses the record"
    );
    let first = body.solids().next().unwrap().0;
    let joined = finish_assembled(
        Assembled::new(body, records, Arc::new(move |s| Some(u32::from(s != first)))),
        tol(),
    );
    assert!(
        joined.as_ref().map_or(true, |c| c.body.records().rows().is_empty()),
        "a finished body holds a record its gate's census contradicted: {:?}",
        joined.map(|c| (c.body.records().clone(), names(&c.findings)))
    );
}

/// The gate holds no record whose reading its census left undecided.
/// RED: two faces 1e-9 apart, recorded as touching; the census reads
/// the pair `CensusUndecidable` (in the band), and `finish` returns the
/// body holding the record under `Bar::RecordsConfirmed`.
#[test]
fn red_the_gate_holds_no_record_its_census_cannot_decide() {
    let gap = 1e-9;
    let body = two_cubes((1.0 + gap, 0.0, 0.0));
    let (fa, fb) = (face_at_x(&body, 0, 1.0), face_at_x(&body, 1, 1.0 + gap));
    let records = one(&body, Cell::Face(fa), Cell::Face(fb));
    let census = validate_pseudomanifold(&body, &records, tol()).expect_err("in the band");
    assert!(
        census.iter().any(|f| matches!(
            f,
            topo::ValidationError::CensusUndecidable { a, b, .. }
                if format!("{a:?}{b:?}") == format!("Face({fa:?})Face({fb:?})")
        )),
        "the census cannot decide the recorded pair: {:?}",
        names(&census)
    );
    let gate = finish(Built::of(body).recording(records), tol());
    assert!(
        gate.is_err(),
        "the gate confirmed a record its census could not decide: {:?}",
        gate.map(|b| (b.records().clone(), b.bar()))
    );
}

/// A finished body's records are not removed through the public
/// trait. RED: a recite that refuses leaves the body with none, its
/// bar unchanged, from any crate.
#[test]
fn red_a_refused_recite_leaves_a_finished_body_its_records() {
    let at_rest = |b| finish::<f64>(b, tol()).unwrap();
    let a = at_rest(common::brick::<f64>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol()));
    let b = at_rest(common::brick::<f64>((1.0, 2.0), (1.0, 2.0), (1.0, 2.0), tol()));
    let BooleanResult::Body(kiss) = union(&a, &b, tol()).unwrap() else {
        panic!("empty");
    };
    let mut body = kiss.body.clone();
    let before = body.records().rows().len();
    let _: Result<(), ()> = FinishedBody::try_recite(&mut body, |_| Err(()), |_| ());
    assert_eq!(
        body.records().rows().len(),
        before,
        "the kiss's record is gone from a body still marked {:?}",
        body.bar()
    );
}

/// A `⊔`'s leaf moved as a leaf keeps the bar its gate paid
/// (`AtRestBody::bar`: "an isometry keeps it"). RED: it comes back
/// `RecordsConfirmed`.
#[test]
fn red_a_joins_leaf_keeps_its_bar_under_a_motion() {
    use topo::{AtRestPolicy, LeafBody};
    let body = two_cubes((1.0, 1.0, 1.0));
    let first = body.solids().next().unwrap().0;
    let joined = <f64 as AtRestPolicy>::leaf_assembled(
        Assembled::new(
            body,
            ContactRecords::default(),
            Arc::new(move |s| Some(u32::from(s != first))),
        ),
        tol(),
    )
    .unwrap();
    let motion = geom_core::RigidMotion::translation(geom_core::Vec3::new(10.0, 0.0, 0.0)).unwrap();
    let moved = LeafBody::moved(&joined.leaf, &motion, tol()).unwrap();
    assert_eq!(moved.bar(), joined.leaf.bar());
}
