//! **The drive's table at the tier's door**: what one leaf reads of
//! another's, what stays a leaf's own, and what a value from outside
//! every session on the table is.
//!
//! The dials are the plain walk with A0 and no early walk wherever a
//! row is about the plain form, so that the plain form is the whole
//! answer and nothing downstream re-folds it.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::sync::Arc;

use geom_core::k_stats::decide;
use geom_core::predicate::{Band, Margin, Sign};
use geom_core::sym::{DriveMemo, with_session_memo};
use geom_core::{Interval, ParamSymbol, Real, Sym, SymBudget, SymCounts, SymRules, Tol};

fn budget() -> SymBudget {
    SymBudget {
        max_terms: 4096,
        max_degree: 128,
    }
}

fn band() -> Band {
    Band::linear(Tol::witness()).expect("the witness tolerance has a linear band")
}

fn lit(v: f64) -> Sym<f64> {
    Sym::from_f64(v)
}

fn zero(m: Sym<f64>) -> bool {
    decide("a_drives_table", Margin::of(m), band()) == Ok(Sign::Zero)
}

fn decisions(c: SymCounts) -> (u64, u64) {
    (c.symbolic, c.numeric)
}

fn folding() -> SymRules {
    SymRules {
        const_fold: true,
        ..SymRules::none()
    }
}

fn folding_table() -> Arc<DriveMemo> {
    Arc::new(DriveMemo::new(budget(), folding()))
}

/// One leaf on `m`, handed `c`, deciding `c·c − 4`.
fn handed(m: &Arc<DriveMemo>, c: Sym<f64>) -> SymCounts {
    with_session_memo(budget(), folding(), m, || zero(c * c - lit(4.0))).1
}

/// One leaf on `m` that mints `c = 2` itself and decides `c·c − 4`.
fn minting(m: &Arc<DriveMemo>) -> SymCounts {
    with_session_memo(budget(), folding(), m, || {
        let c = lit(2.0);
        zero(c * c - lit(4.0))
    })
    .1
}

/// **A value built in one leaf of a drive resolves in another through
/// the drive's table**, where the second leaf holds no cache of the
/// first's: it starts while the first is still open, and builds an
/// expression over the first leaf's `c` that the first never built, so
/// neither a warm cache nor a served form answers for `c`.
///
/// Mutant: a table that names no node leaves `c` unresolved in the
/// second leaf, and its decision numeric.
#[test]
fn a_value_resolves_through_the_table_in_a_leaf_with_no_cache_of_its_maker() {
    let m = folding_table();
    let ((), _first) = with_session_memo(budget(), folding(), &m, || {
        let c = lit(2.0);
        zero(c * c - lit(4.0));
        let second = std::thread::scope(|s| {
            s.spawn(|| with_session_memo(budget(), folding(), &m, || zero(c * c * c - lit(8.0))).1)
                .join()
                .expect("the second leaf ran")
        });
        assert_eq!(
            decisions(second),
            (1, 0),
            "the second leaf reads `c` through the drive's table: {second:?}"
        );
    });
}

/// **A leaf's early forms are its box's.** Two leaves of one drive
/// build one expression, `(max(x, 0) − x)·t`, over opposite brackets
/// of `x`: over the positive one the early walk reads the maximum as
/// `x` and the margin is an identity; over the negative one it is
/// `−xt` and the tier proves nothing.
///
/// Mutant: a leaf handed an earlier leaf's early forms proves the
/// second margin zero, which it is not.
#[test]
fn a_leafs_early_forms_are_its_boxs() {
    let table = Arc::new(DriveMemo::new(budget(), SymRules::shipped()));
    let leaf = |lo: f64, hi: f64| {
        with_session_memo(budget(), SymRules::shipped(), &table, || {
            let over = |name: &str, lo: f64, hi: f64| {
                Sym::<Interval>::param_over(
                    ParamSymbol::new(test_utils::symbol_id(name)),
                    Interval::from_bounds(lo, hi),
                    lo,
                    hi,
                )
            };
            let (x, t) = (over("x", lo, hi), over("t", 0.0, 1.0e-13));
            let zero = Sym::<Interval>::from_f64(0.0);
            let _ = decide("a_drives_table", Margin::of((x.max(zero) - x) * t), band());
        })
        .1
    };
    let positive = leaf(1.0, 2.0);
    assert_eq!(decisions(positive), (1, 0), "over x > 0 it is an identity");
    let negative = leaf(-2.0, -1.0);
    assert_eq!(
        decisions(negative),
        (0, 1),
        "over x < 0 it is no identity, whatever the leaf before it built"
    );
}

/// **A value minted outside every session on the table is an unknown,
/// and proves nothing**: `c·c − 4` is numeric in the leaf handed `c`
/// from outside, where a leaf that mints `c` folds it to zero. The
/// unknown is among what the leaf's reasoning rests on.
#[test]
fn a_value_from_outside_every_session_is_an_unknown() {
    let got = handed(&folding_table(), lit(2.0));
    assert_eq!(decisions(got), (0, 1), "an unknown is no theorem: {got:?}");
    assert_eq!(got.frozen, 1, "the leaf's reasoning rests on it: {got:?}");
}

/// **What a leaf builds over an unknown is not published.** Leaf A is
/// handed `c` from outside; its form for `c·c` is built over the
/// unknown, under the content id a leaf that mints `c` gives the same
/// product. Leaf B runs on another thread while A is open, so it holds
/// none of A's caches; it mints `c` and folds `c·c − 4` to zero.
///
/// Mutant: publishing a form built over an unknown hands leaf B the
/// frozen product, and its decision is numeric.
#[test]
fn a_form_over_an_unknown_is_not_published() {
    let outside = lit(2.0);
    let m = folding_table();
    let ((), _a) = with_session_memo(budget(), folding(), &m, || {
        zero(outside * outside - lit(4.0));
        let b = std::thread::scope(|s| s.spawn(|| minting(&m)).join().expect("leaf B ran"));
        assert_eq!(decisions(b), (1, 0), "leaf B builds the real form: {b:?}");
    });
    assert_eq!(
        m.size().frozen,
        0,
        "no freeze of the unknown is in the drive's set"
    );
}

/// **What a leaf builds over an unknown does not stay in the caches
/// the drive keeps warm.** Leaf A is handed `c` from outside; a second
/// leaf on the same thread starts from A's caches and reads what A
/// read; a third mints `c` itself and folds the product.
///
/// Mutant: a frozen form left in the warm cache answers for `c·c` in
/// the third leaf, and its decision is numeric.
#[test]
fn a_form_over_an_unknown_is_not_kept_warm() {
    let outside = lit(2.0);
    let m = folding_table();
    assert_eq!(decisions(handed(&m, outside)), (0, 1));
    // The table still names no `c`: checked, so the next leaf's fold
    // is its own minting and not a resolution through the table.
    assert_eq!(
        m.size().nodes,
        3,
        "the product, the literal 4 and the difference"
    );
    assert_eq!(
        decisions(minting(&m)),
        (1, 0),
        "a leaf that mints `c` folds"
    );
}

/// **What is left of the order between leaves, pinned as it stands.**
/// An id is its content's hash, so once ANY leaf of the drive has
/// minted `c = 2` inside a session, a leaf handed a `c` from outside
/// every session resolves it through the table and folds. The answer
/// is sound either way (the id names that content) and it is the
/// stronger one after the minting leaf: an order dependence between
/// leaves, reached only by a value from outside every session
/// (`sym::memo`'s header).
#[test]
fn an_outside_value_resolves_once_a_leaf_has_minted_its_content() {
    let outside = lit(2.0);
    let m = folding_table();
    assert_eq!(
        decisions(handed(&m, outside)),
        (0, 1),
        "before any leaf minted it"
    );
    let _ = with_session_memo(budget(), folding(), &m, || lit(2.0));
    assert_eq!(
        decisions(handed(&m, outside)),
        (1, 0),
        "after a leaf minted it"
    );
}

/// **A leaf that panics leaves no session installed.** The thread's
/// next session opens as any other and decides.
///
/// Mutant: a session taken only after its work returns stays installed
/// through the unwind, and the next session is refused as nested.
#[test]
fn a_leaf_that_panics_leaves_no_session_installed() {
    let m = folding_table();
    let unwound = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        with_session_memo(budget(), folding(), &m, || {
            let _ = lit(2.0);
            panic!("the leaf's own failure");
        })
    }));
    assert!(unwound.is_err(), "the leaf panicked");
    assert!(
        !geom_core::sym::session_installed(),
        "no session is left on the thread"
    );
    assert_eq!(decisions(minting(&m)), (1, 0), "and the next leaf decides");
}

/// **The structural question reads a node where the leaf does**
/// ([`geom_core::Expression::proves_zero`]): a leaf asks whether
/// `d − x² − 2x − 1` is zero, where `d = (x + 1)²` was built by another
/// leaf of the drive that is still open, so `d` is in the drive's table
/// and in no cache of the asking leaf's. The ask's state is its own and
/// holds no form of the drive's; the node it resolves is the table's.
///
/// Mutant: an ask that reads the installed session's node cache alone
/// finds no `d`, freezes it as an unknown, and proves nothing.
#[test]
fn the_structural_question_resolves_a_node_another_leaf_recorded() {
    use geom_core::Expression;
    let x = || Sym::<f64>::param(ParamSymbol::new(test_utils::symbol_id("x")), 0.5);
    let m = Arc::new(DriveMemo::new(budget(), SymRules::shipped()));
    let ((), _first) = with_session_memo(budget(), SymRules::shipped(), &m, || {
        let d = (x() + lit(1.0)) * (x() + lit(1.0));
        let proved = std::thread::scope(|s| {
            s.spawn(|| {
                with_session_memo(budget(), SymRules::shipped(), &m, || {
                    let residual = d - x() * x() - lit(2.0) * x() - lit(1.0);
                    residual
                        .expression()
                        .and_then(Expression::proves_zero)
                        .is_some()
                })
                .0
            })
            .join()
            .expect("the asking leaf ran")
        });
        assert!(proved, "the ask reads `d` through the drive's table");
    });
}
