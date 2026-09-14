//! **SYM-7 review probes (R1)** — the drive memo at the scalar's own
//! public door (`with_session_memo`, `DriveMemo`), independent of the
//! driver.
//!
//! Attacks claim 3 ("a hit hands back the form the leaf would have
//! built") from the two directions the unit's own rows do not: the
//! atoms a hit's form needs but the hitting leaf never minted, and the
//! WRITE side of the "asked only for a node this leaf recorded"
//! guard.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::sync::Arc;

use geom_core::k_stats::decide;
use geom_core::predicate::{Band, Margin, Sign};
use geom_core::sym::{DriveMemo, with_session_memo};
use geom_core::{ParamSymbol, Real, Sym, SymBudget, SymCounts, SymRules, Tol};

fn budget() -> SymBudget {
    SymBudget {
        max_terms: 4096,
        max_degree: 128,
    }
}

fn band() -> Band {
    Band::linear(Tol::witness()).expect("linear band")
}

fn p(name: &str, v: f64) -> Sym<f64> {
    Sym::param(ParamSymbol::of(name), v)
}

fn zero(m: Sym<f64>) -> bool {
    decide("sym7_r1_probe", Margin::of(m), band()) == Ok(Sign::Zero)
}

fn memo() -> Arc<DriveMemo> {
    Arc::new(DriveMemo::new(budget(), SymRules::shipped()))
}

fn leaf(m: &Arc<DriveMemo>, f: impl FnOnce() -> bool) -> (bool, SymCounts) {
    with_session_memo(budget(), SymRules::shipped(), m, f)
}

/// The identity the probes ask about: `√x · √x − x`, which rule A
/// discharges only if the leaf can find the `sqrt` atom's `AtomInfo`.
fn rule_a_identity() -> Sym<f64> {
    let x = p("x", 2.5);
    x.sqrt() * x.sqrt() - x
}

/// **Claim 3, the atoms a hit needs.** Leaf 1 builds the rule-A
/// identity and publishes its plain forms and the `sqrt` atom; leaf 2
/// hits the memo at the root and never mints that atom itself. If
/// `seed_atoms` did not carry the `AtomInfo` across, the top-residual
/// reduce would not find the atom's argument and the identity would go
/// from a theorem to a numeric decision.
#[test]
fn a_hit_carries_the_atoms_the_top_residual_reduce_needs() {
    let m = memo();
    let (z1, c1) = leaf(&m, || zero(rule_a_identity()));
    let (z2, c2) = leaf(&m, || zero(rule_a_identity()));
    let fresh = memo();
    let (z3, c3) = leaf(&fresh, || zero(rule_a_identity()));
    println!("leaf1 {z1} {c1:?}\nleaf2(hit) {z2} {c2:?}\ncontrol {z3} {c3:?}");
    assert_eq!(
        (z1, z2, z3),
        (true, true, true),
        "the rule-A identity must be a theorem on every leaf"
    );
    assert_eq!(
        (c2.symbolic_zero, c2.sign_gated, c2.registered, c2.numeric),
        (c3.symbolic_zero, c3.sign_gated, c3.registered, c3.numeric),
        "a leaf that hit the memo decided differently from one that did not"
    );
}

/// **Claim 3, the WRITE side of the unrecorded guard.**
///
/// `form_in` refuses to ASK the drive memo for a node absent from this
/// leaf's `Session::nodes` (the walk freezes such a node by design, and
/// the comment says taking a drive-built form for it would move a
/// decision). Nothing refuses to PUBLISH one: the unrecorded branch
/// pushes the id into `plain_built`, so the frozen indeterminate the
/// leaf built for it is published to the drive memo under the node's
/// content-hashed id — the same id a leaf that DID record the node
/// computes a real form for.
///
/// Leaf 1 builds the expression outside its session (so every node is
/// unrecorded there); leaf 2 builds the same expression inside its
/// own session, where the tier would prove it zero. The control is
/// leaf 2 with a memo leaf 1 never touched.
#[test]
fn an_unrecorded_node_publishes_its_frozen_form_and_a_later_leaf_inherits_it() {
    let m = memo();
    // Built BEFORE the session is installed: `intern` records nothing.
    let outside = rule_a_identity();
    let (z1, c1) = leaf(&m, || zero(outside));
    println!("memo after leaf1: {:?}", m.size());
    let (z2, c2) = leaf(&m, || zero(rule_a_identity()));
    println!("memo after leaf2: {:?}", m.size());
    let fresh = memo();
    let (z3, c3) = leaf(&fresh, || zero(rule_a_identity()));
    println!("leaf1(outside) {z1} {c1:?}\nleaf2(shared memo) {z2} {c2:?}\ncontrol {z3} {c3:?}");
    assert_eq!(z3, true, "the control leaf proves the identity");
    assert_eq!(
        (z2, c2.symbolic_zero, c2.numeric),
        (z3, c3.symbolic_zero, c3.numeric),
        "leaf 2 inherited a form leaf 1 froze for a node leaf 1 never recorded — \
         the read side guards this and the write side does not"
    );
}

/// The same asymmetry with the two leaves the other way round: the
/// leaf that RECORDS the node runs first and publishes the real form,
/// and the leaf that does not record it freezes as it always did. This
/// is the order that is sound; it is here to show the defect above is
/// an ORDER dependence — that is, under a drive, a schedule
/// dependence — and not a constant.
#[test]
fn the_order_of_the_two_leaves_decides_which_form_the_memo_keeps() {
    let m = memo();
    let (_, recorded) = leaf(&m, || zero(rule_a_identity()));
    let outside = rule_a_identity();
    let (z2, c2) = leaf(&m, || zero(outside));
    println!("recorded-first {recorded:?}\nunrecorded-second {z2} {c2:?}");
}
