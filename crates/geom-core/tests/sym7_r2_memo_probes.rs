//! **SYM-7 R2 probes at the tier's own door** — `with_session_memo`
//! and `DriveMemo` driven by hand, hunting the box where "a hit hands
//! back the form the leaf would have built" fails.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::sync::Arc;

use geom_core::k_stats::decide;
use geom_core::predicate::{Band, Margin, Sign};
use geom_core::sym::{DriveMemo, with_session, with_session_memo};
use geom_core::{Real, Sym, SymBudget, SymCounts, SymRules, Tol};

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
    decide("r2_probe", Margin::of(m), band()) == Ok(Sign::Zero)
}

fn memo() -> Arc<DriveMemo> {
    Arc::new(DriveMemo::new(budget(), SymRules::shipped()))
}

/// The decision columns of a session, `frozen` set aside.
fn decisions(c: SymCounts) -> (u64, u64, u64) {
    (c.symbolic_zero, c.numeric, c.sign_gated)
}

/// **The box: a node UNRECORDED in one leaf is frozen there — and that
/// frozen form is PUBLISHED, so a later leaf that RECORDED the node
/// takes the indeterminate for a node it would have computed.**
///
/// `sym.rs`'s hit path is guarded ("asked only for a node THIS leaf
/// recorded ... that path is bit-identical"); the publish path is not:
/// `form_in`'s unrecorded branch pushes the id to `plain_built`, so the
/// leaf that saw the node unrecorded hands its freeze to every leaf
/// after it. Here `c = 2` is minted BEFORE leaf A's session (so A does
/// not record it) and INSIDE leaf B's. Without a memo B decides
/// `c·c − 4` as a symbolic zero; with A's memo it decides it
/// numerically — a decision count moved.
#[test]
fn r2_an_unrecorded_freeze_published_by_one_leaf_is_served_to_a_leaf_that_recorded_the_node() {
    let (_, reference) = with_session(budget(), || {
        let c = lit(2.0);
        zero(c * c - lit(4.0))
    });
    assert_eq!(
        decisions(reference),
        (1, 0, 0),
        "without a memo the recorded node folds: {reference:?}"
    );

    let m = memo();
    // Leaf A: `c` minted outside any session — unrecorded, frozen.
    let c_outside = lit(2.0);
    let (_, a) = with_session_memo(budget(), SymRules::shipped(), &m, || {
        zero(c_outside * c_outside - lit(4.0))
    });
    assert_eq!(decisions(a), (0, 1, 0), "leaf A freezes the unrecorded node: {a:?}");
    assert!(a.frozen >= 1, "{a:?}");

    // Leaf B: `c` minted inside its session — recorded, foldable.
    let (_, b) = with_session_memo(budget(), SymRules::shipped(), &m, || {
        let c = lit(2.0);
        zero(c * c - lit(4.0))
    });
    assert_eq!(
        decisions(b),
        decisions(reference),
        "the memo moved a decision: leaf B took leaf A's frozen form for a node B recorded \
         (B with the memo {b:?}, B without one {reference:?})"
    );
}

/// **A leaf-varying opaque sequence does not move a decision through
/// the memo**: an `Opaque` id names a syntactic unknown, a plain form
/// is a syntactic normal form of a syntactic id, and a `Zero` it
/// answers is an identity in whatever unknowns the id's syntax has —
/// so two leaves that give one sequence number to two different reals
/// still build, and are handed back, the same sound form. What a
/// leaf-varying mint costs the memo is HITS, not soundness.
#[test]
fn r2_a_leaf_varying_opaque_sequence_moves_no_decision_through_the_memo() {
    let m = memo();
    let leaf = |extra: bool| {
        with_session_memo(budget(), SymRules::shipped(), &m, || {
            if extra {
                // A value-dependent mint stand-in: one more opaque
                // before the real ones shifts every later sequence
                // number on this leaf.
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
    assert_eq!(r1, r0);
    assert_eq!(r2, r0);
    assert_eq!(r3, r0);
    for c in [c0, c1, c2, c3] {
        assert_eq!(decisions(c), (2, 1, 0), "{c:?}");
    }
    println!("memo after four leaves with a leaf-varying opaque sequence: {m:?}");
}

/// **A budget mismatch is refused** (a `debug_assert!` at the door).
#[cfg(debug_assertions)]
#[test]
#[should_panic(expected = "a drive memo is valid for the budget and rules it was made for")]
fn r2_a_budget_mismatch_is_refused_at_the_door() {
    let m = memo();
    let other = SymBudget {
        max_terms: 64,
        max_degree: 128,
    };
    let _ = with_session_memo(other, SymRules::shipped(), &m, || zero(lit(1.0) - lit(1.0)));
}

/// **A hit's atoms are seeded**: leaf A mints a `sqrt` atom in its plain
/// walk and freezes nothing; leaf B takes the whole form from the memo
/// and never mints the atom, and the top-residual reduce (rule A) still
/// finds the atom's argument — `sqrt(x)² − x` is a theorem on both.
#[test]
fn r2_a_hit_finds_the_atoms_the_leaf_never_minted() {
    let m = memo();
    let leaf = || {
        with_session_memo(budget(), SymRules::shipped(), &m, || {
            let x = Sym::param(geom_core::ParamSymbol::of("x"), 0.37);
            let s = x.sqrt();
            zero(s * s - x)
        })
    };
    let (a, ca) = leaf();
    let (b, cb) = leaf();
    assert!(a && b, "sqrt(x)² − x is a theorem under rule A: {ca:?} {cb:?}");
    assert_eq!(decisions(ca), decisions(cb), "{ca:?} {cb:?}");
    println!("memo: {m:?}");
}
