//! **Whether a pair may meet is a fact about the pair, not its pose.**
//!
//! Each fixture below is two operands with an unarmed face (a cone
//! wall, which no op has an arm for; an `Approx` cap, which none
//! admits) clear of a planar operand, turned TOGETHER through a set of
//! rigid poses and scales. Every pose gets the same verdict: every op,
//! in both orders, builds, and weighs what the disjoint operands'
//! closed forms say — the cone frustum's `π∫(r_out² − r_in²) dy`, the
//! boxes' products — never the kernel's own reading of either operand.
//! The world box of a turned face widens by how it is turned, so a
//! verdict read off world-axis box overlap alone flips with the pose.
//!
//! The twin that must NOT clear: the same cone with the bar through
//! its wall refuses at every pose, under every op, at the up-front
//! gate. A pair cleared that touches would be a wrong body.
//!
//! The ball rows hold the extent scan's section circle to its own
//! extent: a brick beside a ball whose face carriers cut the sphere in
//! circles that lie off the faces.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use geom_core::{Affine3, Point2, Point3, Tol, Vec2, Vec3};
use profile::{Profile, SketchPlane, test_support::bulge_loop};
use sweep::test_support::brick;
use sweep::{Revolution, RevolveAxis, revolve};
use topo::{AtRestBody, Body, BooleanError, BooleanResult, PairRefusalSite};

use crate::common::approx::box_with_approx_cap;

/// The rigid poses every pair is turned through, together: the
/// identity and three turns about each world axis and about a skew
/// one, all through the origin.
fn poses() -> Vec<(String, Affine3<f64>)> {
    let axes = [
        ("x", Vec3::new(1.0, 0.0, 0.0)),
        ("y", Vec3::new(0.0, 1.0, 0.0)),
        ("z", Vec3::new(0.0, 0.0, 1.0)),
        ("(1,2,3)", Vec3::new(1.0, 2.0, 3.0).normalize()),
    ];
    let mut out = vec![("identity".to_owned(), Affine3::identity())];
    for (name, axis) in axes {
        for angle in [0.3, PI / 4.0, 1.2] {
            out.push((
                format!("{angle:.4} about {name}"),
                Affine3::rotation_about_axis(Point3::new(0.0, 0.0, 0.0), axis, angle),
            ));
        }
    }
    out
}

/// The scales the cone pair is built at.
const SCALES: [f64; 3] = [0.1, 1.0, 10.0];

/// An annular frustum about the y axis, scaled by `s`: inner wall a
/// cylinder `r = 0.2`, outer wall the cone from `r = 0.6` at `y = 0`
/// to `r = 0.4` at `y = 0.6`.
fn frustum(s: f64, tol: Tol) -> Body<f64> {
    let lp = bulge_loop(
        [(0.2, 0.0), (0.6, 0.0), (0.4, 0.6), (0.2, 0.6)]
            .into_iter()
            .map(|(r, y)| (Point2::new(r * s, y * s), 0.0))
            .collect(),
    );
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(tol)
        .unwrap();
    revolve(
        &profile,
        RevolveAxis {
            origin: Point2::new(0.0, 0.0),
            dir: Vec2::new(0.0, 1.0),
        },
        Revolution::Full,
        tol,
    )
    .unwrap()
    .body
}

/// [`frustum`]'s volume: `π∫₀^{0.6}((0.6 − y/3)² − 0.2²) dy · s³`
/// `= π·((0.6³ − 0.4³) − 0.024)·s³ = 0.128π·s³`.
fn frustum_volume(s: f64) -> f64 {
    0.128 * PI * s.powi(3)
}

/// A ball of radius `r` about the origin, poled on y.
fn ball(r: f64) -> Body<f64> {
    sweep::test_support::revolved_about_y(
        vec![(Point2::new(0.0, -r), 1.0), (Point2::new(0.0, r), 0.0)],
        Revolution::Full,
        Tol::witness(),
    )
}

fn posed(what: &str, body: &Body<f64>, map: &Affine3<f64>, tol: Tol) -> AtRestBody<f64> {
    let b = topo::transform_rigid(body, map, tol).unwrap_or_else(|e| panic!("{what}: {e:?}"));
    topo::test_support::finished(what, b, tol)
}

/// `a` and `b`, disjoint, under ∪, ∩ and both ∖: each body held to
/// every validation tier, to being a legal operand of the next boolean,
/// and to the closed form, `∩` empty.
fn assert_apart_every_op(
    label: &str,
    (a, v_a): (&AtRestBody<f64>, f64),
    (b, v_b): (&AtRestBody<f64>, f64),
) {
    let tol = Tol::witness();
    let run = |op: &str, out: Result<BooleanResult<f64>, BooleanError>, truth: f64| {
        let label = format!("{label}, {op}");
        let out = out.unwrap_or_else(|e| panic!("{label}: refused {e:?}"));
        let BooleanResult::Body(out) = out else {
            panic!("{label}: came back empty");
        };
        assert_eq!(topo::validate(&out.body), Ok(()), "{label}: validate");
        assert_eq!(topo::validate_closed(&out.body), Ok(()), "{label}: closed");
        assert_eq!(
            topo::validate_geometric(&out.body, tol),
            Ok(()),
            "{label}: tier 3"
        );
        sweep::test_support::assert_legal_operand(&label, &out.body, tol);
        let m = topo::mass_properties(&out.body, tol)
            .unwrap_or_else(|e| panic!("{label}: mass properties {e:?}"));
        assert!(
            (m.volume - truth).abs() <= m.volume_pad + 1e-9 * truth,
            "{label}: volume {} ± {} against the closed form {truth}",
            m.volume,
            m.volume_pad
        );
    };
    run("∪", topo::union(a, b, tol), v_a + v_b);
    run("a ∖ b", topo::subtract(a, b, tol), v_a);
    run("b ∖ a", topo::subtract(b, a, tol), v_b);
    for (op, out) in [
        ("a ∩ b", topo::intersect(a, b, tol)),
        ("b ∩ a", topo::intersect(b, a, tol)),
    ] {
        assert!(
            matches!(out, Ok(BooleanResult::Empty)),
            "{label}, {op}: disjoint operands meet in nothing, got {:?}",
            out.map(|r| r.body().is_some())
        );
    }
}

#[test]
fn a_cone_wall_clear_of_a_bar_builds_at_every_pose_and_scale() {
    let tol = Tol::witness();
    for s in SCALES {
        let cone = frustum(s, tol);
        let bar = brick::<f64>(
            (0.7 * s, 1.2 * s),
            (0.1 * s, 0.5 * s),
            (-0.2 * s, 0.2 * s),
            tol,
        );
        for (pose, map) in poses() {
            let label = format!("ε {:e}, scale {s}, {pose}", tol.eps());
            assert_apart_every_op(
                &label,
                (&posed("the frustum", &cone, &map, tol), frustum_volume(s)),
                (&posed("the bar", &bar, &map, tol), 0.08 * s.powi(3)),
            );
        }
    }
}

/// The `Approx` cap clears every face of the bar at every pose, so each
/// op reaches the classification, and refuses there by the kind's own
/// door ([`topo::PointInSolidError::KindUnsupported`]): `Approx` stays off
/// the boolean roster by decision. The verdict is the pose's no more
/// than the cone's is. Five of [`poses`]: re-certifying the fit per pose
/// is what this row spends.
#[test]
fn an_approx_cap_clear_of_a_bar_reads_alike_at_every_pose() {
    let tol = Tol::witness();
    let (capped, _) = box_with_approx_cap(0.05, 1e-9);
    let bar = brick::<f64>((2.3, 2.8), (0.5, 1.5), (0.2, 0.8), tol);
    for (pose, map) in poses().into_iter().step_by(3) {
        let (a, b) = (
            posed("the capped box", &capped, &map, tol),
            posed("the bar", &bar, &map, tol),
        );
        for (op, out) in [
            ("∪", topo::union(&a, &b, tol)),
            ("∩", topo::intersect(&a, &b, tol)),
            ("a ∖ b", topo::subtract(&a, &b, tol)),
            ("b ∖ a", topo::subtract(&b, &a, tol)),
        ] {
            assert!(
                matches!(
                    out,
                    Err(BooleanError::Containment(
                        topo::PointInSolidError::KindUnsupported {
                            kind: geom::SurfaceKind::Approx,
                            ..
                        }
                    ))
                ),
                "ε {:e}, {pose}, {op}: the cap clears the bar, so the op reaches the \
                 classification's Approx door: {:?}",
                tol.eps(),
                out.map(|r| r.body().is_some())
            );
        }
    }
}

#[test]
fn a_bar_through_the_cone_wall_refuses_at_every_pose() {
    let tol = Tol::witness();
    for s in SCALES {
        let cone = frustum(s, tol);
        let bar = brick::<f64>((-s, s), (0.25 * s, 0.35 * s), (-0.05 * s, 0.05 * s), tol);
        for (pose, map) in poses() {
            let label = format!("ε {:e}, scale {s}, {pose}", tol.eps());
            let (a, b) = (
                posed("the frustum", &cone, &map, tol),
                posed("the bar", &bar, &map, tol),
            );
            for (op, out, site) in [
                ("∪", topo::union(&a, &b, tol), PairRefusalSite::OperandGate),
                (
                    "∩",
                    topo::intersect(&a, &b, tol),
                    PairRefusalSite::RevertRoster,
                ),
                (
                    "a ∖ b",
                    topo::subtract(&a, &b, tol),
                    PairRefusalSite::RevertRoster,
                ),
                (
                    "b ∖ a",
                    topo::subtract(&b, &a, tol),
                    PairRefusalSite::RevertRoster,
                ),
            ] {
                assert!(
                    matches!(
                        out,
                        Err(BooleanError::CurvedPairUnsupported {
                            site: got,
                            kind: geom::SurfaceKind::Cone,
                            ..
                        }) if got == site
                    ),
                    "{label}, {op}: the bar meets the cone wall, so the pair refuses up front \
                     at {site:?}: {:?}",
                    out.map(|r| r.body().is_some())
                );
            }
        }
    }
}

#[test]
fn a_brick_beside_a_ball_builds_at_every_pose() {
    let tol = Tol::witness();
    let sphere = ball(1.0);
    let block = brick::<f64>((1.1, 2.0), (-0.3, 0.3), (0.5, 1.5), tol);
    for (pose, map) in poses() {
        assert_apart_every_op(
            &format!("ε {:e}, {pose}", tol.eps()),
            (&posed("the ball", &sphere, &map, tol), 4.0 / 3.0 * PI),
            (&posed("the brick", &block, &map, tol), 0.9 * 0.6),
        );
    }
}
