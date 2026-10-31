//! **A keel lying in the plane, wrapped round at both ends, splits with
//! the keel a zero-area ring of the side round it.**
//!
//! The box `[0, 4] × [0, 4] × [−1, 1]` less V-trenches whose keels, the
//! valley edges, lie in the plane `z = 0`: each the prism over a
//! triangle with its tip on the keel, rising `SPREAD` either side per
//! unit of height. Split by that plane either way up, at every pose:
//!
//! - one trench along `y = 2`, closed at both ends (`x ∈ [1.5, 2.5]`):
//!   the material above joins round each end of the keel, and the side
//!   above carries it on its section face as a ring of two coincident
//!   distinct edges, floor | wall a and floor | wall b, between one
//!   vertex at each end; the side below keeps neither the keel nor its
//!   end vertices (`crates/topo/README.md`, tier 3: where the line ends
//!   inside a face, that face carries its edges as a zero-area ring);
//! - two trenches crossing, and two meeting at a bend: the keels are a
//!   tree whose every end the material above wraps, and the ring runs
//!   round the tree, two edges over each of its edges;
//! - one trench open at one end (`x ∈ [1.5, 4.5]`): the same two edges
//!   are a slit of the section face's outline;
//! - one trench running wall to wall (`x ∈ [−0.5, 4.5]`): the side above
//!   is two solids, one either side of it.
//!
//! Each side passes tiers 1, 2, 3 and 3′, holds its closed-form volume,
//! and `plane_section` reads the box's square, the keels taken out of it.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use common::meeting::{Pose, poses};
use common::split_plane;
use geom_core::{Point3, Tol, Vec3};
use topo::splitting::plane_section;
use topo::{
    AtRestBody, Body, BooleanResult, ContactRecords, LoopBoundary, SplitPart, split, subtract,
    validate, validate_closed, validate_geometric, validate_pseudomanifold,
};

fn t() -> Tol {
    Tol::witness()
}

/// A trench's half-width per unit of height above its keel.
const SPREAD: f64 = 0.3;

/// A V-trench: its keel on `z = 0` along x at `y = at`, or along y at
/// `x = at`, over `span`.
struct Trench {
    along_x: bool,
    at: f64,
    span: (f64, f64),
}

/// The box less `trenches`, placed by `pose`.
fn keeled(pose: &Pose, trenches: &[Trench]) -> AtRestBody<f64> {
    use topo::test_support::{FaceGeometry, describe_as_intersections, finished, prism_ops};
    let tol = t();
    let prism =
        |what: &str, profile: &[(f64, f64)], span, map: &dyn Fn(f64, f64, f64) -> [f64; 3]| {
            let mut b = Body::<f64>::new();
            prism_ops(
                &mut b,
                profile,
                span,
                |a, c, h| pose.at(map(a, c, h)),
                FaceGeometry::Certified,
                tol,
            );
            describe_as_intersections(&mut b, tol);
            finished(what, b, tol)
        };
    let mut body = prism(
        "the box",
        &[(0.0, 0.0), (4.0, 0.0), (4.0, 4.0), (0.0, 4.0)],
        (-1.0, 1.0),
        &|x, y, z| [x, y, z],
    );
    let rise = 2.0;
    for trench in trenches {
        let (a, w) = (trench.at, SPREAD * rise);
        // Counterclockwise in the profile's frame, so the prism faces out.
        let tool = if trench.along_x {
            prism(
                "a trench",
                &[(a, 0.0), (a + w, rise), (a - w, rise)],
                trench.span,
                &|y, z, x| [x, y, z],
            )
        } else {
            prism(
                "a trench",
                &[(a, 0.0), (a - w, rise), (a + w, rise)],
                trench.span,
                &|x, z, y| [x, y, z],
            )
        };
        let BooleanResult::Body(cut) = subtract(&body, &tool, tol)
            .unwrap_or_else(|e| panic!("{}: the box less a trench: {e:?}", pose.label))
        else {
            panic!("{}: the box less a trench is empty", pose.label);
        };
        body = cut.body;
    }
    body
}

/// The plane `z = 0` at `pose`, its normal `+z` where `up`.
fn plane_at(pose: &Pose, up: bool) -> topo::SplitPlane<f64> {
    let o = pose.at([0.0, 0.0, 0.0]);
    let n = pose.at([0.0, 0.0, if up { 1.0 } else { -1.0 }]) - o;
    split_plane(o, Vec3::new(n.x, n.y, n.z), t())
}

/// The vertices of `body` at `p`.
fn vertices_at(body: &Body<f64>, p: Point3<f64>) -> usize {
    body.vertices()
        .filter(|(_, v)| body.get_point(v.point).unwrap().distance(p) < 1e-9)
        .count()
}

/// The ends of `edge` of `body`, each as its vertex's point key (a
/// vertex's copies share one), in key order.
fn ends(body: &Body<f64>, edge: topo::EdgeKey) -> [topo::PointKey; 2] {
    let he = body.get_edge(edge).unwrap().he_plus;
    let mut v = [
        body.get_half_edge(he).unwrap().start,
        body.half_edge_end(he).unwrap(),
    ]
    .map(|v| body.get_vertex(v).unwrap().point);
    v.sort();
    v
}

/// The edges of each ring of `body`'s faces.
fn rings(body: &Body<f64>) -> Vec<Vec<topo::EdgeKey>> {
    body.faces()
        .flat_map(|(_, f)| f.rings.clone())
        .filter_map(|l| match body.get_loop(l).unwrap().boundary {
            LoopBoundary::Cycle { first } => Some(
                body.loop_cycle(first)
                    .unwrap()
                    .into_iter()
                    .map(|he| body.get_half_edge(he).unwrap().edge)
                    .collect(),
            ),
            LoopBoundary::Empty { .. } => None,
        })
        .collect()
}

/// One scene, and what the side the trenches' walls are on makes of it.
/// Its volume is the box's top half less the trenches' parts in it:
/// `½ · 1 · 2·SPREAD = 0.3` per unit of a trench's length in the box,
/// less what two trenches share, `∫₀¹ (2·SPREAD·z)² dz = 0.12` where they
/// cross and `∫₀¹ (SPREAD·z)² dz = 0.03` where one ends in the other.
struct Row {
    what: &'static str,
    trenches: &'static [Trench],
    walls: f64,
    /// The solids the walls' side sorts into.
    solids: usize,
    /// The edges of its keels' ring; 0 where it has none.
    ring: usize,
    /// The keels' ends and junctions, where the floor side keeps no
    /// vertex when the walls' side holds a ring.
    keel: &'static [[f64; 3]],
}

const ROWS: [Row; 5] = [
    Row {
        what: "closed at both ends",
        trenches: &[Trench {
            along_x: true,
            at: 2.0,
            span: (1.5, 2.5),
        }],
        walls: 16.0 - 0.3,
        solids: 1,
        ring: 2,
        keel: &[[1.5, 2.0, 0.0], [2.5, 2.0, 0.0]],
    },
    Row {
        what: "two crossing",
        trenches: &[
            Trench {
                along_x: true,
                at: 2.0,
                span: (1.5, 2.5),
            },
            Trench {
                along_x: false,
                at: 2.0,
                span: (1.5, 2.5),
            },
        ],
        walls: 16.0 - 0.6 + 0.12,
        solids: 1,
        ring: 8,
        keel: &[
            [1.5, 2.0, 0.0],
            [2.5, 2.0, 0.0],
            [2.0, 1.5, 0.0],
            [2.0, 2.5, 0.0],
            [2.0, 2.0, 0.0],
        ],
    },
    Row {
        what: "two meeting at a bend",
        trenches: &[
            Trench {
                along_x: true,
                at: 2.0,
                span: (1.5, 2.5),
            },
            Trench {
                along_x: false,
                at: 2.5,
                span: (2.0, 3.0),
            },
        ],
        walls: 16.0 - 0.6 + 0.03,
        solids: 1,
        ring: 4,
        keel: &[[1.5, 2.0, 0.0], [2.5, 2.0, 0.0], [2.5, 3.0, 0.0]],
    },
    Row {
        what: "open at one end",
        trenches: &[Trench {
            along_x: true,
            at: 2.0,
            span: (1.5, 4.5),
        }],
        walls: 16.0 - 0.3 * 2.5,
        solids: 1,
        ring: 0,
        keel: &[],
    },
    Row {
        what: "wall to wall",
        trenches: &[Trench {
            along_x: true,
            at: 2.0,
            span: (-0.5, 4.5),
        }],
        walls: 16.0 - 0.3 * 4.0,
        solids: 2,
        ring: 0,
        keel: &[],
    },
];

/// **The keels split sound either way up, at every pose and ε**: both
/// sides at tiers 1, 2, 3 and 3′ and their closed-form volumes. Where
/// every end of the keels is wrapped, the walls' side carries them as
/// one ring of distinct edges, each coinciding with exactly one other
/// between the same two points, and the floor side keeps no vertex at
/// any end or junction of the keels. `plane_section` reads the box's
/// square, with no hole.
#[test]
fn a_keel_wrapped_at_both_ends_splits_with_its_keel_a_ring() {
    let tol = t();
    for row in &ROWS {
        for pose in poses() {
            let body = keeled(&pose, row.trenches);
            for up in [true, false] {
                let what = format!("{}, {}, normal +z {up}", row.what, pose.label);
                let plane = plane_at(&pose, up);
                let r = split(&body, &plane, tol).unwrap_or_else(|e| panic!("{what}: {e:?}"));
                let (SplitPart::Body(above), SplitPart::Body(below)) = (&r.above, &r.below) else {
                    panic!("{what}: a side is empty");
                };
                let (walls, floor) = if up { (above, below) } else { (below, above) };
                for (side, b, volume) in [("walls", walls, row.walls), ("floor", floor, 16.0)] {
                    let what = format!("{what}, the {side} side");
                    assert_eq!(validate(b), Ok(()), "{what}: tier 1");
                    assert_eq!(validate_closed(b), Ok(()), "{what}: tier 2");
                    assert_eq!(validate_geometric(b, tol), Ok(()), "{what}: tier 3");
                    assert_eq!(
                        validate_pseudomanifold(b, &ContactRecords::default(), tol),
                        Ok(()),
                        "{what}: tier 3′"
                    );
                    let v = topo::test_support::pinned_volume("the measured body", b, tol).volume;
                    assert!(
                        (v - volume).abs() < 1e-9 * volume,
                        "{what}: volume {v}, closed form {volume}"
                    );
                }
                assert_eq!(
                    walls.solids().count(),
                    row.solids,
                    "{what}: the walls' side's solids"
                );
                assert!(
                    rings(floor).iter().all(|r| r.len() > 2),
                    "{what}: the floor side holds a ring of two edges or fewer"
                );
                let zero_area: Vec<_> = rings(walls)
                    .into_iter()
                    .filter(|r| {
                        r.iter().all(|&e| {
                            r.iter()
                                .filter(|&&g| g != e && ends(walls, g) == ends(walls, e))
                                .count()
                                == 1
                        })
                    })
                    .collect();
                match (row.ring, zero_area.as_slice()) {
                    (0, []) => {}
                    (n, [ring]) if n == ring.len() => {
                        for &p in row.keel {
                            let p = pose.at(p);
                            assert_eq!(
                                vertices_at(floor, p),
                                0,
                                "{what}: the floor side keeps a vertex at {p:?}"
                            );
                            assert!(
                                ring.iter().any(|&e| ends(walls, e).iter().any(|&k| walls
                                    .get_point(k)
                                    .unwrap()
                                    .distance(p)
                                    < 1e-9)),
                                "{what}: the ring misses {p:?}"
                            );
                        }
                    }
                    (n, rings) => panic!("{what}: zero-area rings {rings:?}, not one of {n} edges"),
                }
                let section =
                    plane_section(&body, &plane, tol).unwrap_or_else(|e| panic!("{what}: {e:?}"));
                let areas: Vec<f64> = section.regions.iter().map(|r| r.area()).collect();
                let holes: usize = section.regions.iter().map(|r| r.holes.len()).sum();
                let whole: f64 = areas.iter().sum();
                assert!(
                    (whole - 16.0).abs() < 1e-9 && holes == 0,
                    "{what}: the section's regions {areas:?} with {holes} hole(s), not the \
                     square"
                );
            }
        }
    }
}

/// **A later cut across the ring builds, and so does one through its
/// tip.** The side round the keel, split again: by `x = 2`, across the
/// ring's two edges, into halves of `15.7 / 2`; by `x = 1.5` and
/// `x = 2.5` through its tips, and by `y = 2` and `z = 0` along it. At a
/// tip the section face wraps a full turn round the ring's two
/// coincident edges, and each side holds its closed form and passes
/// tier 3′.
#[test]
fn a_later_cut_across_the_ring_builds_and_so_does_one_through_its_tip() {
    let tol = t();
    let pose = Pose::rest();
    let row = &ROWS[0];
    let body = keeled(&pose, row.trenches);
    let r = split(&body, &plane_at(&pose, true), tol).unwrap();
    let SplitPart::Body(walls) = r.above else {
        panic!("nothing above the plane");
    };
    let plane = |o: [f64; 3], n: [f64; 3]| {
        split_plane(
            Point3::new(o[0], o[1], o[2]),
            Vec3::new(n[0], n[1], n[2]),
            tol,
        )
    };
    let across = split(&walls, &plane([2.0, 0.0, 0.0], [1.0, 0.0, 0.0]), tol)
        .unwrap_or_else(|e| panic!("the cut across the ring: {e:?}"));
    for (side, part) in [("left", &across.below), ("right", &across.above)] {
        let SplitPart::Body(b) = part else {
            panic!("the cut across the ring: nothing on the {side}");
        };
        let v = topo::test_support::pinned_volume("the measured body", b, tol).volume;
        let half = row.walls / 2.0;
        assert!(
            (v - half).abs() < 1e-9 * half,
            "the cut across the ring: the {side} half holds {v}, not {half}"
        );
    }
    // The trench takes `0.3` out of the walls, all of it on the side
    // the ring lies on.
    for (what, o, n, above, below) in [
        (
            "through one tip",
            [1.5, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            9.7,
            6.0,
        ),
        (
            "through the other",
            [2.5, 0.0, 0.0],
            [-1.0, 0.0, 0.0],
            9.7,
            6.0,
        ),
        (
            "along it, upright",
            [0.0, 2.0, 0.0],
            [0.0, 1.0, 0.0],
            7.85,
            7.85,
        ),
        (
            "along it, in its plane",
            [0.0, 0.0, 0.0],
            [0.0, 0.0, 1.0],
            row.walls,
            0.0,
        ),
    ] {
        let r =
            split(&walls, &plane(o, n), tol).unwrap_or_else(|e| panic!("the cut {what}: {e:?}"));
        for (side, part, volume) in [("above", &r.above, above), ("below", &r.below, below)] {
            let what = format!("the cut {what}, {side}");
            let b = match part {
                SplitPart::Body(b) => b,
                SplitPart::Empty if volume == 0.0 => continue,
                SplitPart::Empty => panic!("{what}: empty, not {volume}"),
            };
            assert_eq!(
                validate_pseudomanifold(b, &ContactRecords::default(), tol),
                Ok(()),
                "{what}: tier 3′"
            );
            let v = topo::test_support::pinned_volume("the measured body", b, tol).volume;
            assert!(
                (v - volume).abs() < 1e-9 * volume,
                "{what}: volume {v}, closed form {volume}"
            );
        }
    }
}

/// Reviewer probe (`splitdoor/pocket-floor-review`): oblique cuts through
/// a keel's wrapped tip, where the full turn's two side quarters fall on
/// opposite sides of the plane, so their order and sides matter. Each
/// side's volume is integrated independently (the square less the
/// trench rectangle, clipped by the plane, per height).
#[test]
fn review_an_oblique_cut_through_a_wrapped_tip_holds_its_closed_form() {
    let tol = t();
    let pose = Pose::rest();
    let plane = |o: [f64; 3], n: [f64; 3]| {
        split_plane(
            Point3::new(o[0], o[1], o[2]),
            Vec3::new(n[0], n[1], n[2]),
            tol,
        )
    };
    let ring = &ROWS[0];
    let slit = &ROWS[3];
    let tree = &ROWS[1];
    // The open slit's trench runs on to the wall, so the side holding
    // it is two wedges meeting along the keel, as the wall-to-wall row's,
    // unless the plane tilts out of the floor and wraps the trench's end.
    for (row, o, n, above, below, want_shells) in [
        (ring, [1.5, 2.0, 0.0], [1.0, 1.0, 0.0], 9.59, 6.11, 2),
        (ring, [1.5, 2.0, 0.0], [1.0, -1.0, 0.0], 9.59, 6.11, 2),
        (ring, [1.5, 2.0, 0.0], [-1.0, -1.0, 0.0], 6.11, 9.59, 2),
        (
            ring,
            [1.5, 2.0, 0.0],
            [1.0, 1.0, 1.0],
            11.158333333333333,
            4.541666666666667,
            2,
        ),
        (ring, [1.5, 2.0, 0.0], [1.0, -0.4, 0.7], 11.1, 4.6, 2),
        (ring, [2.5, 2.0, 0.0], [-1.0, -1.0, 0.0], 9.59, 6.11, 2),
        (slit, [1.5, 2.0, 0.0], [1.0, 0.0, 0.0], 9.25, 6.0, 3),
        (slit, [1.5, 2.0, 0.0], [1.0, 1.0, 0.0], 9.14, 6.11, 3),
        (
            slit,
            [1.5, 2.0, 0.0],
            [1.0, 1.0, 1.0],
            10.708333333333333,
            4.541666666666667,
            2,
        ),
        (tree, [1.5, 2.0, 0.0], [1.0, 0.0, 0.0], 9.52, 6.0, 2),
        (tree, [2.0, 2.0, 0.0], [1.0, 1.0, 0.0], 7.76, 7.76, 2),
    ] {
        let body = keeled(&pose, row.trenches);
        let r = split(&body, &plane_at(&pose, true), tol).unwrap();
        let SplitPart::Body(walls) = r.above else {
            panic!("nothing above the plane");
        };
        let what = format!("{}: {o:?} {n:?}", row.what);
        let r = split(&walls, &plane(o, n), tol).unwrap_or_else(|e| panic!("{what}: {e:?}"));
        let mut shells = 0;
        for (side, part, volume) in [("above", &r.above, above), ("below", &r.below, below)] {
            let SplitPart::Body(b) = part else {
                panic!("{what} {side}: empty, not {volume}");
            };
            assert_eq!(validate(b), Ok(()), "{what} {side}: tiers 1-3");
            assert_eq!(
                validate_pseudomanifold(b, &ContactRecords::default(), tol),
                Ok(()),
                "{what} {side}: tier 3′"
            );
            let v = topo::test_support::pinned_volume("the measured body", b, tol).volume;
            assert!(
                (v - volume).abs() < 1e-9 * volume,
                "{what} {side}: volume {v}, closed form {volume}"
            );
            shells += b.shells().count();
        }
        assert_eq!(shells, want_shells, "{what}: shells");
    }
}
