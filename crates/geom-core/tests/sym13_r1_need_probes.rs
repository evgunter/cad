//! **R1's probes for SYM-13's leaf NEED column**, at the tier's own
//! door: the cases the unit's own scalar-door row does not reach.
//!
//! The shipped row `a_leaf_needs_a_frozen_node_once_however_often_it_
//! reaches_it` covers reached-twice, reached-and-not-frozen and
//! inherited-and-still-counted. These rows cover the two the review
//! asked about: a node the DRIVE froze that this leaf never reached,
//! and the UNRECORDED branch with the drive's frozen set non-empty —
//! where the column a leaf reports depends on which leaf ran first.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::sync::Arc;

use geom_core::k_stats::decide;
use geom_core::predicate::{Band, Margin, Sign};
use geom_core::sym::{DriveMemo, with_session_memo};
use geom_core::{ParamSymbol, Sym, SymBudget, SymRules, Tol};

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

fn band() -> Band {
    Band::linear(Tol::witness()).expect("linear band")
}

fn zero(m: Sym<f64>) -> bool {
    decide("sym13_r1", Margin::of(m), band()) == Ok(Sign::Zero)
}

fn p(n: &str, v: f64) -> Sym<f64> {
    Sym::param(ParamSymbol::of(n), v)
}

/// A three-term sum, which does not fit `tight()` and therefore freezes.
fn three() -> Sym<f64> {
    p("a", 0.25) + p("b", 0.5) + p("c", 0.75)
}

/// A DIFFERENT three-term sum, so two leaves can freeze two nodes.
fn other_three() -> Sym<f64> {
    p("d", 1.25) + p("e", 1.5) + p("f", 1.75)
}

/// **A node the DRIVE froze but this leaf never reached counts zero** —
/// with the drive's set holding TWO nodes, so the row is not the
/// `frozen_is_empty` short circuit wearing a different hat.
#[test]
fn a_frozen_node_outside_this_leafs_closure_is_not_its_need() {
    let m = Arc::new(DriveMemo::new(tight(), rules()));
    let (_, one) = with_session_memo(tight(), rules(), &m, || zero(three() - three()));
    let (_, two) = with_session_memo(tight(), rules(), &m, || zero(other_three() - other_three()));
    assert_eq!(
        m.size().frozen,
        2,
        "two distinct nodes froze: {:?}",
        m.size()
    );
    assert_eq!(one.frozen, 1, "leaf one reached only its own: {one:?}");
    assert_eq!(two.frozen, 1, "leaf two reached only its own: {two:?}");
    // The leaf that reaches BOTH reports both.
    let (_, both) = with_session_memo(tight(), rules(), &m, || {
        zero(three() - three()) && zero(other_three() - other_three())
    });
    assert_eq!(both.frozen, 2, "the leaf that reached both: {both:?}");
}

/// **THE UNRECORDED BRANCH, with the drive's frozen set non-empty**:
/// the same leaf, the same box, reports a DIFFERENT column depending on
/// which leaf of the drive ran first.
///
/// Leaf U mints the freezing node OUTSIDE its session, so the node is
/// unrecorded there: `form_in` freezes it, taints everything above it
/// and publishes NOTHING (`memo`'s unrecorded paragraph). The node is
/// nevertheless in U's plain closure — `leaf_need` walks the leaf's
/// hash-consing table from its walk roots and an unrecorded id is
/// reached as a child of a recorded parent.
///
/// So whether that id is in the drive's frozen set when U reads it is
/// whether some OTHER leaf, which did record the node, published its
/// freeze first. Run U first and U's column is 0; run the recording
/// leaf first and U's column is 1. Both are the same leaf.
#[test]
fn an_unrecorded_node_in_the_closure_makes_the_column_order_dependent() {
    // Order A: the leaf that does not record the node runs FIRST.
    let outside_a = three();
    let m_a = Arc::new(DriveMemo::new(tight(), rules()));
    let (_, u_first) = with_session_memo(tight(), rules(), &m_a, || {
        zero(outside_a.clone() - outside_a.clone())
    });
    let (_, r_after) = with_session_memo(tight(), rules(), &m_a, || zero(three() - three()));

    // Order B: the RECORDING leaf runs first and publishes the freeze.
    let outside_b = three();
    let m_b = Arc::new(DriveMemo::new(tight(), rules()));
    let (_, r_first) = with_session_memo(tight(), rules(), &m_b, || zero(three() - three()));
    let (_, u_after) = with_session_memo(tight(), rules(), &m_b, || {
        zero(outside_b.clone() - outside_b.clone())
    });

    println!(
        "unrecorded leaf first: U={} R={} (drive set {:?}) | recording leaf first: R={} U={} \
         (drive set {:?})",
        u_first.frozen,
        r_after.frozen,
        m_a.size(),
        r_first.frozen,
        u_after.frozen,
        m_b.size(),
    );
    assert_eq!(m_a.size().frozen, m_b.size().frozen, "same drive set size");
    assert_eq!(
        r_after.frozen, r_first.frozen,
        "the recording leaf's column does not move"
    );
    // The finding: the UNRECORDED leaf's column does.
    assert_ne!(
        u_first.frozen, u_after.frozen,
        "if these agree the branch is closed and this row should be retired"
    );
    assert_eq!((u_first.frozen, u_after.frozen), (0, 1), "the two readings");
}
