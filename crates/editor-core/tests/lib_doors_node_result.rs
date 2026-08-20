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
    let mut doc = ProfileDoc::empty_derived("lib_doors_node_result");
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
    assert!(
        result.error().is_none(),
        "a poisoned node did not itself fail"
    );
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

/// F6 (reopened on the PR #308 review): the refusal enums render as
/// PROSE — problem statements, not the payloads' `Debug` guts. Pins
/// one message per new `Display` (EditError, NodeError/-Kind,
/// DimensionError, ProgramRefusal) plus the no-guts property on the
/// live coincidence refusal.
#[test]
fn refusals_render_as_prose_not_debug_guts() {
    use editor_core::{DimensionError, EditError};

    let edit = EditError::UnknownNode {
        id: RecipeNodeId(7),
    };
    assert_eq!(edit.to_string(), "edit: node 7 is not live");

    let literal = Expr::literal(f64::NAN, Dimension::Length).expect_err("NaN refuses");
    assert!(matches!(literal, DimensionError::NonFiniteLiteral));
    assert_eq!(literal.to_string(), "a literal value must be finite");

    // The live failure: the coincident Boolean's message states the
    // problem and the two-armed recourse (since R3 the refusal is the
    // typed menu variant); the enum's structure (variant names,
    // braces) stays OUT of the prose.
    let (doc, cut, _) = doc_with_failure();
    let ev = run(&doc);
    let error = ev.node_error(cut).expect("the Boolean failed");
    let message = error.to_string();
    assert!(
        message.starts_with(&format!("node {} failed: ", cut.0)),
        "{message}"
    );
    assert!(
        message.contains("Boolean refused an undeclared contact"),
        "{message}"
    );
    assert!(message.contains("declare that finding"), "{message}");
    for guts in [
        "UndeclaredCoincidence",
        "UndeclaredContact",
        "FlushFinding",
        "{",
        "Indeterminate",
    ] {
        assert!(!message.contains(guts), "Debug guts leaked: {message}");
    }
}

/// **A `NodeErrorKind` arm that holds a kernel refusal RENDERS it.**
/// The variant carries the typed error for a caller who can match; the
/// message is the whole channel for one who cannot, and the bindings'
/// `kind` attribute is the discriminant alone — so an arm that names
/// the op and stops has spent the payload's class, keys and recourse
/// on nothing.
///
/// **Representative, not exhaustive**, and nothing makes it
/// exhaustive: `NodeErrorKind` cannot be enumerated at runtime and a
/// hand-kept roster of arms is the very shape this repo keeps
/// retiring. One arm per payload-owning crate is pinned instead —
/// `profile`, `sweep` (three doors), `topo` (two). A new arm wrapping
/// a new kernel error is not covered here; the module comment on the
/// `Display` impl is what states the rule for it.
#[test]
fn a_kernel_payload_arm_forwards_the_payloads_own_message() {
    use editor_core::NodeErrorKind as K;

    let cases: Vec<K> = vec![
        K::Profile(profile::ProfileError::EmptyProfile),
        K::Extrude(sweep::ExtrudeError::ObliqueExtrusion),
        K::Revolve(sweep::RevolveError::DegenerateAxis),
        K::Skin(sweep::SkinError::TooFewSections { have: 1, need: 2 }),
        K::Loft(sweep::LoftError::SeamStructure),
        K::Fillet(sweep::fillet::FilletError::RepeatedEdge {
            edge: topo::EdgeKey::default(),
        }),
        K::Transform(topo::transform::TransformError::NurbsPlaceholder),
    ];

    for kind in cases {
        let rendered = kind.to_string();
        let payload = match &kind {
            K::Profile(e) => e.to_string(),
            K::Extrude(e) => e.to_string(),
            K::Revolve(e) => e.to_string(),
            K::Skin(e) => e.to_string(),
            K::Loft(e) => e.to_string(),
            K::Fillet(e) => e.to_string(),
            K::Transform(e) => e.to_string(),
            other => panic!("add the new case's payload here: {other:?}"),
        };
        assert!(
            rendered.ends_with(&payload),
            "the arm dropped its payload: rendered {rendered:?}, payload {payload:?}"
        );
        assert!(
            rendered.len() > payload.len(),
            "the arm must still name the failing op: {rendered:?}"
        );
    }
}
