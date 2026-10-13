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
//! - three prisms touching pairwise along one vertical line, with the
//!   plate and alone, in every member order: the line held by two
//!   coincident edges meets the third, and folded first the three
//!   pierce the top at one point;
//! - the plate against the joined blocks as a pair boolean, both ways
//!   round, so the pierced face sits on each operand side in turn;
//! - the plate minus the joined blocks, where the pierced face is the
//!   kept A side, and the plate intersected with them, where the two
//!   pierces stay in two faces and so stay two vertices;
//! - the plate against a slab the two blocks were cut from, in all six
//!   ops: the pierces survive on two fragments of the plate's top, or on
//!   one, and are one vertex only in the second case.
//!
//! Each row also reads the result's contact record, its tier-3′ verdict
//! and `check_mesh` on its tessellation: every result passes 3′, every
//! order of one union publishes one record, and `check_mesh` diverges
//! only while [`DOUBLED_EDGE`] stands.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::BTreeMap;

use crate::blocks::{block, failure, run};
use crate::corpus::body_of;
use crate::fixture::{ends, face_vertices, insert, len, on_frame, point};
use editor_core::{
    BooleanValue, Evaluation, ExtrudeSide, Node, ProfileDoc, RecipeNodeId, ValuePayload,
};
use geom_core::Tol;
use topo::BooleanOp;

/// The document's node for the kernel verb `op` over `a` and `b`,
/// undeclared: a two-member union or intersect, or `a` cut by `b`.
fn pair_node(op: BooleanOp, a: RecipeNodeId, b: RecipeNodeId) -> editor_core::AuthoredNode {
    match op {
        BooleanOp::Union => Node::Union {
            members: editor_core::Bodies::Spelled(vec![a.into(), b.into()]),
        },
        BooleanOp::Intersect => Node::Intersect {
            members: editor_core::Bodies::Spelled(vec![a.into(), b.into()]),
        },
        BooleanOp::Subtract => Node::Subtract {
            from: a.into(),
            tool: b.into(),
        },
    }
}
use topo::{Body, ContactRecords};

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

/// A prism over the counterclockwise `corners` from `z0`, `dz` high.
fn prism(doc: ProfileDoc, corners: &[(f64, f64)], z0: f64, dz: f64) -> (ProfileDoc, RecipeNodeId) {
    let (doc, p) = on_frame(
        doc,
        [0.0, 0.0, z0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![corners.to_vec()],
    );
    insert(
        doc,
        Node::Extrude {
            profile: p.into(),
            distance: len(dz),
            side: ExtrudeSide::Along,
        },
    )
}

/// The plate and two blocks whose footprints on its top are holes
/// meeting at the corner (1.5, 1): the blocks of [`pinch`], cut short of
/// the plate's sides.
fn corner_holes(doc: ProfileDoc) -> (ProfileDoc, Vec<RecipeNodeId>) {
    let (doc, plate) = block(doc, (0.0, 3.0), (0.0, 2.0), 0.0, 1.0);
    let (doc, p1) = block(doc, (1.0, 1.5), (0.5, 1.0), 0.5, 1.5);
    let (doc, p2) = block(doc, (1.5, 2.0), (1.0, 1.5), 0.47, 1.23);
    (doc, vec![plate, p1, p2])
}

/// [`corner_holes`] with `p1` through the plate's y = 0 side: a notch
/// and a hole meeting at the corner.
fn notch_and_hole(doc: ProfileDoc) -> (ProfileDoc, Vec<RecipeNodeId>) {
    let (doc, plate) = block(doc, (0.0, 3.0), (0.0, 2.0), 0.0, 1.0);
    let (doc, p1) = block(doc, (1.0, 1.5), (-1.0, 1.0), 0.5, 1.5);
    let (doc, p2) = block(doc, (1.5, 2.0), (1.0, 1.5), 0.47, 1.23);
    (doc, vec![plate, p1, p2])
}

/// The triangle of [`reflex_hole`]'s first block, every edge leaving the
/// pinch inside the L's missing quadrant.
const WEDGE: [(f64, f64); 3] = [(1.5, 1.0), (1.0, 0.6), (1.2, 0.5)];

/// The plate, a wedge-footprint block and an L-footprint block, holes
/// in its top meeting at (1.5, 1): the L's reflex corner, an interior
/// angle of 3π/2, with the wedge inside the quadrant the L leaves.
fn reflex_hole(doc: ProfileDoc) -> (ProfileDoc, Vec<RecipeNodeId>) {
    let (doc, plate) = block(doc, (0.0, 3.0), (0.0, 2.0), 0.0, 1.0);
    let (doc, wedge) = prism(doc, &WEDGE, 0.5, 1.5);
    let ell = [
        (1.0, 1.0),
        (1.5, 1.0),
        (1.5, 0.5),
        (2.0, 0.5),
        (2.0, 1.5),
        (1.0, 1.5),
    ];
    let (doc, l) = prism(doc, &ell, 0.47, 1.23);
    (doc, vec![plate, wedge, l])
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
#[derive(Clone, Debug, PartialEq, Eq)]
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

/// A point rounded to a micron.
fn micron(x: f64, y: f64, z: f64) -> Point {
    let n = |x: f64| (x * 1e6).round() as i64;
    (n(x), n(y), n(z))
}

/// A vertex's point, rounded to a micron.
fn at_point(body: &Body<f64>, v: topo::VertexKey) -> Point {
    let p = point(body, v);
    micron(p.x, p.y, p.z)
}

fn shape(body: &Body<f64>) -> Shape {
    let at = |v| at_point(body, v);
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

/// A contact record, key-free: each vertex-vertex pair by its points,
/// each vertex-on-face rest by its vertex's point (on either side), and
/// the curve and patch rows by count.
#[derive(Clone, Debug, PartialEq, Eq)]
struct Record {
    vv: Vec<[Point; 2]>,
    vf: Vec<Point>,
    curves: usize,
    patches: usize,
}

fn record(body: &Body<f64>, c: &ContactRecords) -> Record {
    let mut vv: Vec<[Point; 2]> = c
        .vertex_vertex()
        .map(|(a, b)| {
            let mut ps = [at_point(body, a), at_point(body, b)];
            ps.sort_unstable();
            ps
        })
        .collect();
    vv.sort_unstable();
    let mut vf: Vec<Point> = c.vertex_face().map(|(v, _)| at_point(body, v)).collect();
    vf.sort_unstable();
    Record {
        vv,
        vf,
        curves: c.edge_face().count(),
        patches: c.face_face().count(),
    }
}

/// What one result is, key-free.
#[derive(Clone, Debug, PartialEq, Eq)]
struct Outcome {
    shape: Shape,
    record: Record,
    /// The tier-3′ verdict over the body and its record, each error by
    /// its kind and, for an unrecorded contact, its census kind and
    /// witness point.
    verdict: Result<(), Vec<(String, Option<Point>)>>,
    /// `mesh::validate::check_mesh` on the tessellation, by error kind.
    manifold: Result<(), String>,
}

/// A census witness `"(x, y, z)…"` as a point rounded to a micron.
fn witness_point(witness: &str) -> Option<Point> {
    let inside = witness.strip_prefix('(')?.split(')').next()?;
    let n: Vec<i64> = inside
        .split(", ")
        .map(|x| x.parse::<f64>().map(|x| (x * 1e6).round() as i64))
        .collect::<Result<_, _>>()
        .ok()?;
    match n[..] {
        [x, y, z] => Some((x, y, z)),
        _ => None,
    }
}

/// An error's variant name: its `Debug` up to the first field.
fn kind(debug: &str) -> &str {
    debug.split([' ', '{', '(']).next().unwrap_or(debug)
}

/// Asserts `id`'s body is tier-3 valid, tessellates and holds `volume`,
/// and returns what it is.
fn checked(ev: &Evaluation<f64>, id: RecipeNodeId, what: &str, volume: f64) -> Outcome {
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
    let none = ContactRecords::default();
    let contacts = match &ev.value(id).expect("node evaluated to a value").payload {
        ValuePayload::Boolean(BooleanValue::Body { contacts, .. }) => &**contacts,
        _ => &none,
    };
    let verdict = topo::validate_pseudomanifold(body, contacts, Tol::witness()).map_err(|es| {
        let mut es: Vec<(String, Option<Point>)> = es
            .iter()
            .map(|e| match e {
                topo::ValidationError::UnrecordedContact {
                    contact, witness, ..
                } => (
                    format!("UnrecordedContact {}", kind(&format!("{contact:?}"))),
                    witness_point(witness),
                ),
                other => (kind(&format!("{other:?}")).to_owned(), None),
            })
            .collect();
        es.sort_unstable();
        es
    });
    if let Err(e) = topo::test_support::meeting::corners_disjoint(body, Tol::witness()) {
        panic!("{what}: {e}");
    }
    Outcome {
        shape: shape(body),
        record: record(body, contacts),
        verdict,
        manifold: mesh::validate::check_mesh(&mesh).map_err(|e| kind(&format!("{e:?}")).to_owned()),
    }
}

/// The vertices standing at `p`.
fn at(s: &Shape, p: Point) -> usize {
    s.vertices.get(&p).copied().unwrap_or(0)
}

/// The pinch point on the plate's top: [`MEET`], rounded as
/// [`at_point`] rounds a vertex.
fn top() -> Point {
    micron(MEET[0], MEET[1], MEET[2])
}

/// Asserts `o`, node `id`'s result, records its two vertices at the
/// pinch as a pair, and that the node recorded a `VertexFusion`: the
/// ring that served both blocks' pierces of the plate's top grouped
/// them by value (`topo` `vtxfac::ring_groups`).
fn ring_fusion_recorded(ev: &Evaluation<f64>, id: RecipeNodeId, o: &Outcome, what: &str) {
    assert!(
        o.record.vv.contains(&[top(), top()]),
        "{what}: a record of the two vertices at the pinch: {:?}",
        o.record.vv
    );
    let value = ev.value(id).expect("the node evaluated");
    assert!(
        value
            .coincidences
            .iter()
            .any(|c| c.site == topo::DecisionSite::VertexFusion),
        "{what}: the ring's grouping recorded: {:?}",
        value.coincidences
    );
}

/// Asserts node `id`'s result holds one overlap of two edges along the
/// blocks' contact line x = 1.5, y = 1, and that its end away from the
/// pinch is backed: the two edges end at one vertex there, or one ends
/// on the other's interior and a vertex-on-edge record names that vertex
/// and edge. The end at the pinch is the pair [`ring_fusion_recorded`]
/// reads.
fn far_end_backed(ev: &Evaluation<f64>, id: RecipeNodeId, what: &str) {
    let body = body_of(ev, id);
    let none = ContactRecords::default();
    let contacts = match &ev.value(id).expect("the node evaluated").payload {
        ValuePayload::Boolean(BooleanValue::Body { contacts, .. }) => &**contacts,
        _ => &none,
    };
    let z = |v: topo::VertexKey| {
        body.vertex_points()
            .find(|&(k, _)| k == v)
            .map(|(_, p)| p)
            .expect("a vertex of the body")
    };
    let on_line = |v| {
        let p = z(v);
        (p.x - 1.5).abs() < 1e-9 && (p.y - 1.0).abs() < 1e-9
    };
    let line: Vec<(topo::EdgeKey, [topo::VertexKey; 2])> = body
        .edges()
        .filter_map(|(e, d)| {
            let ends = [
                body.get_half_edge(d.he_plus)?.start,
                body.get_half_edge(d.he_minus)?.start,
            ];
            (ends[0] != ends[1] && ends.iter().all(|&v| on_line(v))).then_some((e, ends))
        })
        .collect();
    let span = |ends: [topo::VertexKey; 2]| {
        let (a, b) = (z(ends[0]).z, z(ends[1]).z);
        (a.min(b), a.max(b))
    };
    let mut overlaps = Vec::new();
    for (i, &(e1, ends1)) in line.iter().enumerate() {
        for &(e2, ends2) in &line[i + 1..] {
            let ((lo1, hi1), (lo2, hi2)) = (span(ends1), span(ends2));
            if hi1.min(hi2) - lo1.max(lo2) > 1e-9 {
                overlaps.push(((e1, ends1), (e2, ends2), lo1.max(lo2), hi1.min(hi2)));
            }
        }
    }
    let [((e1, ends1), (e2, ends2), lo, hi)] = overlaps[..] else {
        panic!("{what}: one overlap along the contact line: {overlaps:?}")
    };
    let far = if (hi - 1.0).abs() < 1e-9 { lo } else { hi };
    let at = |ends: [topo::VertexKey; 2]| ends.into_iter().find(|&v| (z(v).z - far).abs() < 1e-9);
    match (at(ends1), at(ends2)) {
        (Some(v), Some(w)) => assert_eq!(v, w, "{what}: the overlap's far end is one vertex"),
        (Some(v), None) => assert!(
            contacts.vertex_edge().any(|r| r == (v, e2)),
            "{what}: a vertex-on-edge record backs the overlap's far end"
        ),
        (None, Some(w)) => assert!(
            contacts.vertex_edge().any(|r| r == (w, e1)),
            "{what}: a vertex-on-edge record backs the overlap's far end"
        ),
        (None, None) => panic!("{what}: neither edge ends at the overlap's far end"),
    }
}

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

/// The double edge two touching solids leave meshes as one segment of
/// four triangles, which `check_mesh` refuses.
const DOUBLED_EDGE: &str =
    "work/tess/two-coincident-edges-between-one-vertex-pair-mesh-non-manifold.md";

/// Asserts every member order of `fixture`'s union builds one body,
/// with `counts` (faces, edges, vertices), `volume` and one vertex at
/// each of `pinches`, passes 3′, and publishes one record and one mesh
/// verdict.
fn every_order(
    label: &str,
    fixture: fn(ProfileDoc) -> (ProfileDoc, Vec<RecipeNodeId>),
    counts: [usize; 3],
    volume: f64,
    pinches: &[Point],
) {
    let mut first: Option<Outcome> = None;
    let n = fixture(ProfileDoc::empty_derived("union_pinch", Tol::witness()))
        .1
        .len();
    for order in orders(n) {
        let (doc, m) = fixture(ProfileDoc::empty_derived("union_pinch", Tol::witness()));
        let members: Vec<RecipeNodeId> = order.iter().map(|&i| m[i]).collect();
        let (doc, u) = crate::fixture::union_over(doc, &members);
        let what = format!("{label}, member order {order:?} (0 = plate)");
        let ev = run(&doc);
        let o = checked(&ev, u, &what, volume);
        assert_eq!(o.shape.counts(), counts, "{what}: faces, edges, vertices");
        for &p in pinches {
            assert_eq!(at(&o.shape, p), 1, "{what}: vertices at the pinch {p:?}");
        }
        assert_eq!(o.verdict, Ok(()), "{what}: 3′");
        if let Some(f) = &first {
            assert_eq!(
                o.shape, f.shape,
                "{what}: a different body from the first order"
            );
            assert_eq!(
                o.record, f.record,
                "{what}: a different record from the first order"
            );
            assert_eq!(
                o.manifold, f.manifold,
                "{what}: a different mesh verdict from the first order"
            );
        } else {
            first = Some(o);
        }
    }
    assert_eq!(
        first.map(|f| f.manifold),
        Some(Ok(())),
        "{label}: check_mesh"
    );
}

/// **Every member order of the union builds one body: on the top, two
/// faces sharing one vertex at the pinch; on the side, one hole through
/// it.**
#[test]
fn a_pinch_union_builds_one_body_in_every_member_order() {
    every_order("top", pinch, [19, 48, 31], UNION_VOLUME, &[top()]);
    every_order(
        "side",
        side_pinch,
        [16, 36, 23],
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
        [24, 60, 38],
        6.0 + (2.5 - 0.5) + (2.17 - 0.5),
        &[top(), (1_500_000, 1_000_000, 0)],
    );
    every_order(
        "two pinches on the top",
        two_pinches,
        [26, 66, 42],
        UNION_VOLUME + 0.5 * (0.9 + 0.5),
        &[top(), (1_000_000, 1_000_000, 1_000_000)],
    );
}

/// **Holes touching at a corner of the top build one body in every
/// member order**, and so do a notch and a hole, and a wedge in an L's
/// reflex corner. When the blocks fold first, each of their two edges at
/// the pinch pierces the top there, and the second pierce's ring is a
/// strut with every point on the first polygon's outline; it is placed
/// with its own polygon.
#[test]
fn holes_touching_at_a_corner_build_one_body_in_every_member_order() {
    every_order(
        "corner holes",
        corner_holes,
        [16, 36, 23],
        6.0 + 0.25 * (2.0 - 1.0) + 0.25 * (1.7 - 1.0),
        &[top()],
    );
    every_order(
        "notch and hole",
        notch_and_hole,
        [17, 42, 27],
        6.0 + 0.5 * 2.0 * (2.0 - 1.0) + 0.5 * 1.0 * (1.0 - 0.5) + 0.25 * (1.7 - 1.0),
        &[top()],
    );
    let wedge = 0.5
        * ((WEDGE[1].0 - WEDGE[0].0) * (WEDGE[2].1 - WEDGE[0].1)
            - (WEDGE[1].1 - WEDGE[0].1) * (WEDGE[2].0 - WEDGE[0].0));
    every_order(
        "wedge in an L",
        reflex_hole,
        [17, 39, 25],
        6.0 + wedge * (2.0 - 1.0) + 0.75 * (1.7 - 1.0),
        &[top()],
    );
}

/// **The plate against the joined blocks: a union both ways round
/// builds the member-order body, a subtraction keeps the pinched top
/// in two faces, and an intersection keeps the two footprints apart.**
#[test]
fn the_plate_against_the_joined_blocks_builds_in_every_op() {
    let (doc, m) = pinch(ProfileDoc::empty_derived("union_pinch", Tol::witness()));
    let [plate, p1, p2] = m[..] else {
        panic!("the pinch fixture is three members")
    };
    let (doc, blocks) = crate::fixture::union_over(doc, &[p1, p2]);
    let (doc, folded) = crate::fixture::union_over(doc, &[p1, p2, plate]);
    let pair = |doc, op, a: RecipeNodeId, b: RecipeNodeId| insert(doc, pair_node(op, a, b));
    let (doc, plate_first) = pair(doc, BooleanOp::Union, plate, blocks);
    let (doc, blocks_first) = pair(doc, BooleanOp::Union, blocks, plate);
    let (doc, notched) = pair(doc, BooleanOp::Subtract, plate, blocks);
    let (doc, footprints) = pair(doc, BooleanOp::Intersect, plate, blocks);
    let ev = run(&doc);

    // The joined blocks' own contact record carries into each result.
    let m = |what: &str, o: Outcome| {
        assert_eq!(o.verdict, Ok(()), "{what}: 3′");
        assert_eq!(o.manifold, Ok(()), "{what}: check_mesh");
        o
    };
    let union = m(
        "the folded union",
        checked(&ev, folded, "the folded union", UNION_VOLUME),
    );
    for (what, id) in [
        ("plate ∪ blocks", plate_first),
        ("blocks ∪ plate", blocks_first),
    ] {
        let o = m(what, checked(&ev, id, what, UNION_VOLUME));
        assert_eq!(
            o.shape, union.shape,
            "{what}: a different body from the member-order union"
        );
        assert_eq!(
            o.record, union.record,
            "{what}: a different record from the member-order union"
        );
    }

    let s = m(
        "plate ∖ blocks",
        checked(&ev, notched, "plate ∖ blocks", 6.0 - NOTCHES),
    )
    .shape;
    assert_eq!(at(&s, top()), 1, "plate ∖ blocks: vertices at the pinch");
    let tops = s
        .faces
        .keys()
        .filter(|ps| ps.iter().all(|p| p.2 == 1_000_000))
        .count();
    assert_eq!(tops, 2, "plate ∖ blocks: faces on the top");

    let o = checked(&ev, footprints, "plate ∩ blocks", NOTCHES);
    assert_eq!(o.verdict, Ok(()), "plate ∩ blocks: 3′");
    ring_fusion_recorded(&ev, footprints, &o, "plate ∩ blocks");
    far_end_backed(&ev, footprints, "plate ∩ blocks");
    assert_eq!(o.manifold, Ok(()), "plate ∩ blocks: check_mesh");
    let s = o.shape;
    assert_eq!(
        at(&s, top()),
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
///   they are one vertex. In the intersection the contact
///   then runs between one vertex at each end, as two coincident edges.
#[test]
fn a_slab_holding_the_contact_is_one_vertex_only_on_one_fragment() {
    let doc = ProfileDoc::empty_derived("union_pinch", Tol::witness());
    let (doc, slab) = block(doc, (-1.0, 4.0), (-1.0, 3.0), 0.5, 2.5);
    let (doc, p1) = block(doc, (1.0, 1.5), (-2.0, 1.0), 0.3, 2.0);
    let (doc, p2) = block(doc, (1.5, 2.0), (1.0, 4.0), 0.27, 1.73);
    let (doc, plate) = block(doc, (0.0, 3.0), (0.0, 2.0), 0.0, 1.0);
    let pair = |doc, op, a: RecipeNodeId, b: RecipeNodeId| insert(doc, pair_node(op, a, b));
    // One cut: cutting the blocks one at a time sets the second block's
    // wall flush with the first's hole wall, an undeclared continuation.
    let (doc, blocks) = crate::fixture::union_over(doc, &[p1, p2]);
    let (doc, x) = pair(doc, BooleanOp::Subtract, slab, blocks);
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
            [23, 60, 39],
            1,
        ),
        (
            "X ∪ plate",
            BooleanOp::Union,
            x,
            plate,
            x_volume + 6.0 - shared,
            [22, 60, 40],
            2,
        ),
        (
            "plate ∪ X",
            BooleanOp::Union,
            plate,
            x,
            x_volume + 6.0 - shared,
            [22, 60, 40],
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
    let mut outcomes = Vec::new();
    for (&(what, op, _, _, volume, counts, pinch), &id) in rows.iter().zip(&ids) {
        let o = checked(&ev, id, what, volume);
        // X's own contact record carries into each result. Where the op
        // keeps the pierce as two vertices, a record of the pair cites the
        // fusion the ring serving both pierces decided.
        assert_eq!(o.verdict, Ok(()), "{what}: 3′");
        if pinch == 2 {
            ring_fusion_recorded(&ev, id, &o, what);
            far_end_backed(&ev, id, what);
        }
        if op == BooleanOp::Intersect {
            // Two L-prisms touching along the contact line, which runs
            // between one vertex at each end as two coincident edges.
            assert_eq!(
                o.manifold,
                Err("NonManifoldEdge".to_owned()),
                "{what}: check_mesh, while {DOUBLED_EDGE} stands"
            );
        } else {
            assert_eq!(o.manifold, Ok(()), "{what}: check_mesh");
        }
        assert_eq!(o.shape.counts(), counts, "{what}: faces, edges, vertices");
        assert_eq!(at(&o.shape, top()), pinch, "{what}: vertices at the pinch");
        outcomes.push(o);
    }
    for (i, what) in [
        (2, "X ∪ plate and plate ∪ X"),
        (4, "X ∩ plate and plate ∩ X"),
    ] {
        let (o, swapped) = (&outcomes[i], &outcomes[i + 1]);
        assert_eq!(o.shape, swapped.shape, "{what}: one body");
        assert_eq!(o.record, swapped.record, "{what}: one record");
    }
}

use topo::test_support::meeting::{
    Hole, MEET, PLATE, ell_and_wedges, four_wedges, three_wedges, two_wedges, wedges_on_one_side,
};

/// The plate and a prism per hole, each sketched on its tilted frame
/// and extruded along it: above the top the prisms lean apart and touch
/// nowhere, below it they cross inside the plate.
fn tilted_holes(doc: ProfileDoc, holes: &[Hole]) -> (ProfileDoc, Vec<RecipeNodeId>) {
    let (mut doc, plate) = block(doc, PLATE[0], PLATE[1], PLATE[2].0, PLATE[2].1);
    let mut m = vec![plate];
    for h in holes {
        let [o, u, v, _] = h.frame();
        let (d, p) = on_frame(doc, o, u, v, vec![h.profile()]);
        let (d, w) = insert(
            d,
            Node::Extrude {
                profile: p.into(),
                distance: len(h.length),
                side: ExtrudeSide::Along,
            },
        );
        doc = d;
        m.push(w);
    }
    (doc, m)
}

/// A row over [`tilted_holes`]: its label, its holes, the fixture over
/// them and its faces, edges and vertices.
type TiltedRow = (
    &'static str,
    fn() -> Vec<Hole>,
    fn(ProfileDoc) -> (ProfileDoc, Vec<RecipeNodeId>),
    [usize; 3],
);

/// The plate's volume plus each prism's part above its top.
fn tilted_volume(holes: &[Hole]) -> f64 {
    6.0 + holes.iter().map(Hole::above).sum::<f64>()
}

/// **Two, three and four wedges whose footprints are holes meeting at
/// one vertex of the top build one body in every member order.** With
/// three or more wedges folded before the plate, their vertex there
/// pierces the top with one Out run per wedge, and the pierce's ring
/// struts hang round its ring vertex in the runs' angular order.
#[test]
fn wedges_meeting_at_a_vertex_build_one_body_in_every_member_order() {
    let rows: [TiltedRow; 3] = [
        (
            "two wedges",
            two_wedges,
            |d| tilted_holes(d, &two_wedges()),
            [14, 30, 19],
        ),
        (
            "three wedges",
            three_wedges,
            |d| tilted_holes(d, &three_wedges()),
            [18, 39, 24],
        ),
        (
            "four wedges",
            four_wedges,
            |d| tilted_holes(d, &four_wedges()),
            [22, 48, 29],
        ),
    ];
    for (label, holes, fixture, counts) in rows {
        every_order(label, fixture, counts, tilted_volume(&holes()), &[top()]);
    }
}

/// **Holes meeting at a vertex with a reflex sector there build one body
/// in every member order**: three wedges on one side, leaving the top a
/// reflex sector between them, and an L-shaped hole whose reflex corner
/// is the vertex, with two wedges in the quadrant it leaves.
#[test]
fn holes_with_a_reflex_sector_at_their_vertex_build_one_body_in_every_member_order() {
    let rows: [TiltedRow; 2] = [
        (
            "three wedges on one side",
            wedges_on_one_side,
            |d| tilted_holes(d, &wedges_on_one_side()),
            [18, 39, 24],
        ),
        (
            "an L and two wedges",
            ell_and_wedges,
            |d| tilted_holes(d, &ell_and_wedges()),
            [21, 48, 30],
        ),
    ];
    for (label, holes, fixture, counts) in rows {
        every_order(label, fixture, counts, tilted_volume(&holes()), &[top()]);
    }
}

/// The names the union's table binds to the vertex at `p`.
fn names_at(ev: &Evaluation<f64>, id: RecipeNodeId, p: Point) -> Vec<String> {
    let body = body_of(ev, id);
    let mut out: Vec<String> = crate::fixture::table(ev, id)
        .iter()
        .filter(|(_, e)| {
            let keys: Vec<editor_core::EntityKey> = match e {
                editor_core::Entry::Unique(r) => vec![r.key],
                editor_core::Entry::Tied(rs) => rs.iter().map(|r| r.key).collect(),
            };
            keys.iter()
                .any(|k| matches!(k, editor_core::EntityKey::Vertex(v) if at_point(body, *v) == p))
        })
        .map(|(n, _)| format!("{n:?}"))
        .collect();
    out.sort_unstable();
    out
}

/// The senses a vertex name's `EdgeCrossing` run carries, line by
/// line in name order: an edge's, or `None` for a seam.
fn run_senses(name: &str) -> Vec<String> {
    name.split("sense: ")
        .skip(1)
        .map(|rest| {
            rest.split([' ', ',', '}'])
                .next()
                .unwrap_or_default()
                .to_owned()
        })
        .collect()
}

/// Asserts every order of `orders` (member indices into
/// [`tilted_holes`]'s list, `0` the plate) names the point where
/// `holes`' legs meet with one `EdgeCrossing` run of the legs, each with
/// sense `sense`, the same in every order.
fn one_run(label: &str, holes: fn() -> Vec<Hole>, sense: &str, orders: &[Vec<usize>]) {
    let mut first: Option<Vec<String>> = None;
    for order in orders {
        let (doc, m) = tilted_holes(
            ProfileDoc::empty_derived("union_pinch", Tol::witness()),
            &holes(),
        );
        let members: Vec<RecipeNodeId> = order.iter().map(|&i| m[i]).collect();
        let (doc, u) = crate::fixture::union_over(doc, &members);
        let ev = run(&doc);
        if let Some(e) = failure(&ev, u) {
            panic!("{label}, member order {order:?}: refused: {e:?}");
        }
        let names = names_at(&ev, u, top());
        let [name] = names.as_slice() else {
            panic!("{label}, member order {order:?}: one name at the meeting point: {names:?}");
        };
        assert!(
            name.contains("path: [EdgeCrossing(")
                && name.matches("role: Leg").count() == holes().len(),
            "{label}, member order {order:?}: the run of the legs: {name}"
        );
        assert_eq!(
            run_senses(name),
            vec![format!("Some({sense})"); holes().len()],
            "{label}, member order {order:?}: each leg against the rest: {name}"
        );
        match &first {
            None => first = Some(names),
            Some(f) => assert_eq!(
                &names, f,
                "{label}, member order {order:?}: the meeting point's name"
            ),
        }
    }
}

/// Every order of `n` members, or with `plate` false every order of
/// members `1..n`.
fn every_order_of(n: usize, plate: bool) -> Vec<Vec<usize>> {
    orders(n)
        .into_iter()
        .filter(|o| plate || o[n - 1] == 0)
        .map(|o| o.into_iter().filter(|&i| plate || i != 0).collect())
        .collect()
}

/// **Where three or more wedges' legs meet, the vertex is one run of the
/// legs whichever fold step minted it** (N2, `emit_union::Flush::meeting`).
/// Three wedges and the plate, folded plate first and plate last; every
/// order is [`the_junction_where_the_wedges_meet_has_one_name_in_every_member_order`]'s.
#[test]
fn the_junction_where_three_wedges_meet_has_one_name_plate_first_or_last() {
    one_run(
        "three wedges and the plate",
        three_wedges,
        "Outside",
        &[vec![0, 1, 2, 3], vec![3, 2, 1, 0]],
    );
}

/// **Where three or more wedges' legs meet, the vertex is one run of the
/// legs in every member order**, with the plate and without it (N2,
/// `emit_union::Flush::meeting`): the legs locate the point, so the
/// plate, whose top passes through it, is not in the name. Each leg's
/// sense is read against the other wedges together: three lean apart,
/// each leg outside the other two on both sides; four lean so that
/// opposite wedges overlap below the top, so each leg leaves the rest.
#[test]
fn the_junction_where_the_wedges_meet_has_one_name_in_every_member_order() {
    type Row = (&'static str, fn() -> Vec<Hole>, bool, &'static str);
    let rows: [Row; 4] = [
        ("three wedges and the plate", three_wedges, true, "Outside"),
        ("four wedges and the plate", four_wedges, true, "Leaves"),
        ("three wedges", three_wedges, false, "Outside"),
        ("four wedges", four_wedges, false, "Leaves"),
    ];
    for (label, holes, plate, sense) in rows {
        one_run(
            label,
            holes,
            sense,
            &every_order_of(holes().len() + 1, plate),
        );
    }
}

/// **Where two leaning wedges' legs meet at the plate's top, their union
/// names the vertex the edge crossing of the two legs, each touching the
/// other wedge from outside, in either member order**: each leg lies
/// outside the other wedge on both sides of the vertex. The fold reaches
/// one wedge's copy of the vertex only through two fusions
/// (`names::emit_topo::fused_partners`).
#[test]
fn two_wedges_name_their_meeting_point_the_touch_of_their_legs_in_either_order() {
    let mut first: Option<Vec<String>> = None;
    for order in [[1, 2], [2, 1]] {
        let (doc, m) = tilted_holes(
            ProfileDoc::empty_derived("union_pinch", Tol::witness()),
            &two_wedges(),
        );
        let (doc, u) = crate::fixture::union_over(doc, &[m[order[0]], m[order[1]]]);
        let ev = run(&doc);
        if let Some(e) = failure(&ev, u) {
            panic!("member order {order:?}: refused: {e:?}");
        }
        let names = names_at(&ev, u, top());
        assert_eq!(
            names.len(),
            1,
            "member order {order:?}: the meeting point's names"
        );
        let n = &names[0];
        assert!(
            n.contains("path: [EdgeCrossing(") && run_senses(n) == ["Some(Outside)"; 2],
            "member order {order:?}: the meeting point is the legs' touch from outside: {n}"
        );
        assert_eq!(
            n.matches("role: Leg").count(),
            2,
            "member order {order:?}: the crossing is of the two legs: {n}"
        );
        match &first {
            None => first = Some(names),
            Some(f) => assert_eq!(
                &names, f,
                "member order {order:?}: the meeting point's name"
            ),
        }
    }
}

/// An upright prism over the triangle from (1.5, 1) to the points at the
/// bearings `a0` and `a1` (degrees) 0.4 from it, over `z`.
fn sector_prism(doc: ProfileDoc, a0: f64, a1: f64, z: (f64, f64)) -> (ProfileDoc, RecipeNodeId) {
    let at = |a: f64| {
        let (s, c) = f64::to_radians(a).sin_cos();
        (0.4f64.mul_add(c, MEET[0]), 0.4f64.mul_add(s, MEET[1]))
    };
    prism(doc, &[(MEET[0], MEET[1]), at(a0), at(a1)], z.0, z.1 - z.0)
}

/// The plate and three prisms over 50° sectors about (1.5, 1), each 70°
/// from the next, touching pairwise along the vertical line through it.
fn three_on_a_line(doc: ProfileDoc) -> (ProfileDoc, Vec<RecipeNodeId>) {
    let (doc, plate) = block(doc, (0.0, 3.0), (0.0, 2.0), 0.0, 1.0);
    let (doc, a) = sector_prism(doc, 0.0, 50.0, (0.5, 2.0));
    let (doc, b) = sector_prism(doc, 120.0, 170.0, (0.47, 1.7));
    let (doc, c) = sector_prism(doc, 240.0, 290.0, (0.44, 1.81));
    (doc, vec![plate, a, b, c])
}

/// The plate and four prisms over 50° sectors about (1.5, 1), each 40°
/// from the next, touching pairwise along the vertical line through it.
fn four_on_a_line(doc: ProfileDoc) -> (ProfileDoc, Vec<RecipeNodeId>) {
    let (doc, plate) = block(doc, (0.0, 3.0), (0.0, 2.0), 0.0, 1.0);
    let (doc, a) = sector_prism(doc, 0.0, 50.0, (0.5, 2.0));
    let (doc, b) = sector_prism(doc, 90.0, 140.0, (0.47, 1.7));
    let (doc, c) = sector_prism(doc, 180.0, 230.0, (0.44, 1.81));
    let (doc, d) = sector_prism(doc, 270.0, 320.0, (0.41, 1.63));
    (doc, vec![plate, a, b, c, d])
}

/// [`four_on_a_line`] without the plate.
fn four_prisms(doc: ProfileDoc) -> (ProfileDoc, Vec<RecipeNodeId>) {
    let (doc, m) = four_on_a_line(doc);
    (doc, m[1..].to_vec())
}

/// [`three_on_a_line`] without the plate.
fn three_prisms(doc: ProfileDoc) -> (ProfileDoc, Vec<RecipeNodeId>) {
    let (doc, m) = three_on_a_line(doc);
    (doc, m[1..].to_vec())
}

/// Asserts every member order of `fixture`'s union builds one body, as
/// [`every_order`] does, where three solids touch along one line.
fn every_order_on_a_line(
    label: &str,
    fixture: fn(ProfileDoc) -> (ProfileDoc, Vec<RecipeNodeId>),
    counts: [usize; 3],
    volume: f64,
    pinch: bool,
) {
    let mut first: Option<Outcome> = None;
    let n = fixture(ProfileDoc::empty_derived("on_a_line", Tol::witness()))
        .1
        .len();
    for order in orders(n) {
        let (doc, m) = fixture(ProfileDoc::empty_derived("on_a_line", Tol::witness()));
        let members: Vec<RecipeNodeId> = order.iter().map(|&i| m[i]).collect();
        let (doc, u) = crate::fixture::union_over(doc, &members);
        let what = format!("{label}, member order {order:?}");
        let o = checked(&run(&doc), u, &what, volume);
        assert_eq!(o.shape.counts(), counts, "{what}: faces, edges, vertices");
        if pinch {
            assert_eq!(at(&o.shape, top()), 1, "{what}: vertices at the pinch");
        }
        assert_eq!(o.verdict, Ok(()), "{what}: 3′");
        if let Some(f) = &first {
            assert_eq!(
                o.shape, f.shape,
                "{what}: a different body from the first order"
            );
            assert_eq!(o.record, f.record, "{what}: a different record");
            assert_eq!(o.manifold, f.manifold, "{what}: a different mesh verdict");
        } else {
            first = Some(o);
        }
    }
    assert_eq!(
        first.map(|f| f.manifold),
        Some(Ok(())),
        "{label}: check_mesh"
    );
}

/// **Three and four solids touching along one line build one body in
/// every member order**, with the plate and alone: the third meets the first two's
/// coincident edges along the ray they hold, and, folded first, the
/// three edges pierce the plate's top at one point once each.
#[test]
fn solids_touching_along_one_line_build_one_body_in_every_member_order() {
    let above = 0.5 * 0.16 * 50f64.to_radians().sin();
    every_order_on_a_line(
        "three prisms and the plate",
        three_on_a_line,
        [18, 39, 24],
        above.mul_add(1.0 + 0.7 + 0.81, 6.0),
        true,
    );
    every_order_on_a_line(
        "three prisms",
        three_prisms,
        [15, 27, 18],
        above * (1.5 + 1.23 + 1.37),
        false,
    );
    every_order_on_a_line(
        "four prisms and the plate",
        four_on_a_line,
        [22, 48, 29],
        above.mul_add(1.0 + 0.7 + 0.81 + 0.63, 6.0),
        true,
    );
    every_order_on_a_line(
        "four prisms",
        four_prisms,
        [20, 36, 24],
        above * (1.5 + 1.23 + 1.37 + 1.22),
        false,
    );
}

/// **Two edges of one member that run flush where they pierce another
/// are one line** (N2): the joined blocks hold their contact as two
/// coincident edges, which pierce the plate's top at the pinch. They are
/// cited once, as the least, so the pinch is that edge's `Crossing` of
/// the plate's top, leaving the plate, alike in both orders.
#[test]
fn two_edges_of_one_member_at_the_pinch_are_one_crossing() {
    let (doc, m) = pinch(ProfileDoc::empty_derived("union_pinch", Tol::witness()));
    let [plate, p1, p2] = m[..] else {
        panic!("the pinch fixture is three members")
    };
    let (doc, blocks) = crate::fixture::union_over(doc, &[p1, p2]);
    let mut first: Option<Vec<String>> = None;
    for members in [[plate, blocks], [blocks, plate]] {
        let (doc, u) = crate::fixture::union_over(doc.clone(), &members);
        let ev = run(&doc);
        if let Some(e) = failure(&ev, u) {
            panic!("{members:?}: refused: {e:?}");
        }
        let names = names_at(&ev, u, top());
        assert!(
            matches!(names.as_slice(), [n] if n.contains("path: [Crossing {")
                && n.contains("Cap(End)")
                && n.contains("sense: Leaves")
                && !n.contains("Seam {")),
            "{members:?}: the pinch is the contact's crossing of the plate's top: {names:?}"
        );
        match &first {
            None => first = Some(names),
            Some(f) => assert_eq!(&names, f, "{members:?}"),
        }
    }
}

/// **Two wedges' legs meeting on the plate's top are their touch in
/// every member order** (N2): the legs locate the point, so the plate's
/// top, which passes through it, is not in the name, and each leg's sense
/// is read against the other wedge alone, from the pair's judgement.
/// Each lies outside the other on both sides.
#[test]
fn two_legs_on_the_plate_are_their_touch_in_every_member_order() {
    let mut first: Option<String> = None;
    for order in orders(3) {
        let (doc, m) = tilted_holes(
            ProfileDoc::empty_derived("union_pinch", Tol::witness()),
            &two_wedges(),
        );
        let members: Vec<RecipeNodeId> = order.iter().map(|&i| m[i]).collect();
        let (doc, u) = crate::fixture::union_over(doc, &members);
        let ev = run(&doc);
        if let Some(e) = failure(&ev, u) {
            panic!("member order {order:?}: refused: {e:?}");
        }
        let names = names_at(&ev, u, top());
        let [name] = names.as_slice() else {
            panic!("member order {order:?}: one name at the meeting point: {names:?}");
        };
        assert!(
            name.contains("path: [EdgeCrossing(")
                && name.matches("role: Leg").count() == 2
                && run_senses(name) == ["Some(Outside)"; 2],
            "member order {order:?}: the legs' touch from outside: {name}"
        );
        match &first {
            None => first = Some(name.clone()),
            Some(f) => assert_eq!(name, f, "member order {order:?}: the meeting point's name"),
        }
    }
}

/// **The plate less the wedges' union, and the wedges' union less the
/// plate, carry the union's name for the point where the legs meet alike
/// in every member order of the union**
/// (`work/wire/a-pinch-vertex-is-named-by-the-fold-step-that-mints-it.md`):
/// the union's run is order-free, so what a subtract carries of it is
/// too. Where a subtract holds the union's vertex as several vertices,
/// the set of their names is compared. Three and four wedges, every
/// order: four refused P − U in member order `[3, 2, 1, 0]` while a
/// split per cone's copy of the union's vertex was read as no operand
/// vertex (`BooleanNaming::cone_splits`). The union's and the
/// subtracts' own ids, which the member order moves, are masked.
#[test]
fn a_subtract_against_the_wedges_union_names_their_meeting_alike_in_every_member_order() {
    for (label, holes) in [
        ("three wedges", three_wedges as fn() -> Vec<Hole>),
        ("four wedges", four_wedges),
    ] {
        let n = holes().len();
        let mut first: Option<[Vec<String>; 2]> = None;
        for order in orders(n) {
            let (doc, m) = tilted_holes(
                ProfileDoc::empty_derived("union_pinch", Tol::witness()),
                &holes(),
            );
            let plate = m[0];
            let wedges: Vec<RecipeNodeId> = order.iter().map(|&i| m[i + 1]).collect();
            let (doc, u) = crate::fixture::union_over(doc, &wedges);
            let (doc, p_u) = insert(doc, pair_node(BooleanOp::Subtract, plate, u));
            let (doc, u_p) = insert(doc, pair_node(BooleanOp::Subtract, u, plate));
            let ev = run(&doc);
            let mut masks: Vec<(String, &str)> = Vec::new();
            for (id, tag) in [(u, "U"), (p_u, "P-U"), (u_p, "U-P")] {
                masks.push((format!("{id:?}"), tag));
                masks.push((format!("{:?}", crate::fixture::out(&doc, id)), tag));
            }
            let named = |id: RecipeNodeId, what: &str| -> Vec<String> {
                if let Some(e) = failure(&ev, id) {
                    panic!("{label} {what}, member order {order:?}: refused: {e:?}");
                }
                names_at(&ev, id, top())
                    .into_iter()
                    .map(|n| {
                        masks
                            .iter()
                            .fold(n, |n, (id, tag)| n.replace(id.as_str(), tag))
                    })
                    .collect()
            };
            // The names a subtract carries from the union's vertex; its
            // own seam junctions there embed the union's face pieces,
            // whose `Borders` sets depend on member order
            // (`work/emit/a-merged-face-pieces-borders-in-a-union-depend-on-member-order.md`).
            let carried = |names: Vec<String>| -> Vec<String> {
                names
                    .into_iter()
                    .filter(|n| n.contains("path: [From { read: U,"))
                    .collect()
            };
            let both = [carried(named(p_u, "P − U")), carried(named(u_p, "U − P"))];
            for (names, what) in both.iter().zip(["P − U", "U − P"]) {
                assert!(
                    !names.is_empty(),
                    "{label} {what}, member order {order:?}: no name carried from the union's vertex"
                );
            }
            match &first {
                None => first = Some(both),
                Some(f) => assert_eq!(&both, f, "{label}, member order {order:?}"),
            }
        }
    }
}
