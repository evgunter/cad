//! **The plane × torus germ frame, through the public ops**: the poses
//! of `docs/GERM-TORUS-PLANE-FRAME-SPEC.md` §0, each under ∪ and ∩ in
//! both member orders and both differences.
//!
//! `D` is the donut, `R = 2`, `r = 0.5` about `y`, split on its
//! equators into an inner and an outer face. A half-space is a 12 m
//! square bar whose start cap is the named plane. What each pose meets:
//!
//! - a plane through the axis, square to it, or parallel to it short of
//!   the inner equator passes the frame, which serves each germ its own
//!   loop's frame, and stops at the germ-pair dispatch, which has no
//!   torus arm (`CurvedBooleanUnsupported` naming the torus);
//! - any other transverse plane cuts a general spiric quartic and
//!   refuses at the frame, naming the loops the section certificate
//!   reads (`GermSectionSpiric`);
//! - a plane touching the torus keeps its on-carrier refusal, ahead of
//!   the frame.
//!
//! The matcher's segments are read too: on a two-loop section each
//! joins two sites of one loop.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common::germ_pair::cyl;
use crate::common::operands::{bar, framed_bar};
use crate::conic_edge_curved_face::ball;
use crate::revolve_common;
use geom_core::{Point2, Point3, Tol, Vec3};
use profile::{RawLoop, test_support::bulge_loop};
use revolve_common::{axis_y, validated};
use sweep::test_support::finished;
use sweep::{Revolution, revolve};
use topo::{AtRestBody, BooleanError, BooleanOp, SpiricLoops};

/// A ring torus about `y`, `R`, `r`, as one revolve.
fn torus(big_r: f64, r: f64) -> AtRestBody<f64> {
    let lp = bulge_loop(vec![
        (Point2::new(big_r, -r), 1.0),
        (Point2::new(big_r, r), 1.0),
    ]);
    let t = revolve(
        &validated(vec![lp]),
        axis_y(),
        Revolution::Full,
        Tol::witness(),
    )
    .expect("the torus revolves")
    .body;
    finished("the torus", t, Tol::witness())
}

/// The donut `D`.
fn donut() -> AtRestBody<f64> {
    torus(2.0, 0.5)
}

/// The half-space bounded by the plane through `o` with normal `n`, on
/// the side `n` points to (`n` not along `y`).
fn half_space(o: Point3<f64>, n: Vec3<f64>) -> AtRestBody<f64> {
    finished(
        "the half-space",
        framed_bar(o, n, 0.0, 12.0, 12.0),
        Tol::witness(),
    )
}

/// The half-space `y ≥ y0`.
fn above(y0: f64) -> AtRestBody<f64> {
    bar((-6.0, 6.0), (y0, y0 + 12.0), (-6.0, 6.0))
}

/// A `2w`-square bar along `z` across the hole of the torus `(R, r)`,
/// cut to the length that puts its eight corners on the inner face.
fn corner_bar(big_r: f64, r: f64, w: f64) -> AtRestBody<f64> {
    let rho = big_r - (r * r - w * w).sqrt();
    let z = (rho * rho - w * w).sqrt();
    bar((-w, w), (-w, w), (-z, z))
}

fn degrees(a: f64) -> f64 {
    a.to_radians()
}

/// TANG's pose: the full torus `R = 2`, `r = 1` about `y`, and a tube of
/// radius `2 + 1/√2` about `y` standing on it at `y = 1/√2`, its rim on
/// the torus's 45° latitude.
fn tang() -> (AtRestBody<f64>, AtRestBody<f64>) {
    let s = core::f64::consts::FRAC_1_SQRT_2;
    let lp = profile::ProfileLoop::polygon([
        Point2::new(0.0, s),
        Point2::new(2.0 + s, s),
        Point2::new(2.0 + s, s + 1.0),
        Point2::new(0.0, s + 1.0),
    ]);
    let tube = revolve(
        &validated(vec![lp]),
        axis_y(),
        Revolution::Full,
        Tol::witness(),
    )
    .expect("the tube revolves")
    .body;
    (torus(2.0, 1.0), finished("the tube", tube, Tol::witness()))
}

type Outcome = Result<topo::BooleanResult<f64>, BooleanError>;

/// The six runs of a torus solid `t` against a planar solid `p`, each
/// labelled.
fn six(t: &AtRestBody<f64>, p: &AtRestBody<f64>) -> Vec<(&'static str, Outcome)> {
    let tol = Tol::witness();
    vec![
        ("T ∪ P", topo::union(t, p, tol)),
        ("P ∪ T", topo::union(p, t, tol)),
        ("T ∩ P", topo::intersect(t, p, tol)),
        ("P ∩ T", topo::intersect(p, t, tol)),
        ("T ∖ P", topo::subtract(t, p, tol)),
        ("P ∖ T", topo::subtract(p, t, tol)),
    ]
}

/// A torus refusal at the germ-pair dispatch, or at the on-carrier
/// sector ahead of the frame: the two raise one payload.
fn names_the_torus(r: &Outcome) -> bool {
    matches!(
        r,
        Err(BooleanError::CurvedBooleanUnsupported {
            kind: geom::SurfaceKind::Torus,
            ..
        })
    )
}

/// **Every pose the frame serves passes it**, in every op, and stops at
/// the dispatch: the meridian planes (M1, M2, and M3's box face), the
/// planes square to the axis (P1, and P4's box face), the
/// axis-parallel plane short of the inner equator (S1), the bars across
/// the hole (B1) and through the tube (B2), and the corner bar on a fat
/// torus (B5, B5′), whose end squares cut two spiric ovals.
#[test]
fn every_served_pose_passes_the_frame_to_the_dispatch() {
    let d = donut();
    let fat = torus(1.0, 0.6);
    let m2 = Vec3::new(degrees(30.0).cos(), 0.0, degrees(30.0).sin());
    for (pose, t, p) in [
        ("M1", &d, half_space(Point3::origin(), Vec3::unit_x())),
        ("M2", &d, half_space(Point3::origin(), m2)),
        ("M3", &d, bar((0.0, 1.0), (-1.0, 1.0), (1.2, 2.8))),
        ("P1", &d, above(0.2)),
        ("P4", &d, bar((-1.0, 1.0), (0.2, 1.0), (1.2, 2.8))),
        (
            "S1",
            &d,
            half_space(Point3::new(0.3, 0.0, 0.0), Vec3::unit_x()),
        ),
        ("B1", &d, bar((-0.1, 0.1), (-0.1, 0.1), (-2.0, 2.0))),
        ("B2", &d, bar((-0.05, 0.05), (0.25, 0.35), (1.0, 3.0))),
        ("B5", &fat, corner_bar(1.0, 0.6, 0.1)),
        ("B5′", &fat, corner_bar(1.0, 0.6, 0.3)),
    ] {
        for (op, r) in six(t, &p) {
            assert!(names_the_torus(&r), "{pose}, {op}: {:?}", r.err());
        }
    }
}

/// **A general pose refuses at its frame, by name, with the loops the
/// certificate reads**: one null oval past the inner equator (S3, the
/// axis-parallel plane `x = 1.8`) and on B3's tilted bar; two loops
/// round the tube on the oblique planes O1 (20° off the axis, through
/// `(0, 0.1, 0)`) and O2 (20° off the meridian normal, through the
/// centre); two loops round the axis on a plane 3° off axis-normal
/// through `(0, 0.2, 0)`. B3's pierce escalates ahead of the frame at a
/// coarse run band, and says so.
#[test]
fn a_general_pose_refuses_at_its_frame_naming_its_loops() {
    let d = donut();
    let tilted = |a: f64, at: Point3<f64>| {
        half_space(at, Vec3::new(degrees(a).sin(), degrees(a).cos(), 0.0))
    };
    let b3 = finished(
        "B3",
        framed_bar(
            Point3::new(0.0, 0.25, 2.0),
            Vec3::new(0.0, 0.3, 1.0),
            -1.0,
            1.0,
            0.02,
        ),
        Tol::witness(),
    );
    for (pose, p, want) in [
        (
            "S3",
            half_space(Point3::new(1.8, 0.0, 0.0), Vec3::unit_x()),
            SpiricLoops::Null,
        ),
        ("B3", b3, SpiricLoops::Null),
        (
            "O1",
            tilted(20.0, Point3::new(0.0, 0.1, 0.0)),
            SpiricLoops::AroundTube,
        ),
        (
            "O2",
            tilted(70.0, Point3::origin()),
            SpiricLoops::AroundTube,
        ),
        (
            "3° off axis-normal",
            tilted(3.0, Point3::new(0.0, 0.2, 0.0)),
            SpiricLoops::AroundAxis,
        ),
    ] {
        for (op, r) in six(&d, &p) {
            // B3's bar is 0.02 wide, and its pierce sector's curvature
            // margin (5.2e-6 m) lies in a coarse run's band: there the
            // pierce escalates ahead of the frame, which is not reached.
            if let Err(BooleanError::Escalated {
                decision: topo::BooleanDecision::PierceCurvature,
                diag,
            }) = &r
                && pose == "B3"
            {
                println!(
                    "SKIPPED {pose}, {op}: the run's band escalates the pierce first: {diag:?}"
                );
                continue;
            }
            assert!(
                matches!(
                    &r,
                    Err(BooleanError::GermSectionSpiric { loops, .. }) if *loops == want
                ),
                "{pose}, {op}: want {want:?}, got {:?}",
                r.err()
            );
        }
    }
}

/// **The corner bar on the donut refuses at its end square**, in every
/// op: the end squares stand past the inner equator (`z² − (R − r)² =
/// 2(r − s)(R − r − s) > 0`), so each cuts one null oval, a general
/// pose. The plane the refusal names is an end square, square to `z`.
#[test]
fn the_donuts_corner_bar_refuses_at_its_end_square() {
    let d = donut();
    let b = corner_bar(2.0, 0.5, 0.1);
    for (op, r) in six(&d, &b) {
        let Err(BooleanError::GermSectionSpiric {
            a_face,
            a_kind,
            b_face,
            loops: SpiricLoops::Null,
            ..
        }) = r
        else {
            panic!("{op}: {:?}", r.err());
        };
        let face = if a_kind == geom::SurfaceKind::Plane {
            a_face
        } else {
            b_face
        };
        let surface = b.get_surface(b.get_face(face).unwrap().surface).unwrap();
        let geom::Surface::Plane { normal, .. } = surface else {
            panic!("{op}: the refusal's planar face is not a plane: {surface:?}");
        };
        assert_eq!(
            (normal.x, normal.y),
            (0.0, 0.0),
            "{op}: an end square is square to z, got {normal:?}"
        );
    }
}

/// **TANG's tube passes the frame** in every op, both member orders: the
/// tube's rim is an edge lying on the torus, so past the frame its germ
/// takes the edge-plane lane and stops at the wall side's section, which
/// has no plane × torus arm.
#[test]
fn tangs_tube_passes_the_frame_to_the_wall_side_section() {
    let (t, p) = tang();
    for (op, r) in six(&t, &p) {
        assert!(
            matches!(
                &r,
                Err(BooleanError::Join(topo::SplitJoinError::SectionInvariant { what, .. }))
                    if what.contains("plane×torus")
            ),
            "{op}: {:?}",
            r.err()
        );
    }
}

/// **A plane touching the torus keeps its on-carrier refusal**, ahead of
/// the frame: square to the axis on the tube's top (P3, `y = 0.5`),
/// parallel to it on the inner equator (S2, `x = 1.5`) and on the outer
/// one (S4, `x = 2.5`), and the bitangent plane through the centre
/// (V1, `asin(r/R)` off the equator plane).
#[test]
fn a_touching_plane_keeps_its_on_carrier_refusal() {
    let d = donut();
    let villarceau = (0.5_f64 / 2.0).asin();
    for (pose, p) in [
        ("P3", above(0.5)),
        ("S2", half_space(Point3::new(1.5, 0.0, 0.0), Vec3::unit_x())),
        ("S4", half_space(Point3::new(2.5, 0.0, 0.0), Vec3::unit_x())),
        (
            "V1",
            half_space(
                Point3::origin(),
                Vec3::new(villarceau.sin(), villarceau.cos(), 0.0),
            ),
        ),
    ] {
        for (op, r) in six(&d, &p) {
            assert!(names_the_torus(&r), "{pose}, {op}: {:?}", r.err());
        }
    }
}

/// The matcher's segments for `op` on `x`, `y`, every germ consumed.
fn segments(
    op: BooleanOp,
    x: &AtRestBody<f64>,
    y: &AtRestBody<f64>,
    what: &str,
) -> Vec<[Point3<f64>; 2]> {
    let (records, segments) = topo::test_support::boolean_segment_sites(op, x, y, Tol::witness())
        .unwrap_or_else(|e| panic!("{what}: the matcher refused {e:?}"))
        .unwrap_or_else(|| panic!("{what}: the solids cross"));
    assert_eq!(segments.len(), records, "{what}: every germ is consumed");
    segments
}

/// The ops whose segments are read, with whether the operands swap.
const ORDERS: [(BooleanOp, bool); 5] = [
    (BooleanOp::Union, false),
    (BooleanOp::Union, true),
    (BooleanOp::Intersect, false),
    (BooleanOp::Subtract, false),
    (BooleanOp::Subtract, true),
];

/// **Every segment joins two sites of one loop.** On the meridian planes
/// (M1, and M2 turned 30° round the axis) and the axis-parallel plane
/// S1, each loop holds two sites, where the donut's equators cross the
/// plane, and the two loops stand a whole diameter of the ring apart:
/// the side of the axis's trace in the plane tells them. A matcher that
/// faced germs across the loops pairs each site with the other loop's,
/// in every op.
#[test]
fn every_segment_joins_two_sites_of_one_loop() {
    let d = donut();
    let m2 = Vec3::new(degrees(30.0).cos(), 0.0, degrees(30.0).sin());
    for (pose, n, p) in [
        (
            "M1",
            Vec3::unit_x(),
            half_space(Point3::origin(), Vec3::unit_x()),
        ),
        ("M2", m2, half_space(Point3::origin(), m2)),
        (
            "S1",
            Vec3::unit_x(),
            half_space(Point3::new(0.3, 0.0, 0.0), Vec3::unit_x()),
        ),
    ] {
        // The side of the axis's trace: `ŵ = â × n̂`, `â = ŷ`.
        let w = Vec3::unit_y().cross(n);
        for (op, swap) in ORDERS {
            let what = format!("{pose}, {op:?}{}", if swap { " swapped" } else { "" });
            let (x, y) = if swap { (&p, &d) } else { (&d, &p) };
            let segments = segments(op, x, y, &what);
            assert_eq!(segments.len(), 4, "{what}: two arcs on each loop");
            for [s, e] in segments {
                let side = |q: Point3<f64>| (q - Point3::origin()).dot(w);
                assert!(
                    side(s).abs() > 1.0 && side(s).signum() == side(e).signum(),
                    "{what}: a segment joins the two loops: {s:?} → {e:?}"
                );
            }
        }
    }
}

/// **The coaxial cylinder × sphere pair never pairs across its two
/// circles** (the spec's Q6, measured): its one frame serves both
/// circles, and nothing in the frame tells them apart. A ball of radius
/// `0.52` on a `0.5` rod puts the circles `0.29` apart, nearer than the
/// half-turn along either, and each holds two sites, where the rod's
/// seams cross it.
#[test]
fn the_coaxial_cylinder_sphere_pair_keeps_each_circle_to_itself() {
    let tol = Tol::witness();
    let rod = finished("the rod", cyl(0.5, 2.0), tol);
    let b = ball(0.52, [0.0, 0.0, 0.0]);
    for (op, swap) in ORDERS {
        let what = format!("{op:?}{}", if swap { " swapped" } else { "" });
        let (x, y) = if swap { (&b, &rod) } else { (&rod, &b) };
        let segments = segments(op, x, y, &what);
        assert_eq!(segments.len(), 4, "{what}: two arcs on each circle");
        for [s, e] in segments {
            assert!(
                s.z.abs() > 0.1 && s.z.signum() == e.z.signum(),
                "{what}: a segment joins the two circles: {s:?} → {e:?}"
            );
        }
    }
}
