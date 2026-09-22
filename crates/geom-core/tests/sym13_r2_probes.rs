//! **SYM-13 reviewer r2's probes at the scalar door** — `leaf_need` on
//! one session, by hand, without a driver between.
//!
//! What the unit's own row (`sym_drive_memo::a_leaf_needs_a_frozen_node_once_however_often_it_reaches_it`)
//! does not pin: that the set is the closure of the walk ROOTS and not
//! the whole hash-consing table (a leaf that BUILDS a frozen node and
//! never asks about it), that a leaf with no plain-walk root at all
//! reads 0, and what the column says on the unrecorded branch the
//! memo's header names — where it is order-dependent, by execution.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::sync::Arc;

use geom_core::k_stats::decide;
use geom_core::predicate::{Band, Margin, Sign};
use geom_core::sym::{DriveMemo, with_session_memo};
use geom_core::{ParamSymbol, Sym, SymBudget, SymRules, Tol};

fn band() -> Band {
    Band::linear(Tol::witness()).expect("linear band")
}

fn zero(m: Sym<f64>) -> bool {
    decide("sym13_r2_probes", Margin::of(m), band()) == Ok(Sign::Zero)
}

/// Two terms: `a + b + c` freezes and nothing else in these rows does.
fn tight() -> SymBudget {
    SymBudget {
        max_terms: 2,
        max_degree: 128,
    }
}

fn rules() -> SymRules {
    SymRules {
        const_fold: true,
        ..SymRules::none()
    }
}

fn p(n: &str, v: f64) -> Sym<f64> {
    Sym::param(ParamSymbol::of(n), v)
}

fn three() -> Sym<f64> {
    p("a", 0.25) + p("b", 0.5) + p("c", 0.75)
}

fn memo() -> Arc<DriveMemo> {
    Arc::new(DriveMemo::new(tight(), rules()))
}

/// One frozen node under two different decision roots counts once.
#[test]
fn r2_a_frozen_node_under_two_roots_counts_once() {
    let m = memo();
    let (_, one) = with_session_memo(tight(), rules(), &m, || {
        let first = zero(three() - three());
        let second = zero((three() + p("e", 1.0)) - (three() + p("e", 1.0)));
        (first, second)
    });
    assert_eq!(m.size().frozen, 1, "{:?}", m.size());
    assert_eq!(one.frozen, 1, "two roots, one frozen node: {one:?}");
}

/// **The set is the closure of the ROOTS, not the whole table**: a leaf
/// that BUILDS the frozen node (so it is in its hash-consing table) and
/// never asks a decision that reaches it reads 0. A whole-table
/// intersection would read 1 here while every row the unit ships stays
/// green — this is the row that tells the two apart.
#[test]
fn r2_a_node_built_but_never_asked_is_not_a_need() {
    let m = memo();
    let (_, paid) = with_session_memo(tight(), rules(), &m, || zero(three() - three()));
    assert_eq!(paid.frozen, 1, "{paid:?}");
    assert_eq!(m.size().frozen, 1, "{:?}", m.size());

    let (_, built) = with_session_memo(tight(), rules(), &m, || {
        // Built — minted into this session's table — and never walked.
        let _in_the_table = three();
        let x = p("x", 0.125);
        zero(x - x)
    });
    assert_eq!(
        built.frozen, 0,
        "the node is in the table and in the drive's set, but in no root's closure: {built:?}"
    );
}

/// A leaf with no plain-walk root at all — no decision, or nothing but
/// construction — reads 0 against a non-empty drive set.
#[test]
fn r2_a_leaf_with_no_roots_needs_nothing() {
    let m = memo();
    let (_, paid) = with_session_memo(tight(), rules(), &m, || zero(three() - three()));
    assert_eq!(paid.frozen, 1, "{paid:?}");

    let (_, idle) = with_session_memo(tight(), rules(), &m, || 0u8);
    assert_eq!(idle.frozen, 0, "no roots: {idle:?}");

    let (_, builder) = with_session_memo(tight(), rules(), &m, || {
        let _s = three();
    });
    assert_eq!(builder.frozen, 0, "built the frozen node, asked nothing: {builder:?}");
}

/// **The unrecorded branch, by execution.** Leaf A reaches, in its
/// closure, a node it never RECORDED (`a + b + c` minted before the
/// session) — the plain walk freezes it and publishes nothing. What A's
/// column says depends on whether a leaf that DID record the node has
/// already published its freeze: 0 before leaf B, 1 after. A's
/// reasoning rested on an indeterminate in both orders.
///
/// The memo's header names this branch and `editor-core` pins it at
/// zero on the measured drives; this row records what the column does
/// when the branch is reached, so that the sentence "an unrecorded
/// freeze is in no drive's set" is read with the order in mind.
#[test]
fn r2_an_unrecorded_node_in_the_closure_is_counted_by_order() {
    let m = memo();
    let outside = three();

    let (_, a_before) = with_session_memo(tight(), rules(), &m, || zero(outside - outside));
    assert_eq!(
        m.size().frozen,
        0,
        "A's unrecorded freeze is not published: {:?}",
        m.size()
    );
    assert_eq!(
        a_before.frozen, 0,
        "A before B: the freeze A made is in no drive's set: {a_before:?}"
    );

    let (_, b) = with_session_memo(tight(), rules(), &m, || zero(three() - three()));
    assert_eq!(m.size().frozen, 1, "B recorded it and froze it: {:?}", m.size());
    assert_eq!(b.frozen, 1, "{b:?}");

    let (_, a_after) = with_session_memo(tight(), rules(), &m, || zero(outside - outside));
    assert_eq!(
        a_after.frozen, 1,
        "the same leaf A after B: the node is now in the set and in A's closure: {a_after:?}"
    );
    assert_eq!(
        (a_before.symbolic_zero, a_before.numeric),
        (a_after.symbolic_zero, a_after.numeric),
        "the decision columns did not move; only `frozen` did"
    );
}
