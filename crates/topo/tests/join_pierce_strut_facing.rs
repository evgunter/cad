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

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use geom_core::{Point3, Tol};
use topo::validate::{validate_closed, validate_geometric};
use topo::{Body, BooleanDeclarations, BooleanResult, mass_properties, validate_pseudomanifold};

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
    common::mapped_cube::<f64>(
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

/// Builds every op in both operand orders and checks each body at tiers
/// 2, 3 and 3′ and by volume against the prism's cut by the plane.
fn assert_pose_builds(pose: &str, m: [f64; 3]) {
    let prism = common::prism::<f64>(&PROFILE, 1.0, tol()).body;
    let cube = cube_beyond(m);
    let vol = |b: &Body<f64>| mass_properties(b, tol()).unwrap().volume;
    let (va, vb, shared) = (vol(&prism), vol(&cube), prism_beyond(m));
    let decls = BooleanDeclarations::default();
    for (order, x, y, vx) in [
        ("prism-cube", &prism, &cube, va),
        ("cube-prism", &cube, &prism, vb),
    ] {
        let ops: [(&str, _, f64); 3] = [
            (
                "union",
                topo::union_with(x, y, &decls, tol()),
                va + vb - shared,
            ),
            (
                "intersect",
                topo::intersect_with(x, y, &decls, tol()),
                shared,
            ),
            (
                "subtract",
                topo::subtract_with(x, y, &decls, tol()),
                vx - shared,
            ),
        ];
        for (op, r, want) in ops {
            let what = format!("{pose}: {order} {op}");
            let BooleanResult::Body(bb) = r.unwrap_or_else(|e| panic!("{what} refused: {e:?}"))
            else {
                panic!("{what}: overlapping operands cannot be Empty");
            };
            assert_eq!(validate_closed(&bb.body), Ok(()), "{what}: tier 2");
            assert_eq!(
                validate_geometric(&bb.body, tol()),
                Ok(()),
                "{what}: tier 3"
            );
            assert_eq!(
                validate_pseudomanifold(&bb.body, &bb.contacts, tol()),
                Ok(()),
                "{what}: tier 3′"
            );
            let got = vol(&bb.body);
            assert!(
                (got - want).abs() < 1e-9,
                "{what}: volume {got}, want {want}"
            );
        }
    }
}

/// The bare-bisector run: the start germ is the one the walk from the
/// arrival edge meets first, so `he_plus` faces it.
#[test]
fn a_bare_bisector_strut_faces_its_start_germ_with_he_plus() {
    assert_pose_builds("bare", [1.0, 1.3, -0.7]);
}

/// The whole-orbit run: the start germ is the one the walk meets last,
/// so `he_minus` faces it.
#[test]
fn a_whole_orbit_strut_faces_its_start_germ_with_he_minus() {
    assert_pose_builds("whole orbit", [-1.0, -1.3, 0.7]);
}

/// The oracle itself: the two poses cut the prism along one plane from
/// opposite sides.
#[test]
fn the_two_poses_split_the_prism() {
    let (m, n) = ([1.0, 1.3, -0.7], [-1.0, -1.3, 0.7]);
    let (a, b) = (prism_beyond(m), prism_beyond(n));
    assert!(
        (a + b - 3.0).abs() < 1e-12,
        "{a} + {b} is not the prism's 3"
    );
    assert!(a > 0.5 && b > 0.5, "both cuts are substantial: {a}, {b}");
}
