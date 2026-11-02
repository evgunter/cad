//! **A blend's refusal names its edge by the name the document holds
//! for it**, not by the kernel's arena key: the reader of the refusal
//! is told which edge of the operand to look at.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture::{ang, len};
use editor_core::{
    AuthoredNode, CancelToken, DocEdit, DocumentId, EntityKey, EntityRef, EvalOptions, Evaluation,
    LoopProgram, Node, NodeErrorKind, NodeResult, ProfileDoc, ProfileProgram, ProgramStep,
    ProgramTarget, RecipeNodeId, apply, evaluate,
};
use geom_core::Tol;

fn eval(doc: &ProfileDoc) -> Evaluation<f64> {
    evaluate::<f64>(
        doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    )
}

fn mint(doc: &ProfileDoc, node: AuthoredNode) -> (ProfileDoc, RecipeNodeId) {
    let applied = apply(
        doc,
        &DocEdit::InsertNode {
            node: Box::new(node),
            fresh: Vec::new(),
        },
        Tol::witness(),
    )
    .unwrap_or_else(|e| panic!("edit refused: {e}"));
    let id = applied.record.minted.expect("an insert mints an id");
    (applied.doc, id)
}

/// The lune between the internally tangent circles (0,1) r 1 and
/// (0,2) r 2, `ProgramStep::Cusp` at the kiss: its extrude sweeps one
/// strut at material wedge 0, a knife edge no blend has a side for.
fn cusp_lune() -> LoopProgram<editor_core::Formula> {
    LoopProgram::Chain(vec![
        ProgramStep::At([len(0.0), len(4.0)]),
        ProgramStep::Angle(ang(-std::f64::consts::FRAC_PI_2)),
        ProgramStep::Line(len(2.0)),
        ProgramStep::Turn(ang(std::f64::consts::FRAC_PI_2)),
        ProgramStep::TangentArcTo(ProgramTarget::Point([len(0.0), len(0.0)])),
        ProgramStep::Cusp,
        ProgramStep::TangentArcTo(ProgramTarget::Start),
    ])
}

/// **Every edge of a cusp body filleted**: the kiss refuses as a knife
/// edge, and the node's refusal names that edge by its name in the
/// operand's table — one of the names the selection was made of.
#[test]
fn a_knife_edge_refusal_names_the_edge_by_its_stable_name() {
    let (doc, profile) = mint(
        &ProfileDoc::empty(DocumentId::derive("knife-named"), Tol::witness()),
        Node::Profile(ProfileProgram {
            loops: vec![cusp_lune()],
            ids: Vec::new(),
            on: None,
        }),
    );
    let (doc, extrude) = mint(
        &doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(1.0),
        },
    );
    let ev0 = eval(&doc);
    let every = editor_core::all_edges(&ev0, extrude);
    // The kiss: the one edge whose two faces are both arcs' walls, named
    // as the operand's table names it.
    let kiss = {
        let Some(NodeResult::Ok(v)) = ev0.nodes.get(&extrude) else {
            panic!("the cusp extrude builds");
        };
        let body = v.payload.output_body(0).expect("one body").body();
        let on_cylinder = |he| {
            body.get_half_edge(he)
                .and_then(|h| body.get_loop(h.parent_loop))
                .and_then(|l| body.get_face(l.face))
                .and_then(|f| body.get_surface(f.surface))
                .is_some_and(|s| matches!(s, geom::Surface::Cylinder { .. }))
        };
        let kisses: Vec<_> = body
            .edges()
            .filter(|(_, e)| on_cylinder(e.he_plus) && on_cylinder(e.he_minus))
            .map(|(k, _)| k)
            .collect();
        let [kiss] = kisses[..] else {
            panic!("the lune has one kiss strut, got {kisses:?}");
        };
        ev0.table_at(extrude)
            .expect("the extrude's table")
            .name_of(&EntityRef {
                body: 0,
                key: EntityKey::Edge(kiss),
            })
            .cloned()
            .expect("the table names the kiss")
    };
    let (doc, fillet) = mint(&doc, Node::fillet(extrude, len(0.05), every.clone()));
    let ev = eval(&doc);
    let Some(NodeResult::Failed(failure)) = ev.nodes.get(&fillet) else {
        panic!(
            "a fillet over a cusp must refuse, got {:?}",
            ev.nodes.get(&fillet)
        );
    };
    let NodeErrorKind::Blend {
        error: sweep::blend::BlendError::KnifeEdge { .. },
        edge: Some(name),
        ..
    } = &failure.kind
    else {
        panic!("expected a knife edge named in the operand's table, got {failure:?}");
    };
    assert!(
        every.contains(name),
        "the refusal names an edge of the selection: {name}"
    );
    assert_eq!(
        **name, kiss,
        "the refusal names the kiss, the knife edge itself"
    );
    let text = failure.kind.to_string();
    assert!(
        text.contains(&format!("refused at {name}")),
        "the refusal says which edge: {text}"
    );
}
