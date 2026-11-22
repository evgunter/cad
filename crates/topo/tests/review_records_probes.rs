//! Review probes on PR 7124's finished type: which records a gate holds
//! without confirming them.
#![allow(clippy::panic, clippy::expect_used, clippy::unwrap_used)]

use crate::common;
use geom_core::{Point3, Tol};
use std::sync::Arc;
use topo::{
    AtRestBody, Assembled, BooleanResult, Built, Cell, Cited, Cites, Contact, ContactRecords,
    FinishedBody as _, union,
};

fn tol() -> Tol {
    Tol::witness()
}

fn brick(x: (f64, f64), y: (f64, f64), z: (f64, f64)) -> AtRestBody<f64> {
    common::finished("brick", common::brick::<f64>(x, y, z, tol()), tol())
}

fn faces_on(body: &topo::Body<f64>, axis: usize, at: f64) -> Vec<topo::entity::FaceKey> {
    body.faces()
        .filter(|(f, _)| {
            matches!(
                body.face_surface(*f),
                Some(topo::Surface::Plane { frame })
                    if [frame.axis().x, frame.axis().y, frame.axis().z][axis].abs() == 1.0
                        && [frame.origin().x, frame.origin().y, frame.origin().z][axis] == at
            )
        })
        .map(|(f, _)| f)
        .collect()
}

/// The face on the plane `axis = at` of the solid that holds `of`.
fn face_of(body: &topo::Body<f64>, axis: usize, at: f64, of: topo::SolidKey) -> topo::entity::FaceKey {
    faces_on(body, axis, at)
        .into_iter()
        .find(|f| body.solid_of_face(*f) == Some(of))
        .unwrap_or_else(|| panic!("a face on axis {axis} at {at} of {of:?}"))
}

/// Compat: the one face on `axis = at` (first solid's for 1.0 with n>0).
fn plane_face(body: &topo::Body<f64>, axis: usize, at: f64, _n: f64) -> topo::entity::FaceKey {
    faces_on(body, axis, at)[0]
}

fn vertex_at(body: &topo::Body<f64>, p: Point3<f64>) -> Vec<topo::entity::VertexKey> {
    body.vertices()
        .filter(|(v, _)| {
            topo::readback::vertex_point(body, *v).is_ok_and(|q| (q - p).norm() < 1e-12)
        })
        .map(|(v, _)| v)
        .collect()
}

fn with(records: &ContactRecords, x: Cell, y: Cell, body: &topo::Body<f64>) -> ContactRecords {
    let mut r = records.clone();
    match Contact::new(body, x, y) {
        Ok(c) => r.extend([Cited::new(c, Cites::decided(7))]),
        Err(e) => eprintln!("PROBE   (pair refused at Contact::new: {e:?})"),
    }
    r
}

/// Two cubes kissing at (1, 1, 1).
fn kiss() -> AtRestBody<f64> {
    let a = brick((0.0, 1.0), (0.0, 1.0), (0.0, 1.0));
    let b = brick((1.0, 2.0), (1.0, 2.0), (1.0, 2.0));
    let BooleanResult::Body(k) = union(&a, &b, tol()).unwrap() else {
        panic!("empty");
    };
    k.body
}

fn report(what: &str, body: &topo::Body<f64>, records: ContactRecords) -> bool {
    let raw = topo::validate_pseudomanifold(body, &records, tol());
    eprintln!("PROBE {what}: raw tier 3' {:?}", raw.as_ref().map_err(|e| format!("{e:?}").chars().take(600).collect::<String>()));
    let r = topo::finish(Built::of(body.clone()).recording(records), tol());
    match &r {
        Ok(f) => eprintln!(
            "PROBE {what}: FINISHED holding {} records, bar {:?}",
            f.records().rows().len(),
            f.bar()
        ),
        Err(e) => eprintln!("PROBE {what}: refused {e:?}"),
    }
    r.is_ok()
}

/// The table: false or mis-kinded records handed to a bare finish.
#[test]
fn probe_records_handed_to_a_bare_finish() {
    let k = kiss();
    let body = topo::Body::clone(&k);
    let held = k.records().clone();
    eprintln!("PROBE kiss holds {:?}", held);
    let ax0 = plane_face(&body, 0, 0.0, -1.0);
    let sa = body.solid_of_face(ax0).unwrap();
    let bx2 = plane_face(&body, 0, 2.0, 1.0);
    let sb = body.solid_of_face(bx2).unwrap();
    let ax1 = face_of(&body, 0, 1.0, sa);
    let bx1 = face_of(&body, 0, 1.0, sb);
    let az1 = face_of(&body, 2, 1.0, sa);
    let bz1 = face_of(&body, 2, 1.0, sb);
    let corners = vertex_at(&body, Point3::new(1.0, 1.0, 1.0));
    eprintln!("PROBE corners at the kiss: {corners:?} ax1 {ax1:?} bx1 {bx1:?}");
    let mut passed = Vec::new();
    let mut go = |what: &str, r: ContactRecords| {
        if report(what, &body, r) {
            passed.push(what.to_owned());
        }
    };
    go("as held", held.clone());
    go(
        "faces on one plane, opposed, meeting at a point",
        with(&held, Cell::Face(ax1), Cell::Face(bx1), &body),
    );
    go(
        "a's x=1 face against b's z=1 face (meet at a point)",
        with(&held, Cell::Face(ax1), Cell::Face(bz1), &body),
    );
    go(
        "a's z=1 against b's z=1 (parallel planes, one plane, opposed)",
        with(&held, Cell::Face(az1), Cell::Face(bz1), &body),
    );
    go(
        "same solid: a's x=0 against a's x=1",
        with(&held, Cell::Face(ax0), Cell::Face(ax1), &body),
    );
    for (i, v) in corners.iter().enumerate() {
        go(
            &format!("corner {i} on b's x=1 face (on its boundary only)"),
            with(&held, Cell::Vertex(*v), Cell::Face(bx1), &body),
        );
        go(
            &format!("corner {i} on a's x=0 face (far)"),
            with(&held, Cell::Vertex(*v), Cell::Face(ax0), &body),
        );
    }
    if let Some(first) = held.rows().first() {
        let mut dup = held.clone();
        dup.extend([first.clone()]);
        go("the held record twice", dup);
    }
    // A key from another arena: the record names a face the body does
    // not hold.
    let other = brick((5.0, 6.0), (5.0, 6.0), (5.0, 6.0));
    let foreign = plane_face(&other, 0, 5.0, -1.0);
    if body.face_surface(foreign).is_none() {
        go(
            "a face the body does not hold",
            with(&held, Cell::Face(foreign), Cell::Face(bx1), &body),
        );
    } else {
        eprintln!("PROBE the foreign key aliases a face of the kiss: {foreign:?}");
        go(
            "an aliased foreign key",
            with(&held, Cell::Face(foreign), Cell::Face(bx1), &body),
        );
    }
    eprintln!("PROBE passed: {passed:?}");
}

/// A `⊔`'s public gate, handed a false record by hand.
#[test]
fn probe_a_false_record_through_finish_assembled() {
    let k = kiss();
    let body = topo::Body::clone(&k);
    let held = k.records().clone();
    let ax0 = plane_face(&body, 0, 0.0, -1.0);
    let bx2 = plane_face(&body, 0, 2.0, 1.0);
    let false_record = with(&held, Cell::Face(ax0), Cell::Face(bx2), &body);
    let sa = body.solid_of_face(ax0).unwrap();
    for (label, arrivals) in [
        ("in no arrival", Arc::new(|_| None) as topo::Arrivals),
        ("one arrival", Arc::new(|_| Some(0u32)) as topo::Arrivals),
        (
            "two arrivals",
            Arc::new(move |s| Some(u32::from(s != sa))) as topo::Arrivals,
        ),
    ] {
        let r = topo::finish_assembled(
            Assembled::new(body.clone(), false_record.clone(), arrivals),
            tol(),
        );
        match r {
            Ok(c) => {
                eprintln!(
                    "PROBE assembled {label}: FINISHED holding {} records (bar {:?}); findings {:?}",
                    c.body.records().rows().len(),
                    c.body.bar(),
                    c.findings
                );
                // The finished body is an operand: what does a door do with it?
                let far = brick((10.0, 11.0), (0.0, 1.0), (0.0, 1.0));
                let u = union(&c.body, &far, tol());
                eprintln!("PROBE   union with a far brick: {:?}", u.as_ref().map(|_| "built").map_err(|e| format!("{e:?}").chars().take(300).collect::<String>()));
                let moved = c.body.moved_by(
                    &topo::Isometry::Proper(geom_core::RigidMotion::translation(geom_core::Vec3::new(3.0, 0.0, 0.0)).unwrap()),
                    tol(),
                );
                eprintln!(
                    "PROBE   moved: {:?}",
                    moved.map(|m| m.records().rows().len())
                );
            }
            Err(e) => eprintln!("PROBE assembled {label}: refused {e:?}"),
        }
    }
}

/// A `⊔`'s finished body moved as a leaf (`LeafBody::moved_by`, the
/// evaluator's path for a placement of an assemble at `f64`): which bar
/// does the image hold?
#[test]
fn probe_a_move_of_an_assembled_leaf_keeps_its_bar() {
    let a = brick((0.0, 1.0), (0.0, 1.0), (0.0, 1.0));
    let b = brick((1.0, 2.0), (0.0, 1.0), (0.0, 1.0));
    // A graft of two arrivals face to face: build it as a union's body
    // is not a graft, so graft by hand through the public graft door.
    let mut fused = topo::Body::<f64>::new();
    let ka = topo::graft_disjoint_all_keyed(&mut fused, &a).unwrap();
    let kb = topo::graft_disjoint_all_keyed(&mut fused, &b).unwrap();
    let sa: Vec<topo::SolidKey> = ka.solids().to_vec();
    let _ = kb;
    let arrivals: topo::Arrivals =
        Arc::new(move |s| Some(if sa.contains(&s) { 0u32 } else { 1 }));
    let c = topo::finish_assembled(
        Assembled::new(fused, ContactRecords::default(), arrivals),
        tol(),
    )
    .expect("the graft finishes");
    eprintln!(
        "PROBE assembled: bar {:?}, {} records, findings {:?}",
        c.body.bar(),
        c.body.records().rows().len(),
        c.findings
    );
    let iso = topo::Isometry::Proper(
        geom_core::RigidMotion::translation(geom_core::Vec3::new(3.0, 0.0, 0.0)).unwrap(),
    );
    let as_finished = topo::FinishedBody::moved_by(&c.body, &iso, tol()).unwrap();
    eprintln!(
        "PROBE moved as finished: bar {:?}, {} records",
        as_finished.bar(),
        as_finished.records().rows().len()
    );
    let as_leaf = topo::LeafBody::moved_by(&c.body, &iso, tol()).unwrap();
    eprintln!(
        "PROBE moved as leaf: bar {:?}, {} records",
        as_leaf.bar(),
        as_leaf.records().rows().len()
    );
}

/// The public re-cite door, refused by its closure: what does the
/// finished body hold after?
#[test]
fn probe_a_refused_recite_strips_a_finished_body() {
    let mut k = kiss();
    let before = k.records().rows().len();
    let r: Result<(), ()> =
        topo::FinishedBody::try_recite(&mut k, |_| Err(()), |_| ());
    eprintln!(
        "PROBE recite refused ({r:?}): records {before} -> {}, bar {:?}",
        k.records().rows().len(),
        k.bar()
    );
    // And a re-cite to a row nothing decided is accepted as it stands.
    let mut j = kiss();
    let ok: Result<(), ()> = topo::FinishedBody::try_recite(
        &mut j,
        |_| Ok(vec![topo::Backing::Decided(4_000_000)]),
        |_| (),
    );
    eprintln!("PROBE recite to Decided(4e6): {ok:?}, records {:?}", j.records());
}
