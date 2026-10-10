//! **A box edge through a cone's wall, where the section lands as a
//! ring.** The box is turned so both faces at its edge cut the cone in
//! ellipses, and the edge pierces the wall twice: the corners of the
//! lune between the two sections, which the join's ring lane winds
//! (`chord_join::path_island_winding`, held to its oracle on a cone
//! sheet in `topo`'s `chord_join::cone_ring_rows`).
//!
//! The crossing sweep's crossings are held to the closed form here
//! (`topo::sweep_split`, `sweep-testing` only), and each
//! op's answer to the closed-form overlap, the ring kept on the cone
//! face in ∪ and cone ∖ box.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{FRAC_PI_2, FRAC_PI_6};

use crate::revolve_common::{axis_y, validated};
use geom_core::{Affine3, Point2, Point3, Tol, Vec3};
use profile::{ProfileLoop, RawLoop};
use sweep::test_support::{brick, finished};
use sweep::{Revolution, revolve};
use topo::{AtRestBody, BooleanDeclarations};

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

/// The cone ∩ box at scale `s`, by an exact 1-D quadrature. The box's
/// section is constant along `z`, so each slice `y` of the overlap is
/// the cone's disc of radius `ρ(y)` cut to `x ≥ d(y)`, `d` the larger of
/// the box's two faces' traces: the segment area
/// `ρ²·acos(d/ρ) − d·√(ρ² − d²)`. That integrand is smooth between
/// breakpoints taken in closed form (where `d` changes face, and where
/// `d = ±ρ` on either face), and each piece takes 1000 panels of 5-point
/// Gauss–Legendre: 0.057152624892371 at `s = 0.6`, within 1e-13 of a
/// 30-digit adaptive quadrature.
pub(crate) fn overlap(s: f64) -> f64 {
    const GL: [(f64, f64); 5] = [
        (-0.906_179_845_938_664, 0.236_926_885_056_189_1),
        (-0.538_469_310_105_683_1, 0.478_628_670_499_366_5),
        (0.0, 0.568_888_888_888_888_9),
        (0.538_469_310_105_683_1, 0.478_628_670_499_366_5),
        (0.906_179_845_938_664, 0.236_926_885_056_189_1),
    ];
    let (h, k) = (height(), FRAC_PI_6.tan());
    let e = edge_point(s);
    let t = 50f64.to_radians().tan();
    // The two faces' traces `d = a·y + b`.
    let faces = [(t, e.x - t * e.y), (-1.0 / t, e.x + e.y / t)];
    let slice = |y: f64| {
        let r = (h - y) * k;
        let d = faces
            .iter()
            .map(|(a, b)| a * y + b)
            .fold(f64::NEG_INFINITY, f64::max);
        if d >= r {
            0.0
        } else if d <= -r {
            core::f64::consts::PI * r * r
        } else {
            r * r * (d / r).acos() - d * (r * r - d * d).sqrt()
        }
    };
    let mut cuts = vec![0.0, h, e.y];
    for (a, b) in faces {
        cuts.extend(
            [(h * k - b) / (a + k), (-h * k - b) / (a - k)]
                .into_iter()
                .filter(|y| (0.0..h).contains(y)),
        );
    }
    cuts.sort_by(f64::total_cmp);
    let panels = 1000;
    cuts.windows(2)
        .map(|w| {
            let step = (w[1] - w[0]) / f64::from(panels);
            (0..panels)
                .map(|i| {
                    let m = w[0] + (f64::from(i) + 0.5) * step;
                    GL.iter()
                        .map(|(x, wt)| wt * slice(m + x * step / 2.0))
                        .sum::<f64>()
                        * step
                        / 2.0
                })
                .sum::<f64>()
        })
        .sum()
}

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
/// tier 3 and `point_in_solid`; so do ∪ and cone ∖ box, which keep the
/// ring on the cone face, its volume read off every loop's vector area
/// (`geom_brep::props::cone_face_closed_form`).
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
                let (sa, sb, _, _) = topo::sweep_split(a, b, tol)
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
                // The cone face the box's tilted sections bound has no
                // trim reading: as built, 16 grid points refuse
                // `PartialConeFace` in ∩ and box ∖ cone, and in ∪ and
                // cone ∖ box, which keep the ring, 20 at the lune's scale
                // and 52 at a tenth of it, at every ε row and member
                // order. Tilted, at a tenth of the scale, one point
                // refuses in those two at the 1e-6 row; otherwise none
                // does.
                let as_built = pose_name == "as built";
                let partial = if as_built { 16 } else { 0 };
                let ring_partial = match (as_built, s == 0.1) {
                    (false, false) => 0,
                    (false, true) => 1,
                    (true, false) => 20,
                    (true, true) => 52,
                };
                let vc = core::f64::consts::PI * R * R * height() / 3.0;
                // ∪ keeps the lune as a ring on the cone face.
                let union = topo::union_with(a, b, &none, tol);
                solid_truth::assert_is_but(
                    &format!("{label}, {order}, ∪"),
                    &union,
                    Want::Body(vc + 216.0 - overlap(s), 1e-9 * 216.0),
                    &|q| Op::Union.depth(&sc, &sx, q),
                    &[],
                    &points,
                    ring_partial,
                );
                let inter = topo::intersect_with(a, b, &none, tol);
                let vi = solid_truth::assert_is_but(
                    &format!("{label}, {order}, ∩"),
                    &inter,
                    Want::Body(overlap(s), 1e-9 * overlap(s)),
                    &|q| Op::Intersect.depth(&sc, &sx, q),
                    &[],
                    &points,
                    partial,
                );
                let diff = topo::subtract_with(a, b, &none, tol);
                if cone_first {
                    // The cone less the box keeps the ring too.
                    solid_truth::assert_is_but(
                        &format!("{label}, {order}, cone ∖ box"),
                        &diff,
                        Want::Body(vc - vi, 1e-9 * vc),
                        &|q| Op::Subtract.depth(&sc, &sx, q),
                        &[],
                        &points,
                        ring_partial,
                    );
                } else {
                    solid_truth::assert_is_but(
                        &format!("{label}, {order}, box ∖ cone"),
                        &diff,
                        Want::Body(216.0 - vi, 1e-9 * 216.0),
                        &|q| Op::Subtract.depth(&sx, &sc, q),
                        &[],
                        &points,
                        partial,
                    );
                }
            }
        }
    }
}

/// **The overlap's quadrature agrees with a grid integral.** The
/// midpoint rule over `{cone ∩ box}` at 100³ cells of the overlap's
/// bounding box, read through the two closed-form memberships alone
/// ([`cone_truth`], [`wedge_truth`]), lands within 4.1e-5 of
/// [`overlap`] at the lune's scale (measured), held to 2e-4.
#[test]
fn the_overlap_is_its_grid_integral() {
    let (sc, sx) = (
        cone_truth(Affine3::identity()),
        wedge_truth(0.6, Affine3::identity()),
    );
    let grid = solid_truth::grid_volume(
        |q| Op::Intersect.depth(&sc, &sx, q),
        Point3::new(0.25, 0.3, -0.5),
        Point3::new(1.05, 1.35, 0.5),
        100,
    );
    assert!(
        (grid - overlap(0.6)).abs() <= 2e-4,
        "the grid integral {grid} against the quadrature {}",
        overlap(0.6)
    );
}

/// **The ring's volume at the certified scalar.** T1 built at
/// `Interval` (the cone revolved from the same triangle, the box turned
/// and moved by the same maps, lifted): ∪ and cone ∖ box, which keep
/// the lune as a ring on the cone face, pass tier 3 and their volume
/// enclosures bracket the closed forms (about 1.3e-11 wide, measured).
/// At the 1e-12 row the build stops before the volume, typed: the join
/// certifies each section on its surfaces, and that residual's
/// enclosure, ±1.3e-12 wide, escalates against the band's 1e-12.
#[test]
fn the_rings_volume_brackets_its_closed_form_at_the_certified_scalar() {
    use crate::common::interval::{iv, p2, p3, v2, v3};
    use geom_core::{Bounds, Interval};
    use profile::{Profile, SketchPlane};
    let tol = Tol::witness();
    let tri = ProfileLoop::polygon([p2(0.0, 0.0), p2(R, 0.0), p2(0.0, height())]);
    let profile = Profile::new(SketchPlane::<Interval>::xy(), vec![tri])
        .validate(tol)
        .unwrap();
    let axis = sweep::RevolveAxis {
        origin: p2(0.0, 0.0),
        dir: v2(0.0, 1.0),
    };
    let raw = revolve(&profile, axis, Revolution::Full, tol).unwrap().body;
    let spin = Affine3::rotation_about_axis(p3(0.0, 0.0, 0.0), v3(0.0, 1.0, 0.0), iv(FRAC_PI_2));
    let c = finished(
        "the cone",
        topo::transform_rigid(&raw, &spin, tol).unwrap(),
        tol,
    );
    let e = edge_point(0.6);
    let turn = Affine3::rotation_about_axis(
        p3(0.0, 0.0, 0.0),
        v3(0.0, 0.0, 1.0),
        iv(-50f64.to_radians()),
    );
    let to = Affine3::translation(v3(e.x, e.y, e.z));
    let raw = brick::<Interval>((0.0, 6.0), (0.0, 6.0), (-3.0, 3.0), tol);
    let x = finished(
        "the box",
        topo::transform_rigid(&raw, &(to * turn), tol).unwrap(),
        tol,
    );
    let vc = core::f64::consts::PI * R * R * height() / 3.0;
    let i = overlap(0.6);
    for (op, r, want) in [
        ("cone ∪ box", topo::union(&c, &x, tol), vc + 216.0 - i),
        ("box ∪ cone", topo::union(&x, &c, tol), vc + 216.0 - i),
        ("cone ∖ box", topo::subtract(&c, &x, tol), vc - i),
    ] {
        if tol.eps() <= 1e-12 {
            assert!(
                matches!(
                    &r,
                    Err(topo::BooleanError::Join(topo::SplitJoinError::Euler(
                        topo::EulerOpError::Certification {
                            error: topo::CertifyError::Escalated { .. }
                        }
                    )))
                ),
                "{op}: at ε = {}, the join's certification escalates, got {:?}",
                tol.eps(),
                r.as_ref().map(|r| r.body().map(|b| b.kind))
            );
            continue;
        }
        let bb = match r.as_ref().map(topo::BooleanResult::body) {
            Ok(Some(bb)) => bb,
            _ => panic!("{op}: wanted a body, got {r:?}"),
        };
        topo::validate_geometric(&bb.body, tol).unwrap_or_else(|e| panic!("{op}: tier 3: {e:?}"));
        let v = topo::mass_properties(&bb.body, tol).unwrap().volume;
        assert!(
            v.lo() <= want && want <= v.hi() && v.hi() - v.lo() < 1e-10,
            "{op}: [{}, {}] against {want}",
            v.lo(),
            v.hi()
        );
    }
}

/// **A ringed cone face's sense is checked against its boundary.** In
/// ∪ and cone ∖ box, both member orders where the op has them, the cone
/// face carrying the lune as a ring, its stored sense flipped, fails
/// tier 3 as `CurvedSenseInverted` naming it (check 6): its outer
/// loop's traversal encodes the face's side whatever holes it carries
/// (`geom_brep::props::boundary_material_sign_loops`). The flip also
/// reads as `LaminaWedge` at the face's edges.
#[test]
fn a_ringed_cone_faces_sense_flip_fails_tier_3() {
    let (tol, none) = (Tol::witness(), BooleanDeclarations::none());
    let (c, x) = (cone(Affine3::identity()), wedge(0.6, Affine3::identity()));
    for (op, r) in [
        ("cone ∪ box", topo::union_with(&c, &x, &none, tol)),
        ("box ∪ cone", topo::union_with(&x, &c, &none, tol)),
        ("cone ∖ box", topo::subtract_with(&c, &x, &none, tol)),
    ] {
        let mut body = r.unwrap().body().unwrap().body.clone().into_body();
        let ringed: Vec<_> = body
            .faces()
            .filter(|(_, f)| !f.rings.is_empty())
            .map(|(k, f)| (k, f.sense))
            .collect();
        let [(face, sense)] = ringed[..] else {
            panic!("{op}: one ringed face, got {ringed:?}");
        };
        body.set_face_sense(face, !sense).unwrap();
        let got = topo::validate_geometric(&body, tol);
        assert!(
            matches!(
                &got,
                Err(errors) if errors.iter().any(|e| matches!(
                    e,
                    topo::ValidationError::CurvedSenseInverted { face: f } if *f == face
                ))
            ),
            "{op}: {got:?}"
        );
    }
}
