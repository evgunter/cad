//! **Rule F's rows at the INTERVAL lift** — the half of
//! `sym_rule_f_rows` that needs a BOX rather than a point, in its own
//! wholly feature-gated file because `crates/*/tests` owes WHOLE-ITEM
//! gating (`scripts/check-interval-cfg-additive.py`: a test present in
//! both builds must run identical code, so a row whose body is gated
//! inside a shared file runs nowhere on the interval legs).
//!
//! Clause 1 — the value channel certified the computation on the whole
//! input box — is a statement about a BOX, and three of rule F's
//! claims rest on it: that a denominator the predicate leans on is
//! non-zero wherever the form has a value, that a form the predicate
//! calls positive may still be UNDEFINED inside the box and is clause
//! 1's there, and that the `f64` lift's disagreements do not reach the
//! certified lane. Those rows are here; the point-lift rows, the
//! predicate's boundary and the negatives are in `sym_rule_f_rows`,
//! whose helpers this file shares.
#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::predicate::Margin;
use geom_core::sym::with_session_rules;
use geom_core::{Interval, ParamSymbol, Real, Sym, SymRules};

use crate::sym_rule_f_rows::{budget, how, label, one, p, sound};

fn one_i() -> Sym<Interval> {
    Sym::from_f64(1.0)
}
/// A bracketed parameter at the INTERVAL lift, for the rows that need a
/// box rather than a point (clause 1 answers on a box).
fn over(name: &str, lo: f64, hi: f64) -> Sym<Interval> {
    Sym::param_over(ParamSymbol::of(name), Interval::from_bounds(lo, hi), lo, hi)
}
/// [`how`] at the interval lift: the answer, and the margin's
/// enclosure.
fn how_i(rules: SymRules, build: impl FnOnce() -> Sym<Interval>) -> (String, Interval) {
    let band = geom_core::predicate::Band::linear(geom_core::Tol::witness())
        .expect("the witness tolerance has a linear band");
    let ((out, value), counts) = with_session_rules(budget(), rules, || {
        let m = build();
        (
            geom_core::k_stats::decide("sym_rule_f_row", Margin::of(m), band),
            m.value,
        )
    });
    (label(out, counts), value)
}
/// **THE `D = 0` EDGE (R2, addendum 1).** `abs(1/sqrt(t²)) − 1/sqrt(t²)`
/// — the argument's numerator is a positive constant and its
/// denominator a `sqrt` atom, which is non-negative and NOT positive
/// (`t² = 0` at `t = 0`). The predicate calls the FORM positive anyway,
/// on the strength of `quotient`'s side condition: `D ≠ 0` at every
/// point of a box clause 1 admits. This row is that condition's pin —
/// the fold at a point clear of zero, and no theorem over a box that
/// holds `t = 0`, where the value channel divided by an interval
/// containing zero.
#[test]
fn the_denominator_that_vanishes_is_refused_by_clause_1_not_folded() {
    assert_eq!(
        sound(
            "abs(1/sqrt(t²)) − 1/sqrt(t²) at t = 0.25",
            how(SymRules::shipped(), || {
                let x = one() / p("t", 0.25).powi(2).sqrt();
                x.abs() - x
            })
        ),
        "theorem"
    );
    let over_box = |lo: f64, hi: f64| {
        how_i(SymRules::shipped(), move || {
            let x = one_i() / over("t", lo, hi).powi(2).sqrt();
            x.abs() - x
        })
    };
    let (holds_zero, _) = over_box(-0.1, 0.4);
    println!("  … over t ∈ [−0.1, 0.4] (D = 0 inside): {holds_zero}");
    assert_ne!(
        holds_zero, "theorem",
        "a box the value channel divided by zero on is clause 1's, not the fold's"
    );
    let (clear, _) = over_box(0.2, 0.3);
    println!("  … over t ∈ [0.2, 0.3]: {clear}");
    assert_eq!(clear, "theorem");
}
/// **A MANIFESTLY POSITIVE FORM UNDEFINED INSIDE THE BOX (R1, item G).**
/// `1/(t − 1)²` is a positive constant over a PERFECT SQUARE, which the
/// predicate accepts for a DENOMINATOR, so `manifest::positive` says
/// yes — and at `t = 1` the form has no value at all. The header says
/// clause 1 refuses there first; this row drives a box that contains
/// the pole and one clear of it, under both arms.
#[test]
fn a_manifestly_positive_form_undefined_inside_the_box() {
    for (name, lo, hi, want_theorem) in [
        ("straddles the pole", 0.9, 1.1, false),
        ("clear of it", 0.2, 0.4, true),
    ] {
        for what in ["abs", "copysign"] {
            let (l, v) = how_i(SymRules::shipped(), || {
                let x = one_i() / (over("t", lo, hi) - one_i()).powi(2);
                if what == "abs" {
                    x.abs() - x
                } else {
                    one_i().copysign(x) - one_i()
                }
            });
            println!("  [{name}] {what}: {l} enclosure {v:?}");
            assert_eq!(
                l == "theorem",
                want_theorem,
                "[{name}] {what}: a box holding the pole must be clause 1's"
            );
        }
    }
}
/// **THE HAND-MINTED MAGNITUDE IS THE ATOM AN `abs` NODE MINTS (R1,
/// addendum 3).** `magnitude` folds `copysign(Y, X)` to the `Abs` ATOM
/// over `Y` when `Y` is neither constant nor manifestly non-negative,
/// and the whole value of doing so is that the atom is the SAME
/// indeterminate an `abs(Y)` node elsewhere in the DAG mints. With a
/// compound `Y` that claim is what makes this residual the zero form,
/// and it is the pin of the `mint_atom` door the fold now goes through.
#[test]
fn the_minted_magnitude_is_the_same_indeterminate_an_abs_node_mints() {
    let resid = || {
        let y = over("x", -0.6, 0.6) - over("z", 0.1, 0.2);
        y.copysign(one_i()) - y.abs()
    };
    let (on, _) = how_i(SymRules::shipped(), resid);
    let (off, _) = how_i(SymRules::without_rule_f(), resid);
    println!("  copysign(x − z, 1) − |x − z|: F-on {on} | F-off {off}");
    assert_eq!(on, "theorem", "the minted atom must be the abs node's atom");
    assert_ne!(off, "theorem", "rule F is what takes it");
}
/// The same adversary at the INTERVAL lift, which is the certified
/// lane: `E`'s enclosure over a box around `x = 1e8` straddles zero,
/// the value channel cannot decide, and the tier answers `theorem` —
/// the identity, correctly. Gating, because nothing here panics.
#[test]
fn the_adversary_at_the_interval_lift_is_a_plain_theorem() {
    let tiny = 1.0e-30;
    let (l, v) = how_i(SymRules::shipped(), || {
        let x = over("x", 1.0e8 - 1.0, 1.0e8 + 1.0);
        let y = over("y", 0.4, 0.6);
        let e = (x + one_i()).powi(2) - x.powi(2) - Sym::from_f64(2.0) * x - one_i()
            + Sym::from_f64(tiny) * (one_i() + y.powi(2));
        one_i().copysign(e) - one_i()
    });
    println!("  interval lift over x ∈ [1e8 ∓ 1]: {l} enclosure {v:?}");
    assert_eq!(l, "theorem");
}
