//! **LIB-SWITCH §4c/§4d: program slot addressing and the VQ9 doors.**
//!
//! Profile nodes gained expression slots (behavior delta 3): the
//! `SlotId::Profile { loop_, step, arg }` address routes `SetParam`/
//! `SetExpression`/`Doc::expr_at` into the program; the authoring-time
//! check (VQ9) refuses program-breaking edits typed AT THE DOOR under
//! the current environment, while `SetDocParam` NEVER refuses for
//! downstream profile breakage — that surfaces as the node's typed
//! evaluation error (V1 class 2). Both directions pinned here.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use editor_core::{
    CancelToken, Dimension, DocEdit, DocParam, EditError, EvalOptions, Expr, ExprPath, LoopProgram,
    Node, NodeErrorKind, NodeResult, ParamName, ProfileDoc, ProfileProgram, ProgramRefusal,
    RecipeNodeId, SlotId, StepArg, ValuePayload, evaluate,
};
use geom_core::Tol;

/// Every document below is a frame and then the profile drawn on it,
/// in that order.
const PLANE: RecipeNodeId = RecipeNodeId(0);
const PROFILE: RecipeNodeId = RecipeNodeId(1);

fn circle_doc(r: f64) -> ProfileDoc {
    let doc = ProfileDoc::empty_derived("switch_slots", Tol::witness())
        .apply(
            &DocEdit::InsertNode {
                node: fixture::xy_frame(),
            },
            Tol::witness(),
        )
        .unwrap()
        .doc;
    doc.apply(
        &DocEdit::InsertNode {
            node: Node::Profile(ProfileProgram {
                plane: PLANE,
                loops: vec![LoopProgram::circle(0.0, 0.0, r).unwrap()],
            }),
        },
        Tol::witness(),
    )
    .unwrap()
    .doc
}

fn radius_slot() -> SlotId {
    SlotId::Profile {
        loop_: 0,
        step: 0,
        arg: StepArg::Radius,
    }
}

/// Delta 3, pinned: the formerly slot-free profile node enumerates its
/// program slots, in deterministic (loop, step, arg) order, dimensions
/// per V2's table, none structural.
#[test]
fn profile_nodes_enumerate_program_slots() {
    let doc = circle_doc(0.5);
    let Some(node) = doc.node(PROFILE) else {
        panic!("profile node");
    };
    let slots = node.slots();
    assert_eq!(
        slots,
        vec![
            SlotId::Profile {
                loop_: 0,
                step: 0,
                arg: StepArg::CenterX
            },
            SlotId::Profile {
                loop_: 0,
                step: 0,
                arg: StepArg::CenterY
            },
            radius_slot(),
        ]
    );
    for s in slots {
        assert!(!s.is_structural(), "no StepArg is structural (§4c)");
        assert!(node.expr(s).is_some(), "slots() is expr()'s domain");
    }
    assert_eq!(radius_slot().dimension(), Dimension::Length);
}

/// The continuous-edit path: `SetParam` on a program slot re-evaluates
/// to NEW geometry (the whole point of the switch).
#[test]
fn set_param_on_a_program_slot_moves_geometry() {
    let doc = circle_doc(0.5);
    let grown = doc
        .apply(
            &DocEdit::SetParam {
                node: PROFILE,
                slot: radius_slot(),
                expr: Expr::literal(0.75, Dimension::Length).unwrap(),
            },
            Tol::witness(),
        )
        .expect("a legal radius edit applies");
    assert!(
        !grown.record.structural,
        "a program slot edit is continuous"
    );
    let ev = evaluate::<f64>(
        &grown.doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    );
    let Some(v) = ev.value(PROFILE) else {
        panic!("profile evaluates");
    };
    let ValuePayload::Profile(pv) = &v.payload else {
        panic!("profile payload");
    };
    let x = pv.validated.loops()[0].vertices()[0].pos().x;
    assert_eq!(
        x.to_bits(),
        (-0.75_f64).to_bits(),
        "canonical −x pole at the new radius"
    );
}

/// `SetExpression` descends INTO a program expression with the same
/// u8 sub-paths node slots use, and `Doc::expr_at` reads them back.
#[test]
fn set_expression_and_expr_at_route_into_programs() {
    let doc = circle_doc(0.5);
    // Replace the radius with (0.5 + 0.25), then re-point its LEFT
    // literal via a sub-path edit.
    let sum = Expr::add(
        Expr::literal(0.5, Dimension::Length).unwrap(),
        Expr::literal(0.25, Dimension::Length).unwrap(),
    )
    .unwrap();
    let doc = doc
        .apply(
            &DocEdit::SetParam {
                node: PROFILE,
                slot: radius_slot(),
                expr: sum,
            },
            Tol::witness(),
        )
        .unwrap()
        .doc;
    let path = ExprPath {
        node: PROFILE,
        slot: radius_slot(),
        path: vec![0],
    };
    assert_eq!(
        doc.expr_at(&path).and_then(Expr::literal_value),
        Some(0.5),
        "expr_at descends into the program slot"
    );
    let doc = doc
        .apply(
            &DocEdit::SetExpression {
                path: path.clone(),
                expr: Expr::literal(0.375, Dimension::Length).unwrap(),
            },
            Tol::witness(),
        )
        .expect("sub-path edit applies")
        .doc;
    assert_eq!(
        doc.expr_at(&path).and_then(Expr::literal_value),
        Some(0.375)
    );
}

/// Dimension discipline at the door: a program slot refuses an
/// expression of the wrong dimension exactly as node slots do.
#[test]
fn program_slots_refuse_wrong_dimensions() {
    let doc = circle_doc(0.5);
    match doc.apply(
        &DocEdit::SetParam {
            node: PROFILE,
            slot: radius_slot(),
            expr: Expr::literal(0.5, Dimension::Angle).unwrap(),
        },
        Tol::witness(),
    ) {
        Err(EditError::SlotDimensionMismatch {
            expected: Dimension::Length,
            found: Dimension::Angle,
            ..
        }) => {}
        other => panic!("wrong dimension must refuse, got {other:?}"),
    }
}

/// VQ9 direction one: an edit that breaks the program under the
/// CURRENT env refuses typed AT THE DOOR — and WHICH geometry refusal
/// fired is read off the typed class, not the rendered sentence, so
/// rewording a `Display` arm cannot move this assertion.
#[test]
fn program_breaking_slot_edit_refuses_at_the_door() {
    let doc = circle_doc(0.5);
    match doc.apply(
        &DocEdit::SetParam {
            node: PROFILE,
            slot: radius_slot(),
            expr: Expr::literal(0.0, Dimension::Length).unwrap(),
        },
        Tol::witness(),
    ) {
        Err(EditError::ProfileProgramRefused {
            node,
            refusal:
                ProgramRefusal::Geometry {
                    loop_: 0,
                    step: 0,
                    kind,
                    ..
                },
        }) => {
            assert_eq!(node, PROFILE);
            assert_eq!(kind, profile::PathErrorKind::NonpositiveCircleRadius);
        }
        other => panic!("r = 0 must refuse at the edit door, got {other:?}"),
    }
}

/// VQ9 direction two: `SetDocParam` NEVER refuses for downstream
/// profile breakage — the broken binding surfaces as the NODE's typed
/// evaluation error naming (loop, step): V1 class 2, refusing programs
/// exist at rest.
#[test]
fn set_doc_param_never_refuses_for_downstream_profiles() {
    let doc = ProfileDoc::empty_derived("switch_slots", Tol::witness())
        .apply(
            &DocEdit::SetDocParam {
                name: ParamName::new("r"),
                value: DocParam::continuous(Dimension::Length, 0.5),
            },
            Tol::witness(),
        )
        .unwrap()
        .doc;
    let doc = doc
        .apply(
            &DocEdit::InsertNode {
                node: fixture::xy_frame(),
            },
            Tol::witness(),
        )
        .unwrap()
        .doc;
    let doc = doc
        .apply(
            &DocEdit::InsertNode {
                node: Node::Profile(ProfileProgram {
                    plane: PLANE,
                    loops: vec![LoopProgram::Circle {
                        centre: [
                            Expr::literal(0.0, Dimension::Length).unwrap(),
                            Expr::literal(0.0, Dimension::Length).unwrap(),
                        ],
                        radius: Expr::param(ParamName::new("r"), Dimension::Length),
                    }],
                }),
            },
            Tol::witness(),
        )
        .unwrap()
        .doc;
    // The breaking param edit APPLIES (never refused here)…
    let broken = doc
        .apply(
            &DocEdit::SetDocParam {
                name: ParamName::new("r"),
                value: DocParam::continuous(Dimension::Length, 0.0),
            },
            Tol::witness(),
        )
        .expect("SetDocParam never refuses for downstream profile breakage (VQ9)")
        .doc;
    // …and the refusal surfaces at evaluation, typed, naming the loop.
    let ev = evaluate::<f64>(
        &broken,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    );
    match ev.nodes.get(&PROFILE) {
        Some(NodeResult::Failed(e)) => match &e.kind {
            NodeErrorKind::ProfileReplay { loop_: 0, error } => {
                assert_eq!(error.step, 0, "the circle step names itself");
            }
            other => panic!("expected ProfileReplay, got {other:?}"),
        },
        other => panic!("the broken binding must fail the node, got {other:?}"),
    }
}

/// The payload trait's check is also the door for hand-built
/// payloads carried by InsertNode: dimension faults refuse before the
/// program ever enters the document (the check_node_slots walk).
#[test]
fn insert_node_checks_program_dimensions() {
    // The frame goes in first. The insert door checks that a node's
    // INPUTS resolve before it walks the slot dimensions, so a profile
    // naming a plane the document does not have is turned away as an
    // unresolved input — which is not the refusal this row is about.
    let doc = ProfileDoc::empty_derived("switch_slots", Tol::witness())
        .apply(
            &DocEdit::InsertNode {
                node: fixture::xy_frame(),
            },
            Tol::witness(),
        )
        .unwrap()
        .doc;
    let bad = ProfileProgram {
        plane: PLANE,
        loops: vec![LoopProgram::Circle {
            centre: [
                Expr::literal(0.0, Dimension::Length).unwrap(),
                Expr::literal(0.0, Dimension::Length).unwrap(),
            ],
            // An Angle where the Radius role demands Length.
            radius: Expr::literal(0.5, Dimension::Angle).unwrap(),
        }],
    };
    match doc.apply(
        &DocEdit::InsertNode {
            node: Node::Profile(bad),
        },
        Tol::witness(),
    ) {
        Err(EditError::SlotDimensionMismatch {
            expected: Dimension::Length,
            found: Dimension::Angle,
            ..
        }) => {}
        other => panic!("a wrong-dimension role must refuse at insert, got {other:?}"),
    }
}
