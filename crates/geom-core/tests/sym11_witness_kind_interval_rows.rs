//! **The EXACT half of the witness partition** — the same two rows as
//! `sym11_witness_kind_rows`, at the lane scalar whose value channel
//! is a certified enclosure, in its own wholly feature-gated file
//! because `crates/*/tests` owes WHOLE-ITEM gating
//! (`scripts/check-interval-cfg-additive.py`).
//!
//! What the twin says is that the residuals the point channel disputes
//! are not disputable claims: over a box the enclosure of each one
//! contains zero, the numeric channel cannot decide, and the tier
//! answers the identity. So the dispute column is ZERO here, and it is
//! zero because the enclosure is an enclosure — not because the
//! documents happen to behave.
#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::predicate::{Band, Margin};
use geom_core::sym::{SymRegistration, with_session_rules};
use geom_core::tolerance::Tol;
use geom_core::{Interval, ParamSymbol, Real, Sym, SymBudget, SymRules, Witness};

fn budget() -> SymBudget {
    SymBudget {
        max_terms: 4096,
        max_degree: 128,
    }
}

/// **THE DISPUTE COLUMN IS ZERO AT THE EXACT WITNESS.** R2's adversary
/// and R1's pole over boxes around the points the inexact lanes
/// dispute: the enclosure of the adversary's margin over `x ∈ [1e8 ∓ 1]`
/// straddles zero, so the numeric channel does not answer a definite
/// sign at all and there is nothing to contradict; the box holding the
/// pole is refused by clause 1.
///
/// The row is the `Interval` twin of
/// `sym11_witness_kind_rows::sym11_the_adversary_is_a_counted_dispute_at_sym_f64`,
/// and it asserts the count `== 0` rather than the absence of a panic:
/// at an EXACT witness the contradiction is a `debug_assert!`, so a
/// non-zero count here would mean the const on this scalar had moved.
#[test]
fn sym11_the_exact_witness_disputes_nothing_on_either_mechanism() {
    let band = Band::linear(Tol::witness()).expect("the witness tolerance has a linear band");
    let over = |name: &str, lo: f64, hi: f64| {
        Sym::param_over(ParamSymbol::of(name), Interval::from_bounds(lo, hi), lo, hi)
    };
    let one = || Sym::<Interval>::from_f64(1.0);
    let cases: [(&str, &dyn Fn() -> Sym<Interval>); 3] = [
        ("the adversary over x ∈ [1e8 ∓ 1]", &|| {
            let x = over("x", 1.0e8 - 1.0, 1.0e8 + 1.0);
            let y = over("y", 0.4, 0.6);
            let e = (x + one()).powi(2) - x.powi(2) - Sym::from_f64(2.0) * x - one()
                + Sym::from_f64(1.0e-30) * (one() + y.powi(2));
            one().copysign(e) - one()
        }),
        ("the pole, box straddling t = 1", &|| {
            let x = one() / (over("t", 0.9, 1.1) - one()).powi(2);
            one().copysign(x) - one()
        }),
        ("the pole, box clear of it", &|| {
            let x = one() / (over("t", 0.2, 0.4) - one()).powi(2);
            one().copysign(x) - one()
        }),
    ];
    for (what, build) in cases {
        let (out, counts) = with_session_rules(budget(), SymRules::shipped(), || {
            geom_core::k_stats::decide("sym11_witness_kind", Margin::of(build()), band)
        });
        println!("  {what}: {out:?} | {counts:?}");
        assert_eq!(
            counts.theorems_disputed, 0,
            "{what}: a DEFINITE non-zero sign at a certified enclosure is a proof that the \
             margin is not zero, so a form that is the zero polynomial under it is a \
             soundness defect in one of the two channels — which is asserted at this \
             scalar and never counted: {counts:?}"
        );
    }
}

/// The EXACT half of the two-contract pin
/// (`sym11_witness_kind_rows::sym11_the_witness_kind_and_the_refusal_arm_are_one_claim`):
/// a scalar that declares an exact witness refuses with
/// `Contradicted` and never with `Disputed`.
#[test]
fn sym11_the_exact_witness_kind_and_the_refusal_arm_are_one_claim() {
    let tol = Tol::witness();
    let disjoint = || {
        (
            Interval::from_bounds(0.0, 1.0),
            Interval::from_bounds(2.0, 3.0),
        )
    };
    let (a, b) = disjoint();
    let rows: [(&str, Witness, SymRegistration); 2] = [
        (
            "Interval",
            <Interval as Real>::WITNESS,
            a.register_equal(b, tol),
        ),
        (
            "Sym<Interval>",
            <Sym<Interval> as Real>::WITNESS,
            with_session_rules(budget(), SymRules::shipped(), || {
                let (a, b) = disjoint();
                Sym::param(ParamSymbol::of("a"), a)
                    .register_equal(Sym::param(ParamSymbol::of("b"), b), tol)
            })
            .0,
        ),
    ];
    for (name, witness, arm) in rows {
        println!("  {name}: {witness:?} refuses {arm:?}");
        assert_eq!(
            witness,
            Witness::Exact,
            "{name} is in this row's EXACT roster"
        );
        assert_eq!(
            arm,
            SymRegistration::Contradicted,
            "{name} declares an EXACT witness and refuses {arm:?} — two certified \
             enclosures that do not meet PROVE the reals differ, so `Disputed` is not an \
             arm it may answer"
        );
    }
}
