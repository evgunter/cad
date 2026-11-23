//! **A value with no expression is named by the scope that minted it
//! and its ordinal there** (`geom_core::sym::mint`): the same name in
//! every session for one scope's k-th mint, another name at another
//! ordinal or in another scope, and no name twice in one session.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::predicate::{Band, Sign};
use geom_core::sym::{MintScope, with_session};
use geom_core::{Decide, Interval, Sym, SymBudget, SymId, Tol};

fn budget() -> SymBudget {
    SymBudget {
        max_terms: 256,
        max_degree: 128,
    }
}

fn session<R>(f: impl FnOnce() -> R) -> (R, geom_core::SymCounts) {
    with_session(budget(), f)
}

fn mint(x: f64) -> Sym<Interval> {
    Sym::opaque(Interval::from_bounds(x, x))
}

/// `words`' scope under the one open on this thread, opened for `f`.
fn scoped<R>(words: &[u64], f: impl FnOnce() -> R) -> R {
    let _open = MintScope::current().child(words).enter();
    f()
}

fn sign_of(m: Sym<Interval>) -> Result<Sign, ()> {
    m.sign_within(Band::linear(Tol::witness()).unwrap())
        .map(|d| d.sign)
        .map_err(|_| ())
}

/// One scope's k-th mint has one name in two sessions, whatever else
/// each session minted first and whatever number it holds; two ordinals
/// of a scope and two scopes are different names.
///
/// Mutants: a name taken from one count for the whole session (the
/// first session's root mint shifts every later name, so `in_a` differs
/// between the sessions); a name without its ordinal (`in_a[0] ==
/// in_a[1]`).
#[test]
fn a_scopes_kth_mint_has_one_name_in_every_session() {
    type Names = (SymId, [SymId; 2], SymId);
    let replay = |before: bool, x: f64| -> Names {
        session(|| {
            // A mint under no opened scope, in one replay and not the
            // other.
            let root = mint(9.0).node();
            if before {
                let _ = mint(8.0);
            }
            let in_a = scoped(&[1], || [mint(x).node(), mint(x).node()]);
            let in_b = scoped(&[2], || mint(x).node());
            (root, in_a, in_b)
        })
        .0
    };
    let (root, in_a, in_b) = replay(true, 1.0);
    assert_eq!(
        replay(false, 5.0),
        (root, in_a, in_b),
        "the names are the scopes' and the ordinals', not the session's or the numbers'"
    );
    let all = [root, in_a[0], in_a[1], in_b];
    for (i, a) in all.iter().enumerate() {
        for b in &all[i + 1..] {
            assert_ne!(a, b, "two mints of one session, two names");
        }
    }
}

/// Inside one session no name is minted twice, so two values with no
/// expression never cancel: a scope opened again, from outside or from
/// inside itself or its own child, mints on from where it stopped.
///
/// Mutant: a scope that counts from zero each time it is opened inside
/// a session gives `1.0` and `2.0` one name, and every row here decides
/// `Zero`.
#[test]
fn a_scope_opened_twice_in_one_session_mints_no_name_twice() {
    let (rows, counts) = session(|| {
        let first = scoped(&[1], || mint(1.0));
        let again = scoped(&[1], || mint(2.0));
        let (outer, inner, after) = scoped(&[3], || {
            let outer = mint(1.0);
            // The scope `[3]` itself, entered from inside itself.
            let inner = {
                let _same = MintScope::current().enter();
                mint(2.0)
            };
            (outer, inner, mint(3.0))
        });
        let (parent, from_child) = {
            let parent_scope = MintScope::current().child(&[4]);
            let _open = parent_scope.enter();
            let parent = mint(1.0);
            let from_child = scoped(&[5], || {
                let _parent_again = parent_scope.enter();
                mint(2.0)
            });
            (parent, from_child)
        };
        [
            ("opened twice", sign_of(first - again)),
            ("entered from inside itself", sign_of(outer - inner)),
            ("and after it", sign_of(inner - after)),
            ("entered from its child", sign_of(parent - from_child)),
        ]
    });
    for (what, got) in rows {
        assert_eq!(got, Ok(Sign::Negative), "{what}: the numbers decide");
    }
    assert_eq!(counts.symbolic, 0, "no identity was there to prove");
}

/// A value minted in one scope in one session and a value minted in
/// another scope in a second session are two unknowns where they meet.
/// (Two sessions' mints at ONE scope and ordinal share a name, and it
/// is the evaluation service's content key that keeps them apart:
/// `editor-core`'s `a_symbolic_nodes_key_is_its_expression_and_its_number`.)
///
/// Mutant: a name that is the session's own count (each session's first
/// mint is number zero) makes the carried value and the fresh one the
/// same unknown, and their difference `Zero`.
#[test]
fn two_scopes_mints_from_two_sessions_do_not_cancel() {
    let (carried, _) = session(|| scoped(&[1], || mint(1.0)));
    let (sign, counts) = session(|| sign_of(carried - scoped(&[2], || mint(2.0))));
    assert_eq!(sign, Ok(Sign::Negative), "the numbers decide");
    assert_eq!(counts.symbolic, 0);
}

/// A value minted under no scope before a session and the session's
/// first such value are two unknowns: the session's root scope is not
/// the thread's.
///
/// Mutant: a session whose root scope is the thread's own gives both
/// the root's first name on a thread that has minted nothing else, and
/// the row runs on a thread of its own so that it has not; their
/// difference is then a proven `Zero`.
#[test]
fn a_value_minted_before_a_session_and_the_sessions_first_do_not_cancel() {
    let on_a_fresh_thread = std::thread::spawn(|| {
        let before = mint(1.0);
        session(|| {
            let inside = mint(2.0);
            (before.node() == inside.node(), sign_of(before - inside))
        })
    });
    let ((same_name, sign), counts) = on_a_fresh_thread.join().unwrap();
    assert!(!same_name, "two mints, two names");
    assert_eq!(sign, Ok(Sign::Negative), "the numbers decide");
    assert_eq!(counts.symbolic, 0, "no identity was there to prove");
}

/// A detached map names each unit's mints by the unit's index under the
/// caller's scope, so the names are the same in two runs and under any
/// schedule, and two units' names differ.
///
/// Mutant: a unit minting in its worker's own root scope takes whatever
/// count that thread has reached, and the second run's names differ
/// from the first's.
#[test]
fn a_detached_maps_units_mint_under_their_indices() {
    let run = || -> Vec<SymId> {
        scoped(&[7], || {
            geom_core::k_stats::map_detached(&[1.0, 2.0, 3.0, 4.0], |&x| mint(x).node())
                .into_iter()
                .map(|(id, _)| id)
                .collect()
        })
    };
    let first = run();
    assert_eq!(run(), first, "two runs, one name per unit");
    for (i, a) in first.iter().enumerate() {
        for b in &first[i + 1..] {
            assert_ne!(a, b, "two units, two names");
        }
    }
}
