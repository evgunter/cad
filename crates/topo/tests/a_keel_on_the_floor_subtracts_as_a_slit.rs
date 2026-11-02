//! **A keel resting on the floor subtracts as the slit form.** The slab
//! `[0, 4]² × [0, 1]` less the prism over the `(y, z)` triangle `(2, 0)`,
//! `(2.6, 2)`, `(1.4, 2)` along x, its keel, the valley edge, on the
//! floor `z = 0`. About the keel the material is two wedges of
//! `atan2(2, 0.6)` each, floor | wall a and wall b | floor, and the floor
//! runs between them (`crates/topo/README.md`, tier 3: no face passes
//! through an edge's material wedge). So the result carries the keel as
//! two coincident distinct edges, each bounding one wedge, in the floor:
//!
//! - closed at both ends (`x ∈ [1.5, 2.5]`): a zero-area ring of the
//!   floor, one vertex at each end;
//! - open at one end (`x ∈ [1.5, 4.5]`): a slit of the floor's outline,
//!   the open end one vertex per wedge;
//! - wall to wall (`x ∈ [−0.5, 4.5]`): two floor faces and two solids,
//!   which touch along the keel only (PR 3901's solid rule).
//!
//! Every row runs at every pose and through three doors that build the
//! one body: `slab ∖ trench`, and `slab ∩ (box ∖ trench)` in either
//! order, the box's keel inside it.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use common::meeting::{Pose, poses};
use common::split_plane;
use geom_core::{Tol, Vec3};
use topo::{
    AtRestBody, Body, BooleanResult, ContactRecords, EdgeKey, FaceKey, LoopBoundary, PointKey,
    SplitPart, intersect, split, subtract, validate, validate_closed, validate_geometric,
    validate_pseudomanifold,
};

fn t() -> Tol {
    Tol::witness()
}

/// A prism over `profile`, its frame `(a, c, h)` placed by `map`.
fn prism(
    pose: &Pose,
    what: &str,
    profile: &[(f64, f64)],
    span: (f64, f64),
    map: &dyn Fn(f64, f64, f64) -> [f64; 3],
) -> AtRestBody<f64> {
    use topo::test_support::{FaceGeometry, describe_as_intersections, finished, prism_ops};
    let mut b = Body::<f64>::new();
    prism_ops(
        &mut b,
        profile,
        span,
        |a, c, h| pose.at(map(a, c, h)),
        FaceGeometry::Certified,
        t(),
    );
    describe_as_intersections(&mut b, t());
    finished(what, b, t())
}

/// The box `[0, 4]² × [lo, hi]`.
fn slab(pose: &Pose, (lo, hi): (f64, f64)) -> AtRestBody<f64> {
    prism(
        pose,
        "the slab",
        &[(0.0, 0.0), (4.0, 0.0), (4.0, 4.0), (0.0, 4.0)],
        (lo, hi),
        &|x, y, z| [x, y, z],
    )
}

/// The trench over `span` of x.
fn trench(pose: &Pose, span: (f64, f64)) -> AtRestBody<f64> {
    prism(
        pose,
        "the trench",
        &[(2.0, 0.0), (2.6, 2.0), (1.4, 2.0)],
        span,
        &|y, z, x| [x, y, z],
    )
}

/// One extent of the trench and what the material makes of it.
struct Row {
    what: &'static str,
    span: (f64, f64),
    /// `16 − 0.3` per unit of the keel's length in the slab.
    volume: f64,
    solids: usize,
    /// The vertices at each end of the keel, one per wedge where it
    /// reaches the floor's boundary.
    at_ends: [usize; 2],
}

const ROWS: [Row; 3] = [
    Row {
        what: "closed at both ends",
        span: (1.5, 2.5),
        volume: 16.0 - 0.3,
        solids: 1,
        at_ends: [1, 1],
    },
    Row {
        what: "open at one end",
        span: (1.5, 4.5),
        volume: 16.0 - 0.3 * 2.5,
        solids: 1,
        at_ends: [1, 2],
    },
    Row {
        what: "wall to wall",
        span: (-0.5, 4.5),
        volume: 16.0 - 0.3 * 4.0,
        solids: 2,
        at_ends: [2, 2],
    },
];

impl Row {
    /// The keel's ends in the slab.
    fn ends(&self) -> [[f64; 3]; 2] {
        [
            [self.span.0.max(0.0), 2.0, 0.0],
            [self.span.1.min(4.0), 2.0, 0.0],
        ]
    }
}

/// The three doors' results for `row` at `pose`.
fn doors(row: &Row, pose: &Pose) -> Vec<(String, topo::BooleanBody<f64>)> {
    let tol = t();
    let s = slab(pose, (0.0, 1.0));
    let tr = trench(pose, row.span);
    let big = prism(
        pose,
        "the box",
        &[(-1.0, -1.0), (5.0, -1.0), (5.0, 5.0), (-1.0, 5.0)],
        (-1.0, 3.0),
        &|x, y, z| [x, y, z],
    );
    let inner = trench(pose, (row.span.0.max(-0.5), row.span.1.min(4.5)));
    let Ok(BooleanResult::Body(holed)) = subtract(&big, &inner, tol) else {
        panic!("{}, {}: the box less the trench", row.what, pose.label);
    };
    let holed = holed.body;
    [
        ("slab ∖ trench", subtract(&s, &tr, tol)),
        ("slab ∩ (box ∖ trench)", intersect(&s, &holed, tol)),
        ("(box ∖ trench) ∩ slab", intersect(&holed, &s, tol)),
    ]
    .into_iter()
    .map(|(door, r)| {
        let what = format!("{}, {}, {door}", row.what, pose.label);
        match r {
            Ok(BooleanResult::Body(b)) => (what, b),
            other => panic!("{what}: {other:?}"),
        }
    })
    .collect()
}

/// The point key of each end of `edge`, in key order.
fn ends_of(body: &Body<f64>, edge: EdgeKey) -> [PointKey; 2] {
    let he = body.get_edge(edge).unwrap().he_plus;
    let mut v = [
        body.get_half_edge(he).unwrap().start,
        body.half_edge_end(he).unwrap(),
    ]
    .map(|v| body.get_vertex(v).unwrap().point);
    v.sort();
    v
}

/// The spans along the line `from + t·(to − from)`, in `t`, of the
/// edges of `body` lying on it, `from` and `to` placed by `pose` a unit
/// apart.
fn spans_along(body: &Body<f64>, pose: &Pose, [from, to]: [[f64; 3]; 2]) -> Vec<(f64, f64)> {
    let (o, d) = (pose.at(from), pose.at(to) - pose.at(from));
    let along = |p: geom_core::Point3<f64>| {
        let t = (p - o).dot(d);
        ((o + d * t - p).norm() < 1e-9).then_some(t)
    };
    body.edges()
        .filter_map(|(e, _)| {
            let [a, b] = ends_of(body, e).map(|k| *body.get_point(k).unwrap());
            Some((along(a)?, along(b)?))
        })
        .map(|(a, b)| (a.min(b), a.max(b)))
        .collect()
}

/// The edges of `body` running between the keel's two ends.
fn keel_edges(body: &Body<f64>, pose: &Pose, ends: [[f64; 3]; 2]) -> Vec<EdgeKey> {
    let at = |k: PointKey, p: [f64; 3]| body.get_point(k).unwrap().distance(pose.at(p)) < 1e-9;
    body.edges()
        .map(|(e, _)| e)
        .filter(|&e| {
            let [k0, k1] = ends_of(body, e);
            (at(k0, ends[0]) && at(k1, ends[1])) || (at(k0, ends[1]) && at(k1, ends[0]))
        })
        .collect()
}

/// The vertices of `body` at `p`.
fn vertices_at(body: &Body<f64>, pose: &Pose, p: [f64; 3]) -> usize {
    body.vertices()
        .filter(|(_, v)| body.get_point(v.point).unwrap().distance(pose.at(p)) < 1e-9)
        .count()
}

/// The edges of each loop of `face`, the outline first.
fn loops(body: &Body<f64>, face: FaceKey) -> Vec<Vec<EdgeKey>> {
    let f = body.get_face(face).unwrap();
    core::iter::once(f.outer)
        .chain(f.rings.iter().copied())
        .map(|l| match body.get_loop(l).unwrap().boundary {
            LoopBoundary::Cycle { first } => body
                .loop_cycle(first)
                .unwrap()
                .into_iter()
                .map(|he| body.get_half_edge(he).unwrap().edge)
                .collect(),
            LoopBoundary::Empty { .. } => Vec::new(),
        })
        .collect()
}

/// Whether every vertex of `face` lies on the floor `z = 0` at `pose`.
fn on_floor(body: &Body<f64>, pose: &Pose, face: FaceKey) -> bool {
    let (o, n) = (pose.at([0.0, 0.0, 0.0]), pose.at([0.0, 0.0, 1.0]));
    let n = n - o;
    loops(body, face).iter().flatten().all(|&e| {
        ends_of(body, e)
            .iter()
            .all(|&k| (*body.get_point(k).unwrap() - o).dot(n).abs() < 1e-9)
    })
}

/// The two faces of `edge`, the floor's first.
fn floor_and_wall(body: &Body<f64>, pose: &Pose, edge: EdgeKey) -> (FaceKey, FaceKey) {
    let face_of = |he| {
        let l = body.get_half_edge(he).unwrap().parent_loop;
        body.get_loop(l).unwrap().face
    };
    let e = body.get_edge(edge).unwrap();
    let (f0, f1) = (face_of(e.he_plus), face_of(e.he_minus));
    match (on_floor(body, pose, f0), on_floor(body, pose, f1)) {
        (true, false) => (f0, f1),
        (false, true) => (f1, f0),
        other => panic!("{edge:?} bounds faces on the floor {other:?}, not one floor and one wall"),
    }
}

/// The keel of a slit form of `row` in `b`: two distinct edges on one
/// line, each between the floor and its own wall, carried by the floor
/// as a ring, a slit of its outline, or across two floor faces.
fn slit_form_keel(b: &Body<f64>, pose: &Pose, row: &Row, what: &str) {
    let ends = row.ends();
    let keel = keel_edges(b, pose, ends);
    let [ka, kb] = keel[..] else {
        panic!("{what}: the keel is {keel:?}, not two edges");
    };
    assert_ne!(ka, kb, "{what}: the keel's two edges are one");
    assert_eq!(
        ends_of(b, ka),
        ends_of(b, kb),
        "{what}: the keel's edges end on different points"
    );
    let ((fa, wa), (fb, wb)) = (floor_and_wall(b, pose, ka), floor_and_wall(b, pose, kb));
    assert_ne!(wa, wb, "{what}: the keel's edges bound one wall");
    let in_loop = |f: FaceKey, l: usize, edges: &[EdgeKey]| {
        loops(b, f)
            .get(l)
            .is_some_and(|lp| edges.iter().all(|e| lp.contains(e)))
    };
    match row.solids {
        2 => {
            assert_ne!(fa, fb, "{what}: one floor face across the keel");
            assert!(
                in_loop(fa, 0, &[ka]) && in_loop(fb, 0, &[kb]),
                "{what}: a keel edge off its floor's outline"
            );
        }
        _ => {
            assert_eq!(fa, fb, "{what}: the keel's edges in two floor faces");
            let shape = loops(b, fa);
            if row.at_ends == [1, 1] {
                assert!(
                    shape[1..]
                        .iter()
                        .any(|r| r.len() == 2 && r.contains(&ka) && r.contains(&kb)),
                    "{what}: the floor's loops {shape:?}, no ring of the keel's edges"
                );
            } else {
                assert!(
                    in_loop(fa, 0, &[ka, kb]) && shape[1..].is_empty(),
                    "{what}: the floor's loops {shape:?}, not a slit of its outline"
                );
            }
        }
    }
}

/// **Each extent subtracts as its slit form, at every pose and through
/// every door**: tiers 1, 2, 3 and 3′ with no records, the closed-form
/// volume and solid count, and the keel as two distinct edges on one
/// line, each between the floor and its own wall, carried by the floor
/// as a ring, a slit of its outline, or across two floor faces.
#[test]
fn a_keel_on_the_floor_subtracts_as_its_slit_form() {
    let tol = t();
    for row in &ROWS {
        for pose in poses() {
            for (what, r) in doors(row, &pose) {
                let b = &r.body;
                assert_eq!(validate(b), Ok(()), "{what}: tier 1");
                assert_eq!(validate_closed(b), Ok(()), "{what}: tier 2");
                assert_eq!(validate_geometric(b, tol), Ok(()), "{what}: tier 3");
                assert!(
                    r.contacts.rows().is_empty(),
                    "{what}: records {:?}",
                    r.contacts
                        .rows()
                        .iter()
                        .map(|c| c.record.kind())
                        .collect::<Vec<_>>()
                );
                assert_eq!(
                    validate_pseudomanifold(b, &ContactRecords::default(), tol),
                    Ok(()),
                    "{what}: tier 3′"
                );
                let v = topo::test_support::pinned_volume("the measured body", b, tol).volume;
                assert!(
                    (v - row.volume).abs() < 1e-9 * row.volume,
                    "{what}: volume {v}, closed form {}",
                    row.volume
                );
                assert_eq!(b.solids().count(), row.solids, "{what}: solids");
                let ends = row.ends();
                for (k, p) in ends.into_iter().enumerate() {
                    assert_eq!(
                        vertices_at(b, &pose, p),
                        row.at_ends[k],
                        "{what}: vertices at the keel's end {p:?}"
                    );
                }
                slit_form_keel(b, &pose, row, &what);
            }
        }
    }
}

/// **The doors agree**: the split's side holding the walls, of the box
/// `[0, 4]² × [−1, 1]` less the trench split by `z = 0`, is the body the
/// Boolean builds, at every extent: the same volume, solids, and
/// vertex, edge and face counts.
#[test]
fn the_split_and_the_boolean_build_one_body() {
    let tol = t();
    for row in &ROWS {
        for pose in poses() {
            let Ok(BooleanResult::Body(cut)) =
                subtract(&slab(&pose, (-1.0, 1.0)), &trench(&pose, row.span), tol)
            else {
                panic!("{}, {}: the box less the trench", row.what, pose.label);
            };
            let o = pose.at([0.0, 0.0, 0.0]);
            let n = pose.at([0.0, 0.0, 1.0]) - o;
            let sides = split(
                &cut.body,
                &split_plane(o, Vec3::new(n.x, n.y, n.z), tol),
                tol,
            )
            .unwrap_or_else(|e| panic!("{}, {}: the split: {e:?}", row.what, pose.label));
            let SplitPart::Body(walls) = sides.above else {
                panic!("{}, {}: nothing above the floor", row.what, pose.label);
            };
            let count = |b: &Body<f64>| {
                (
                    b.vertices().count(),
                    b.edges().count(),
                    b.faces().count(),
                    b.solids().count(),
                )
            };
            let split_volume =
                topo::test_support::pinned_volume("the measured body", &walls, tol).volume;
            for (what, r) in doors(row, &pose) {
                assert_eq!(
                    count(&r.body),
                    count(&walls),
                    "{what}: (v, e, f, solids) against the split"
                );
                let v = topo::test_support::pinned_volume("the measured body", &r.body, tol).volume;
                assert!(
                    (v - split_volume).abs() < 1e-9 * v,
                    "{what}: volume {v}, the split's {split_volume}"
                );
            }
        }
    }
}

/// **A trench inside the slab, its keel on the floor, opens as a ring.**
/// The V `(2, 0)`, `(2.15, 0.5)`, `(1.85, 0.5)` over `x ∈ [1.5, 2.5]`
/// touches the slab's boundary only along its keel, so no boundaries
/// cross and ∖ takes the containment fallback, which grafts it as a
/// cavity. About the keel the material is two wedges all the same: the
/// cavity opens through the floor along the keel, a zero-area ring of
/// the floor, and the result is one shell.
#[test]
fn a_cavity_resting_its_keel_on_the_floor_opens_through_it_as_a_ring() {
    let tol = t();
    for pose in poses() {
        let what = pose.label;
        let tr = prism(
            &pose,
            "the low trench",
            &[(2.0, 0.0), (2.15, 0.5), (1.85, 0.5)],
            (1.5, 2.5),
            &|y, z, x| [x, y, z],
        );
        let Ok(BooleanResult::Body(r)) = subtract(&slab(&pose, (0.0, 1.0)), &tr, tol) else {
            panic!("{what}: the slab less the low trench");
        };
        let b = &r.body;
        assert_eq!(validate_closed(b), Ok(()), "{what}: tier 2");
        assert_eq!(validate_geometric(b, tol), Ok(()), "{what}: tier 3");
        assert!(r.contacts.rows().is_empty(), "{what}: records");
        assert_eq!(
            validate_pseudomanifold(b, &ContactRecords::default(), tol),
            Ok(()),
            "{what}: tier 3′"
        );
        // `16 − ½ · 0.3 · 0.5` along the trench's unit length.
        let v = topo::test_support::pinned_volume("the measured body", b, tol).volume;
        assert!((v - 15.925).abs() < 1e-9 * v, "{what}: volume {v}");
        assert_eq!(
            (b.solids().count(), b.shells().count()),
            (1, 1),
            "{what}: (solids, shells)"
        );
        let keel = keel_edges(b, &pose, [[1.5, 2.0, 0.0], [2.5, 2.0, 0.0]]);
        let [ka, kb] = keel[..] else {
            panic!("{what}: the keel is {keel:?}, not two edges");
        };
        let ((fa, _), (fb, _)) = (floor_and_wall(b, &pose, ka), floor_and_wall(b, &pose, kb));
        assert_eq!(fa, fb, "{what}: the keel's edges in two floor faces");
        assert!(
            loops(b, fa)[1..]
                .iter()
                .any(|r| r.len() == 2 && r.contains(&ka) && r.contains(&kb)),
            "{what}: no ring of the keel's edges in the floor"
        );
    }
}

/// The slit form of `row` at `pose` from both doors that build it: the
/// Boolean's `slab ∖ trench`, and the split's side holding the walls, of
/// the box `[0, 4]² × [−1, 1]` less the trench split by `z = 0`.
fn slit_operands(row: &Row, pose: &Pose) -> [(&'static str, AtRestBody<f64>); 2] {
    let tol = t();
    let cut = |lo| match subtract(&slab(pose, (lo, 1.0)), &trench(pose, row.span), tol) {
        Ok(BooleanResult::Body(b)) => b.body,
        other => panic!(
            "{}, {}: the slab less the trench: {other:?}",
            row.what, pose.label
        ),
    };
    let o = pose.at([0.0, 0.0, 0.0]);
    let n = pose.at([0.0, 0.0, 1.0]) - o;
    let sides = split(
        &cut(-1.0),
        &split_plane(o, Vec3::new(n.x, n.y, n.z), tol),
        tol,
    )
    .unwrap_or_else(|e| panic!("{}, {}: the split: {e:?}", row.what, pose.label));
    let SplitPart::Body(walls) = sides.above else {
        panic!("{}, {}: nothing above the floor", row.what, pose.label);
    };
    [
        ("the Boolean's slit", cut(0.0)),
        ("the split's slit", walls),
    ]
}

/// One op on the slit operand and a slab `[0, 4]² × slab` whose face on
/// `z = 0` the slit's floor is glued to, and what it builds at each
/// extent.
struct Glued {
    what: &'static str,
    slab: (f64, f64),
    op: topo::BooleanOp,
    /// Whether the slab is the op's first operand.
    slab_first: bool,
    /// The closed form, from the keel's length in the slab.
    volume: fn(f64) -> f64,
    /// `(vertices, edges, faces)` closed, open at one end, wall to wall.
    counts: [(usize, usize, usize); 3],
    /// Whether the keel is an edge of the result, between the walls.
    keel: bool,
}

const GLUED: [Glued; 5] = [
    Glued {
        what: "slit ∪ the slab under",
        slab: (-1.0, 0.0),
        op: topo::BooleanOp::Union,
        slab_first: false,
        volume: |l| 32.0 - 0.3 * l,
        counts: [(14, 21, 10), (14, 21, 9), (14, 21, 9)],
        keel: true,
    },
    Glued {
        what: "the slab under ∪ slit",
        slab: (-1.0, 0.0),
        op: topo::BooleanOp::Union,
        slab_first: true,
        volume: |l| 32.0 - 0.3 * l,
        counts: [(14, 21, 10), (14, 21, 9), (14, 21, 9)],
        keel: true,
    },
    Glued {
        what: "the slab inside ∖ slit",
        slab: (0.0, 0.5),
        op: topo::BooleanOp::Subtract,
        slab_first: true,
        volume: |l| 0.075 * l,
        counts: [(6, 9, 5), (6, 9, 5), (6, 9, 5)],
        keel: true,
    },
    Glued {
        what: "slit ∪ the slab inside",
        slab: (0.0, 0.5),
        op: topo::BooleanOp::Union,
        slab_first: false,
        volume: |l| 16.0 - 0.225 * l,
        counts: [(16, 24, 11), (16, 24, 10), (16, 24, 10)],
        keel: false,
    },
    Glued {
        what: "the slab inside ∪ slit",
        slab: (0.0, 0.5),
        op: topo::BooleanOp::Union,
        slab_first: true,
        volume: |l| 16.0 - 0.225 * l,
        counts: [(16, 24, 11), (16, 24, 10), (16, 24, 10)],
        keel: false,
    },
];

/// **A face glued to the slit floor merges the slit's two edges into
/// one**: the slab less the trench, its floor carrying the keel as a
/// ring, a slit or across two faces, from the Boolean and from the split,
/// against a slab whose face on `z = 0` the floor is glued to: the slab
/// `[0, 4]² × [−1, 0]` under it, ∪ in either order, and the slab
/// `[0, 4]² × [0, 0.5]` inside it, which ∖ the slit leaves as the
/// trench's foot. The floor is glued away, so the material about the
/// keel is the two walls' alone (tier 3: the material decides the
/// edges), and the keel is one edge between them. ∪ the slab inside
/// fills the keel. At every extent and pose: tiers 1, 2, 3 and 3′ with
/// no records, one solid, the closed-form volume, the vertex, edge and
/// face counts, and the keel one edge between two walls.
#[test]
fn a_face_glued_to_the_slit_floor_merges_its_two_edges() {
    let tol = t();
    for (k, row) in ROWS.iter().enumerate() {
        let length = row.ends()[1][0] - row.ends()[0][0];
        for pose in poses() {
            for (origin, cut) in slit_operands(row, &pose) {
                for g in &GLUED {
                    let what = format!("{}, {}, {origin}, {}", row.what, pose.label, g.what);
                    let other = slab(&pose, g.slab);
                    let (a, b) = if g.slab_first {
                        (&other, &cut)
                    } else {
                        (&cut, &other)
                    };
                    let r = match topo::boolean_op_with(
                        g.op,
                        a,
                        b,
                        &topo::BooleanInputs::default(),
                        topo::SweepStrategy::Realized,
                        tol,
                    ) {
                        Ok(BooleanResult::Body(r)) => r,
                        other => panic!("{what}: {other:?}"),
                    };
                    let b = &r.body;
                    assert_eq!(validate(b), Ok(()), "{what}: tier 1");
                    assert_eq!(validate_closed(b), Ok(()), "{what}: tier 2");
                    assert_eq!(validate_geometric(b, tol), Ok(()), "{what}: tier 3");
                    assert!(r.contacts.rows().is_empty(), "{what}: records");
                    assert_eq!(
                        validate_pseudomanifold(b, &ContactRecords::default(), tol),
                        Ok(()),
                        "{what}: tier 3′"
                    );
                    let v = topo::test_support::pinned_volume("the measured body", b, tol).volume;
                    let want = (g.volume)(length);
                    assert!(
                        (v - want).abs() < 1e-9 * want,
                        "{what}: volume {v}, closed form {want}"
                    );
                    assert_eq!(b.solids().count(), 1, "{what}: solids");
                    assert_eq!(
                        (b.vertices().count(), b.edges().count(), b.faces().count()),
                        g.counts[k],
                        "{what}: (v, e, f)"
                    );
                    let keel = keel_edges(b, &pose, row.ends());
                    if !g.keel {
                        assert!(keel.is_empty(), "{what}: the keel is {keel:?}, not filled");
                        continue;
                    }
                    let [edge] = keel[..] else {
                        panic!("{what}: the keel is {keel:?}, not one edge");
                    };
                    let sides = topo::readback::edge_sides(b, edge).unwrap().faces();
                    assert_ne!(sides.0, sides.1, "{what}: the keel bounds one face twice");
                    assert!(
                        !on_floor(b, &pose, sides.0) && !on_floor(b, &pose, sides.1),
                        "{what}: the keel bounds a face on the floor"
                    );
                }
            }
        }
    }
}

/// **A slab flush with the slit floor intersects it keeping the floor
/// and its keel**: the slit form ∩ the slab `[0, 4]² × [0, 0.5]` inside
/// it, in either order. The two floors coincide with one sense, and ∩
/// keeps one of them; the material about the keel is the two wedges
/// floor | wall a and wall b | floor, so the kept floor wraps the keel,
/// and the dropped copy is substituted on both sides of it: no cut runs
/// along the keel in it. At every extent and pose: tiers 1, 2, 3 and 3′
/// against its records, the solids, the closed-form volume
/// `8 − 0.075` per unit of the keel's length, the vertex, edge and face
/// counts, and the keel the slit form's ([`slit_form_keel`]).
#[test]
fn a_slab_flush_with_the_slit_floor_intersects_it_keeping_the_keel() {
    let tol = t();
    let counts = [(14, 22, 10), (15, 23, 10), (16, 24, 12)];
    for (k, row) in ROWS.iter().enumerate() {
        let length = row.ends()[1][0] - row.ends()[0][0];
        for pose in poses() {
            let inside = slab(&pose, (0.0, 0.5));
            for (origin, cut) in slit_operands(row, &pose) {
                for (order, a, b) in [
                    ("slit ∩ the slab inside", &cut, &inside),
                    ("the slab inside ∩ slit", &inside, &cut),
                ] {
                    let what = format!("{}, {}, {origin}, {order}", row.what, pose.label);
                    let r = match topo::boolean_op_with(
                        topo::BooleanOp::Intersect,
                        a,
                        b,
                        &topo::BooleanInputs::default(),
                        topo::SweepStrategy::Realized,
                        tol,
                    ) {
                        Ok(BooleanResult::Body(r)) => r,
                        other => panic!("{what}: {other:?}"),
                    };
                    let b = &r.body;
                    assert_eq!(validate(b), Ok(()), "{what}: tier 1");
                    assert_eq!(validate_closed(b), Ok(()), "{what}: tier 2");
                    assert_eq!(validate_geometric(b, tol), Ok(()), "{what}: tier 3");
                    assert_eq!(
                        validate_pseudomanifold(b, &r.contacts, tol),
                        Ok(()),
                        "{what}: tier 3′"
                    );
                    assert_eq!(b.solids().count(), row.solids, "{what}: solids");
                    let v = topo::test_support::pinned_volume("the measured body", b, tol).volume;
                    let want = 8.0 - 0.075 * length;
                    assert!(
                        (v - want).abs() < 1e-9 * want,
                        "{what}: volume {v}, closed form {want}"
                    );
                    assert_eq!(
                        (b.vertices().count(), b.edges().count(), b.faces().count()),
                        counts[k],
                        "{what}: (v, e, f)"
                    );
                    slit_form_keel(b, &pose, row, &what);
                }
            }
        }
    }
}

/// **A partner flush beside the keel, its edge along the keel's line,
/// unions with the slit at tier 3**: the slit form open at one end
/// (the Boolean's), ∪ a part whose bottom lies on the floor on one side
/// of the keel and whose edge runs along the keel's line: the box
/// `[1, 3] × [2, 2.5] × [0, 0.5]`, and the prisms over the `(y, z)`
/// triangles `(2, 0), (2, 0.8), (1.2, 0)` and its mirror over the keel,
/// along x over `[1, 3]` and `[1.5, 2.5]`. The op keeps one of the two
/// flush floors, and the cut through the dropped one ends at the keel's
/// tip. Each builds, tiers 1, 2 and 3, one solid, at the closed-form
/// volume: the slit's `16 − 0.3 · 2.5` and the part's share of the
/// trench (`0.3 z` wide at height `z` for the box, `0.96 / 13` of the
/// triangle's section for the prisms) along their x overlap. Tier 3′
/// refuses an `EdgeEdgeOverlap` the result carries no record of
/// (`work/cleave/a-union-beside-a-slit-misses-an-edge-overlap-record.md`);
/// the Boolean door certifies tier 3, and 3′ waits on REACHHOLD's census
/// over its result. At every pose.
#[test]
fn a_partner_flush_beside_the_keel_unions_with_the_slit_at_tier_three() {
    let tol = t();
    let row = &ROWS[1];
    let trench_share = 0.96 / 13.0;
    for pose in poses() {
        let [(_, cut), _] = slit_operands(row, &pose);
        let parts: [(&str, AtRestBody<f64>, f64); 5] = [
            (
                "the box",
                prism(
                    &pose,
                    "the box",
                    &[(1.0, 2.0), (3.0, 2.0), (3.0, 2.5), (1.0, 2.5)],
                    (0.0, 0.5),
                    &|x, y, z| [x, y, z],
                ),
                1.5 * 0.3 * 0.125,
            ),
            (
                "the prism over [1, 3]",
                prism(
                    &pose,
                    "the prism",
                    &[(2.0, 0.0), (2.0, 0.8), (1.2, 0.0)],
                    (1.0, 3.0),
                    &|y, z, x| [x, y, z],
                ),
                1.5 * trench_share,
            ),
            (
                "the prism over [1.5, 2.5]",
                prism(
                    &pose,
                    "the prism",
                    &[(2.0, 0.0), (2.0, 0.8), (1.2, 0.0)],
                    (1.5, 2.5),
                    &|y, z, x| [x, y, z],
                ),
                trench_share,
            ),
            (
                "the mirrored prism over [1, 3]",
                prism(
                    &pose,
                    "the prism",
                    &[(2.0, 0.0), (2.8, 0.0), (2.0, 0.8)],
                    (1.0, 3.0),
                    &|y, z, x| [x, y, z],
                ),
                1.5 * trench_share,
            ),
            (
                "the mirrored prism over [1.5, 2.5]",
                prism(
                    &pose,
                    "the prism",
                    &[(2.0, 0.0), (2.8, 0.0), (2.0, 0.8)],
                    (1.5, 2.5),
                    &|y, z, x| [x, y, z],
                ),
                trench_share,
            ),
        ];
        for (part, x, share) in &parts {
            let what = format!("{}, slit ∪ {part}", pose.label);
            let r = match topo::boolean_op_with(
                topo::BooleanOp::Union,
                &cut,
                x,
                &topo::BooleanInputs::default(),
                topo::SweepStrategy::Realized,
                tol,
            ) {
                Ok(BooleanResult::Body(r)) => r,
                other => panic!("{what}: {other:?}"),
            };
            let b = &r.body;
            assert_eq!(validate(b), Ok(()), "{what}: tier 1");
            assert_eq!(validate_closed(b), Ok(()), "{what}: tier 2");
            assert_eq!(validate_geometric(b, tol), Ok(()), "{what}: tier 3");
            assert_eq!(b.solids().count(), 1, "{what}: solids");
            let v = topo::test_support::pinned_volume("the measured body", b, tol).volume;
            let want = row.volume + share;
            assert!(
                (v - want).abs() < 1e-9 * want,
                "{what}: volume {v}, closed form {want}"
            );
            match validate_pseudomanifold(b, &r.contacts, tol) {
                Err(errs) => assert!(
                    errs.iter().all(|e| matches!(
                        e,
                        topo::ValidationError::UnrecordedContact {
                            contact: topo::CensusContact::EdgeEdgeOverlap { .. },
                            ..
                        }
                    )),
                    "{what}: tier 3′ refuses {errs:?}, not only an unrecorded edge overlap"
                ),
                Ok(()) => panic!(
                    "{what}: tier 3′ passes: the overlap is recorded now, so pin it and close the row"
                ),
            }
        }
    }
}

/// **A keel glued to a face from outside stays one edge with its
/// records**: the trench's V turned up on the slab's top, and turned
/// down under its bottom, each resting its keel on the face. Both of the
/// edge's faces leave the line on the face's empty side, so the edge
/// touches the face from outside its material wedge and is not parted
/// (D1, `crates/topo/README.md` tier 3): ∪ builds two solids, the keel
/// one edge, an edge-on-face record of it on the face, a vertex-on-face
/// record of each end inside the face, and an edge-edge record where it
/// crosses the face's boundary. At every extent and pose.
#[test]
fn a_keel_glued_to_a_face_from_outside_stays_one_edge() {
    let tol = t();
    for (side, z, profile) in [
        ("on the top", 1.0, [(2.0, 1.0), (2.6, 3.0), (1.4, 3.0)]),
        (
            "under the bottom",
            0.0,
            [(2.0, 0.0), (1.4, -2.0), (2.6, -2.0)],
        ),
    ] {
        for row in &ROWS {
            for pose in poses() {
                let what = format!("{side}, {}, {}", row.what, pose.label);
                let v = prism(&pose, "the V", &profile, row.span, &|y, h, x| [x, y, h]);
                let Ok(BooleanResult::Body(r)) = topo::union(&slab(&pose, (0.0, 1.0)), &v, tol)
                else {
                    panic!("{what}: the slab and the V");
                };
                let b = &r.body;
                assert_eq!(b.solids().count(), 2, "{what}: solids");
                let [p, q] = [[0.0, 2.0, z], [4.0, 2.0, z]].map(|c| pose.at(c));
                let on_keel = |v| {
                    let x = topo::readback::vertex_point(b, v).unwrap();
                    let (d, l) = (q - p, x - p);
                    (l - d * (l.dot(d) / d.dot(d))).norm() < 1e-9
                };
                let keel: Vec<EdgeKey> = b
                    .edges()
                    .map(|(e, _)| e)
                    .filter(|&e| {
                        let he = b.get_edge(e).unwrap().he_plus;
                        on_keel(b.get_half_edge(he).unwrap().start)
                            && on_keel(b.half_edge_end(he).unwrap())
                    })
                    .collect();
                let [keel] = keel[..] else {
                    panic!("{what}: the keel is {keel:?}, not one edge");
                };
                let kinds: Vec<_> = r.contacts.rows().iter().map(|r| r.record.kind()).collect();
                let count =
                    |f: &dyn Fn(&topo::ContactKind) -> bool| kinds.iter().filter(|k| f(k)).count();
                assert_eq!(
                    count(&|k| matches!(k, topo::ContactKind::EdgeFace(e, _) if *e == keel)),
                    1,
                    "{what}: the keel's edge-on-face record among {kinds:?}"
                );
                let inside = row.at_ends.iter().filter(|&&n| n == 1).count();
                assert_eq!(
                    count(&|k| matches!(k, topo::ContactKind::VertexFace(..))),
                    inside,
                    "{what}: vertex-on-face records among {kinds:?}"
                );
                assert_eq!(
                    count(
                        &|k| matches!(k, topo::ContactKind::EdgeEdge(_, e) | topo::ContactKind::EdgeEdge(e, _) if *e == keel)
                    ),
                    2 - inside,
                    "{what}: edge-edge records among {kinds:?}"
                );
            }
        }
    }
}

/// **A box flush under a ringed top builds in every op**: the plate less
/// a pyramid lying under its top (`below(lying(0))`) holds the pyramid's
/// ray as a zero-area ring of the top, and the box
/// `[1, 2] × [0.5, 1.5] × [lo, 1]` lies flush under that top round it.
/// The join cuts the box's top along the ring, the sliver between the
/// cuts outside the plate. Where the op keeps the sliver (`x − y`,
/// `y ∪ x`) the ring's two edges merge across it; where it keeps the
/// flush region (`x ∩ y`, `y ∩ x`, `x ∪ y`), one of the two flush tops
/// is dropped and cut where its fate changes, a cut ending at the ring's
/// tip in the full turn the ring leaves there. Each builds, tier 3 and
/// 3′. The pyramid's part in the box `p` is the whole pyramid, `|det|/6`
/// of its corners, where the box holds it, and is read off `x − y`
/// where it does not; every other volume follows from `p` by
/// additivity: `|x| − p` for `x ∩ y` and `y ∩ x`, `6 − pyramid − |x| + p`
/// for `y − x`, and `6 − pyramid + p` for `x ∪ y` and `y ∪ x`. At rest
/// and turned.
#[test]
fn a_box_flush_under_a_ringed_top_builds_in_every_op() {
    use common::meeting::{PLATE, apex_pyramid, lying, posed_box};
    let tol = t();
    let below = |b: [[f64; 3]; 3]| b.map(|[x, y, z]| [x, y, -z]);
    let poses = poses();
    for pose in [&poses[0], &poses[2]] {
        let plate = posed_box("the plate", PLATE, pose, tol);
        let void = apex_pyramid(&below(lying(0.0)), pose, tol);
        let Ok(BooleanResult::Body(y)) = subtract(&plate, &void, tol) else {
            panic!("{}: the plate less the pyramid", pose.label);
        };
        let y = y.body;
        let [a, b, c] = below(lying(0.0));
        let pyramid = (a[0] * (b[1] * c[2] - b[2] * c[1]) - a[1] * (b[0] * c[2] - b[2] * c[0])
            + a[2] * (b[0] * c[1] - b[1] * c[0]))
            .abs()
            / 6.0;
        for (lo, holds) in [(0.0, true), (0.3, true), (0.6, false)] {
            let what = format!("{}, the box from {lo}", pose.label);
            let x = posed_box("the box", [(1.0, 2.0), (0.5, 1.5), (lo, 1.0)], pose, tol);
            let boxed = 1.0 - lo;
            let built = |op: &str, r| match r {
                Ok(BooleanResult::Body(r)) => {
                    let r: topo::BooleanBody<f64> = r;
                    assert_eq!(
                        validate_geometric(&r.body, tol),
                        Ok(()),
                        "{what}: {op}: tier 3"
                    );
                    assert_eq!(
                        validate_pseudomanifold(&r.body, &r.contacts, tol),
                        Ok(()),
                        "{what}: {op}: tier 3′"
                    );
                    topo::test_support::pinned_volume("the measured body", &r.body, tol).volume
                }
                other => panic!("{what}: {op}: {:?}", other.map(|_| ())),
            };
            let part = built("x − y", subtract(&x, &y, tol));
            assert!(
                part > 0.0 && part <= pyramid * (1.0 + 1e-9),
                "{what}: x − y: volume {part}, the pyramid's {pyramid}"
            );
            if holds {
                assert!(
                    (part - pyramid).abs() < 1e-9,
                    "{what}: x − y: volume {part}, the whole pyramid's {pyramid}"
                );
            }
            for (op, r, want) in [
                ("x ∩ y", intersect(&x, &y, tol), boxed - part),
                ("y ∩ x", intersect(&y, &x, tol), boxed - part),
                ("y − x", subtract(&y, &x, tol), 6.0 - pyramid - boxed + part),
                ("x ∪ y", topo::union(&x, &y, tol), 6.0 - pyramid + part),
                ("y ∪ x", topo::union(&y, &x, tol), 6.0 - pyramid + part),
            ] {
                let v = built(op, r);
                assert!(
                    (v - want).abs() < 1e-9,
                    "{what}: {op}: volume {v}, closed form {want}"
                );
            }
        }
    }
}

/// **A flush partner whose outline crosses or meets the slit intersects
/// it in either order**: the floor and the partner's bottom coincide,
/// ∩ keeps one copy, and the dropped one is cut where its holder
/// changes, not along the slit, where the kept face lies on both sides.
/// - the slit form closed and open at one end ∩ `[0, 2] × [0, 4] × [0, 0.5]`,
///   whose edge crosses the keel: `4 − 0.075 · 0.5`;
/// - the slit form open at one end ∩ `[1, 5] × [1, 3] × [0, 0.5]`,
///   overhanging the floor across the keel's open end, whose two
///   vertices touch the partner's bottom at one point: `3 − 0.075 · 2.5`;
/// - the plate less the pyramid lying under its top ∩ boxes flush under
///   the top whose outline crosses the ring, has a corner on its middle
///   (and the mirror box) or an edge through it: both orders alike, and
///   `|x ∩ y| + |x − y| = |x|` (additivity);
/// - the box `[0, 1]³` ∪ the wedge over the `(x, z)` triangle
///   `(1, 1), (2, 0), (2, 1)`, touching along their coplanar tops' edge,
///   ∩ `[−0.5, 2.5] × [−0.5, 1.5] × [0.5, 1]`: `0.5 + 0.375`.
///
/// Each builds, tiers 3 and 3′, at rest, turned and flipped and turned.
#[test]
fn a_flush_partner_crossing_the_slit_intersects_it_in_either_order() {
    use common::meeting::{MEET, PLATE, apex_pyramid, bearing, lying, posed_box};
    let tol = t();
    let built = |what: &str, r: Result<BooleanResult<f64>, topo::BooleanError>| match r {
        Ok(BooleanResult::Body(r)) => {
            assert_eq!(validate_geometric(&r.body, tol), Ok(()), "{what}: tier 3");
            assert_eq!(
                validate_pseudomanifold(&r.body, &r.contacts, tol),
                Ok(()),
                "{what}: tier 3′"
            );
            topo::test_support::pinned_volume("the measured body", &r.body, tol).volume
        }
        other => panic!("{what}: {:?}", other.map(|_| ())),
    };
    let both = |what: &str, a: &AtRestBody<f64>, b: &AtRestBody<f64>, want: Option<f64>| {
        let v = [
            built(&format!("{what}, a ∩ b"), intersect(a, b, tol)),
            built(&format!("{what}, b ∩ a"), intersect(b, a, tol)),
        ];
        assert!(
            (v[0] - v[1]).abs() < 1e-9,
            "{what}: |a ∩ b| {v:?} in the two orders"
        );
        if let Some(want) = want {
            assert!(
                (v[0] - want).abs() < 1e-9,
                "{what}: volume {}, closed form {want}",
                v[0]
            );
        }
        v[0]
    };
    let poses = poses();
    for pose in [&poses[0], &poses[2], &poses[4]] {
        let slab_box = |b: [(f64, f64); 3]| {
            prism(
                pose,
                "the partner",
                &[
                    (b[0].0, b[1].0),
                    (b[0].1, b[1].0),
                    (b[0].1, b[1].1),
                    (b[0].0, b[1].1),
                ],
                b[2],
                &|x, y, z| [x, y, z],
            )
        };
        for (k, partner, want) in [
            (0, [(0.0, 2.0), (0.0, 4.0), (0.0, 0.5)], 4.0 - 0.075 * 0.5),
            (1, [(0.0, 2.0), (0.0, 4.0), (0.0, 0.5)], 4.0 - 0.075 * 0.5),
            (1, [(1.0, 5.0), (1.0, 3.0), (0.0, 0.5)], 3.0 - 0.075 * 2.5),
        ] {
            let [(_, slit), _] = slit_operands(&ROWS[k], pose);
            let what = format!("{}, {}, {partner:?}", ROWS[k].what, pose.label);
            both(&what, &slit, &slab_box(partner), Some(want));
        }
        let below = |b: [[f64; 3]; 3]| b.map(|[x, y, z]| [x, y, -z]);
        let plate = posed_box("the plate", PLATE, pose, tol);
        let void = apex_pyramid(&below(lying(0.0)), pose, tol);
        let Ok(BooleanResult::Body(y)) = subtract(&plate, &void, tol) else {
            panic!("{}: the plate less the pyramid", pose.label);
        };
        let y = y.body;
        let mid = [0, 1, 2].map(|k| MEET[k] + bearing(60.0, 0.25, 0.0)[k]);
        for bx in [
            [(1.0, 1.65), (0.5, 1.5), (0.3, 1.0)],
            [(1.0, 2.0), (0.5, 1.2), (0.3, 1.0)],
            [(mid[0], 2.2), (mid[1], 1.8), (0.3, 1.0)],
            [(1.0, mid[0]), (0.5, mid[1]), (0.3, 1.0)],
            [(mid[0], 2.0), (0.5, 1.5), (0.3, 1.0)],
        ] {
            let what = format!("the ringed plate, {}, the box {bx:?}", pose.label);
            let x = posed_box("the box", bx, pose, tol);
            let i = both(&what, &y, &x, None);
            let rest = built(&format!("{what}, x − y"), subtract(&x, &y, tol));
            let vx = (bx[0].1 - bx[0].0) * (bx[1].1 - bx[1].0) * (bx[2].1 - bx[2].0);
            assert!(
                (i + rest - vx).abs() < 1e-9,
                "{what}: |x ∩ y| {i} + |x − y| {rest} = |x| {vx}"
            );
        }
        let cube = posed_box("the box", [(0.0, 1.0), (0.0, 1.0), (0.0, 1.0)], pose, tol);
        let wedge = prism(
            pose,
            "the wedge",
            &[(1.0, 2.0), (0.0, 2.0), (1.0, 1.0)],
            (0.0, 1.0),
            &|z, x, y| [x, y, z],
        );
        let Ok(BooleanResult::Body(lumps)) = topo::union(&cube, &wedge, tol) else {
            panic!("{}: the box and the wedge", pose.label);
        };
        let lumps = lumps.body;
        let partner = posed_box(
            "the partner",
            [(-0.5, 2.5), (-0.5, 1.5), (0.5, 1.0)],
            pose,
            tol,
        );
        let what = format!("the two lumps, {}", pose.label);
        both(&what, &lumps, &partner, Some(0.875));
    }
}

/// **A face closed across part of the slit refuses typed.** The slab
/// less the trench closed at both ends, from either origin, carries the
/// keel as a ring of its floor. A box under and around the floor less a
/// wide vee along the middle of the keel, `(1.7, 2.3)` of its
/// `(1.5, 2.5)`, less the slab keeps the floor on both sides of that
/// stretch of the keel and the walls beyond it: the zip glues the floor
/// to itself there, a strut whose ends both run on into the ring's
/// edges, and the closing deletes it, parting the ring in two
/// (`slit::part_mid_slit`). Beyond the stretch the floor continues
/// across the line, a half-turn, and the trench's walls crease the
/// narrow vee above it, one edge, but the ring's two edges stay there,
/// floor | wall each, their wedges overlapping: `x − y` refuses
/// `LineWedges` `Overlap` at the result gate, a construction gap
/// (`work/cleave/a-difference-whose-partner-ends-inside-a-slit-overlaps-its-wedges.md`).
#[test]
fn a_face_closed_across_part_of_the_slit_refuses_typed() {
    let tol = t();
    let pose = &poses()[0];
    let vee = prism(
        pose,
        "the wide vee",
        &[(2.0, 0.0), (2.9, 2.0), (1.1, 2.0)],
        (1.7, 2.3),
        &|y, z, x| [x, y, z],
    );
    let around = prism(
        pose,
        "the box",
        &[(-1.0, -1.0), (5.0, -1.0), (5.0, 5.0), (-1.0, 5.0)],
        (-1.0, 0.5),
        &|x, y, z| [x, y, z],
    );
    let Ok(BooleanResult::Body(x)) = subtract(&around, &vee, tol) else {
        panic!("the box less the wide vee");
    };
    for (origin, y) in slit_operands(&ROWS[0], pose) {
        let what = format!("{origin}, {}", pose.label);
        match subtract(&x.body, &y, tol) {
            Err(e) => assert_eq!(
                e.kind(),
                topo::BooleanErrorKind::LineWedges,
                "{what}: x − y: {e:?}"
            ),
            Ok(_) => panic!("{what}: x − y builds, where it refuses typed"),
        }
    }
}

/// **A flush partner over two lumps touching along an edge builds in
/// every op, either order.** The box `[0, 1]³` ∪ a second lump touching
/// it along the edge `x = 1, z = 1`: the box `[1, 2] × [0, 1] × [1, 2]`
/// (an S, opposite faces across the line), or the wedge over the
/// `(x, z)` triangle `(1, 1), (2, 1), (2, 2)` (above it, its bottom
/// coplanar with the box's top), or `(1, 1), (2, 0), (2, 1)` or
/// `(1, 1), (1.3, 0), (1.3, 1)` (beside it, the tops coplanar), united
/// in either order. The partner `[−0.5, 2.5] × [−0.5, 1.5]` over
/// `[1, 1.5]` or `[0.5, 1]` lies flush with a face of each lump on the
/// line's plane, so the lumps' corners pierce its face at one point and
/// their sections run along the line from there, two cones touching
/// along a ray in the face. Where a join closes the corner of no angle
/// between the two, it winds as the ring's order places them apart
/// (`join::tie_winding`). Beside the steep wedge, with the partner above,
/// both of that run's arcs capture cleanly, so no capture rank orders it
/// either (`join::best_arc`'s order refuses there). Where the result
/// keeps the sliver between the two cones' sections as a face beside the
/// kept lump's flush face, the two plug the partner face's hole together,
/// and the merge keeps the partner's face (`outermost_survivor`); where
/// the op keeps the partner's face over one
/// lump, the dropped copy's corner at the lumps' shared point is held by
/// the piece of the face whose corner there holds its direction
/// (`fate::piece_holding`).
///
/// Every op builds in both orders as the closed form says, tier 3 and
/// 3′, at rest, turned and flipped and turned, with one edge along the
/// line per sector of the result's material about it that is not flat
/// (two where the result is two lumps touching there).
#[test]
fn a_flush_partner_over_two_lumps_touching_along_an_edge_builds() {
    use common::meeting::posed_box;
    let tol = t();
    let poses = poses();
    let within = |lo: f64, hi: f64| move |a: f64| (lo..hi).contains(&a);
    for pose in [&poses[0], &poses[2], &poses[4]] {
        let cube = posed_box("the box", [(0.0, 1.0), (0.0, 1.0), (0.0, 1.0)], pose, tol);
        // Each second lump's `(z, x)` profile, counterclockwise, its
        // volume, its volume above `z = 1` up to 1.5 and below it down
        // to 0.5, and its material about the line.
        let lumps = [
            (
                "the S",
                &[(2.0, 1.0), (2.0, 2.0), (1.0, 2.0), (1.0, 1.0)][..],
                1.0,
                [0.5, 0.0],
                within(0.0, 90.0),
            ),
            (
                "the wedge above",
                &[(2.0, 2.0), (1.0, 2.0), (1.0, 1.0)][..],
                0.5,
                [0.5 - 0.125, 0.0],
                within(0.0, 45.0),
            ),
            (
                "the wedge beside",
                &[(1.0, 2.0), (0.0, 2.0), (1.0, 1.0)][..],
                0.5,
                [0.0, 0.375],
                within(315.0, 360.0),
            ),
            (
                "the steep wedge beside",
                &[(1.0, 1.3), (0.0, 1.3), (1.0, 1.0)][..],
                0.15,
                [0.0, 0.3 * 0.375],
                within(360.0 - 1.0f64.atan2(0.3).to_degrees(), 360.0),
            ),
        ];
        for (lump, profile, whole, part, about) in lumps {
            let second = prism(pose, lump, profile, (0.0, 1.0), &|z, x, y| [x, y, z]);
            for (order, x) in [
                ("the box first", topo::union(&cube, &second, tol)),
                ("the lump first", topo::union(&second, &cube, tol)),
            ] {
                let Ok(BooleanResult::Body(x)) = x else {
                    panic!("{}: the box and {lump}, {order}", pose.label);
                };
                let x = x.body;
                let in_x = |a: f64| within(180.0, 270.0)(a) || about(a);
                for (side, z, vi, in_y) in [
                    ("above", (1.0, 1.5), part[0], within(0.0, 180.0)),
                    ("below", (0.5, 1.0), 0.5 + part[1], within(180.0, 360.0)),
                ] {
                    let what = format!("{lump}, {order}, the partner {side}, {}", pose.label);
                    let y = posed_box("the partner", [(-0.5, 2.5), (-0.5, 1.5), z], pose, tol);
                    let line = [[1.0, 0.0, 1.0], [1.0, 1.0, 1.0]];
                    every_op_builds(
                        &what,
                        pose,
                        (&x, &y),
                        (1.0 + whole, 3.0, vi),
                        (&in_x, &in_y),
                        (line, 1.0),
                    );
                }
            }
        }
    }
}

/// **Every op of `x` and `y` builds as the closed form says**, in both
/// orders: tier 3 and 3′, the volume from `|x|`, `|y|` and `|x ∩ y|`
/// (`v`), and one edge along `line` (from its first point toward its
/// second, a unit apart, for `reach`) per sector
/// of the result's material about it that is neither empty, whole nor
/// flat. The material is read by angle in degrees about the line, the
/// operands' own as `inside`; an op whose closed form is empty may
/// return [`BooleanResult::Empty`].
fn every_op_builds(
    what: &str,
    pose: &Pose,
    (x, y): (&AtRestBody<f64>, &AtRestBody<f64>),
    (vx, vy, vi): (f64, f64, f64),
    inside: (&dyn Fn(f64) -> bool, &dyn Fn(f64) -> bool),
    (line, reach): ([[f64; 3]; 2], f64),
) {
    let tol = t();
    let creased = |kept: &dyn Fn(f64) -> bool| {
        let held: Vec<bool> = (0..360).map(|k| kept(f64::from(k) + 0.5)).collect();
        let Some(start) = (0..360).find(|&k| !held[k]) else {
            return 0;
        };
        let mut runs = vec![0usize];
        for k in 1..=360 {
            if held[(start + k) % 360] {
                *runs.last_mut().unwrap() += 1;
            } else {
                runs.push(0);
            }
        }
        runs.into_iter().filter(|&n| n > 0 && n != 180).count()
    };
    // An op, its result, its closed-form volume, and whether it keeps a
    // point inside `x` and `y` or not.
    type Op = (
        &'static str,
        Result<BooleanResult<f64>, topo::BooleanError>,
        f64,
        fn(bool, bool) -> bool,
    );
    let ops: [Op; 6] = [
        ("x − y", subtract(x, y, tol), vx - vi, |a, b| a && !b),
        ("y − x", subtract(y, x, tol), vy - vi, |a, b| b && !a),
        ("x ∩ y", intersect(x, y, tol), vi, |a, b| a && b),
        ("y ∩ x", intersect(y, x, tol), vi, |a, b| a && b),
        ("x ∪ y", topo::union(x, y, tol), vx + vy - vi, |a, b| {
            a || b
        }),
        ("y ∪ x", topo::union(y, x, tol), vx + vy - vi, |a, b| {
            a || b
        }),
    ];
    for (op, r, want, kept) in ops {
        let what = format!("{what}: {op}");
        let r = match r {
            Ok(BooleanResult::Body(r)) => r,
            Ok(BooleanResult::Empty) if want == 0.0 => continue,
            other => panic!("{what}: builds, got {:?}", other.map(|_| ())),
        };
        assert_eq!(validate_geometric(&r.body, tol), Ok(()), "{what}: tier 3");
        assert_eq!(
            validate_pseudomanifold(&r.body, &r.contacts, tol),
            Ok(()),
            "{what}: tier 3′"
        );
        let v = topo::test_support::pinned_volume("the measured body", &r.body, tol).volume;
        assert!(
            (v - want).abs() < 1e-9,
            "{what}: volume {v}, closed form {want}"
        );
        let along = spans_along(&r.body, pose, line)
            .into_iter()
            .filter(|&(a, b)| a > -1e-9 && b < reach + 1e-9)
            .count();
        assert_eq!(
            along,
            creased(&|a| kept(inside.0(a), inside.1(a))),
            "{what}: an edge along the lumps' line per creased sector about it"
        );
    }
}

/// **A flush partner over two lumps touching along a diagonal builds in
/// every op, either order.** The prism over the `(x, y)` triangle
/// `(0, 0), (1, 0), (1, 1)` on `z ∈ [0, 1]` ∪ the one over
/// `(0, 0), (1, 1), (0, 1)` on `z ∈ [1, 2]`, united in either order,
/// touch along the segment `(0, 0, 1)`–`(1, 1, 1)`, a line the two
/// lumps' faces run along at no axis. The partner
/// `[−0.5, 1.5]² × [1, 1.5]` or `× [0.5, 1]` lies flush with a face of
/// each. The union keeps the sliver between the two cones' sections as a
/// face beside the kept lump's flush face, the two together a plug in
/// the partner face's hole, and the merge keeps the partner's face
/// (`merge_faces::outermost_survivor`), where it refused `Merge` when it
/// read only the face the ring borders as nested. Every op builds as the
/// closed form says, tier 3 and 3′, at rest, turned and flipped and
/// turned, with an edge along the line per creased sector.
#[test]
fn a_flush_partner_over_two_lumps_touching_along_a_diagonal_builds() {
    use common::meeting::posed_box;
    let tol = t();
    let poses = poses();
    let within = |lo: f64, hi: f64| move |a: f64| (lo..hi).contains(&a);
    let r = std::f64::consts::FRAC_1_SQRT_2;
    // Angles about the line from the side `y > x`, in the plane across
    // it: the high lump holds `[0, 90]`, the low one `[180, 270]`.
    let in_x = |a: f64| within(0.0, 90.0)(a) || within(180.0, 270.0)(a);
    for pose in [&poses[0], &poses[2], &poses[4]] {
        let lumps = [
            (0.0, 1.0, &[(0.0, 0.0), (1.0, 0.0), (1.0, 1.0)][..]),
            (1.0, 2.0, &[(0.0, 0.0), (1.0, 1.0), (0.0, 1.0)][..]),
        ]
        .map(|(lo, hi, profile)| prism(pose, "a lump", profile, (lo, hi), &|x, y, z| [x, y, z]));
        for (order, x) in [
            ("the low lump first", topo::union(&lumps[0], &lumps[1], tol)),
            (
                "the high lump first",
                topo::union(&lumps[1], &lumps[0], tol),
            ),
        ] {
            let Ok(BooleanResult::Body(x)) = x else {
                panic!("{}: the two lumps, {order}", pose.label);
            };
            let x = x.body;
            for (side, z, in_y) in [
                ("above", (1.0, 1.5), within(0.0, 180.0)),
                ("below", (0.5, 1.0), within(180.0, 360.0)),
            ] {
                let what = format!(
                    "the diagonal lumps, {order}, the partner {side}, {}",
                    pose.label
                );
                let y = posed_box("the partner", [(-0.5, 1.5), (-0.5, 1.5), z], pose, tol);
                let line = [[0.0, 0.0, 1.0], [r, r, 1.0]];
                every_op_builds(
                    &what,
                    pose,
                    (&x, &y),
                    (1.0, 2.0, 0.25),
                    (&in_x, &in_y),
                    (line, 2.0f64.sqrt()),
                );
            }
        }
    }
}

/// A lump: a prism over a counterclockwise `(x, y)` polygon, over `z`.
type Lump = (&'static [(f64, f64)], (f64, f64));

/// The cube `[0, 1]³`.
const CUBE: Lump = (
    &[(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)],
    (0.0, 1.0),
);

/// The cube and the box `[1, 2]² × [0.5, 1.5]`, touching along
/// `x = y = 1, z ∈ [0.5, 1]`.
const BESIDE: [Lump; 2] = [
    CUBE,
    (
        &[(1.0, 1.0), (2.0, 1.0), (2.0, 2.0), (1.0, 2.0)],
        (0.5, 1.5),
    ),
];

/// The cube and two prisms over `z ∈ [0.5, 1.5]`, each touching it along
/// `x = y = 1, z ∈ [0.5, 1]`, apart across the diagonal: their imprints
/// on `z = 1` and the gap between them make three pieces of a partner's
/// face beside the cube's corner there.
const THREE_PIECES: [Lump; 3] = [
    CUBE,
    (&[(1.0, 1.0), (2.0, 1.0), (2.0, 1.8)], (0.5, 1.5)),
    (&[(1.0, 1.0), (1.8, 2.0), (1.0, 2.0)], (0.5, 1.5)),
];

/// The volume of `lump` inside the box `b`: its polygon clipped to the
/// box's rectangle, by the overlap of their spans in z.
fn lump_in_box((polygon, z): Lump, b: [(f64, f64); 3]) -> f64 {
    let mut poly: Vec<(f64, f64)> = polygon.to_vec();
    // Each half-plane `s · (p[k] − c) ≤ 0` of the rectangle.
    for (k, c, s) in [
        (0, b[0].1, 1.0),
        (0, b[0].0, -1.0),
        (1, b[1].1, 1.0),
        (1, b[1].0, -1.0),
    ] {
        let at = |p: (f64, f64)| s * ([p.0, p.1][k] - c);
        let mut out = Vec::new();
        for i in 0..poly.len() {
            let (p, q) = (poly[i], poly[(i + 1) % poly.len()]);
            if at(p) <= 0.0 {
                out.push(p);
            }
            if (at(p) < 0.0) != (at(q) < 0.0) && at(p) != 0.0 && at(q) != 0.0 {
                let u = at(p) / (at(p) - at(q));
                out.push((p.0 + u * (q.0 - p.0), p.1 + u * (q.1 - p.1)));
            }
        }
        poly = out;
    }
    let area: f64 = (0..poly.len())
        .map(|i| {
            let (p, q) = (poly[i], poly[(i + 1) % poly.len()]);
            p.0 * q.1 - q.0 * p.1
        })
        .sum::<f64>()
        / 2.0;
    area * (z.1.min(b[2].1) - z.0.max(b[2].0)).max(0.0)
}

/// `lumps` united in order at `pose`, with the records each step's
/// result carries into the next op.
fn lumps_united(lumps: &[Lump], pose: &Pose) -> (AtRestBody<f64>, topo::OperandRecords) {
    let tol = t();
    let body = |(polygon, z): Lump| prism(pose, "a lump", polygon, z, &|x, y, z| [x, y, z]);
    let (first, rest) = lumps.split_first().expect("a lump");
    let mut acc = (body(*first), topo::OperandRecords::default());
    for &lump in rest {
        let inputs = topo::BooleanInputs {
            records_a: acc.1,
            ..topo::BooleanInputs::default()
        };
        let r = topo::boolean_op_with(
            topo::BooleanOp::Union,
            &acc.0,
            &body(lump),
            &inputs,
            topo::SweepStrategy::Realized,
            tol,
        );
        let Ok(BooleanResult::Body(r)) = r else {
            panic!("{}: the lumps unite, got {:?}", pose.label, r.map(|_| ()));
        };
        acc = (r.body, r.contacts.carried());
    }
    acc
}

/// `x op y`, or `y op x` where `partner_first`, `x` the lumps' union
/// with its records carried and `y` the box `partner`, builds tier 3 and
/// 3′ at the volume the lumps' overlaps with it give, each substituted
/// fragment held by a face of the result, which holds the points just
/// inside `corners` its dropped face does ([`held_by_kept_faces`]); the
/// count of those read.
fn builds_against(
    what: &str,
    pose: &Pose,
    (lumps, (x, records)): (&[Lump], &(AtRestBody<f64>, topo::OperandRecords)),
    (partner, corners): ([(f64, f64); 3], &[Corner]),
    (op, partner_first): (topo::BooleanOp, bool),
) -> usize {
    let tol = t();
    let y = common::meeting::posed_box("the partner", partner, pose, tol);
    let total: f64 = lumps
        .iter()
        .map(|&l| lump_in_box(l, [(-9.0, 9.0); 3]))
        .sum();
    let common: f64 = lumps.iter().map(|&l| lump_in_box(l, partner)).sum();
    let whole: f64 = partner.iter().map(|(lo, hi)| hi - lo).product();
    let want = match (op, partner_first) {
        (topo::BooleanOp::Subtract, false) => total - common,
        (topo::BooleanOp::Subtract, true) => whole - common,
        (topo::BooleanOp::Intersect, _) => common,
        (topo::BooleanOp::Union, _) => total + whole - common,
    };
    let (records_a, records_b) = if partner_first {
        (topo::OperandRecords::default(), records.clone())
    } else {
        (records.clone(), topo::OperandRecords::default())
    };
    let inputs = topo::BooleanInputs {
        records_a,
        records_b,
    };
    let (a, b) = if partner_first { (&y, x) } else { (x, &y) };
    let r = topo::boolean_op_with(op, a, b, &inputs, topo::SweepStrategy::Realized, tol);
    let Ok(BooleanResult::Body(r)) = r else {
        panic!("{what}: builds, got {:?}", r.map(|_| ()));
    };
    assert_eq!(validate_geometric(&r.body, tol), Ok(()), "{what}: tier 3");
    assert_eq!(
        validate_pseudomanifold(&r.body, &r.contacts, tol),
        Ok(()),
        "{what}: tier 3′"
    );
    let v = topo::test_support::pinned_volume("the measured body", &r.body, tol).volume;
    assert!(
        (v - want).abs() < 1e-9,
        "{what}: volume {v}, closed form {want}"
    );
    let inside: Vec<_> = corners
        .iter()
        .map(|&(p, [u, v])| {
            let s = 1e-2 / u.hypot(v);
            pose.at([p[0] + s * u, p[1] + s * v, p[2]])
        })
        .collect();
    let operands = if partner_first { [&y, x] } else { [x, &y] };
    held_by_kept_faces(what, &r, operands, &inside)
}

/// Each fragment of a dropped coincident face the result substitutes is
/// held by a face the result holds: the kept face its row names, or the
/// face the output stage merged it into. Where the fragment's face in
/// the dropped operand holds one of `inside` (points just inside the
/// dropped corners the cut reads), the kept face holds that point too.
/// The count of such points read.
fn held_by_kept_faces(
    what: &str,
    r: &topo::BooleanBody<f64>,
    [kept_operand, dropped]: [&AtRestBody<f64>; 2],
    inside: &[geom_core::Point3<f64>],
) -> usize {
    use topo::test_support::point_in_face;
    use topo::{Cell, Operand, OperandKeys};
    let band = geom_core::Band::linear(t()).unwrap();
    let n = &r.naming;
    assert!(
        matches!(n.a_keys, OperandKeys::Direct),
        "{what}: the result is operand A's clone"
    );
    let mut read = 0;
    for row in &n.substituted {
        // The lump keeps operand A's copy of a coincident pair.
        assert_eq!(row.operand, Operand::B, "{what}: a fragment of B dropped");
        let settled = n.substitutions.chase(
            Cell::Face(row.by),
            |c| matches!(c, Cell::Face(f) if r.body.get_face(f).is_some()),
        );
        let Ok(Cell::Face(kept)) = settled else {
            panic!(
                "{what}: the kept face {:?} holding the dropped fragment {:?} is in the \
                 result, got {settled:?}",
                row.by, row.face
            );
        };
        let roots = [
            (kept_operand, n.face_fragments_a.root(row.by)),
            (dropped, n.face_fragments_b.root(row.face)),
        ];
        for &q in inside {
            if roots
                .iter()
                .all(|&(b, f)| matches!(point_in_face(b, f, q, band), Ok(Some(true))))
            {
                read += 1;
                assert!(
                    matches!(point_in_face(&r.body, kept, q, band), Ok(Some(true))),
                    "{what}: the face {kept:?} holding the dropped fragment {:?} holds {q:?}",
                    row.face
                );
            }
        }
    }
    read
}

/// **A fate edge touching a corner of its face's division builds.** A
/// partner flush with the cube's top on `z = 1`, over [`BESIDE`] or
/// [`THREE_PIECES`], whose other lumps cross it: the partner's face there
/// is divided by their imprints, with a vertex at the cube's top corner
/// `(1, 1, 1)`: the ring vertex, and its copies, of the other lumps'
/// pierce there. In `x ∖ y` with the partner above and `x ∩ y` with it
/// below, the cube's top edges are fate edges of that face ending at the
/// corner, which no record pairs with that ring (the cube's corner is
/// recorded on the face, not in the ring): each chord ends at the
/// division's vertex there, its corner read by its direction
/// (`fate::place`). The right one of those vertices sorts last beside
/// one box and not last beside the two prisms. One cell of each, at rest.
#[test]
fn a_fate_edge_touching_a_corner_of_its_faces_division_builds_at_rest() {
    let pose = &poses()[0];
    for (lumps, partner, op, what) in [
        (
            &BESIDE[..],
            [(-0.5, 2.5), (-0.5, 2.5), (1.0, 1.5)],
            topo::BooleanOp::Subtract,
            "beside one box, the partner above: x ∖ y",
        ),
        (
            &THREE_PIECES[..],
            [(-0.5, 2.5), (-0.5, 2.5), (0.5, 1.0)],
            topo::BooleanOp::Intersect,
            "three pieces, the partner below: x ∩ y",
        ),
    ] {
        let x = lumps_united(lumps, pose);
        builds_against(what, pose, (lumps, &x), (partner, &[]), (op, false));
    }
}

/// **The touching corner's family builds in every pose and order.** The
/// lumps of [`a_fate_edge_touching_a_corner_of_its_faces_division_builds_at_rest`]
/// united in either order, at rest and flipped and turned, against a
/// partner over `[1, 1.5]`, `[0.5, 1]` or `[0.5, 1.5]` in z, over
/// `[−0.5, 2.5]²`, `[−0.5, 2.5] × [0.5, 1.5]` or `[0.5, 1.5]²`. Of the
/// 18 cells of each lumps, pose and order, 9 reach the arm that ends a
/// touching cut at a division vertex: `x ∖ y` with the partner above,
/// and `x ∩ y` with it below or across `z = 1`, at each footprint. The
/// other 9 do not reach it and are pinned beside them.
#[test]
fn a_fate_edge_touching_a_corner_of_its_faces_division_builds() {
    let poses = poses();
    for pose in [&poses[0], &poses[4]] {
        for lumps in [&BESIDE[..], &THREE_PIECES[..]] {
            for order in ["cube first", "cube last"] {
                let mut ordered = lumps.to_vec();
                if order == "cube last" {
                    ordered.reverse();
                }
                let x = lumps_united(&ordered, pose);
                for z in [(1.0, 1.5), (0.5, 1.0), (0.5, 1.5)] {
                    for (x_span, y_span) in [
                        ((-0.5, 2.5), (-0.5, 2.5)),
                        ((-0.5, 2.5), (0.5, 1.5)),
                        ((0.5, 1.5), (0.5, 1.5)),
                    ] {
                        let partner = [x_span, y_span, z];
                        for (op, kind) in [
                            ("x ∖ y", topo::BooleanOp::Subtract),
                            ("x ∩ y", topo::BooleanOp::Intersect),
                        ] {
                            let what = format!(
                                "{} lumps, {}, {order}, the partner {partner:?}: {op}",
                                lumps.len(),
                                pose.label
                            );
                            let at = (partner, &[][..]);
                            builds_against(&what, pose, (lumps, &x), at, (kind, false));
                        }
                    }
                }
            }
        }
    }
}

/// The cube and the box `[1, 2] × [0.5, 1.5] × [1, 2]`, touching along
/// `x = 1, z = 1, y ∈ [0.5, 1]`: the box's corner `(1, 0.5, 1)` lies
/// inside the cube's top edge on `x = 1`, and the cube's corner
/// `(1, 1, 1)` inside the box's bottom edge there.
const OVERLAPPING_EDGES: [Lump; 2] = [
    CUBE,
    (
        &[(1.0, 0.5), (2.0, 0.5), (2.0, 1.5), (1.0, 1.5)],
        (1.0, 2.0),
    ),
];

/// A lump's corner on `z = 1` inside the other lump's edge there, and
/// its sector's direction in that plane, `(x, y)`.
type Corner = ([f64; 3], [f64; 2]);

/// The corners of [`OVERLAPPING_EDGES`] inside each other's edge.
const OVERLAPPING_CORNERS: [Corner; 2] = [
    ([1.0, 1.0, 1.0], [-1.0, -1.0]),
    ([1.0, 0.5, 1.0], [1.0, 1.0]),
];

/// The cube and the box `[1, 2] × [0.3, 0.7] × [1, 2]`, its bottom edge
/// on `x = 1` inside the cube's top edge there.
const AN_EDGE_INSIDE_THE_CUBES: [Lump; 2] = [
    CUBE,
    (
        &[(1.0, 0.3), (2.0, 0.3), (2.0, 0.7), (1.0, 0.7)],
        (1.0, 2.0),
    ),
];

/// The box's corners of [`AN_EDGE_INSIDE_THE_CUBES`], inside the cube's
/// edge.
const THE_BOXS_CORNERS: [Corner; 2] = [
    ([1.0, 0.3, 1.0], [1.0, 1.0]),
    ([1.0, 0.7, 1.0], [1.0, -1.0]),
];

/// The cube and the box `[1, 2] × [0, 2] × [1, 2]`, the cube's top edge
/// on `x = 1` inside its bottom edge there.
const THE_CUBES_EDGE_INSIDE: [Lump; 2] = [
    CUBE,
    (
        &[(1.0, 0.0), (2.0, 0.0), (2.0, 2.0), (1.0, 2.0)],
        (1.0, 2.0),
    ),
];

/// The cube's corners of [`THE_CUBES_EDGE_INSIDE`], inside the box's
/// edge.
const THE_CUBES_CORNERS: [Corner; 2] = [
    ([1.0, 0.0, 1.0], [-1.0, 1.0]),
    ([1.0, 1.0, 1.0], [-1.0, -1.0]),
];

/// **A fate corner inside an edge of its kept face's division builds.**
/// A partner flush on `z = 1` with the cube's top and the bottom of the
/// box of [`OVERLAPPING_EDGES`]: the corner of one lump that lies inside
/// the other's edge pierces the partner's face, which the other lump's
/// imprint divides, inside an edge of that division and at no vertex of
/// it. In `y ∖ x` with the partner above and `y ∪ x` with it below, the
/// partner's face keeps that corner's sector, and the piece holding it
/// is the one on whose side of that edge its direction lies
/// (`fate::piece_beside`). One cell of each, at rest.
#[test]
fn a_fate_corner_inside_an_edge_of_its_faces_division_builds_at_rest() {
    let pose = &poses()[0];
    let x = lumps_united(&OVERLAPPING_EDGES, pose);
    for (z, op, what) in [
        (
            (1.0, 1.5),
            topo::BooleanOp::Subtract,
            "the partner above: y ∖ x",
        ),
        (
            (0.5, 1.0),
            topo::BooleanOp::Union,
            "the partner below: y ∪ x",
        ),
    ] {
        let at = ([(-0.5, 2.5), (-0.5, 2.5), z], &OVERLAPPING_CORNERS[..]);
        let read = builds_against(what, pose, (&OVERLAPPING_EDGES, &x), at, (op, true));
        assert!(read > 0, "{what}: a dropped corner's kept face is read");
    }
}

/// Whether a cell reaches the arm, by whether its footprint is wide, its
/// op a difference and its partner above.
type Reaches = fn(bool, bool, bool) -> bool;

/// **The fate corner inside an edge of its kept face's division builds
/// in every pose and order.** The lumps of [`OVERLAPPING_EDGES`],
/// [`AN_EDGE_INSIDE_THE_CUBES`] and [`THE_CUBES_EDGE_INSIDE`], united in
/// either order, at every pose, against a partner over `[1, 1.5]` or
/// `[0.5, 1]` in z, over `[−0.5, 2.5]²`, `[−0.5, 2.5] × [−0.5, 1.5]`,
/// `[−0.5, 2.5] × [0.5, 1.5]` or `[0.5, 1.5]²`: `y ∖ x` and `y ∪ x`
/// build as the closed form says, tier 3 and 3′. Of the 48 cells of each
/// pose and order, 28 reach the arm that reads a corner inside an edge of
/// the division: with [`OVERLAPPING_EDGES`] both ops over the two wide
/// footprints, else `y ∖ x` above and `y ∪ x` below; with
/// [`THE_CUBES_EDGE_INSIDE`] `y ∖ x` above and `y ∪ x` below, and with
/// [`AN_EDGE_INSIDE_THE_CUBES`] `y ∪ x` above and `y ∖ x` below, at every
/// footprint; in each of those the kept face the result names for the
/// dropped corner holds a point just inside it. The other 20 do not
/// reach it and are pinned beside them.
#[test]
fn a_fate_corner_inside_an_edge_of_its_faces_division_builds() {
    for pose in &poses() {
        // Each lumps' corners, and which cells reach the arm.
        let lump_sets: [(&str, &[Lump], [Corner; 2], Reaches); 3] = [
            (
                "overlapping edges",
                &OVERLAPPING_EDGES,
                OVERLAPPING_CORNERS,
                |wide, minus, above| wide || minus == above,
            ),
            (
                "an edge inside the cube's",
                &AN_EDGE_INSIDE_THE_CUBES,
                THE_BOXS_CORNERS,
                |_, minus, above| minus != above,
            ),
            (
                "the cube's edge inside",
                &THE_CUBES_EDGE_INSIDE,
                THE_CUBES_CORNERS,
                |_, minus, above| minus == above,
            ),
        ];
        for (name, lumps, corners, reaches) in lump_sets {
            for order in ["cube first", "cube last"] {
                let mut ordered = lumps.to_vec();
                if order == "cube last" {
                    ordered.reverse();
                }
                let x = lumps_united(&ordered, pose);
                for (z, above) in [((1.0, 1.5), true), ((0.5, 1.0), false)] {
                    for (k, (x_span, y_span)) in [
                        ((-0.5, 2.5), (-0.5, 2.5)),
                        ((-0.5, 2.5), (-0.5, 1.5)),
                        ((-0.5, 2.5), (0.5, 1.5)),
                        ((0.5, 1.5), (0.5, 1.5)),
                    ]
                    .into_iter()
                    .enumerate()
                    {
                        let partner = [x_span, y_span, z];
                        for (op, cell) in [
                            ("y ∖ x", (topo::BooleanOp::Subtract, true)),
                            ("y ∪ x", (topo::BooleanOp::Union, true)),
                        ] {
                            let what = format!(
                                "{name}, {}, {order}, the partner {partner:?}: {op}",
                                pose.label
                            );
                            let read =
                                builds_against(&what, pose, (lumps, &x), (partner, &corners), cell);
                            let minus = cell.0 == topo::BooleanOp::Subtract;
                            if reaches(k < 2, minus, above) {
                                assert!(read > 0, "{what}: the dropped corner's kept face is read");
                            }
                        }
                    }
                }
            }
        }
    }
}

/// **A partner along part of the keel unites with the slit in either
/// order.** The slit form closed at both ends or open at one, from
/// either origin, ∪ a wide vee standing on the floor along part of the
/// keel, `(1.7, 2.3)` or `(2.2, 3.4)`: the vee fills the trench about its
/// stretch, so the union keeps the floor on both sides of the keel
/// there and the zip leaves the stretch an edge of the floor on both
/// sides, between stretches of the keel the union keeps. The closing
/// deletes it, parting the keel's ring or slit (`slit::part_mid_slit`):
/// no edge lies on the keel inside the vee's stretch, and the keel's two
/// edges lie beside it. Both orders build, tier 3 and 3′, as the closed
/// form says
/// (`work/cleave/a-partner-along-part-of-a-slit-refuses-result-invalid.md`).
#[test]
fn a_partner_along_part_of_the_keel_unites_with_the_slit_in_either_order() {
    let tol = t();
    let poses = poses();
    let volume =
        |b: &AtRestBody<f64>| topo::test_support::pinned_volume("the measured body", b, tol).volume;
    for pose in [&poses[0], &poses[2]] {
        for (row, stretch, beside) in [
            (&ROWS[0], (1.7, 2.3), 1.6),
            (&ROWS[0], (2.2, 3.4), 1.8),
            (&ROWS[1], (1.7, 2.3), 2.8),
            (&ROWS[1], (2.2, 3.4), 3.7),
        ] {
            let vee = prism(
                pose,
                "the wide vee",
                &[(2.0, 0.0), (2.9, 2.0), (1.1, 2.0)],
                stretch,
                &|y, z, x| [x, y, z],
            );
            let inside = (stretch.0 + stretch.1.min(row.span.1)) / 2.0;
            // The wide vee's section is 1.8, 0.45 of it under the floor's
            // top at z = 1, where the trench's 0.3 lies inside it along
            // their common stretch in the slab: |x ∪ y| = |x| + |y| − |x ∩ y|.
            let common = (stretch.1.min(row.span.1).min(4.0) - stretch.0.max(row.span.0)).max(0.0);
            let length = stretch.1 - stretch.0;
            let want = 1.8 * length + row.volume - (0.45 * length - 0.3 * common);
            for (origin, y) in slit_operands(row, pose) {
                let what = format!(
                    "{}, {origin}, the vee {stretch:?}, {}",
                    row.what, pose.label
                );
                let mut unions = Vec::new();
                for (op, r) in [
                    ("x ∪ y", topo::union(&vee, &y, tol)),
                    ("y ∪ x", topo::union(&y, &vee, tol)),
                ] {
                    let what = format!("{what}: {op}");
                    let Ok(BooleanResult::Body(r)) = r else {
                        panic!("{what}: builds, got {:?}", r.map(|_| ()));
                    };
                    assert_eq!(validate_geometric(&r.body, tol), Ok(()), "{what}: tier 3");
                    assert_eq!(
                        validate_pseudomanifold(&r.body, &r.contacts, tol),
                        Ok(()),
                        "{what}: tier 3′"
                    );
                    let keel = spans_along(&r.body, pose, [[0.0, 2.0, 0.0], [1.0, 2.0, 0.0]]);
                    let on_keel = |s: f64| keel.iter().filter(|&&(a, b)| a < s && s < b).count();
                    assert_eq!(
                        on_keel(inside),
                        0,
                        "{what}: no edge on the keel at {inside}"
                    );
                    assert_eq!(
                        on_keel(beside),
                        2,
                        "{what}: the keel's two edges at {beside}"
                    );
                    unions.push(volume(&r.body));
                }
                for v in unions {
                    assert!(
                        (v - want).abs() < 1e-9,
                        "{what}: |x ∪ y| {v}, closed form {want}"
                    );
                }
            }
        }
    }
}
