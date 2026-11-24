//! **The drive-scoped plain memo at the tier's own door** —
//! `with_session_memo` and [`geom_core::sym::DriveMemo`] driven by
//! hand, without a driver between.
//!
//! The unit's drive-level rows (`editor-core`'s
//! `m10_sym_drive_memo_interval`) can only show what a DRIVE produces.
//! These rows build the leaves themselves, so they can put a leaf in a
//! state no fixture in the tree reaches (a node minted outside the
//! session, a leaf whose mints take other names, a budget the table was not
//! made for) and ask what the table does with it.
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

fn lit(v: f64) -> Sym<f64> {
    Sym::from_f64(v)
}

fn zero(m: Sym<f64>) -> bool {
    decide("sym_drive_memo", Margin::of(m), band()) == Ok(Sign::Zero)
}

fn memo() -> Arc<DriveMemo> {
    Arc::new(DriveMemo::new(budget(), SymRules::shipped()))
}

/// **Two terms**, the budget the NEED rows freeze under: a three-term
/// sum does not fit and nothing else they build comes near it.
fn tight() -> SymBudget {
    SymBudget {
        max_terms: 2,
        max_degree: 128,
    }
}

/// The plain quotient form and the constant fold — no rule can rewrite
/// a sum into something that fits, so what freezes under [`tight`] is
/// the budget's verdict on the sum itself.
fn plain_rules() -> SymRules {
    SymRules {
        const_fold: true,
        ..SymRules::none()
    }
}

fn tight_memo() -> Arc<DriveMemo> {
    Arc::new(DriveMemo::new(tight(), plain_rules()))
}

fn p(n: &str, v: f64) -> Sym<f64> {
    Sym::param(ParamSymbol::new(test_utils::symbol_id(n)), v)
}

/// A three-term sum, which does not fit [`tight`] and therefore freezes.
fn three() -> Sym<f64> {
    p("a", 0.25) + p("b", 0.5) + p("c", 0.75)
}

/// A DIFFERENT three-term sum, so two leaves can freeze two nodes.
fn other_three() -> Sym<f64> {
    p("d", 1.25) + p("e", 1.5) + p("f", 1.75)
}

/// One leaf of a drive over `m`, and its receipt.
fn leaf<R>(m: &Arc<DriveMemo>, f: impl FnOnce() -> R) -> SymCounts {
    with_session_memo(tight(), plain_rules(), m, f).1
}

/// The decision columns, `frozen` set aside — the rows that compare two
/// leaves here compare what each DECIDED, and the two leaves of a
/// comparison do not always reach the same nodes.
/// The rows from [`a_leaf_needs_a_frozen_node_once_however_often_it_reaches_it`]
/// down are the ones about `frozen` itself.
fn decisions(c: SymCounts) -> (u64, u64) {
    (c.symbolic, c.numeric)
}

/// **A leaf whose mints take other names moves no decision through
/// the memo** (R2, adopted): an `Opaque` id names a syntactic unknown, a
/// plain form is a syntactic normal form of a syntactic id, and a
/// `Zero` it answers is an identity in whatever unknowns the id's
/// syntax names — so two leaves that give one name (`sym::mint`: here
/// the session root's ordinal) to two different reals are still each
/// handed a sound form. What a leaf-varying mint costs is HITS
/// (`sym::memo`'s header).
#[test]
fn a_leaf_varying_opaque_sequence_moves_no_decision_through_the_memo() {
    let m = memo();
    let leaf = |extra: bool| {
        with_session_memo(budget(), SymRules::shipped(), &m, || {
            if extra {
                // The stand-in for a value-dependent mint: one more
                // opaque before the real ones shifts every later
                // ordinal of this leaf's root scope.
                let _ = Sym::<f64>::opaque(9.0);
            }
            let u = Sym::<f64>::opaque(1.0);
            let v = Sym::<f64>::opaque(2.0);
            let a = zero(u - v);
            let b = zero((u + v) * (u + v) - (u * u + lit(2.0) * u * v + v * v));
            let c = zero(u * u - v * v - (u - v) * (u + v));
            (a, b, c)
        })
    };
    let (r0, c0) = leaf(false);
    let (r1, c1) = leaf(true);
    let (r2, c2) = leaf(false);
    let (r3, c3) = leaf(true);
    assert_eq!(r0, (false, true, true));
    assert_eq!(r1, r0, "a shifted ordinal changed an answer");
    assert_eq!(r2, r0);
    assert_eq!(r3, r0);
    for c in [c0, c1, c2, c3] {
        assert_eq!(decisions(c), (2, 1), "{c:?}");
    }
    println!("memo after four leaves with leaf-varying mint names: {m:?}");
}

/// **A budget mismatch is refused at the DOOR** — a `debug_assert!` in
/// `with_session_memo`, so it is loud in every profile this workspace
/// builds (`[profile.release]` keeps debug assertions on). This is the
/// half `DriveMemo::accepts` cannot state: what the door DOES with the
/// answer.
#[cfg(debug_assertions)]
#[test]
#[should_panic(expected = "a drive memo is valid for the budget and rules it was made for")]
fn a_budget_mismatch_is_refused_at_the_door() {
    let m = memo();
    let other = SymBudget {
        max_terms: 64,
        max_degree: 128,
    };
    let _ = with_session_memo(other, SymRules::shipped(), &m, || zero(lit(1.0) - lit(1.0)));
}

/// **A hit carries the atoms the top-residual reduce needs** (R1/R2,
/// the same row): leaf A mints a `sqrt` atom in its plain walk; leaf B
/// takes the whole form from the memo and never mints the atom itself.
/// If `seed_atoms` did not carry the `AtomInfo` across, rule A could not
/// look the atom's argument back up and `sqrt(x)² − x` would fall from a
/// theorem to a numeric decision.
#[test]
fn a_hit_carries_the_atoms_the_top_residual_reduce_needs() {
    let m = memo();
    let leaf = || {
        with_session_memo(budget(), SymRules::shipped(), &m, || {
            let x = Sym::param(ParamSymbol::new(test_utils::symbol_id("x")), 0.37);
            let s = x.sqrt();
            zero(s * s - x)
        })
    };
    let (a, ca) = leaf();
    let (b, cb) = leaf();
    assert!(
        a && b,
        "sqrt(x)² − x is a theorem under rule A: {ca:?} {cb:?}"
    );
    assert_eq!(decisions(ca), decisions(cb), "{ca:?} {cb:?}");
    println!("memo after two leaves that share one sqrt atom: {m:?}");
}

/// **A leaf's `frozen` column is its NEED, at the tier's own door**:
/// the distinct nodes of the DRIVE's frozen set the leaf reached — a
/// node reached twice counted once, a node reached and not frozen
/// counted zero, and a node this leaf never froze because it inherited
/// the form counted all the same.
///
/// The budget is two terms, so `(a + b) + c` freezes and nothing else
/// in the row does. Leaf 1 builds it and freezes it; leaf 2 takes the
/// frozen form from the memo and freezes NOTHING, and still reports 1 —
/// which is the whole unit: the column says what the leaf's reasoning
/// needed, not which leaf paid for it. Leaf 3 reaches only nodes that
/// fit and reports 0 with the drive's set non-empty.
#[test]
fn a_leaf_needs_a_frozen_node_once_however_often_it_reaches_it() {
    let m = tight_memo();

    // Leaf 1: builds the three-term sum, which does not fit, and
    // reaches it TWICE — once per decision, the second a second walk
    // over the same id.
    let one = leaf(&m, || {
        let first = zero(three() - three());
        let second = zero(three() * p("d", 2.0) - three() * p("d", 2.0));
        (first, second)
    });
    assert_eq!(
        m.size().frozen,
        1,
        "exactly one node of the row freezes: {:?}",
        m.size()
    );
    assert_eq!(one.frozen, 1, "reached twice, needed once: {one:?}");

    // Leaf 2: the same reasoning, every form inherited — it freezes
    // nothing itself and its column is the same 1.
    let two = leaf(&m, || zero(three() - three()));
    assert_eq!(
        two.frozen, 1,
        "the leaf that paid nothing needs exactly what leaf 1 needed: {two:?}"
    );

    // Leaf 3: reaches nothing that froze, with the drive's set
    // non-empty.
    let third = leaf(&m, || {
        let x = p("x", 0.125);
        zero(x - x)
    });
    assert_eq!(
        third.frozen, 0,
        "a leaf that reached no frozen node needs none of them: {third:?}"
    );
    assert_eq!(m.size().frozen, 1, "and the drive's set is still one node");
}

/// **A node the DRIVE froze but this leaf never reached counts zero** —
/// with the drive's set holding TWO nodes, so the row is not the
/// nothing-froze short circuit wearing a different hat.
///
/// Adopted from SYM-13's first reviewer.
#[test]
fn a_frozen_node_outside_this_leafs_closure_is_not_its_need() {
    let m = tight_memo();
    let one = leaf(&m, || zero(three() - three()));
    let two = leaf(&m, || zero(other_three() - other_three()));
    assert_eq!(
        m.size().frozen,
        2,
        "two distinct nodes froze: {:?}",
        m.size()
    );
    assert_eq!(one.frozen, 1, "leaf one reached only its own: {one:?}");
    assert_eq!(two.frozen, 1, "leaf two reached only its own: {two:?}");
    let both = leaf(&m, || {
        zero(three() - three()) && zero(other_three() - other_three())
    });
    assert_eq!(both.frozen, 2, "the leaf that reached both: {both:?}");
}

/// **One frozen node under two different decision roots counts once.**
///
/// Adopted from SYM-13's second reviewer, beside the same claim within
/// one root ([`a_leaf_needs_a_frozen_node_once_however_often_it_reaches_it`]).
#[test]
fn a_frozen_node_under_two_roots_counts_once() {
    let m = tight_memo();
    let one = leaf(&m, || {
        let first = zero(three() - three());
        let second = zero((three() + p("e", 1.0)) - (three() + p("e", 1.0)));
        (first, second)
    });
    assert_eq!(m.size().frozen, 1, "{:?}", m.size());
    assert_eq!(one.frozen, 1, "two roots, one frozen node: {one:?}");
}

/// **The set is the closure of the ROOTS, not the whole table**: a leaf
/// that BUILDS the frozen node — so it is in its hash-consing table —
/// and never asks a decision that reaches it reads 0.
///
/// The row that tells the two spellings apart: seeded from
/// `Session::nodes` instead of the roots, every other row in the tree
/// stays green and this one reds. Adopted from SYM-13's second
/// reviewer.
#[test]
fn a_node_built_but_never_asked_is_not_a_need() {
    let m = tight_memo();
    let paid = leaf(&m, || zero(three() - three()));
    assert_eq!(paid.frozen, 1, "{paid:?}");
    assert_eq!(m.size().frozen, 1, "{:?}", m.size());

    let built = leaf(&m, || {
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

/// **A leaf with no plain-walk root at all** — no decision, or nothing
/// but construction — reads 0 against a non-empty drive set.
///
/// Adopted from SYM-13's second reviewer.
#[test]
fn a_leaf_with_no_roots_needs_nothing() {
    let m = tight_memo();
    let paid = leaf(&m, || zero(three() - three()));
    assert_eq!(paid.frozen, 1, "{paid:?}");

    let idle = leaf(&m, || 0u8);
    assert_eq!(idle.frozen, 0, "no roots: {idle:?}");

    let builder = leaf(&m, || {
        let _s = three();
    });
    assert_eq!(
        builder.frozen, 0,
        "built the frozen node, asked nothing: {builder:?}"
    );
}

/// **An id named before the session mints it, and minted in it
/// afterwards, is the session's node.** The parent is built over a
/// value minted outside; the session then mints the same id, so by the
/// time the leaf reasons the table holds the node and nothing of its
/// reasoning rests on an indeterminate.
#[test]
fn a_foreign_id_the_leaf_later_records_itself_is_not_a_need() {
    let m = tight_memo();
    let outside = lit(2.0);
    let c = leaf(&m, || {
        // The parent names the outside id BEFORE the session mints it.
        let parent = outside * outside;
        // Now the session mints the very same id.
        let inside = lit(2.0);
        let _ = inside * inside;
        zero(parent - parent)
    });
    assert_eq!(
        c.frozen, 0,
        "the leaf's own table holds the node, so it needs nothing for it: {c:?}"
    );
    assert_eq!(m.size().frozen, 0, "and nothing froze: {:?}", m.size());
}
