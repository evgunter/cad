//! Parameter-interval helpers shared by every carrier kind.

use geom_core::Real;

/// The middle of the parameter interval `[t0, t1]`, spelled
/// `(t0 + t1)·½`.
///
/// Each end enters once, so at `T = Interval` the enclosure is as tight
/// as the ends allow (`t0 + (t1 − t0)·½` reads `t0` twice and widens by
/// its width). At `f64` the result lies in `[t0, t1]`; it overflows only
/// when `t0 + t1` does (two same-sign ends near the range limit), which
/// no curve parameter approaches.
pub fn mid_param<T: Real>(t0: T, t1: T) -> T {
    (t0 + t1) * T::from_f64(0.5)
}

/// **Whether the angle bracket `phi` may hold a point of the periodic
/// window `window`** — whether some `2πk`-translate of `[phi.0, phi.1]`
/// possibly meets the closed interval `[window.0, window.1]`. `false`
/// is the only definite answer: every point of the bracket lies
/// certainly outside every translate of the window.
///
/// The one home of that question for a caller that SELECTS between two
/// sound bounds by it — an arc's end values where the angle that
/// realises a whole circle's extreme is certainly off the arc, the
/// whole circle's otherwise ([`crate::curves::boxes::circle_arc_aabb`],
/// and `sweep`'s ring-clearance piece meters). Such a caller reads
/// `true` as "take the bound that holds either way", so `true` is the
/// answer wherever exclusion is not proved: a NaN anywhere, a bracket
/// or a window a period wide or wider (which holds every angle).
///
/// **The window's ends are taken as given.** Within one translate the
/// test is an exact comparison of `f64`s, the translate's integer is
/// chosen by monotone rounding (so a bracket within rounding of a
/// period off the window's start reads the nearer translate, the
/// inclusive side), and the period is `f64`'s `TAU`. What that leaves
/// is a misplacement within rounding of a window END, which costs
/// nothing to a caller whose two bounds agree there (an end IS the
/// point realising the extreme as the angle crosses it); a caller whose
/// own arithmetic is not outward widens the window before asking (the
/// curve boxes' `ANGLE_SLOP`).
///
/// A DECIDED membership — a point inside, outside or in band of a
/// window's edge, metered in metres and escalating in band — is a
/// different question with its own homes in `topo` (the chart
/// windows' cosine construction, the arc trim's chordal one): there an
/// uncertain answer must escalate, here it must loosen.
pub fn angle_window_may_hold((phi_lo, phi_hi): (f64, f64), (lo, hi): (f64, f64)) -> bool {
    if phi_lo.is_nan() || phi_hi.is_nan() || lo.is_nan() || hi.is_nan() {
        return true;
    }
    let tau = core::f64::consts::TAU;
    if hi - lo >= tau || phi_hi - phi_lo >= tau {
        return true;
    }
    // The smallest k with phi_hi + τ·k ≥ lo; the bracket meets the
    // window iff that translate's lower end clears hi from below.
    let k = ((lo - phi_hi) / tau).ceil();
    let rep_lo = phi_lo + tau * k;
    // NaN-inclusive: a poisoned representative cannot prove exclusion.
    rep_lo.is_nan() || rep_lo <= hi
}

#[cfg(test)]
mod tests {
    use super::angle_window_may_hold;
    use core::f64::consts::{PI, TAU};

    /// A bracket inside the window, and one inside it a turn away, may
    /// hold; one between the window's translates certainly does not.
    #[test]
    fn a_bracket_is_read_against_every_translate_of_the_window() {
        let window = (0.5, 1.5);
        assert!(angle_window_may_hold((1.0, 1.0), window));
        assert!(angle_window_may_hold((1.0 + TAU, 1.0 + TAU), window));
        assert!(angle_window_may_hold(
            (1.0 - 3.0 * TAU, 1.0 - 3.0 * TAU),
            window
        ));
        assert!(!angle_window_may_hold((2.0, 2.0), window));
        assert!(!angle_window_may_hold((2.0 - TAU, 2.0 - TAU), window));
        // A bracket straddling an end may hold.
        assert!(angle_window_may_hold((1.4, 1.6), window));
        assert!(angle_window_may_hold((0.4 + TAU, 0.6 + TAU), window));
    }

    /// The window is closed: a bracket on either end holds.
    #[test]
    fn the_windows_ends_are_inside_it() {
        let window = (0.5, 1.5);
        assert!(angle_window_may_hold((1.5, 1.5), window));
        assert!(angle_window_may_hold((0.5, 0.5), window));
        assert!(!angle_window_may_hold((1.5 + 1e-12, 1.5 + 1e-12), window));
        assert!(!angle_window_may_hold((0.5 - 1e-12, 0.5 - 1e-12), window));
    }

    /// Exclusion is the one definite answer, so a whole-turn window, a
    /// whole-turn bracket and a NaN anywhere all may hold.
    #[test]
    fn what_cannot_be_excluded_may_hold() {
        assert!(angle_window_may_hold((PI, PI), (0.0, TAU)));
        assert!(angle_window_may_hold((-1.0, -1.0 + TAU), (0.5, 0.6)));
        assert!(angle_window_may_hold((f64::NAN, 1.0), (0.5, 0.6)));
        assert!(angle_window_may_hold((2.0, 2.0), (f64::NAN, 0.6)));
    }

    /// A window wrapping past its own translate's start — `[5, 7]` holds
    /// `0.5` a turn on — is read across the wrap.
    #[test]
    fn a_window_across_a_turn_holds_the_angle_past_it() {
        assert!(angle_window_may_hold((0.5, 0.5), (5.0, 7.0)));
        assert!(!angle_window_may_hold((1.0, 1.0), (5.0, 7.0)));
    }
}
