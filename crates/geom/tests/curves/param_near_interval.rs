//! The interval lane's rows for [`geom::Curve3::param_near`].
//!
//! A file of its own, named so the CI filter FORCES the interval
//! compile mode on a change to it (`scripts/ci-filter.py`'s
//! `_forces_interval` matches `interval` in the basename) rather than
//! sampling it. The shipped body has no interval-gated block to name —
//! the anchored form's whole claim is that there is no fork — so this
//! suite is the only interval-specific artifact the rule can match.

#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{PI, TAU};

use geom::Curve3;
use geom_core::{Bounds, Point3, Real, Vec3, interval::Interval};

fn lift_p(p: Point3<f64>) -> Point3<Interval> {
    Point3::new(
        Interval::from_f64(p.x),
        Interval::from_f64(p.y),
        Interval::from_f64(p.z),
    )
}

fn lift_v(v: Vec3<f64>) -> Vec3<Interval> {
    Vec3::new(
        Interval::from_f64(v.x),
        Interval::from_f64(v.y),
        Interval::from_f64(v.z),
    )
}

/// The `f64` carrier and its componentwise lift — the SAME circle at
/// two scalars, which is what makes the enclosure comparison
/// meaningful.
fn pair() -> (Curve3<f64>, Curve3<Interval>) {
    let center = Point3::new(1.0, 2.0, 3.0);
    (
        Curve3::Circle {
            center,
            axis: Vec3::unit_z(),
            radius: 1.5,
            u_ref: Vec3::unit_x(),
        },
        Curve3::Circle {
            center: lift_p(center),
            axis: lift_v(Vec3::unit_z()),
            radius: Interval::from_f64(1.5),
            u_ref: lift_v(Vec3::unit_x()),
        },
    )
}

/// **The lane runs the same body, and the row can go red if it stops
/// doing so.** Three things are asserted, and each is a different way
/// the guarantee could degrade:
///
/// 1. **Enclosure** — the interval answer contains the `f64` one. A
///    mis-selected branch would put the `f64` value outside a narrow
///    enclosure a whole turn away, which is the failure the anchored
///    form exists to prevent.
/// 2. **Strictly inside the span** — the enclosure as a whole, not
///    just its centre. This is what a consumer's interiority trilean
///    will ask, so a widening that costs the split is caught here
///    rather than three layers down as an escalation.
/// 3. **A width BUDGET** — the enclosure is at most a few ulps of the
///    span. Without this the row passes on an answer of `[t₀, t₁]`,
///    which encloses everything and decides nothing; the budget is
///    what makes "no fork, no degradation" falsifiable instead of
///    merely true-by-containment.
///
/// The spans deliberately include a NEARLY-FULL period walked at
/// points near both ends — the seam-straddling regime the derivation
/// claims widens rather than mis-selects.
#[test]
fn the_interval_lane_holds_at_the_cut() {
    let (carrier64, carrier) = pair();
    for (t0, t1) in [(0.0, PI), (0.0, TAU * 0.999), (-5.0, -2.0)] {
        let span = t1 - t0;
        // A generous budget that is still far tighter than the span:
        // the derivation is three dot products and one `atan2`, so a
        // handful of ulps is the honest expectation, and 1e-9 of the
        // span catches any structural widening.
        let budget = span * 1e-9;
        let mid = (t0 + t1) * 0.5;
        for f in [0.001, 0.02, 0.5, 0.98, 0.999] {
            let t = t0 + span * f;
            let p = carrier64.eval(t);
            let got64 = carrier64.param_near(p, mid).unwrap();
            let got = carrier
                .param_near(lift_p(p), Interval::from_f64(mid))
                .unwrap();
            assert!(
                got.lo() <= got64 && got64 <= got.hi(),
                "span ({t0},{t1}) at {t}: f64 {got64} outside [{}, {}]",
                got.lo(),
                got.hi()
            );
            assert!(
                got.lo() > t0 && got.hi() < t1,
                "span ({t0},{t1}) at {t}: enclosure [{}, {}] not strictly inside",
                got.lo(),
                got.hi()
            );
            assert!(
                got.hi() - got.lo() <= budget,
                "span ({t0},{t1}) at {t}: enclosure width {:e} over budget {budget:e}",
                got.hi() - got.lo()
            );
        }
    }
}

/// The FULL period at the cut. `atan2`'s enclosure widens across the
/// negative-`x` axis, so the two points nearest the seam are the ones
/// that must NOT come back mis-selected: the enclosure may be wide,
/// but it must still contain the truth. The width budget is dropped
/// here on purpose — this is the regime where widening is the
/// documented behaviour, and asserting a narrow width would be
/// asserting the derivation does something it never claimed.
#[test]
fn a_full_period_widens_at_the_seam_without_mis_selecting() {
    let (carrier64, carrier) = pair();
    let (t0, t1) = (0.0, TAU);
    let mid = Interval::from_f64((t0 + t1) * 0.5);
    for t in [1e-6, 0.5, PI, TAU - 0.5, TAU - 1e-6] {
        let p = carrier64.eval(t);
        let got = carrier.param_near(lift_p(p), mid).unwrap();
        assert!(
            got.lo() <= t && t <= got.hi(),
            "at {t}: truth outside [{}, {}]",
            got.lo(),
            got.hi()
        );
    }
}

/// **The endpoint-anchored entry point widens the same way.** The
/// offset door anchors at the moving endpoint's own old parameter, not
/// at a span midpoint, so its interval posture is a separate claim:
/// the enclosure of the answer contains the truth for an anchor
/// anywhere on the line, and stays narrow while the point is not half
/// a turn from it.
#[test]
fn an_endpoint_anchor_encloses_the_truth_at_every_offset() {
    let (carrier64, carrier) = pair();
    for near in [-9.0, -PI, 0.0, 0.7, 7.0] {
        for delta in [-3.0, -0.5, 0.0, 0.5, 3.0] {
            let t = near + delta;
            let p = carrier64.eval(t);
            let got = carrier
                .param_near(lift_p(p), Interval::from_f64(near))
                .unwrap();
            assert!(
                got.lo() <= t && t <= got.hi(),
                "near={near} delta={delta}: truth {t} outside [{}, {}]",
                got.lo(),
                got.hi()
            );
            // Away from the cut the enclosure is a handful of ulps, so
            // an anchored read never costs a consumer its gate.
            if delta.abs() < 3.0 {
                assert!(
                    got.hi() - got.lo() < 1e-9,
                    "near={near} delta={delta}: enclosure width {:e}",
                    got.hi() - got.lo()
                );
            }
        }
    }
}

/// The LINE arm carries no `atan2` and so no cut: the enclosure is the
/// projection's own, and it does not move with the anchor.
#[test]
fn the_line_arm_has_no_cut_to_widen_at() {
    let line = Curve3::Line {
        origin: lift_p(Point3::new(0.5, -1.0, 2.0)),
        dir: lift_v(Vec3::new(2.0 / 3.0, 2.0 / 3.0, 1.0 / 3.0)),
    };
    let p = line.eval(Interval::from_f64(3.25));
    let first = line.param_near(p, Interval::from_f64(0.0)).unwrap();
    for near in [-100.0, 0.0, 1e6] {
        let got = line.param_near(p, Interval::from_f64(near)).unwrap();
        assert!(
            got.lo() == first.lo() && got.hi() == first.hi(),
            "near={near}: the line arm's enclosure moved with its anchor"
        );
    }
    assert!(first.lo() <= 3.25 && 3.25 <= first.hi());
}
