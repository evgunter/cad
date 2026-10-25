//! **The deferring session's rows**: a decision several Zeros back is
//! proven only when every one of them is ([`MarginDiag::jointly`]),
//! and a margin read in a session that did not decide it is no Zero
//! there ([`deferred::discharge`]); a margin that is not proven
//! says whether what kept it open is an opaque atom
//! ([`deferred::discharge_or_why`]).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::predicate::{Band, Margin, MarginDiag, Sign};
use geom_core::sym::deferred::{Undischarged, discharge, discharge_or_why, with_deferred_session};
use geom_core::{ParamSymbol, Sym, SymBudget, SymRetry, SymRules};

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
/// proven; joined with itself, it is.
#[test]
fn a_joint_decision_is_proven_only_when_both_are() {
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
    assert_eq!(got, [true, false, false, false, true]);
}

/// A margin carried out of its session reads as no Zero in another one,
/// even one whose own entry would discharge.
#[test]
fn a_margin_discharges_only_in_its_own_session() {
    let (open, _) = with_deferred_session(budget(), SymRules::shipped(), SymRetry::none(), || {
        let open = zero(p("x", 1.0) - p("y", 1.0));
        assert!(!discharge(open));
        open
    });
    let (got, _) = with_deferred_session(budget(), SymRules::shipped(), SymRetry::none(), || {
        let z = p("z", 1.0);
        let thm = zero(z - z);
        (discharge(thm), discharge(open))
    });
    assert_eq!(got, (true, false));
    assert!(!discharge(open), "outside every session");
}

/// The tier's word rides in the reading's padding: a margin is the size
/// of its numbers.
const _: () = assert!(core::mem::size_of::<MarginDiag>() == 24);

/// **What keeps a margin open**, read off the Zero that failed. `x − y`
/// at `x = y` is a polynomial in two parameters: open, and no opaque
/// atom. An opaque value against a parameter keeps one. Joined with an
/// identity in front, the identity discharges and the open Zero is the
/// one read; joined with the opaque one in front of the parameter one,
/// the first that fails decides. Reds if a parameter reads as opaque,
/// or an opaque value as a parameter.
#[test]
fn an_open_margin_says_whether_it_keeps_an_opaque_atom() {
    let (got, _) = with_deferred_session(budget(), SymRules::shipped(), SymRetry::none(), || {
        let x = p("x", 1.0);
        let thm = zero(x - x);
        let open = zero(x - p("y", 1.0));
        let opaque = zero(Sym::opaque(1.0) - p("z", 1.0));
        [
            discharge_or_why(thm),
            discharge_or_why(open),
            discharge_or_why(opaque),
            discharge_or_why(thm.jointly(open)),
            discharge_or_why(thm.jointly(opaque)),
        ]
    });
    assert_eq!(
        got,
        [
            Ok(()),
            Err(Undischarged::Otherwise),
            Err(Undischarged::KeepsAnOpaqueAtom),
            Err(Undischarged::Otherwise),
            Err(Undischarged::KeepsAnOpaqueAtom),
        ]
    );
}

/// **A form too big for the budget is no opaque value, and says the
/// budget.** `xy + yz + zx − 3` at `x = y = z = 1` decides Zero by
/// value; at a budget of two terms its form freezes, so it does not
/// discharge, and what keeps it open is a frozen form over the
/// parameters alone, not a value computed apart from them, though the
/// session holds an opaque value elsewhere. Reds if a frozen node reads
/// as opaque, or as a form read whole (`x − y` above is one, and says
/// `Otherwise`).
#[test]
fn a_frozen_form_over_the_parameters_keeps_no_opaque_atom() {
    let tight = SymBudget {
        max_terms: 2,
        max_degree: 128,
    };
    let (got, _) = with_deferred_session(tight, SymRules::shipped(), SymRetry::none(), || {
        let _elsewhere = Sym::opaque(1.0) - p("w", 1.0);
        let (x, y, z) = (p("x", 1.0), p("y", 1.0), p("z", 1.0));
        let three = <Sym<f64> as geom_core::Real>::from_f64(3.0);
        let frozen = zero(x * y + y * z + z * x - three);
        discharge_or_why(frozen)
    });
    assert_eq!(got, Err(Undischarged::OverTheBudget));
}

/// **A form the ring's guards freeze is not over the budget.**
/// `x·c⁸ − y·c⁸` at `x = y = 1`, `c = 0.1`, decides Zero by value; the
/// products' coefficients outgrow the ring's coefficient bound well
/// inside a generous term budget, so the form freezes and does not
/// discharge, and what keeps it open is no budget a larger one would
/// read past. Reds if a guard's freeze reads as the budget's.
#[test]
fn a_form_the_rings_guards_freeze_is_not_over_the_budget() {
    let (got, _) = with_deferred_session(budget(), SymRules::shipped(), SymRetry::none(), || {
        let c = || <Sym<f64> as geom_core::Real>::from_f64(0.1);
        let scaled = |v: Sym<f64>| (0..8).fold(v, |acc, _| acc * c());
        let frozen = zero(scaled(p("x", 1.0)) - scaled(p("y", 1.0)));
        discharge_or_why(frozen)
    });
    assert_eq!(got, Err(Undischarged::Otherwise));
}
