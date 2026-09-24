//! **Rule G's exact quotient** ([`SymRules::root_quotient`]) — a root
//! whose argument's denominator divides its numerator exactly is
//! minted over the polynomial quotient — driven through
//! `Sym<Interval>` at the scalar door.
//!
//! Three rows, each a soundness row first (the value channel's
//! enclosure must admit zero wherever the tier says zero):
//!
//! 1. the positive row: the shape R1's boss carries at its `arc_span`
//!    identity, `sqrt(5·(a + h)⁶/(a + h)⁴)` against `sqrt(5)·|a + h|`,
//!    is a THEOREM with the dial on and not with it off;
//! 2. the negative row: the quotient of a SIGN-CARRYING square,
//!    `sqrt(t²·(1 + t)²/(1 + t)²)`, is `|t|` and never `t` — against
//!    `t` it stays opaque over a box where `t > 0` (true there, and a
//!    claim no value-free rule may make) and over one where `t < 0`
//!    (false there), and against `|t|` it is a theorem;
//! 3. the decline row: halves that share a factor neither divides out
//!    (`(1 + t)(2 + t)/((1 + t)(3 + t))`) are left as the walk built
//!    them — the rewrite is not a GCD.

#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::predicate::{Band, Margin, Sign};
use geom_core::sym::with_session_rules;
use geom_core::{Decide, Interval, ParamSymbol, Real, Sym, SymBudget, SymCounts, SymRules, Tol};

fn budget() -> SymBudget {
    SymBudget {
        max_terms: 4096,
        max_degree: 128,
    }
}

fn band() -> Band {
    Band::linear(Tol::witness()).expect("linear band")
}

fn lit(v: f64) -> Sym<Interval> {
    Sym::from_f64(v)
}

fn over(name: &str, lo: f64, hi: f64) -> Sym<Interval> {
    Sym::param_over(ParamSymbol::of(name), Interval::from_bounds(lo, hi), lo, hi)
}

/// How the tier answered, as one word.
fn label(out: Result<Sign, geom_core::predicate::Indeterminate>, c: SymCounts) -> String {
    match out {
        Ok(Sign::Zero) if c.symbolic_zero > 0 => "theorem".to_owned(),
        Ok(Sign::Zero) if c.registered > 0 => "registered".to_owned(),
        Ok(Sign::Zero) if c.sign_gated > 0 => "sign_gated".to_owned(),
        Ok(s) => format!("numeric {s:?}"),
        Err(e) => format!("refused {:?}", e.margin),
    }
}

/// One decision under `rules`, with the soundness check every row
/// makes: a zero the tier claims is one the enclosure admits.
fn decide(what: &str, rules: SymRules, build: impl FnOnce() -> Sym<Interval>) -> String {
    let ((out, value), counts) = with_session_rules(budget(), rules, || {
        let m = build();
        (
            geom_core::k_stats::decide("decide_4_root_quotient_rows", Margin::of(m), band()),
            m.value,
        )
    });
    let l = label(out, counts);
    let v = value.enclosure_probe();
    println!("  {what}: {l} enclosure {v:?}");
    if matches!(l.as_str(), "theorem" | "sign_gated" | "registered") {
        let (lo, hi) = v.expect("a certified enclosure");
        assert!(
            lo <= 1e-9 && hi >= -1e-9,
            "{what}: the tier SAYS ZERO and the value channel encloses [{lo:e}, {hi:e}] — UNSOUND"
        );
    }
    l
}

/// `sqrt(N/D)` spelled so that the walk keeps `D` as a POLYNOMIAL
/// factor of both halves: `(k·p^num) / p^den` for a non-monomial `p`.
fn root_over_shared_factor(k: f64, p: Sym<Interval>, num: i32, den: i32) -> Sym<Interval> {
    (lit(k) * p.powi(num) / p.powi(den)).sqrt()
}

/// **The positive row: the boss's shape.** `h` a chord deviation over
/// a box, `a + h` the chord half: the root over the quotient with the
/// chord's polynomial to the fourth power in both halves meets
/// `sqrt(5)·|a + h|` with the dial on, and does not with it off. `a`
/// is dyadic so that `(a + h)⁶`'s coefficients stay inside the ring
/// and the row measures the quotient, not a freeze.
#[test]
fn a_root_whose_denominator_divides_its_numerator_is_the_quotients_root() {
    let build = || {
        let h = over("h", -1.0e-4, 1.0e-4);
        let p = lit(0.5) + h;
        root_over_shared_factor(5.0, p, 6, 4) - lit(5.0).sqrt() * p.abs()
    };
    let on = decide("dial on", SymRules::shipped(), build);
    let off = decide("dial off", SymRules::without_root_quotient(), build);
    assert_eq!(on, "theorem", "the exact quotient takes the boss's shape");
    assert_ne!(
        off, "theorem",
        "without the rewrite the root is keyed on the quotient and the shape stands"
    );
}

/// **The negative row: a sign-carrying square is a magnitude, never
/// the signed root.** `sqrt(t²(1 + t)²/(1 + t)²)` is `|t|`: against `t`
/// it is NOT a theorem on either side of zero — over `t ∈ [0.5, 1]`
/// the identity `|t| = t` is true but only a read of the box could say
/// so, and over `t ∈ [−0.9, −0.5]` it is false — while against `|t|` it
/// is a theorem on both.
#[test]
fn a_sign_carrying_quotient_root_stays_a_magnitude() {
    for (lo, hi) in [(0.5, 1.0), (-0.9, -0.5)] {
        let signed = decide(
            &format!("sqrt(t^2 (1+t)^2 / (1+t)^2) - t, t in [{lo}, {hi}]"),
            SymRules::shipped(),
            || {
                let t = over("t", lo, hi);
                let p = lit(1.0) + t;
                (t.powi(2) * p.powi(2) / p.powi(2)).sqrt() - t
            },
        );
        assert_ne!(
            signed, "theorem",
            "|t| = t is a SIGN of t, and the rewrite reads none (t in [{lo}, {hi}])"
        );
        let magnitude = decide(
            &format!("sqrt(t^2 (1+t)^2 / (1+t)^2) - |t|, t in [{lo}, {hi}]"),
            SymRules::shipped(),
            || {
                let t = over("t", lo, hi);
                let p = lit(1.0) + t;
                (t.powi(2) * p.powi(2) / p.powi(2)).sqrt() - t.abs()
            },
        );
        assert_eq!(
            magnitude, "theorem",
            "the quotient's root is the magnitude |t| (t in [{lo}, {hi}])"
        );
    }
}

/// **The decline row: not a GCD.** `(1 + t)(2 + t)/((1 + t)(3 + t))`
/// shares `(1 + t)` but neither half divides the other, so the root
/// over it is left keyed on the quotient the walk built — the same
/// answer with the dial on and off.
#[test]
fn a_shared_factor_neither_half_divides_is_not_taken() {
    let build = || {
        let t = over("t", 0.5, 1.0);
        let q = (lit(1.0) + t) * (lit(2.0) + t) / ((lit(1.0) + t) * (lit(3.0) + t));
        q.sqrt() - ((lit(2.0) + t) / (lit(3.0) + t)).sqrt()
    };
    let on = decide("dial on", SymRules::shipped(), build);
    let off = decide("dial off", SymRules::without_root_quotient(), build);
    assert_eq!(
        on, off,
        "a factor neither half divides is not the rewrite's"
    );
}
