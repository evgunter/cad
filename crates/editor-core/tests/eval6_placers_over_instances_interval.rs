//! **The placers over `Instances`, at the `Interval` lane** — the
//! bit-identity claims of `eval6_placers_over_instances`, evaluated
//! at the lane scalar: a transform of a pattern is the transform of
//! each instance, and a nested pattern's body `j·M + i` is placement
//! `j` of the inner instance `i` alone. The documents and the bits
//! instrument are that suite's own, so the two lanes pin one thing.

#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::eval6_placers_over_instances::{
    M, N, bits, body_of, cube_doc, instances_of, linear, nested_doc, part, skew,
};
use crate::fixture::insert;

use editor_core::{CancelToken, EvalOptions, Evaluation, ProfileDoc, evaluate};
use geom_core::{Interval, Tol};

fn run(doc: &ProfileDoc) -> Evaluation<Interval> {
    evaluate::<Interval>(
        doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    )
}

/// Claim 1 at `Interval`.
#[test]
fn a_transform_of_a_pattern_is_the_transform_of_each_instance_at_interval() {
    let (doc, cube) = cube_doc("eval6-c1-interval");
    let (doc, pattern) = insert(doc, linear(cube, [1.0, 0.0, 0.0], 2.0, M));
    let (doc, whole) = insert(doc, skew(pattern));
    let mut doc = doc;
    let mut each = Vec::new();
    for i in 0..M {
        let (next, selected) = insert(doc, part(pattern, i));
        let (next, moved) = insert(next, skew(selected));
        doc = next;
        each.push(moved);
    }
    let ev = run(&doc);
    let placed = instances_of(&ev, whole);
    assert_eq!(placed.len() as i64, M);
    for (i, moved) in each.iter().enumerate() {
        assert_eq!(bits(&placed[i]), bits(&body_of(&ev, *moved)), "body {i}");
    }
}

/// Claim 3's per-body identity at `Interval`.
#[test]
fn a_nested_pattern_lays_out_placement_major_at_interval() {
    let (doc, _cube, inner, outer) = nested_doc("eval6-c3-interval");
    let mut doc = doc;
    let mut per_instance = Vec::new();
    for i in 0..M {
        let (next, selected) = insert(doc, part(inner, i));
        let (next, over_one) = insert(next, linear(selected, [0.0, 1.0, 0.0], 2.0, N));
        doc = next;
        per_instance.push(over_one);
    }
    let ev = run(&doc);
    let nested = instances_of(&ev, outer);
    assert_eq!(nested.len() as i64, N * M);
    for (i, over_one) in per_instance.iter().enumerate() {
        let alone = instances_of(&ev, *over_one);
        for j in 0..N as usize {
            assert_eq!(
                bits(&nested[j * M as usize + i]),
                bits(&alone[j]),
                "body {j}·{M} + {i}"
            );
        }
    }
}
