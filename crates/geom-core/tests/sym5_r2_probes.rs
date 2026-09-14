//! **R2's independent probes of SYM-5's rule E** (PR #2589, frozen head
//! `480704dbb`): the quotient's common factor, attacked through the
//! PUBLIC scalar at `Sym<Interval>` over boxes — the box where the
//! shared factor's indeterminate vanishes, the nested-atom shapes only
//! the early walk reaches, a non-unit vector, and the coefficient
//! WIDTH the scale step moves. Every row that expects a theorem also
//! reads the residual's `f64` value at the point.
#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::linalg::Vec3;
use geom_core::predicate::{Band, Margin, Sign};
use geom_core::sym::with_session_rules;
use geom_core::{Interval, ParamSymbol, Real, Sym, SymBudget, SymRules, Tol};

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

/// How the tier answered over the box under `rules`, and the counts.
fn how(rules: SymRules, build: impl FnOnce() -> Sym<Interval>) -> String {
    let (out, counts) = with_session_rules(budget(), rules, || {
        geom_core::k_stats::decide("sym5_r2_probe", Margin::of(build()), band())
    });
    match out {
        Ok(Sign::Zero) if counts.symbolic_zero == 1 => "theorem".to_owned(),
        Ok(Sign::Zero) if counts.registered == 1 => "registered".to_owned(),
        Ok(Sign::Zero) if counts.sign_gated == 1 => "sign_gated".to_owned(),
        Ok(s) => format!("numeric {s:?} (frozen {})", counts.frozen),
        Err(e) => format!("refused {:?}", e.margin),
    }
}

fn on() -> SymRules {
    SymRules::shipped()
}
fn off() -> SymRules {
    SymRules::without_rule_e()
}

/// `x·a/(x·b)` — the shared monomial is a PARAMETER, and the box
/// contains its zero. Through a `sqrt` so the plain form's own
/// field-of-fractions cancellation is not what answers.
#[test]
fn r2_a_shared_parameter_factor_that_vanishes_in_the_box_is_refused_not_folded() {
    let resid = |lo: f64, hi: f64| {
        move || {
            let x = over("x", lo, hi);
            let a = over("a", 2.0, 3.0);
            let b = over("b", 4.0, 5.0);
            ((x * a) / (x * b)).sqrt() - (a / b).sqrt()
        }
    };
    let straddle_on = how(on(), resid(-1.0, 1.0));
    let straddle_off = how(off(), resid(-1.0, 1.0));
    let away_on = how(on(), resid(0.5, 1.0));
    let away_off = how(off(), resid(0.5, 1.0));
    println!("x∋0 on {straddle_on} | off {straddle_off}; x>0 on {away_on} | off {away_off}");
    assert!(
        straddle_on.starts_with("refused"),
        "the box where x·b vanishes must be refused by clause 1, got {straddle_on}"
    );
    assert!(
        !straddle_on.starts_with("theorem") && !straddle_off.starts_with("theorem"),
        "no theorem where the function is undefined"
    );
    assert_eq!(away_on, "theorem", "away from zero the rule keys one atom");
}

/// The same with the factor an ATOM whose argument vanishes at a point:
/// `sqrt(t²)` at `t = 0`, and through `powi(-1)` rather than a `Div`,
/// in case the negative power's value-channel decoration differs.
#[test]
fn r2_a_shared_atom_factor_that_vanishes_at_a_point_is_refused_through_div_and_powi() {
    let via_div = |lo: f64, hi: f64| {
        move || {
            let t = over("t", lo, hi);
            let a = over("a", 2.0, 3.0);
            let b = over("b", 4.0, 5.0);
            let s = (t * t).sqrt();
            ((s * a) / (s * b)).sqrt() - (a / b).sqrt()
        }
    };
    let via_powi = |lo: f64, hi: f64| {
        move || {
            let t = over("t", lo, hi);
            let a = over("a", 2.0, 3.0);
            let b = over("b", 4.0, 5.0);
            let s = (t * t).sqrt();
            ((s * a) * (s * b).powi(-1)).sqrt() - (a / b).sqrt()
        }
    };
    for (name, f) in [
        ("div", how(on(), via_div(-1.0, 1.0))),
        ("powi", how(on(), via_powi(-1.0, 1.0))),
    ] {
        println!("t∋0 {name}: {f}");
        assert!(
            f.starts_with("refused"),
            "{name}: sqrt(t²) vanishes at t = 0, clause 1 refuses; got {f}"
        );
    }
    // The endpoint case: `t ∈ [0, 1]` — the divisor's enclosure is
    // `[0, …]`, zero at one endpoint only.
    for (name, f) in [
        ("div", how(on(), via_div(0.0, 1.0))),
        ("powi", how(on(), via_powi(0.0, 1.0))),
    ] {
        println!("t∈[0,1] {name}: {f}");
        assert!(
            f.starts_with("refused"),
            "{name}: a divisor enclosure touching zero is Trv; got {f}"
        );
    }
    println!(
        "t>0 div on {} off {}",
        how(on(), via_div(0.5, 1.0)),
        how(off(), via_div(0.5, 1.0))
    );
}

/// `P/P` where `P` vanishes inside the box: `‖v̂‖ − 1` for `v = (t, 0, 0)`
/// over `t ∋ 0` — the normalisation divides by `|t|`.
#[test]
fn r2_the_unit_norm_theorem_is_refused_where_the_vector_can_vanish() {
    let unit = |lo: f64, hi: f64| {
        move || {
            let v = Vec3::new(over("t", lo, hi), Sym::from_f64(0.0), Sym::from_f64(0.0));
            v.normalize().norm() - Sym::from_f64(1.0)
        }
    };
    let z = how(on(), unit(-1.0, 1.0));
    let e = how(on(), unit(0.0, 1.0));
    let p = how(on(), unit(0.5, 1.0));
    println!("t∋0 {z}; t∈[0,1] {e}; t>0 {p}");
    assert!(z.starts_with("refused"), "got {z}");
    assert!(e.starts_with("refused"), "got {e}");
    assert_eq!(p, "theorem");
    // And a general three-component vector over a box, with the value
    // check a label cannot make.
    let ((out, value), counts) = with_session_rules(budget(), on(), || {
        let v = Vec3::new(
            over("x", 0.5, 1.5),
            over("y", -2.0, 2.0),
            over("z", 3.0, 4.0),
        );
        let m = v.normalize().norm() - Sym::from_f64(1.0);
        (
            geom_core::k_stats::decide("sym5_r2_probe", Margin::of(m), band()),
            m.value,
        )
    });
    println!("3-vector: {out:?} {counts:?} value {value:?}");
    assert!(matches!(out, Ok(Sign::Zero)) && counts.symbolic_zero == 1);
}

/// A genuinely non-unit vector over a box never acquires a unit norm,
/// and its true norm IS reached.
#[test]
fn r2_a_non_unit_vector_over_a_box_keeps_its_length() {
    let scaled = |k: f64, target: f64| {
        move || {
            let v = Vec3::new(
                over("x", 0.5, 1.5),
                over("y", -2.0, 2.0),
                over("z", 3.0, 4.0),
            );
            (v.normalize() * Sym::from_f64(k)).norm() - Sym::from_f64(target)
        }
    };
    for k in [3.0, 0.5, 1.0000001] {
        let wrong = how(on(), scaled(k, 1.0));
        println!("‖{k}·v̂‖ − 1: {wrong}");
        assert!(
            !wrong.starts_with("theorem") && !wrong.starts_with("sign_gated"),
            "{k}: {wrong}"
        );
    }
    assert_eq!(how(on(), scaled(3.0, 3.0)), "theorem");
    // A vector that is unit only at ONE point of the box: `(t, sqrt(1 − t²)·…)`
    // is not spellable without a sqrt of a straddling form; use
    // `(c, s)` with `c² + s² = 1` at the nominal only: the box breaks it.
    let near = how(on(), || {
        let c = over("c", 0.6 - 1e-3, 0.6 + 1e-3);
        let s = over("s", 0.8 - 1e-3, 0.8 + 1e-3);
        Vec3::new(c, s, Sym::from_f64(0.0)).norm() - Sym::from_f64(1.0)
    });
    println!("nearly-unit (c, s): {near}");
    assert!(
        !near.starts_with("theorem"),
        "a nearly-unit vector is not unit: {near}"
    );
}

/// **The coefficient width the scale step moves.** Rule E rescales both
/// halves by `1/|s|` with `s` the denominator's smallest coefficient, so
/// every numerator coefficient gains `s`'s bits in ITS denominator. The
/// ring refuses a coefficient past `COEFF_BITS` = 256, and a refusal is
/// a freeze. `f = (x + 1)/(x + 0.1)` (`0.1` an odd 52-bit mantissa) times
/// `1/q` with `q = 3^132` (210 bits, four exact literals): with the rule
/// the product's coefficients need 262 bits and freeze; without it they
/// need 210 and fit. Nested under a `sqrt` so only the early walk can
/// reach the identity.
#[test]
fn r2_evidence_the_scale_step_can_push_a_coefficient_over_the_ring() {
    let q3_33 = 5_559_060_566_555_523.0_f64; // 3^33, exact in f64
    let resid = || {
        let x = over("x", 1.0, 2.0);
        let one = Sym::from_f64(1.0);
        let f = (x + one) / (x + Sym::from_f64(0.1));
        let q = Sym::from_f64(q3_33)
            * Sym::from_f64(q3_33)
            * Sym::from_f64(q3_33)
            * Sym::from_f64(q3_33);
        let lhs = (f.sqrt().powi(2) / q).sqrt();
        let rhs = (f / q).sqrt();
        lhs - rhs
    };
    let on_ = how(on(), resid);
    let off_ = how(off(), resid);
    println!("scale-width probe: rule E on {on_} | off {off_}");
    // Evidence: printed, and the one thing asserted is soundness.
    assert!(!on_.starts_with("theorem") || !off_.starts_with("theorem") || on_ == off_);
}

/// The scale step's canonical representative: two spellings of one
/// quotient meet as one atom with the rule on.
#[test]
fn r2_two_spellings_of_one_quotient_key_one_atom() {
    let resid = || {
        let s = over("s", 1.0, 2.0);
        let one = Sym::from_f64(1.0);
        let two = Sym::from_f64(2.0);
        let half = Sym::from_f64(0.5);
        ((s + one) / (two * s)).sqrt() - ((half * s + half) / s).sqrt()
    };
    let on_ = how(on(), resid);
    let off_ = how(off(), resid);
    println!("two spellings: on {on_} | off {off_}");
    assert_eq!(on_, "theorem");
    // Without the rule these are two atoms (the plain form cancels no
    // constant factor); recorded as a fact, not asserted either way.
}

/// **A theorem the early walk reaches WITHOUT rule E and loses WITH it**,
/// through coefficient width. The scale step divides every numerator
/// coefficient by `s`, so a coefficient that already carries a wide
/// denominator part (`1/q`, `q = 3^126` ≈ 200 bits, an A0 fold of
/// `abs(1/q)`) is `1/(c·q)` afterwards (252 bits, still inside the
/// ring); the next product by `1/1000` (odd part 125, 7 bits) needs
/// 259 > `COEFF_BITS` and FREEZES, where the unscaled form's `1/(125·q)`
/// needs 207 and fits. The two sides spell `1/q` once through `abs`
/// (A0 folds it in the early walk only) and once bare, so the plain
/// walk cannot see the identity and only the early walk decides it.
#[test]
fn r2_the_scale_step_loses_an_early_theorem_to_the_ring() {
    let l33 = 5_559_060_566_555_523.0_f64; // 3^33, exact in f64
    let l27 = 7_625_597_484_987.0_f64; // 3^27
    let resid = || {
        let x = over("x", 1.0, 2.0);
        let y = over("y", 1.0, 2.0);
        let one = Sym::from_f64(1.0);
        let q = Sym::from_f64(l33) * Sym::from_f64(l33) * Sym::from_f64(l33) * Sym::from_f64(l27);
        let inv_q = one / q;
        let c = Sym::from_f64(0.1);
        let f_abs = (x + inv_q.abs()) / (x + c);
        let f_raw = (x + inv_q) / (x + c);
        let h = y + (one / Sym::from_f64(1000.0)).abs();
        (f_abs * h).sqrt() - (f_raw * h).sqrt()
    };
    let on_ = how(on(), resid);
    let off_ = how(off(), resid);
    println!("width probe: rule E on {on_} | off {off_}");
    // Evidence: both refuse — without the rule the two spellings are two
    // forms (a constant polynomial denominator against a coefficient),
    // with it the scaled product overflows the ring; `r2_diag_*` splits
    // the stages and the row below pins the loss proper.
    assert!(
        !on_.starts_with("theorem") && !off_.starts_with("theorem"),
        "{on_} | {off_}"
    );
}

/// Diagnostic for the width row: which stage the identity is lost at.
#[test]
fn r2_diag_width_row_stages() {
    let l33 = 5_559_060_566_555_523.0_f64;
    let l27 = 7_625_597_484_987.0_f64;
    for (stage, k) in [
        ("abs only", 0),
        ("+ quotient", 1),
        ("+ product", 2),
        ("small q", 3),
    ] {
        let resid = move || {
            let x = over("x", 1.0, 2.0);
            let y = over("y", 1.0, 2.0);
            let one = Sym::from_f64(1.0);
            let q = if k == 3 {
                Sym::from_f64(l27)
            } else {
                Sym::from_f64(l33) * Sym::from_f64(l33) * Sym::from_f64(l33) * Sym::from_f64(l27)
            };
            let inv_q = one / q;
            let c = Sym::from_f64(0.1);
            let (a, b) = match k {
                0 => (x + inv_q.abs(), x + inv_q),
                1 | 3 => ((x + inv_q.abs()) / (x + c), (x + inv_q) / (x + c)),
                _ => {
                    let h = y + (one / Sym::from_f64(1000.0)).abs();
                    ((x + inv_q.abs()) / (x + c) * h, (x + inv_q) / (x + c) * h)
                }
            };
            a.sqrt() - b.sqrt()
        };
        println!(
            "{stage}: on {} | off {}",
            how(on(), resid),
            how(off(), resid)
        );
    }
}

/// The width control: the same product with a 42-bit `q` fits and is
/// a theorem; so the 200-bit refusal above is the ring, reached
/// through the scale step's `1/(c·q)`.
#[test]
fn r2_diag_width_control_small_q_product() {
    let l27 = 7_625_597_484_987.0_f64;
    let resid = move || {
        let x = over("x", 1.0, 2.0);
        let y = over("y", 1.0, 2.0);
        let one = Sym::from_f64(1.0);
        let inv_q = one / Sym::from_f64(l27);
        let c = Sym::from_f64(0.1);
        let h = y + (one / Sym::from_f64(1000.0)).abs();
        ((x + inv_q.abs()) / (x + c) * h).sqrt() - ((x + inv_q) / (x + c) * h).sqrt()
    };
    println!(
        "small q + product: on {} | off {}",
        how(on(), resid),
        how(off(), resid)
    );
}

/// **A theorem the tier reaches with rule E OFF and not ON.** Both sides
/// spell `P = (x + abs(1/q))/(x + 0.1)` the same way, so the plain walk
/// holds one `abs` atom on each; the left side squares a `sqrt(P)` so
/// only rule A in the early walk can meet the right. Without rule E
/// rule A substitutes `P` (coefficients `1/q`, 200 bits) and the product
/// by `h` fits the ring; with rule E `P` is scaled to `1/(c·q)` (252
/// bits), the product needs 259 and freezes on BOTH sides — to two
/// different indeterminates.
#[test]
fn r2_the_scale_step_loses_a_rule_a_theorem_to_the_ring() {
    let l33 = 5_559_060_566_555_523.0_f64;
    let l27 = 7_625_597_484_987.0_f64;
    let resid = || {
        let x = over("x", 1.0, 2.0);
        let y = over("y", 1.0, 2.0);
        let one = Sym::from_f64(1.0);
        let q = Sym::from_f64(l33) * Sym::from_f64(l33) * Sym::from_f64(l33) * Sym::from_f64(l27);
        let inv_q = one / q;
        let c = Sym::from_f64(0.1);
        let p = (x + inv_q.abs()) / (x + c);
        let h = y + (one / Sym::from_f64(1000.0)).abs();
        (p.sqrt().powi(2) * h).sqrt() - (p * h).sqrt()
    };
    let on_ = how(on(), resid);
    let off_ = how(off(), resid);
    println!("rule-A width probe: rule E on {on_} | off {off_}");
    assert_eq!(off_, "theorem", "without rule E the early walk reaches it");
    assert!(
        !on_.starts_with("theorem"),
        "PINS A LOSS: with rule E this identity is no longer reached — the scale step's \
         coefficient width freezes the product; if this reds the loss has been closed: {on_}"
    );
}
