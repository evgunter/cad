//! **A pattern over `Instances`, at the `Interval` lane** — the
//! bit-identity claim of `eval6_placers_over_instances`, evaluated at
//! the lane scalar: a nested pattern's body `j·M + i` is placement `j`
//! of the inner instance `i` alone. The documents and the bits
//! instrument are that suite's own, so the two lanes pin one thing.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::eval6_placers_over_instances::{M, N, bits, instances_of, linear, nested_doc, part};
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
