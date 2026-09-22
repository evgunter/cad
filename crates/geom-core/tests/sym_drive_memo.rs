//! **The drive-scoped plain memo at the tier's own door** —
//! `with_session_memo` and [`geom_core::sym::DriveMemo`] driven by
//! hand, without a driver between.
//!
//! The unit's drive-level rows (`editor-core`'s
//! `m10_sym_drive_memo_interval`) can only show what a DRIVE produces.
//! These rows build the leaves themselves, so they can put a leaf in a
//! state no fixture in the tree reaches — a node minted outside the
//! session, a leaf-varying opaque sequence, a budget the memo was not
//! made for — and ask what the memo does with it.
//!
//! Adopted from both reviews of SYM-7 (R1's `sym7_r1_memo_probes`, R2's
//! `sym7_r2_memo_probes`); the rows that found the write-side defect are
//! kept as the regression pins for it.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::sync::Arc;

use geom_core::k_stats::decide;
use geom_core::predicate::{Band, Margin, Sign};
use geom_core::sym::{DriveMemo, with_session_memo, with_session_rules};
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

/// The decision columns, `frozen` set aside — the rows that compare two
/// leaves here compare what each DECIDED, and the two leaves of a
/// comparison do not always reach the same nodes.
/// [`a_leaf_needs_a_frozen_node_once_however_often_it_reaches_it`] is
/// the row about `frozen` itself.
fn decisions(c: SymCounts) -> (u64, u64, u64) {
    (c.symbolic_zero, c.numeric, c.sign_gated)
}

/// **The write side of the unrecorded guard** (R1 M6 / R2 MINOR-2 —
/// RED before the fix, green after).
///
/// `form_in` refuses to ASK the memo for a node absent from this leaf's
/// `Session::nodes`: such a node is frozen by design, and taking a
/// drive-built form for it would move a decision. The publish side used
/// to do the opposite — it pushed the unrecorded freeze into the
/// publication list, so the frozen indeterminate reached the memo under
/// the node's CONTENT id, the same id a leaf that DID record the node
/// computes a real form for.
///
/// Leaf A mints `c = 2` OUTSIDE its session, so `c` is unrecorded there
/// and freezes. Leaf B mints it inside, where the constant fold proves
/// `c·c − 4` zero. The dials are the plain walk with A0 and no early
/// walk, so nothing downstream can rescue the form (under
/// `SymRules::shipped` the EARLY walk, which stays per leaf, re-folds it
/// and hides the move). Before the fix leaf B came back
/// `symbolic_zero 0 / numeric 1`; the reference — the same leaf with no
/// memo at all — is `1 / 0`.
#[test]
fn an_unrecorded_freeze_is_never_published_to_the_drive_memo() {
    let rules = SymRules {
        const_fold: true,
        ..SymRules::none()
    };
    let (_, reference) = with_session_rules(budget(), rules, || {
        let c = lit(2.0);
        zero(c * c - lit(4.0))
    });
    assert_eq!(
        decisions(reference),
        (1, 0, 0),
        "without a memo the recorded node folds: {reference:?}"
    );

    let m = Arc::new(DriveMemo::new(budget(), rules));
    // Leaf A: `c` minted outside any session — unrecorded, frozen.
    let c_outside = lit(2.0);
    let (_, a) = with_session_memo(budget(), rules, &m, || {
        zero(c_outside * c_outside - lit(4.0))
    });
    assert_eq!(
        decisions(a),
        (0, 1, 0),
        "leaf A freezes the unrecorded node: {a:?}"
    );
    // **The freeze is not on the column, and that is the column being
    // right**: `frozen` on a leaf is its NEED of the DRIVE's frozen set
    // (`SymCounts::frozen`), and an unrecorded node's freeze is never
    // published to that set — it is this leaf's own answer about a node
    // another leaf computes a real form for, which is what the
    // assertion below is about. The profile's `FreezeCause::Unrecorded`
    // is where an unrecorded freeze is counted, and
    // `editor-core`'s `no_leaf_of_a_drive_freezes_a_node_its_session_never_recorded`
    // pins that branch at zero over both measured documents.
    assert_eq!(
        a.frozen, 0,
        "an unrecorded freeze is in no drive's set: {a:?}"
    );

    // Leaf B: `c` minted inside its session — recorded, foldable.
    let (_, b) = with_session_memo(budget(), rules, &m, || {
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

/// **The same two leaves the other way round** — and the answer is
/// different, which is the point.
///
/// Leaf 1 RECORDS the node and publishes the real form. Leaf 2 does not
/// record it, and would have frozen it and decided numerically on its
/// own; with the memo it takes leaf 1's form at the recorded PARENT
/// (whose id is the same content hash in both leaves) and reaches the
/// theorem instead.
///
/// That direction is SOUND — the form is the plain form of the
/// expression that id names, and the tier's freeze was a loss, not an
/// answer — and it is strictly stronger than the leaf alone. What it is
/// not is order-INDEPENDENT: leaf 2's decision columns depend on
/// whether leaf 1 ran first, so a drive that produced an unrecorded
/// node would have a schedule-dependent receipt. It does not: no leaf
/// of a drive reaches that branch at all, which `editor-core`'s
/// `no_leaf_of_a_drive_freezes_a_node_its_session_never_recorded` pins
/// on both documents, and `sym::memo`'s header names as the condition.
#[test]
fn a_leaf_that_records_the_node_hands_a_later_leaf_a_theorem_it_would_have_missed() {
    let rules = SymRules {
        const_fold: true,
        ..SymRules::none()
    };
    let c_outside = lit(2.0);
    let alone = with_session_rules(budget(), rules, || zero(c_outside * c_outside - lit(4.0))).1;
    assert_eq!(
        decisions(alone),
        (0, 1, 0),
        "on its own, a leaf that did not record the node freezes it: {alone:?}"
    );

    let m = Arc::new(DriveMemo::new(budget(), rules));
    let (_, first) = with_session_memo(budget(), rules, &m, || {
        let c = lit(2.0);
        zero(c * c - lit(4.0))
    });
    assert_eq!(decisions(first), (1, 0, 0), "{first:?}");
    let (_, second) = with_session_memo(budget(), rules, &m, || {
        zero(c_outside * c_outside - lit(4.0))
    });
    assert_eq!(
        decisions(second),
        (1, 0, 0),
        "the leaf that did not record the node takes the recorded parent's published form and \
         reaches the theorem it would have missed alone ({alone:?} without the memo): {second:?}"
    );
}

/// **A leaf-varying opaque sequence moves no decision through the
/// memo** (R2, adopted): an `Opaque` id names a syntactic unknown, a
/// plain form is a syntactic normal form of a syntactic id, and a
/// `Zero` it answers is an identity in whatever unknowns the id's
/// syntax names — so two leaves that give one sequence number to two
/// different reals are still each handed a sound form. What a
/// leaf-varying mint costs is HITS (`sym::memo`'s header).
#[test]
fn a_leaf_varying_opaque_sequence_moves_no_decision_through_the_memo() {
    let m = memo();
    let leaf = |extra: bool| {
        with_session_memo(budget(), SymRules::shipped(), &m, || {
            if extra {
                // The stand-in for a value-dependent mint: one more
                // opaque before the real ones shifts every later
                // sequence number on this leaf.
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
    assert_eq!(r1, r0, "a shifted opaque sequence changed an answer");
    assert_eq!(r2, r0);
    assert_eq!(r3, r0);
    for c in [c0, c1, c2, c3] {
        assert_eq!(decisions(c), (2, 1, 0), "{c:?}");
    }
    println!("memo after four leaves with a leaf-varying opaque sequence: {m:?}");
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
            let x = Sym::param(ParamSymbol::of("x"), 0.37);
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
    let tight = SymBudget {
        max_terms: 2,
        max_degree: 128,
    };
    // The plain quotient form and the constant fold: no rule can
    // rewrite the sum into something that fits, so what freezes is the
    // budget's verdict on the sum itself.
    let rules = SymRules {
        const_fold: true,
        ..SymRules::none()
    };
    let m = Arc::new(DriveMemo::new(tight, rules));
    let p = |n: &str, v: f64| Sym::param(ParamSymbol::of(n), v);
    let three = || p("a", 0.25) + p("b", 0.5) + p("c", 0.75);

    // Leaf 1: builds the three-term sum, which does not fit, and
    // reaches it TWICE — once per decision, the second a second walk
    // over the same id.
    let (_, one) = with_session_memo(tight, rules, &m, || {
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
    let (_, two) = with_session_memo(tight, rules, &m, || zero(three() - three()));
    assert_eq!(
        two.frozen, 1,
        "the leaf that paid nothing needs exactly what leaf 1 needed: {two:?}"
    );

    // Leaf 3: reaches nothing that froze, with the drive's set
    // non-empty.
    let (_, three_) = with_session_memo(tight, rules, &m, || {
        let x = p("x", 0.125);
        zero(x - x)
    });
    assert_eq!(
        three_.frozen, 0,
        "a leaf that reached no frozen node needs none of them: {three_:?}"
    );
    assert_eq!(m.size().frozen, 1, "and the drive's set is still one node");
}
