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
                let v = topo::mass_properties(b, tol).unwrap().volume;
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
                let keel = keel_edges(b, &pose, ends);
                let [ka, kb] = keel[..] else {
                    panic!("{what}: the keel is {keel:?}, not two edges");
                };
                assert_ne!(ka, kb, "{what}: the keel's two edges are one");
                assert_eq!(
                    ends_of(b, ka),
                    ends_of(b, kb),
                    "{what}: the keel's edges end on different points"
                );
                let ((fa, wa), (fb, wb)) =
                    (floor_and_wall(b, &pose, ka), floor_and_wall(b, &pose, kb));
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
            let split_volume = topo::mass_properties(&walls, tol).unwrap().volume;
            for (what, r) in doors(row, &pose) {
                assert_eq!(
                    count(&r.body),
                    count(&walls),
                    "{what}: (v, e, f, solids) against the split"
                );
                let v = topo::mass_properties(&r.body, tol).unwrap().volume;
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
        let v = topo::mass_properties(b, tol).unwrap().volume;
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
        (
            "the split's slit",
            topo::test_support::finished("the split's walls", walls, tol),
        ),
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
                    let v = topo::mass_properties(b, tol).unwrap().volume;
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
