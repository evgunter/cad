//! Reviewer probes of `geom_core::sym::mint` (PR 7470 at 090c8510493):
//! where two different values with no expression come to share a name,
//! and what a session then says of their difference.
//!
//! Each row prints what it measured and asserts the measurement, so a
//! row that goes red reports that the behaviour changed, in whichever
//! direction. Every row runs on a thread of its own: the root scope
//! outside a session counts for the life of the thread.
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

/// One unknown real somewhere in `[0, 1]`: two of them are two reals
/// the numeric channel cannot tell apart (their difference encloses
/// zero and is nowhere near the band), so only their names decide. With
/// point values the numeric channel separates them first and the
/// exact-witness arm of `decide_sign` panics on the contradiction
/// (measured: "the two channels contradict each other"), which is the
/// loud half of this hazard; these rows are the silent half.
fn mint(_x: f64) -> Sym<Interval> {
    Sym::opaque(Interval::from_bounds(0.0, 1.0))
}

fn scoped<R>(words: &[u64], f: impl FnOnce() -> R) -> R {
    let _open = MintScope::current().child(words).enter();
    f()
}

fn sign_of(m: Sym<Interval>) -> Result<Sign, ()> {
    m.sign_within(Band::linear(Tol::witness()).unwrap())
        .map(|d| d.sign)
        .map_err(|_| ())
}

fn fresh<R: Send + 'static>(f: impl FnOnce() -> R + Send + 'static) -> R {
    std::thread::spawn(f).join().unwrap()
}

/// What a session decides of `a - b`, and how many identities it
/// counted proving it.
fn met(a: Sym<Interval>, b: Sym<Interval>) -> (Result<Sign, ()>, u64) {
    let (sign, counts) = session(|| sign_of(a - b));
    (sign, counts.symbolic as u64)
}

/// A: a thread's first mint outside any session, against a session's
/// first root mint: 1.0 and 2.0 under one name.
#[test]
fn a_mint_outside_any_session_meets_a_sessions_first_root_mint() {
    let (same_id, sign, symbolic) = fresh(|| {
        let outside = mint(1.0);
        let ((same_id, sign), counts) = session(|| {
            let inside = mint(2.0);
            (outside.node() == inside.node(), sign_of(outside - inside))
        });
        (same_id, sign, counts.symbolic)
    });
    eprintln!("A: same id {same_id}, sign {sign:?}, symbolic {symbolic}");
    assert!(same_id, "A: one name for two unknowns");
    assert_eq!(sign, Ok(Sign::Zero), "A: a proven Zero of two unknowns");
}

/// B: one scope opened twice OUTSIDE a session counts from zero both
/// times; the two values then meet in a session.
#[test]
fn a_scope_opened_twice_outside_a_session_names_two_values_alike() {
    let (ids, got) = fresh(|| {
        let a = scoped(&[1], || mint(1.0));
        let b = scoped(&[1], || mint(2.0));
        ((a.node(), b.node()), met(a, b))
    });
    eprintln!("B: ids equal {}, met {got:?}", ids.0 == ids.1);
    assert_eq!(ids.0, ids.1, "B: one name");
    assert_eq!(got.0, Ok(Sign::Zero), "B: a proven Zero of two unknowns");
}

/// B′: the same two mints under NO scope outside a session (the shape
/// the tree had before mint scopes: one count per thread).
#[test]
fn two_root_mints_outside_a_session_are_two_names() {
    let (ids, got) = fresh(|| {
        let a = mint(1.0);
        let b = mint(2.0);
        ((a.node(), b.node()), met(a, b))
    });
    eprintln!("B': ids equal {}, met {got:?}", ids.0 == ids.1);
    assert_ne!(ids.0, ids.1);
    assert_eq!(
        got.0,
        Err(()),
        "B': two unknowns in [0, 1]: refused, not Zero"
    );
}

/// C: two detached maps under one caller scope name their units alike.
#[test]
fn two_detached_maps_under_one_scope_name_their_units_alike() {
    let (first, second, got) = fresh(|| {
        let run = |xs: &[f64]| -> Vec<Sym<Interval>> {
            geom_core::k_stats::map_detached(xs, |&x| mint(x))
                .into_iter()
                .map(|(v, _)| v)
                .collect()
        };
        let (first, second) = scoped(&[7], || (run(&[1.0, 2.0]), run(&[3.0, 4.0])));
        let ids = |v: &[Sym<Interval>]| v.iter().map(|s| s.node()).collect::<Vec<SymId>>();
        (ids(&first), ids(&second), met(first[0], second[0]))
    });
    eprintln!("C: first == second {}, met {got:?}", first == second);
    assert_eq!(first, second, "C: unit i of both maps, one name");
    assert_eq!(got.0, Ok(Sign::Zero), "C: a proven Zero of two unknowns");
}

/// D: a scope entered from inside itself outside a session restarts at
/// zero, so one OPENING mints a name twice.
#[test]
fn a_scope_entered_from_inside_itself_outside_a_session_restarts() {
    let (same, got) = fresh(|| {
        let (outer, inner) = scoped(&[3], || {
            let outer = mint(1.0);
            let inner = {
                let _same = MintScope::current().enter();
                mint(2.0)
            };
            (outer, inner)
        });
        (outer.node() == inner.node(), met(outer, inner))
    });
    eprintln!("D: same id {same}, met {got:?}");
    assert!(same);
    assert_eq!(got.0, Ok(Sign::Zero));
}

/// E: a guard that outlives the session it was opened in hands the
/// thread's root scope the SESSION's root count when it drops.
#[test]
fn a_guard_that_outlives_its_session_moves_the_threads_root_count() {
    let (before, after) = fresh(|| {
        let before = [mint(1.0).node(), mint(2.0).node(), mint(3.0).node()];
        let (guard, _) = session(|| {
            let _ = mint(9.0);
            MintScope::current().child(&[1]).enter()
        });
        drop(guard);
        (before, mint(4.0).node())
    });
    eprintln!(
        "E: the mint after is the thread's {:?}",
        before.iter().position(|b| *b == after)
    );
    assert_eq!(
        before.iter().position(|b| *b == after),
        Some(1),
        "E: the thread's fourth root mint took its second's name"
    );
}

/// F: two sessions' root mints (a read after the evaluation, the
/// clearance query's) share names; carried, they cancel.
#[test]
fn two_sessions_root_mints_share_a_name() {
    let got = fresh(|| {
        let (carried, _) = session(|| mint(1.0));
        let (sign, _) = session(|| sign_of(carried - mint(2.0)));
        sign
    });
    eprintln!("F: {got:?}");
    assert_eq!(got, Ok(Sign::Zero));
}

/// G: a panic unwinding through an open scope inside a session leaves
/// the scope that was open, and no name is minted twice after it.
#[test]
fn a_panic_through_a_scope_in_a_session_restores_the_outer_scope() {
    let ok = fresh(|| {
        session(|| {
            let root = MintScope::current();
            let a = mint(1.0).node();
            let inner = std::panic::catch_unwind(|| {
                let _open = MintScope::current().child(&[1]).enter();
                let id = mint(2.0).node();
                std::panic::resume_unwind(Box::new(id));
            })
            .unwrap_err()
            .downcast::<SymId>()
            .unwrap();
            let b = mint(3.0).node();
            let again = scoped(&[1], || mint(4.0).node());
            MintScope::current() == root && a != b && *inner != again && b != again
        })
        .0
    });
    assert!(ok, "G");
}

/// H: a name minted in a scope outside a session, and the same scope's
/// first mint inside one.
#[test]
fn a_scopes_mint_outside_a_session_meets_the_same_scopes_mint_inside_one() {
    let got = fresh(|| {
        let outside = scoped(&[1], || mint(1.0));
        session(|| sign_of(outside - scoped(&[1], || mint(2.0)))).0
    });
    eprintln!("H: {got:?}");
    assert_eq!(got, Ok(Sign::Zero));
}
