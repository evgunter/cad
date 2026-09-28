//! **Two falsifier sweeps over the census's touch analysis**, rebuilt as
//! rows: the reviewers' rotated-prism sweep (CONTACT-1) and crossed-ridge
//! sweep (CONTACT-5), each against exact ground truth.
//!
//! A pose CLEARS when the census raises no placement finding on its
//! solid pair (no solid-pair refusal and no interference). A pose whose
//! materials overlap and clears is a WRONG CLEAR, and there must be none.
//! A pose whose materials do not overlap and is refused is a FALSE
//! REFUSAL: a cost, printed and pinned so that a change which adds one
//! has to say why.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;
use geom_core::{Point3, Tol};
use topo::{Body, ContactRecords, EntityId, ValidationError, validate_pseudomanifold};

/// Whether the census raised a placement finding on a solid pair.
fn refused(body: &Body<f64>) -> bool {
    validate_pseudomanifold(body, &ContactRecords::default(), Tol::witness())
        .err()
        .unwrap_or_default()
        .iter()
        .any(|e| {
            matches!(
                e,
                ValidationError::CensusUndecidable {
                    a: EntityId::Solid(_),
                    b: EntityId::Solid(_),
                    ..
                } | ValidationError::InstanceInterference { .. }
            )
        })
}

/// A prism over `profile`, `z` its extrusion, carried by `map`, grafted
/// into `body` as a new solid.
fn graft_prism(
    body: &mut Body<f64>,
    profile: &[(f64, f64)],
    z: (f64, f64),
    map: impl Fn(f64, f64, f64) -> Point3<f64>,
) {
    let tol = Tol::witness();
    let mut part = Body::<f64>::new();
    common::prism_ops(&mut part, profile, z, map, common::FaceGeometry::Certified, tol);
    common::describe_as_intersections(&mut part, tol);
    topo::graft_disjoint(body, &part, tol).unwrap();
}

/// The rotation by `ang` about `axis`.
fn rotation(axis: [f64; 3], ang: f64) -> impl Fn([f64; 3]) -> [f64; 3] + Copy {
    let n = (axis[0] * axis[0] + axis[1] * axis[1] + axis[2] * axis[2]).sqrt();
    let k = [axis[0] / n, axis[1] / n, axis[2] / n];
    let (c, s) = (ang.cos(), ang.sin());
    move |v: [f64; 3]| {
        let kv = k[0] * v[0] + k[1] * v[1] + k[2] * v[2];
        let cr = [
            k[1] * v[2] - k[2] * v[1],
            k[2] * v[0] - k[0] * v[2],
            k[0] * v[1] - k[1] * v[0],
        ];
        [
            v[0] * c + cr[0] * s + k[0] * kv * (1.0 - c),
            v[1] * c + cr[1] * s + k[1] * kv * (1.0 - c),
            v[2] * c + cr[2] * s + k[2] * kv * (1.0 - c),
        ]
    }
}

/// Whether two closed intervals overlap in a positive length.
fn overlaps(a: (f64, f64), b: (f64, f64)) -> bool {
    a.1.min(b.1) - a.0.max(b.0) > 0.0
}

/// The L profile: counterclockwise from `+z`, reflex at (1, 1).
const L_PROFILE: [(f64, f64); 6] = [
    (0.0, 0.0),
    (3.0, 0.0),
    (3.0, 1.0),
    (1.0, 1.0),
    (1.0, 3.0),
    (0.0, 3.0),
];

/// **The rotated-prism sweep.** The L-bracket over `z ∈ [0, 1]` against
/// a brick on a grid of spans around it — beside its walls, in its
/// inner corner, on and under it, sunk into it — the whole assembly
/// under the identity and two generic rotations, so that every
/// coincidence is read off-axis. The bracket's floor and ceiling are
/// faces that are not convex, reaching back across every wall plane.
/// The bracket is the union of the boxes `[0, 3] × [0, 1] × [0, 1]` and
/// `[0, 1] × [0, 3] × [0, 1]`, so the materials overlap exactly when
/// the brick's intervals overlap either box's in a positive length on
/// all three axes.
#[test]
fn a_rotated_bracket_and_brick_sweep_clears_no_overlap() {
    let xy = [
        (-1.0, 0.0),
        (0.0, 1.0),
        (1.0, 2.0),
        (2.0, 3.0),
        (1.0, 3.0),
        (0.5, 1.5),
    ];
    let zs = [(-1.0, 0.0), (1.0, 2.0), (0.0, 1.0), (0.2, 0.8)];
    let rotations = [
        ([0.0, 0.0, 1.0], 0.0),
        ([1.0, 2.0, 3.0], 0.7),
        ([-0.3, 1.0, 0.2], 2.1),
    ];
    let boxes = [((0.0, 3.0), (0.0, 1.0)), ((0.0, 1.0), (0.0, 3.0))];
    let (mut wrong_clears, mut false_refusals, mut overlapping, mut apart) =
        (Vec::new(), Vec::new(), 0, 0);
    for &(axis, ang) in &rotations {
        let r = rotation(axis, ang);
        let map = move |x: f64, y: f64, z: f64| {
            let q = r([x, y, z]);
            Point3::new(q[0], q[1], q[2])
        };
        for &x in &xy {
            for &y in &xy {
                for &z in &zs {
                    let mut body = Body::<f64>::new();
                    graft_prism(&mut body, &L_PROFILE, (0.0, 1.0), map);
                    let square = [(x.0, y.0), (x.1, y.0), (x.1, y.1), (x.0, y.1)];
                    graft_prism(&mut body, &square, z, map);
                    let overlap = overlaps(z, (0.0, 1.0))
                        && boxes
                            .iter()
                            .any(|&(bx, by)| overlaps(x, bx) && overlaps(y, by));
                    let refused = refused(&body);
                    if overlap {
                        overlapping += 1;
                        if !refused {
                            wrong_clears.push((ang, x, y, z));
                        }
                    } else {
                        apart += 1;
                        if refused {
                            false_refusals.push((ang, x, y, z));
                        }
                    }
                }
            }
        }
    }
    println!(
        "rotated-prism sweep: {overlapping} overlapping, {apart} not; wrong clears {}, \
         false refusals {}: {false_refusals:?}",
        wrong_clears.len(),
        false_refusals.len()
    );
    assert_eq!(overlapping + apart, 432);
    assert!(wrong_clears.is_empty(), "{wrong_clears:?}");
}

/// A parallelepiped `p + u·a + v·b + w·c` over the unit cube, grafted
/// into `body` (`det[a, b, c] > 0`).
fn graft_parallelepiped(body: &mut Body<f64>, p: [f64; 3], a: [f64; 3], b: [f64; 3], c: [f64; 3]) {
    let part = common::mapped_cube(
        move |u, v, w| {
            Point3::new(
                p[0] + u * a[0] + v * b[0] + w * c[0],
                p[1] + u * a[1] + v * b[1] + w * c[1],
                p[2] + u * a[2] + v * b[2] + w * c[2],
            )
        },
        Tol::witness(),
    );
    topo::graft_disjoint(body, &part, Tol::witness()).unwrap();
}

type Solid3 = ([f64; 3], [f64; 3], [f64; 3], [f64; 3]);

fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}
fn cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

/// The eight corners of a parallelepiped.
fn corners((p, a, b, c): Solid3) -> Vec<[f64; 3]> {
    let mut out = Vec::new();
    for i in 0..8 {
        let (u, v, w) = (f64::from(i & 1), f64::from((i >> 1) & 1), f64::from((i >> 2) & 1));
        out.push([
            p[0] + u * a[0] + v * b[0] + w * c[0],
            p[1] + u * a[1] + v * b[1] + w * c[1],
            p[2] + u * a[2] + v * b[2] + w * c[2],
        ]);
    }
    out
}

/// How deep two parallelepipeds' interiors overlap, by the separating
/// axis test over their face normals and edge cross products: the least
/// overlap of their projections on any axis, in metres (negative when
/// apart). Exact for two convex polytopes.
fn overlap_depth(s: Solid3, t: Solid3) -> f64 {
    let edges = |(_, a, b, c): Solid3| [a, b, c];
    let (es, et) = (edges(s), edges(t));
    let mut axes = vec![
        cross(es[0], es[1]),
        cross(es[1], es[2]),
        cross(es[2], es[0]),
        cross(et[0], et[1]),
        cross(et[1], et[2]),
        cross(et[2], et[0]),
    ];
    for &e in &es {
        for &f in &et {
            axes.push(cross(e, f));
        }
    }
    let (cs, ct) = (corners(s), corners(t));
    let mut depth = f64::INFINITY;
    for axis in axes {
        let n = dot(axis, axis).sqrt();
        if n < 1e-9 {
            continue;
        }
        let proj = |cs: &[[f64; 3]]| {
            cs.iter().map(|&q| dot(q, axis) / n).fold((f64::INFINITY, f64::NEG_INFINITY), |(lo, hi), x| {
                (lo.min(x), hi.max(x))
            })
        };
        let ((a0, a1), (b0, b1)) = (proj(&cs), proj(&ct));
        depth = depth.min(a1.min(b1) - a0.max(b0));
    }
    depth
}

/// **The crossed-ridge sweep.** Two square bars turned 45° about their
/// axes, crossed ridge on ridge as in `contact5_gate_and_beam`: the
/// lower bar's ridge along `x` at `y = 0, z = 2`, over `x ∈ [−1, 1]`;
/// the upper bar's ridge turned `θ` from it about `z`, crossing the
/// lower ridge's line at `x = o`, shifted `s` along its own axis, and
/// lifted `h`. Five crossings (two at or past the lower bar's end),
/// three angles, three shifts (one putting the crossing at the upper
/// bar's end) and three lifts — sunk a centimetre, resting, standing a
/// centimetre off. Ground truth is the separating-axis depth of the two
/// convex bars.
#[test]
fn a_crossed_ridge_sweep_clears_no_overlap() {
    let lower: Solid3 = ([-1.0, 0.0, 0.0], [2.0, 0.0, 0.0], [0.0, 1.0, 1.0], [0.0, -1.0, 1.0]);
    let (mut wrong_clears, mut false_refusals, mut overlapping, mut apart) =
        (Vec::new(), Vec::new(), 0, 0);
    for o in [-1.5, -1.0, -0.5, 0.0, 0.5] {
        for deg in [90.0f64, 60.0, 30.0] {
            let (c, s) = (deg.to_radians().cos(), deg.to_radians().sin());
            // The upper bar's axis `d`, and `n` across it in the plane.
            let (d, n) = ([c, s, 0.0], [-s, c, 0.0]);
            for shift in [-1.0, 0.0, 0.5] {
                for lift in [-0.01, 0.0, 0.01] {
                    let p = [
                        o - d[0] * (1.0 - shift),
                        -d[1] * (1.0 - shift),
                        2.0 + lift,
                    ];
                    let upper: Solid3 = (
                        p,
                        [2.0 * d[0], 2.0 * d[1], 0.0],
                        [n[0], n[1], 1.0],
                        [-n[0], -n[1], 1.0],
                    );
                    let mut body = Body::<f64>::new();
                    graft_parallelepiped(&mut body, lower.0, lower.1, lower.2, lower.3);
                    graft_parallelepiped(&mut body, upper.0, upper.1, upper.2, upper.3);
                    let overlap = overlap_depth(lower, upper) > 1e-6;
                    let refused = refused(&body);
                    let pose = (o, deg, shift, lift);
                    if overlap {
                        overlapping += 1;
                        if !refused {
                            wrong_clears.push(pose);
                        }
                    } else {
                        apart += 1;
                        if refused {
                            false_refusals.push(pose);
                        }
                    }
                }
            }
        }
    }
    println!(
        "crossed-ridge sweep: {overlapping} overlapping, {apart} not; wrong clears {}, \
         false refusals {}: {false_refusals:?}",
        wrong_clears.len(),
        false_refusals.len()
    );
    assert_eq!(overlapping + apart, 135);
    assert!(wrong_clears.is_empty(), "{wrong_clears:?}");
}
