//! **The contact vocabulary at the boolean door** (CONTACT-DESIGN
//! C3/C4): the records replay bit-identically.
//!
//! The per-class verification tables and their ε rows live beside the
//! ladders they exercise (`boolean::carrier_eq`,
//! `boolean::contact_verify`); this file pins the behaviour a CALLER
//! can see.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use common::{brick, finished};
use geom_core::Tol;
use topo::{AtRestBody, BooleanResult, mass_properties, union};

/// A flush stack: two bricks meeting on z = 1, independently authored.
fn stacked() -> (AtRestBody<f64>, AtRestBody<f64>) {
    let tol = Tol::witness();
    (
        finished(
            "the lower brick",
            brick((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol),
            tol,
        ),
        finished(
            "the upper brick",
            brick((0.5, 1.5), (0.25, 1.25), (1.0, 2.0), tol),
            tol,
        ),
    )
}

/// A planar `Rest` unions, and its RECORDS replay
/// bit-identically — the D9/C4 replay clause checked on the records
/// themselves rather than on naming standing in for them (the new
/// `PartialEq`). Records carry arena keys, and key identity is part of
/// what determinism means for a rerun of the same pipeline.
#[test]
fn a_rest_unions_and_replays_records_bit_identically() {
    let (a, b) = stacked();
    let first = union(&a, &b, Tol::witness()).expect("the flush union runs");
    let second = union(&a, &b, Tol::witness()).expect("the rerun runs");
    let (BooleanResult::Body(x), BooleanResult::Body(y)) = (first, second) else {
        panic!("an overlapping union cannot be Empty");
    };
    assert_eq!(
        x.contacts, y.contacts,
        "bit-identical replay must reproduce the RECORDS bit-identically (D9/C4)"
    );
    assert_eq!(
        mass_properties(
            &x.body,
            Tol::witness(),
            topo::test_support::pinned_resolution(Tol::witness())
        )
        .unwrap()
        .met()
        .expect("the pinned resolution is met")
        .volume,
        mass_properties(
            &y.body,
            Tol::witness(),
            topo::test_support::pinned_resolution(Tol::witness())
        )
        .unwrap()
        .met()
        .expect("the pinned resolution is met")
        .volume
    );
}
