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
    Node, NodeErrorKind, NodeResult, ParamName, ProfileDoc, ProfilePayload, ProfileProgram,
    ProgramArcData, ProgramRefusal, ProgramStep, ProgramTarget, RecipeNodeId, SlotId, StepArg,
    ValuePayload, evaluate,
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

/// **The arrival spec's `Sweep`/`ArcLen`/`Bulge` argument has a role of
/// its own** — `SweepVal2`, `ArcLenVal2`, `Bulge2`.
///
/// A fused step carries two specs, and `spec_slots` enumerates the
/// arrival's roles as the spec₂ twins. With a twin for every mode, a
/// hand-built `ArcFilletArc` whose two specs share a mode addresses
/// each spec's argument exactly once; the second clause here is the
/// bijection the census walks, read at one step.
///
/// **Where the reach ends**, and it is the lattice's answer rather than
/// addressing's: `profile`'s `family::ArrivalSpec` is implemented for
/// `Center`, `Via` and `Radius` alone, so replay refuses one of these
/// three modes in second position and the VQ9 door turns the program
/// away at `InsertNode` (the persistence snapshot walk refuses the same
/// program at load). No document holds the shape, so `SetParam`,
/// `SetExpression` and `Doc::expr_at` have no node to route into — the
/// payload's `expr`/`expr_mut`, which is the half of that route the
/// three doors share and the only half a hand-built program reaches,
/// is what the clauses below exercise.
#[test]
fn the_arrival_specs_sweep_arclen_and_bulge_arguments_are_their_own_slots() {
    let len = |v: f64| Expr::literal(v, Dimension::Length).unwrap();
    let ang = |v: f64| Expr::literal(v, Dimension::Angle).unwrap();
    let sca = |v: f64| Expr::literal(v, Dimension::Scalar).unwrap();
    let sweep = |a: f64| ProgramArcData::Sweep {
        r: len(1.5),
        side: profile::ArcSide::Left,
        angle: ang(a),
    };
    let arclen = |l: f64| ProgramArcData::ArcLen {
        r: len(2.5),
        side: profile::ArcSide::Right,
        len: len(l),
    };
    let bulge = |b: f64| ProgramArcData::Bulge {
        target: ProgramTarget::Point([len(2.0), len(1.0)]),
        b: sca(b),
    };

    // (incoming spec, arrival spec, incoming role, arrival role, the
    // arrival argument's authored value, a replacement for it).
    let rows: Vec<(ProgramArcData, ProgramArcData, StepArg, StepArg, f64, Expr)> = vec![
        (
            sweep(0.25),
            sweep(0.6),
            StepArg::SweepVal,
            StepArg::SweepVal2,
            0.6,
            ang(0.7),
        ),
        (
            arclen(0.3),
            arclen(0.7),
            StepArg::ArcLenVal,
            StepArg::ArcLenVal2,
            0.7,
            len(0.8),
        ),
        (
            bulge(0.2),
            bulge(0.45),
            StepArg::Bulge,
            StepArg::Bulge2,
            0.45,
            sca(0.55),
        ),
    ];

    let doc = ProfileDoc::empty_derived("switch_slots", Tol::witness())
        .apply(
            &DocEdit::InsertNode {
                node: fixture::xy_frame(),
            },
            Tol::witness(),
        )
        .unwrap()
        .doc;

    for (spec, spec2, incoming, arrival, authored, replacement) in rows {
        let fused = SlotId::Profile {
            loop_: 0,
            step: 1,
            arg: arrival,
        };
        let mut program = ProfileProgram {
            plane: PLANE,
            loops: vec![LoopProgram::Chain(vec![
                ProgramStep::At([len(0.0), len(0.0)]),
                ProgramStep::ArcFilletArc {
                    spec,
                    radius: len(0.5),
                    spec2,
                },
            ])],
        };

        // Both roles are enumerated, once each.
        let slots = program.slots();
        for arg in [incoming, arrival] {
            let hits = slots
                .iter()
                .filter(|s| {
                    matches!(s, SlotId::Profile { step: 1, arg: a, .. } if *a == arg)
                })
                .count();
            assert_eq!(hits, 1, "{arg:?} is enumerated {hits} times at the fused step");
        }

        // …and the arrival role addresses the ARRIVAL spec's argument,
        // which is the whole of issue #829.
        assert_eq!(
            program.expr(fused).and_then(Expr::literal_value),
            Some(authored),
            "{arrival:?} addresses the arrival spec's argument"
        );
        assert_eq!(fused.dimension(), replacement.dim());

        // The write half — the path `SetParam` takes once a node is in
        // hand — reaches the arrival argument and leaves the incoming
        // one alone.
        let incoming_before = program
            .expr(SlotId::Profile {
                loop_: 0,
                step: 1,
                arg: incoming,
            })
            .and_then(Expr::literal_value);
        *program.expr_mut(fused).expect("the arrival role is writable") = replacement.clone();
        assert_eq!(
            program.expr(fused).and_then(Expr::literal_value),
            replacement.literal_value()
        );
        assert_eq!(
            program
                .expr(SlotId::Profile {
                    loop_: 0,
                    step: 1,
                    arg: incoming,
                })
                .and_then(Expr::literal_value),
            incoming_before,
            "writing the arrival argument moved the incoming one"
        );

        // The document door: the shape is refused as a lattice
        // transition, so no `SetParam`/`SetExpression`/`expr_at` row
        // can exist for these three roles.
        match doc.apply(
            &DocEdit::InsertNode {
                node: Node::Profile(program),
            },
            Tol::witness(),
        ) {
            Err(EditError::ProfileProgramRefused {
                refusal: ProgramRefusal::Transition { loop_: 0, step: 1, verb, .. },
                ..
            }) => assert_eq!(verb, Some(profile::Verb::ArcFilletArc)),
            other => panic!(
                "a {arrival:?}-carrying program must refuse at the VQ9 door, got {other:?}"
            ),
        }
    }
}
