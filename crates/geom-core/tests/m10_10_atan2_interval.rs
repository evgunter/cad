//! **Rule D's `atan2` fold over a BOX** (M10-10, spec amendment A1):
//! at `Sym<Interval>` the numeric channel decides `atan2(0, N) − 0` at
//! a point or over a box exactly as a value, and the form decides it as
//! a THEOREM — for an `N` non-negative by its syntax, including the
//! chart phase's own `r²/sqrt(r²)` over a `r` whose box STRADDLES zero
//! — while a box on which `N`'s own evaluation is undefined refuses
//! through clause 1 before any form is asked, and a plain parameter in
//! `N`'s place never folds.
//!
//! Every expression is built INSIDE the session: a node minted before
//! the session is installed is an unknown to it (`geom_core::sym`'s
//! "no session, no tier"), which is the conservative direction and not
//! the thing under test.
#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::predicate::{Band, Margin, Sign};
use geom_core::real::Bounds;
use geom_core::sym::with_session;
use geom_core::{Interval, ParamSymbol, Real, Sym, SymBudget, Tol};

fn budget() -> SymBudget {
    crate::m10_10_r2_probes::shipped_budget()
}

fn band() -> Band {
    Band::linear(Tol::witness()).expect("the witness tolerance has a linear band")
}

/// A parameter over a box, minted in the installed session.
fn over(name: &str, lo: f64, hi: f64) -> Sym<Interval> {
    Sym::param_over(ParamSymbol::of(name), Interval::from_bounds(lo, hi), lo, hi)
}

/// How the tier answered the margin `build` makes inside a fresh
/// session: `theorem`, `numeric <sign>`, or the refusal (an
/// indeterminate or a domain violation).
fn how(build: impl FnOnce() -> Sym<Interval>) -> String {
    let (out, counts) = with_session(budget(), || {
        geom_core::k_stats::decide("atan2_interval", Margin::of(build()), band())
    });
    match out {
        Ok(Sign::Zero) if counts.symbolic_zero == 1 => "theorem".to_owned(),
        Ok(s) => format!("numeric {s:?}"),
        Err(e) => format!("refused {:?}", e.margin),
    }
}

/// **`atan2(0, N)` decides `Zero` at every width, for a straddling
/// OFFSET** — the parameter's box straddles zero while `N` stays
/// positive on it, which is the study's own shape (`r = nominal + δ`);
/// and where the box reaches `N = 0` (a radius through zero) the value
/// channel refuses through clause 1 first, so the fold is never asked.
#[test]
fn the_atan2_fold_decides_over_a_box_and_a_straddling_offset() {
    for (lo, hi) in [(0.1, 0.2), (0.5, 1.5), (1.0e-9, 1.0e3)] {
        assert_eq!(
            how(|| Sym::zero().atan2(over("x", lo, hi).sqrt())),
            "theorem",
            "sqrt over [{lo}, {hi}]"
        );
        assert_eq!(
            how(|| {
                let x = over("x", lo, hi);
                Sym::zero().atan2(x * x)
            }),
            "theorem",
            "even power over [{lo}, {hi}]"
        );
    }
    // The chart phase's own shape, `r = nominal + δ` with δ over a box
    // that STRADDLES zero — the numerator `(k + δ)²` is a perfect
    // square, not term-wise non-negative, and the fold still stands.
    for half in [1.0e-9, 1.0e-6, 1.0e-3] {
        assert_eq!(
            how(|| {
                let r = Sym::from_f64(1.25e-3) + over("delta", -half, half);
                let r2 = r * r;
                Sym::zero().atan2(r2 / r2.sqrt())
            }),
            "theorem",
            "r²/sqrt(r²) over δ ∈ ±{half:e}"
        );
    }
    // A box on which `r` itself passes through zero: the arc is
    // degenerate somewhere in it, `r²/sqrt(r²)` has no certified value
    // there, and clause 1 refuses before the form is asked.
    let answer = how(|| {
        let r = Sym::from_f64(1.25e-3) + over("delta", -1.0, 1.0);
        let r2 = r * r;
        Sym::zero().atan2(r2 / r2.sqrt())
    });
    assert!(
        answer.starts_with("refused"),
        "a radius through zero is clause 1's: {answer}"
    );
}

/// **A plain parameter in `N`'s place NEVER folds** — over a positive
/// box the numeric channel answers, over a straddling one it refuses,
/// and neither is a theorem.
#[test]
fn a_plain_parameter_never_folds() {
    assert_eq!(
        how(|| Sym::zero().atan2(over("x", 0.5, 1.5))),
        "numeric Zero"
    );
    let answer = how(|| Sym::zero().atan2(over("s", -1.0, 1.0)));
    assert!(
        answer.starts_with("refused") || answer.starts_with("numeric"),
        "a straddling plain parameter is the numeric channel's: {answer}"
    );
    assert_ne!(answer, "theorem");
}

/// **The `N = 0` box refuses through clause 1**: `sqrt(x)` over a box
/// straddling zero has no certified value, so the decision is refused
/// as a domain violation before the form is asked — and the same
/// expression's form WOULD fold, which is exactly why clause 1 comes
/// first.
#[test]
fn the_degenerate_box_refuses_through_clause_one() {
    let answer = how(|| Sym::zero().atan2(over("x", -1.0, 1.0).sqrt()));
    assert!(
        answer.starts_with("refused"),
        "sqrt over a straddling box is undefined and clause 1 refuses it: {answer}"
    );
}

/// **The half-π fold and the numeric channel fold at the SAME
/// constant** (R1 MIN-5): `trig::fold_at_half_pi` reads the form's
/// `π` indeterminate as the true π (`cos π = −1` exactly), which is
/// right only because the scalar's `pi()` node carries the interval
/// `[fl(π), next_up(fl(π))]` — the 1-ulp enclosure of the real π that
/// `interval.rs` pins for `Interval::pi()` — so every value the
/// numeric channel computes for that node encloses the constant the
/// form folds at. This row ties the two: the node's VALUE is exactly
/// that enclosure, and the fold on the interval scalar is a theorem
/// (the literal `fl(π)` is not the form's π and never folds:
/// `m10_10_r1_sym_probes::r1_the_half_pi_fold_takes_the_indeterminate_and_not_the_literal`).
#[test]
fn the_pi_fold_and_the_interval_pi_enclose_the_same_constant() {
    let pi = Interval::pi();
    assert_eq!(pi.lo(), core::f64::consts::PI);
    assert_eq!(pi.hi(), core::f64::consts::PI.next_up());
    let ((lo, hi), counts) = with_session(budget(), || {
        let node = Sym::<Interval>::pi();
        let bounds = (node.lo(), node.hi());
        let residual = node.sin_cos().1 + Sym::from_f64(1.0);
        let band = Band::linear(Tol::witness()).expect("a linear band");
        let sign = geom_core::k_stats::decide("pi_tie", Margin::of(residual), band);
        assert!(
            matches!(sign, Ok(Sign::Zero)),
            "cos π + 1 is the zero form: {sign:?}"
        );
        bounds
    });
    assert_eq!(
        (lo, hi),
        (pi.lo(), pi.hi()),
        "the node's value IS Interval::pi()"
    );
    assert_eq!(counts.symbolic_zero, 1, "{counts:?}");
    assert!(
        lo <= core::f64::consts::PI && core::f64::consts::PI.next_up() >= hi && lo < hi,
        "a 1-ulp enclosure of the real π"
    );
}
