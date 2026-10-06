//! **A pierce's strut faces its germs by the corner walk.** The
//! piercing vertex is the reflex top corner `v = (1, 1, 1)` of an
//! L-prism; the pierced face is the near face of a cube standing on
//! the plane through `v` with unit normal `m`, the cube on `m`'s side
//! and large enough to hold all of the prism there. The plane leaves
//! the corner's three real edges (+x, +y, −z) on one side and the
//! reflex top sector's bisector on the other, so the pierce run is a
//! strut in that sector, with both germs inside the top face: no germ
//! runs along an edge, so the corner's edges name no facing and the
//! walk from the corner's arrival edge decides it.
//!
//! - `m = (1, 1.3, −0.7)`: the edges read In and the bisector alone
//!   Out, the bare-bisector run (`classify_vertex_on_face`'s `_` arm).
//! - `−m`: the edges read Out and the bisector alone In, the run holds
//!   the whole orbit (`RunSite::WholeOrbit`).
//!
//! The two facings are opposite (the start germ is the one the walk
//! meets first in the bare run and last in the whole-orbit one), and
//! the other facing refuses `JoinDesync` in every op and order of
//! either pose.
//!
//! **Two Out runs at one pierce.** Tilted so that the −z edge reads
//! Out too (`TWO_RUNS`), the corner has two Out runs, the bisector
//! alone and the −z edge alone, and the plane meets the prism in two
//! lobes that touch at `v`. The union and the intersection pinch there
//! (two cones of boundary meet at one point); each difference does
//! not. Every op, in both orders, builds `SOUND`, one vertex per cone
//! at `v`, all on one point key, and tessellates. Red if the ring
//! struts ignore the walk (`insert::strut_order`), or if the zips fuse
//! the pinch to itself: the intersection keeps one vertex at `v` in
//! both operands, and `zip::split_cones` splits each per cone.
//!
//! Two neighbouring families pinch too, and build in every op. A
//! second run holding the x = 1 face's bisector as well (`WIDE_RUN`)
//! pinches its intersection. Two runs that are each a lone edge
//! (`EDGE_RUNS`) pinch cube ∖ prism over two seams, the cube face's two
//! holes meeting at `v`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point3, Tol};
use topo::test_support as fixtures;
use topo::validate::validate_geometric;
use topo::{
    AtRestBody, Body, BooleanDeclarations, BooleanError, BooleanResult, LoopBoundary,
    mass_properties,
};

use crate::common::differential::outcome;

const PROFILE: [(f64, f64); 6] = [
    (0.0, 0.0),
    (2.0, 0.0),
    (2.0, 1.0),
    (1.0, 1.0),
    (1.0, 2.0),
    (0.0, 2.0),
];
const V: [f64; 3] = [1.0, 1.0, 1.0];
const SIDE: f64 = 4.0;
/// The bare-bisector tilt.
const BARE: [f64; 3] = [1.0, 1.3, -0.7];
/// The tilts at which the −z edge reads Out too: two Out runs.
const TWO_RUNS: [[f64; 3]; 3] = [[1.0, 1.3, 0.7], [1.0, 1.3, 0.4], [1.2, 1.0, 0.3]];
/// Two Out runs, the second the x = 1 face's bisector and the −z edge.
const WIDE_RUN: [[f64; 3]; 2] = [[2.0, 0.4, 1.0], [1.0, 0.2, 0.5]];
/// Two Out runs, the +x edge alone and the +y edge alone.
const EDGE_RUNS: [[f64; 3]; 3] = [[-1.0, -0.9, -1.3], [-1.0, -0.6, -1.2], [-0.8, -0.7, -1.0]];

fn tol() -> Tol {
    Tol::witness()
}

fn unit(m: [f64; 3]) -> [f64; 3] {
    let n = m.iter().map(|c| c * c).sum::<f64>().sqrt();
    m.map(|c| c / n)
}

/// The cube of side [`SIDE`] whose bottom face lies on the plane
/// through `V` with unit normal `m`, centred on `V`, standing on `m`'s
/// side: `u`, `w`, `m` a right-handed frame, so the map keeps the
/// unit cube's orientation.
fn cube_beyond(m: [f64; 3]) -> Body<f64> {
    let m = unit(m);
    let u = unit([m[1], -m[0], 0.0]);
    let w = [
        m[1] * u[2] - m[2] * u[1],
        m[2] * u[0] - m[0] * u[2],
        m[0] * u[1] - m[1] * u[0],
    ];
    fixtures::mapped_cube::<f64>(
        move |x, y, z| {
            let (a, b, c) = (SIDE * (x - 0.5), SIDE * (y - 0.5), SIDE * z);
            Point3::new(
                V[0] + a * u[0] + b * w[0] + c * m[0],
                V[1] + a * u[1] + b * w[1] + c * m[1],
                V[2] + a * u[2] + b * w[2] + c * m[2],
            )
        },
        tol(),
    )
}

/// The area of the part of `PROFILE` where `a·x + b·y ≥ c`.
fn clipped_area(a: f64, b: f64, c: f64) -> f64 {
    let f = |p: (f64, f64)| a * p.0 + b * p.1 - c;
    let n = PROFILE.len();
    let mut kept = Vec::new();
    for i in 0..n {
        let (p, q) = (PROFILE[i], PROFILE[(i + 1) % n]);
        if f(p) >= 0.0 {
            kept.push(p);
        }
        if (f(p) >= 0.0) != (f(q) >= 0.0) {
            let t = f(p) / (f(p) - f(q));
            kept.push((p.0 + t * (q.0 - p.0), p.1 + t * (q.1 - p.1)));
        }
    }
    let k = kept.len();
    (0..k)
        .map(|i| kept[i].0 * kept[(i + 1) % k].1 - kept[(i + 1) % k].0 * kept[i].1)
        .sum::<f64>()
        / 2.0
}

/// The volume of the prism (`PROFILE` × z ∈ [0, 1]) on `m`'s side of
/// the plane through `V`, which the cube holds whole. Each slice's area
/// is quadratic in z between the heights where the cut line meets a
/// profile vertex, so Simpson's rule on each piece is exact.
fn prism_beyond(m: [f64; 3]) -> f64 {
    let m = unit(m);
    let c = |z: f64| m[0] * V[0] + m[1] * V[1] - m[2] * (z - V[2]);
    let area = |z: f64| clipped_area(m[0], m[1], c(z));
    let mut cuts: Vec<f64> = PROFILE
        .iter()
        .map(|p| V[2] + (m[0] * (V[0] - p.0) + m[1] * (V[1] - p.1)) / m[2])
        .filter(|&z| z > 0.0 && z < 1.0)
        .chain([0.0, 1.0])
        .collect();
    cuts.sort_by(f64::total_cmp);
    cuts.windows(2)
        .map(|w| (w[1] - w[0]) / 6.0 * (area(w[0]) + 4.0 * area((w[0] + w[1]) / 2.0) + area(w[1])))
        .sum()
}

/// `body` as a boolean operand: the at-rest gate's finished body.
fn finished(what: &str, body: Body<f64>) -> AtRestBody<f64> {
    AtRestBody::validate(body, tol())
        .unwrap_or_else(|e| panic!("{what} is not a finished body: {e:?}"))
}

type Op = fn(
    &AtRestBody<f64>,
    &AtRestBody<f64>,
    &BooleanDeclarations,
    Tol,
) -> Result<BooleanResult<f64>, BooleanError>;

/// Builds every op in both operand orders.
/// Each body is `SOUND` by [`outcome`] (tiers 2 and 3′, the
/// certificate, a legal operand, its volume), passes tier 3, holds the
/// prism's cut by the plane to 1e-9, and holds the pierce point as one
/// vertex wherever a face meets it: its vertices there share one point,
/// and no face runs through two of them.
fn assert_pose(pose: &str, m: [f64; 3]) {
    let prism = finished(
        "the prism",
        fixtures::prism::<f64>(&PROFILE, 1.0, tol()).body,
    );
    let cube = finished("the cube", cube_beyond(m));
    let vol = |b: &Body<f64>| mass_properties(b, tol()).unwrap().volume;
    let (va, vb, shared) = (vol(&prism), vol(&cube), prism_beyond(m));
    let decls = BooleanDeclarations::default();
    for (order, x, y, vx) in [
        ("prism-cube", &prism, &cube, va),
        ("cube-prism", &cube, &prism, vb),
    ] {
        let ops: [(&str, Op, f64); 3] = [
            ("union", topo::union_with, va + vb - shared),
            ("intersect", topo::intersect_with, shared),
            ("subtract", topo::subtract_with, vx - shared),
        ];
        for (op, run, want) in ops {
            let what = format!("{pose}: {order} {op}");
            let line = outcome(run(x, y, &decls, tol()), want, tol());
            assert!(line.starts_with("OK SOUND"), "{what}: {line}");
            let Ok(BooleanResult::Body(bb)) = run(x, y, &decls, tol()) else {
                panic!("{what}: a second run did not build");
            };
            assert_eq!(
                validate_geometric(&bb.body, tol()),
                Ok(()),
                "{what}: tier 3"
            );
            let got = vol(&bb.body);
            assert!(
                (got - want).abs() < 1e-9,
                "{what}: volume {got}, want {want}"
            );
            let at_v: Vec<_> = bb
                .body
                .vertex_points()
                .filter(|(_, p)| [p.x, p.y, p.z] == V)
                .map(|(k, _)| k)
                .collect();
            let point = |k| bb.body.get_vertex(k).unwrap().point;
            assert!(
                at_v.iter().all(|&k| point(k) == point(at_v[0])),
                "{what}: the vertices at the pierce point share one point: {at_v:?}"
            );
            let meshed =
                mesh::tessellate(&bb.body, 0.05, tol()).map(|m| mesh::validate::check_mesh(&m));
            assert!(
                matches!(meshed, Ok(Ok(()))),
                "{what}: the body does not mesh: {meshed:?}"
            );
        }
    }
}

/// The bare-bisector run: the start germ is the one the walk from the
/// arrival edge meets first, so `he_plus` faces it.
#[test]
fn a_bare_bisector_strut_faces_its_start_germ_with_he_plus() {
    assert_pose("bare", BARE);
}

/// The whole-orbit run: the start germ is the one the walk meets last,
/// so `he_minus` faces it.
#[test]
fn a_whole_orbit_strut_faces_its_start_germ_with_he_minus() {
    assert_pose("whole orbit", BARE.map(|c| -c));
}

/// Two Out runs at the corner, the bisector alone and the −z edge
/// alone: every op builds.
#[test]
fn two_out_runs_at_the_corner_build_in_every_op() {
    for m in TWO_RUNS {
        assert_pose(&format!("two runs {m:?}"), m);
    }
}

/// The −z edge's run holding the x = 1 face's bisector too, tilted far
/// toward x: every op builds, the intersection pinched at `v`.
#[test]
fn a_wide_run_builds_in_every_op() {
    for m in WIDE_RUN {
        assert_pose(&format!("wide run {m:?}"), m);
    }
}

/// The +x and +y edges alone read Out, two fans: every op builds, cube
/// ∖ prism pinched at `v` across two seams.
#[test]
fn two_edge_runs_build_in_every_op() {
    for m in EDGE_RUNS {
        assert_pose(&format!("edge runs {m:?}"), m);
    }
}

/// **A pinch keeps one vertex per cone.** The two-run intersection
/// pinches at `v`: the result holds two vertices there, one per cone,
/// on one point key, and no face passes either of them twice. Red if
/// the zips fuse the cones into one vertex, or a split leaves a cone's
/// corners on two vertices.
#[test]
fn a_pinch_keeps_one_vertex_per_cone() {
    let prism = finished(
        "the prism",
        fixtures::prism::<f64>(&PROFILE, 1.0, tol()).body,
    );
    let cube = finished("the cube", cube_beyond(TWO_RUNS[0]));
    let decls = BooleanDeclarations::default();
    for (order, x, y) in [("prism-cube", &prism, &cube), ("cube-prism", &cube, &prism)] {
        let Ok(BooleanResult::Body(bb)) = topo::intersect_with(x, y, &decls, tol()) else {
            panic!("{order}: the intersection did not build");
        };
        let body = &bb.body;
        let at_v: Vec<_> = body
            .vertex_points()
            .filter(|(_, p)| [p.x, p.y, p.z] == V)
            .map(|(k, _)| k)
            .collect();
        assert_eq!(at_v.len(), 2, "{order}: one vertex per cone at v");
        for (face, f) in body.faces() {
            for &l in std::iter::once(&f.outer).chain(&f.rings) {
                if let LoopBoundary::Cycle { first } = body.get_loop(l).unwrap().boundary {
                    let starts: Vec<_> = body
                        .loop_cycle(first)
                        .unwrap()
                        .into_iter()
                        .map(|he| body.get_half_edge(he).unwrap().start)
                        .filter(|v| at_v.contains(v))
                        .collect();
                    let mut once = starts.clone();
                    once.sort();
                    once.dedup();
                    assert_eq!(
                        once.len(),
                        starts.len(),
                        "{order}: face {face:?} passes a vertex at v twice"
                    );
                }
            }
        }
    }
}

/// The oracle itself: each pose and its opposite cut the prism along
/// one plane from either side.
#[test]
fn opposite_poses_split_the_prism() {
    for m in TWO_RUNS
        .into_iter()
        .chain(WIDE_RUN)
        .chain(EDGE_RUNS)
        .chain([BARE])
    {
        let (a, b) = (prism_beyond(m), prism_beyond(m.map(|c| -c)));
        assert!(
            (a + b - 3.0).abs() < 1e-12,
            "{m:?}: {a} + {b} is not the prism's 3"
        );
        assert!(
            a > 0.1 && b > 0.1,
            "{m:?}: both cuts are substantial: {a}, {b}"
        );
    }
}
