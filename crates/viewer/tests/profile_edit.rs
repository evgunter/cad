//! **The profile editor's edit door, headlessly**: a committed
//! profile loaded into the add-profile form's currency
//! (`sketch::held_loops`), lowered back, and committed through
//! `SessionOp::EditProfile`.
//!
//! The rows claim four things the one-editor design rests on:
//! everything the create form can author loads into the editor and
//! lowers back to the program it came from (the round trip); an
//! untouched editor applied costs nothing — no edit, no history
//! state (the no-op); an applied editor is ONE `SetProgram`, one
//! undo, whatever it changed; and what the editor cannot hold or the
//! door cannot write refuses typed, with the document left alone.

// Panicking is a test's failure mechanism (workspace lint note).
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use crate::common;

use common::{session_insert, shape};
use pncad::document::{Dimension, Doc, DocParam, ParamName};
use pncad::document::{
    DocEdit, EditError, Expr, LoopProgram, Node, ParamEnv, ProfileProgram, RecipeNodeId, SlotId,
    StepArg, StepId, apply,
};
use pncad::geom_core::{Point2, Tol};
use pncad::profile::{ArcData, ArcMode, Step, Target, TargetKind, Verb};
use viewer::session::{DocSession, ProfilePlane, ProfileShape, Refusal, SessionOp};
use viewer::sketch::{self, HeldRefusal, Notation};

/// A session over a throwaway document.
fn session(tol: Tol) -> DocSession {
    DocSession::inline(Doc::empty_derived("profile-edit", tol), tol)
}

/// The notation a person writing in millimetres and degrees mints.
const MM: Notation = Notation {
    length: pncad::quantity::MM,
    angle: pncad::quantity::DEG,
};

/// A closed square authored the way the form's default chain is.
fn square(x0: f64, side: f64) -> Vec<Step<f64>> {
    vec![
        Step::At(Point2::new(x0, 0.0)),
        Step::LineTo(Target::Point(Point2::new(x0 + side, 0.0))),
        Step::LineTo(Target::Point(Point2::new(x0 + side, side))),
        Step::LineTo(Target::Point(Point2::new(x0, side))),
        Step::LineTo(Target::Start),
    ]
}

/// Every loop list the add-profile form can author from its three
/// shapes, each as the form would hand it over.
fn authored() -> Vec<(&'static str, Vec<ProfileShape>)> {
    vec![
        (
            "circle",
            vec![ProfileShape::Circle {
                centre: [0.001, -0.002],
                radius: 0.01,
            }],
        ),
        (
            "bored circle",
            vec![
                ProfileShape::Circle {
                    centre: [0.0; 2],
                    radius: 0.01,
                },
                ProfileShape::Circle {
                    centre: [0.0; 2],
                    radius: 0.004,
                },
            ],
        ),
        (
            "rectangle",
            vec![ProfileShape::Rectangle {
                width: 0.03,
                height: 0.01,
            }],
        ),
        (
            "square path",
            vec![ProfileShape::Path {
                steps: square(0.0, 0.01),
            }],
        ),
        (
            "arc path",
            vec![ProfileShape::Path {
                steps: vec![
                    Step::At(Point2::new(-0.01, 0.0)),
                    Step::ArcTo(ArcData::Bulge {
                        target: Target::Point(Point2::new(0.01, 0.0)),
                        b: 1.0,
                    }),
                    Step::LineTo(Target::Start),
                ],
            }],
        ),
        (
            "continue-to path",
            vec![ProfileShape::Path {
                steps: vec![
                    Step::At(Point2::new(0.005, 0.0)),
                    Step::LineTo(Target::Point(Point2::new(0.01, 0.0))),
                    Step::LineTo(Target::Point(Point2::new(0.01, 0.01))),
                    Step::LineTo(Target::Point(Point2::new(0.0, 0.01))),
                    Step::LineTo(Target::Point(Point2::new(0.0, 0.005))),
                    Step::ContinueTo(Target::Point(Point2::new(0.0, 0.0))),
                    Step::LineTo(Target::StartArriving),
                ],
            }],
        ),
        (
            "tangent-arc path",
            vec![ProfileShape::Path {
                steps: vec![
                    Step::At(Point2::new(0.0, 0.0)),
                    Step::Angle(0.0),
                    Step::TangentArcTo(Target::Point(Point2::new(0.0, 0.01))),
                    Step::LineTo(Target::Start),
                ],
            }],
        ),
        (
            "split circle",
            vec![ProfileShape::Path {
                steps: vec![Step::CircleSplit {
                    centre: Point2::new(0.0, 0.0),
                    radius: 0.01,
                    n: 3,
                    phase: 0.25,
                }],
            }],
        ),
    ]
}

/// The held loops lowered the way the editor's Apply lowers them.
fn lowered(held: &[Vec<Step<f64>>], notation: Notation) -> Vec<LoopProgram> {
    sketch::path_shapes(held)
        .iter()
        .map(|shape| sketch::loop_program(shape, notation).expect("finite held numbers"))
        .collect()
}

/// A session holding one frame and one profile of `loops`, lowered in
/// `notation`; answers the session and the profile node.
fn with_profile(loops: &[ProfileShape], notation: Notation) -> (DocSession, RecipeNodeId) {
    let mut session = session(Tol::witness());
    let plane = common::xy_frame_in(&mut session);
    let loops = loops
        .iter()
        .map(|loop_| sketch::loop_program(loop_, notation).expect("a finite template"))
        .collect();
    let profile = session_insert(
        &mut session,
        SessionOp::AddProfile {
            plane: ProfilePlane::Existing(plane),
            loops,
        },
    );
    (session, profile)
}

/// The committed program of `node`.
fn program(session: &DocSession, node: RecipeNodeId) -> &ProfileProgram {
    match session.committed_doc().node(node) {
        Some(Node::Profile(program)) => program,
        other => panic!("feature {} is not a profile: {other:?}", node.0),
    }
}

/// The op the editor's Apply sends for `loops` over `node`'s committed
/// program, every step kept where it is.
fn edit_of(session: &DocSession, node: RecipeNodeId, loops: Vec<LoopProgram>) -> SessionOp {
    let base = program(session, node).clone();
    SessionOp::EditProfile {
        node,
        ids: sketch::kept_in_place(&base),
        base,
        loops,
    }
}

/// **Everything the create form authors loads, and an untouched load
/// applied is nothing.** For every shape the form offers, written in
/// either notation: the committed profile loads into the editor, its
/// held loops lower back to a program with no argument moved (in the
/// notation it was authored in AND in another — the comparison is by
/// value, so re-minting in a different unit is not an edit), and the
/// edit door handed them commits nothing, records no history state,
/// and leaves the document bit-identical.
#[test]
fn every_authored_profile_round_trips_and_an_untouched_apply_is_a_no_op() {
    for (name, loops) in authored() {
        for (authored_in, other) in [(Notation::CANONICAL, MM), (MM, Notation::CANONICAL)] {
            let (mut session, profile) = with_profile(&loops, authored_in);
            let held = sketch::held_loops(session.committed_doc(), profile)
                .unwrap_or_else(|refusal| panic!("{name}: the editor refused it: {refusal}"));
            assert_eq!(held.len(), loops.len(), "{name}: one held list per loop");
            for notation in [authored_in, other] {
                let committed = program(&session, profile);
                let lowered = lowered(&held, notation);
                assert!(
                    sketch::is_committed(committed, &lowered, &sketch::kept_in_place(committed)),
                    "{name}: an untouched load lowers to another program: {lowered:?}"
                );
            }
            let before = session.committed_doc().clone();
            let state = session.history().current();
            let out = session.perform(edit_of(&session, profile, lowered(&held, other)));
            assert!(out.refusal.is_none(), "{name}: {:?}", out.refusal);
            assert!(out.committed.is_empty(), "{name}: {:?}", out.committed);
            assert_eq!(
                session.history().current(),
                state,
                "{name}: a history state was recorded"
            );
            assert!(
                session.committed_doc().bit_eq(&before),
                "{name}: the document moved under a no-op"
            );
        }
    }
}

/// **Every verb, arc mode and target form the form's pickers offer
/// comes back as itself.** The rows above go through the document
/// door, which admits only programs that replay; this one takes each
/// fresh step the form can put in a row — valid profile or not — to
/// its program and back through the editor's loader, and asks the
/// edit door's own structural question of the result.
#[test]
fn every_verb_the_form_offers_loads_back_as_itself() {
    let mut steps: Vec<Step<f64>> = Verb::ALL
        .iter()
        .map(|&verb| sketch::fresh_step(verb))
        .collect();
    steps.extend(
        ArcMode::ALL
            .iter()
            .map(|&mode| Step::ArcTo(sketch::fresh_arc(mode))),
    );
    for &kind in TargetKind::ALL {
        steps.push(Step::LineTo(sketch::fresh_target(kind)));
    }
    let node = RecipeNodeId(1);
    for step in steps {
        let verb = step.verb();
        let program = ProfileProgram {
            plane: RecipeNodeId(0),
            loops: vec![shape(&ProfileShape::Path { steps: vec![step] })],
            ids: Vec::new(),
        };
        let held = sketch::held_program(node, &program, &ParamEnv::default())
            .unwrap_or_else(|refusal| panic!("{verb}: {refusal}"));
        let back = lowered(&held, MM);
        assert!(
            sketch::is_committed(&program, &back, &sketch::kept_in_place(&program)),
            "{verb}: came back as {back:?}"
        );
    }
}

/// The committed expression at `slot` of `node`.
fn committed_expr(session: &DocSession, node: RecipeNodeId, slot: SlotId) -> Expr {
    session
        .committed_doc()
        .node(node)
        .and_then(|held| held.expr(slot))
        .cloned()
        .unwrap_or_else(|| panic!("feature {} has no {}", node.0, slot.label()))
}

/// **A moved number is one undoable edit, written in the editor's
/// notation, and what did not move keeps its own.** The square,
/// authored in metres, has its second corner moved out by an editor
/// writing millimetres: the door commits one `SetProgram` keeping
/// every step, whose moved argument reads millimetres and whose
/// unmoved ones still read as authored; undo and redo walk it.
#[test]
fn a_moved_number_is_one_edit_and_undoes() {
    let loops = [ProfileShape::Path {
        steps: square(0.0, 0.01),
    }];
    let (mut session, profile) = with_profile(&loops, Notation::CANONICAL);
    let original = session.committed_doc().clone();
    let at = |step, arg| SlotId::Profile {
        loop_: 0,
        step,
        arg,
    };
    let (moved, unmoved) = (at(1, StepArg::TargetX), at(2, StepArg::TargetX));
    let authored_unit = committed_expr(&session, profile, unmoved).display_unit();
    assert_ne!(
        authored_unit.map(|unit| unit.symbol()),
        Some("mm"),
        "the premise: the program was not authored in the editor's notation"
    );
    let mut held = sketch::held_loops(session.committed_doc(), profile).expect("held");
    held[0][1] = Step::LineTo(Target::Point(Point2::new(0.015, 0.0)));
    let out = session.perform(edit_of(&session, profile, lowered(&held, MM)));
    assert!(out.refusal.is_none(), "{:?}", out.refusal);
    let kept = sketch::kept_in_place(original_program(&original, profile));
    assert!(
        matches!(
            out.committed.as_slice(),
            [DocEdit::SetProgram { node, ids, .. }] if *node == profile && *ids == kept
        ),
        "one whole-program edit keeping every step: {:?}",
        out.committed
    );
    let written = committed_expr(&session, profile, moved);
    assert_eq!(written.literal_value(), Some(0.015));
    assert_eq!(
        written.display_unit().map(|unit| unit.symbol()),
        Some("mm"),
        "what moved is written as the editor wrote it"
    );
    assert_eq!(
        committed_expr(&session, profile, unmoved).display_unit(),
        authored_unit,
        "what did not move keeps the notation it was authored in"
    );
    let edited = session.committed_doc().clone();
    let reloaded = sketch::held_loops(&edited, profile).expect("held");
    assert!(
        sketch::authors_same_loops(&sketch::path_shapes(&reloaded), &sketch::path_shapes(&held)),
        "the editor reloads what it applied"
    );
    assert!(session.perform(SessionOp::Undo).refusal.is_none());
    assert!(
        session.committed_doc().bit_eq(&original),
        "undo restores the program"
    );
    assert!(session.perform(SessionOp::Redo).refusal.is_none());
    assert!(session.committed_doc().bit_eq(&edited), "redo reapplies it");
}

/// `node`'s program in `doc`.
fn original_program(doc: &Doc<ProfileProgram>, node: RecipeNodeId) -> &ProfileProgram {
    match doc.node(node) {
        Some(Node::Profile(program)) => program,
        other => panic!("feature {} is not a profile: {other:?}", node.0),
    }
}

/// **A reshaped program lands as one edit, one undo, and its names
/// follow.** A step added, a verb changed and a hole added are each
/// one `SetProgram`, stated the way the editor states it: every step
/// it kept by its committed id, wherever it now stands, and every step
/// it made as new. The door keeps each kept step's id — so every name
/// that spells one still denotes that step — and mints a fresh id for
/// each new one; one undo puts the committed program back.
#[test]
fn a_reshaped_program_lands_as_one_edit_and_its_names_follow() {
    let loops = [ProfileShape::Path {
        steps: square(0.0, 0.01),
    }];
    let (mut session, profile) = with_profile(&loops, Notation::CANONICAL);
    let before = session.committed_doc().clone();
    let base = program(&session, profile).clone();
    let kept = sketch::kept_in_place(&base);
    let held = sketch::held_loops(session.committed_doc(), profile).expect("held");
    let mut longer = (held.clone(), kept.clone());
    longer.0[0].insert(4, Step::LineTo(Target::Point(Point2::new(-0.005, 0.005))));
    longer.1[0].insert(4, None);
    let mut reverbed = (held.clone(), kept.clone());
    reverbed.0[0][1] = Step::ArcTo(ArcData::Bulge {
        target: Target::Point(Point2::new(0.01, 0.0)),
        b: 0.3,
    });
    reverbed.1[0][1] = None;
    let mut two = (held.clone(), kept.clone());
    two.0.push(vec![
        Step::At(Point2::new(0.003, 0.003)),
        Step::LineTo(Target::Point(Point2::new(0.006, 0.003))),
        Step::LineTo(Target::Point(Point2::new(0.006, 0.006))),
        Step::LineTo(Target::Point(Point2::new(0.003, 0.006))),
        Step::LineTo(Target::Start),
    ]);
    two.1.push(vec![None; 5]);
    for (name, (loops, ids)) in [
        ("a step added", longer),
        ("a verb changed", reverbed),
        ("a hole added", two),
    ] {
        let state = session.history().current();
        let out = session.perform(SessionOp::EditProfile {
            node: profile,
            base: base.clone(),
            loops: lowered(&loops, Notation::CANONICAL),
            ids: ids.clone(),
        });
        assert!(out.refusal.is_none(), "{name}: {:?}", out.refusal);
        assert!(
            matches!(out.committed.as_slice(), [DocEdit::SetProgram { .. }]),
            "{name}: one whole-program edit: {:?}",
            out.committed
        );
        let now = program(&session, profile);
        let minted: Vec<StepId> = now.ids.iter().flatten().copied().collect();
        for (loop_, given) in ids.iter().enumerate() {
            for (step, kept) in given.iter().enumerate() {
                let id = now.ids[loop_][step];
                match kept {
                    Some(kept) => assert_eq!(id, *kept, "{name}: a kept step keeps its id"),
                    None => assert!(
                        !base.ids.iter().flatten().any(|old| *old == id),
                        "{name}: a new step is minted afresh, not handed a committed id"
                    ),
                }
            }
        }
        assert_eq!(
            minted.len(),
            ids.iter().map(Vec::len).sum::<usize>(),
            "{name}: one id per step"
        );
        assert!(session.perform(SessionOp::Undo).refusal.is_none());
        assert_eq!(session.history().current(), state, "{name}: one undo");
        assert!(
            session.committed_doc().bit_eq(&before),
            "{name}: the undo restores the committed program"
        );
    }
}

/// **A kept argument an expression drives is not written over.** The
/// editor cannot hold such a program, but the op is the API's too: a
/// program that moves the driven argument refuses with the slot
/// field's affordance, naming the slot; one that leaves it as it was
/// lands, and the expression is still what drives it.
#[test]
fn a_kept_driven_argument_is_not_written_over() {
    let loops = [ProfileShape::Path {
        steps: square(0.0, 0.01),
    }];
    let (mut session, profile) = with_profile(&loops, Notation::CANONICAL);
    let out = session.perform(SessionOp::CreateParam {
        name: ParamName::from_static("side"),
        value: DocParam::continuous(Dimension::Length, 0.01),
    });
    assert!(out.refusal.is_none(), "{:?}", out.refusal);
    let driven = SlotId::Profile {
        loop_: 0,
        step: 2,
        arg: StepArg::TargetX,
    };
    let out = session.perform(SessionOp::SetSlotExpression {
        node: profile,
        slot: driven,
        text: "side".to_owned(),
    });
    assert!(out.refusal.is_none(), "{:?}", out.refusal);
    let committed = program(&session, profile).loops.clone();
    let mut moved = committed.clone();
    let mut probe = Node::Profile(ProfileProgram {
        plane: program(&session, profile).plane,
        loops: moved,
        ids: Vec::new(),
    });
    *probe.expr_mut(driven).expect("the slot") = common::len(0.012);
    let Node::Profile(ProfileProgram { loops, .. }) = probe else {
        unreachable!("built as a profile")
    };
    moved = loops;
    let before = session.committed_doc().clone();
    let out = session.perform(edit_of(&session, profile, moved));
    match out.refusal {
        Some(Refusal::DrivenByExpression { node, slot, .. }) => {
            assert_eq!((node, slot), (profile, driven));
        }
        other => panic!("a driven argument was written over: {other:?}"),
    }
    assert!(session.committed_doc().bit_eq(&before));
    // The same program with another argument moved and the driven one
    // left alone lands, still driven.
    let mut probe = Node::Profile(ProfileProgram {
        plane: program(&session, profile).plane,
        loops: committed,
        ids: Vec::new(),
    });
    let other = SlotId::Profile {
        loop_: 0,
        step: 1,
        arg: StepArg::TargetX,
    };
    *probe.expr_mut(other).expect("the slot") = common::len(0.015);
    let Node::Profile(ProfileProgram { loops, .. }) = probe else {
        unreachable!("built as a profile")
    };
    let out = session.perform(edit_of(&session, profile, loops));
    assert!(out.refusal.is_none(), "{:?}", out.refusal);
    assert_eq!(
        committed_expr(&session, profile, driven).literal_value(),
        None,
        "the driven argument is still an expression"
    );
}

/// **A program the editor cannot hold refuses to load, naming why.**
/// An argument an expression drives has no plain number a `Step` could
/// hold, and writing the number it evaluates to back over it would
/// rewrite the computation — so the load refuses, naming the driven
/// argument and its source, rather than dropping or flattening it.
#[test]
fn a_driven_argument_refuses_to_load() {
    let loops = [ProfileShape::Path {
        steps: square(0.0, 0.01),
    }];
    let (mut session, profile) = with_profile(&loops, Notation::CANONICAL);
    let out = session.perform(SessionOp::CreateParam {
        name: ParamName::from_static("side"),
        value: DocParam::continuous(Dimension::Length, 0.01),
    });
    assert!(out.refusal.is_none(), "{:?}", out.refusal);
    let slot = SlotId::Profile {
        loop_: 0,
        step: 2,
        arg: StepArg::TargetX,
    };
    let out = session.perform(SessionOp::SetSlotExpression {
        node: profile,
        slot,
        text: "side".to_owned(),
    });
    assert!(out.refusal.is_none(), "{:?}", out.refusal);
    match sketch::held_loops(session.committed_doc(), profile) {
        Err(refusal @ HeldRefusal::Driven { .. }) => {
            let HeldRefusal::Driven { node, slots } = &refusal else {
                unreachable!("matched above")
            };
            assert_eq!(*node, profile);
            assert_eq!(slots.as_slice(), &[(slot, "side".to_owned())]);
            let said = refusal.to_string();
            assert!(said.contains("side"), "{said}");
            assert!(said.contains(slot.label().as_str()), "{said}");
        }
        other => panic!("a driven argument loaded: {other:?}"),
    }
    assert!(matches!(
        sketch::held_loops(session.committed_doc(), RecipeNodeId(0)),
        Err(HeldRefusal::NotAProfile { .. })
    ));
}

/// **A program that does not validate refuses as a whole, in the
/// edit door's words.** A bow-tie is not a profile; the door says so
/// about the program, and nothing lands.
#[test]
fn an_invalid_program_refuses_as_itself() {
    let loops = [ProfileShape::Path {
        steps: square(0.0, 0.01),
    }];
    let (mut session, profile) = with_profile(&loops, Notation::CANONICAL);
    let before = session.committed_doc().clone();
    let mut held = sketch::held_loops(session.committed_doc(), profile).expect("held");
    // Swap the two far corners: the loop crosses itself.
    held[0][2] = Step::LineTo(Target::Point(Point2::new(0.0, 0.01)));
    held[0][3] = Step::LineTo(Target::Point(Point2::new(0.01, 0.01)));
    let out = session.perform(edit_of(
        &session,
        profile,
        lowered(&held, Notation::CANONICAL),
    ));
    match out.refusal {
        Some(Refusal::Edit(error)) => assert!(
            matches!(*error, EditError::ProfileProgramRefused { node, .. } if node == profile),
            "{error:?}"
        ),
        other => panic!("{other:?}"),
    }
    assert!(session.committed_doc().bit_eq(&before));
}

/// **Numbers that are valid together land together, even where one
/// argument alone would not.** A square moved bodily to the right
/// crosses itself if only its first corner moves; the program as a
/// whole is valid, and lands as the one edit it is, one undo.
#[test]
fn a_move_whose_first_argument_alone_crosses_still_lands() {
    let tol = Tol::witness();
    let loops = [ProfileShape::Path {
        steps: square(0.0, 0.01),
    }];
    let (mut session, profile) = with_profile(&loops, Notation::CANONICAL);
    // The premise, checked rather than assumed: the first corner's
    // write alone is refused by the door.
    let first = DocEdit::SetParam {
        node: profile,
        slot: SlotId::Profile {
            loop_: 0,
            step: 0,
            arg: StepArg::PointX,
        },
        expr: common::len(0.02),
    };
    assert!(
        matches!(
            apply(
                session.committed_doc(),
                &first,
                tol,
                &pncad::document::RefusingReach
            ),
            Err(EditError::ProfileProgramRefused { .. })
        ),
        "the premise: the first corner alone makes a crossed loop"
    );
    let moved = square(0.02, 0.01);
    let state = session.history().current();
    let out = session.perform(edit_of(
        &session,
        profile,
        lowered(std::slice::from_ref(&moved), Notation::CANONICAL),
    ));
    assert!(out.refusal.is_none(), "{:?}", out.refusal);
    assert!(
        matches!(out.committed.as_slice(), [DocEdit::SetProgram { .. }]),
        "one whole-program edit: {:?}",
        out.committed
    );
    let held = sketch::held_loops(session.committed_doc(), profile).expect("held");
    assert!(sketch::authors_same_loops(
        &sketch::path_shapes(&held),
        &sketch::path_shapes(&[moved])
    ));
    assert!(session.perform(SessionOp::Undo).refusal.is_none());
    assert_eq!(session.history().current(), state, "one action, one undo");
}

/// **The door refuses a node that is not a profile**, as every seat
/// does.
#[test]
fn editing_a_non_profile_refuses_wrong_kind() {
    let mut session = session(Tol::witness());
    let plane = common::xy_frame_in(&mut session);
    let out = session.perform(SessionOp::EditProfile {
        node: plane,
        base: ProfileProgram {
            plane,
            loops: Vec::new(),
            ids: Vec::new(),
        },
        loops: Vec::new(),
        ids: Vec::new(),
    });
    assert!(
        matches!(out.refusal, Some(Refusal::WrongNodeKind { node, .. }) if node == plane),
        "{:?}",
        out.refusal
    );
}

/// **Numbers loaded from a program the document no longer holds are
/// not written over it.** The editor loads, an undo-like change lands
/// underneath (here, the same edit from elsewhere), and the stale
/// numbers refuse `ProfileEditStale` rather than landing on top.
#[test]
fn numbers_loaded_from_a_program_since_replaced_refuse_stale() {
    let (mut session, profile) = with_profile(
        &[ProfileShape::Path {
            steps: square(0.0, 0.01),
        }],
        Notation::CANONICAL,
    );
    let loaded = program(&session, profile).clone();
    let mut held = sketch::held_loops(session.committed_doc(), profile).expect("held");
    // Something else moves corner 2 first.
    let out = session.perform(SessionOp::SetSlot {
        node: profile,
        slot: SlotId::Profile {
            loop_: 0,
            step: 2,
            arg: StepArg::TargetY,
        },
        value: viewer::props::SlotValue::of(Dimension::Length, 0.02)
            .expect("a finite length is a value"),
    });
    assert!(out.refusal.is_none(), "{:?}", out.refusal);
    let between = session.committed_doc().clone();
    held[0][1] = Step::LineTo(Target::Point(Point2::new(0.015, 0.0)));
    let out = session.perform(SessionOp::EditProfile {
        node: profile,
        ids: sketch::kept_in_place(&loaded),
        base: loaded,
        loops: lowered(&held, Notation::CANONICAL),
    });
    assert!(
        matches!(out.refusal, Some(Refusal::ProfileEditStale { node }) if node == profile),
        "{:?}",
        out.refusal
    );
    assert!(session.committed_doc().bit_eq(&between));
}
