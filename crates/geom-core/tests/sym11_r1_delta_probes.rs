//! SYM-11 delta probe (r1): the gated-dispute row's residual, with the
//! placement removed, so the discharge KIND that row's dispute comes
//! from is visible — at `d = 0` the point channel reads `0` and the tier
//! must answer through rule C's gated fold, never as a plain theorem.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::predicate::{Band, Margin, Sign};
use geom_core::sym::with_session_rules;
use geom_core::{ParamSymbol, Real, Sym, SymBudget, SymRules};

fn resid(d0: f64) -> Sym<f64> {
    let r = Sym::<f64>::param_over(ParamSymbol::of("r"), 1.25e-3, 1.0e-3, 2.0e-3);
    let d = Sym::<f64>::param(ParamSymbol::of("d"), d0);
    (((r + d) - d) * ((r + d) - d)).sqrt() - r
}

#[test]
fn r1_the_gated_rows_residual_is_sign_gated_and_not_a_theorem_at_the_origin() {
    let budget = SymBudget {
        max_terms: 4096,
        max_degree: 128,
    };
    let band = Band::new(1.0e-9, 1.0e-8).unwrap();
    for (rules, what) in [(SymRules::all(), "all"), (SymRules::shipped(), "shipped")] {
        let (out, c) = with_session_rules(budget, rules, || {
            geom_core::k_stats::decide("r1_delta", Margin::of(resid(0.0)), band)
        });
        println!("  d=0 rules={what}: {out:?} | {c:?}");
        if what == "all" {
            assert_eq!(out, Ok(Sign::Zero));
            assert_eq!(
                (c.sign_gated, c.symbolic_zero),
                (1, 0),
                "the zero is rule C's GATED fold: {c:?}"
            );
        } else {
            assert_eq!(
                (c.sign_gated, c.symbolic_zero),
                (0, 0),
                "rule C off: no fold at all: {c:?}"
            );
        }
    }
}
