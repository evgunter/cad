//! **∖ in both operand orders and ∩** — the three ops that route
//! through `revert`, run over one operand pair the way a roster row
//! asks its question. What a suite drives a door WITH, so it routes
//! here beside [`super::poses`] ([`super`]'s routing rule).

use geom_core::Tol;
use sweep::test_support::finished;
use topo::{Body, BooleanError, BooleanResult};

/// `A ∖ B`, `B ∖ A` and `A ∩ B`, each labelled for an assertion
/// message. Each operand is finished once
/// (`sweep::test_support::finished`) before the three ops read it.
pub fn subtract_both_orders_and_intersect(
    a: &Body<f64>,
    b: &Body<f64>,
) -> [(&'static str, Result<BooleanResult<f64>, BooleanError>); 3] {
    let tol = Tol::witness();
    let a = &finished("operand A", a.clone(), tol);
    let b = &finished("operand B", b.clone(), tol);
    [
        ("A ∖ B", topo::subtract(a, b, tol)),
        ("B ∖ A", topo::subtract(b, a, tol)),
        ("A ∩ B", topo::intersect(a, b, tol)),
    ]
}
