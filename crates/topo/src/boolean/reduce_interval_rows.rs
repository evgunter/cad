//! The interval lane's rows for the `Circle` split parameter.
//!
//! Here rather than in `reduce.rs`'s test module for one reason: the
//! CI filter forces the interval compile mode on a changed file whose
//! basename carries `interval` (`scripts/ci-filter.py`'s
//! `_forces_interval`), and otherwise samples it. The shipped code has
//! no interval-gated block to name — the mid anchor's whole claim is
//! that there is no fork — so this suite is the only interval-specific
//! artifact the rule can match, and it is named so the lane is pinned.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{PI, TAU};

use geom_core::{Bounds, Point3, Real, Vec3, interval::Interval};

use super::circle_split_param;

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

/// The f64 carrier and its componentwise lift — the SAME circle at two
/// scalars, which is what makes the enclosure comparison meaningful.
fn pair() -> (geom::Curve3<f64>, geom::Curve3<Interval>, Point3<f64>) {
    let center = Point3::new(1.0, 2.0, 3.0);
    (
        geom::Curve3::Circle {
            center,
            axis: Vec3::unit_z(),
            radius: 1.5,
            u_ref: Vec3::unit_x(),
        },
        geom::Curve3::Circle {
            center: lift_p(center),
            axis: lift_v(Vec3::unit_z()),
            radius: Interval::from_f64(1.5),
            u_ref: lift_v(Vec3::unit_x()),
        },
        center,
    )
}

/// **The lane runs the same body, and the row can go red if it stops
/// doing so.** Three things are asserted, and each is a different way
/// the guarantee could degrade:
///
/// 1. **Enclosure** — the interval answer contains the `f64` one. A
///    mis-selected branch would put the `f64` value outside a narrow
///    enclosure a whole turn away, which is the failure the mid anchor
///    exists to prevent.
/// 2. **Strictly inside the span** — the enclosure as a whole, not
///    just its centre. This is what `split_edge`'s interiority trilean
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
/// claims widens rather than mis-selects — which the first shipped row
/// (span `(0, π)`, `w·r̂` definitely positive) never approached. Its
/// author found that gap; the row is adopted from the R1 probe branch
/// and given teeth.
#[test]
fn r1_the_interval_lane_holds_at_the_cut() {
    let (carrier64, carrier, center64) = pair();
    for (t0, t1) in [(0.0, PI), (0.0, TAU * 0.999), (-5.0, -2.0)] {
        let span = t1 - t0;
        // A generous budget that is still far tighter than the span:
        // the derivation is three dot products and one `atan2`, so a
        // handful of ulps is the honest expectation, and 1e-9 of the
        // span catches any structural widening.
        let budget = span * 1e-9;
        for f in [0.001, 0.02, 0.5, 0.98, 0.999] {
            let t = t0 + span * f;
            let p = carrier64.eval(t);
            let got64 = circle_split_param(&carrier64, center64, t0, t1, p);
            let got = circle_split_param(
                &carrier,
                lift_p(center64),
                Interval::from_f64(t0),
                Interval::from_f64(t1),
                lift_p(p),
            );
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
fn r1_a_full_period_widens_at_the_seam_without_mis_selecting() {
    let (carrier64, carrier, center64) = pair();
    let (t0, t1) = (0.0, TAU);
    for t in [1e-6, 0.5, PI, TAU - 0.5, TAU - 1e-6] {
        let p = carrier64.eval(t);
        let got = circle_split_param(
            &carrier,
            lift_p(center64),
            Interval::from_f64(t0),
            Interval::from_f64(t1),
            lift_p(p),
        );
        assert!(
            got.lo() <= t && t <= got.hi(),
            "at {t}: truth outside [{}, {}]",
            got.lo(),
            got.hi()
        );
    }
}
