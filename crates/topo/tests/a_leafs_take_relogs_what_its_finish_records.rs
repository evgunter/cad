//! **A leaf's take relogs exactly what its finish records** (the raw
//! logs `geom_core::k_stats::relog`'s contract asks a row to compare;
//! `docs/predicate-dimension-audit.md` F31).
//!
//! At `f64` the leaf runs its finish under a bracket and keeps the
//! record on the body (`topo::AtRestPolicy::leaf`); the one taker door
//! (`topo::take`) relogs it. The verdicts and escalations the take
//! leaves in its frame are those a fresh finish of the same body
//! records, in order. A leaf that refuses has no taker: its record is
//! relogged where it refuses, so that frame holds what the finish
//! recorded on the way to the refusal.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::Tol;
use geom_core::k_stats::{Bracket, Recorded};
use topo::{AtRestPolicy, Body};

fn recorded(f: impl FnOnce()) -> Recorded {
    let scope = Bracket::open();
    f();
    scope.finish()
}

fn bodies() -> Vec<(&'static str, Body<f64>)> {
    let t = Tol::witness();
    vec![
        (
            "a brick",
            topo::test_support::brick((0.0, 2.0), (0.0, 1.0), (0.0, 0.5), t),
        ),
        (
            "a slab off the origin",
            topo::test_support::brick((-3.0, 1.0), (0.25, 4.0), (1.0, 1.125), t),
        ),
    ]
}

#[test]
fn a_take_relogs_the_verdicts_a_finish_records() {
    let t = Tol::witness();
    for (what, body) in bodies() {
        let fresh = recorded(|| {
            topo::finish::<f64>(body.clone(), t).expect("it finishes");
        });
        assert!(!fresh.verdicts.is_empty(), "{what}: the finish decides");
        let mut leaf = None;
        let built = recorded(|| {
            leaf = Some(<f64 as AtRestPolicy>::leaf(body.clone(), t).expect("its leaf finishes"));
        });
        assert_eq!(
            built,
            Recorded::default(),
            "{what}: the leaf holds its finish off the log"
        );
        let leaf = std::sync::Arc::new(leaf.expect("built"));
        let taken = recorded(|| {
            topo::take::<f64>(&leaf, t).expect("the take passes it through");
        });
        assert_eq!(taken, fresh, "{what}: the take relogs the finish's record");
    }
}

#[test]
fn a_leaf_that_refuses_relogs_its_record_where_it_refuses() {
    let t = Tol::witness();
    for (what, body) in bodies() {
        let inside_out = body.revert().expect("it reverts");
        let fresh = recorded(|| {
            topo::finish::<f64>(inside_out.clone(), t).expect_err("inside out, it refuses");
        });
        assert!(!fresh.verdicts.is_empty(), "{what}: the refusal decided");
        let refused = recorded(|| {
            <f64 as AtRestPolicy>::leaf(inside_out.clone(), t).expect_err("its leaf refuses");
        });
        assert_eq!(
            refused, fresh,
            "{what}: the refusal's frame holds the record"
        );
    }
}
