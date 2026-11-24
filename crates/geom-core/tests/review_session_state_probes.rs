//! Reviewer rows for the session-state unit's delta round, at the
//! tier's door.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::sync::Arc;

use geom_core::predicate::{Band, Margin};
use geom_core::sym::{MintScope, NodeRecord, with_session_retry};
use geom_core::{ParamSymbol, Real, Sym, SymBudget, SymCounts, SymRetry, SymRules, Tol};

fn budget() -> SymBudget {
    SymBudget {
        max_terms: 4096,
        max_degree: 128,
    }
}

fn decide(m: Sym<f64>) {
    let band = Band::new(1.0e-9, 1.0e-8).unwrap();
    let _ = geom_core::k_stats::decide("review_session_state", Margin::of(m), band);
}

fn p(name: &str, v: f64) -> Sym<f64> {
    Sym::<f64>::param(ParamSymbol::new(test_utils::symbol_id(name)), v)
}

/// `work` as the node `name`, reading `read` in the order given.
fn as_node(name: u64, read: &[&Arc<NodeRecord>], work: impl FnOnce()) -> Arc<NodeRecord> {
    let (_, record) = MintScope::current().child(&[name]).node([], || {
        geom_core::sym::reads(read.iter().copied());
        work();
    });
    Arc::new(record)
}

fn columns(c: SymCounts) -> (u64, u64) {
    (c.symbolic, c.numeric)
}

/// **The order two records are read in decides an answer.** Node `A`
/// states `u ≡ sin(v)`, node `B` states `v ≡ atan2(u, c)`; each is true
/// at the values, and either closes a cycle through the other, so a
/// reader keeps the one it reads first. A node that reads `A` then `B`
/// proves `u − sin(v)` through the door; one that reads `B` then `A`
/// holds `v ≡ atan2(u, c)` alone and does not.
#[test]
fn the_order_two_records_are_read_in_decides_an_answer() {
    let v0 = 0.5_f64;
    let (u, v, c) = (
        move || p("u", v0.sin()),
        move || p("v", v0),
        move || p("c", v0.cos()),
    );
    let ((a_then_b, b_then_a, reads), _) =
        with_session_retry(budget(), SymRules::shipped(), SymRetry::none(), || {
            let a = as_node(1, &[], || {
                let _ = u().register_equal(v().sin_cos().0, Tol::witness());
            });
            let b = as_node(2, &[], || {
                let _ = v().register_equal(u().atan2(c()), Tol::witness());
            });
            assert_eq!((a.registrations(), b.registrations()), (1, 1), "each states its identity");
            let ask = || decide(u() - v().sin_cos().0);
            let first = as_node(3, &[&a, &b], ask);
            let second = as_node(4, &[&b, &a], ask);
            (
                columns(first.counts()),
                columns(second.counts()),
                (first.read(), second.read()),
            )
        });
    println!("A then B: {a_then_b:?}; B then A: {b_then_a:?}; identities read {reads:?}");
    assert_eq!(a_then_b, (1, 0), "read A first: the door proves it");
    assert_eq!(b_then_a, (0, 1), "read B first: A's identity is the one the cycle drops");
}

/// **Two equal records at two addresses are both read, and the
/// reader's record counts both.** One record read twice enters once.
#[test]
fn two_equal_records_at_two_addresses_are_both_counted() {
    let (counts, _) = with_session_retry(budget(), SymRules::shipped(), SymRetry::none(), || {
        let a = as_node(1, &[], || {
            let _ = p("u", 0.5_f64.sin()).register_equal(p("v", 0.5).sin_cos().0, Tol::witness());
        });
        let copy = Arc::new((*a).clone());
        assert_eq!(*a, *copy, "equal records");
        let same = as_node(2, &[&a, &a], || {});
        let twice = as_node(3, &[&a, &copy], || {});
        println!("equal as records: {}", *same == *twice);
        (same.read(), twice.read())
    });
    assert_eq!(counts, (1, 2), "one address is read once; an equal record elsewhere is read again");
}
