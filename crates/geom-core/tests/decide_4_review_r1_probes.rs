//! Review probes (DECIDE-4, reviewer r1) for rule G's exact quotient
//! (`SymRules::root_quotient`), driven through `Sym<Interval>` at the
//! scalar door with the dial on and off. Every row prints the tier's
//! answer beside the enclosure and asserts soundness (a zero the tier
//! claims is one the enclosure admits); the `expect` strings name what
//! each row predicts.

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

fn label(out: Result<Sign, geom_core::predicate::Indeterminate>, c: SymCounts) -> String {
    match out {
        Ok(Sign::Zero) if c.symbolic_zero > 0 => "theorem".to_owned(),
        Ok(Sign::Zero) if c.registered > 0 => "registered".to_owned(),
        Ok(Sign::Zero) if c.sign_gated > 0 => "sign_gated".to_owned(),
        Ok(s) => format!("numeric {s:?}"),
        Err(_) => "refused".to_owned(),
    }
}

fn decide(what: &str, rules: SymRules, build: impl FnOnce() -> Sym<Interval>) -> String {
    let ((out, value), counts) = with_session_rules(budget(), rules, || {
        let m = build();
        (
            geom_core::k_stats::decide("decide_4_review_r1_probes", Margin::of(m), band()),
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

/// `(on, off)` — shipped, and shipped without the exact quotient.
fn both(what: &str, build: impl Fn() -> Sym<Interval>) -> (String, String) {
    let on = decide(&format!("{what} [on]"), SymRules::shipped(), &build);
    let off = decide(
        &format!("{what} [off]"),
        SymRules::without_root_quotient(),
        &build,
    );
    (on, off)
}

// ---- quotients that should fold -------------------------------------

#[test]
fn r1_fold_univariate_non_square_q() {
    let (on, off) = both("sqrt((2+t)(1+t)/(1+t)) - sqrt(2+t)", || {
        let t = over("t", 0.5, 1.0);
        let d = lit(1.0) + t;
        ((lit(2.0) + t) * d / d).sqrt() - (lit(2.0) + t).sqrt()
    });
    assert_eq!(on, "theorem");
    assert_ne!(off, "theorem");
}

#[test]
fn r1_fold_multivariate() {
    // D = x - y + 3, Q = x^2 + x*y + 3 (not a square).
    let (on, off) = both("sqrt(Q*(x-y+3)/(x-y+3)) - sqrt(Q), x,y", || {
        let x = over("x", 0.5, 1.0);
        let y = over("y", 0.25, 0.75);
        let d = x - y + lit(3.0);
        let q = x * x + x * y + lit(3.0);
        (q * d / d).sqrt() - q.sqrt()
    });
    assert_eq!(on, "theorem");
    assert_ne!(off, "theorem");
}

#[test]
fn r1_fold_multivariate_perfect_square() {
    let (on, off) = both("sqrt((x+y)^2 (1+x y)/(1+x y)) - |x+y|", || {
        let x = over("x", -1.0, 1.0);
        let y = over("y", -0.5, 0.25);
        let d = lit(1.0) + x * y + lit(2.0);
        let r = x + y;
        (r.powi(2) * d / d).sqrt() - r.abs()
    });
    assert_eq!(on, "theorem");
    assert_ne!(off, "theorem");
}

#[test]
fn r1_fold_q_with_a_real_zero_in_the_box() {
    let (on, _off) = both("sqrt(t^2 (2+t)/(2+t)) - |t|, t in [-0.5, 0.5]", || {
        let t = over("t", -0.5, 0.5);
        let d = lit(2.0) + t;
        (t.powi(2) * d / d).sqrt() - t.abs()
    });
    assert_eq!(on, "theorem");
}

#[test]
fn r1_fold_negative_denominator_not_manifest() {
    // D = t - 2 < 0 on [0, 1], not manifestly signed; N = (3+t)(t-2).
    let (on, off) = both("sqrt((3+t)(t-2)/(t-2)) - sqrt(3+t), D<0", || {
        let t = over("t", 0.0, 1.0);
        let d = t - lit(2.0);
        ((lit(3.0) + t) * d / d).sqrt() - (lit(3.0) + t).sqrt()
    });
    assert_eq!(on, "theorem");
    assert_ne!(off, "theorem");
}

// ---- quotients that must not fold -----------------------------------

#[test]
fn r1_no_fold_n_divides_d() {
    let (on, off) = both("sqrt((1+t)/((1+t)(2+t))) - sqrt(1/(2+t))", || {
        let t = over("t", 0.5, 1.0);
        let n = lit(1.0) + t;
        (n / (n * (lit(2.0) + t))).sqrt() - (lit(1.0) / (lit(2.0) + t)).sqrt()
    });
    assert_eq!(on, off, "N | D is not the rewrite's");
}

#[test]
fn r1_no_fold_constant_d() {
    let (on, off) = both("sqrt((1+t)/3) - sqrt(1+t)/sqrt(3)", || {
        let t = over("t", 0.5, 1.0);
        ((lit(1.0) + t) / lit(3.0)).sqrt() - (lit(1.0) + t).sqrt() / lit(3.0).sqrt()
    });
    assert_eq!(on, off, "a constant D is not the rewrite's");
}

#[test]
fn r1_no_fold_near_miss() {
    // N = (2+t)(1+t) + 1e-30 t^3: D divides N except for one tiny term.
    let (on, off) = both("near miss sqrt(((2+t)(1+t)+1e-30 t^3)/(1+t)) - sqrt(2+t)", || {
        let t = over("t", 0.5, 1.0);
        let d = lit(1.0) + t;
        (((lit(2.0) + t) * d + lit(1e-30) * t * t * t) / d).sqrt() - (lit(2.0) + t).sqrt()
    });
    assert_ne!(on, "theorem", "a near miss is not an exact quotient");
    assert_eq!(on, off);
}

// ---- adversarial ----------------------------------------------------

/// The step cap: `(1+x+y+z)^k · (x − y) / (x − y)`. The quotient has
/// C(k+3, 3) terms: 455 at k = 12 (under `QUOTIENT_STEPS = 512`), 680
/// at k = 14 (over). Both products are inside the drive's own budget
/// (4096 pairs), so the cap, not the budget, is what declines at 14.
fn capped(k: u32) -> Sym<Interval> {
    let x = over("x", 0.5, 1.0);
    let y = over("y", 0.125, 0.25);
    let z = over("z", 0.25, 0.5);
    let base = lit(1.0) + x + y + z;
    let mut q = base;
    for _ in 1..k {
        q = q * base;
    }
    let d = x - y;
    (q * d / d).sqrt() - q.sqrt()
}

#[test]
fn r1_step_cap_under() {
    let (on, off) = both("(1+x+y+z)^12 (x-y)/(x-y)", || capped(12));
    assert_eq!(on, "theorem");
    assert_ne!(off, "theorem");
}

#[test]
fn r1_step_cap_over_declines() {
    let (on, off) = both("(1+x+y+z)^14 (x-y)/(x-y)", || capped(14));
    assert_ne!(on, "theorem", "past the step cap the rewrite declines");
    assert_eq!(on, off);
}

/// A division that runs into the ring: `D = t + 3^80`, `N = t^4 + 1`
/// (not divisible); the remainder's coefficient is 3^240 at step 3, past
/// the 256-bit ring. It must decline, not panic, and answer as off.
#[test]
fn r1_ring_overflow_partway_declines() {
    let (on, off) = both("sqrt((t^4+1)/(t+3^80)) - itself spelled apart", || {
        let t = over("t", 0.5, 1.0);
        let big = lit(3.0).powi(80);
        let d = t + big;
        let n = t.powi(4) + lit(1.0);
        (n / d).sqrt() - n.sqrt() / d.sqrt()
    });
    assert_eq!(on, off);
}

/// grlex's tie-break over two indeterminates: `D = y^2 - x^2 + 4`,
/// `Q = x y + x + 3`, degree ties everywhere between x-terms and
/// y-terms. With the lex part wrong the leading terms no longer cancel.
#[test]
fn r1_grlex_tie_break_multivariate() {
    let (on, off) = both("sqrt(Q (y^2 - x^2 + 4)/(y^2 - x^2 + 4)) - sqrt(Q), ties", || {
        let x = over("x", 0.5, 1.0);
        let y = over("y", 0.25, 0.75);
        let d = y * y - x * x + lit(4.0);
        let q = x * y + x * x + y * y * y + lit(3.0);
        (q * d / d).sqrt() - q.sqrt()
    });
    assert_eq!(on, "theorem");
    assert_ne!(off, "theorem");
}

// ---- claim 3: one atom per value class ------------------------------

/// The value class of `sqrt(N/D)` spelled the way rule G's split spells
/// it, `sqrt(N)/sqrt(D)`, with `D = 1 + x²` manifestly positive. With the
/// dial off the split mints exactly those two atoms and the residual is
/// a theorem; with the dial on the root is re-keyed to `sqrt(Q)` and
/// the meeting is lost.
#[test]
fn r1_the_split_spelling_loses_its_meeting() {
    let (on, off) = both("sqrt(N/D) - sqrt(N)/sqrt(D), D = 1+x^2, Q = 2+x", || {
        let x = over("x", 1.0, 2.0);
        let d = lit(1.0) + x * x;
        let n = (lit(2.0) + x) * d;
        (n / d).sqrt() - n.sqrt() / d.sqrt()
    });
    println!("  split spelling: on {on}, off {off}");
    assert_eq!(off, "theorem", "the split spelling meets with the dial off");
    assert_eq!(on, "theorem", "the split spelling meets with the dial on");
}

/// A sign-carrying `R` that is a PRODUCT of sign-carrying factors, over
/// a box where the product is positive: `sqrt(t^2 (t-0.25)^2 D / D)`.
#[test]
fn r1_sign_carrying_product_stays_a_magnitude() {
    for (lo, hi) in [(0.5, 1.0), (-1.0, -0.5), (-0.5, 0.5)] {
        let signed = decide(
            &format!("sqrt(R^2 D/D) - R, R = t(t-0.25), t in [{lo},{hi}]"),
            SymRules::shipped(),
            || {
                let t = over("t", lo, hi);
                let r = t * (t - lit(0.25));
                let d = lit(3.0) + t;
                (r.powi(2) * d / d).sqrt() - r
            },
        );
        assert_ne!(signed, "theorem");
        let mag = decide(
            &format!("sqrt(R^2 D/D) - |R|, t in [{lo},{hi}]"),
            SymRules::shipped(),
            || {
                let t = over("t", lo, hi);
                let r = t * (t - lit(0.25));
                let d = lit(3.0) + t;
                (r.powi(2) * d / d).sqrt() - r.abs()
            },
        );
        assert_eq!(mag, "theorem");
    }
}

// ---- claims 2 and 7: labels and the dial pairing ---------------------

/// A GATED argument: under `SymRules::all()` rule C folds `|t| = t`
/// over `t > 0` (a read), so the root's argument is gated; the quotient
/// must keep the label and the zero is `sign_gated`, never a theorem.
#[test]
fn r1_gated_argument_keeps_its_label() {
    let l = decide("sqrt(|t|(2+t)(1+t)/(1+t)) - sqrt(t(2+t)), all()", SymRules::all(), || {
        let t = over("t", 0.5, 1.0);
        let d = lit(1.0) + t;
        (t.abs() * (lit(2.0) + t) * d / d).sqrt() - (t * (lit(2.0) + t)).sqrt()
    });
    assert_ne!(l, "theorem", "a gated argument's zero is not a theorem");
}

/// `canonical_root` off with `root_quotient` on: the step must not run.
#[test]
fn r1_quotient_without_rule_g_does_not_run() {
    let rules = SymRules {
        root_quotient: true,
        ..SymRules::without_canonical_root()
    };
    let build = || {
        let t = over("t", 0.5, 1.0);
        let d = lit(1.0) + t;
        ((lit(2.0) + t) * d / d).sqrt() - (lit(2.0) + t).sqrt()
    };
    let with = decide("rule G off, quotient on", rules, build);
    let without = decide(
        "rule G off, quotient off",
        SymRules::without_canonical_root(),
        build,
    );
    assert_eq!(with, without);
    assert_ne!(with, "theorem");
}
