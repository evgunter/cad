//! **No fillet corner patch holds a chart pole off its vertices, and
//! every one encodes its side.** The rule (`geom-brep` README C12 (7)):
//! a chart singularity inside a face is a vertex of it. A corner patch
//! whose pole lies inside it reads `MaterialSign::Unencoded` — its loop
//! has a pole off it on each side — so its sense bit goes uncrossed.
//!
//! Over a corpus of corner patches — a cube's convex corners at six
//! rigid poses and two scales, the vented cavity's concave ones, the
//! slim-wedge skewed cavity at two scales, the oblique trihedra of
//! `m5_pr12_fix_pass`'s `f4` clip, of a void cut in its shape and of
//! clips of an acute and an obtuse prism, and a cube at `Interval` —
//! every corner face reads `MaterialSign::Encoded` agreeing with its
//! stored bit, and each pole of its sphere chart is one of its vertices
//! or lies outside it. That is the rule; the stored chart is held to
//! its construction too (`sweep::blend::arms::corner_chart`): one pole
//! IS a vertex, and the sampled boundary sits in one `u` window inside
//! `[0, π)`.
//!
//! "Outside" is read off the face's own vertices, not the chart: a
//! corner patch's sides are great-circle arcs, so the patch is the
//! spherical polygon its vertex directions span, and a pole strictly
//! inside it is a strictly positive combination of a fan triangle's
//! three vertex directions.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common::cavity::{brick, cavity_edges, cut, rod, skewed_cavity_edges, vented_cavity};
use crate::common::interval::iv;
use crate::common::oblique;
use crate::common::poses::poses;
use std::f64::consts::PI;

use geom::Surface;
use geom_core::{Band, Bounds, Decide, Interval, Point2, Point3, Sign, Tol, Vec3};

use sweep::blend::arms::corner_ball;
use sweep::blend::build::fillet_edges;
use sweep::blend::{Convexity, Filleted};
use sweep::test_support::{at_rest, corners, cube, prism};
use topo::{Body, EdgeKey, LoopBoundary, query};

fn mid<T: Bounds>(x: T) -> f64 {
    0.5 * (x.lo() + x.hi())
}

fn mid3<T: Bounds>(p: Point3<T>) -> Vec3<f64> {
    Vec3::new(mid(p.x), mid(p.y), mid(p.z))
}

/// Whether direction `p` is a strictly positive combination of `a`,
/// `b`, `c` (Cramer over the 3×3 solve), at a relative margin.
fn strictly_spans(p: Vec3<f64>, [a, b, c]: [Vec3<f64>; 3]) -> bool {
    let det = a.dot(b.cross(c));
    let w = [
        p.dot(b.cross(c)) / det,
        a.dot(p.cross(c)) / det,
        a.dot(b.cross(p)) / det,
    ];
    w.iter().all(|&x| x > 1e-9)
}

/// Checks every corner face of `out`; returns how many it read.
fn every_corner_encodes_and_holds_no_pole<T: Decide + Bounds>(
    tag: &str,
    out: &Filleted<T>,
) -> usize {
    let body = &out.body;
    let band = Band::linear(Tol::witness()).unwrap();
    for &face in &out.corner_faces {
        let f = body.get_face(face).expect("a minted corner face");
        let surface = body.get_surface(f.surface).expect("its surface");
        let Surface::Sphere {
            center,
            radius,
            axis,
            u_ref,
        } = *surface
        else {
            panic!("{tag}: a fillet corner patch is a sphere");
        };
        let (outer, _) = topo::props::loop_edges(body, f.outer).expect("the loop reads");
        let want = if f.sense {
            Sign::Positive
        } else {
            Sign::Negative
        };
        assert_eq!(
            geom_brep::props::boundary_material_sign(surface, &outer, band),
            Ok(geom_brep::props::MaterialSign::Encoded(want)),
            "{tag}: corner face {face:?} encodes its side, and the side is its bit's"
        );

        let LoopBoundary::Cycle { first } = body.get_loop(f.outer).unwrap().boundary else {
            panic!("{tag}: a minted face's boundary is a cycle");
        };
        let c = mid3(center);
        let verts: Vec<Vec3<f64>> = body
            .loop_cycle(first)
            .unwrap()
            .into_iter()
            .map(|he| {
                let vk = body.get_half_edge(he).unwrap().start;
                let q = *body
                    .get_vertex(vk)
                    .and_then(|v| body.get_point(v.point))
                    .unwrap();
                (mid3(q) - c).normalize()
            })
            .collect();
        assert!(verts.len() >= 3, "{tag}: a corner patch has three corners");
        let r = mid(radius);
        let axis = mid3(Point3::new(axis.x, axis.y, axis.z));
        let u_ref = mid3(Point3::new(u_ref.x, u_ref.y, u_ref.z));
        assert!(
            verts
                .iter()
                .any(|v| (*v - axis).norm() * r < 1e-9 || (*v + axis).norm() * r < 1e-9),
            "{tag}: corner face {face:?}'s pole {axis:?} is on none of its vertices {verts:?}"
        );
        let (mut lo, mut hi) = (f64::INFINITY, f64::NEG_INFINITY);
        for e in &outer {
            let (t0, t1) = (mid(e.t0), mid(e.t1));
            for k in 0..=32 {
                let t = t0 + (t1 - t0) * f64::from(k) / 32.0;
                let d = (mid3(e.carrier.eval(T::from_f64(t))) - c).normalize();
                if d.cross(axis).norm() < 1e-6 {
                    continue;
                }
                let u = d.dot(axis.cross(u_ref)).atan2(d.dot(u_ref));
                lo = lo.min(u);
                hi = hi.max(u);
            }
        }
        assert!(
            lo > -1e-9 && hi < PI,
            "{tag}: corner face {face:?}'s boundary spans u [{lo}, {hi}], not one window in \
             [0, π)"
        );
        for pole in [axis, -axis] {
            let on_vertex = verts.iter().any(|v| (*v - pole).norm() * r < 1e-9);
            let inside = (1..verts.len() - 1)
                .any(|i| strictly_spans(pole, [verts[0], verts[i], verts[i + 1]]));
            assert!(
                on_vertex || !inside,
                "{tag}: corner face {face:?}'s pole {pole:?} lies inside it off its vertices \
                 {verts:?}"
            );
        }
    }
    out.corner_faces.len()
}

fn fillet_all(tag: &str, body: &Body<f64>, edges: &[EdgeKey], r: f64) -> Filleted<f64> {
    fillet_edges(&at_rest(body, Tol::witness()), edges, r, Tol::witness())
        .unwrap_or_else(|e| panic!("{tag}: the fillet builds: {e:?}"))
}

#[test]
fn every_corner_patch_encodes_its_side_and_holds_no_pole_off_a_vertex() {
    let mut read = 0;
    for scale in [2.0, 0.05] {
        for (pose, a) in poses() {
            let tag = format!("cube {scale} at {pose}");
            let body = topo::transform_rigid(
                &cube::<f64>(scale, Tol::witness()),
                &geom_core::test_support::rigid(&a, Tol::witness()),
                Tol::witness(),
            )
            .unwrap();
            let edges = query::all_edges(&body);
            let out = fillet_all(&tag, &body, &edges, 0.15 * scale);
            read += every_corner_encodes_and_holds_no_pole(&tag, &out);
        }
    }

    let cavity = vented_cavity();
    let out = fillet_all("the vented cavity", &cavity, &cavity_edges(&cavity), 0.15);
    read += every_corner_encodes_and_holds_no_pole("the vented cavity", &out);

    for (theta, scale) in [(0.5, 1.0), (0.2, 0.1)] {
        let tag = format!("the skewed cavity at {theta} x{scale}");
        let (body, edges) = skewed_cavity_edges(theta, scale);
        let out = fillet_all(&tag, &body, &edges, 0.05 * scale);
        read += every_corner_encodes_and_holds_no_pole(&tag, &out);
    }

    let clip = oblique::f4();
    let out = fillet_all("the f4 oblique clip", &clip, &query::all_edges(&clip), 0.08);
    read += every_corner_encodes_and_holds_no_pole("the f4 oblique clip", &out);

    // Oblique clips of an acute and an obtuse prism: corners whose
    // dihedrals stray far from a right angle.
    let acute: f64 = 0.35;
    for (name, base, angle, ax) in [
        (
            "the acute prism",
            prism(
                corners(&[
                    (0.0, 0.0),
                    (1.2, 0.0),
                    (1.2 * acute.cos(), 1.2 * acute.sin()),
                ]),
                1.0,
                Tol::witness(),
            ),
            0.15,
            Vec3::new(1.0, 2.0, 3.0),
        ),
        (
            "the obtuse prism",
            prism(
                corners(&[(0.0, 0.2), (1.0, 0.0), (2.0, 0.2), (1.0, 1.0)]),
                1.0,
                Tol::witness(),
            ),
            0.4,
            Vec3::new(1.0, -1.0, 0.0),
        ),
    ] {
        let tag = format!("{name} clipped {angle} about {ax:?}");
        let body = oblique::clip(base, angle, ax).expect("the clip");
        let out = fillet_all(&tag, &body, &query::all_edges(&body), 0.03);
        read += every_corner_encodes_and_holds_no_pole(&tag, &out);
    }

    // The same oblique trihedra as a vented void: concave corners whose
    // supports are not orthogonal.
    let clip_at = topo::transform_rigid(
        &clip,
        &geom_core::RigidMotion::translation(Vec3::new(1.5, 1.5, 1.5)).unwrap(),
        Tol::witness(),
    )
    .unwrap();
    let block = brick(Point3::new(0.0, 0.0, 0.0), Point3::new(4.0, 4.0, 4.0));
    let vent = rod(Point2::new(2.0, 2.0), 0.1, 2.0, 5.0);
    let void = cut("oblique void", &cut("vent", &block, &vent), &clip_at);
    let inside = |p: Point3<f64>| p.to_array().iter().all(|c| *c > 1.0 && *c < 3.0);
    let void_edges: Vec<EdgeKey> = query::all_edges(&void)
        .into_iter()
        .filter(|&k| {
            let e = void.get_edge(k).unwrap();
            let line = void
                .get_curve_geom(e.curve)
                .and_then(|g| g.certified())
                .is_some_and(|c| matches!(c.carrier(), geom::Curve3::Line { .. }));
            let at = |vk| {
                *void
                    .get_vertex(vk)
                    .and_then(|x| void.get_point(x.point))
                    .unwrap()
            };
            let (a, b) = (
                void.get_half_edge(e.he_plus).unwrap().start,
                void.half_edge_end(e.he_plus).unwrap(),
            );
            line && inside(at(a)) && inside(at(b))
        })
        .collect();
    assert_eq!(void_edges.len(), 12, "the oblique void's twelve edges");
    let out = fillet_all("the oblique void", &void, &void_edges, 0.05);
    read += every_corner_encodes_and_holds_no_pole("the oblique void", &out);

    // The Interval lane: the cube's eight corners.
    let body = cube::<Interval>(2.0, Tol::witness());
    let edges = query::all_edges(&body);
    let out = fillet_edges(
        &at_rest(&body, Tol::witness()),
        &edges,
        iv(0.3),
        Tol::witness(),
    )
    .expect("the cube fillets at Interval");
    read += every_corner_encodes_and_holds_no_pole("the cube at Interval", &out);

    // 12 cubes, the cavity, two skewed cavities, the clip, the void
    // and the Interval cube: eight corners each; the two prism clips,
    // sixteen between them.
    assert_eq!(read, 18 * 8 + 16, "every corner of the corpus was read");
}

/// **The arm's own chart, as a value**: `corner_ball`'s pole is the
/// third foot or its antipode, and neither pole is strictly inside the
/// spherical triangle of its side's feet — over a box trihedron and
/// two oblique ones (one near-degenerate: two supports `0.2` rad
/// apart), on both sides, at `f64` and at `Interval`.
#[test]
fn the_corner_balls_pole_is_a_foot_or_its_antipode_on_either_side() {
    let trihedra = [
        [
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
        ],
        [
            Vec3::new(0.0, -0.958, -0.286),
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.3, 0.4, -0.866),
        ],
        [
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.2f64.cos(), 0.2f64.sin(), 0.0),
            Vec3::new(-0.3, 0.5, 0.8),
        ],
    ];
    for normals in trihedra {
        let normals = normals.map(Vec3::normalize);
        for side in [Convexity::Convex, Convexity::Concave] {
            let sign = if matches!(side, Convexity::Convex) {
                1.0
            } else {
                -1.0
            };
            let feet = normals.map(|n| n * sign);
            let f64_axis = match corner_ball([Point3::origin(); 3], normals, 0.1, side).surface {
                Surface::Sphere { axis, .. } => axis,
                _ => panic!("the corner ball is a sphere"),
            };
            let iv_normals = normals.map(|n| n.map(iv));
            let iv_axis =
                match corner_ball([Point3::origin().map(iv); 3], iv_normals, iv(0.1), side).surface
                {
                    Surface::Sphere { axis, .. } => mid3(Point3::new(axis.x, axis.y, axis.z)),
                    _ => panic!("the corner ball is a sphere"),
                };
            for (lane, axis) in [("f64", f64_axis), ("Interval", iv_axis)] {
                let tag = format!("{lane} {side} {normals:?}");
                assert!(
                    (axis - feet[2]).norm() < 1e-12 || (axis + feet[2]).norm() < 1e-12,
                    "{tag}: the pole {axis:?} is the third foot {:?} up to sign",
                    feet[2]
                );
                for pole in [axis, -axis] {
                    let on_vertex = feet.iter().any(|f| (*f - pole).norm() < 1e-12);
                    assert!(
                        on_vertex || !strictly_spans(pole, feet),
                        "{tag}: pole {pole:?} lies inside the patch off its vertices"
                    );
                }
            }
        }
    }
}
