//! **The authored angle askers agree with the spellings they replaced**
//! (the PRED charter's agreement test, `geom_core::period`'s turn rule):
//! the revolve's partial angle and the tube's arc window. Each old
//! spelling is an oracle here, bit for bit and generic over the scalar
//! as the shipped code was, run against the shipped site on a full
//! turn, `τ ± k·e/rate` for `k ∈ {0.5, 3, 20}` at both band edges `e`,
//! in both senses where the input is signed, a short angle, a zero one,
//! one in the zero band and one in the band, and a reversed window. No
//! case moves: both rates are exact, and the sign and forward halves
//! read the same margin through `Margin::metered` as through `levered`.

#![allow(clippy::expect_used, clippy::panic)]

use core::f64::consts::{FRAC_PI_2, TAU};

use geom_core::k_stats::decide;
use geom_core::{Band, Decide, Indeterminate, Margin, Sign, Tol};

use super::RevolveError;
use super::tube::{TubeDecision, TubeError, arc_window};

/// What an asker reached.
#[derive(Clone, Copy, Debug, PartialEq)]
enum Outcome {
    /// Admitted, with the revolve's reversal or the tube's span.
    Admitted(f64),
    /// The angle or window is degenerate.
    Degenerate,
    /// The angle or window is a full turn, or past one.
    FullRange,
    /// Escalated under a name, with the point margin.
    Escalated(Option<&'static str>, Option<f64>),
}

fn escalated(cause: Indeterminate) -> Outcome {
    let margin = cause.margin.diagnostic_f64_for_error_text().value();
    Outcome::Escalated(cause.predicate, margin)
}

fn band() -> Band {
    Band::linear(Tol::witness()).expect("the witness band")
}

/// The defects, labelled by `k·e/ε`, in radians at `rate`; positive is
/// past a turn.
fn defects(band: Band, rate: f64) -> Vec<(f64, f64)> {
    let mut out = vec![(0.0, 0.0)];
    for edge in [band.zero(), band.escalate()] {
        for k in [0.5, 3.0, 20.0] {
            for sign in [1.0, -1.0] {
                out.push((
                    sign * k * (edge / band.zero()).round(),
                    sign * k * edge / rate,
                ));
            }
        }
    }
    out
}

/// **The oracle**: the revolve's partial angle as it was spelled.
fn old_revolve<T: Decide>(t: T, r_max: T, band: Band) -> Outcome {
    let reverse = match decide("revolve_angle", Margin::levered(t, r_max), band) {
        Err(cause) => return escalated(cause),
        Ok(Sign::Zero) => return Outcome::Degenerate,
        Ok(Sign::Positive) => true,
        Ok(Sign::Negative) => false,
    };
    let headroom = Margin::levered(T::tau() - t.abs(), r_max);
    match decide("revolve_angle_headroom", headroom, band) {
        Err(cause) => escalated(cause),
        Ok(Sign::Positive) => Outcome::Admitted(if reverse { 1.0 } else { 0.0 }),
        Ok(Sign::Zero | Sign::Negative) => Outcome::FullRange,
    }
}

fn new_revolve(t: f64, r_max: f64, band: Band) -> Outcome {
    match super::partial_angle_reverses(t, r_max, band) {
        Ok(reverse) => Outcome::Admitted(if reverse { 1.0 } else { 0.0 }),
        Err(RevolveError::DegenerateAngle) => Outcome::Degenerate,
        Err(RevolveError::FullRangeAngle) => Outcome::FullRange,
        Err(RevolveError::AngleEscalated { source }) => escalated(source),
        Err(other) => panic!("the angle refuses only as an angle: {other:?}"),
    }
}

/// **A revolve's partial angle agrees with its old spelling.**
#[test]
fn the_revolve_angle_agrees_with_the_old_pair() {
    let band = band();
    let r_max = 3.5;
    let eps = band.zero();
    let mut angles: Vec<f64> = defects(band, r_max).iter().map(|&(_, d)| TAU + d).collect();
    angles.extend([FRAC_PI_2, 0.0, 0.5 * eps / r_max, 3.0 * eps / r_max]);
    for sense in [1.0, -1.0] {
        for &angle in &angles {
            let t = sense * angle;
            assert_eq!(
                new_revolve(t, r_max, band),
                old_revolve(t, r_max, band),
                "t = {t}"
            );
        }
    }
    assert_eq!(new_revolve(0.0, r_max, band), Outcome::Degenerate);
    assert_eq!(new_revolve(TAU, r_max, band), Outcome::FullRange);
}

/// The tube's funnel as the oracle spells it, under the closed
/// decision's own name.
fn old_decide<T: Decide>(
    decision: TubeDecision,
    margin: Margin<T>,
    band: Band,
) -> Result<Sign, Indeterminate> {
    let name = decision.predicate();
    decide(name, margin, band)
}

/// **The oracle**: the tube's arc window as it was spelled.
fn old_tube<T: Decide + Into<f64>>(t0: T, t1: T, arm: T, band: Band) -> Outcome {
    let span = t1 - t0;
    match old_decide(TubeDecision::WindowSpan, Margin::levered(span, arm), band) {
        Err(cause) => return escalated(cause),
        Ok(Sign::Positive) => {}
        Ok(Sign::Zero | Sign::Negative) => return Outcome::Degenerate,
    }
    let headroom = Margin::levered(T::tau() - span, arm);
    match old_decide(TubeDecision::WindowHeadroom, headroom, band) {
        Err(cause) => escalated(cause),
        Ok(Sign::Positive) => Outcome::Admitted(span.into()),
        Ok(Sign::Zero | Sign::Negative) => Outcome::FullRange,
    }
}

fn new_tube(t0: f64, t1: f64, arm: f64, band: Band) -> Outcome {
    match arc_window(t0, t1, arm, band) {
        Ok(span) => Outcome::Admitted(span),
        Err(TubeError::DegenerateWindow) => Outcome::Degenerate,
        Err(TubeError::FullRangeWindow) => Outcome::FullRange,
        Err(TubeError::Escalated { source, .. }) => escalated(source),
        Err(other) => panic!("the window refuses only as a window: {other:?}"),
    }
}

/// **A tube's arc window agrees with its old spelling, bit for bit**,
/// the admitted span included.
#[test]
fn the_tube_window_agrees_with_the_old_pair() {
    let band = band();
    let arm = 1.4;
    let eps = band.zero();
    let t0 = -0.25;
    let mut windows: Vec<(f64, f64)> = defects(band, arm)
        .iter()
        .map(|&(_, d)| (t0, t0 + TAU + d))
        .collect();
    windows.extend([
        (t0, t0 + FRAC_PI_2),
        (t0, t0),
        (t0, t0 + 0.5 * eps / arm),
        (t0, t0 + 3.0 * eps / arm),
        (t0, t0 - 0.5),
    ]);
    for (a, b) in windows {
        assert_eq!(
            new_tube(a, b, arm, band),
            old_tube(a, b, arm, band),
            "[{a}, {b}]"
        );
    }
    assert_eq!(new_tube(t0, t0 - 0.5, arm, band), Outcome::Degenerate);
}
