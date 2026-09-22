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
    Sym::param(ParamSymbol::of(n), v)
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
    // **The freeze is on A's column although no drive holds it.** A's
    // `frozen` is its NEED (`SymCounts::frozen`), and a freeze a leaf
    // cannot publish is still one its own reasoning rested on, so it is
    // counted on the leaf's own side of the union — which is what makes
    // the reading the same whichever leaf ran first
    // (`a_leafs_need_does_not_move_with_the_order_across_the_unrecorded_branch`
    // walks both orders). `editor-core`'s
    // `no_leaf_of_a_drive_freezes_a_node_its_session_never_recorded`
    // pins the branch at zero over every drive measured.
    assert_eq!(
        a.frozen, 1,
        "the freeze A could not publish is still A's own need: {a:?}"
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

/// **THE UNRECORDED BRANCH, in both orders**: the same leaf, the same
/// box, reads the same column whichever leaf of the drive ran first.
///
/// Leaf U mints the freezing node OUTSIDE its session, so the node is
/// unrecorded there: the plain walk freezes it, taints everything above
/// it and publishes NOTHING (`sym::memo`'s unrecorded paragraph). The
/// node is nevertheless in U's closure — it is a child of a recorded
/// parent — so an intersection with the drive's set ALONE would read it
/// as 0 when U ran first and 1 once the recording leaf had published
/// the same freeze. That is a reading of the schedule, and it is why a
/// leaf's NEED unions its own unpublishable freezes with the drive's
/// set: U needs the node in both orders because U froze it in both.
///
/// Both reviewers of SYM-13 demonstrated the dependence, each with a
/// row of their own; this is those two rows as the claim that closes
/// them.
#[test]
fn a_leafs_need_does_not_move_with_the_order_across_the_unrecorded_branch() {
    // Order A: the leaf that does not record the node runs FIRST.
    let outside_a = three();
    let m_a = tight_memo();
    let u_first = leaf(&m_a, || zero(outside_a - outside_a));
    assert_eq!(
        m_a.size().frozen,
        0,
        "U's freeze is not published: {:?}",
        m_a.size()
    );
    let r_after = leaf(&m_a, || zero(three() - three()));

    // Order B: the RECORDING leaf runs first and publishes the freeze.
    let outside_b = three();
    let m_b = tight_memo();
    let r_first = leaf(&m_b, || zero(three() - three()));
    let u_after = leaf(&m_b, || zero(outside_b - outside_b));

    assert_eq!(m_a.size().frozen, m_b.size().frozen, "same drive set size");
    assert_eq!(
        (u_first.frozen, u_after.frozen),
        (1, 1),
        "the leaf that could not publish reads the same in both orders: \
         {u_first:?} {u_after:?}"
    );
    assert_eq!(
        (r_after.frozen, r_first.frozen),
        (1, 1),
        "and so does the leaf that could: {r_after:?} {r_first:?}"
    );
    // The same leaf, twice in one drive, before and after the leaf that
    // records the node — the second reviewer's shape of the row.
    let m_c = tight_memo();
    let outside_c = three();
    let before = leaf(&m_c, || zero(outside_c - outside_c));
    let recorder = leaf(&m_c, || zero(three() - three()));
    let after = leaf(&m_c, || zero(outside_c - outside_c));
    assert_eq!(
        (before.frozen, recorder.frozen, after.frozen),
        (1, 1, 1),
        "before and after the recording leaf: {before:?} {recorder:?} {after:?}"
    );
    assert_eq!(
        (before.symbolic_zero, before.numeric),
        (after.symbolic_zero, after.numeric),
        "and no decision column moved either"
    );
}

/// **A TAINTED freeze is the leaf's own need too, in either order**:
/// not only the unrecorded node itself but the recorded ancestor whose
/// form this leaf built out of it, which the taint guard also keeps out
/// of the memo.
///
/// `outside + d + e` over an unrecorded `outside`: the leaf freezes the
/// unrecorded node AND the three-term sum above it, publishes neither,
/// and needs both. A leaf that recorded the node freezes and publishes
/// the same two ids — the sum is over the budget either way — so the
/// union counts two whichever ran first.
#[test]
fn a_tainted_freeze_is_the_leafs_own_need_in_either_order() {
    let over = |base: Sym<f64>| base + p("d", 2.0) + p("e", 3.0);

    let m_a = tight_memo();
    let outside_a = three();
    let tainted_first = leaf(&m_a, || zero(over(outside_a) - over(outside_a)));
    assert_eq!(
        m_a.size().frozen,
        0,
        "a tainted freeze is published no more than an unrecorded one: {:?}",
        m_a.size()
    );
    let recorder_after = leaf(&m_a, || zero(over(three()) - over(three())));

    let m_b = tight_memo();
    let recorder_first = leaf(&m_b, || zero(over(three()) - over(three())));
    let outside_b = three();
    let tainted_after = leaf(&m_b, || zero(over(outside_b) - over(outside_b)));

    assert_eq!(
        m_b.size().frozen,
        2,
        "the recording leaf publishes both freezes: {:?}",
        m_b.size()
    );
    assert_eq!(
        (tainted_first.frozen, tainted_after.frozen),
        (2, 2),
        "the tainted leaf needs both in either order: {tainted_first:?} {tainted_after:?}"
    );
    assert_eq!(
        (recorder_after.frozen, recorder_first.frozen),
        (2, 2),
        "and so does the recording leaf: {recorder_after:?} {recorder_first:?}"
    );
}

/// **The inherit branch moves a DECISION while the column stands
/// still.** The first reviewer's delta probe, adopted.
///
/// `outside` is `2.0`, minted outside the session, and the decision is
/// `outside · outside − 4`. A leaf that does not record it freezes it
/// and answers numerically; the same leaf, run after one that DID
/// record it, takes the published form for the product and proves the
/// identity — the stronger answer `sym::memo`'s header describes. Its
/// NEED is 1 in both orders: what its reasoning could not resolve is
/// the id its table does not hold, which is the same id whichever
/// order ran.
#[test]
fn the_inherit_branch_moves_the_decision_while_the_column_stands_still() {
    let outside = lit(2.0);
    let m_a = tight_memo();
    let u_first = leaf(&m_a, || zero(outside * outside - lit(4.0)));

    let m_b = tight_memo();
    let _recorder = leaf(&m_b, || {
        let c = lit(2.0);
        zero(c * c - lit(4.0))
    });
    let u_after = leaf(&m_b, || zero(outside * outside - lit(4.0)));

    assert_ne!(
        decisions(u_first),
        decisions(u_after),
        "the decision column moves on this branch: {u_first:?} {u_after:?}"
    );
    assert_eq!(
        (u_first.frozen, u_after.frozen),
        (1, 1),
        "and the leaf's NEED does not: {u_first:?} {u_after:?}"
    );
}

/// **An inherited form does not move the leaf's NEED**, with every
/// decision column standing still — the first reviewer's delta probe
/// that found the fix-pass gap, adopted as the claim that closes it.
///
/// `P = outside · outside` with `outside` minted outside the session,
/// and the decision is `P − P`, an identity whatever `P`'s form is, so
/// no decision column can move. Before the leaf's own side was counted
/// from its TABLE, this read 1 when the leaf ran first (it froze the
/// unrecorded node itself) and 0 when it ran after a leaf that
/// recorded the node (it took the published form for `P` and never
/// walked below it). It reads 1 in both orders now, and in both orders
/// with the drive's frozen set non-empty — the reviewer's third probe,
/// which differs only in seeding the set, is the second arm here.
#[test]
fn an_inherited_form_does_not_move_the_leafs_need() {
    let outside = lit(2.0);
    let identity = move || zero(outside * outside - outside * outside);

    // Arm 1: nothing else froze over either drive.
    let m_a = tight_memo();
    let u_first = leaf(&m_a, identity);
    let m_b = tight_memo();
    let recorder = leaf(&m_b, || {
        let c = lit(2.0);
        zero(c * c - lit(4.0))
    });
    let u_after = leaf(&m_b, identity);
    assert_eq!(
        decisions(u_first),
        decisions(u_after),
        "every decision column is the same in both orders: {u_first:?} {u_after:?}"
    );
    assert_eq!(
        (u_first.frozen, u_after.frozen, recorder.frozen),
        (1, 1, 0),
        "and so is the column: {u_first:?} {u_after:?} {recorder:?}"
    );

    // Arm 2: the same, with the drive's set non-empty in both orders,
    // so neither reading is the nothing-froze short circuit.
    let m_c = tight_memo();
    let seed_c = leaf(&m_c, || zero(three() - three()));
    let u_first_seeded = leaf(&m_c, identity);
    let m_d = tight_memo();
    let _seed_d = leaf(&m_d, || zero(three() - three()));
    let _recorder_d = leaf(&m_d, || {
        let c = lit(2.0);
        zero(c * c - lit(4.0))
    });
    let u_after_seeded = leaf(&m_d, identity);
    assert_eq!(
        (m_c.size().frozen, m_d.size().frozen),
        (1, 1),
        "the set is non-empty in both orders"
    );
    assert_eq!(
        (seed_c.frozen, u_first_seeded.frozen, u_after_seeded.frozen),
        (1, 1, 1),
        "the seeded node is the seeding leaf's need and neither of the others': \
         {seed_c:?} {u_first_seeded:?} {u_after_seeded:?}"
    );
}

/// **The one reading the order still moves**: a freeze the TAINT
/// caused, under a hit — `work/sym/a-taint-induced-freeze-under-a-hit-still-reads-by-order`.
///
/// `outside` is `0.0` and minted outside the session. A leaf that
/// RECORDS it folds it away, so `0 + a + b` is two terms and fits, and
/// **no leaf ever publishes a freeze of that sum**. A leaf that does
/// not record it carries an indeterminate in the zero's place, which
/// makes the sum three terms and freezes it — a freeze the taint
/// caused, which no drive holds and which the leaf's table cannot
/// predict. Run first, that leaf makes the freeze and counts it; run
/// after the recording leaf, it takes the published form at the
/// decision root and never walks there.
///
/// The row pins the reading rather than a repair: closing it would
/// mean computing the form the memo just handed the leaf, which is the
/// memo. Both readings are true of what the leaf did — its reasoning
/// really is the stronger one in the second order — and the branch is
/// pinned at zero on every drive measured (`editor-core`'s
/// `no_leaf_of_a_drive_freezes_a_node_its_session_never_recorded`).
#[test]
fn a_taint_induced_freeze_under_a_hit_is_read_by_order() {
    let sum = |base: Sym<f64>| base + p("a", 0.25) + p("b", 0.5);

    let m_a = tight_memo();
    let outside_a = lit(0.0);
    let tainted_first = leaf(&m_a, || zero(sum(outside_a) - sum(outside_a)));

    let m_b = tight_memo();
    let _recorder = leaf(&m_b, || {
        let c = lit(0.0);
        zero(sum(c) - sum(c))
    });
    let outside_b = lit(0.0);
    let tainted_after = leaf(&m_b, || zero(sum(outside_b) - sum(outside_b)));

    assert_eq!(
        (m_a.size().frozen, m_b.size().frozen),
        (0, 0),
        "no leaf publishes a freeze of the folded sum: {:?} {:?}",
        m_a.size(),
        m_b.size()
    );
    assert_eq!(
        (tainted_first.frozen, tainted_after.frozen),
        (2, 1),
        "the taint-induced freeze is counted only where the walk reached it: \
         {tainted_first:?} {tainted_after:?}"
    );
}
