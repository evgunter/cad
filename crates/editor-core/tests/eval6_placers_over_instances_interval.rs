//! **A family of families, at the `Interval` lane** — the bit-identity
//! claim of `eval6_placers_over_instances`, evaluated at the lane
//! scalar: member `(i, j)` of a placement over a family is placement
//! `j` of the inner member `i` alone. The documents and the bits
//! instrument are that suite's own, so the two lanes pin one thing.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::eval6_placers_over_instances::{M, N, bits, instances_of, nested_doc, per_member};

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

/// The nested layout's per-member identity at `Interval`.
#[test]
fn a_family_of_families_is_one_member_per_pair_at_interval() {
    let (doc, _cube, inner, outer, _, _) = nested_doc("eval6-c3-interval");
    let (doc, rows) = per_member(doc, inner);
    let ev = run(&doc);
    let nested = instances_of(&ev, outer);
    assert_eq!(nested.len() as i64, N * M);
    for (i, row) in rows.iter().enumerate() {
        let alone = instances_of(&ev, *row);
        for j in 0..N as usize {
            assert_eq!(
                bits(&nested[i * N as usize + j]),
                bits(&alone[j]),
                "member ({i}, {j})"
            );
        }
    }
}
