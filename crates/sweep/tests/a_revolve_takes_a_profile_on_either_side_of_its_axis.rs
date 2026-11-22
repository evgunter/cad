//! **A revolve takes a profile on either side of its axis.** The axis
//! line and the signed angle fix the turn: a profile to the left of the
//! directed axis is the same revolve as its image across the axis line
//! turned half a turn, so it builds where that image builds, and lands
//! where the authored turn carries it.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::revolve_common;

use core::f64::consts::{FRAC_PI_2, PI, TAU};
use geom_core::{Arc2, Band, Point2, Point3, Tol, Vec2};
use profile::{ProfileLoop, RawLoop, test_support::bulge_loop};
use revolve_common::*;
use sweep::{Revolution, RevolveAxis, RevolveError, revolve};

/// The full circle of radius `r` about `(cx, cy)`, from carrier angle
/// `phase`.
fn circle(cx: f64, cy: f64, r: f64, phase: f64) -> ProfileLoop<f64> {
    RawLoop::new([(
        Point2::new(cx + r * phase.cos(), cy + r * phase.sin()),
        profile::Segment::Arc(Arc2 {
            centre: Point2::new(cx, cy),
            radius: r,
            sweep: TAU,
        }),
    )])
}

/// The axis through `(x, 0)` heading `±y`.
fn axis(x: f64, up: bool) -> RevolveAxis<f64> {
    RevolveAxis {
        origin: Point2::new(x, 0.0),
        dir: Vec2::new(0.0, if up { 1.0 } else { -1.0 }),
    }
}

/// `(x, y)` of the sketch plane (world XY) turned by `phi` about the
/// axis through `(ax, 0)` heading `±y`, right-handed.
fn turned(x: f64, y: f64, ax: f64, up: bool, phi: f64) -> Point3<f64> {
    let h = if up { 1.0 } else { -1.0 };
    let w = x - ax;
    Point3::new(ax + w * phi.cos(), y, -h * w * phi.sin())
}

fn containment(body: &topo::Body<f64>, q: Point3<f64>) -> topo::SolidContainment {
    let tol = Tol::witness();
    topo::point_in_solid(body, q, Band::linear(tol).unwrap(), tol).unwrap()
}

fn volume(body: &topo::AtRestBody<f64>, row: &str) -> topo::MassProperties<f64> {
    let tol = Tol::witness();
    topo::mass_properties(body, tol, topo::test_support::pinned_resolution(tol))
        .unwrap_or_else(|e| panic!("{row}: {e:?}"))
        .enclosure
}

/// **The pipe's end loops revolved into an elbow**, the PROFILE probe's
/// geometry on a free profile: the ring `0.6 ≤ ρ ≤ 1` about the origin
/// turned a quarter about an axis 2 clear of it, through `(±3, 0)`, for
/// both headings and both signs of the angle. Every row builds, with
/// the ring's material a quarter-turn's half along the authored turn
/// and none the other way, and the bore empty.
#[test]
fn a_ring_beside_its_axis_revolves_for_either_heading() {
    for ax in [3.0, -3.0] {
        for up in [true, false] {
            for theta in [FRAC_PI_2, -FRAC_PI_2] {
                let row = format!(
                    "axis through ({ax}, 0) heading {}y, angle {theta}",
                    if up { "+" } else { "-" }
                );
                let vp = validated(vec![circle(0.0, 0.0, 1.0, 0.0), circle(0.0, 0.0, 0.6, 0.0)]);
                let built = revolve(
                    &vp,
                    axis(ax, up),
                    Revolution::Partial(theta),
                    Tol::witness(),
                )
                .unwrap_or_else(|e| panic!("{row}: {e}"));
                let body = &built.body;
                assert_all_tiers(body);
                for (x, phi, want, what) in [
                    (
                        0.8,
                        theta / 2.0,
                        topo::SolidContainment::In,
                        "the ring along the turn",
                    ),
                    (
                        0.8,
                        -theta / 2.0,
                        topo::SolidContainment::Out,
                        "the ring against the turn",
                    ),
                    (
                        -0.8,
                        theta / 2.0,
                        topo::SolidContainment::In,
                        "the ring's far side along the turn",
                    ),
                    (
                        0.0,
                        theta / 2.0,
                        topo::SolidContainment::Out,
                        "the bore along the turn",
                    ),
                ] {
                    assert_eq!(
                        containment(body, turned(x, 0.0, ax, up, phi)),
                        want,
                        "{row}: {what}"
                    );
                }
            }
        }
    }
}

/// Each profile beside the y axis, mirrored across it: the mirror turns
/// by the same signed angle about the same directed axis, so it is the
/// first body turned half a turn — it builds exactly where the first
/// does, to the same volume.
///
/// - A washer, clear of the axis.
/// - A half-disc on the axis (its vertices on the axis, its side the
///   arc's alone).
/// - A circle touching the axis at its one vertex, which pinches there
///   however far it turns and refuses on both sides.
/// - An apple's long arc between its two apexes, closed along the axis.
#[test]
fn a_profile_and_its_mirror_across_the_axis_revolve_alike() {
    let rows: [(&str, fn(f64) -> ProfileLoop<f64>); 4] = [
        ("a washer", |s| {
            ProfileLoop::polygon([
                Point2::new(s, 0.0),
                Point2::new(2.0 * s, 0.0),
                Point2::new(2.0 * s, 1.0),
                Point2::new(s, 1.0),
            ])
        }),
        ("a half-disc on the axis", |s| {
            bulge_loop(vec![
                (Point2::new(0.0, -s), 1.0),
                (Point2::new(0.0, s), 0.0),
            ])
        }),
        ("a circle touching the axis", |s| {
            RawLoop::new([(
                Point2::new(0.0, 0.0),
                profile::Segment::Arc(Arc2 {
                    centre: Point2::new(s, 0.0),
                    radius: 1.0,
                    sweep: TAU,
                }),
            )])
        }),
        ("an apple's long arc between its apexes", |s| {
            let h = 0.75_f64.sqrt();
            bulge_loop(vec![
                (Point2::new(0.0, -s * h), (PI / 3.0).tan()),
                (Point2::new(0.0, s * h), 0.0),
            ])
        }),
    ];
    for (what, lp) in rows {
        for revolution in [
            Revolution::Partial(1.0),
            Revolution::Partial(-1.0),
            Revolution::Full,
        ] {
            let right = revolve(
                &validated(vec![lp(1.0)]),
                axis(0.0, true),
                revolution,
                Tol::witness(),
            );
            let left = revolve(
                &validated(vec![lp(-1.0)]),
                axis(0.0, true),
                revolution,
                Tol::witness(),
            );
            let row = format!("{what}, {revolution:?}");
            match (&right, &left) {
                (Ok(r), Ok(l)) => {
                    assert_all_tiers(&l.body);
                    let (vr, vl) = (volume(&r.body, &row), volume(&l.body, &row));
                    assert!(
                        vl.volume > 0.0,
                        "{row}: the left body is inside out ({vl:?})"
                    );
                    assert!(
                        (vr.volume - vl.volume).abs()
                            <= 1e-9 * vr.volume + vr.volume_pad + vl.volume_pad,
                        "{row}: volumes {vr:?} right, {vl:?} left"
                    );
                }
                (Err(r), Err(l)) => assert_eq!(
                    core::mem::discriminant::<RevolveError>(r),
                    core::mem::discriminant::<RevolveError>(l),
                    "{row}: {r} right, {l} left"
                ),
                _ => panic!(
                    "{row}: right {:?}, left {:?}",
                    right.as_ref().map(|_| "built"),
                    left.as_ref().map(|_| "built")
                ),
            }
        }
    }
}

/// **What still refuses crosses the axis**: a profile reaching both
/// sides of it refuses whichever way the axis heads, and the refusal
/// says it crosses.
#[test]
fn a_profile_across_its_axis_refuses_for_either_heading() {
    let lp = ProfileLoop::polygon([
        Point2::new(-1.0, 0.0),
        Point2::new(2.0, 0.0),
        Point2::new(2.0, 1.0),
        Point2::new(-1.0, 1.0),
    ]);
    for up in [true, false] {
        let e = revolve(
            &validated(vec![lp.clone()]),
            axis(0.0, up),
            Revolution::Partial(1.0),
            Tol::witness(),
        )
        .unwrap_err();
        assert!(
            matches!(e, RevolveError::VertexCrossesAxis { loop_index: 0, .. }),
            "heading {up}: {e:?}"
        );
        assert!(
            e.to_string().contains("crosses the axis"),
            "heading {up}: {e}"
        );
    }
}
