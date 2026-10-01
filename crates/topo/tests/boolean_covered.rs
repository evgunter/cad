//! **A boolean records each coincident face pair whose region it holds
//! through one face only** (`BooleanNaming::covered`).
//!
//! `b` = x 0.5..1.5 lies inside `a` = x 0..2, both over y 0..1 and z 0..1,
//! flush on both caps and both y-walls. With `a` as operand A the union
//! keeps `a`'s copies, cuts nothing and is `a`, by the containment
//! fallback; with `b` as operand A it keeps `b`'s copies and takes the
//! section path. Raised to z 0..2, `b` pokes out of `a`'s top and the
//! union takes the section path in either order; its bottom and y-walls
//! stay flush.
//! Every flush pair is declared. On either path, and with either block as
//! operand A, each flush pair is recorded once, as `(A face, B face)`,
//! whichever copy the classification kept.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::BTreeSet;

use crate::common;

use common::brick;
use geom_core::Tol;
use topo::flush::{declare_all, find_flush_candidates};
use topo::{
    Body, BooleanResult, BooleanResultKind, CarrierDesc, FaceKey, face_carrier, union_with,
};

/// The outward normal of `body`'s face `f`, rounded to a unit axis.
fn normal(body: &Body<f64>, f: FaceKey) -> [i8; 3] {
    let Some(CarrierDesc::Plane { normal, .. }) = face_carrier(body, f) else {
        panic!("face {f:?} is not a plane of its operand");
    };
    [normal.x, normal.y, normal.z].map(|c| c.round() as i8)
}

/// The union of `x` and `y` with every flush pair declared: its kind and
/// the normals of each covered pair, operand A's then operand B's.
fn covered(x: &Body<f64>, y: &Body<f64>) -> (BooleanResultKind, BTreeSet<([i8; 3], [i8; 3])>) {
    let tol = Tol::witness();
    let decls = declare_all(&find_flush_candidates(x, y, tol).expect("the flush detector decides"));
    let BooleanResult::Body(out) = union_with(x, y, &decls, tol).expect("the union fuses") else {
        panic!("a union of non-empty blocks cannot be empty");
    };
    let pairs: BTreeSet<_> = out
        .naming
        .covered
        .iter()
        .map(|&(fa, fb)| (normal(x, fa), normal(y, fb)))
        .collect();
    assert_eq!(
        pairs.len(),
        out.naming.covered.len(),
        "one row per coincident pair"
    );
    (out.kind, pairs)
}

/// Same-normal pairs on each of `normals`.
fn flush_on(normals: &[[i8; 3]]) -> BTreeSet<([i8; 3], [i8; 3])> {
    normals.iter().map(|&n| (n, n)).collect()
}

#[test]
fn a_union_records_each_flush_pair_it_holds_through_one_face_in_either_operand_order() {
    let tol = Tol::witness();
    let unit = (0.0, 1.0);
    let a = brick((0.0, 2.0), unit, unit, tol);
    let inside = brick((0.5, 1.5), unit, unit, tol);
    let poking = brick((0.5, 1.5), unit, (0.0, 2.0), tol);
    let walls_and_caps = flush_on(&[[0, -1, 0], [0, 1, 0], [0, 0, -1], [0, 0, 1]]);
    let walls_and_bottom = flush_on(&[[0, -1, 0], [0, 1, 0], [0, 0, -1]]);
    for (label, x, y, kind, want) in [
        (
            "a ∪ inside",
            &a,
            &inside,
            BooleanResultKind::OperandA,
            &walls_and_caps,
        ),
        (
            "inside ∪ a",
            &inside,
            &a,
            BooleanResultKind::Seamed,
            &walls_and_caps,
        ),
        (
            "a ∪ poking",
            &a,
            &poking,
            BooleanResultKind::Seamed,
            &walls_and_bottom,
        ),
        (
            "poking ∪ a",
            &poking,
            &a,
            BooleanResultKind::Seamed,
            &walls_and_bottom,
        ),
    ] {
        let (got_kind, got) = covered(x, y);
        assert_eq!(got_kind, kind, "{label}: the path taken");
        assert_eq!(&got, want, "{label}: the covered pairs");
    }
}
