//! Which door a crossing sphere pair reaches, per offset direction
//! relative to the operand's own chart.
//!
//! `ball_at` revolves an XY-plane profile about world Y, so the polar
//! axis is **Y** and the seam great circle is the circle of radius `r`
//! in the plane `z = c_z` (azimuth 0 = +X and its π copy). That single
//! fact splits the configuration space in two, and the split is
//! geometric, not tolerance-shaped:
//!
//! * **Offset along Z.** Every point of one ball's seam circle sits at
//!   the *same* distance `√(r² + Δz²)` from the other centre, so the
//!   seam is wholly inside or wholly outside the other sphere and can
//!   never cross it — at any depth and at any radius ratio. No crossing
//!   is found, the containment fallback runs, and its curved-extent
//!   scan refuses `SpheresMeet`.
//! * **Offset along X or Y.** The seam circle now meets the other
//!   sphere, so a seam edge crosses a CURVED face and the circle ×
//!   sphere roots pierce it. The pair reaches the join, which hands each
//!   side the pair's radical plane. Offset along Y (the polar axis) the
//!   section is polar for both operands and the azimuth-window rule
//!   selects its arcs; offset along X it is tilted against both charts
//!   and the run-side rule does. Either way the union builds, because
//!   both balls are revolved from the same seam — each seam meridian
//!   pierces the other sphere ON the other's seam. Spin either ball
//!   about Y and the pierce lands inside a half-band: the pierce-ring
//!   door (`snowman.rs`).
//!
//! Nested balls answer, and must keep answering.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;
use geom_core::{Affine3, Point2, Tol, Vec2, Vec3};
use profile::{Profile, SketchPlane, test_support::bulge_loop};
use sweep::test_support::finished;
use sweep::{Revolution, RevolveAxis, revolve};
use topo::{AtRestBody, BooleanError};

/// A radius-`r` ball at `centre`, poles on world Y (the pip corpus's
/// constructor chart).
fn ball_at(r: f64, centre: Vec3<f64>) -> AtRestBody<f64> {
    ball_at_tol(r, centre, Tol::witness())
}

/// [`ball_at`], built at `tol`.
fn ball_at_tol(r: f64, centre: Vec3<f64>, tol: Tol) -> AtRestBody<f64> {
    let lp = bulge_loop(vec![
        (Point2::new(0.0, -r), 1.0),
        (Point2::new(0.0, r), 0.0),
    ]);
    let vp = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(tol)
        .unwrap();
    let axis = RevolveAxis {
        origin: Point2::new(0.0, 0.0),
        dir: Vec2::new(0.0, 1.0),
    };
    let ball = revolve(&vp, axis, Revolution::Full, tol).unwrap().body;
    let ball = topo::transform_rigid(&ball, &Affine3::translation(centre), tol).unwrap();
    finished("the ball", ball, tol)
}

fn union_err(a: &AtRestBody<f64>, b: &AtRestBody<f64>) -> BooleanError {
    topo::union(a, b, Tol::witness()).expect_err("the pair is refused")
}

/// **The nesting question offers a tolerance only where a smaller one
/// passes, and its decided arms tell the same story** (the second
/// review's probe P5, adopted). A half ball inside a unit ball, its
/// centre offset along Z (clear of both seams): the nesting clearance
/// passes only positive, so
///
/// - an in-band depth `δ` short of internal tangency offers the
///   tolerance its margin gives, and `δ` past it offers none (a smaller
///   tolerance decides it negative, and the pair meets);
/// - a zero-band depth short of tangency is a decided zero, refused as
///   the pair meeting with the tolerance its margin gives, and a clear
///   depth past it is the boundaries crossing, refused with the same
///   lever alone.
#[test]
fn a_nested_ball_near_tangency_offers_a_tolerance_only_short_of_it() {
    let tol = Tol::witness();
    let band = geom_core::Band::linear(tol).unwrap();
    let k = band.escalate() / band.zero();
    const LEVER: &str = "Recourse: move the spheres so they clearly stand apart, or so one lies \
                         clearly inside the other";
    let offer = |m: f64| {
        format!(
            "{LEVER}, or, if this clearance is intended, tighten the tolerance below {:e} m",
            m / k
        )
    };
    let union = |d: f64| {
        topo::union(
            &ball_at(1.0, Vec3::new(2.0, 2.0, 0.5)),
            &ball_at(0.5, Vec3::new(2.0, 2.0, 1.0 + d)),
            tol,
        )
        .expect_err("a pair at tangency refuses")
    };
    let delta = (band.zero() + band.escalate()) / 2.0;
    for (d, offered) in [(-delta, true), (delta, false)] {
        let err = union(d);
        let BooleanError::Escalated {
            decision: topo::BooleanDecision::Sphere(topo::SphereQuestion::Nested),
            diag,
        } = &err
        else {
            panic!("{d:e}: the nesting question escalates: {err:?}");
        };
        let m = diag.margin.diagnostic_f64_for_error_text().value().unwrap();
        let text = err.to_string();
        assert_eq!(
            text.ends_with(&offer(m)),
            offered,
            "{d:e}: a tolerance only where a smaller one passes: {text}"
        );
        assert!(offered || text.ends_with(LEVER), "{d:e}: {text}");
    }
    let err = union(-0.5 * band.zero());
    let BooleanError::SpheresMeet {
        verdict: geom_brep::recourse::Refused::Zero(classified),
        ..
    } = &err
    else {
        panic!("a zero-band clearance is a decided zero: {err:?}");
    };
    let m = classified
        .margin
        .diagnostic_f64_for_error_text()
        .value()
        .unwrap();
    assert!(err.to_string().ends_with(&offer(m)), "{err}");
    let err = union(1e-3);
    assert!(
        matches!(
            err,
            BooleanError::SpheresMeet {
                verdict: geom_brep::recourse::Refused::Negative { .. },
                ..
            }
        ) && err.to_string().ends_with(LEVER),
        "crossing boundaries end in the same lever alone: {err}"
    );
}

/// A Z offset keeps both seams clear of the other sphere at every depth
/// and every radius ratio, so the scan — not the pierce — is the door.
#[test]
fn z_offset_pairs_refuse_at_the_curved_extent_scan() {
    for (r2, z2, label) in [
        (1.0, 1.9, "shallow, equal radii"),
        (1.0, 1.1, "deep, equal radii"),
        (0.2, 1.4, "unequal radii, seam wholly inside the big ball"),
    ] {
        let err = union_err(
            &ball_at(1.0, Vec3::new(2.0, 2.0, 0.5)),
            &ball_at(r2, Vec3::new(2.0, 2.0, z2)),
        );
        let BooleanError::SpheresMeet { verdict, .. } = err else {
            panic!("{label}: expected the scan's typed refusal, got {err:?}");
        };
        assert!(
            matches!(verdict, geom_brep::recourse::Refused::Negative { .. }),
            "{label}: the boundaries cross: {verdict:?}"
        );
    }
}

/// An in-seam-plane or polar-axis offset drives a seam meridian through
/// the other ball's sphere face, and the circle × sphere roots pierce
/// it. Both offsets build, the polar one through the azimuth window and
/// the in-seam-plane one through the run side, to one lens.
#[test]
fn seam_crossing_pairs_build() {
    let a = ball_at(1.0, Vec3::new(2.0, 2.0, 0.5));
    let joined = topo::union(&a, &ball_at(1.0, Vec3::new(3.4, 2.0, 0.5)), Tol::witness())
        .unwrap_or_else(|e| {
            panic!("offset along X, in the seam plane: the union builds, got {e:?}")
        });
    let tilted = topo::mass_properties(&joined.body().expect("a body").body, Tol::witness())
        .unwrap()
        .volume;
    let joined = topo::union(&a, &ball_at(1.0, Vec3::new(2.0, 3.4, 0.5)), Tol::witness())
        .unwrap_or_else(|e| panic!("offset along Y, the polar axis: the union builds, got {e:?}"));
    let joined = &joined.body().expect("a body").body;
    let v = topo::mass_properties(joined, Tol::witness())
        .unwrap()
        .volume;
    // Two unit balls 1.4 apart share a lens of two caps of height 0.3.
    let lens = 2.0 * PI * 0.3_f64.powi(2) * (3.0 - 0.3) / 3.0;
    let want = 2.0 * 4.0 * PI / 3.0 - lens;
    for (offset, v) in [("Y", v), ("X", tilted)] {
        assert!(
            (v - want).abs() < 1e-9 * want,
            "offset along {offset}: union volume {v}, want {want}"
        );
    }
}

/// Nested balls never reach either door; the outer ball is the answer.
#[test]
fn nested_balls_still_answer() {
    let joined = topo::union(
        &ball_at(1.0, Vec3::new(2.0, 2.0, 0.5)),
        &ball_at(0.4, Vec3::new(2.0, 2.0, 0.5)),
        Tol::witness(),
    )
    .expect("nested balls answer");
    let joined = &joined.body().expect("a body").body;
    let v = topo::mass_properties(joined, Tol::witness())
        .unwrap()
        .volume;
    assert!((v - 4.0 * PI / 3.0).abs() < 1e-9, "{v}");
}
