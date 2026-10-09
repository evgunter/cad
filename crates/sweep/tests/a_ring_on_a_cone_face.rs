//! **A box edge through a cone's wall, where the section lands as a
//! ring.** The box is turned so both faces at its edge cut the cone in
//! ellipses, and the edge pierces the wall twice: the corners of the
//! lune between the two sections, which the join's ring lane winds
//! (`chord_join::path_island_winding`, held to its oracle on a cone
//! sheet in `topo`'s `chord_join::cone_ring_rows`).
//!
//! The crossing sweep's crossings are held to the closed form here
//! (`topo::sweep_split_admitting_cones`, `sweep-testing` only), and each
//! op's answer to the closed-form overlap. Where the ring stays on the
//! cone face, the result door refuses its volume
//! (`work/germ/boolean-sector-algebra-has-no-cone-arm.md`'s D7).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{FRAC_PI_2, FRAC_PI_6};

use crate::revolve_common::{axis_y, validated};
use geom_core::{Affine3, Point2, Point3, Tol, Vec3};
use profile::{ProfileLoop, RawLoop};
use sweep::test_support::{brick, finished};
use sweep::{Revolution, revolve};
use topo::{AtRestBody, BooleanDeclarations, BooleanError};

use crate::common::solid_truth::{self, Op, Solid, Want};

/// The cone's base radius; its half-angle is π/6.
const R: f64 = 1.2;

/// The cone's height, apex to base.
fn height() -> f64 {
    R / FRAC_PI_6.tan()
}

/// The cone of base radius [`R`] at `y = 0`, apex `(0, height, 0)`, its
/// seam meridians turned onto `±z`, then moved by `pose`.
pub(crate) fn cone(pose: Affine3<f64>) -> AtRestBody<f64> {
    let tol = Tol::witness();
    let tri = ProfileLoop::polygon([
        Point2::new(0.0, 0.0),
        Point2::new(R, 0.0),
        Point2::new(0.0, height()),
    ]);
    let body = revolve(&validated(vec![tri]), axis_y(), Revolution::Full, tol)
        .unwrap()
        .body;
    let spin = Affine3::rotation_about_axis(Point3::origin(), Vec3::new(0.0, 1.0, 0.0), FRAC_PI_2);
    finished(
        "the cone",
        topo::transform_rigid(&body, &(pose * spin), tol).unwrap(),
        tol,
    )
}

/// The point of the lune's edge at scale `s`: `s·(0.5, 1.5)` from the
/// apex, out along `+x` and down the axis.
fn edge_point(s: f64) -> Point3<f64> {
    Point3::new(0.5 * s, height() - 1.5 * s, 0.0)
}

/// The box whose edge, along `z` through [`edge_point`], has its two
/// faces' inward normals 40° and 50° off the cone's axis (turned −50°
/// about `z`): each cuts the cone in an ellipse, the far side of one
/// and the apex side of the other holding the lune. Moved by `pose`.
pub(crate) fn wedge(s: f64, pose: Affine3<f64>) -> AtRestBody<f64> {
    let tol = Tol::witness();
    let raw = brick((0.0, 6.0), (0.0, 6.0), (-3.0, 3.0), tol);
    let turn = Affine3::rotation_about_axis(
        Point3::origin(),
        Vec3::new(0.0, 0.0, 1.0),
        -50f64.to_radians(),
    );
    let to = Affine3::translation(edge_point(s) - Point3::origin());
    finished(
        "the box",
        topo::transform_rigid(&raw, &(pose * to * turn), tol).unwrap(),
        tol,
    )
}

/// The cone's closed-form membership, moved by `pose`.
fn cone_truth(pose: Affine3<f64>) -> Solid {
    Solid::Revolved {
        y0: 0.0,
        y1: height(),
        r0: R,
        k: -R / height(),
        window: None,
    }
    .posed(pose)
}

/// The box's closed-form membership at scale `s`, moved by `pose`.
fn wedge_truth(s: f64, pose: Affine3<f64>) -> Solid {
    let turn = Affine3::rotation_about_axis(
        Point3::origin(),
        Vec3::new(0.0, 0.0, 1.0),
        -50f64.to_radians(),
    );
    let to = Affine3::translation(edge_point(s) - Point3::origin());
    Solid::Brick([(0.0, 6.0), (0.0, 6.0), (-3.0, 3.0)]).posed(pose * to * turn)
}

/// The cone ∩ box at scale 0.6: the box's section is constant along
/// `z`, so the overlap is the integral over the box's quadrant of the
/// plane `z = 0` of the cone's chord along `z`, `2√(ρ(y)² − x²)`, by a
/// midpoint rule at 8000² cells, stable to 1e-6 from 2000². The pose is
/// self-similar about the apex, so scale `s` holds `(s/0.6)³` of it.
const OVERLAP: f64 = 0.057_153;

/// The points the bodies are read at: a grid over the apex's
/// neighbourhood at scale `s`, moved by `pose`.
fn truth_grid(s: f64, pose: Affine3<f64>) -> Vec<Point3<f64>> {
    let h = height();
    solid_truth::grid(
        Point3::new(-1.5 * s, h - 2.5 * s, -1.5 * s),
        Point3::new(1.5 * s, h + 0.2 * s, 1.5 * s),
        6,
    )
    .into_iter()
    .map(|q| pose.transform_point(q))
    .collect()
}

/// The edge's crossings of the wall, closed form: at `z = ±√(ρ² − x²)`,
/// `ρ` the wall's radius at the edge's height, moved by `pose`.
fn pierce_points(s: f64, pose: Affine3<f64>) -> [Point3<f64>; 2] {
    let e = edge_point(s);
    let rho = (height() - e.y) * FRAC_PI_6.tan();
    let z = (rho * rho - e.x * e.x).sqrt();
    [z, -z].map(|z| pose.transform_point(Point3::new(e.x, e.y, z)))
}

/// The poses: as built, turned off every axis, and turned upside down
/// and moved.
fn poses() -> [(&'static str, Affine3<f64>); 3] {
    let o = Point3::origin();
    let tilted = Affine3::rotation_about_axis(o, Vec3::new(0.3, -0.5, 0.8), 0.9);
    let flipped = Affine3::translation(Vec3::new(1.5, -0.25, 2.0))
        * Affine3::rotation_about_axis(o, Vec3::new(1.0, 0.0, 0.0), 3.0);
    [
        ("as built", Affine3::identity()),
        ("tilted", tilted),
        ("upside down", flipped),
    ]
}

/// **The sweep crosses the wall at the lune's corners, and every op
/// answers or refuses at the result door.** In each pose, at the lune's
/// scale and a tenth of it (the ring by the apex), in both member orders:
/// the box's split carries exactly the two closed-form crossings as new
/// vertices; ∩ and box ∖ cone build, held to the overlap's closed form,
/// tier 3 and `point_in_solid`; ∪ and cone ∖ box keep the ring on the
/// cone face, whose volume the result door cannot yet read
/// (`ResultInvalid`, `RingOnCurvedFace`), so their opening is a red row.
#[test]
fn a_box_edge_through_a_cone_wall_crosses_it_at_the_rings_corners() {
    let (tol, none) = (Tol::witness(), BooleanDeclarations::none());
    for (pose_name, pose) in poses() {
        for s in [0.6, 0.1] {
            let (c, x) = (cone(pose), wedge(s, pose));
            let label = format!("{pose_name}, scale {s}");
            let want = pierce_points(s, pose);
            for (order, cone_first) in [("cone first", true), ("box first", false)] {
                let (a, b) = if cone_first { (&c, &x) } else { (&x, &c) };
                let (sa, sb, _, _) = topo::sweep_split_admitting_cones(a, b, tol)
                    .unwrap_or_else(|e| panic!("{label}, {order}: the sweep refused {e:?}"));
                let split = if cone_first { sb } else { sa };
                let fresh: Vec<_> = split
                    .vertex_points()
                    .map(|(_, p)| p)
                    .filter(|p| x.vertex_points().all(|(_, q)| (*p - q).norm() > 1e-12))
                    .collect();
                assert_eq!(fresh.len(), 2, "{label}, {order}: new vertices {fresh:?}");
                for q in want {
                    assert!(
                        fresh.iter().any(|p| (*p - q).norm() <= 1e-10),
                        "{label}, {order}: no crossing at {q:?} among {fresh:?}"
                    );
                }
                let (sc, sx) = (cone_truth(pose), wedge_truth(s, pose));
                let points = truth_grid(s, pose);
                // ∪ keeps the ring on the cone face, and the result door
                // has no volume for a ring there yet.
                let union = topo::union_with(a, b, &none, tol);
                assert!(
                    matches!(
                        &union,
                        Err(BooleanError::ResultInvalid { errors })
                            if matches!(
                                errors[..],
                                [topo::ValidationError::VolumeUncomputable {
                                    source: topo::MassPropsError::RingOnCurvedFace { .. },
                                    ..
                                }]
                            )
                    ),
                    "{label}, {order}, ∪: {:?}",
                    union.map(|r| r.body().map(|b| b.kind))
                );
                let inter = topo::intersect_with(a, b, &none, tol);
                let vi = solid_truth::assert_is(
                    &format!("{label}, {order}, ∩"),
                    &inter,
                    Want::Body(OVERLAP * (s / 0.6).powi(3), 2e-4 * (s / 0.6).powi(3)),
                    &|q| Op::Intersect.depth(&sc, &sx, q),
                    &[],
                    &points,
                );
                let diff = topo::subtract_with(a, b, &none, tol);
                if cone_first {
                    // The cone less the box keeps the ring too.
                    assert!(
                        matches!(&diff, Err(BooleanError::ResultInvalid { .. })),
                        "{label}, {order}, cone ∖ box: {:?}",
                        diff.map(|r| r.body().map(|b| b.kind))
                    );
                } else {
                    solid_truth::assert_is(
                        &format!("{label}, {order}, box ∖ cone"),
                        &diff,
                        Want::Body(216.0 - vi, 1e-9 * 216.0),
                        &|q| Op::Subtract.depth(&sx, &sc, q),
                        &[],
                        &points,
                    );
                }
            }
        }
    }
}
