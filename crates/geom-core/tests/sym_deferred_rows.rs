//! **The deferring session's rows**: a decision several Zeros back is a
//! theorem only when every one of them is ([`MarginDiag::jointly`]),
//! and a margin read in a session that did not decide it is no Zero
//! there ([`deferred::discharge`]).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::predicate::{Band, Margin, MarginDiag, Sign};
use geom_core::sym::deferred::{discharge, with_deferred_session};
use geom_core::{ParamSymbol, Sym, SymBudget, SymRetry, SymRules, Theorem};

fn budget() -> SymBudget {
    SymBudget {
        max_terms: 256,
        max_degree: 128,
    }
}

fn band() -> Band {
    Band::new(1.0e-9, 1.0e-8).unwrap()
}

fn p(name: &str, v: f64) -> Sym<f64> {
    Sym::param(ParamSymbol::new(test_utils::symbol_id(name)), v)
}

/// `margin`'s reading, decided Zero.
fn zero(margin: Sym<f64>) -> MarginDiag {
    let d = geom_core::k_stats::decide_reported("sym_deferred_row", Margin::of(margin), band())
        .expect("decided");
    assert_eq!(d.sign, Sign::Zero);
    d.margin
}

/// In one session: `x − x`, an identity, and `x − y` at `x = y`, a
/// coincidence of values. Joined either way round, the decision is no
/// theorem; joined with itself, it is.
#[test]
fn a_joint_decision_is_a_theorem_only_when_both_are() {
    let (got, _) = with_deferred_session(budget(), SymRules::shipped(), SymRetry::none(), || {
        let x = p("x", 1.0);
        let thm = zero(x - x);
        let open = zero(x - p("y", 1.0));
        [
            discharge(thm),
            discharge(open),
            discharge(thm.jointly(open)),
            discharge(open.jointly(thm)),
            discharge(thm.jointly(thm)),
        ]
    });
    assert_eq!(
        got,
        [
            Some(Theorem::SymbolicZero),
            None,
            None,
            None,
            Some(Theorem::SymbolicZero)
        ]
    );
}

/// A margin carried out of its session reads as no Zero in another one,
/// even one whose own entry would discharge as a theorem.
#[test]
fn a_margin_discharges_only_in_its_own_session() {
    let (open, _) = with_deferred_session(budget(), SymRules::shipped(), SymRetry::none(), || {
        let open = zero(p("x", 1.0) - p("y", 1.0));
        assert_eq!(discharge(open), None);
        open
    });
    let (got, _) = with_deferred_session(budget(), SymRules::shipped(), SymRetry::none(), || {
        let z = p("z", 1.0);
        let thm = zero(z - z);
        (discharge(thm), discharge(open))
    });
    assert_eq!(got, (Some(Theorem::SymbolicZero), None));
    assert_eq!(discharge(open), None, "outside every session");
}

/// The tier's word rides in the reading's padding: a margin is the size
/// of its numbers.
const _: () = assert!(core::mem::size_of::<MarginDiag>() == 24);
