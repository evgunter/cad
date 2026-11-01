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
//! - a third prism bridging the gap between two wedges that touch along
//!   the line, with the plate and alone: every order that builds
//!   publishes one table;
//! - the plate against the joined blocks as a pair boolean, both ways
//!   round, so the pierced face sits on each operand side in turn;
//! - the plate intersected with the joined blocks, where the two
//!   pierces stay in two faces and so stay two vertices;
//! - the plate minus the joined blocks, and a slab less them, which
//!   refuse `LineWedges`: the blocks' contact runs inside the other
//!   operand, where the complement pairs one block's faces with the
//!   other's, and the result's two edges there overlap.
//!
//! Each row also reads the result's contact record, its tier-3′ verdict
//! and `check_mesh` on its tessellation: every result passes 3′, every
//! order of one union publishes one record, and every mesh passes.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::BTreeMap;

use crate::blocks::{Spans, block, block_of, block_on, block_on_of, failure, run};
use crate::corpus::body_of;
use crate::fixture::{ends, face_vertices, insert, len, point};
use editor_core::{
    BooleanValue, EntityKind, Evaluation, Node, ProfileDoc, Qualifier, RecipeNodeId, RoleSeg,
    StableName, ValuePayload,
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
pub(crate) fn pinch(doc: ProfileDoc) -> (ProfileDoc, Vec<RecipeNodeId>) {
    let (doc, plate) = block_of(doc, PLATE_SPANS);
    let (doc, p1) = block_on_of(doc, plate, P1);
    let (doc, p2) = block_on_of(doc, plate, P2);
    (doc, vec![plate, p1, p2])
}

/// The plate under [`pinch`]'s blocks and the sector prisms.
pub(crate) const PLATE_SPANS: Spans = ((0.0, 3.0), (0.0, 2.0), 0.0, 1.0);

/// [`pinch`]'s first block.
pub(crate) const P1: Spans = ((1.0, 1.5), (-1.0, 1.0), 0.5, 1.5);

/// [`pinch`]'s second block, meeting [`P1`]'s footprint at (1.5, 1).
pub(crate) const P2: Spans = ((1.5, 2.0), (1.0, 3.0), 0.47, 1.23);

/// A [`sector_prism`]'s bearings in degrees and its z span.
pub(crate) type Sector = (f64, f64, (f64, f64));

/// [`crossing_one_wedge`]'s wedges and the prism crossing the first.
pub(crate) const WEDGE_A: Sector = (0.0, 50.0, (0.5, 2.0));
pub(crate) const WEDGE_B: Sector = (120.0, 170.0, (0.47, 1.7));
pub(crate) const CROSSING_A: Sector = (30.0, 60.0, (0.44, 1.81));

/// [`sector_prism`] over `sector`, its copy standing in the world.
pub(crate) fn sector_in_world(doc: ProfileDoc, (a0, a1, z): Sector) -> (ProfileDoc, RecipeNodeId) {
    sector_prism(doc, Stand::World, a0, a1, z)
}

/// [`sector_prism`] over `sector`, placed against `plate`.
pub(crate) fn sector_of(
    doc: ProfileDoc,
    plate: RecipeNodeId,
    (a0, a1, z): Sector,
) -> (ProfileDoc, RecipeNodeId) {
    sector_prism(doc, Stand::On(plate), a0, a1, z)
}

/// The two blocks run through the plate: a pinch on its top and one on
/// its bottom.
pub(crate) fn through(doc: ProfileDoc) -> (ProfileDoc, Vec<RecipeNodeId>) {
    let (doc, plate) = block(doc, (0.0, 3.0), (0.0, 2.0), 1.0);
    let (doc, p1) = block_on(doc, plate, (1.0, 1.5), (-1.0, 1.0), -0.5, 2.5);
    let (doc, p2) = block_on(doc, plate, (1.5, 2.0), (1.0, 3.0), -0.47, 2.17);
    (doc, vec![plate, p1, p2])
}

/// [`pinch`] and a third block whose footprint meets `p1`'s at (1, 1, 1):
/// two pinches on the top.
pub(crate) fn two_pinches(doc: ProfileDoc) -> (ProfileDoc, Vec<RecipeNodeId>) {
    let (doc, mut m) = pinch(doc);
    let (doc, p3) = block_on(doc, m[0], (0.5, 1.0), (1.0, 3.0), 0.6, 0.9);
    m.push(p3);
    (doc, m)
}

/// The plate and two blocks through its x = 3 side whose footprints there
/// are holes meeting at (3, 1, 0.5).
pub(crate) fn side_pinch(doc: ProfileDoc) -> (ProfileDoc, Vec<RecipeNodeId>) {
    let (doc, plate) = block(doc, (0.0, 3.0), (0.0, 2.0), 1.0);
    let (doc, p1) = block_on(doc, plate, (2.5, 4.0), (0.5, 1.0), 0.2, 0.3);
    let (doc, p2) = block_on(doc, plate, (2.47, 3.9), (1.0, 1.5), 0.5, 0.3);
    (doc, vec![plate, p1, p2])
}

/// Where a prism's copy stands: placed against a body, its base `z0` up
/// that body's own z, or in the world `z0` up the world's z.
#[derive(Clone, Copy)]
enum Stand {
    On(RecipeNodeId),
    World,
}

/// A prism over the counterclockwise `corners` on its own xy, `dz`
/// high, its copy standing from `z0` as `at` says: the placement.
fn prism(
    doc: ProfileDoc,
    at: Stand,
    corners: &[(f64, f64)],
    z0: f64,
    dz: f64,
) -> (ProfileDoc, RecipeNodeId) {
    let (doc, p) = insert(
        doc,
        Node::Profile(crate::fixture::desc(vec![corners.to_vec()])),
    );
    let (doc, body) = insert(
        doc,
        Node::Extrude {
            profile: p.into(),
            distance: len(dz),
        },
    );
    match at {
        Stand::On(target) => crate::fixture::slid(doc, target, body, [0.0, 0.0, z0]),
        Stand::World => {
            crate::fixture::in_world(doc, body, &editor_core::Frame::translation([0.0, 0.0, z0]))
        }
    }
}

/// The plate and two blocks whose footprints on its top are holes
/// meeting at the corner (1.5, 1): the blocks of [`pinch`], cut short of
/// the plate's sides.
pub(crate) fn corner_holes(doc: ProfileDoc) -> (ProfileDoc, Vec<RecipeNodeId>) {
    let (doc, plate) = block(doc, (0.0, 3.0), (0.0, 2.0), 1.0);
    let (doc, p1) = block_on(doc, plate, (1.0, 1.5), (0.5, 1.0), 0.5, 1.5);
    let (doc, p2) = block_on(doc, plate, (1.5, 2.0), (1.0, 1.5), 0.47, 1.23);
    (doc, vec![plate, p1, p2])
}

/// [`corner_holes`] with `p1` through the plate's y = 0 side: a notch
/// and a hole meeting at the corner.
pub(crate) fn notch_and_hole(doc: ProfileDoc) -> (ProfileDoc, Vec<RecipeNodeId>) {
    let (doc, plate) = block(doc, (0.0, 3.0), (0.0, 2.0), 1.0);
    let (doc, p1) = block_on(doc, plate, (1.0, 1.5), (-1.0, 1.0), 0.5, 1.5);
    let (doc, p2) = block_on(doc, plate, (1.5, 2.0), (1.0, 1.5), 0.47, 1.23);
    (doc, vec![plate, p1, p2])
}

/// The triangle of [`reflex_hole`]'s first block, every edge leaving the
/// pinch inside the L's missing quadrant.
const WEDGE: [(f64, f64); 3] = [(1.5, 1.0), (1.0, 0.6), (1.2, 0.5)];

/// The plate, a wedge-footprint block and an L-footprint block, holes
/// in its top meeting at (1.5, 1): the L's reflex corner, an interior
/// angle of 3π/2, with the wedge inside the quadrant the L leaves.
pub(crate) fn reflex_hole(doc: ProfileDoc) -> (ProfileDoc, Vec<RecipeNodeId>) {
    let (doc, plate) = block(doc, (0.0, 3.0), (0.0, 2.0), 1.0);
    let (doc, wedge) = prism(doc, Stand::On(plate), &WEDGE, 0.5, 1.5);
    let ell = [
        (1.0, 1.0),
        (1.5, 1.0),
        (1.5, 0.5),
        (2.0, 0.5),
        (2.0, 1.5),
        (1.0, 1.5),
    ];
    let (doc, l) = prism(doc, Stand::On(plate), &ell, 0.47, 1.23);
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
    if let Err(e) = topo::validate_geometric(&body, Tol::witness()) {
        panic!("{what}: tier 3 refused the body: {e:?}");
    }
    let mesh = mesh::tessellate(&body, 1e-3, Tol::witness())
        .unwrap_or_else(|e| panic!("{what}: tessellation refused: {e:?}"));
    let v = mesh::validate::signed_volume(&mesh);
    assert!(
        (v - volume).abs() < 1e-9,
        "{what}: volume {v}, closed form {volume}"
    );
    let none = ContactRecords::default();
    let value = ev.value(id).expect("node evaluated to a value");
    let contacts = match &value.payload {
        ValuePayload::Boolean(BooleanValue::Body { .. }) => &*value.carry.contacts,
        _ => &none,
    };
    let verdict = topo::validate_pseudomanifold(&body, contacts, Tol::witness()).map_err(|es| {
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
    if let Err(e) = topo::test_support::meeting::corners_disjoint(&body, Tol::witness()) {
        panic!("{what}: {e}");
    }
    Outcome {
        shape: shape(&body),
        record: record(&body, contacts),
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
    let value = ev.value(id).expect("the node evaluated");
    let contacts = match &value.payload {
        ValuePayload::Boolean(BooleanValue::Body { .. }) => &*value.carry.contacts,
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

/// **Every member order of a pinch union publishes one table**, the
/// vertex at each pinch under one name: for the top pinch, for blocks
/// through the plate, for the two holes meeting at a corner, and for a
/// third block making a second pinch, over every order of its four
/// members. Red if a pinch vertex keeps the name of whichever block
/// reached it last.
#[test]
fn a_pinch_union_publishes_one_table_in_every_member_order() {
    type Fixture = fn(ProfileDoc) -> (ProfileDoc, Vec<RecipeNodeId>);
    let rows: [(&str, Fixture, &[Point]); 4] = [
        ("top", pinch, &[top()]),
        ("through", through, &[top(), (1_500_000, 1_000_000, 0)]),
        ("corner holes", corner_holes, &[top()]),
        (
            "two pinches on the top",
            two_pinches,
            &[top(), (1_000_000, 1_000_000, 1_000_000)],
        ),
    ];
    for (label, fixture, pinches) in rows {
        let mut first: Option<NamedShape> = None;
        let n = fixture(ProfileDoc::empty_derived("union_pinch", Tol::witness()))
            .1
            .len();
        for order in orders(n) {
            let (doc, m) = fixture(ProfileDoc::empty_derived("union_pinch", Tol::witness()));
            let members: Vec<RecipeNodeId> = order.iter().map(|&i| m[i]).collect();
            let (doc, u) = crate::fixture::union_over(doc, &members);
            let what = format!("{label}, member order {order:?} (0 = plate)");
            let ev = run(&doc);
            if let Some(e) = failure(&ev, u) {
                panic!("{what}: refused: {e:?}");
            }
            for &p in pinches {
                let names = names_at(&ev, u, p);
                assert_eq!(
                    names.len(),
                    1,
                    "{what}: one name at the pinch {p:?}: {names:?}"
                );
            }
            let here = named_shape(&ev, u);
            match &first {
                None => first = Some(here),
                Some(f) => same_table(&here, f, &what),
            }
        }
    }
}

/// **The plate against the joined blocks: a union both ways round
/// builds the member-order body, an intersection keeps the two
/// footprints apart, and a subtraction refuses `LineWedges`**: the
/// blocks' contact runs inside the plate, and ∖ does not build the
/// re-pairing its complement needs, so the result's two edges there
/// bound 270° each, overlapping.
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

    // The blocks' contact runs inside the plate, where the complement
    // pairs one block's faces with the other's; the reverted pair would
    // bound 270° each, overlapping.
    let refusal = failure(&ev, notched);
    assert!(
        format!("{refusal:?}").contains("LineWedges { fault: Overlap"),
        "plate ∖ blocks: {refusal:?}, not the overlap's refusal"
    );

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

/// **The slab the two blocks are cut from refuses**: `slab ∖ blocks`
/// keeps the blocks' contact inside the slab, where the complement pairs
/// one block's faces with the other's. The result would carry the
/// contact as two coincident edges bounding 270° each, overlapping, and
/// the result gate refuses `LineWedges`. The six ops of that slab against the plate return
/// with the re-pairing
/// (`work/topo/revert-re-pairs-the-sheets-on-a-line-where-coincident-edges-meet.md`).
#[test]
fn a_slab_holding_the_contact_refuses_to_be_cut() {
    let doc = ProfileDoc::empty_derived("union_pinch", Tol::witness());
    let (doc, slab) = block(doc, (-1.0, 4.0), (-1.0, 3.0), 2.5);
    // The slab's own z = 0 is z = 0.5 where the blocks stand.
    let (doc, p1) = block_on(doc, slab, (1.0, 1.5), (-2.0, 1.0), 0.3 - 0.5, 2.0);
    let (doc, p2) = block_on(doc, slab, (1.5, 2.0), (1.0, 4.0), 0.27 - 0.5, 1.73);
    let (doc, blocks) = crate::fixture::union_over(doc, &[p1, p2]);
    let (doc, x) = insert(doc, pair_node(BooleanOp::Subtract, slab, blocks));
    let ev = run(&doc);
    let refusal = failure(&ev, x);
    assert!(
        format!("{refusal:?}").contains("LineWedges { fault: Overlap"),
        "slab ∖ blocks: {refusal:?}, not the overlap's refusal"
    );
}

use topo::test_support::meeting::{
    Hole, MEET, PLATE, ell_and_wedges, four_wedges, three_wedges, two_wedges, wedges_on_one_side,
};

/// The plate and a prism per hole, each extruded along its own z and
/// placed against the plate on its tilted frame: above the top the
/// prisms lean apart and touch nowhere, below it they cross inside the
/// plate.
pub(crate) fn tilted_holes(doc: ProfileDoc, holes: &[Hole]) -> (ProfileDoc, Vec<RecipeNodeId>) {
    assert_eq!(PLATE[2].0, 0.0, "the plate's base is its own xy");
    let (mut doc, plate) = block(doc, PLATE[0], PLATE[1], PLATE[2].1);
    let mut m = vec![plate];
    for h in holes {
        let [o, u, v, _] = h.frame();
        let (d, p) = insert(doc, Node::Profile(crate::fixture::desc(vec![h.profile()])));
        let (d, w) = insert(
            d,
            Node::Extrude {
                profile: p.into(),
                distance: len(h.length),
            },
        );
        let (d, w) = crate::fixture::against(d, plate, w, &crate::fixture::axes_at(o, u, v));
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
                .any(|k| matches!(k, editor_core::EntityKey::Vertex(v) if at_point(&body, *v) == p))
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
            vec![sense.to_owned(); holes().len()],
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
            n.contains("path: [EdgeCrossing(") && run_senses(n) == ["Outside"; 2],
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
fn sector_prism(
    doc: ProfileDoc,
    at: Stand,
    a0: f64,
    a1: f64,
    z: (f64, f64),
) -> (ProfileDoc, RecipeNodeId) {
    let stand = at;
    let on = |a: f64| {
        let (s, c) = f64::to_radians(a).sin_cos();
        (0.4f64.mul_add(c, MEET[0]), 0.4f64.mul_add(s, MEET[1]))
    };
    prism(
        doc,
        stand,
        &[(MEET[0], MEET[1]), on(a0), on(a1)],
        z.0,
        z.1 - z.0,
    )
}

/// The plate and three prisms over 50° sectors about (1.5, 1), each 70°
/// from the next, touching pairwise along the vertical line through it.
pub(crate) fn three_on_a_line(doc: ProfileDoc) -> (ProfileDoc, Vec<RecipeNodeId>) {
    let (doc, plate) = block(doc, (0.0, 3.0), (0.0, 2.0), 1.0);
    let (doc, a) = sector_prism(doc, Stand::On(plate), 0.0, 50.0, (0.5, 2.0));
    let (doc, b) = sector_prism(doc, Stand::On(plate), 120.0, 170.0, (0.47, 1.7));
    let (doc, c) = sector_prism(doc, Stand::On(plate), 240.0, 290.0, (0.44, 1.81));
    (doc, vec![plate, a, b, c])
}

/// The plate and four prisms over 50° sectors about (1.5, 1), each 40°
/// from the next, touching pairwise along the vertical line through it.
fn four_on_a_line(doc: ProfileDoc) -> (ProfileDoc, Vec<RecipeNodeId>) {
    let (doc, plate) = block(doc, (0.0, 3.0), (0.0, 2.0), 1.0);
    let (doc, a) = sector_prism(doc, Stand::On(plate), 0.0, 50.0, (0.5, 2.0));
    let (doc, b) = sector_prism(doc, Stand::On(plate), 90.0, 140.0, (0.47, 1.7));
    let (doc, c) = sector_prism(doc, Stand::On(plate), 180.0, 230.0, (0.44, 1.81));
    let (doc, d) = sector_prism(doc, Stand::On(plate), 270.0, 320.0, (0.41, 1.63));
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
/// one line, the `Merged` set of them, so the pinch is that line's
/// `Crossing` of the plate's top, read `Through` the edges whose
/// direction enters the plate, alike in both orders.
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
                && n.contains("sense: Through")
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
                && run_senses(name) == ["Outside"; 2],
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
/// too. Each subtract names the point by the union's vertex alone, every
/// name it binds there carried from it (N2: a vertex of either operand is
/// that vertex), so nothing of its own there reads the member order.
/// Three and four wedges, every order. P − U refused three wedges in
/// member order `[2, 1, 0]` once it named its own meeting by its lines,
/// and four in `[3, 2, 1, 0]` before that, while the zip's split per cone
/// of the union's vertex was read as no operand vertex
/// (`BooleanNaming::cone_splits`). The union's and the subtracts' own
/// ids, which the member order moves, are masked.
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
            let both = [named(p_u, "P − U"), named(u_p, "U − P")];
            for (names, what) in both.iter().zip(["P − U", "U − P"]) {
                assert!(
                    !names.is_empty() && names.iter().all(|n| n.contains("path: [From { read: U,")),
                    "{label} {what}, member order {order:?}: every name at the point is carried \
                     from the union's vertex: {names:?}"
                );
            }
            match &first {
                None => first = Some(both),
                Some(f) => assert_eq!(&both, f, "{label}, member order {order:?}"),
            }
        }
    }
}

/// [`three_on_a_line`] with the third prism over 30°–140°: it bridges
/// the gap between the first two's wedges, crossing both.
pub(crate) fn bridging(doc: ProfileDoc) -> (ProfileDoc, Vec<RecipeNodeId>) {
    let (doc, plate) = block(doc, (0.0, 3.0), (0.0, 2.0), 1.0);
    let (doc, a) = sector_prism(doc, Stand::On(plate), 0.0, 50.0, (0.5, 2.0));
    let (doc, b) = sector_prism(doc, Stand::On(plate), 120.0, 170.0, (0.47, 1.7));
    let (doc, c) = sector_prism(doc, Stand::On(plate), 30.0, 140.0, (0.44, 1.81));
    (doc, vec![plate, a, b, c])
}

/// Where the bridge folds last, after the plate, its edge meets the
/// line's two coincident edges at two points, which topo does not build.
pub(crate) const TWO_GERMS: &str =
    "work/tangtail/an-edge-crossing-two-wedges-about-a-contact-line-is-unbuilt.md";

/// A table, key-free: each row's name by where its entities stand —
/// a vertex at its point, an edge at its ends, a face at its vertices.
type NamedShape = BTreeMap<String, Vec<Vec<Point>>>;

/// `id`'s table as a [`NamedShape`].
fn named_shape(ev: &Evaluation<f64>, id: RecipeNodeId) -> NamedShape {
    let body = body_of(ev, id);
    let at = |k: &editor_core::EntityKey| -> Vec<Point> {
        let mut ps: Vec<Point> = match *k {
            editor_core::EntityKey::Vertex(v) => vec![at_point(&body, v)],
            editor_core::EntityKey::Edge(e) => ends(&body, e).map(|v| at_point(&body, v)).to_vec(),
            editor_core::EntityKey::Face(f) => face_vertices(&body, f)
                .into_iter()
                .map(|v| at_point(&body, v))
                .collect(),
            _ => Vec::new(),
        };
        ps.sort_unstable();
        ps
    };
    crate::fixture::table(ev, id)
        .iter()
        .map(|(n, e)| {
            let mut where_: Vec<Vec<Point>> = match e {
                editor_core::Entry::Unique(r) => vec![at(&r.key)],
                editor_core::Entry::Tied(rs) => rs.iter().map(|r| at(&r.key)).collect(),
            };
            where_.sort_unstable();
            (n.to_string(), where_)
        })
        .collect()
}

/// Asserts `here` is `first`, row by row, listing every row only one
/// of them holds.
fn same_table(here: &NamedShape, first: &NamedShape, what: &str) {
    let differ: Vec<String> = here
        .iter()
        .filter(|(n, w)| first.get(*n) != Some(*w))
        .map(|(n, w)| format!("here only: {n} at {w:?}"))
        .chain(
            first
                .iter()
                .filter(|(n, w)| here.get(*n) != Some(*w))
                .map(|(n, w)| format!("first only: {n} at {w:?}")),
        )
        .collect();
    assert!(
        differ.is_empty(),
        "{what}: a different table from the first order:\n{}",
        differ.join("\n")
    );
}

/// The wedges at 0°–50° and 120°–170° about the line through (1.5, 1),
/// and two bridges over 30°–140°, each filling the gap between the
/// wedges over a stretch of the line: `d` from z 0.2 to 0.4 and `c` over
/// `c_z`.
fn two_bridges(doc: ProfileDoc, c_z: (f64, f64)) -> (ProfileDoc, Vec<RecipeNodeId>) {
    let (doc, a) = sector_prism(doc, Stand::World, 0.0, 50.0, (0.12, 2.0));
    let (doc, b) = sector_prism(doc, Stand::World, 120.0, 170.0, (0.1, 1.7));
    let (doc, d) = sector_prism(doc, Stand::World, 30.0, 140.0, (0.2, 0.4));
    let (doc, c) = sector_prism(doc, Stand::World, 30.0, 140.0, c_z);
    (doc, vec![a, b, d, c])
}

/// [`two_bridges`] with `c` from z 0.6 to 1.5.
pub(crate) fn two_bridges_short(doc: ProfileDoc) -> (ProfileDoc, Vec<RecipeNodeId>) {
    two_bridges(doc, (0.6, 1.5))
}

/// [`two_bridges`] with `c` from z 0.6 to 1.81, past the second wedge.
fn two_bridges_tall(doc: ProfileDoc) -> (ProfileDoc, Vec<RecipeNodeId>) {
    two_bridges(doc, (0.6, 1.81))
}

/// One gap a bridge fills between two wedges' walls along their line.
struct Gap {
    label: &'static str,
    fixture: fn(ProfileDoc) -> (ProfileDoc, Vec<RecipeNodeId>),
    /// The member orders folded, as indices into the fixture's list.
    orders: Vec<Vec<usize>>,
    /// Whether an order refuses in topo ([`TWO_GERMS`]).
    unbuilt: fn(&[usize]) -> bool,
    /// The fixture's indices of the two wedges.
    walls: [usize; 2],
    /// The stretches of the line, as z from and to, along which the
    /// wedges' walls meet: one edge each, a seam of those walls.
    seams: &'static [(f64, f64)],
}

/// Asserts every order of `gap` that builds is tier-3 valid and names
/// one body with one table, compared row by row by where each row's
/// entities stand, the walls meeting along each of its seams in a
/// `Seam` of the two; and every other refuses [`TWO_GERMS`]. Returns
/// how many built.
fn named_alike(gap: &Gap) -> usize {
    let mut first: Option<(Shape, NamedShape)> = None;
    let mut built = 0;
    for order in &gap.orders {
        let (doc, m) = (gap.fixture)(ProfileDoc::empty_derived("bridge", Tol::witness()));
        let members: Vec<RecipeNodeId> = order.iter().map(|&i| m[i]).collect();
        let (doc, u) = crate::fixture::union_over(doc, &members);
        let what = format!("{}, member order {order:?}", gap.label);
        let ev = run(&doc);
        let unbuilt = (gap.unbuilt)(order);
        match failure(&ev, u) {
            Some(e) if unbuilt => {
                let e = format!("{e:?}");
                assert!(
                    e.contains("CrossingsAlongAContactLine"),
                    "{what}: refused other than {TWO_GERMS}: {e}"
                );
                continue;
            }
            Some(e) => panic!("{what}: refused: {e:?}"),
            None => assert!(!unbuilt, "{what}: built ({TWO_GERMS} has landed)"),
        }
        built += 1;
        let body = body_of(&ev, u);
        if let Err(e) = topo::validate_geometric(&body, Tol::witness()) {
            panic!("{what}: tier 3 refused the body: {e:?}");
        }
        let mut walls = gap
            .walls
            .map(|i| doc.output(m[i], 0).expect("a member's body"));
        walls.sort_unstable();
        for &(lo, hi) in gap.seams {
            let on: Vec<&StableName> = crate::fixture::table(&ev, u)
                .iter()
                .filter_map(|(n, e)| match e {
                    editor_core::Entry::Unique(r) => {
                        let editor_core::EntityKey::Edge(k) = r.key else {
                            return None;
                        };
                        let mut ps = ends(&body, k).map(|v| at_point(&body, v));
                        ps.sort_unstable();
                        (ps == [micron(MEET[0], MEET[1], lo), micron(MEET[0], MEET[1], hi)])
                            .then_some(n)
                    }
                    editor_core::Entry::Tied(_) => None,
                })
                .collect();
            let [seam] = on[..] else {
                panic!("{what}: one named edge on the line from z {lo} to {hi}: {on:?}")
            };
            let Some(RoleSeg::Seam { a, b }) = seam.path.first() else {
                panic!("{what}: the edge on the line from z {lo} to {hi}: {seam:?}")
            };
            let mut sides = [side_read(a.name()), side_read(b.name())];
            sides.sort_unstable();
            assert_eq!(
                sides, walls,
                "{what}: the seam from z {lo} to {hi} lies between the wedges' walls: {seam:?}"
            );
        }
        let here = (shape(&body), named_shape(&ev, u));
        if let Some(f) = &first {
            assert_eq!(here.0, f.0, "{what}: a different body from the first order");
            same_table(&here.1, &f.1, &what);
        } else {
            first = Some(here);
        }
    }
    built
}

/// [`bridging`] with the plate or without it (`plate`), in `orders`.
fn bridging_gap(label: &'static str, plate: bool, orders: Vec<Vec<usize>>) -> Gap {
    Gap {
        label,
        fixture: bridging,
        orders,
        unbuilt: if plate { |o| o[3] == 3 } else { |_| false },
        walls: [1, 2],
        seams: if plate { &[(1.0, 1.7)] } else { &[(0.5, 1.7)] },
    }
}

/// [`two_bridges`]'s gaps: `c` from z 0.6 to 1.5 or 1.81, in `orders`.
fn two_bridges_gaps(orders: &[Vec<usize>]) -> [Gap; 2] {
    [
        Gap {
            label: "two bridges, the upper to z 1.5",
            fixture: two_bridges_short,
            orders: orders.to_vec(),
            unbuilt: |_| false,
            walls: [0, 1],
            seams: &[(0.2, 0.4), (0.6, 1.5)],
        },
        Gap {
            label: "two bridges, the upper to z 1.81",
            fixture: two_bridges_tall,
            orders: orders.to_vec(),
            unbuilt: |_| false,
            walls: [0, 1],
            seams: &[(0.2, 0.4), (0.6, 1.7)],
        },
    ]
}

/// **A prism bridging two wedges that touch along a line is named
/// alike in the orders that fold it after both**, against one that
/// folds it first. Folded after the two wedges, the bridge fills the gap
/// between them and is consumed there, so the first wedge's 0° wall
/// meets the second's 170° wall along the line: two faces of one fold
/// operand that share no edge in it, or (a second bridge folded before)
/// share one elsewhere on the line. That edge is their seam, as it is
/// where the bridge comes before either wedge. The fast subset of
/// [`a_prism_bridging_two_wedges_is_named_alike_in_every_member_order_that_builds`].
#[test]
fn a_bridge_folded_after_both_wedges_seams_their_walls() {
    assert_eq!(
        named_alike(&bridging_gap(
            "the bridge alone",
            false,
            vec![vec![3, 2, 1], vec![2, 1, 3], vec![1, 2, 3]],
        )),
        3,
        "the bridge alone: orders that build"
    );
    // Folded bridge, plate, wedges: the bridge's chord wall loses a
    // notch to the second wedge at the last step, across a stretch of
    // the plate's seam the first wedge split, and borders its walls as
    // in every other order.
    assert_eq!(
        named_alike(&bridging_gap(
            "the bridge with the plate",
            true,
            vec![vec![3, 2, 1, 0], vec![3, 0, 1, 2]],
        )),
        2,
        "the bridge with the plate: orders that build"
    );
    for gap in two_bridges_gaps(&[vec![3, 2, 1, 0], vec![0, 1, 2, 3], vec![1, 0, 2, 3]]) {
        assert_eq!(named_alike(&gap), 3, "{}: orders that build", gap.label);
    }
}

/// **A prism bridging two wedges that touch along a line is named
/// alike in every member order that builds**, with the plate and
/// alone, and so are two bridges over two stretches of the line. With
/// the plate, the 6 orders that fold the bridge last refuse in topo
/// ([`TWO_GERMS`]); every other order builds one body and one table.
#[test]
fn a_prism_bridging_two_wedges_is_named_alike_in_every_member_order_that_builds() {
    for (label, plate, builds) in [("with the plate", true, 18), ("alone", false, 6)] {
        let gap = bridging_gap(label, plate, every_order_of(4, plate));
        assert_eq!(named_alike(&gap), builds, "{label}: orders that build");
    }
    for gap in two_bridges_gaps(&orders(4)) {
        assert_eq!(named_alike(&gap), 24, "{}: orders that build", gap.label);
    }
}

/// **A low bridge met with the wedges and the upper bridge holds each
/// of its corners on the line as two copies**, one in each wedge, told
/// apart by the wedge wall each meets (N2's corner held as several):
/// `d` from z 0.2 to 0.4 against `a ∪ b ∪ c`, in both operand orders
/// (`name_invariance` compares the two tables).
#[test]
fn a_bridge_met_with_the_wedges_holds_each_corner_on_the_line_as_two_copies() {
    for swapped in [false, true] {
        let (doc, m) = two_bridges_short(ProfileDoc::empty_derived("bridge", Tol::witness()));
        let [a, b, d, c] = m[..] else {
            unreachable!("two bridges are four members")
        };
        // A copy carries its source's names, so a member's names are
        // spelled with the prism it copies.
        let minted = |copy: RecipeNodeId| match doc.node(copy) {
            Some(editor_core::Node::Place { source, .. }) => doc
                .operation_of(source.read)
                .expect("a copy reads a construction"),
            _ => copy,
        };
        let (pa, pb, pd) = (minted(a), minted(b), minted(d));
        let (doc, rest) = crate::fixture::union_over(doc, &[a, b, c]);
        let (x, y) = if swapped { (rest, d) } else { (d, rest) };
        let (doc, met) = insert(doc, pair_node(BooleanOp::Intersect, x, y));
        let what = format!("d against the rest, swapped {swapped}");
        let ev = run(&doc);
        if let Some(e) = failure(&ev, met) {
            panic!("{what}: refused: {e:?}");
        }
        let body = body_of(&ev, met);
        let mut copies: BTreeMap<Point, Vec<RecipeNodeId>> = BTreeMap::new();
        for (n, e) in crate::fixture::table(&ev, met).iter() {
            let [
                RoleSeg::From { of, .. },
                RoleSeg::Fragment(Qualifier::Borders(walls)),
            ] = n.path.as_slice()
            else {
                continue;
            };
            if n.kind != EntityKind::Vertex || of.name().node != pd {
                continue;
            }
            let editor_core::Entry::Unique(r) = e else {
                panic!("{what}: a copy of a corner of d is tied: {n:?}")
            };
            let editor_core::EntityKey::Vertex(v) = r.key else {
                panic!("{what}: {n:?} names {:?}", r.key)
            };
            let [wall] = walls.as_slice() else {
                panic!("{what}: a copy of a corner of d borders one wedge wall: {n:?}")
            };
            // The wall's member face, through the rest's own `From`.
            let mut face = wall;
            while let [RoleSeg::From { of, .. }] = face.path.as_slice() {
                face = of.name();
            }
            copies
                .entry(at_point(&body, v))
                .or_default()
                .push(face.node);
        }
        for z in [0.2, 0.4] {
            let at = micron(MEET[0], MEET[1], z);
            let mut walls = copies.remove(&at).unwrap_or_default();
            walls.sort_unstable();
            let mut wedges = [pa, pb];
            wedges.sort_unstable();
            assert_eq!(
                walls, wedges,
                "{what}: d's corner at z {z} is two copies, one on each wedge's wall"
            );
        }
        assert!(copies.is_empty(), "{what}: copies off the line: {copies:?}");
    }
}

/// The member read a union's `From` side came in through.
fn side_read(side: &StableName) -> editor_core::VarId {
    match side.path.as_slice() {
        [RoleSeg::From { read, .. }, ..] => *read,
        _ => panic!("a union seam side cites a member face: {side:?}"),
    }
}

/// [`three_on_a_line`] with the third prism over 30°–60°: it crosses the
/// first's wedge about the line and touches the second's edge along it.
pub(crate) fn crossing_one_wedge(doc: ProfileDoc) -> (ProfileDoc, Vec<RecipeNodeId>) {
    let (doc, plate) = block_of(doc, PLATE_SPANS);
    let (doc, a) = sector_of(doc, plate, WEDGE_A);
    let (doc, b) = sector_of(doc, plate, WEDGE_B);
    let (doc, c) = sector_of(doc, plate, CROSSING_A);
    (doc, vec![plate, a, b, c])
}

/// [`crossing_one_wedge`] without the plate.
pub(crate) fn crossing_one_wedge_prisms(doc: ProfileDoc) -> (ProfileDoc, Vec<RecipeNodeId>) {
    let (doc, m) = crossing_one_wedge(doc);
    (doc, m[1..].to_vec())
}

/// **A prism crossing one wedge about the contact line publishes one
/// table in every member order** (N2, N4), with the plate and alone. The
/// first wedge's bottom apex lies on the crossing prism's edge along the
/// line, beside the second wedge's coincident edge there: it is that
/// wedge's own start-cap vertex, cited once, never a crossing of the
/// edge it lies on.
#[test]
fn a_prism_crossing_one_wedge_about_the_line_publishes_one_table_in_every_member_order() {
    type Fixture = fn(ProfileDoc) -> (ProfileDoc, Vec<RecipeNodeId>);
    let rows: [(&str, Fixture, bool); 2] = [
        ("with the plate", crossing_one_wedge, false),
        ("prisms alone", crossing_one_wedge_prisms, true),
    ];
    let apex = (1_500_000, 1_000_000, 500_000);
    for (label, fixture, apex_kept) in rows {
        let mut first: Option<NamedShape> = None;
        let n = fixture(ProfileDoc::empty_derived("crossing", Tol::witness()))
            .1
            .len();
        for order in orders(n) {
            let (doc, m) = fixture(ProfileDoc::empty_derived("crossing", Tol::witness()));
            let members: Vec<RecipeNodeId> = order.iter().map(|&i| m[i]).collect();
            let (doc, u) = crate::fixture::union_over(doc, &members);
            let what = format!("{label}, member order {order:?}");
            let ev = run(&doc);
            if let Some(e) = failure(&ev, u) {
                panic!("{what}: refused: {e:?}");
            }
            let names = names_at(&ev, u, apex);
            if apex_kept {
                assert!(
                    matches!(names.as_slice(), [n] if n.contains("CapVertex(Start")
                        && !n.contains("Crossing")),
                    "{what}: the apex is the first wedge's start-cap vertex: {names:?}"
                );
            } else {
                assert!(
                    names.is_empty(),
                    "{what}: the plate holds the apex: {names:?}"
                );
            }
            let here = named_shape(&ev, u);
            match &first {
                None => first = Some(here),
                Some(f) => same_table(&here, f, &what),
            }
        }
    }
}

/// Two boxes whose union has a concave seam along the z axis, and a
/// prism whose diagonal face holds that seam over its height.
fn pencil(doc: ProfileDoc) -> (ProfileDoc, Vec<RecipeNodeId>) {
    let (doc, a) = block(doc, (-1.0, 1.0), (-1.0, 0.0), 2.0);
    let (doc, b) = block_on(doc, a, (0.0, 1.0), (-1.0, 1.0), 0.3, 1.4);
    let (doc, c) = prism(
        doc,
        Stand::On(a),
        &[(-0.5, -0.5), (0.5, -0.5), (0.5, 0.5)],
        0.6,
        0.8,
    );
    (doc, vec![a, b, c])
}

/// **A face through a seam is decided by the step that meets it, as a
/// meeting**: folding the prism into the boxes' union decides the seam
/// crossing each of the prism's two diagonal rim edges, one point each.
/// Read back to the step's inputs, each crossing is the seam edge and a
/// rim edge sharing that point (`Relation::Meet`), never the one edge on
/// the other. The union publishes each as the fold step's row of the
/// member cells that meet there: the two boxes' faces the seam runs
/// between and the prism's rim edge. (The boxes' own step logs its
/// meetings of their edges too, apart from the judged rows stating them:
/// a meeting of two edges names no vertex that would put the two at one
/// point.) Red if a crossing is published as an edge on an edge, or
/// dropped.
#[test]
fn a_face_through_a_seam_is_the_meeting_the_step_that_meets_it_decides() {
    for order in [[0, 1, 2], [1, 0, 2]] {
        let (doc, m) = pencil(ProfileDoc::empty_derived("pencil", Tol::witness()));
        let ev = run(&doc);
        let operand = |i: usize| {
            sweep::test_support::finished("member", body_of(&ev, m[i]).clone(), Tol::witness())
        };
        let joined = match topo::union(&operand(order[0]), &operand(order[1]), Tol::witness()) {
            Ok(topo::BooleanResult::Body(out)) => out.body,
            other => panic!("{order:?}: the boxes join: {other:?}"),
        };
        let Ok(topo::BooleanResult::Body(out)) =
            topo::union(&joined, &operand(order[2]), Tol::witness())
        else {
            panic!("{order:?}: the prism folds in");
        };
        let [first, second] = out.coincidences.as_slice() else {
            panic!(
                "{order:?}: the step states both crossings: {:?}",
                out.coincidences
            );
        };
        for r in [first, second] {
            assert!(
                r.site == topo::DecisionSite::VertexFusion
                    && r.relation == topo::Relation::Meet
                    && matches!(
                        r.cells,
                        [
                            topo::RowCell::Input {
                                input: topo::Operand::A,
                                cell: topo::Cell::Edge(_),
                            },
                            topo::RowCell::Input {
                                input: topo::Operand::B,
                                cell: topo::Cell::Edge(_),
                            },
                        ]
                    ),
                "{order:?}: the seam edge meeting a rim edge: {:?}",
                out.coincidences
            );
        }
        assert!(
            first.cells != second.cells,
            "{order:?}: two crossings: {:?}",
            out.coincidences
        );

        let members: Vec<RecipeNodeId> = order.iter().map(|&i| m[i]).collect();
        let (doc, u) = crate::fixture::union_over(doc, &members);
        let ev = run(&doc);
        assert!(failure(&ev, u).is_none(), "{order:?}: the union builds");
        let read = |i: usize| doc.output(m[i], 0).expect("a member's read");
        let fold: Vec<String> = ev
            .value(u)
            .expect("the union evaluates")
            .coincidences
            .iter()
            .filter(|r| matches!(r.origin, editor_core::RowOrigin::Fold { step: 2 }))
            .map(|r| {
                let cells: Vec<String> = r
                    .cells
                    .iter()
                    .map(|c| match c {
                        editor_core::NamedCell::Entity { input, name } => {
                            let at = (0..3).find(|&i| read(i) == *input).map_or(9, |i| i);
                            format!("{:?}@{at}", name.kind)
                        }
                        other => format!("{other:?}"),
                    })
                    .collect();
                format!("{:?} {:?} {:?} {cells:?}", r.origin, r.site, r.relation)
            })
            .collect();
        let want = format!(
            "Fold {{ step: 2 }} VertexFusion Meet {:?}",
            ["Face@0", "Face@1", "Edge@2"]
        );
        assert_eq!(
            fold,
            [want.clone(), want],
            "{order:?}: the step's two rows, the boxes' faces and the prism's rim edge"
        );
    }
}

/// **Two wedges overlapping about one line publish their apex rows**:
/// each wedge's apex corner lies on the other's apex edge, and the
/// two-member union records both, in either list order.
#[test]
fn two_wedges_overlapping_about_one_line_publish_their_apex_rows() {
    for order in [[1, 3], [3, 1]] {
        let (doc, m) = crossing_one_wedge(ProfileDoc::empty_derived("wedges", Tol::witness()));
        let (doc, u) = crate::fixture::union_over(doc, &[m[order[0]], m[order[1]]]);
        let ev = run(&doc);
        if let Some(e) = failure(&ev, u) {
            panic!("{order:?}: refused: {e:?}");
        }
        let rows: Vec<String> = ev
            .value(u)
            .expect("the union evaluates")
            .coincidences
            .iter()
            .map(|r| format!("{:?} {:?} {:?}", r.site, r.relation, r.cells))
            .collect();
        assert!(
            rows.len() == 2
                && rows.iter().all(|r| r.starts_with("VertexFusion OnCarrier")
                    && r.contains("CapVertex(")
                    && r.contains("LateralEdge(")),
            "{order:?}: each apex corner on the other's apex edge: {rows:#?}"
        );
    }
}

/// [`crossing_one_wedge`] with a block over `x`, `y` and `z` (its low
/// end and height) covering the junction where the plate's top meets
/// the line.
fn covered_junction(
    doc: ProfileDoc,
    (x, y, z): ((f64, f64), (f64, f64), (f64, f64)),
) -> (ProfileDoc, Vec<RecipeNodeId>) {
    let (doc, mut m) = crossing_one_wedge(doc);
    let (doc, cover) = block_on(doc, m[0], x, y, z.0, z.1);
    m.push(cover);
    (doc, m)
}

/// **A block covering the junction a fold step mints builds in every
/// member order, every row of it member cells that meet.** The orders
/// that fold the two wedges, the plate and the third prism before the
/// block log a row at a point the block's step then swallows, which the
/// finished body does not hold: the row names the member cells that
/// met there, which every member still holds.
#[test]
fn a_block_over_the_wedges_junction_builds_in_every_member_order() {
    let covers = [
        ("a small cover", ((1.4, 1.6), (0.9, 1.1), (0.9, 0.2))),
        ("a wide cover", ((1.2, 1.8), (0.7, 1.3), (0.8, 0.4))),
    ];
    for (label, cover) in covers {
        for order in orders(5) {
            let (doc, m) =
                covered_junction(ProfileDoc::empty_derived("covered", Tol::witness()), cover);
            let members: Vec<RecipeNodeId> = order.iter().map(|&i| m[i]).collect();
            let (doc, u) = crate::fixture::union_over(doc, &members);
            let ev = run(&doc);
            assert!(
                failure(&ev, u).is_none(),
                "{label}, member order {order:?}: refused: {:?}",
                failure(&ev, u)
            );
            crate::union_member_orders_decide_alike::cells_are_held(&ev, u, &order);
        }
    }
}
