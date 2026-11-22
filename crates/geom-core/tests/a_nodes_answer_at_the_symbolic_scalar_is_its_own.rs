//! **What a node decides at the symbolic scalar does not depend on a
//! node that ran before it and feeds it nothing.**
//!
//! Each row runs one node `B` twice in a session, once alone and once
//! after an unrelated node `A`, and compares what `B` decided. A node
//! here is a mint scope, as the evaluation service opens one per
//! document node.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::predicate::{Band, Margin};
use geom_core::sym::{MintScope, session_counts, with_session_retry};
use geom_core::{ParamSymbol, Real, Sym, SymBudget, SymCounts, SymRetry, SymRules, Tol};

fn budget() -> SymBudget {
    SymBudget {
        max_terms: 4096,
        max_degree: 128,
    }
}

fn band() -> Band {
    Band::new(1.0e-9, 1.0e-8).unwrap()
}

fn decide<T: geom_core::Decide>(m: Sym<T>) {
    let _ = geom_core::k_stats::decide("a_nodes_answer", Margin::of(m), band());
}

/// `[symbolic, numeric, retried]` decided while `work` ran, as the node
/// `name`.
fn as_node(name: u64, work: impl FnOnce()) -> [u64; 3] {
    let _scope = MintScope::current().child(&[name]).enter();
    let columns = |c: SymCounts| [c.symbolic, c.numeric, c.retried];
    let before = columns(session_counts().unwrap());
    work();
    let after = columns(session_counts().unwrap());
    [0, 1, 2].map(|i| after[i] - before[i])
}

/// What `b` decides alone, and what it decides after `a`.
fn alone_and_after(retry: SymRetry, a: impl Fn(), b: impl Fn()) -> ([u64; 3], [u64; 3]) {
    let session = |with_a: bool| {
        with_session_retry(budget(), SymRules::shipped(), retry, || {
            if with_a {
                as_node(1, &a);
            }
            as_node(2, &b)
        })
        .0
    };
    (session(false), session(true))
}

/// **An identity a node registered is not read by a node it does not
/// feed.** `A` registers `sin(u) ≡ w`; `B` builds the same two
/// expressions from the parameters and decides their difference, which
/// the algebra does not prove.
///
/// Mutant: a node's scope opened over the registry of every node that
/// ran before it turns `B` after `A` into a proof.
#[test]
fn a_registration_is_not_read_by_a_node_it_does_not_feed() {
    let p = |name: &str, v: f64| Sym::<f64>::param(ParamSymbol::new(test_utils::symbol_id(name)), v);
    let sin_u = move || p("u", 0.5).sin_cos().0;
    let w = move || p("w", 0.5_f64.sin());
    let (alone, after) = alone_and_after(
        SymRetry::none(),
        || {
            let recorded = sin_u().register_equal(w(), Tol::witness());
            assert_eq!(recorded, geom_core::sym::SymRegistration::Recorded);
        },
        || decide(sin_u() - w()),
    );
    assert_eq!(alone, [0, 1, 0], "alone, the difference is numeric");
    assert_eq!(after, alone, "after a node that registered the identity");
}

/// **The retry ladder's growth guard is a node's own.** Each node
/// decides a margin only a 512-bit retry closes, under a guard of one
/// form: `A`'s retry fills its memos past the guard.
///
/// Mutant: one guard over the replay's retry memos withholds `B`'s
/// attempt after `A`, and `B` stays numeric.
#[test]
fn the_retry_guard_counts_a_nodes_own_forms() {
    let two_spellings = |n: usize, a: usize| {
        let c = || Sym::<f64>::from_f64(1.0 / 3.0);
        let chain = |k: usize| (1..k).fold(c(), |acc, _| acc * c());
        chain(n) - chain(a) * chain(n - a)
    };
    let guarded = SymRetry {
        bits: Some(512),
        max_forms: 1,
        ..SymRetry::none()
    };
    let (alone, after) = alone_and_after(
        guarded,
        || decide(two_spellings(5, 2)),
        || decide(two_spellings(6, 2)),
    );
    assert_eq!(alone, [1, 0, 1], "alone, the retry closes it");
    assert_eq!(after, alone, "after a node whose retry passed the guard");
}
