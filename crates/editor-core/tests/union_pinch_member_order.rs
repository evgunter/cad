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
//! - the union in every member order: one body, compared by geometry,
//!   with its counts, tier-3 validity, closed-form volume and a
//!   tessellation — for the top pinch, for blocks through the plate
//!   (a pinch on the top and one on the bottom), for a third block
//!   making a second pinch on the top, and for two blocks whose
//!   footprints on the plate's side face are holes touching at a corner,
//!   which join into one hole through one vertex;
//! - the plate against the joined blocks as a pair boolean, both ways
//!   round, so the pierced face sits on each operand side in turn;
//! - the plate minus the joined blocks, where the pierced face is the
//!   kept A side, and the plate intersected with them, where the two
//!   pierces stay in two faces and so stay two vertices;
//! - the plate against a slab the two blocks were cut from, in all six
//!   ops: the pierces survive on two fragments of the plate's top, or on
//!   one, and are welded only in the second case.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::BTreeMap;

use crate::corpus::body_of;
use crate::docm7_union_declare::{block, failure, run};
use crate::fixture::{ends, face_vertices, insert, point};
use editor_core::{BooleanOp, Evaluation, Node, ProfileDoc, RecipeNodeId};
use geom_core::Tol;
use topo::Body;

/// The plate and the two blocks, in a document of their own.
fn pinch(doc: ProfileDoc) -> (ProfileDoc, Vec<RecipeNodeId>) {
    let (doc, plate) = block(doc, (0.0, 3.0), (0.0, 2.0), 0.0, 1.0);
    let (doc, p1) = block(doc, (1.0, 1.5), (-1.0, 1.0), 0.5, 1.5);
    let (doc, p2) = block(doc, (1.5, 2.0), (1.0, 3.0), 0.47, 1.23);
    (doc, vec![plate, p1, p2])
}

/// The two blocks run through the plate: a pinch on its top and one on
/// its bottom.
fn through(doc: ProfileDoc) -> (ProfileDoc, Vec<RecipeNodeId>) {
    let (doc, plate) = block(doc, (0.0, 3.0), (0.0, 2.0), 0.0, 1.0);
    let (doc, p1) = block(doc, (1.0, 1.5), (-1.0, 1.0), -0.5, 2.5);
    let (doc, p2) = block(doc, (1.5, 2.0), (1.0, 3.0), -0.47, 2.17);
    (doc, vec![plate, p1, p2])
}

/// [`pinch`] and a third block whose footprint meets `p1`'s at (1, 1, 1):
/// two pinches on the top.
fn two_pinches(doc: ProfileDoc) -> (ProfileDoc, Vec<RecipeNodeId>) {
    let (doc, mut m) = pinch(doc);
    let (doc, p3) = block(doc, (0.5, 1.0), (1.0, 3.0), 0.6, 0.9);
    m.push(p3);
    (doc, m)
}

/// The plate and two blocks through its x = 3 side whose footprints there
/// are holes meeting at (3, 1, 0.5).
fn side_pinch(doc: ProfileDoc) -> (ProfileDoc, Vec<RecipeNodeId>) {
    let (doc, plate) = block(doc, (0.0, 3.0), (0.0, 2.0), 0.0, 1.0);
    let (doc, p1) = block(doc, (2.5, 4.0), (0.5, 1.0), 0.2, 0.3);
    let (doc, p2) = block(doc, (2.47, 3.9), (1.0, 1.5), 0.5, 0.3);
    (doc, vec![plate, p1, p2])
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

/// The vertices standing at `p`.
fn at(s: &Shape, p: Point) -> usize {
    s.vertices.get(&p).copied().unwrap_or(0)
}

const TOP: Point = (1_500_000, 1_000_000, 1_000_000);

/// Every order of `0..n`.
fn orders(n: usize) -> Vec<Vec<usize>> {
    if n == 0 {
        return vec![Vec::new()];
    }
    let mut out = Vec::new();
    for order in orders(n - 1) {
        for i in 0..n {
            let mut o = order.clone();
            o.insert(i, n - 1);
            out.push(o);
        }
    }
    out
}

/// Asserts every member order of `fixture`'s union builds one body,
/// with `counts` (faces, edges, vertices), `volume` and one vertex at
/// each of `pinches`.
fn every_order(
    label: &str,
    fixture: fn(ProfileDoc) -> (ProfileDoc, Vec<RecipeNodeId>),
    counts: [usize; 3],
    volume: f64,
    pinches: &[Point],
) {
    let mut first: Option<Shape> = None;
    let n = fixture(ProfileDoc::empty_derived("union_pinch", Tol::witness()))
        .1
        .len();
    for order in orders(n) {
        let (doc, m) = fixture(ProfileDoc::empty_derived("union_pinch", Tol::witness()));
        let members: Vec<RecipeNodeId> = order.iter().map(|&i| m[i]).collect();
        let (doc, u) = crate::fixture::union_over(doc, &members, None);
        let what = format!("{label}, member order {order:?} (0 = plate)");
        let s = checked(&run(&doc), u, &what, volume);
        assert_eq!(s.counts(), counts, "{what}: faces, edges, vertices");
        for &p in pinches {
            assert_eq!(at(&s, p), 1, "{what}: vertices at the pinch {p:?}");
        }
        match &first {
            None => first = Some(s),
            Some(f) => assert_eq!(&s, f, "{what}: a different body from the first order"),
        }
    }
}

/// **Every member order of the union builds one body: on the top, two
/// faces sharing one vertex at the pinch; on the side, one hole through
/// it.**
#[test]
fn a_pinch_union_builds_one_body_in_every_member_order() {
    every_order("top", pinch, [19, 49, 32], UNION_VOLUME, &[TOP]);
    every_order(
        "side",
        side_pinch,
        [16, 37, 24],
        6.0 + (0.225 - 0.075) + (0.2145 - 0.0795),
        &[(3_000_000, 1_000_000, 500_000)],
    );
}

/// **Blocks through the plate pinch its top and its bottom, and a third
/// block pinches the top a second time: every member order builds one
/// body.** Through the plate, blocks first left two vertices at a pinch
/// (V 42, which did not tessellate) or refused ring homing, by order.
#[test]
fn two_pinches_build_one_body_in_every_member_order() {
    every_order(
        "through",
        through,
        [24, 62, 40],
        6.0 + (2.5 - 0.5) + (2.17 - 0.5),
        &[TOP, (1_500_000, 1_000_000, 0)],
    );
    every_order(
        "two pinches on the top",
        two_pinches,
        [26, 68, 44],
        UNION_VOLUME + 0.5 * (0.9 + 0.5),
        &[TOP, (1_000_000, 1_000_000, 1_000_000)],
    );
}

/// **The plate against the joined blocks: a union both ways round
/// builds the member-order body, a subtraction keeps the pinched top
/// in two faces, and an intersection keeps the two footprints apart.**
#[test]
fn the_plate_against_the_joined_blocks_welds_a_kept_pinch_only() {
    let (doc, m) = pinch(ProfileDoc::empty_derived("union_pinch", Tol::witness()));
    let [plate, p1, p2] = m[..] else {
        panic!("the pinch fixture is three members")
    };
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
    assert_eq!(at(&s, TOP), 1, "plate ∖ blocks: vertices at the pinch");
    let tops = s
        .faces
        .keys()
        .filter(|ps| ps.iter().all(|p| p.2 == 1_000_000))
        .count();
    assert_eq!(tops, 2, "plate ∖ blocks: faces on the top");

    let s = checked(&ev, footprints, "plate ∩ blocks", NOTCHES);
    assert_eq!(
        at(&s, TOP),
        2,
        "plate ∩ blocks: the footprints' corners stay one vertex each"
    );
}

/// **A slab the two blocks were cut from, against the plate, in all six
/// ops.** The slab `X` holds the blocks' contact as two coincident
/// edges, which pierce the plate's top at the pinch.
/// - The plate minus `X`, and either union, keep the two pieces of the
///   top the slab does not cover, and those are two fragments: the
///   pierces stay two vertices, the body the contact builds.
/// - `X` minus the plate, and either intersection, keep the piece the
///   slab covers, which is one fragment running through both pierces:
///   they are welded into one vertex. In the intersection the contact
///   then runs between one vertex at each end, as two coincident edges.
#[test]
fn a_slab_holding_the_contact_welds_only_a_pinch_on_one_fragment() {
    let doc = ProfileDoc::empty_derived("union_pinch", Tol::witness());
    let (doc, slab) = block(doc, (-1.0, 4.0), (-1.0, 3.0), 0.5, 2.5);
    let (doc, p1) = block(doc, (1.0, 1.5), (-2.0, 1.0), 0.3, 2.0);
    let (doc, p2) = block(doc, (1.5, 2.0), (1.0, 4.0), 0.27, 1.73);
    let (doc, plate) = block(doc, (0.0, 3.0), (0.0, 2.0), 0.0, 1.0);
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
    let (doc, cut) = pair(doc, BooleanOp::Subtract, slab, p1);
    let (doc, x) = pair(doc, BooleanOp::Subtract, cut, p2);
    // The slab less the two notches, and the plate's part inside it.
    let x_volume = 50.0 - 0.5 * 2.0 * 1.8 - 0.5 * 2.0 * 1.5;
    let shared = 3.0 - 2.0 * 0.25;
    let rows = [
        (
            "plate ∖ X",
            BooleanOp::Subtract,
            plate,
            x,
            6.0 - shared,
            [15, 36, 23],
            2,
        ),
        (
            "X ∖ plate",
            BooleanOp::Subtract,
            x,
            plate,
            x_volume - shared,
            [23, 61, 40],
            1,
        ),
        (
            "X ∪ plate",
            BooleanOp::Union,
            x,
            plate,
            x_volume + 6.0 - shared,
            [22, 61, 41],
            2,
        ),
        (
            "plate ∪ X",
            BooleanOp::Union,
            plate,
            x,
            x_volume + 6.0 - shared,
            [22, 61, 41],
            2,
        ),
        (
            "X ∩ plate",
            BooleanOp::Intersect,
            x,
            plate,
            shared,
            [16, 36, 22],
            1,
        ),
        (
            "plate ∩ X",
            BooleanOp::Intersect,
            plate,
            x,
            shared,
            [16, 36, 22],
            1,
        ),
    ];
    let mut doc = doc;
    let mut ids = Vec::new();
    for &(_, op, a, b, ..) in &rows {
        let (d, id) = pair(doc, op, a, b);
        doc = d;
        ids.push(id);
    }
    let ev = run(&doc);
    let mut shapes = Vec::new();
    for (&(what, _, _, _, volume, counts, pinch), &id) in rows.iter().zip(&ids) {
        let s = checked(&ev, id, what, volume);
        assert_eq!(s.counts(), counts, "{what}: faces, edges, vertices");
        assert_eq!(at(&s, TOP), pinch, "{what}: vertices at the pinch");
        shapes.push(s);
    }
    assert_eq!(shapes[2], shapes[3], "X ∪ plate and plate ∪ X: one body");
    assert_eq!(shapes[4], shapes[5], "X ∩ plate and plate ∩ X: one body");
}
