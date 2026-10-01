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
//!
//! A discarded face that a kept face holds part of records the kept
//! face's edges that run into it (`DiscardRow::held`).

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

/// `b` = x 0.5..1.5 cut by a slab over x 0.499..0.501 from z 0.5 up,
/// then united with `a` = x 0..1 flush on both caps and y-walls: `a`'s
/// top is discarded from x 0.499 on, and from x 0.501 `b`'s cut top
/// holds it. The edge of `b`'s top along the slab at x 0.501 runs into
/// it, and on each y-wall so do the two edges of the notch the slab cut
/// in `b`'s wall.
#[test]
fn a_discarded_face_holds_the_edges_of_the_kept_face_that_runs_into_it() {
    let tol = Tol::witness();
    let unit = (0.0, 1.0);
    let a: Body<f64> = brick((0.0, 1.0), unit, unit, tol);
    let b: Body<f64> = brick((0.5, 1.5), unit, unit, tol);
    let slab = brick((0.499, 0.501), (-1.0, 2.0), (0.5, 2.0), tol);
    let BooleanResult::Body(cut) =
        union_with(&b, &slab, &topo::BooleanDeclarations::default(), tol).expect("b ∪ slab")
    else {
        panic!("a union of non-empty blocks cannot be empty");
    };
    let decls = declare_all(
        &find_flush_candidates(&cut.body, &a, tol).expect("the flush detector decides"),
    );
    let BooleanResult::Body(out) = union_with(&cut.body, &a, &decls, tol).expect("the union fuses")
    else {
        panic!("a union of non-empty blocks cannot be empty");
    };
    // Each holding row: `a`'s face's normal, and each held edge's ends,
    // sorted.
    let mut got: Vec<([i8; 3], Vec<[[i64; 3]; 2]>)> = out
        .naming
        .discards
        .iter()
        .filter(|r| !r.held.is_empty())
        .map(|r| {
            let ends = r
                .held
                .iter()
                .map(|&e| {
                    let edge = out.body.get_edge(e).expect("a held edge is live");
                    let mut ends = [edge.he_plus, edge.he_minus].map(|he| {
                        let v = out.body.get_half_edge(he).expect("a live half-edge").start;
                        let p = topo::readback::vertex_point(&out.body, v).expect("a live vertex");
                        [p.x, p.y, p.z].map(|c| (c * 1000.0).round() as i64)
                    });
                    ends.sort();
                    ends
                })
                .collect();
            (normal(&a, a_face(&out, r.face)), ends)
        })
        .collect();
    got.sort();
    assert_eq!(
        got,
        vec![
            (
                [0, -1, 0],
                vec![
                    [[500, 0, 500], [501, 0, 500]],
                    [[501, 0, 500], [501, 0, 1000]]
                ]
            ),
            ([0, 0, 1], vec![[[501, 0, 1000], [501, 1000, 1000]]]),
            (
                [0, 1, 0],
                vec![
                    [[501, 1000, 500], [501, 1000, 1000]],
                    [[500, 1000, 500], [501, 1000, 500]]
                ]
            ),
        ],
        "a's top and y-walls hold the edges of `b`'s notch inside them"
    );
}

/// The operand face of `a` (operand B) that discarded clone face `f` is
/// a fragment of.
fn a_face(out: &topo::BooleanBody<f64>, f: FaceKey) -> FaceKey {
    let mut f = f;
    while let Some(&(_, up)) = out
        .naming
        .face_fragments_b
        .iter()
        .find(|(new, _)| *new == f)
    {
        f = up;
    }
    f
}
