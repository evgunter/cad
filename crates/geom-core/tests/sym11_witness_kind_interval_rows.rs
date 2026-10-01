//! **The EXACT half of the witness partition** — the same rows as
//! `sym11_witness_kind_rows`, at the lane scalar whose value channel is
//! a certified enclosure, in its own file.
//!
//! What the twin says is that the residuals the point channel disputes
//! are not disputable claims: over a box the enclosure of each one
//! contains zero, the numeric channel cannot decide, and the tier
//! answers the identity. The residual builders are the inexact file's
//! own, taking the parameters the caller built — a point there, a box
//! here — so the two halves are one residual and not two copies of it.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::predicate::{Band, Margin};
use geom_core::sym::with_session_rules;
use geom_core::tolerance::Tol;
use geom_core::{Interval, ParamSymbol, Sym, SymRules};

use crate::sym11_witness_kind_rows::{adversary_of, budget, pole_of, witness_agrees};

/// **THE DISPUTE COLUMN IS ZERO AT THE EXACT WITNESS.** R2's adversary
/// and R1's pole over boxes around the points the inexact lanes
/// dispute: the enclosure of the adversary's margin over `x ∈ [1e8 ∓ 1]`
/// straddles zero, so the numeric channel does not answer a definite
/// sign at all and there is nothing to contradict; the box holding the
/// pole is refused by clause 1.
///
/// The row is the `Interval` twin of
/// `sym11_witness_kind_rows::sym11_the_adversary_is_a_counted_dispute_at_sym_f64`,
/// and what it PINS is the charge's arm at this scalar: the count is
/// zero here because `Interval::WITNESS` is `Exact`, which routes the
/// contradiction to the `debug_assert!` and never to the column. A
/// non-zero count is that const having moved, not a document behaving
/// badly.
#[test]
fn sym11_the_exact_witness_disputes_nothing_on_either_mechanism() {
    let band = Band::linear(Tol::witness()).expect("the witness tolerance has a linear band");
    let over = |name: &str, lo: f64, hi: f64| {
        Sym::param_over(ParamSymbol::of(name), Interval::from_bounds(lo, hi), lo, hi)
    };
    let cases: [(&str, &dyn Fn() -> Sym<Interval>); 3] = [
        ("the adversary over x ∈ [1e8 ∓ 1]", &|| {
            adversary_of(over("x", 1.0e8 - 1.0, 1.0e8 + 1.0), over("y", 0.4, 0.6))
        }),
        ("the pole, box straddling t = 1", &|| {
            pole_of(over("t", 0.9, 1.1))
        }),
        ("the pole, box clear of it", &|| {
            pole_of(over("t", 0.2, 0.4))
        }),
    ];
    for (what, build) in cases {
        let (out, counts) = with_session_rules(budget(), SymRules::shipped(), || {
            geom_core::k_stats::decide("sym11_witness_kind", Margin::of(build()), band)
        });
        println!("  {what}: {out:?} | {counts:?}");
        assert_eq!(
            counts.theorems_disputed, 0,
            "{what}: this scalar's witness is EXACT, so the contradiction is routed to the \
             `debug_assert!` and never to this column — a count here is `Interval::WITNESS` \
             having moved: {counts:?}"
        );
    }
}

/// **The EXACT roster**: every `impl Real` in the tree whose witness is
/// a certified enclosure, each named once, through the same generic
/// predicate the inexact roster uses
/// (`sym11_witness_kind_rows::witness_agrees`). The inexact half is
/// that file's own two rows.
#[test]
fn sym11_every_exact_scalar_agrees_with_its_own_refusal_arm() {
    witness_agrees::<Interval>("Interval");
    witness_agrees::<geom_core::Dual<Interval>>("Dual<Interval>");
    witness_agrees::<Sym<Interval>>("Sym<Interval>");
    witness_agrees::<Sym<geom_core::Dual<Interval>>>("Sym<Dual<Interval>>");
}
