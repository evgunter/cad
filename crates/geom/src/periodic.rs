//! Membership against a periodic window, read on `f64` brackets.

/// **Whether some point of the bracket `[phi.0, phi.1]` may lie in some
/// `period·k`-translate of the closed window `[window.0, window.1]`**,
/// for integer `k`. `false` is the one definite answer: every point of
/// the bracket lies outside every translate of the window. `true` is
/// the answer wherever that is not proved — a NaN anywhere, a period
/// that is not positive and finite, or a bracket or a window a period
/// wide or wider (which meets every translate).
///
/// **How exact it is.** The translate tried is the smallest `k` that
/// puts the bracket's upper end at or past the window's start,
/// `⌈(window.0 − phi.1) / period⌉`. Rounding in that quotient can only
/// pick `k` one too small, and that translate's lower end then lies
/// below the window's start, so it answers `true`. The comparison
/// against the window's end is exact on the `f64` value
/// `phi.0 + period·k`: at `k = 0` that is `phi.0` itself, and at
/// `k = ±1` one correctly rounded addition, which is monotone. Further
/// translates round `period·k` too. The window's ends and the period
/// are taken as given; a caller whose own arithmetic is not outward
/// widens the window before asking.
pub fn periodic_window_may_hold(
    (phi_lo, phi_hi): (f64, f64),
    (lo, hi): (f64, f64),
    period: f64,
) -> bool {
    if [phi_lo, phi_hi, lo, hi, period].iter().any(|x| x.is_nan())
        || !(period > 0.0 && period.is_finite())
    {
        return true;
    }
    if hi - lo >= period || phi_hi - phi_lo >= period {
        return true;
    }
    let k = ((lo - phi_hi) / period).ceil();
    let rep_lo = phi_lo + period * k;
    // NaN-inclusive: a poisoned representative cannot prove exclusion.
    rep_lo.is_nan() || rep_lo <= hi
}

#[cfg(test)]
mod tests {
    use super::periodic_window_may_hold as may_hold;
    use core::f64::consts::{FRAC_PI_2, PI, TAU};

    /// A bracket inside the window, and one inside it a turn away, may
    /// hold; one between the window's translates certainly does not.
    #[test]
    fn a_bracket_is_read_against_every_translate_of_the_window() {
        let window = (0.5, 1.5);
        assert!(may_hold((1.0, 1.0), window, TAU));
        assert!(may_hold((1.0 + TAU, 1.0 + TAU), window, TAU));
        assert!(may_hold((1.0 - 3.0 * TAU, 1.0 - 3.0 * TAU), window, TAU));
        assert!(!may_hold((2.0, 2.0), window, TAU));
        assert!(!may_hold((2.0 - TAU, 2.0 - TAU), window, TAU));
        // A bracket straddling an end may hold.
        assert!(may_hold((1.4, 1.6), window, TAU));
        assert!(may_hold((0.4 + TAU, 0.6 + TAU), window, TAU));
    }

    /// The window is closed: a bracket on either end holds.
    #[test]
    fn the_windows_ends_are_inside_it() {
        let window = (0.5, 1.5);
        assert!(may_hold((1.5, 1.5), window, TAU));
        assert!(may_hold((0.5, 0.5), window, TAU));
        assert!(!may_hold((1.5 + 1e-12, 1.5 + 1e-12), window, TAU));
        assert!(!may_hold((0.5 - 1e-12, 0.5 - 1e-12), window, TAU));
    }

    /// Exclusion is the one definite answer, so a whole-period window, a
    /// whole-period bracket, a NaN anywhere and a period that is no
    /// period all may hold.
    #[test]
    fn what_cannot_be_excluded_may_hold() {
        assert!(may_hold((PI, PI), (0.0, TAU), TAU));
        assert!(may_hold((-1.0, -1.0 + TAU), (0.5, 0.6), TAU));
        assert!(may_hold((f64::NAN, 1.0), (0.5, 0.6), TAU));
        assert!(may_hold((2.0, 2.0), (f64::NAN, 0.6), TAU));
        for period in [0.0, -TAU, f64::INFINITY, f64::NAN] {
            assert!(may_hold((2.0, 2.0), (0.5, 0.6), period));
        }
    }

    /// A window across a period boundary — `[5, 7]` holds `0.5` a turn
    /// on — is read across it.
    #[test]
    fn a_window_across_a_turn_holds_the_angle_past_it() {
        assert!(may_hold((0.5, 0.5), (5.0, 7.0), TAU));
        assert!(!may_hold((1.0, 1.0), (5.0, 7.0), TAU));
    }

    /// The period is the caller's: `π/2` recurs every `π`, so `[2, 4.8]`
    /// holds it (at `3π/2`) and `[2, 4.7]` does not.
    #[test]
    fn a_half_turn_period_reads_its_own_translates() {
        assert!(may_hold((FRAC_PI_2, FRAC_PI_2), (2.0, 4.8), PI));
        assert!(!may_hold((FRAC_PI_2, FRAC_PI_2), (2.0, 4.7), PI));
        assert!(!may_hold((FRAC_PI_2, FRAC_PI_2), (2.0, 4.7), TAU));
    }
}
