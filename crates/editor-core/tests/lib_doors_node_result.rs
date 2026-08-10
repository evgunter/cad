//! **LIB-DOORS F3: failed and poisoned nodes are reachable as typed
//! data from an `Evaluation`.**
//!
//! `Evaluation::value` deliberately collapses `Failed` and `Poisoned`
//! into `None`; before these accessors, NOTHING public distinguished
//! them — `NodeResult` had no `impl` block at all, and `nodes` is a
//! map a caller could only pattern-match by naming the enum. The
//! bindings' §L4 contract (typed exceptions carrying the real
//! `NodeError`) needs the distinction, so this suite pins it.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use editor_core::{
    BooleanOp, CancelToken, Dimension, DocEdit, EvalOptions, Expr, LoopProgram, Node, NodeResult,
    ProfileDoc, ProfileProgram, ProgramStep, ProgramTarget, RecipeNodeId, evaluate,
};
use profile::SketchPlane;

/// A square profile `[0,s]²` on the xy-plane, as a loop program.
fn square(s: f64) -> Node<ProfileProgram> {
    let lit = |v: f64| Expr::literal(v, Dimension::Length).unwrap();
    Node::Profile(ProfileProgram {
        plane: SketchPlane::xy(),
        loops: vec![LoopProgram::Chain(vec![
            ProgramStep::At([lit(0.0), lit(0.0)]),
            ProgramStep::LineTo(ProgramTarget::Point([lit(s), lit(0.0)])),
            ProgramStep::LineTo(ProgramTarget::Point([lit(s), lit(s)])),
            ProgramStep::LineTo(ProgramTarget::Point([lit(0.0), lit(s)])),
            ProgramStep::LineTo(ProgramTarget::Start),
        ])],
    })
}

/// Two boxes SHARING the z=0 plane (and the x=0 / y=0 side planes),
/// subtracted: the kernel never infers coincidence, so the Boolean
/// node FAILS — and a node downstream of it is POISONED. Returns the
/// document plus the failing and poisoned ids.
fn doc_with_failure() -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
    let lit = |v: f64| Expr::literal(v, Dimension::Length).unwrap();
    let mut doc = ProfileDoc::empty();
    let insert = |doc: &mut ProfileDoc, node| {
        let applied = doc.apply(&DocEdit::InsertNode { node }).unwrap();
        *doc = applied.doc;
        applied.record.minted.unwrap()
    };
    let outer_profile = insert(&mut doc, square(2.0));
    let outer = insert(
        &mut doc,
        Node::Extrude {
            profile: outer_profile,
            distance: lit(2.0),
        },
    );
    let inner_profile = insert(&mut doc, square(1.0));
    let inner = insert(
        &mut doc,
        Node::Extrude {
            profile: inner_profile,
            distance: lit(1.0),
        },
    );
    let cut = insert(
        &mut doc,
        Node::Boolean {
            op: BooleanOp::Subtract,
            a: outer,
            b: inner,
            declare: None,
        },
    );
    let downstream = insert(
        &mut doc,
        Node::Boolean {
            op: BooleanOp::Union,
            a: cut,
            b: outer,
            declare: None,
        },
    );
    (doc, cut, downstream)
}

fn run(doc: &ProfileDoc) -> editor_core::Evaluation<f64> {
    evaluate::<f64>(doc, None, &CancelToken::new(), &EvalOptions::default())
}

#[test]
fn a_failed_node_answers_its_typed_error() {
    let (doc, cut, _) = doc_with_failure();
    let ev = run(&doc);
    // The collapse is unchanged: `value` still answers `None`.
    assert!(ev.value(cut).is_none());
    // The new door distinguishes what `value` collapses.
    let result = ev.result(cut).expect("the node has an entry");
    assert!(result.value().is_none());
    assert!(result.poisoned_through().is_none());
    let error = result.error().expect("a failed node carries its error");
    assert_eq!(error.node, cut);
    // And the one-hop convenience agrees.
    let root = ev.node_error(cut).expect("node_error reaches it too");
    assert_eq!(root.node, cut);
}

#[test]
fn a_poisoned_node_answers_the_nearest_failed_ancestor() {
    let (doc, cut, downstream) = doc_with_failure();
    let ev = run(&doc);
    let result = ev.result(downstream).expect("the node has an entry");
    assert!(result.value().is_none());
    assert!(result.error().is_none(), "a poisoned node did not itself fail");
    assert_eq!(result.poisoned_through(), Some(cut));
    // `node_error` walks the hop: the root cause is the ANCESTOR's.
    let root = ev.node_error(downstream).expect("root cause reachable");
    assert_eq!(root.node, cut);
}

#[test]
fn ok_and_absent_nodes_answer_none() {
    let (doc, _, _) = doc_with_failure();
    let ev = run(&doc);
    let ok_node = *ev.order.first().expect("the order is non-empty");
    assert!(ev.value(ok_node).is_some());
    assert!(matches!(ev.result(ok_node), Some(NodeResult::Ok(_))));
    assert!(ev.node_error(ok_node).is_none());
    let absent = RecipeNodeId(u64::MAX);
    assert!(ev.result(absent).is_none());
    assert!(ev.node_error(absent).is_none());
}
