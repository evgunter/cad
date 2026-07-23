//! M3 PR 6a acceptance: `validate_pseudomanifold` (tier 3′) —
//! declared-contact certification on the touching-configuration corpus
//! at rest, negative controls, closure stress, and exports.
//!
//! Every promotion scenario tests BOTH directions: the boolean result
//! is green under its carried contacts AND red (`UndeclaredContact`)
//! when the declarations are withheld — the census never blesses
//! (F1/F2). Scenarios are generic over `T` (f64 ε rows via CI; the
//! explicit Interval lane at the bottom, per suite convention).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod common;

use common::prism_z;
use geom_core::Decide;
use topo::{
    Body, BooleanBody, BooleanError, BooleanResult, ContactRecords, ValidationError, union,
    validate_pseudomanifold,
};

fn brick<T: Decide>(x: (f64, f64), y: (f64, f64), z: (f64, f64)) -> Body<T> {
    prism_z::<T>(&[(x.0, y.0), (x.1, y.0), (x.1, y.1), (x.0, y.1)], z.0, z.1).body
}

type BoolOp<T> = fn(&Body<T>, &Body<T>) -> Result<BooleanResult<T>, BooleanError>;

fn run_body<T: Decide>(op: BoolOp<T>, a: &Body<T>, b: &Body<T>) -> BooleanBody<T> {
    match op(a, b).unwrap() {
        BooleanResult::Body(body) => body,
        BooleanResult::Empty => panic!("expected a non-empty boolean result"),
    }
}

/// The green/red promotion pair: 3′ passes with the carried contacts,
/// and withholding them yields `UndeclaredContact` (and nothing else).
fn assert_promoted<T: Decide>(b: &BooleanBody<T>) {
    assert_eq!(validate_pseudomanifold(&b.body, &b.contacts), Ok(()));
    let withheld = validate_pseudomanifold(&b.body, &ContactRecords::default()).unwrap_err();
    assert!(!withheld.is_empty());
    for e in &withheld {
        assert!(
            matches!(e, ValidationError::UndeclaredContact { .. }),
            "withheld contacts must surface as UndeclaredContact, got {e:?}"
        );
    }
}

// ---------------------------------------------------------------
// PROMOTION (D9): the touching corpus at rest.
// ---------------------------------------------------------------

/// Corner kiss (v-v): the PR 5 assembly, now certified at rest.
fn corner_kiss_scenario<T: Decide>() {
    let a = brick::<T>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0));
    let b = brick::<T>((1.0, 2.0), (1.0, 2.0), (1.0, 2.0));
    let body = run_body(union as BoolOp<T>, &a, &b);
    assert_eq!(body.contacts.vv.len(), 1);
    assert_promoted(&body);
}

#[test]
fn corner_kiss_promoted() {
    corner_kiss_scenario::<f64>();
}

// ---- Interval lane (same scenarios at T = Interval). ----
#[cfg(feature = "interval")]
mod interval {
    use super::*;

    #[test]
    fn tier3prime_interval() {
        corner_kiss_scenario::<geom_core::Interval>();
    }
}
