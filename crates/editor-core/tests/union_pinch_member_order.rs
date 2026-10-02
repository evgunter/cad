//! **A pinch builds one body whatever order its members fold in.**
//!
//! Two blocks stand on a plate with footprints that meet at one corner of
//! its top, (1.5, 1, 1), so the plate's top is two pieces meeting at a
//! vertex. The blocks also touch each other along the vertical line
//! through that corner. When the blocks are joined first, their touching
//! edges pierce the plate's top at one point, once each. When the plate
//! comes before one of them, the second block meets that point as a vertex
//! the plate already holds. Both routes have to produce the same body: two
//! top faces sharing one vertex.
//!
//! The rows:
//! - the union in all six member orders: one body, compared by geometry,
//!   with its counts, tier-3 validity, closed-form volume and a
//!   tessellation;
//! - the plate against the joined blocks as a pair boolean, both ways
//!   round, so the pierced face sits on each operand side in turn;
//! - the plate minus the joined blocks, where the pierced face is the
//!   kept A side, and the plate intersected with them, where the two
//!   pierces stay in two faces and so stay two vertices.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::BTreeMap;

use crate::corpus::body_of;
use crate::docm7_union_declare::{block, failure, run};
use crate::fixture::{ends, face_vertices, insert, point};
use editor_core::{BooleanOp, Evaluation, Node, ProfileDoc, RecipeNodeId};
use geom_core::Tol;
use topo::Body;

/// The plate and the two blocks, in a document of their own.
fn pinch(doc: ProfileDoc) -> (ProfileDoc, [RecipeNodeId; 3]) {
    let (doc, plate) = block(doc, (0.0, 3.0), (0.0, 2.0), 0.0, 1.0);
    let (doc, p1) = block(doc, (1.0, 1.5), (-1.0, 1.0), 0.5, 1.5);
    let (doc, p2) = block(doc, (1.5, 2.0), (1.0, 3.0), 0.47, 1.23);
    (doc, [plate, p1, p2])
}

/// The plate's volume plus each block's part above or beside it; the
/// blocks share only a line.
const UNION_VOLUME: f64 = 6.0 + (1.5 - 0.25) + (1.23 - 0.265);
/// The two notches the blocks cut into the plate.
const NOTCHES: f64 = 0.25 + 0.265;

type Point = (i64, i64, i64);

/// A body's geometry, key-free: its faces by the points of their
/// boundary vertices, its edges by their ends, its vertices by where they
/// stand — each a multiset.
#[derive(Debug, PartialEq, Eq)]
struct Shape {
    faces: BTreeMap<Vec<Point>, usize>,
    edges: BTreeMap<[Point; 2], usize>,
    vertices: BTreeMap<Point, usize>,
}

impl Shape {
    /// Faces, edges, vertices.
    fn counts(&self) -> [usize; 3] {
        [
            self.faces.values().sum(),
            self.edges.values().sum(),
            self.vertices.values().sum(),
        ]
    }
}

fn shape(body: &Body<f64>) -> Shape {
    let at = |v| {
        let p = point(body, v);
        let n = |x: f64| (x * 1e6).round() as i64;
        (n(p.x), n(p.y), n(p.z))
    };
    let mut s = Shape {
        faces: BTreeMap::new(),
        edges: BTreeMap::new(),
        vertices: BTreeMap::new(),
    };
    for (f, _) in body.faces() {
        let mut ps: Vec<Point> = face_vertices(body, f).into_iter().map(at).collect();
        ps.sort_unstable();
        *s.faces.entry(ps).or_default() += 1;
    }
    for (e, _) in body.edges() {
        let mut ps = ends(body, e).map(at);
        ps.sort_unstable();
        *s.edges.entry(ps).or_default() += 1;
    }
    for (v, _) in body.vertices() {
        *s.vertices.entry(at(v)).or_default() += 1;
    }
    s
}

/// Asserts `id`'s body is tier-3 valid, tessellates and holds `volume`,
/// and returns its shape.
fn checked(ev: &Evaluation<f64>, id: RecipeNodeId, what: &str, volume: f64) -> Shape {
    if let Some(e) = failure(ev, id) {
        panic!("{what}: refused: {e:?}");
    }
    let body = body_of(ev, id);
    if let Err(e) = topo::validate_geometric(body, Tol::witness()) {
        panic!("{what}: tier 3 refused the body: {e:?}");
    }
    let mesh = mesh::tessellate(body, 1e-3, Tol::witness())
        .unwrap_or_else(|e| panic!("{what}: tessellation refused: {e:?}"));
    let v = mesh::validate::signed_volume(&mesh);
    assert!(
        (v - volume).abs() < 1e-9,
        "{what}: volume {v}, closed form {volume}"
    );
    shape(body)
}

/// The vertices standing at the pinch, (1.5, 1, 1).
fn at_pinch(s: &Shape) -> usize {
    s.vertices
        .get(&(1_500_000, 1_000_000, 1_000_000))
        .copied()
        .unwrap_or(0)
}

/// **Every member order of the union builds one body: the top in two
/// faces sharing one vertex at the pinch.**
#[test]
fn a_pinch_union_builds_one_body_in_every_member_order() {
    let orders: [[usize; 3]; 6] = [
        [0, 1, 2],
        [0, 2, 1],
        [1, 0, 2],
        [1, 2, 0],
        [2, 0, 1],
        [2, 1, 0],
    ];
    let mut first: Option<Shape> = None;
    for order in orders {
        let (doc, m) = pinch(ProfileDoc::empty_derived("union_pinch", Tol::witness()));
        let members: Vec<RecipeNodeId> = order.iter().map(|&i| m[i]).collect();
        let (doc, u) = crate::fixture::union_over(doc, &members, None);
        let what = format!("member order {order:?} (0 = plate)");
        let s = checked(&run(&doc), u, &what, UNION_VOLUME);
        assert_eq!(s.counts(), [19, 49, 32], "{what}: faces, edges, vertices");
        assert_eq!(at_pinch(&s), 1, "{what}: vertices at the pinch");
        match &first {
            None => first = Some(s),
            Some(f) => assert_eq!(&s, f, "{what}: a different body from order [0, 1, 2]"),
        }
    }
}

/// **The plate against the joined blocks: a union both ways round
/// builds the member-order body, a subtraction keeps the pinched top
/// in two faces, and an intersection keeps the two footprints apart.**
#[test]
fn the_plate_against_the_joined_blocks_welds_a_kept_pinch_only() {
    let (doc, [plate, p1, p2]) = pinch(ProfileDoc::empty_derived("union_pinch", Tol::witness()));
    let (doc, blocks) = crate::fixture::union_over(doc, &[p1, p2], None);
    let (doc, folded) = crate::fixture::union_over(doc, &[p1, p2, plate], None);
    let pair = |doc, op, a, b| {
        insert(
            doc,
            Node::Boolean {
                op,
                a,
                b,
                declare: None,
            },
        )
    };
    let (doc, plate_first) = pair(doc, BooleanOp::Union, plate, blocks);
    let (doc, blocks_first) = pair(doc, BooleanOp::Union, blocks, plate);
    let (doc, notched) = pair(doc, BooleanOp::Subtract, plate, blocks);
    let (doc, footprints) = pair(doc, BooleanOp::Intersect, plate, blocks);
    let ev = run(&doc);

    let union = checked(&ev, folded, "the folded union", UNION_VOLUME);
    for (what, id) in [
        ("plate ∪ blocks", plate_first),
        ("blocks ∪ plate", blocks_first),
    ] {
        assert_eq!(
            checked(&ev, id, what, UNION_VOLUME),
            union,
            "{what}: a different body from the member-order union"
        );
    }

    let s = checked(&ev, notched, "plate ∖ blocks", 6.0 - NOTCHES);
    assert_eq!(at_pinch(&s), 1, "plate ∖ blocks: vertices at the pinch");
    let tops = s
        .faces
        .keys()
        .filter(|ps| ps.iter().all(|p| p.2 == 1_000_000))
        .count();
    assert_eq!(tops, 2, "plate ∖ blocks: faces on the top");

    let s = checked(&ev, footprints, "plate ∩ blocks", NOTCHES);
    assert_eq!(
        at_pinch(&s),
        2,
        "plate ∩ blocks: the footprints' corners stay one vertex each"
    );
}
