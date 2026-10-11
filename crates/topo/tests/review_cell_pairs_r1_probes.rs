//! Reviewer probes for PR 4598 (`at-rest-records-are-unordered-cell-pairs`),
//! R1 of the dual: the one list's positional citations across two
//! booleans with records of two kinds, and what `Contact::new`'s
//! refusals do and do not cover.
#![allow(clippy::panic, clippy::expect_used, clippy::unwrap_used)]

use crate::common;
use common::{brick, finished};
use geom_core::Tol;
use topo::{
    AtRestBody, Backing, BooleanBody, BooleanDeclarations, BooleanResult, Cell, Contact,
    ContactClass, ContactKind, ContactRecords, union_with,
};

fn tol() -> Tol {
    Tol::witness()
}

fn box_of(x: (f64, f64), y: (f64, f64), z: (f64, f64), what: &str) -> AtRestBody<f64> {
    finished(what, brick(x, y, z, tol()), tol())
}

fn union(
    a: &AtRestBody<f64>,
    b: &AtRestBody<f64>,
    decls: &BooleanDeclarations,
) -> BooleanBody<f64> {
    match union_with(a, b, decls, tol()) {
        Ok(BooleanResult::Body(r)) => r,
        other => panic!("the union builds: {other:?}"),
    }
}

/// A unit cube balanced on one corner at `(cx, cy, top)`.
fn cube_on_corner(cx: f64, cy: f64, top: f64) -> AtRestBody<f64> {
    use geom_core::{Affine3, Point3, Vec3};
    let cube = brick((-0.5, 0.5), (-0.5, 0.5), (-0.5, 0.5), tol());
    let turn = Affine3::rotation_about_axis(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(1.0, -1.0, 0.0),
        (1.0_f64 / 3.0_f64.sqrt()).acos(),
    );
    let turned = topo::transform_rigid(&cube, &turn, tol()).expect("a rigid turn");
    let low = turned
        .vertex_points()
        .map(|(_, p)| p)
        .min_by(|p, q| p.z.total_cmp(&q.z))
        .expect("a vertex");
    let lift = Affine3::translation(Vec3::new(cx - low.x, cy - low.y, top - low.z));
    finished(
        "the cube on its corner",
        topo::transform_rigid(&turned, &lift, tol()).expect("a rigid lift"),
        tol(),
    )
}

/// A record's cells as points: a vertex's point, an edge's or face's key
/// kind only (enough for the bodies here, whose records are v-v and v-f).
fn shape(body: &topo::Body<f64>, kind: ContactKind) -> String {
    let p = |v| {
        let q = body
            .vertex_points()
            .find(|&(k, _)| k == v)
            .map(|(_, q)| q)
            .expect("a live vertex");
        format!("({:.3},{:.3},{:.3})", q.x, q.y, q.z)
    };
    match kind {
        ContactKind::VertexVertex(a, b) => format!("vv {} {}", p(a), p(b)),
        ContactKind::VertexFace(v, _) => format!("vf {}", p(v)),
        other => format!("{other:?}"),
    }
}

/// **Citations stay faithful across two carries with records of two
/// kinds.** `kiss` holds one vertex–face record; `two` adds a corner
/// kiss (a discovered vertex–vertex record) and carries the vertex–face
/// one, so its list mixes a decided record and a carried one; `three`
/// carries both of `two`'s into a union with a far box. Each of
/// `three`'s records must cite `two`'s record of the same kind at the
/// same point, by position. Red if a carried index counts per kind, or
/// if the list order a record is cited by is not the order `rows()`
/// reads.
#[test]
fn a_carried_citation_names_the_same_record_through_two_booleans() {
    let a = box_of((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), "a");
    let kiss = union(
        &a,
        &cube_on_corner(0.5, 0.5, 1.0),
        &BooleanDeclarations::none(),
    );
    assert_eq!(kiss.contacts.len(), 1, "{:?}", kiss.contacts);
    let corner = box_of((1.0, 2.0), (1.0, 2.0), (-1.0, 0.0), "corner");
    let two = union(
        &kiss.body,
        &corner,
        &BooleanDeclarations {
            carried_a: kiss.contacts.carried(ContactClass::Rest),
            ..BooleanDeclarations::none()
        },
    );
    let kinds2: Vec<_> = two.contacts.kinds().map(|k| shape(&two.body, k)).collect();
    println!("two: {kinds2:?} {:?}", two.contacts);
    assert_eq!(two.contacts.len(), 2, "{:?}", two.contacts);
    let far = box_of((5.0, 6.0), (5.0, 6.0), (5.0, 6.0), "far");
    let three = union(
        &two.body,
        &far,
        &BooleanDeclarations {
            carried_a: two.contacts.carried(ContactClass::Rest),
            ..BooleanDeclarations::none()
        },
    );
    println!("three: {:?}", three.contacts);
    assert_eq!(three.contacts.len(), 2, "{:?}", three.contacts);
    let mut seen = Vec::new();
    for row in three.contacts.rows() {
        let cited: Vec<_> = row.cites.iter().collect();
        let [Backing::Carried { input: 0, record }] = cited[..] else {
            panic!("one carried citation of operand 0: {cited:?}")
        };
        let source = &two.contacts.rows()[record as usize];
        assert_eq!(
            shape(&three.body, row.kind()),
            shape(&two.body, source.kind()),
            "record {row:?} cites {record}, which is {source:?}"
        );
        seen.push(record);
    }
    seen.sort_unstable();
    assert_eq!(seen, [0, 1], "each of two's records cited once");
}

/// **`Contact::new` refuses incidence only between cells of two
/// dimensions**: two faces sharing an edge (a brick's adjacent sides)
/// make a record. Pins the premise behind the PR's tier-3 test edit,
/// which dropped a face pair whose faces share the witness edge as
/// "structure under the new type".
#[test]
fn two_faces_sharing_an_edge_make_a_record() {
    let b = brick::<f64>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol());
    let faces: Vec<_> = b.faces().map(|(k, _)| k).collect();
    let adjacent = faces
        .iter()
        .flat_map(|&f| faces.iter().map(move |&g| (f, g)))
        .find(|&(f, g)| {
            f != g
                && b.edges().any(|(_, e)| {
                    let fs = [e.he_plus, e.he_minus].map(|h| b.face_of_half_edge(h));
                    fs.contains(&Some(f)) && fs.contains(&Some(g))
                })
        })
        .expect("a brick has adjacent faces");
    assert!(
        Contact::new(&b, Cell::Face(adjacent.0), Cell::Face(adjacent.1)).is_ok(),
        "an adjacent face pair is a record"
    );
}

/// **`rekeyed` is public and checks neither incidence nor duplicates**:
/// a non-incident vertex–face record re-keyed onto a vertex of that
/// face holds a pair `Contact::new` refuses, and two records re-keyed
/// onto one pair stay two rows. Demonstrates claim 3's "by another
/// route": the map's incidence and injectivity are the caller's word.
#[test]
fn rekeyed_can_hold_an_incident_pair_and_a_duplicate() {
    let b = brick::<f64>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol());
    let face = b.faces().map(|(k, _)| k).next().expect("a face");
    let on: Vec<_> = b
        .vertices()
        .map(|(v, _)| v)
        .filter(|&v| Contact::new(&b, Cell::Vertex(v), Cell::Face(face)).is_err())
        .collect();
    let off: Vec<_> = b
        .vertices()
        .map(|(v, _)| v)
        .filter(|&v| Contact::new(&b, Cell::Vertex(v), Cell::Face(face)).is_ok())
        .collect();
    assert_eq!((on.len(), off.len()), (4, 4));
    let records: ContactRecords = common::records(
        &b,
        [
            (Cell::Vertex(off[0]), Cell::Face(face)),
            (Cell::Vertex(off[1]), Cell::Face(face)),
        ],
    );
    let moved = records
        .rekeyed(|c| match c {
            Cell::Vertex(_) => Some(Cell::Vertex(on[0])),
            other => Some(other),
        })
        .expect("the map keeps kinds and never collapses one pair");
    assert_eq!(moved.len(), 2, "two rows of one pair: {moved:?}");
    assert_eq!(moved.rows()[0].record, moved.rows()[1].record);
    assert!(
        matches!(
            Contact::new(&b, Cell::Vertex(on[0]), Cell::Face(face)),
            Err(topo::ContactPairError::Incident(_))
        ),
        "the pair the list now holds is one Contact::new refuses"
    );
    assert_eq!(
        moved.position((Cell::Vertex(on[0]), Cell::Face(face))),
        Some(0),
        "position finds only the first of the two"
    );
}
