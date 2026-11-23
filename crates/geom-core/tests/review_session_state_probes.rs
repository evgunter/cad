//! Reviewer rows for the session-state unit, at the tier's door.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::sync::Arc;

use geom_core::predicate::{Band, Margin};
use geom_core::sym::{DriveMemo, with_session_memo};
use geom_core::{Interval, ParamSymbol, Real, Sym, SymBudget, SymCounts, SymRules, Tol};

fn budget() -> SymBudget {
    SymBudget {
        max_terms: 4096,
        max_degree: 128,
    }
}

fn decide(m: Sym<f64>) {
    let band = Band::linear(Tol::witness()).expect("linear band");
    let _ = geom_core::k_stats::decide("review_session_state", Margin::of(m), band);
}

fn columns(c: SymCounts) -> (u64, u64) {
    (c.symbolic, c.numeric)
}

/// **A leaf's early forms are its box's.** Two leaves of one drive
/// build one expression, `(max(x, 0) − x)·t`, over opposite brackets of `x`:
/// over the positive one the early walk reads the maximum as `x` and the
/// margin is an identity; over the negative one it is `−xt` and the
/// tier proves nothing.
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
            let band = Band::linear(Tol::witness()).expect("linear band");
            let _ = geom_core::k_stats::decide(
                "review_session_state",
                Margin::of((x.max(zero) - x) * t),
                band,
            );
        })
        .1
    };
    let positive = leaf(1.0, 2.0);
    assert_eq!(columns(positive), (1, 0), "over x > 0 it is an identity: {positive:?}");
    let negative = leaf(-2.0, -1.0);
    assert_eq!(
        columns(negative),
        (0, 1),
        "over x < 0 it is no identity, whatever the leaf before it built: {negative:?}"
    );
}

/// **A value built in one leaf of a drive resolves in another through
/// the drive's table**, where the second leaf holds no cache of the
/// first's: it starts while the first is still open, and builds an
/// expression over the first leaf's `c` that the first never built, so
/// neither a warm cache nor a served form answers for `c`.
///
/// Mutant: a table that names no node leaves `c` unresolved in the
/// second leaf.
#[test]
fn a_value_resolves_through_the_table_in_a_leaf_with_no_cache_of_its_maker() {
    let rules = SymRules {
        const_fold: true,
        ..SymRules::none()
    };
    let table = Arc::new(DriveMemo::new(budget(), rules));
    let ((), _first) = with_session_memo(budget(), rules, &table, || {
        let c = Sym::<f64>::from_f64(2.0);
        decide(c * c - Sym::from_f64(4.0));
        let second = std::thread::scope(|s| {
            s.spawn(|| {
                with_session_memo(budget(), rules, &table, || {
                    decide(c * c * c - Sym::from_f64(8.0));
                })
                .1
            })
            .join()
            .expect("the second leaf ran")
        });
        assert_eq!(
            columns(second),
            (1, 0),
            "the second leaf reads `c` through the drive's table: {second:?}"
        );
    });
}
