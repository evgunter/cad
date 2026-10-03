//! **∖ in both operand orders and ∩, under one set of declarations** —
//! the three ops that route through `revert`, run over one operand pair
//! the way a roster row asks its question. What a suite drives a door
//! WITH, so it routes here beside [`super::poses`] ([`super`]'s routing
//! rule).
//!
//! **Deliberately not absorbed**, and the whole of it: [`super::poses`]
//! and [`super::charts`], which a declaration swap would join as
//! something a suite drives a door with and does not — it moves no
//! geometry, only which operand a declared face is named against.

use geom_core::Tol;
use sweep::test_support::finished;
use topo::{Body, BooleanDeclarations, BooleanError, BooleanResult, FacePairDeclaration};

/// The same declarations with the operands' roles swapped, for
/// `B ∖ A`: every declared face pair names its faces the other way
/// round, and the carried contacts trade places.
pub fn swapped(d: &BooleanDeclarations) -> BooleanDeclarations {
    let mut out = BooleanDeclarations::none();
    out.coincident_faces = d
        .coincident_faces
        .iter()
        .map(|p| FacePairDeclaration::new(p.b, p.a, p.class))
        .collect();
    out.carried_a = d.carried_b.clone();
    out.carried_b = d.carried_a.clone();
    out
}

/// `A ∖ B`, `B ∖ A` (under [`swapped`] declarations) and `A ∩ B`, each
/// labelled for an assertion message. Each operand is finished once
/// (`sweep::test_support::finished`) before the three ops read it.
pub fn subtract_both_orders_and_intersect(
    a: &Body<f64>,
    b: &Body<f64>,
    d: &BooleanDeclarations,
) -> [(&'static str, Result<BooleanResult<f64>, BooleanError>); 3] {
    let tol = Tol::witness();
    let a = &finished("operand A", a.clone(), tol);
    let b = &finished("operand B", b.clone(), tol);
    [
        ("A ∖ B", topo::subtract_with(a, b, d, tol)),
        ("B ∖ A", topo::subtract_with(b, a, &swapped(d), tol)),
        ("A ∩ B", topo::intersect_with(a, b, d, tol)),
    ]
}
