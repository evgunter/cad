//! Review r2's probes of rule G's exact quotient (`SymRules::root_quotient`)
//! at the scalar door: each root is decided with the dial on and off, and
//! every zero the tier claims is checked against the value channel's
//! enclosure. Review evidence, not a pin.

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
        Err(e) => format!("refused {:?}", e.margin),
    }
}

fn decide(what: &str, rules: SymRules, build: &dyn Fn() -> Sym<Interval>) -> String {
    let ((out, value), counts) = with_session_rules(budget(), rules, || {
        let m = build();
        (
            geom_core::k_stats::decide("decide_4_review_r2_probes", Margin::of(m), band()),
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

/// (on, off) under the shipped set and its `without_root_quotient`.
fn both(what: &str, build: &dyn Fn() -> Sym<Interval>) -> (String, String) {
    let on = decide(&format!("{what} [on]"), SymRules::shipped(), build);
    let off = decide(
        &format!("{what} [off]"),
        SymRules::without_root_quotient(),
        build,
    );
    (on, off)
}

// ---------------------------------------------------------------- folds

/// `D | N` in one indeterminate: `sqrt((t+2)^3/(t+2)) = |t+2|`.
#[test]
fn r2_fold_one_indeterminate() {
    let (on, off) = both("sqrt((t+2)^3/(t+2)) - |t+2|", &|| {
        let t = over("t", 0.5, 1.0);
        let p = lit(2.0) + t;
        (p.powi(3) / p).sqrt() - p.abs()
    });
    assert_eq!(on, "theorem");
    assert_ne!(off, "theorem");
}

/// `D | N` in several indeterminates: `sqrt((x+y)^2 (1+xy)/(1+xy)) = |x+y|`.
#[test]
fn r2_fold_several_indeterminates() {
    let (on, off) = both("sqrt((x+y)^2 (1+xy)/(1+xy)) - |x+y|", &|| {
        let x = over("x", 0.5, 1.0);
        let y = over("y", -0.25, 0.25);
        let d = lit(1.0) + x * y;
        ((x + y).powi(2) * d / d).sqrt() - (x + y).abs()
    });
    assert_eq!(on, "theorem");
    assert_ne!(off, "theorem");
}

/// A non-square `Q` in two indeterminates, against the same `Q` spelled
/// without a denominator — one atom per value class.
#[test]
fn r2_fold_non_square_q_meets_its_own_spelling() {
    let (on, off) = both("sqrt((1+x^2+y) (2+x)/(2+x)) - sqrt(1+x^2+y)", &|| {
        let x = over("x", 0.5, 1.0);
        let y = over("y", 0.1, 0.2);
        let q = lit(1.0) + x.powi(2) + y;
        let d = lit(2.0) + x;
        (q * d / d).sqrt() - q.sqrt()
    });
    assert_eq!(on, "theorem");
    assert_ne!(off, "theorem");
}

/// `Q` with a real zero INSIDE the box: `sqrt(t^2 (2+t)/(2+t)) = |t|`,
/// `t` over `[-0.5, 0.5]`.
#[test]
fn r2_fold_q_with_a_zero_in_the_box() {
    let (on, off) = both("sqrt(t^2 (2+t)/(2+t)) - |t|, t in [-0.5,0.5]", &|| {
        let t = over("t", -0.5, 0.5);
        let d = lit(2.0) + t;
        (t.powi(2) * d / d).sqrt() - t.abs()
    });
    assert_eq!(on, "theorem");
    assert_ne!(off, "theorem");
    // and never the signed root, on a box straddling the zero
    let (on, _) = both("sqrt(t^2 (2+t)/(2+t)) - t, t in [-0.5,0.5]", &|| {
        let t = over("t", -0.5, 0.5);
        let d = lit(2.0) + t;
        (t.powi(2) * d / d).sqrt() - t
    });
    assert_ne!(on, "theorem");
}

/// A NEGATIVE `D` that no form shows negative: `D = t - 3` over
/// `t in [0, 1]`, `N = (t - 3) s^2`.
#[test]
fn r2_fold_negative_denominator() {
    let (on, off) = both("sqrt((t-3) s^2/(t-3)) - |s|", &|| {
        let t = over("t", 0.0, 1.0);
        let s = over("s", 0.5, 1.0);
        let d = t - lit(3.0);
        (d * s.powi(2) / d).sqrt() - s.abs()
    });
    assert_eq!(on, "theorem");
    assert_ne!(off, "theorem");
    // And with rule C on (`all`), the dial-off path reads D's sign
    // (sign_gated); the quotient takes it value-free (a theorem).
    let build = || {
        let t = over("t", 0.0, 1.0);
        let s = over("s", 0.5, 1.0);
        let d = t - lit(3.0);
        (d * s.powi(2) / d).sqrt() - s.abs()
    };
    let all_on = decide("negative D under all() [on]", SymRules::all(), &build);
    let all_off = decide(
        "negative D under all() [off]",
        SymRules {
            root_quotient: false,
            ..SymRules::all()
        },
        &build,
    );
    println!("  all(): on {all_on} / off {all_off}");
}

/// A manifestly NEGATIVE `D`: `D = -(1 + x^2)`.
#[test]
fn r2_fold_manifestly_negative_denominator() {
    let (on, _off) = both("sqrt(-(1+x^2) s^2 / -(1+x^2)) - |s|", &|| {
        let x = over("x", 0.5, 1.0);
        let s = over("s", -1.0, 1.0);
        let d = lit(0.0) - (lit(1.0) + x.powi(2));
        (d * s.powi(2) / d).sqrt() - s.abs()
    });
    assert_eq!(on, "theorem");
}

// ------------------------------------------------------------ no-folds

/// `N | D`: `sqrt((t+2)/(t+2)^3) - 1/|t+2|` — not tried, same either way.
#[test]
fn r2_no_fold_n_divides_d() {
    let (on, off) = both("sqrt((t+2)/(t+2)^3) - 1/|t+2|", &|| {
        let t = over("t", 0.5, 1.0);
        let p = lit(2.0) + t;
        (p / p.powi(3)).sqrt() - lit(1.0) / p.abs()
    });
    assert_eq!(on, off);
}

/// `D` a constant: the quotient is not asked; same either way.
#[test]
fn r2_no_fold_constant_denominator() {
    let (on, off) = both("sqrt(4 s^2 / 2) - sqrt(2)|s|", &|| {
        let s = over("s", -1.0, 1.0);
        (lit(4.0) * s.powi(2) / lit(2.0)).sqrt() - lit(2.0).sqrt() * s.abs()
    });
    assert_eq!(on, off);
}

/// A near miss: `N = (t+2)^3 + 2^-80 t`, `D = t + 2` — `D` divides `N`
/// except for one term with a tiny coefficient. Must never be a theorem.
#[test]
fn r2_no_fold_near_miss() {
    let tiny = 2.0_f64.powi(-80);
    let (on, off) = both("sqrt(((t+2)^3 + 2^-80 t)/(t+2)) - |t+2|", &|| {
        let t = over("t", 0.5, 1.0);
        let p = lit(2.0) + t;
        ((p.powi(3) + lit(tiny) * t) / p).sqrt() - p.abs()
    });
    assert_ne!(on, "theorem", "a near miss is not an identity");
    assert_ne!(off, "theorem");
}

/// A near miss whose remainder is a CONSTANT: `N = (t+2)^3 + 2^-80`,
/// `D = t + 2`. A division that dropped its remainder would return
/// `(t+2)^2`, whose root is exactly the other term: only the verified
/// product (or an exact loop) keeps this off the theorem list.
#[test]
fn r2_no_fold_near_miss_constant_remainder() {
    let tiny = 2.0_f64.powi(-80);
    let (on, off) = both("sqrt(((t+2)^3 + 2^-80)/(t+2)) - |t+2|", &|| {
        let t = over("t", 0.5, 1.0);
        let p = lit(2.0) + t;
        ((p.powi(3) + lit(tiny)) / p).sqrt() - p.abs()
    });
    assert_ne!(on, "theorem", "a near miss is not an identity");
    assert_ne!(off, "theorem");
}

/// The dead-arm/`D = 0` shape: `sqrt(x^2 (x - 1)/(x - 1)) - |x|` over a
/// box that CONTAINS `x = 1`, where the value channel divides by zero.
#[test]
fn r2_denominator_zero_inside_the_box() {
    let (on, off) = both("sqrt(x^2 (x-1)/(x-1)) - |x|, x in [0.5, 1.5]", &|| {
        let x = over("x", 0.5, 1.5);
        let d = x - lit(1.0);
        (x.powi(2) * d / d).sqrt() - x.abs()
    });
    println!("  D=0 inside: on {on} / off {off}");
}

// ------------------------------------------------------------- the loss

/// **What the split used to meet.** `sqrt(N/D)·sqrt(D) - sqrt(N)` with
/// `N = Q·D`: dial off, rule G splits `sqrt(N)/sqrt(D)` and the `sqrt(D)`
/// cancels; dial on, `sqrt(Q)`, `sqrt(D)` and `sqrt(N)` are three atoms.
#[test]
fn r2_the_split_met_a_spelling_the_quotient_does_not() {
    let (on, off) = both("sqrt(QD/D) sqrt(D) - sqrt(QD), D = 1+y^2", &|| {
        let x = over("x", 0.5, 1.0);
        let y = over("y", 0.5, 1.0);
        let q = lit(1.0) + x.powi(2);
        let d = lit(1.0) + y.powi(2);
        let n = q * d;
        (n / d).sqrt() * d.sqrt() - n.sqrt()
    });
    println!("  loss probe: on {on} / off {off}");
    assert_eq!(off, "theorem", "the split reaches it");
    assert_eq!(on, "theorem", "the quotient keeps what the split reached");
}

// --------------------------------------------------------- adversarial

/// `QUOTIENT_STEPS` exceeded: `Q = (x+y+z+w)^13` has 560 terms > 512.
/// With the cap the division declines; nothing may be claimed wrongly.
#[test]
fn r2_the_step_cap_declines() {
    let (on, off) = both("sqrt(p^14/p) - sqrt(p^13), p = x+y+z+w", &|| {
        let x = over("x", 0.5, 1.0);
        let y = over("y", 0.5, 1.0);
        let z = over("z", 0.5, 1.0);
        let w = over("w", 0.5, 1.0);
        let p = x + y + z + w;
        (p.powi(14) / p).sqrt() - p.powi(13).sqrt()
    });
    println!("  step cap: on {on} / off {off}");
    // Under the cap (Q = p^11, 364 terms) it folds.
    let (on_small, _) = both("sqrt(p^12/p) - sqrt(p^11), p = x+y+z+w", &|| {
        let x = over("x", 0.5, 1.0);
        let y = over("y", 0.5, 1.0);
        let z = over("z", 0.5, 1.0);
        let w = over("w", 0.5, 1.0);
        let p = x + y + z + w;
        (p.powi(12) / p).sqrt() - p.powi(11).sqrt()
    });
    println!("  under the cap: on {on_small}");
}
