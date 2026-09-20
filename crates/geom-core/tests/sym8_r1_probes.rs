//! **R1's independent probes of SYM-8's rule F** (the manifest sign),
//! at the scalar. Review lane `sym/8-review-r1`; evidence-only rows
//! print, the rest assert what they measured.
//!
//! Three questions, taken from the rules' own stated contracts rather
//! than from the unit's rows:
//!
//! 1. **Is the F-before-C ordering PINNED by the row the PR cites?**
//!    `sym_rule_f_rows::the_manifest_sign_lands_in_symbolic_zero_and_not_sign_gated`
//!    builds its parameter with `Sym::param`, which registers no
//!    bracket, and `signed::fold` returns `None` outright when
//!    `params.is_empty()`. Independently, the residual's denominator is
//!    a `sqrt` ATOM, which `signed::fold`'s `enclosable` test refuses.
//!    So rule C cannot fire on that row at either order.
//! 2. **What DOES discriminate the order** — `abs(2/t²)` over a bracket
//!    that excludes zero is a form both rules take, and the two answers
//!    are `theorem` and `sign_gated`.
//! 3. **The predicate's positivity is "wherever it has a value"** — a
//!    form it calls positive can still be UNDEFINED inside the box
//!    (`1/(t−1)²`, whose denominator is a perfect square and so
//!    manifestly non-negative). The module header says clause 1 refuses
//!    there first. These rows drive that case at both lifts.
#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::interval::Interval;
use geom_core::predicate::{Band, Margin, Sign};
use geom_core::real::Real;
use geom_core::sym::with_session_rules;
use geom_core::{ParamSymbol, Sym, SymBudget, SymCounts, SymRules, Tol};

fn budget() -> SymBudget {
    SymBudget {
        max_terms: 4096,
        max_degree: 128,
    }
}

fn label(out: Result<Sign, geom_core::predicate::Indeterminate>, counts: SymCounts) -> String {
    match out {
        Ok(Sign::Zero) if counts.symbolic_zero == 1 => "theorem".to_owned(),
        Ok(Sign::Zero) if counts.registered == 1 => "registered".to_owned(),
        Ok(Sign::Zero) if counts.sign_gated == 1 => "sign_gated".to_owned(),
        Ok(s) => format!("numeric {s:?}"),
        Err(e) => format!("refused {:?}", e.margin),
    }
}

/// An interval-lifted row: the answer, and the margin's enclosure.
fn how_i(rules: SymRules, build: impl FnOnce() -> Sym<Interval>) -> (String, Interval) {
    let band = Band::linear(Tol::witness()).expect("the witness tolerance has a linear band");
    let ((out, v), counts) = with_session_rules(budget(), rules, || {
        let m = build();
        (
            geom_core::k_stats::decide("sym8_r1", Margin::of(m), band),
            m.value,
        )
    });
    (label(out, counts), v)
}

/// An `f64` row: the answer, and the margin's value at the point.
fn how_f(rules: SymRules, build: impl FnOnce() -> Sym<f64>) -> (String, f64) {
    let band = Band::new(1.0e-9, 1.0e-8).unwrap();
    let ((out, v), counts) = with_session_rules(budget(), rules, || {
        let m = build();
        (
            geom_core::k_stats::decide("sym8_r1", Margin::of(m), band),
            m.value,
        )
    });
    (label(out, counts), v)
}

fn over(name: &str, lo: f64, hi: f64) -> Sym<Interval> {
    Sym::param_over(ParamSymbol::of(name), Interval::from_bounds(lo, hi), lo, hi)
}

fn one_i() -> Sym<Interval> {
    Sym::from_f64(1.0)
}

/// The shipped set with rule C ON as well — the set the unit's
/// ordering row uses.
fn shipped_with_c() -> SymRules {
    SymRules {
        signed_root: true,
        ..SymRules::shipped()
    }
}

/// Rule C on, rule F SHUT: what rule C alone would make of a residual.
fn c_only() -> SymRules {
    SymRules {
        signed_root: true,
        manifest_sign: false,
        ..SymRules::shipped()
    }
}

// ----------------------------------------------------- the ordering

/// **THE ORDERING ROW THE PR CITES CANNOT DISCRIMINATE.** The unit's
/// header, `manifest.rs`'s header and the PR body all say the
/// F-before-C order at the node is "pinned by `sym_rule_f_rows`'s
/// rule-C-on row, which still answers `theorem` and not `sign_gated`".
///
/// Rule C cannot fire on that residual at ANY order, for two
/// independent reasons: `Sym::param` registers no bracket, so
/// `sess.params` is empty and `signed::fold` returns `None` on its
/// first line; and the argument `1/sqrt(1+t²)` carries a `sqrt` ATOM in
/// its denominator, which `signed::fold`'s `enclosable` test refuses
/// whatever the brackets are. This row shows the second reason alone is
/// enough, by taking the SAME residual with brackets installed
/// (`param_over`) and rule F shut: rule C does not take it.
#[test]
fn r1_the_cited_ordering_row_cannot_discriminate_the_order() {
    let resid = || {
        let x = one_i() / (one_i() + over("t", 0.2, 0.3).powi(2)).sqrt();
        x.abs() - x
    };
    let (shipped, _) = how_i(SymRules::shipped(), resid);
    let (with_c, _) = how_i(shipped_with_c(), resid);
    let (c, _) = how_i(c_only(), resid);
    println!("abs(1/sqrt(1+t²)) − 1/sqrt(1+t²): shipped {shipped} | +C {with_c} | C only {c}");
    assert_eq!(shipped, "theorem");
    assert_eq!(with_c, "theorem");
    assert_ne!(
        c, "sign_gated",
        "rule C never takes this residual, so the rule-C-on row is invariant under the order"
    );
}

/// **THE SHAPE THAT DOES DISCRIMINATE.** `abs(2/t²)` over a bracket
/// that excludes zero: manifestly positive as a FORM (a positive
/// constant numerator over an even power), and enclosable with a
/// certified sign, so rule C takes it too. Rule F first is a THEOREM;
/// rule C first is `sign_gated`. This is the row the ordering claim
/// needs and the unit does not have.
#[test]
fn r1_a_shape_both_rules_take_is_what_pins_the_order() {
    let resid = || {
        let x = Sym::from_f64(2.0) / over("t", 0.3, 0.5).powi(2);
        x.abs() - x
    };
    let (shipped, _) = how_i(SymRules::shipped(), resid);
    let (with_c, _) = how_i(shipped_with_c(), resid);
    let (c, _) = how_i(c_only(), resid);
    println!("abs(2/t²) − 2/t²: shipped {shipped} | +C {with_c} | C only {c}");
    assert_eq!(shipped, "theorem", "rule F takes it with no value read");
    assert_eq!(c, "sign_gated", "rule C takes it by reading the bracket");
    assert_eq!(
        with_c, "theorem",
        "with both on the value-free rule must be asked first"
    );
}

// ------------------------- positivity is "wherever it has a value"

/// **A FORM THE PREDICATE CALLS POSITIVE THAT IS UNDEFINED INSIDE THE
/// BOX.** `1/(t−1)²` is a positive constant over a PERFECT SQUARE, and
/// `nonneg_poly` takes the perfect-square branch for the DENOMINATOR,
/// so `manifest::positive` says yes. At `t = 1` the form has no value
/// at all. `manifest.rs`'s header says clause 1 — the whole-box
/// certification — has already refused there. This row drives a box
/// that CONTAINS the pole and one that does not, with rule F on and
/// off, and prints what each door answered.
#[test]
fn r1_a_manifestly_positive_form_undefined_inside_the_box() {
    for (name, lo, hi) in [("straddles the pole", 0.9, 1.1), ("clear of it", 0.2, 0.4)] {
        let abs_r = || {
            let x = one_i() / (over("t", lo, hi) - one_i()).powi(2);
            x.abs() - x
        };
        let cs_r = || {
            let x = one_i() / (over("t", lo, hi) - one_i()).powi(2);
            one_i().copysign(x) - one_i()
        };
        for (what, f) in [
            ("abs(X) − X", &abs_r as &dyn Fn() -> Sym<Interval>),
            ("copysign(1, X) − 1", &cs_r),
        ] {
            let (on, v_on) = how_i(SymRules::shipped(), f);
            let (off, v_off) = how_i(SymRules::without_rule_f(), f);
            println!("[{name}] {what}: F-on {on} {v_on:?} | F-off {off} {v_off:?}");
        }
    }
}

/// The same at `f64`, AT the pole: `X = 1/(t−1)²` is `+inf` at `t = 1`,
/// so `abs(X) − X` is `NaN` and `copysign(1, X) − 1` is `0`. If the
/// tier answers `Zero` for the NaN one, the fold outran the value
/// channel's own domain answer.
#[test]
fn r1_the_same_form_at_the_pole_at_f64() {
    let x = || Sym::from_f64(1.0) / (Sym::param(ParamSymbol::of("t"), 1.0) - Sym::from_f64(1.0)).powi(2);
    let (on, v_on) = how_f(SymRules::shipped(), || x().abs() - x());
    let (off, v_off) = how_f(SymRules::without_rule_f(), || x().abs() - x());
    println!("at the pole, abs(X) − X: F-on {on} value {v_on:e} | F-off {off} value {v_off:e}");
    let (on2, v2) = how_f(SymRules::shipped(), || Sym::from_f64(1.0).copysign(x()) - Sym::from_f64(1.0));
    println!("at the pole, copysign(1, X) − 1: F-on {on2} value {v2:e}");
}

// ---------------------------------------------- the signed-zero hunt

/// **CAN A MANIFESTLY POSITIVE FORM'S VALUE CHANNEL SPELL ITS ZERO
/// NEGATIVE?** The header says no, and that a `−0.0` there would be a
/// failure to enclose rather than a break of this rule. The reachable
/// case is UNDERFLOW: `1/(1 + t²)` at `t = 1e200` overflows the
/// denominator to `+inf` and the quotient to zero. This row asserts the
/// spelling of that zero is `+0.0`, which is the branch the fold
/// assumes, and that the residual is a sound theorem.
#[test]
fn r1_the_underflowed_positive_form_spells_its_zero_positive() {
    let t = || Sym::param(ParamSymbol::of("t"), 1.0e200);
    let x = || Sym::from_f64(1.0) / (Sym::from_f64(1.0) + t().powi(2));
    let v = x().value;
    println!("1/(1 + t²) at t = 1e200 = {v:e}, sign_negative {}", v.is_sign_negative());
    assert!(v == 0.0, "the value underflows");
    assert!(
        !v.is_sign_negative(),
        "a manifestly positive form underflowed to −0.0: copysign would take the other branch"
    );
    let (on, val) = how_f(SymRules::shipped(), || {
        Sym::from_f64(1.0).copysign(x()) - Sym::from_f64(1.0)
    });
    println!("copysign(1, X) − 1 at the underflow: {on} value {val:e}");
    assert!(on != "theorem" || val.abs() <= 1.0e-12, "UNSOUND");
}

// ------------------------------------------- the minted magnitude

/// **`magnitude` mints the `Abs` atom by hand** — `indet_atom(Abs.tag(),
/// 0, &[y.digest()])` rather than through `mint_atom` — and claims it is
/// the same indeterminate an `abs(Y)` node elsewhere mints. With a
/// COMPOUND `Y` (not a constant, not manifestly non-negative) that claim
/// is what makes `copysign(Y, 1) − abs(Y)` a theorem.
#[test]
fn r1_the_minted_magnitude_is_the_same_indeterminate_an_abs_node_mints() {
    let resid = || {
        let y = over("x", -0.6, 0.6) - over("z", 0.1, 0.2);
        y.copysign(one_i()) - y.abs()
    };
    let (on, _) = how_i(SymRules::shipped(), resid);
    let (off, _) = how_i(SymRules::without_rule_f(), resid);
    println!("copysign(x − z, 1) − |x − z|: F-on {on} | F-off {off}");
    assert_eq!(on, "theorem", "the minted atom must be the abs node's atom");
}
