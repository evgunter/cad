//! The **interpretation budget** (M7-2 Leg A): a per-value effective
//! input tolerance that generalizes ε_in with what each literal's own
//! printed text can support.
//!
//! # Why a foreign file needs one
//!
//! The kernel's own writer prints reals in the shortest form that
//! round-trips to identical bits, so own-corpus values ARE their
//! literals and exact `==` is legitimate. A foreign writer need not:
//! Open CASCADE 7.8 (what FreeCAD 1.1.2 emits through) caps mantissas
//! at 12–13 significant digits, so a π-derived value arrives truncated
//! — `0.785398163397` for π/4, which misses the double-precision
//! identity by 4.5e-13 relative, sometimes MORE than the file's own
//! declared uncertainty (1e-7 mm = 1e-10 m). Gating such a value
//! against ε_in alone would refuse geometry the file states as well as
//! it can state it.
//!
//! # The rule
//!
//! `eps_in_eff(x) = max(ε_in, half of x's last printed decimal place)`,
//! where the second term is **zero for a literal that round-trips**:
//!
//! - A literal with FEWER than [`SIGNIFICANT_DIGIT_CAP`] significant
//!   digits was not truncated — a shortest-round-trip printer under a
//!   digit cap only loses digits when it reaches the cap — so its text
//!   pins the value exactly and it contributes nothing. This is what
//!   keeps exact `==` for the dyadics the file prints exactly (`0.`,
//!   `1.`, `0.5`, `0.25`).
//! - A literal AT the cap may have been truncated there, and the most
//!   its text can support is half of its last printed place: for
//!   `0.785398163397` (12 significant digits, last place 1e-12) that
//!   is 5e-13, which covers π/4's 4.5e-13 miss and nothing wider.
//!
//! The term is therefore data-driven and never widens beyond what the
//! file's own text can support — the opposite of a global fudge
//! factor. It is a **firm design decision**, flagged in the PR: a
//! foreign writer that truncates at 12 digits *without* being
//! shortest-round-trip below the cap would be under-budgeted here, and
//! the answer to that would be a declared uncertainty that says so,
//! not a wider default.

/// Open CASCADE 7.8's printed-mantissa cap in significant digits (the
/// measured dialect: 12–13, the low end taken). A literal at or above
/// this many significant digits may have been truncated to it; one
/// below it round-trips.
pub const SIGNIFICANT_DIGIT_CAP: usize = 12;

/// Half of `literal`'s last printed decimal place — the most the
/// literal's own text can support as an uncertainty — or `0.0` when
/// the literal round-trips exactly (module docs).
///
/// The argument is the raw Part 21 token (`-1.499759782662E-32`,
/// `1.E-07`, `0.5`); a token that is not a real returns `0.0` (the
/// caller's `str::parse` is what refuses malformed reals, and this
/// budget never invents slack for text it cannot read).
#[must_use]
pub fn print_half_ulp(literal: &str) -> f64 {
    let body = literal.strip_prefix(['+', '-']).unwrap_or(literal);
    let (mantissa, exponent) = match body.split_once(['E', 'e']) {
        Some((m, e)) => match e.parse::<i32>() {
            Ok(e) => (m, e),
            Err(_) => return 0.0,
        },
        None => (body, 0),
    };
    let (int_part, frac_part) = mantissa.split_once('.').unwrap_or((mantissa, ""));
    if !int_part.chars().all(|c| c.is_ascii_digit()) || !frac_part.chars().all(|c| c.is_ascii_digit())
    {
        return 0.0;
    }
    // Significant digits: every printed digit past the leading zeros.
    // An all-zero mantissa (`0.`) is exact and significant-free.
    let digits: String = format!("{int_part}{frac_part}");
    let significant = digits.trim_start_matches('0').len();
    if significant < SIGNIFICANT_DIGIT_CAP {
        return 0.0;
    }
    // The last printed place: 10^(exponent − fractional digits). Read
    // back through the decimal parser rather than `powi`, so the term
    // IS the correctly-rounded decimal place the text names (`powi`
    // accumulates its own rounding, which would make the budget an
    // ulp-off approximation of the thing it is quoting).
    let place = exponent - i32::try_from(frac_part.len()).unwrap_or(i32::MAX);
    let decimal: f64 = format!("1e{place}").parse().unwrap_or(0.0);
    0.5 * decimal
}

/// The effective input tolerance for a value assembled from `literals`
/// at input tolerance `eps_in`, both in the SAME unit (callers scale
/// length literals into meters before asking).
///
/// Non-finite `eps_in` is returned as-is — the caller's refusal, not
/// this function's to hide.
#[must_use]
pub fn eps_in_eff(eps_in: f64, literals: &[&str]) -> f64 {
    literals
        .iter()
        .fold(eps_in, |acc, l| acc.max(print_half_ulp(l)))
}

#[cfg(test)]
mod tests {
    use super::{eps_in_eff, print_half_ulp};

    /// The π-derived class M7-2 spec row 6 names: the measured cone
    /// semi-angles. Their printed text supports exactly enough slack to
    /// cover the truncation and no more.
    #[test]
    fn pi_derived_semi_angles_are_budgeted_by_their_own_text() {
        // cone_apex.step: CONICAL_SURFACE('',#39,1.,0.785398163397).
        let quarter_pi = "0.785398163397";
        let slack = print_half_ulp(quarter_pi);
        assert_eq!(slack, 5e-13, "12 significant digits, last place 1e-12");
        let miss = (std::f64::consts::FRAC_PI_4 - quarter_pi.parse::<f64>().unwrap()).abs();
        assert!(miss <= slack, "the truncation {miss} must fit the budget {slack}");
        assert!(miss > 1e-13, "and the budget must not be slack for slack's sake");

        // cone_trunc.step: semi-angle atan(1/2).
        let atan_half = "0.463647609001";
        let slack = print_half_ulp(atan_half);
        assert_eq!(slack, 5e-13);
        assert!((0.5f64.atan() - atan_half.parse::<f64>().unwrap()).abs() <= slack);
    }

    /// Dyadics the file prints exactly contribute nothing — exact `==`
    /// survives for them (module docs).
    #[test]
    fn exactly_printed_literals_contribute_no_slack() {
        for exact in ["0.", "1.", "0.5", "0.25", "2.", "-0.", "1.E-07", "0.463647609"] {
            assert_eq!(print_half_ulp(exact), 0.0, "{exact} round-trips");
        }
        assert_eq!(eps_in_eff(1e-10, &["1.", "0.5"]), 1e-10, "ε_in still floors");
    }

    /// The trig-noise class: 13 significant digits at a tiny exponent
    /// budgets a tiny slack, not a tiny-value blanket.
    #[test]
    fn noise_literals_budget_against_their_own_exponent() {
        assert_eq!(print_half_ulp("6.123233995737E-17"), 0.5e-29);
        assert_eq!(print_half_ulp("-1.499759782662E-32"), 0.5e-44);
        assert_eq!(print_half_ulp("-2.449293598295E-16"), 0.5e-28);
        // ε_in dominates every one of them.
        assert_eq!(eps_in_eff(1e-10, &["6.123233995737E-17"]), 1e-10);
    }

    /// Text this budget cannot read gets no slack invented for it.
    #[test]
    fn unreadable_tokens_get_no_slack() {
        for bad in ["", "-", "1.0E", "abc", "1.0E+"] {
            assert_eq!(print_half_ulp(bad), 0.0);
        }
    }
}
