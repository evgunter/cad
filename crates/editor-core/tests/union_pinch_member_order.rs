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
//!   one, and are welded only in the second case.
//!
//! Each row also reads the result's contact record, its tier-3′ verdict
//! and `check_mesh` on its tessellation. Two of those diverge where they
//! should agree, each while a filed row stands ([`DROPPED_RECORDS`],
//! [`DOUBLED_EDGE`]), and the rows assert the divergence as it stands.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::BTreeMap;

use crate::corpus::body_of;
use crate::docm7_union_declare::{block, failure, run};
use crate::fixture::{ends, face_vertices, insert, len, on_frame, point};
use editor_core::{
    BooleanOp, BooleanValue, Evaluation, ExtrudeSide, Node, ProfileDoc, RecipeNodeId, ValuePayload,
};
use geom_core::Tol;
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
            profile: p,
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
    let mut vv: Vec<[Point; 2]> =
        c.vv.iter()
            .map(|r| {
                let mut ps = [at_point(body, r.a), at_point(body, r.b)];
                ps.sort_unstable();
                ps
            })
            .collect();
    vv.sort_unstable();
    let mut vf: Vec<Point> = c
        .a_on_b
        .iter()
        .chain(&c.b_on_a)
        .map(|r| at_point(body, r.vertex))
        .collect();
    vf.sort_unstable();
    Record {
        vv,
        vf,
        curves: c.curves.len(),
        patches: c.patches.len(),
    }
}

/// What one result is, key-free.
#[derive(Clone, Debug, PartialEq, Eq)]
struct Outcome {
    shape: Shape,
    record: Record,
    /// The tier-3′ verdict over the body and its record, each error by
    /// its kind and, for an undeclared contact, its census kind and
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
                topo::ValidationError::UndeclaredContact { contact, witness } => (
                    format!("UndeclaredContact {}", kind(&format!("{contact:?}"))),
                    witness_point(witness),
                ),
                other => (kind(&format!("{other:?}")).to_owned(), None),
            })
            .collect();
        es.sort_unstable();
        es
    });
    if let Err(e) = topo::test_support::meeting::corners_disjoint(body) {
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

/// Where a body's contact record and tier-3′ verdict are owed but, while
/// it stands, not published: a boolean publishes the contacts its own
/// step decided and drops its operands', so a union's record is its last
/// fold step's.
const DROPPED_RECORDS: &str = "work/wire/a-boolean-drops-its-operands-own-contact-records.md";

/// The double edge two touching solids leave meshes as one segment of
/// four triangles, which `check_mesh` refuses.
const DOUBLED_EDGE: &str =
    "work/tess/two-coincident-edges-between-one-vertex-pair-mesh-non-manifold.md";

/// An undeclared contact 3′ reports: its kind and its witness point.
type Contact = (&'static str, Point);

/// Where two blocks touch beyond the plate: the overlap of their
/// coincident edges, witnessed at its middle, and the vertex at its end,
/// which rests on the other block's edge once the output stage has
/// joined that edge's cut vertex away (maximal edges).
fn touch(overlap: Point, end: Point) -> [Contact; 2] {
    [("EdgeEdgeOverlap", overlap), ("VertexOnEdge", end)]
}

/// `p1` against `p2` above the plate's top.
fn p1_p2() -> [Contact; 2] {
    touch(
        (1_500_000, 1_000_000, 1_350_000),
        (1_500_000, 1_000_000, 1_700_000),
    )
}

/// Asserts `o` is refused at 3′ by exactly the undeclared `contacts`,
/// the operand records [`DROPPED_RECORDS`] drops.
fn dropped(o: &Outcome, what: &str, contacts: &[Contact]) {
    let mut want: Vec<(String, Option<Point>)> = contacts
        .iter()
        .map(|&(k, p)| (format!("UndeclaredContact {k}"), Some(p)))
        .collect();
    want.sort_unstable();
    match &o.verdict {
        Err(es) => assert_eq!(
            es, &want,
            "{what}: refused at 3′ by other than the dropped records ({DROPPED_RECORDS})"
        ),
        Ok(()) => {
            panic!("{what}: passes 3′ — {DROPPED_RECORDS} may be fixed; assert one verdict instead")
        }
    }
}

/// Asserts every member order of `fixture`'s union builds one body,
/// with `counts` (faces, edges, vertices), `volume` and one vertex at
/// each of `pinches`, and one mesh verdict.
///
/// The record and the 3′ verdict follow the LAST member folded
/// ([`DROPPED_RECORDS`]): orders that fold one member last agree on
/// both, and 3′ refuses exactly the `touches` between two members
/// neither of which is the last, the contacts no step after them
/// decided.
fn every_order(
    label: &str,
    fixture: fn(ProfileDoc) -> (ProfileDoc, Vec<RecipeNodeId>),
    counts: [usize; 3],
    volume: f64,
    pinches: &[Point],
    touches: &[([usize; 2], &[Contact])],
) {
    let mut first: Option<Outcome> = None;
    let mut by_last: BTreeMap<usize, Outcome> = BTreeMap::new();
    let n = fixture(ProfileDoc::empty_derived("union_pinch", Tol::witness()))
        .1
        .len();
    for order in orders(n) {
        let (doc, m) = fixture(ProfileDoc::empty_derived("union_pinch", Tol::witness()));
        let members: Vec<RecipeNodeId> = order.iter().map(|&i| m[i]).collect();
        let (doc, u) = crate::fixture::union_over(doc, &members, Vec::new());
        let what = format!("{label}, member order {order:?} (0 = plate)");
        let ev = run(&doc);
        let o = checked(&ev, u, &what, volume);
        assert_eq!(o.shape.counts(), counts, "{what}: faces, edges, vertices");
        for &p in pinches {
            assert_eq!(at(&o.shape, p), 1, "{what}: vertices at the pinch {p:?}");
        }
        let last = order[n - 1];
        let undecided: Vec<Contact> = touches
            .iter()
            .filter(|(pair, _)| !pair.contains(&last))
            .flat_map(|&(_, cs)| cs.iter().copied())
            .collect();
        if undecided.is_empty() {
            assert_eq!(o.verdict, Ok(()), "{what}: 3′");
        } else {
            dropped(&o, &what, &undecided);
        }
        if let Some(f) = &first {
            assert_eq!(
                o.shape, f.shape,
                "{what}: a different body from the first order"
            );
            assert_eq!(
                o.manifold, f.manifold,
                "{what}: a different mesh verdict from the first order"
            );
        }
        match by_last.get(&last) {
            Some(f) => assert_eq!(
                (&o.record, &o.verdict),
                (&f.record, &f.verdict),
                "{what}: a different record or 3′ verdict from an order folding the same member last"
            ),
            None => {
                by_last.insert(last, o.clone());
            }
        }
        if first.is_none() {
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
    every_order(
        "top",
        pinch,
        [19, 48, 31],
        UNION_VOLUME,
        &[top()],
        &[([1, 2], &p1_p2())],
    );
    every_order(
        "side",
        side_pinch,
        [16, 36, 23],
        6.0 + (0.225 - 0.075) + (0.2145 - 0.0795),
        &[(3_000_000, 1_000_000, 500_000)],
        &[(
            [1, 2],
            &touch(
                (3_450_000, 1_000_000, 500_000),
                (3_900_000, 1_000_000, 500_000),
            ),
        )],
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
        &[(
            [1, 2],
            &[
                p1_p2(),
                touch(
                    (1_500_000, 1_000_000, -235_000),
                    (1_500_000, 1_000_000, -470_000),
                ),
            ]
            .concat(),
        )],
    );
    every_order(
        "two pinches on the top",
        two_pinches,
        [26, 66, 42],
        UNION_VOLUME + 0.5 * (0.9 + 0.5),
        &[top(), (1_000_000, 1_000_000, 1_000_000)],
        &[
            ([1, 2], &p1_p2()),
            (
                [1, 3],
                &touch(
                    (1_000_000, 1_000_000, 1_250_000),
                    (1_000_000, 1_000_000, 1_500_000),
                ),
            ),
        ],
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
    let touches: &[([usize; 2], &[Contact])] = &[([1, 2], &p1_p2())];
    every_order(
        "corner holes",
        corner_holes,
        [16, 36, 23],
        6.0 + 0.25 * (2.0 - 1.0) + 0.25 * (1.7 - 1.0),
        &[top()],
        touches,
    );
    every_order(
        "notch and hole",
        notch_and_hole,
        [17, 42, 27],
        6.0 + 0.5 * 2.0 * (2.0 - 1.0) + 0.5 * 1.0 * (1.0 - 0.5) + 0.25 * (1.7 - 1.0),
        &[top()],
        touches,
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
        touches,
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
    let (doc, blocks) = crate::fixture::union_over(doc, &[p1, p2], Vec::new());
    let (doc, folded) = crate::fixture::union_over(doc, &[p1, p2, plate], Vec::new());
    let pair = |doc, op, a, b| {
        insert(
            doc,
            Node::Boolean {
                op,
                a,
                b,
                declare: Vec::new(),
            },
        )
    };
    let (doc, plate_first) = pair(doc, BooleanOp::Union, plate, blocks);
    let (doc, blocks_first) = pair(doc, BooleanOp::Union, blocks, plate);
    let (doc, notched) = pair(doc, BooleanOp::Subtract, plate, blocks);
    let (doc, footprints) = pair(doc, BooleanOp::Intersect, plate, blocks);
    let ev = run(&doc);

    // The joined blocks hold their contact in their own record, which
    // none of these booleans carries into its result: 3′ finds it where
    // the result keeps it, the contact the plate clips it to.
    let m = |what: &str, o: Outcome, contacts: &[Contact]| {
        dropped(&o, what, contacts);
        assert_eq!(o.manifold, Ok(()), "{what}: check_mesh");
        o.shape
    };
    let below = |ends: &[(&'static str, i64)]| {
        let mut cs = vec![("EdgeEdgeOverlap", (1_500_000, 1_000_000, 750_000))];
        cs.extend(
            ends.iter()
                .map(|&(kind, z)| (kind, (1_500_000, 1_000_000, z))),
        );
        cs
    };
    let union = m(
        "the folded union",
        checked(&ev, folded, "the folded union", UNION_VOLUME),
        &p1_p2(),
    );
    for (what, id) in [
        ("plate ∪ blocks", plate_first),
        ("blocks ∪ plate", blocks_first),
    ] {
        assert_eq!(
            m(what, checked(&ev, id, what, UNION_VOLUME), &p1_p2()),
            union,
            "{what}: a different body from the member-order union"
        );
    }

    let s = m(
        "plate ∖ blocks",
        checked(&ev, notched, "plate ∖ blocks", 6.0 - NOTCHES),
        &below(&[("VertexOnEdge", 500_000)]),
    );
    assert_eq!(at(&s, top()), 1, "plate ∖ blocks: vertices at the pinch");
    let tops = s
        .faces
        .keys()
        .filter(|ps| ps.iter().all(|p| p.2 == 1_000_000))
        .count();
    assert_eq!(tops, 2, "plate ∖ blocks: faces on the top");

    let s = m(
        "plate ∩ blocks",
        checked(&ev, footprints, "plate ∩ blocks", NOTCHES),
        &below(&[("VertexOnEdge", 500_000), ("VertexVertex", 1_000_000)]),
    );
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
                declare: Vec::new(),
            },
        )
    };
    // One cut: cutting the blocks one at a time sets the second block's
    // wall flush with the first's hole wall, an undeclared continuation.
    let (doc, blocks) = crate::fixture::union_over(doc, &[p1, p2], Vec::new());
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
            (750_000, &[("VertexVertex", 1_000_000)][..]),
        ),
        (
            "X ∖ plate",
            BooleanOp::Subtract,
            x,
            plate,
            x_volume - shared,
            [23, 60, 39],
            1,
            (1_500_000, &[("VertexOnEdge", 2_000_000)][..]),
        ),
        (
            "X ∪ plate",
            BooleanOp::Union,
            x,
            plate,
            x_volume + 6.0 - shared,
            [22, 60, 40],
            2,
            (
                1_500_000,
                &[("VertexVertex", 1_000_000), ("VertexOnEdge", 2_000_000)][..],
            ),
        ),
        (
            "plate ∪ X",
            BooleanOp::Union,
            plate,
            x,
            x_volume + 6.0 - shared,
            [22, 60, 40],
            2,
            (
                1_500_000,
                &[("VertexVertex", 1_000_000), ("VertexOnEdge", 2_000_000)][..],
            ),
        ),
        (
            "X ∩ plate",
            BooleanOp::Intersect,
            x,
            plate,
            shared,
            [16, 36, 22],
            1,
            (0, &[][..]),
        ),
        (
            "plate ∩ X",
            BooleanOp::Intersect,
            plate,
            x,
            shared,
            [16, 36, 22],
            1,
            (0, &[][..]),
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
    for (&(what, op, _, _, volume, counts, pinch, (overlap, ends)), &id) in rows.iter().zip(&ids) {
        let o = checked(&ev, id, what, volume);
        if op == BooleanOp::Intersect {
            // Two L-prisms touching along the contact line, which runs
            // between one vertex at each end as two coincident edges.
            assert_eq!(o.verdict, Ok(()), "{what}: 3′");
            assert_eq!(
                o.manifold,
                Err("NonManifoldEdge".to_owned()),
                "{what}: check_mesh, while {DOUBLED_EDGE} stands"
            );
        } else {
            // X holds the blocks' contact in its own record: 3′ finds it
            // where the result keeps it.
            let line = |z| (1_500_000, 1_000_000, z);
            let mut contacts = vec![("EdgeEdgeOverlap", line(overlap))];
            contacts.extend(ends.iter().map(|&(kind, z)| (kind, line(z))));
            dropped(&o, what, &contacts);
            assert_eq!(o.manifold, Ok(()), "{what}: check_mesh");
        }
        let s = o.shape;
        assert_eq!(s.counts(), counts, "{what}: faces, edges, vertices");
        assert_eq!(at(&s, top()), pinch, "{what}: vertices at the pinch");
        shapes.push(s);
    }
    assert_eq!(shapes[2], shapes[3], "X ∪ plate and plate ∪ X: one body");
    assert_eq!(shapes[4], shapes[5], "X ∩ plate and plate ∩ X: one body");
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
                profile: p,
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
        every_order(
            label,
            fixture,
            counts,
            tilted_volume(&holes()),
            &[top()],
            &[],
        );
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
        every_order(
            label,
            fixture,
            counts,
            tilted_volume(&holes()),
            &[top()],
            &[],
        );
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

/// **Where three wedges' axes cross, at the plate's top, the vertex
/// has one junction name in every member order that folds the wedges
/// before the plate**: the seam lines through it, in canonical order.
/// Orders that fold the plate earlier mint the vertex at another step,
/// under another name
/// (`work/wire/a-pinch-vertex-is-named-by-the-fold-step-that-mints-it.md`).
#[test]
fn the_junction_where_the_wedges_meet_has_one_name_in_every_member_order() {
    let mut first: Option<Vec<String>> = None;
    for order in orders(4).into_iter().filter(|o| o[3] == 0) {
        let (doc, m) = tilted_holes(
            ProfileDoc::empty_derived("union_pinch", Tol::witness()),
            &three_wedges(),
        );
        let members: Vec<RecipeNodeId> = order.iter().map(|&i| m[i]).collect();
        let (doc, u) = crate::fixture::union_over(doc, &members, Vec::new());
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
        assert_eq!(
            names[0].matches("Seam {").count(),
            6,
            "member order {order:?}: the junction is named by its six seam lines"
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

/// **Where two leaning wedges' legs meet at the plate's top, their union
/// names the vertex a seam of the two legs, in either member order**:
/// each leg lies outside the other wedge on both sides of the vertex,
/// so neither enters or leaves the other and the vertex is no crossing.
/// The fold reaches one wedge's copy of the vertex only through two
/// fusions (`names::emit_topo::fused_partners`).
#[test]
fn two_wedges_name_their_meeting_point_a_seam_of_their_legs_in_either_order() {
    let mut first: Option<Vec<String>> = None;
    for order in [[1, 2], [2, 1]] {
        let (doc, m) = tilted_holes(
            ProfileDoc::empty_derived("union_pinch", Tol::witness()),
            &two_wedges(),
        );
        let (doc, u) = crate::fixture::union_over(doc, &[m[order[0]], m[order[1]]], Vec::new());
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
            n.contains("path: [Seam {") && !n.contains("Crossing"),
            "member order {order:?}: the meeting point is a seam, no crossing: {n}"
        );
        assert_eq!(
            n.matches("role: Leg").count(),
            2,
            "member order {order:?}: the seam is of the two legs: {n}"
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
/// [`every_order`] does, where three solids touch along one line. The
/// contacts [`DROPPED_RECORDS`] drops are those of earlier members that
/// the last one's do not cover, which [`every_order`]'s pairs cannot
/// name: `dropped[i]` of them, all on the line, when member `i` folds
/// last.
fn every_order_on_a_line(
    label: &str,
    fixture: fn(ProfileDoc) -> (ProfileDoc, Vec<RecipeNodeId>),
    counts: [usize; 3],
    volume: f64,
    pinch: bool,
    dropped: &[usize],
) {
    let mut first: Option<Outcome> = None;
    let n = dropped.len();
    for order in orders(n) {
        let (doc, m) = fixture(ProfileDoc::empty_derived("on_a_line", Tol::witness()));
        let members: Vec<RecipeNodeId> = order.iter().map(|&i| m[i]).collect();
        let (doc, u) = crate::fixture::union_over(doc, &members, Vec::new());
        let what = format!("{label}, member order {order:?}");
        let o = checked(&run(&doc), u, &what, volume);
        assert_eq!(o.shape.counts(), counts, "{what}: faces, edges, vertices");
        if pinch {
            assert_eq!(at(&o.shape, top()), 1, "{what}: vertices at the pinch");
        }
        let refused = match &o.verdict {
            Ok(()) => Vec::new(),
            Err(es) => es.clone(),
        };
        for (kind, p) in &refused {
            assert!(
                kind.starts_with("UndeclaredContact")
                    && p.is_some_and(|(x, y, _)| (x, y) == (1_500_000, 1_000_000)),
                "{what}: 3′ refused off the line: {kind} at {p:?}"
            );
        }
        assert_eq!(
            refused.len(),
            dropped[order[n - 1]],
            "{what}: 3′ refused by other than the records {DROPPED_RECORDS} drops: {refused:?}"
        );
        if let Some(f) = &first {
            assert_eq!(
                o.shape, f.shape,
                "{what}: a different body from the first order"
            );
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
        &[6, 0, 2, 0],
    );
    every_order_on_a_line(
        "three prisms",
        three_prisms,
        [15, 27, 18],
        above * (1.5 + 1.23 + 1.37),
        false,
        &[2, 2, 0],
    );
    every_order_on_a_line(
        "four prisms and the plate",
        four_on_a_line,
        [22, 48, 29],
        above.mul_add(1.0 + 0.7 + 0.81 + 0.63, 6.0),
        true,
        &[12, 0, 2, 0, 6],
    );
    every_order_on_a_line(
        "four prisms",
        four_prisms,
        [20, 36, 24],
        above * (1.5 + 1.23 + 1.37 + 1.22),
        false,
        &[6, 4, 0, 6],
    );
}

/// Review probe (PR 4346): what each order of the crossing fixtures serves.
#[test]
#[ignore = "probe"]
fn review_4346_crossing_orders() {
    for (a0, a1) in [(30.0, 60.0), (30.0, 140.0)] {
        for plate in [true, false] {
            let fixture = |doc: ProfileDoc| {
                let (doc, pl) = block(doc, (0.0, 3.0), (0.0, 2.0), 0.0, 1.0);
                let (doc, a) = sector_prism(doc, 0.0, 50.0, (0.5, 2.0));
                let (doc, b) = sector_prism(doc, 120.0, 170.0, (0.47, 1.7));
                let (doc, c) = sector_prism(doc, a0, a1, (0.44, 1.81));
                (doc, if plate { vec![pl, a, b, c] } else { vec![a, b, c] })
            };
            let n = if plate { 4 } else { 3 };
            for order in orders(n) {
                let (doc, m) = fixture(ProfileDoc::empty_derived("probe", Tol::witness()));
                let members: Vec<RecipeNodeId> = order.iter().map(|&i| m[i]).collect();
                let (doc, u) = crate::fixture::union_over(doc, &members, Vec::new());
                let ev = run(&doc);
                let tag = match failure(&ev, u) {
                    None => "built".to_string(),
                    Some(e) => format!("{e:?}").chars().take(220).collect(),
                };
                eprintln!("EC {a0}-{a1} plate={plate} {order:?}: {tag}");
            }
        }
    }
}
