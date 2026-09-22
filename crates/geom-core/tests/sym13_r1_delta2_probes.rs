//! **R1's delta-2 probes**: is `Session::foreign` the right set —
//! sound (a foreign id in the closure is one this leaf could not
//! resolve, whatever the order) and complete (does any order still move
//! NEED on the inherit branch and its variants)?
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
    decide("sym13_r1_d2", Margin::of(m), band()) == Ok(Sign::Zero)
}
fn lit(v: f64) -> Sym<f64> {
    Sym::from_f64(v)
}
fn p(n: &str, v: f64) -> Sym<f64> {
    Sym::param(ParamSymbol::of(n), v)
}
fn leaf<R>(b: SymBudget, m: &Arc<DriveMemo>, f: impl FnOnce() -> R) -> SymCounts {
    with_session_memo(b, rules(), m, f).1
}
fn dec(c: SymCounts) -> (u64, u64, u64) {
    (c.symbolic_zero, c.numeric, c.sign_gated)
}

/// **A hit at DEPTH TWO over the foreign node** — the hit is two levels
/// above the id the table does not hold, so the skipped subtree is
/// deeper than the adopted rows' one level.
#[test]
fn a_hit_two_levels_above_the_foreign_node_does_not_move_the_need() {
    let outside = lit(2.0);
    // Q = ((outside·outside) + a) · b: the hit lands at Q, two levels
    // above `outside`.
    let q = |o: Sym<f64>| (o * o + p("a", 1.0)) * p("b", 3.0);

    let m_a = Arc::new(DriveMemo::new(budget(), rules()));
    let first = leaf(budget(), &m_a, || zero(q(outside) - q(outside)));

    let m_b = Arc::new(DriveMemo::new(budget(), rules()));
    let _rec = leaf(budget(), &m_b, || {
        let c = lit(2.0);
        zero(q(c) - q(c))
    });
    let after = leaf(budget(), &m_b, || zero(q(outside) - q(outside)));

    println!(
        "depth-two hit: first {:?} need {} | after {:?} need {} | sets {:?} {:?}",
        dec(first),
        first.frozen,
        dec(after),
        after.frozen,
        m_a.size(),
        m_b.size()
    );
    assert_eq!(dec(first), dec(after), "no decision moved");
    assert_eq!(
        (first.frozen, after.frozen),
        (1, 1),
        "the foreign id is needed in both orders"
    );
}

/// **A foreign id NOTHING ever freezes** — because the leaf that runs
/// later RE-MINTS the same id inside its own session, so its table
/// holds it.
///
/// The question: is a leaf charged a NEED for an id it could resolve
/// itself? Here the SAME leaf builds the parent out of the outside node
/// and then mints the id inside its session, so `foreign` holds an id
/// the table also holds by the time the walk runs.
#[test]
fn a_foreign_id_the_leaf_later_records_itself() {
    let outside = lit(2.0);
    let m = Arc::new(DriveMemo::new(budget(), rules()));
    let c = leaf(budget(), &m, || {
        // The parent names the outside id BEFORE the session mints it.
        let parent = outside * outside;
        // Now the session mints the very same id.
        let inside = lit(2.0);
        let _ = inside * inside;
        zero(parent - parent)
    });
    println!(
        "re-minted foreign id: {:?} need {} | drive set {:?}",
        dec(c),
        c.frozen,
        m.size()
    );
    // What a reader would expect: the leaf resolved the node itself, so
    // nothing of its reasoning rested on an indeterminate.
    assert_eq!(
        c.frozen, 1,
        "R1 FINDING: a node this leaf's own table DOES hold is still charged as a need: {c:?}"
    );
}

/// **A foreign id under a decision that never freezes anywhere** — the
/// drive's set stays EMPTY and no leaf publishes a freeze, yet the
/// column is non-zero. Is that "the frozen nodes its reasoning rested
/// on"?
#[test]
fn a_foreign_id_no_one_freezes_is_still_a_need() {
    let outside = lit(2.0);
    let m = Arc::new(DriveMemo::new(budget(), rules()));
    let rec = leaf(budget(), &m, || {
        let c = lit(2.0);
        zero(c * c - lit(4.0))
    });
    let after = leaf(budget(), &m, || zero(outside * outside - outside * outside));
    println!(
        "nothing frozen anywhere: recorder {:?} need {} | foreign leaf {:?} need {} | set {:?}",
        dec(rec),
        rec.frozen,
        dec(after),
        after.frozen,
        m.size()
    );
    assert_eq!(m.size().frozen, 0, "no drive freeze exists at all");
    assert_eq!(rec.frozen, 0, "the recording leaf needs nothing");
    assert_eq!(
        after.frozen, 1,
        "the foreign leaf reads 1 with nothing frozen"
    );
}

/// **The taint residue, re-derived independently of the shipped row**,
/// with the drive's frozen set NON-EMPTY, to check it is not the short
/// circuit and that the reading really is the order's.
#[test]
fn the_taint_residue_moves_with_the_drive_set_non_empty() {
    let seed = || p("s", 0.25) + p("t", 0.5) + p("u", 0.75);
    let outside = lit(0.0);
    let sum = |o: Sym<f64>| o + p("x", 1.0) + p("y", 2.0);

    let m_a = Arc::new(DriveMemo::new(tight(), rules()));
    let _s1 = leaf(tight(), &m_a, || zero(seed() - seed()));
    let first = leaf(tight(), &m_a, || zero(sum(outside) - sum(outside)));

    let m_b = Arc::new(DriveMemo::new(tight(), rules()));
    let _s2 = leaf(tight(), &m_b, || zero(seed() - seed()));
    let _rec = leaf(tight(), &m_b, || {
        let z = lit(0.0);
        zero(sum(z) - sum(z))
    });
    let after = leaf(tight(), &m_b, || zero(sum(outside) - sum(outside)));

    println!(
        "taint residue, non-empty set: first need {} {:?} | after need {} {:?} | sets {:?} {:?}",
        first.frozen,
        dec(first),
        after.frozen,
        dec(after),
        m_a.size(),
        m_b.size()
    );
    assert!(
        m_a.size().frozen >= 1 && m_b.size().frozen >= 1,
        "non-empty"
    );
    assert_ne!(
        first.frozen, after.frozen,
        "the residue is real with the set non-empty"
    );
}
