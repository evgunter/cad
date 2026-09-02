//! M4 PR 3 naming tests, part 4 (spec D5): f64/Interval agreement —
//! same verdicts ⇒ IDENTICAL name tables at both scalar types (the Q1
//! genericity boundary respected). `NameTable` is scalar-independent
//! (names + arena keys), so the comparison is direct table equality
//! per node, over a boolean-and-split-bearing corpus document.
#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod fixture;

use editor_core::{
    BooleanOp, CancelToken, Datum, EvalOptions, Evaluation, Node, ProfileDoc, RecipeNodeId,
    evaluate,
};
use fixture::{declare_x_offset_flush, desc, insert, len, on_frame, scl, wall};
use geom_core::Interval;
use geom_core::Tol;

fn block(
    doc: ProfileDoc,
    (x0, x1): (f64, f64),
    (y0, y1): (f64, f64),
    z0: f64,
    dz: f64,
) -> (ProfileDoc, RecipeNodeId) {
    let (doc, p) = on_frame(
        doc,
        [0.0, 0.0, z0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(x0, y0), (x1, y0), (x1, y1), (x0, y1)]],
    );
    insert(
        doc,
        Node::Extrude {
            profile: p,
            distance: len(dz),
        },
    )
}

/// The corpus: an overlapping union, a through-slot subtract, and a
/// plane split — all dyadic.
fn corpus() -> ProfileDoc {
    let doc = ProfileDoc::empty_derived("m4_pr3_names_interval", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, b) = block(doc, (0.5, 1.5), (0.0, 1.0), 0.0, 1.0);
    let (doc, decl_u) = declare_x_offset_flush(doc, a, b);
    let (doc, _union) = insert(
        doc,
        Node::Boolean {
            op: BooleanOp::Union,
            a,
            b,
            declare: Some(decl_u),
        },
    );
    let (doc, c) = block(doc, (0.0, 3.0), (0.0, 3.0), 0.0, 1.0);
    // The slot spans z ∈ [0.5, 1.5]: it PROTRUDES through c's top cap
    // (z = 1.0) rather than resting in its plane, so the pair is not a
    // coincidence and carries no declaration — the same shape as the
    // f64 twin (`m4_pr3_names_bool.rs`'s slot subtract).
    //
    // It used to carry one, with a comment claiming "the slot's top
    // cap lies IN c's top plane" — arithmetically false by 0.5 m. The
    // declaration survived a milestone because nothing verified it:
    // the boolean's only verify-at-use site was the REST lane, which
    // is Union-only, and this op is a Subtract. The op-door pass
    // (`boolean::verify_declared_contacts`) now checks every declared
    // pair, which is what surfaced this. Coverage is not lost: `decl_u`
    // above still exercises the declared-boolean naming path with a
    // declaration that is TRUE.
    let (doc, slot) = block(doc, (1.0, 2.0), (-1.0, 4.0), 0.5, 1.0);
    let _ = wall; // shared helper import parity with the f64 lane
    let (doc, _sub) = insert(
        doc,
        Node::Boolean {
            op: BooleanOp::Subtract,
            a: c,
            b: slot,
            declare: None,
        },
    );
    let (doc, d) = block(doc, (4.0, 6.0), (0.0, 2.0), 0.0, 2.0);
    let (doc, plane) = insert(
        doc,
        Node::Datum(Datum::Plane {
            origin: [len(0.0), len(0.0), len(1.0)],
            normal: [scl(0.0), scl(0.0), scl(1.0)],
        }),
    );
    let (doc, _split) = insert(
        doc,
        Node::Split {
            target: d,
            tool: plane,
        },
    );
    doc
}

fn run<T: editor_core::EvalScalar>(doc: &ProfileDoc) -> Evaluation<T> {
    evaluate::<T>(
        doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    )
}

#[test]
fn f64_and_interval_lanes_emit_identical_name_tables() {
    let doc = corpus();
    let ef: Evaluation<f64> = run(&doc);
    let ei: Evaluation<Interval> = run(&doc);
    assert_eq!(ef.order, ei.order);
    for id in &ef.order {
        let vf = ef
            .value(*id)
            .unwrap_or_else(|| panic!("f64 node {id:?} failed: {:?}", ef.nodes.get(id)));
        let vi = ei
            .value(*id)
            .unwrap_or_else(|| panic!("interval node {id:?} failed: {:?}", ei.nodes.get(id)));
        assert_eq!(
            vf.name_table, vi.name_table,
            "lane disagreement at node {id:?}"
        );
    }
}
