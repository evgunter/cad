//! **R1's independent probes of M10-10's rule D** (trig of `atan` in
//! closed form, A1's `atan2` fold and the half-π fold), at the scalar.
//!
//! Derived from the rules' own stated contracts rather than from the
//! unit's rows: what the ARGUMENT READER accepts, whether the stated
//! soundness argument ("clause 1 decides the `N = 0` box first") is the
//! one that actually fires, and what the per-node reduction does at a
//! point where the atom it substitutes has no real value.
//!
//! Evidence-only rows print; the rest assert what they measured.

#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::interval::Interval;
use geom_core::predicate::{Band, Margin, Sign};
use geom_core::real::Real;
use geom_core::sym::with_session;
use geom_core::{ParamSymbol, Sym, SymBudget, Tol};

fn budget() -> SymBudget {
    SymBudget {
        max_terms: 4096,
        max_degree: 128,
    }
}

fn band() -> Band {
    Band::linear(Tol::witness()).expect("the witness tolerance has a linear band")
}

fn over(name: &str, lo: f64, hi: f64) -> Sym<Interval> {
    Sym::param_over(ParamSymbol::of(name), Interval::from_bounds(lo, hi), lo, hi)
}

/// How the tier answered a margin built inside a fresh session.
fn how(build: impl FnOnce() -> Sym<Interval>) -> String {
    let (out, counts) = with_session(budget(), || {
        geom_core::k_stats::decide("r1_m10_10", Margin::of(build()), band())
    });
    match out {
        Ok(Sign::Zero) if counts.symbolic_zero == 1 => "theorem".to_owned(),
        Ok(Sign::Zero) if counts.registered == 1 => "registered".to_owned(),
        Ok(Sign::Zero) if counts.sign_gated == 1 => "gated".to_owned(),
        Ok(s) => format!("numeric {s:?}"),
        Err(e) => format!("refused {:?}", e.margin),
    }
}

// ------------------------------------------------ the argument reader

/// **THE MULTIPLE READER WRAPS ON A HUGE DYADIC COEFFICIENT.**
/// `trig::read_argument` reads `q = k · 2^e` and, for `e ≥ 0`, forms
/// the multiple as `k.checked_shl(e)` — and `i128::checked_shl` only
/// refuses a shift of 128 or more; it WRAPS on overflow. So a
/// coefficient `(2¹²³ + 1) · 32`, whose true multiple is astronomically
/// past `MAX_MULTIPLE = 32`, is read as `k = 32` and the node folds to
/// the closed form of `cos(32 · atan X)`.
///
/// The residual below is `cos(q · atan x) − cos(32 · atan x)` for that
/// `q`. It is NOT zero as a function of `x`; if the tier calls it a
/// theorem, rule D folded at an argument that is not the `q · atan X`
/// it read.
#[test]
fn r1_the_multiple_reader_wraps_on_a_huge_dyadic_coefficient() {
    let answer = how(|| {
        let x = over("x", 0.4, 0.6);
        let phi = x.atan();
        // (2^123 + 1) · 32 — an odd 124-bit mantissa with exp2 = 5.
        let huge = (Sym::from_f64(2.0_f64.powi(123)) + Sym::from_f64(1.0)) * Sym::from_f64(32.0);
        (huge * phi).cos() - (Sym::from_f64(32.0) * phi).cos()
    });
    println!("R1: cos((2^123+1)·32·atan x) − cos(32·atan x) => {answer}");
    assert_ne!(
        answer, "theorem",
        "rule D folded a wrapped multiple as if it were 32"
    );
}

/// The same shape one bit smaller, to show the reader is exact where it
/// does not wrap: `2⁷ · atan x` is `k = 128 > MAX_MULTIPLE`, so nothing
/// folds and the answer is the numeric channel's.
#[test]
fn r1_a_multiple_past_the_cap_does_not_fold() {
    let answer = how(|| {
        let x = over("x", 0.4, 0.6);
        let phi = x.atan();
        (Sym::from_f64(128.0) * phi).cos() - (Sym::from_f64(128.0) * phi).cos() - Sym::from_f64(0.0)
    });
    println!("R1: 128·atan x self-difference => {answer}");
    assert_eq!(answer, "theorem", "one atom cancels with itself regardless");
}

/// **The half-π fold survives the same overflow** — `fold_at_half_pi`
/// shifts through `checked_shl` too, but it only reads `k mod 4`, which
/// a wrapping shift preserves, and every wrapped case is an even
/// integer multiple of `π` whose cosine really is `1`. Evidence that
/// the defect above is rule D's multiple reader alone.
#[test]
fn r1_the_half_pi_fold_is_safe_under_the_same_shift() {
    let answer = how(|| {
        let huge = (Sym::from_f64(2.0_f64.powi(123)) + Sym::from_f64(1.0)) * Sym::from_f64(32.0);
        (huge * Sym::<Interval>::pi()).cos() - Sym::from_f64(1.0)
    });
    println!("R1: cos((2^123+1)·32·π) − 1 => {answer} (an even multiple of π: correct)");
    assert_eq!(answer, "theorem");
}

// -------------------------------------------- rule D's closed forms

/// **The closed forms, checked against an INDEPENDENT spelling** —
/// `sin`/`cos` of `k · atan X` and of the halves, over a box, for `k ∈
/// {1, 2, 3, 4}`, spelled on the right through `S = sqrt(1 + X²)` and
/// the double/triple-angle expansions rather than through the module's
/// own recurrence.
#[test]
fn r1_the_closed_forms_hold_over_boxes_including_straddling_x() {
    for (lo, hi) in [(0.2, 0.4), (-0.9, -0.3), (-0.5, 0.5), (2.0, 3.0)] {
        let mk = |f: fn(Sym<Interval>) -> Sym<Interval>| {
            how(|| {
                let x = over("b", lo, hi);
                f(x)
            })
        };
        // cos(atan X) = 1/S, sin(atan X) = X/S.
        assert_eq!(
            mk(|x| {
                let s = (Sym::from_f64(1.0) + x * x).sqrt();
                x.atan().cos() - Sym::from_f64(1.0) / s
            }),
            "theorem",
            "cos(atan X) over [{lo}, {hi}]"
        );
        assert_eq!(
            mk(|x| {
                let s = (Sym::from_f64(1.0) + x * x).sqrt();
                x.atan().sin() - x / s
            }),
            "theorem",
            "sin(atan X) over [{lo}, {hi}]"
        );
        // cos(2φ) = 2cos²φ − 1, sin(3φ) = 3sinφ − 4sin³φ,
        // cos(4φ) = 8cos⁴φ − 8cos²φ + 1 — spelled through the ATOMS,
        // not through the fold's own recurrence.
        assert_eq!(
            mk(|x| {
                let two = Sym::from_f64(2.0);
                let c = x.atan().cos();
                (two * x.atan()).cos() - (two * c * c - Sym::from_f64(1.0))
            }),
            "theorem",
            "cos(2·atan X) over [{lo}, {hi}]"
        );
        assert_eq!(
            mk(|x| {
                let s = x.atan().sin();
                (Sym::from_f64(3.0) * x.atan()).sin()
                    - (Sym::from_f64(3.0) * s - Sym::from_f64(4.0) * s * s * s)
            }),
            "theorem",
            "sin(3·atan X) over [{lo}, {hi}]"
        );
        assert_eq!(
            mk(|x| {
                let c = x.atan().cos();
                let c2 = c * c;
                (Sym::from_f64(4.0) * x.atan()).cos()
                    - (Sym::from_f64(8.0) * c2 * c2 - Sym::from_f64(8.0) * c2
                        + Sym::from_f64(1.0))
            }),
            "theorem",
            "cos(4·atan X) over [{lo}, {hi}]"
        );
        // The halves, against the double-angle identity read the other
        // way: cos(φ) = 2·cos²(φ/2) − 1 and sin(φ) = 2 sin(φ/2)cos(φ/2).
        assert_eq!(
            mk(|x| {
                let h = Sym::from_f64(0.5) * x.atan();
                x.atan().cos() - (Sym::from_f64(2.0) * h.cos() * h.cos() - Sym::from_f64(1.0))
            }),
            "theorem",
            "the half's double-angle over [{lo}, {hi}]"
        );
        assert_eq!(
            mk(|x| {
                let h = Sym::from_f64(0.5) * x.atan();
                x.atan().sin() - Sym::from_f64(2.0) * h.sin() * h.cos()
            }),
            "theorem",
            "the half's sine over [{lo}, {hi}]"
        );
        // A quarter, two halvings deep.
        assert_eq!(
            mk(|x| {
                let h = Sym::from_f64(0.25) * x.atan();
                (Sym::from_f64(0.5) * x.atan()).cos()
                    - (Sym::from_f64(2.0) * h.cos() * h.cos() - Sym::from_f64(1.0))
            }),
            "theorem",
            "the quarter's double-angle over [{lo}, {hi}]"
        );
    }
}

/// **A WRONG multiple is NOT a theorem** — the negative control for the
/// row above: `cos(2·atan X) − cos(3·atan X)` must never decide `Zero`.
#[test]
fn r1_two_different_multiples_never_decide_zero() {
    for (a, b) in [(1.0, 2.0), (2.0, 3.0), (0.5, 1.5), (0.25, 0.75)] {
        let answer = how(|| {
            let x = over("b", 0.3, 0.5);
            (Sym::from_f64(a) * x.atan()).cos() - (Sym::from_f64(b) * x.atan()).cos()
        });
        assert_ne!(answer, "theorem", "cos({a}·atan X) vs cos({b}·atan X)");
    }
}

/// **Pythagoras closes the ring behind rule D**: `sin² + cos² − 1` at a
/// rule-D argument is a theorem through rule A (the `sqrt` atom's
/// square), and at an argument rule D does NOT read it is a theorem
/// through rule B (the `sin`/`cos` atom pair). Both are checked because
/// the two paths mint different indeterminates for the same angle, and
/// a form that mixed them would cancel neither.
#[test]
fn r1_pythagoras_closes_at_a_rule_d_argument_and_at_an_opaque_one() {
    let folded = how(|| {
        let x = over("b", 0.3, 0.5);
        let t = Sym::from_f64(1.5) * x.atan();
        t.sin() * t.sin() + t.cos() * t.cos() - Sym::from_f64(1.0)
    });
    let opaque = how(|| {
        let x = over("b", 0.3, 0.5);
        let t = x.atan() + Sym::from_f64(1.0);
        t.sin() * t.sin() + t.cos() * t.cos() - Sym::from_f64(1.0)
    });
    println!("R1: sin²+cos²−1 at 1.5·atan X => {folded}; at atan X + 1 => {opaque}");
    assert_eq!(folded, "theorem");
    assert_eq!(opaque, "theorem");
}

// ------------------------------------------- A1: the `atan2` fold

/// **THE `N = 0` POINT IS CLAUSE 1'S, ALSO FOR THE EVEN-POWER SHAPE** —
/// the case `m10_10_atan2_interval` never exercises, since every `X²`
/// row there has `lo > 0`. `trig.rs`'s argument for the `atan2` fold is
/// that where `N = 0` — the one place `atan2(0, N)` is not `0` by a
/// limit — "clause 1, the numeric channel's own domain answer, decides
/// first", and R1 checked it on the shape whose value at `N = 0` is a
/// perfectly finite IEEE `+0`: `atan2(0, X²)` over a box that reaches
/// `X = 0`. The value channel refuses it (`Invalid`) at `Interval`, so
/// the stated argument holds here too.
#[test]
fn r1_the_even_power_at_zero_is_decided_by_the_fold_not_by_clause_one() {
    let straddling = how(|| {
        let x = over("x", -1.0, 1.0);
        Sym::zero().atan2(x * x)
    });
    let at_zero = how(|| {
        let x = over("x", 0.0, 1.0);
        Sym::zero().atan2(x * x)
    });
    println!(
        "R1: atan2(0, X²) over [-1,1] => {straddling}; over [0,1] => {at_zero} \
         (the module claims clause 1 decides the N = 0 box first)"
    );
    assert!(
        straddling.starts_with("refused"),
        "clause 1 is claimed to decide the N = 0 box: {straddling}"
    );
    assert!(
        at_zero.starts_with("refused"),
        "clause 1 is claimed to decide the N = 0 box: {at_zero}"
    );
}

/// **The never-fold set, re-derived.** A negative coefficient, an odd
/// power of a parameter, a `Y` that is zero only numerically, and a
/// `sqrt` atom multiplied by an odd power of a parameter must all keep
/// the atom.
#[test]
fn r1_the_atan2_fold_refuses_everything_it_says_it_refuses() {
    // A term-wise NEGATIVE coefficient: N = −X², non-positive.
    let neg = how(|| {
        let x = over("x", 0.5, 1.5);
        Sym::zero().atan2(Sym::from_f64(0.0) - x * x)
    });
    assert_ne!(neg, "theorem", "atan2(0, −X²) must not fold");
    // An odd power beside a `sqrt` atom: N = sqrt(u)·v, sign unknown.
    let mixed = how(|| {
        let u = over("u", 0.5, 1.5);
        let v = over("v", -1.0, 1.0);
        Sym::zero().atan2(u.sqrt() * v)
    });
    assert_ne!(mixed, "theorem", "atan2(0, sqrt(u)·v) must not fold");
    // `Y` a NUMERICAL coincidence: two distinct parameters that happen
    // to be equal at the nominal. `Y`'s form is not the zero form.
    let coincidence = how(|| {
        let a = Sym::param_over(
            ParamSymbol::of("a"),
            Interval::from_bounds(1.0, 1.0),
            1.0,
            1.0,
        );
        let b = Sym::param_over(
            ParamSymbol::of("b"),
            Interval::from_bounds(1.0, 1.0),
            1.0,
            1.0,
        );
        let u = over("u", 0.5, 1.5);
        (a - b).atan2(u.sqrt())
    });
    assert_ne!(
        coincidence, "theorem",
        "atan2(a − b, N) with a, b distinct parameters must not fold"
    );
    println!("R1: atan2 never-fold set: −X² => {neg}; sqrt(u)·v => {mixed}; a−b => {coincidence}");
}

// ------------------------------------------ A1: the half-π fold (D14)

/// **The half-π fold takes `k·π/2` and nothing else** — including
/// nothing that is only NUMERICALLY a half-multiple of π: a `π` spelled
/// as the `f64` literal is a `Lit` node, not the form's own `π`
/// indeterminate, and must stay an atom.
#[test]
fn r1_the_half_pi_fold_takes_the_indeterminate_and_not_the_literal() {
    let exact = how(|| Sym::<Interval>::pi().cos() + Sym::from_f64(1.0));
    let three_halves = how(|| (Sym::from_f64(1.5) * Sym::<Interval>::pi()).sin() + Sym::from_f64(1.0));
    let third = how(|| (Sym::<Interval>::pi() / Sym::from_f64(3.0)).cos() - Sym::from_f64(0.5));
    let literal =
        how(|| Sym::<Interval>::from_f64(core::f64::consts::PI).cos() + Sym::from_f64(1.0));
    println!(
        "R1: cos π + 1 => {exact}; sin(3π/2) + 1 => {three_halves}; \
         cos(π/3) − ½ => {third}; cos(LIT π) + 1 => {literal}"
    );
    assert_eq!(exact, "theorem", "cos π = −1");
    assert_eq!(three_halves, "theorem", "sin(3π/2) = −1");
    assert_ne!(third, "theorem", "cos(π/3) is not a half-multiple");
    assert_ne!(
        literal, "theorem",
        "a numerically-close literal π is not the form's π"
    );
}

/// **`π` beside anything else stays an atom** — `cos(π·x)` at a
/// parameter that is 1 at the nominal must not fold.
#[test]
fn r1_pi_beside_a_parameter_never_folds() {
    let answer = how(|| {
        let x = Sym::param_over(
            ParamSymbol::of("x"),
            Interval::from_bounds(1.0, 1.0),
            1.0,
            1.0,
        );
        (Sym::<Interval>::pi() * x).cos() + Sym::from_f64(1.0)
    });
    assert_ne!(answer, "theorem", "cos(π·x) at a numerically-unit x");
    println!("R1: cos(π·x) + 1 at x ≡ 1 => {answer}");
}

// ------------------------- D4/D5: the zero normalization and rule A

/// **The zero normalization at a denominator that vanishes on the box.**
/// `0/d + x → x` is sound as a rational-function identity, and its
/// soundness argument is "a point where `d` vanishes is one clause 1 has
/// already refused". Checked directly: `d` straddles zero, so `0/d` has
/// no value on part of the box.
#[test]
fn r1_the_zero_normalization_over_a_vanishing_denominator() {
    let straddling = how(|| {
        let d = over("d", -1.0, 1.0);
        let x = over("x", 1.0, 2.0);
        let z = (x - x) / d;
        (z + x) - x
    });
    let positive = how(|| {
        let d = over("d", 0.5, 1.5);
        let x = over("x", 1.0, 2.0);
        let z = (x - x) / d;
        (z + x) - x
    });
    println!("R1: (0/d + x) − x, d straddling => {straddling}; d positive => {positive}");
    assert_eq!(positive, "theorem");
    assert!(
        straddling.starts_with("refused"),
        "a vanishing denominator is clause 1's: {straddling}"
    );
}

/// **Rule A per node over an odd power and inside a denominator.**
/// `s³ − X·s` and `X/s² − 1` for `s = sqrt(X)` are theorems where `X >
/// 0`; over a box where `X` straddles zero the atom has no real value
/// and clause 1 must refuse rather than the reduction claiming one.
#[test]
fn r1_rule_a_over_odd_powers_and_in_the_denominator() {
    let odd_pos = how(|| {
        let x = over("x", 0.5, 1.5);
        let s = x.sqrt();
        s * s * s - x * s
    });
    let den_pos = how(|| {
        let x = over("x", 0.5, 1.5);
        let s = x.sqrt();
        x / (s * s) - Sym::from_f64(1.0)
    });
    let odd_straddling = how(|| {
        let x = over("x", -1.0, 1.5);
        let s = x.sqrt();
        s * s * s - x * s
    });
    let den_at_zero = how(|| {
        let x = over("x", 0.0, 1.5);
        let s = x.sqrt();
        x / (s * s) - Sym::from_f64(1.0)
    });
    println!(
        "R1: s³ − X·s => {odd_pos}; X/s² − 1 => {den_pos}; \
         s³ − X·s straddling => {odd_straddling}; X/s² − 1 at X ∋ 0 => {den_at_zero}"
    );
    assert_eq!(odd_pos, "theorem");
    assert_eq!(den_pos, "theorem");
    assert!(
        odd_straddling.starts_with("refused"),
        "sqrt of a straddling box: {odd_straddling}"
    );
    assert!(
        den_at_zero.starts_with("refused"),
        "1/sqrt(0): {den_at_zero}"
    );
}
