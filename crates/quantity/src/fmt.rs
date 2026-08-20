//! The quantity display formatter — `fmt_real`'s DISPLAY sibling
//! (step-export's `fmt_real` is a bit-exact Part 21 printer with no
//! unit vocabulary; this one renders a canonical-units value IN a
//! display unit, with the unit suffix, for humans and for the text
//! parser to read back).
//!
//! THE PIN: `parse(fmt(x, unit))` recovers `x` BIT-EXACTLY, where
//! `parse` is the expression text parser's literal semantics — parse
//! the decimal number, then multiply by the unit's factor in f64
//! (`n * factor`, one rounding). The formatter therefore does not
//! render `x / factor` blindly: it searches the (at most five) f64
//! quotient candidates within 2 ulps of `x / factor` for one whose
//! `candidate * factor` reproduces `x`'s exact bits, and renders the
//! shortest such candidate ({:?}'s shortest-round-trip digits, the
//! same guarantee `fmt_real` builds on).
//!
//! FALLBACK: some values have no such candidate — multiplication by
//! the factor occasionally steps 2 ulps of `x` at a time, skipping
//! values, and a skipped `x` has no preimage in that unit at all.
//! Those values render in the CANONICAL unit (`m`/`rad`), whose
//! factor-1.0 multiply is the f64 identity — the pin holds for every
//! finite input at the price of a canonical-unit suffix. HOW OFTEN
//! (measured, 2M uniform random finite f64 per unit — the PR #267
//! review's figures, replacing an earlier estimate that was low by
//! ~3 orders): mm ~2.8%, cm ~16%, in ~15%, deg ~9.5%; m/rad never.
//! Uniform-random f64 is the worst case: a value AUTHORED in the unit
//! (parsed from `250 mm`, computed as `25.0 * MM`) lies in the parse
//! map's image and always keeps its suffix, so the honest statement
//! is per-computed-value and unit-dependent, not "rare" in general.
//! That trade (never a wrong bit, sometimes a canonical suffix) is
//! this module's ruled fork; the alternative — decimal-exact parse
//! semantics — cannot exist for `DEG` at all (π/180 has no decimal),
//! so multiply semantics is the uniform door and this fallback is its
//! price.
//!
//! NaN/±∞ refuse typed ([`FmtQuantityError::NonFinite`]) exactly as
//! `fmt_real` refuses: poison never lands in display text.

use crate::units::{AngleUnit, LengthUnit, M, RAD};

/// Typed refusal from the display formatter.
#[derive(Debug, Clone, PartialEq)]
pub enum FmtQuantityError {
    /// The value is NaN or ±∞ — non-finite quantities are poison and
    /// have no display form (fail loud, the fmt_real posture).
    NonFinite {
        /// The refused value.
        value: f64,
    },
}

/// Formats a canonical-METERS value in `unit` (module docs; the pin:
/// the text parses back to `meters`' exact bits).
///
/// `unit` is not checked here and does not need to be: a
/// [`crate::LengthUnit`] is an INDEX into [`crate::UNITS`] (the seal,
/// whose narrative lives on [`crate::UnitDef`]), so its symbol and its
/// factor are one row's and the rendered suffix names the factor that
/// was applied. That is what makes this module's `parse(fmt(x, unit))`
/// pin a statement about the whole surface rather than about
/// well-behaved callers.
///
/// # Errors
///
/// [`FmtQuantityError::NonFinite`] when `meters` is NaN or ±∞.
pub fn fmt_length(meters: f64, unit: LengthUnit) -> Result<String, FmtQuantityError> {
    fmt_in(meters, unit.symbol(), unit.factor(), M.symbol())
}

/// Formats a canonical-RADIANS value in `unit` (module docs; the pin:
/// the text parses back to `radians`' exact bits).
///
/// `unit` needs no check, for the reason on [`fmt_length`].
///
/// # Errors
///
/// [`FmtQuantityError::NonFinite`] when `radians` is NaN or ±∞.
pub fn fmt_angle(radians: f64, unit: AngleUnit) -> Result<String, FmtQuantityError> {
    fmt_in(radians, unit.symbol(), unit.factor(), RAD.symbol())
}

/// The shared engine: preimage search in `symbol`, canonical fallback
/// (module docs).
fn fmt_in(
    value: f64,
    symbol: &str,
    factor: f64,
    canonical_symbol: &str,
) -> Result<String, FmtQuantityError> {
    if !value.is_finite() {
        return Err(FmtQuantityError::NonFinite { value });
    }
    let q = value / factor;
    // The five candidates within 2 ulps of the rounded quotient. This
    // set is COMPLETE (the PR #267 review's derivation, confirmed by a
    // 2.63M-probe attack finding zero misses): a preimage d satisfies
    // |d·factor − value| ≤ ulp(value)/2, so |d − value/factor| ≤
    // ulp(value)/(2·factor) < spacing(q) (ulp(value) ≤ 2⁻⁵²·|value|);
    // adding |q − value/factor| ≤ spacing/2 gives |d − q| < 1.5
    // spacings — at most 2 next-steps from q even where a binade
    // boundary halves the spacing. For subnormal value the quotient
    // interval is wide and q itself is a preimage. If none of the five
    // reproduces the bits, no decimal in this unit does.
    let candidates = [
        q,
        q.next_up(),
        q.next_down(),
        q.next_up().next_up(),
        q.next_down().next_down(),
    ];
    let best = candidates
        .into_iter()
        .filter(|d| (d * factor).to_bits() == value.to_bits())
        .map(render_shortest)
        .min_by_key(String::len);
    Ok(match best {
        Some(digits) => format!("{digits} {symbol}"),
        // No preimage in the requested unit: canonical fallback —
        // factor exactly 1.0, multiplication is the identity, the
        // rendered digits ARE the value's shortest round-trip form.
        None => format!("{} {canonical_symbol}", render_shortest(value)),
    })
}

/// `{:?}`'s shortest-round-trip digits, with a bare integral form
/// (`250.0` → `250`) — the stripped form parses to the identical f64
/// (`from_str("250")` and `from_str("250.0")` agree), and it matches
/// the spec's `25 mm` surface shape.
fn render_shortest(d: f64) -> String {
    let repr = format!("{d:?}");
    match repr.strip_suffix(".0") {
        Some(integral) => integral.to_string(),
        None => repr,
    }
}
