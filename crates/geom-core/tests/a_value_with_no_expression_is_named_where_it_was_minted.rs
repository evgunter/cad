//! **A value with no expression has a name no other value it can meet
//! has** (`geom_core::sym::mint`). Inside a session the name is the
//! mint scope's and the ordinal's: the same in every session for one
//! scope's k-th mint, another at another ordinal or in another scope,
//! and never given twice in one session. Outside a session it is the
//! thread's own, given once.
//!
//! Every value here is one unknown real somewhere in `[0, 1]`: the
//! difference of two encloses zero far outside the band, so the numbers
//! refuse and only the names can decide. Two names: refused. One name:
//! a proven `Zero` of two reals nobody said were equal.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::predicate::{Band, Sign};
use geom_core::sym::{MintScope, with_session};
use geom_core::{Decide, Interval, Sym, SymBudget, SymId, Tol};

fn session<R>(f: impl FnOnce() -> R) -> (R, geom_core::SymCounts) {
    with_session(
        SymBudget {
            max_terms: 256,
            max_degree: 128,
        },
        f,
    )
}

fn mint() -> Sym<Interval> {
    Sym::opaque(Interval::from_bounds(0.0, 1.0))
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

/// What a session of its own decides of `a − b`, and how many
/// identities it proved deciding it.
fn met(a: Sym<Interval>, b: Sym<Interval>) -> (Result<Sign, ()>, u64) {
    let (sign, counts) = session(|| sign_of(a - b));
    (sign, counts.symbolic)
}

/// Two unknowns, told apart: the numbers refuse and nothing is proved.
const APART: (Result<Sign, ()>, u64) = (Err(()), 0);

/// A thread that has minted nothing: the count outside a session runs
/// for the life of its thread.
fn fresh<R: Send + 'static>(f: impl FnOnce() -> R + Send + 'static) -> R {
    std::thread::spawn(f).join().unwrap()
}

fn all_differ(ids: &[SymId], what: &str) {
    for (i, a) in ids.iter().enumerate() {
        for b in &ids[i + 1..] {
            assert_ne!(a, b, "{what}: two mints, two names");
        }
    }
}

// ------------------------------------------------- inside a session

/// One scope's k-th mint has one name in two sessions, whatever else
/// each session minted first; two ordinals of a scope and two scopes
/// are different names.
///
/// Mutants: a name taken from one count for the whole session (the
/// first session's extra root mint shifts every later name, so `in_a`
/// differs between the sessions); a name without its ordinal
/// (`in_a[0] == in_a[1]`).
#[test]
fn a_scopes_kth_mint_has_one_name_in_every_session() {
    type Names = (SymId, [SymId; 2], SymId);
    let replay = |before: bool| -> Names {
        session(|| {
            let root = mint().node();
            // A mint under no opened scope, in one replay and not the
            // other.
            if before {
                let _ = mint();
            }
            let in_a = scoped(&[1], || [mint().node(), mint().node()]);
            let in_b = scoped(&[2], || mint().node());
            (root, in_a, in_b)
        })
        .0
    };
    let (root, in_a, in_b) = replay(true);
    assert_eq!(
        replay(false),
        (root, in_a, in_b),
        "the names are the scopes' and the ordinals', not the session's"
    );
    all_differ(&[root, in_a[0], in_a[1], in_b], "one session");
}

/// Inside one session no name is minted twice: a scope opened again,
/// from outside or from inside itself or its own child, mints on from
/// where it stopped.
///
/// Mutant: a scope that counts from zero each time it is opened inside
/// a session gives two values one name, and each pair here is a proven
/// `Zero`.
#[test]
fn a_scope_opened_twice_in_one_session_mints_no_name_twice() {
    let (rows, counts) = session(|| {
        let first = scoped(&[1], mint);
        let again = scoped(&[1], mint);
        let (outer, inner, after) = scoped(&[3], || {
            let outer = mint();
            // The scope `[3]` itself, entered from inside itself.
            let inner = {
                let _same = MintScope::current().enter();
                mint()
            };
            (outer, inner, mint())
        });
        let (parent, from_child) = {
            let parent_scope = MintScope::current().child(&[4]);
            let _open = parent_scope.enter();
            let parent = mint();
            let from_child = scoped(&[5], || {
                let _parent_again = parent_scope.enter();
                mint()
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
        assert_eq!(got, Err(()), "{what}: two unknowns, refused");
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
/// same unknown, and their difference a proven `Zero`.
#[test]
fn two_scopes_mints_from_two_sessions_do_not_cancel() {
    let (carried, _) = session(|| scoped(&[1], mint));
    let (sign, counts) = session(|| sign_of(carried - scoped(&[2], mint)));
    assert_eq!((sign, counts.symbolic), APART);
}

/// A panic unwinding through an open scope in a session leaves the
/// scope that was open before it, and the scope it unwound through
/// still mints no name twice.
#[test]
fn a_panic_through_a_scope_in_a_session_restores_the_outer_scope() {
    let (restored, names) = session(|| {
        let root = MintScope::current();
        let a = mint().node();
        let inner = std::panic::catch_unwind(|| {
            let _open = MintScope::current().child(&[1]).enter();
            std::panic::resume_unwind(Box::new(mint().node()));
        })
        .unwrap_err()
        .downcast::<SymId>()
        .unwrap();
        let b = mint().node();
        let again = scoped(&[1], || mint().node());
        (MintScope::current() == root, [a, *inner, b, again])
    })
    .0;
    assert!(restored, "the session's root is open again");
    all_differ(&names, "around the panic");
}

// ------------------------------------------------ outside a session

/// Outside a session a thread gives no name twice, whatever scope is
/// open: one scope opened twice, a scope entered from inside itself,
/// and no scope at all each mint values a later session tells apart.
///
/// Mutant: a name that outside a session is the open scope's and its
/// count from zero gives the two openings' first mints one name, and
/// the pairs are proven `Zero`.
#[test]
fn outside_a_session_a_thread_gives_no_name_twice() {
    let (ids, pairs) = fresh(|| {
        let first = scoped(&[1], mint);
        let again = scoped(&[1], mint);
        let (outer, inner) = scoped(&[3], || {
            let outer = mint();
            let inner = {
                let _same = MintScope::current().enter();
                mint()
            };
            (outer, inner)
        });
        let (root, root_again) = (mint(), mint());
        let all = [first, again, outer, inner, root, root_again];
        (
            all.map(Sym::node),
            [
                ("one scope opened twice", met(first, again)),
                ("a scope entered from inside itself", met(outer, inner)),
                ("no scope", met(root, root_again)),
            ],
        )
    });
    all_differ(&ids, "outside a session");
    for (what, got) in pairs {
        assert_eq!(got, APART, "{what}");
    }
}

/// Two detached maps under one scope: no unit of either shares a name
/// with any other, on whichever workers they ran.
///
/// Mutant: units named by their index under the caller's scope give
/// unit `i` of both maps one name, a proven `Zero`.
#[test]
fn two_detached_maps_under_one_scope_share_no_name() {
    let (ids, got) = fresh(|| {
        let run = |n: usize| -> Vec<Sym<Interval>> {
            geom_core::k_stats::map_detached(&vec![(); n], |()| mint())
                .into_iter()
                .map(|(v, _)| v)
                .collect()
        };
        let (first, second) = scoped(&[7], || (run(2), run(2)));
        let ids: Vec<SymId> = first.iter().chain(&second).map(|s| s.node()).collect();
        (ids, met(first[0], second[0]))
    });
    all_differ(&ids, "two maps' units");
    assert_eq!(got, APART, "unit 0 of each map");
}

/// A value minted before a session, under a scope or under none, and a
/// value the session mints, under the same scope or under none, are
/// two unknowns: no name given outside a session is one a session
/// gives.
///
/// Mutant: outside a session the open scope's name counting from zero,
/// with a session's root the thread's own: the thread's first mint and
/// the session root's first share a name, as do the scope's first
/// outside and inside, and both pairs are proven `Zero`.
#[test]
fn a_value_minted_before_a_session_and_one_minted_inside_it_do_not_cancel() {
    let (pairs, counts) = fresh(|| {
        let before = mint();
        let scoped_before = scoped(&[1], mint);
        session(|| {
            [
                ("under no scope", sign_of(before - mint())),
                (
                    "under one scope",
                    sign_of(scoped_before - scoped(&[1], mint)),
                ),
            ]
        })
    });
    for (what, got) in pairs {
        assert_eq!(got, Err(()), "{what}: two unknowns, refused");
    }
    assert_eq!(counts.symbolic, 0, "no identity was there to prove");
}

/// A walk that maps its units in parallel where decisions are
/// thread-portable and runs them in order under a session
/// (`topo::props`' face walk is one) names the same work two ways: by
/// the minting thread outside a session, by the scope and its count
/// inside one. The names share nothing, which is what keeps a value of
/// the one arm from ever being read as a value of the other.
#[test]
fn the_two_arms_of_a_walk_share_no_name() {
    let (outside, inside) = fresh(|| {
        let faces = || -> Vec<SymId> {
            if geom_core::sym::decisions_are_thread_portable() {
                geom_core::k_stats::map_detached(&[(); 3], |()| mint().node())
                    .into_iter()
                    .map(|(id, _)| id)
                    .collect()
            } else {
                (0..3).map(|_| mint().node()).collect()
            }
        };
        let outside = scoped(&[9], faces);
        let (inside, _) = session(|| scoped(&[9], faces));
        (outside, inside)
    });
    let all: Vec<SymId> = outside.iter().chain(&inside).copied().collect();
    all_differ(&all, "the two arms");
}

/// A scope's guard dropped after the session it was opened in has left
/// is refused, and moves no count: the thread's next mint is a name it
/// has not given.
#[test]
fn a_guard_that_outlives_its_session_is_refused_and_moves_nothing() {
    let (refused, before, after) = fresh(|| {
        let before = [mint().node(), mint().node(), mint().node()];
        let (guard, _) = session(|| {
            let _ = mint();
            MintScope::current().child(&[1]).enter()
        });
        let refused =
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| drop(guard))).is_err();
        (refused, before, mint().node())
    });
    assert_eq!(
        refused,
        cfg!(debug_assertions),
        "the drop out of its session is a bug check"
    );
    assert!(!before.contains(&after), "a fourth name");
}
