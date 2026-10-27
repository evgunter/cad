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
//!   loop's frame, and the germ-pair dispatch, which takes the torus as
//!   a wall. The chord layer takes an arc of the meridian circle, the
//!   concentric circle or the spiric oval the chord's sites lie on, and
//!   the op builds, or stops past the join (`germ_plane_torus_chord` and
//!   `germ_plane_torus_spiric` hold the bodies to the truth);
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
use crate::germ_plane_torus_chord::{cap, volume_tol};
use crate::revolve_common;
use core::f64::consts::PI;
use geom_core::{Point2, Point3, Tol, Vec3};
use profile::{RawLoop, test_support::bulge_loop};
use revolve_common::{axis_y, validated};
use sweep::test_support::finished;
use sweep::{Revolution, revolve};
use topo::{AtRestBody, BooleanError, BooleanOp, SpiricLoops};

/// A ring torus about `y`, `R`, `r`, as one revolve.
pub(crate) fn torus(big_r: f64, r: f64) -> AtRestBody<f64> {
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

/// A one-face torus about `y`: the one-segment circle about `(R, 0)` of
/// radius `r`, its vertex at profile angle `phase`, revolved whole. Its
/// face closes on itself both ways, across the meridian through the
/// vertex and the parallel through it.
pub(crate) fn one_face_torus(big_r: f64, r: f64, phase: f64) -> AtRestBody<f64> {
    let lp = RawLoop::new([(
        Point2::new(big_r + r * phase.cos(), r * phase.sin()),
        profile::Segment::Arc(geom_core::Arc2 {
            centre: Point2::new(big_r, 0.0),
            radius: r,
            sweep: core::f64::consts::TAU,
        }),
    )]);
    let t = revolve(
        &validated(vec![lp]),
        axis_y(),
        Revolution::Full,
        Tol::witness(),
    )
    .expect("the one-face torus revolves")
    .body;
    finished("the one-face torus", t, Tol::witness())
}

/// The torus `(R, r)` about `y` seamed on `y = 0`: two faces, above and
/// below, meeting on the equators, each closed on itself across the
/// meridian `z = 0`, `x > 0`.
pub(crate) fn seamed_torus(big_r: f64, r: f64) -> AtRestBody<f64> {
    let lp = bulge_loop(vec![
        (Point2::new(big_r + r, 0.0), 1.0),
        (Point2::new(big_r - r, 0.0), 1.0),
    ]);
    let t = revolve(
        &validated(vec![lp]),
        axis_y(),
        Revolution::Full,
        Tol::witness(),
    )
    .expect("the seamed torus revolves")
    .body;
    finished("the seamed torus", t, Tol::witness())
}

/// The donut `D`.
pub(crate) fn donut() -> AtRestBody<f64> {
    torus(2.0, 0.5)
}

/// The half-space bounded by the plane through `o` with normal `n`, on
/// the side `n` points to (`n` not along `y`).
pub(crate) fn half_space(o: Point3<f64>, n: Vec3<f64>) -> AtRestBody<f64> {
    finished(
        "the half-space",
        framed_bar(o, n, 0.0, 12.0, 12.0),
        Tol::witness(),
    )
}

/// The half-space `y ≥ y0`.
pub(crate) fn above(y0: f64) -> AtRestBody<f64> {
    bar((-6.0, 6.0), (y0, y0 + 12.0), (-6.0, 6.0))
}

/// A `2w`-square bar along `z` across the hole of the torus `(R, r)`,
/// cut to the length that puts its eight corners on the inner face.
pub(crate) fn corner_bar(big_r: f64, r: f64, w: f64) -> AtRestBody<f64> {
    let rho = big_r - (r * r - w * w).sqrt();
    let z = (rho * rho - w * w).sqrt();
    bar((-w, w), (-w, w), (-z, z))
}

pub(crate) fn degrees(a: f64) -> f64 {
    a.to_radians()
}

/// TANG's pose: the full torus `R = 2`, `r = 1` about `y`, and a tube of
/// radius `2 + 1/√2` about `y` standing on it at `y = 1/√2`, its rim on
/// the torus's 45° latitude.
pub(crate) fn tang() -> (AtRestBody<f64>, AtRestBody<f64>) {
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

pub(crate) type Outcome = Result<topo::BooleanResult<f64>, BooleanError>;

/// The six runs of a torus solid against a planar solid: each op's
/// label, the op, and whether the planar solid is the first operand.
pub(crate) const SIX: [(&str, BooleanOp, bool); 6] = [
    ("T ∪ P", BooleanOp::Union, false),
    ("P ∪ T", BooleanOp::Union, true),
    ("T ∩ P", BooleanOp::Intersect, false),
    ("P ∩ T", BooleanOp::Intersect, true),
    ("T ∖ P", BooleanOp::Subtract, false),
    ("P ∖ T", BooleanOp::Subtract, true),
];

/// `op` on `x`, `y`.
fn run(op: BooleanOp, x: &AtRestBody<f64>, y: &AtRestBody<f64>) -> Outcome {
    let tol = Tol::witness();
    match op {
        BooleanOp::Union => topo::union(x, y, tol),
        BooleanOp::Intersect => topo::intersect(x, y, tol),
        BooleanOp::Subtract => topo::subtract(x, y, tol),
    }
}

/// The six runs of a torus solid `t` against a planar solid `p`, each
/// labelled.
pub(crate) fn six(t: &AtRestBody<f64>, p: &AtRestBody<f64>) -> Vec<(&'static str, Outcome)> {
    SIX.iter()
        .map(|&(label, op, swap)| {
            let (x, y) = if swap { (p, t) } else { (t, p) };
            (label, run(op, x, y))
        })
        .collect()
}

/// Where a torus refusal stands. The on-carrier sector (F0) and the
/// germ-pair dispatch (F2) raise one payload, `CurvedBooleanUnsupported`
/// naming the torus, so the op's error alone cannot tell them apart.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum TorusDoor {
    /// F0: a torus sector on the plane, refused in the reduction, ahead
    /// of the join.
    OnCarrier,
    /// F2: the germ-pair dispatch, past a frame that served every germ
    /// and a matcher that consumed every one.
    Dispatch,
    /// Past the join: the result's volume, which reads no torus face
    /// holding a ring of ovals (`germ_plane_torus_spiric`'s stop, filed
    /// on its owner).
    Stopped,
    /// Past every door: the op returned a result.
    Built,
    /// Past the chord, the result shell's role escalates on its volume
    /// (`positive_volume_exact`): a body too thin for the run's band to
    /// read.
    Escalated,
}

/// The torus refusal's payload.
fn names_the_torus(e: &BooleanError) -> bool {
    matches!(
        e,
        BooleanError::CurvedBooleanUnsupported {
            kind: geom::SurfaceKind::Torus,
            ..
        }
    )
}

/// **The door `op` on `x`, `y` refuses at**, or [`TorusDoor::Built`]
/// where it returns: [`TorusDoor::Stopped`] past the join; or the op
/// names the torus, and the matcher, which
/// runs the reduction and every germ's frame but not the dispatch,
/// either refuses with that payload (the reduction raised it, F0) or
/// consumes every germ (the dispatch raised it, F2). Anything else
/// comes back described.
pub(crate) fn torus_door(
    op: BooleanOp,
    x: &AtRestBody<f64>,
    y: &AtRestBody<f64>,
) -> Result<TorusDoor, String> {
    match run(op, x, y) {
        Ok(_) => return Ok(TorusDoor::Built),
        Err(BooleanError::Escalated {
            decision: topo::BooleanDecision::ShellRole { .. },
            diag,
        }) if diag.predicate == Some("positive_volume_exact") => return Ok(TorusDoor::Escalated),
        Err(e) if names_the_torus(&e) => {}
        Err(e) if crate::germ_plane_torus_spiric::stops_past_the_join(&e) => {
            return Ok(TorusDoor::Stopped);
        }
        other => {
            return Err(format!(
                "the op: {:?}",
                other.map(|r| r.body().map(|b| b.kind))
            ));
        }
    }
    match topo::test_support::boolean_segment_sites(op, x, y, Tol::witness()) {
        Err(e) if names_the_torus(&e) => Ok(TorusDoor::OnCarrier),
        Ok(Some((records, segments))) if segments.len() == records => Ok(TorusDoor::Dispatch),
        other => Err(format!(
            "the matcher: {:?}",
            other.map(|s| s.map(|(records, segments)| (records, segments.len())))
        )),
    }
}

/// [`torus_door`] for the six runs of `t` against `p`, each labelled.
fn doors(
    t: &AtRestBody<f64>,
    p: &AtRestBody<f64>,
) -> Vec<(&'static str, Result<TorusDoor, String>)> {
    SIX.iter()
        .map(|&(label, op, swap)| {
            let (x, y) = if swap { (p, t) } else { (t, p) };
            (label, torus_door(op, x, y))
        })
        .collect()
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
        let band = geom_core::Band::linear(Tol::witness()).unwrap();
        let pierce_in_band = band.zero() < 5.2e-6 && 5.2e-6 < band.escalate();
        for (op, r) in six(&d, &p) {
            // B3's bar is 0.02 wide, and its pierce sector's curvature
            // margin (5.2e-6 m) lies in a coarse run's band: there the
            // pierce escalates ahead of the frame, which is not reached.
            if pose == "B3" && pierce_in_band {
                assert!(
                    matches!(
                        &r,
                        Err(BooleanError::Escalated {
                            decision: topo::BooleanDecision::PierceCurvature,
                            ..
                        })
                    ),
                    "{pose}, {op}: the run's band escalates the pierce first: {:?}",
                    r.err()
                );
                test_utils::vacuity::stood_down(
                    &format!("{pose}, {op}"),
                    "the frame's refusal: the pierce escalates ahead of it in this run's band",
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

/// **The lily's stem glue passes the chord, undeclared, in both member
/// orders, and builds**: its germ pair is the stem's weld cap against
/// the arch's tube wall, whose section is the arch's meridian circle on
/// both sides. Past the join, the section certificate reads the stem's
/// tube wall against the arch's (two tori whose axes are parallel and
/// apart) as two loops, each outside one face's window, and the union is
/// one shell. No declaration is read.
#[test]
fn the_lilys_stem_glue_passes_the_chord_and_builds() {
    let Some(s) = crate::mate7a_r1_probes::stem() else {
        return;
    };
    let a = crate::mate7a_r1_probes::arch();
    for (label, x, y) in [("stem ∪ arch", &s, &a), ("arch ∪ stem", &a, &s)] {
        let r =
            topo::union(x, y, Tol::witness()).unwrap_or_else(|e| panic!("{label} builds: {e:?}"));
        let shells = r.body().map(|b| b.body.shells().count());
        assert_eq!(shells, Some(1), "{label}: one stem");
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
        for (op, door) in doors(&d, &p) {
            assert_eq!(door, Ok(TorusDoor::OnCarrier), "{pose}, {op}");
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

/// **The fat corner bar's eight corners pair corner to corner on one
/// oval** (the spec's §1.4): B5 and B5′ against the fat torus `(1, 0.6)`,
/// every op the matcher reads, both member orders. Each end square cuts
/// two ovals, one each side of the axis's trace, each running from
/// corner to corner, so each segment joins `(±w, w, ±z)` to
/// `(±w, −w, ±z)`: one side `x = ±w`, one end `z = ±z`. Eight records,
/// eight segments; the pairing is decided before any chord exists.
#[test]
fn the_fat_corner_bars_corners_pair_on_one_oval() {
    let fat = torus(1.0, 0.6);
    for (pose, w) in [("B5", 0.1), ("B5′", 0.3)] {
        let b = corner_bar(1.0, 0.6, w);
        for (op, swap) in ORDERS {
            let what = format!("{pose}, {op:?}{}", if swap { " swapped" } else { "" });
            let (x, y) = if swap { (&b, &fat) } else { (&fat, &b) };
            let segments = segments(op, x, y, &what);
            assert_eq!(segments.len(), 8, "{what}: eight corners, eight segments");
            for [s, e] in segments {
                let corner =
                    |q: Point3<f64>| (q.x.abs() - w).abs() < 1e-9 && (q.y.abs() - w).abs() < 1e-9;
                assert!(
                    corner(s)
                        && corner(e)
                        && s.x.signum() == e.x.signum()
                        && s.z.signum() == e.z.signum()
                        && s.y.signum() != e.y.signum(),
                    "{what}: a segment {s:?} → {e:?} is not one oval's, corner to corner"
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

/// The poses that cut the one-face torus in two loops of one face pair,
/// each crossing the face's wrap edges once: the meridian half-spaces
/// M1 and M2 and a box whose face `x = 0` holds both meridian circles;
/// the planes square to the axis at `y = 0.2` and `y = −0.3`, and a box
/// whose face `y = 0.2` holds both circles; the axis-parallel plane S1
/// and a box whose face `x = 0.3` holds both ovals.
pub(crate) fn two_loop_poses() -> Vec<(&'static str, AtRestBody<f64>)> {
    let m2 = Vec3::new(degrees(30.0).cos(), 0.0, degrees(30.0).sin());
    vec![
        ("M1", half_space(Point3::origin(), Vec3::unit_x())),
        ("M2", half_space(Point3::origin(), m2)),
        ("meridian box", bar((0.0, 1.0), (-1.0, 1.0), (-3.0, 3.0))),
        ("P1", above(0.2)),
        ("y ≥ −0.3", above(-0.3)),
        ("concentric box", bar((-3.0, 3.0), (0.2, 1.0), (-3.0, 3.0))),
        ("S1", half_space(Point3::new(0.3, 0.0, 0.0), Vec3::unit_x())),
        ("spiric box", bar((0.3, 1.0), (-1.0, 1.0), (-3.0, 3.0))),
    ]
}

/// **Two one-site loops on one face pair pass the dispatch to the
/// chord**, as the donut's two-site loops do, in every op: built where
/// the section is two meridian or concentric circles
/// (`germ_plane_torus_chord` holds those bodies to the truth), built or
/// stopped past the join where it is two spiric ovals
/// (`germ_plane_torus_spiric` holds those). On the one-face torus (its
/// vertex on the outer equator, off it, and on the inner equator) each
/// loop of a meridian, concentric or spiric section crosses the face's
/// wrap edges once, so the section's two loops are two one-site records
/// of one face pair. On the torus seamed on `y = 0`, the concentric
/// circles of a plane `y = h`, `|h| < r`, both lie on one face and each
/// crosses its wrap edge once; its meridian and spiric sections cross
/// the equators twice per loop, as the donut's do. Each loop is its own
/// closed curve: a check that read one face pair's records as one loop
/// takes the other loop's record for a second record of its own.
#[test]
fn two_one_site_loops_on_one_face_pair_pass_the_dispatch() {
    let mut runs = Vec::new();
    for phase in [0.0, 1.0, PI] {
        let t = one_face_torus(2.0, 0.5, phase);
        for (pose, p) in two_loop_poses() {
            runs.push((
                format!("one-face torus, vertex at {phase}, {pose}"),
                doors(&t, &p),
            ));
        }
    }
    let seamed = seamed_torus(2.0, 0.5);
    for (pose, p) in [
        ("P1", above(0.2)),
        ("y ≥ −0.3", above(-0.3)),
        ("concentric box", bar((-3.0, 3.0), (0.2, 1.0), (-3.0, 3.0))),
    ] {
        runs.push((format!("seamed torus, {pose}"), doors(&seamed, &p)));
    }
    for (what, doors) in runs {
        let spiric = ["meridian box", "S1", "spiric box"]
            .iter()
            .any(|pose| what.ends_with(pose));
        for (op, door) in doors {
            assert!(
                door == Ok(TorusDoor::Built) || spiric && door == Ok(TorusDoor::Stopped),
                "{what}, {op}: {door:?}"
            );
        }
    }
}

/// **A plane square to the axis just under the tube's top reaches the
/// door the donut does**, at `y = 0.5 − δ`: past the frame, the
/// dispatch and the chord where the run's band reads the two circles
/// apart, to a body or, where the cap is too thin for the run's band to
/// read its shell, an escalation; at the on-carrier sector where the
/// band reads them one tangent circle. The
/// one-face torus and the torus seamed on `y = 0` carry both circles on
/// one face pair, each crossing a wrap edge once; the donut carries
/// them on its outer and inner faces. Every body built is held to its
/// closed-form volume and to tier 3.
#[test]
fn a_band_pose_under_the_tubes_top_reaches_the_donuts_door() {
    let (d, one, seamed) = (
        donut(),
        one_face_torus(2.0, 0.5, 0.0),
        seamed_torus(2.0, 0.5),
    );
    for delta in [5e-7, 1e-6, 1e-5, 1e-3] {
        let p = bar((-3.0, 3.0), (0.5 - delta, 1.0), (-3.0, 3.0));
        let want = doors(&d, &p);
        for (name, t) in [
            ("the donut", &d),
            ("the one-face torus", &one),
            ("the seamed torus", &seamed),
        ] {
            for ((op, door), (_, want)) in doors(t, &p).into_iter().zip(&want) {
                assert!(want.is_ok(), "δ = {delta}, the donut, {op}: {want:?}");
                assert_eq!(&door, want, "δ = {delta}, {name}, {op}");
            }
            for ((op, got), &(_, kind, swap)) in six(t, &p).into_iter().zip(&SIX) {
                let Ok(got) = got else { continue };
                let (big_r, r, h) = (2.0, 0.5, 0.5 - delta);
                let (torus_v, bar_v) = (2.0 * PI * PI * big_r * r * r, 36.0 * (0.5 + delta));
                let meet = cap(big_r, r, h);
                let (first, second) = if swap {
                    (bar_v, torus_v)
                } else {
                    (torus_v, bar_v)
                };
                let truth = match kind {
                    BooleanOp::Union => first + second - meet,
                    BooleanOp::Intersect => meet,
                    BooleanOp::Subtract => first - meet,
                };
                let b = &got
                    .body()
                    .unwrap_or_else(|| panic!("δ = {delta}, {name}, {op}: empty"))
                    .body;
                assert_eq!(
                    topo::validate_geometric(b, Tol::witness()),
                    Ok(()),
                    "δ = {delta}, {name}, {op}: tier 3"
                );
                let v = topo::test_support::pinned_volume("the measured body", b, Tol::witness())
                    .volume;
                assert!(
                    (v - truth).abs() <= volume_tol(truth),
                    "δ = {delta}, {name}, {op}: volume {v} against the truth {truth}"
                );
            }
        }
    }
}

/// **A bar across the hole whose sections' ends tie for nearest within a
/// run's band escalates the ranking, by name.** The bar
/// `[x − w, x + w] × [−w, w] × [−2, 2]`, `x = 0.0053`, `w = 0.277`,
/// passes the frame on every face, and where the matcher ranks a germ's
/// candidates, two stand `3.2e-6 m` apart in the ranking's measure. Where
/// that lies in the run's band (`ε = 1e-6`) every op escalates
/// `bool_join_nearest` as a join coincidence; elsewhere every op passes
/// the join, to a body or a stop past it (`germ_plane_torus_spiric`
/// holds them to the truth). The ranking is the one every curved pair's
/// germs take; this pose reaches it through the torus frame.
#[test]
fn a_nearest_tie_in_band_on_a_bar_across_the_hole_escalates_by_name() {
    let d = donut();
    let (x, w) = (0.0053, 0.277);
    let p = bar((x - w, x + w), (-w, w), (-2.0, 2.0));
    let band = geom_core::Band::linear(Tol::witness()).unwrap();
    let tie = 3.2e-6;
    let in_band = band.zero() < tie && tie < band.escalate();
    for &(label, op, swap) in &SIX {
        let (a, b) = if swap { (&p, &d) } else { (&d, &p) };
        if in_band {
            let got = run(op, a, b);
            assert!(
                matches!(
                    &got,
                    Err(BooleanError::Escalated {
                        decision: topo::BooleanDecision::Coincidence(topo::Coincide::Join, _),
                        diag,
                    }) if diag.predicate == Some("bool_join_nearest")
                ),
                "{label}: the tie escalates the ranking: {:?}",
                got.err()
            );
        } else {
            let door = torus_door(op, a, b);
            assert!(
                matches!(door, Ok(TorusDoor::Built | TorusDoor::Stopped)),
                "{label}: {door:?}"
            );
        }
    }
}
