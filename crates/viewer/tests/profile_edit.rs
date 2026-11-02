//! **The profile editor's edit door, headlessly**: a committed
//! profile held as it is written (`sketch::held_loops`), edited
//! through each step's view at plain numbers, and committed through
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
use pncad::document::{Dimension, Doc, FreeVar, VarName};
use pncad::document::{
    DocEdit, EditError, Formula, LoopProgram, Node, ProfileProgram, ProgramStep, ProgramTarget,
    RecipeNodeId, SlotId, StepArg, StepId, apply,
};
use pncad::geom_core::{Point2, Tol};
use pncad::profile::{ArcData, ArcMode, Step, Target, TargetKind, Verb};
use viewer::props::Notation;
use viewer::session::{DocSession, ProfileShape, Refusal, SessionOp};
use viewer::sketch::{self, HeldRefusal, HeldStep, HeldStepFault};

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

/// The held loops as the program the editor's Apply sends.
fn lowered(held: &[Vec<HeldStep>]) -> Vec<LoopProgram<Formula>> {
    held.iter()
        .map(|steps| HeldStep::program(steps).expect("a program's shape"))
        .collect()
}

/// Template loops lowered the way the create form lowers them.
fn templated(loops: &[Vec<Step<f64>>], notation: Notation) -> Vec<LoopProgram<Formula>> {
    sketch::path_shapes(loops)
        .iter()
        .map(|shape| sketch::loop_program(shape, notation).expect("finite numbers"))
        .collect()
}

/// `node`'s loops as the editor holds them.
fn held(session: &DocSession, node: RecipeNodeId) -> Vec<Vec<HeldStep>> {
    sketch::held_loops(session.committed_doc(), node)
        .unwrap_or_else(|refusal| panic!("the editor refused it: {refusal}"))
}

/// The held loops at plain numbers, what the editor's fields draw: no
/// argument here is driven, so none has a reading to take.
fn viewed(held: &[Vec<HeldStep>]) -> Vec<Vec<Step<f64>>> {
    held.iter()
        .map(|steps| {
            steps
                .iter()
                .map(|step| {
                    step.step(&|_| None, Tol::witness())
                        .unwrap_or_else(|fault| panic!("no view: {fault:?}"))
                })
                .collect()
        })
        .collect()
}

/// `step` written over held row `at`, as the row's fields write it
/// back in `notation`.
fn set(
    held: &mut [Vec<HeldStep>],
    (loop_, row): (usize, usize),
    step: Step<f64>,
    notation: Notation,
) {
    held[loop_][row] = held[loop_][row]
        .written(step, &|_| None, notation, &|_| None)
        .expect("finite");
}

/// A row the editor makes of `step`.
fn fresh(step: Step<f64>) -> HeldStep {
    HeldStep::fresh(step, Notation::CANONICAL, &|_| None).expect("finite")
}

/// Every view of `held` written back over its row in `notation`, as a
/// frame of the editor writes its fields back.
fn written_back(held: &[Vec<HeldStep>], notation: Notation) -> Vec<Vec<HeldStep>> {
    let mut back = held.to_vec();
    for (loop_, steps) in viewed(held).into_iter().enumerate() {
        for (row, step) in steps.into_iter().enumerate() {
            set(&mut back, (loop_, row), step, notation);
        }
    }
    back
}

/// The unit every argument of `held` is written in, in order.
fn units(held: &[Vec<HeldStep>]) -> Vec<Option<pncad::quantity::UnitDef>> {
    held.iter()
        .flatten()
        .flat_map(HeldStep::args)
        .map(|(_, formula)| formula.display_unit())
        .collect()
}

/// A session holding one profile of `loops`, lowered in
/// `notation`; answers the session and the profile node.
fn with_profile(loops: &[ProfileShape], notation: Notation) -> (DocSession, RecipeNodeId) {
    let mut session = session(Tol::witness());
    let loops = loops
        .iter()
        .map(|loop_| sketch::loop_program(loop_, notation).expect("a finite template"))
        .collect();
    let profile = session_insert(&mut session, SessionOp::AddProfile { loops });
    (session, profile)
}

/// The committed program of `node`.
fn program(session: &DocSession, node: RecipeNodeId) -> &ProfileProgram {
    match session.committed_doc().node(node) {
        Some(Node::Profile(program)) => program,
        other => panic!("node {node} is not a profile: {other:?}"),
    }
}

/// The op the editor's Apply sends for `loops` over `node`'s committed
/// program, every step kept where it is.
fn edit_of(
    session: &DocSession,
    node: RecipeNodeId,
    loops: Vec<LoopProgram<Formula>>,
) -> SessionOp {
    let base = sketch::written_program(session.committed_doc(), program(session, node));
    SessionOp::EditProfile {
        node,
        ids: base.kept_in_place(),
        base,
        loops,
    }
}

/// **Everything the create form authors loads, and an untouched load
/// applied is nothing.** For every shape the form offers, written in
/// either notation: the committed profile loads into the editor as it
/// is written; every step's view at plain numbers, written back over it
/// in either notation, leaves it as written, unit and all; and the edit
/// door handed the held program commits nothing, records no history
/// state, and leaves the document bit-identical.
#[test]
fn every_authored_profile_round_trips_and_an_untouched_apply_is_a_no_op() {
    for (name, loops) in authored() {
        for (authored_in, other) in [(Notation::CANONICAL, MM), (MM, Notation::CANONICAL)] {
            let (mut session, profile) = with_profile(&loops, authored_in);
            let held = held(&session, profile);
            assert_eq!(held.len(), loops.len(), "{name}: one held list per loop");
            let committed =
                sketch::written_program(session.committed_doc(), program(&session, profile));
            assert_eq!(lowered(&held), committed.loops, "{name}: held as written");
            for notation in [authored_in, other] {
                let back = written_back(&held, notation);
                assert_eq!(back, held, "{name}: an unmoved view moved its step");
                assert_eq!(
                    units(&back),
                    units(&held),
                    "{name}: an unmoved view rewrote a unit in {notation:?}"
                );
            }
            let before = session.committed_doc().clone();
            let state = session.history().current();
            let out = session.perform(edit_of(
                &session,
                profile,
                lowered(&written_back(&held, other)),
            ));
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
/// its program, holds it through the editor's loader, and writes its
/// view back over it.
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
    let node = RecipeNodeId::new(0, 1);
    for step in steps {
        let verb = step.verb();
        let mut doc = pncad::document::Doc::empty_derived("held-program", Tol::witness());
        let program = editor_core::test_support::stored_program(
            &mut doc,
            &ProfileProgram {
                loops: vec![shape(&ProfileShape::Path { steps: vec![step] })],
                ids: Vec::new(),
                on: None,
            },
        );
        sketch::held_program(pncad::document::SpokenNode::absent(node), &program)
            .unwrap_or_else(|refusal| panic!("{verb}: {refusal}"));
        let held = sketch::held_rows(&sketch::written_program(&doc, &program));
        let back = written_back(&held, MM);
        assert_eq!(back, held, "{verb}: came back as {back:?}");
        let written = sketch::written_program(&doc, &program);
        assert!(
            sketch::is_committed(&written, &lowered(&back), &written.kept_in_place()),
            "{verb}: lowered as {:?}",
            lowered(&back)
        );
    }
}

/// The committed expression at `slot` of `node`.
fn committed_expr(
    session: &DocSession,
    node: RecipeNodeId,
    slot: SlotId,
) -> pncad::document::Formula {
    session
        .committed_doc()
        .slot_expansion(node, slot)
        .unwrap_or_else(|| panic!("node {node} has no {}", slot.label()))
}

/// **A moved number is one undoable edit, written in the unit its
/// field shows, and what did not move keeps its own.** The square,
/// authored in metres, has its second corner moved out by an editor
/// writing millimetres: the field over a literal shows the unit the
/// literal is written in, so the door commits one `SetProgram` keeping
/// every step, whose moved argument still reads metres and whose
/// unmoved ones read as authored; undo and redo walk it.
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
    let mut held = held(&session, profile);
    let moved_to = Step::LineTo(Target::Point(Point2::new(0.015, 0.0)));
    set(&mut held, (0, 1), moved_to, MM);
    let out = session.perform(edit_of(&session, profile, lowered(&held)));
    assert!(out.refusal.is_none(), "{:?}", out.refusal);
    let kept = original_program(&original, profile).kept_in_place();
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
        written.display_unit(),
        authored_unit,
        "what moved is written in the unit its field showed it in, the one it was written in, \
         not the editor's notation"
    );
    assert_eq!(
        committed_expr(&session, profile, unmoved).display_unit(),
        authored_unit,
        "what did not move keeps the notation it was authored in"
    );
    let edited = session.committed_doc().clone();
    let reloaded = sketch::held_loops(&edited, profile).expect("held");
    assert_eq!(reloaded, held, "the editor reloads what it applied");
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
        other => panic!("node {node} is not a profile: {other:?}"),
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
    let base = sketch::written_program(session.committed_doc(), program(&session, profile));
    let kept = base.kept_in_place();
    let held = held(&session, profile);
    let mut longer = (held.clone(), kept.clone());
    longer.0[0].insert(
        4,
        fresh(Step::LineTo(Target::Point(Point2::new(-0.005, 0.005)))),
    );
    longer.1[0].insert(4, None);
    let mut reverbed = (held.clone(), kept.clone());
    reverbed.0[0][1] = fresh(Step::ArcTo(ArcData::Bulge {
        target: Target::Point(Point2::new(0.01, 0.0)),
        b: 0.3,
    }));
    reverbed.1[0][1] = None;
    let mut two = (held.clone(), kept.clone());
    two.0.push(
        [
            Step::At(Point2::new(0.003, 0.003)),
            Step::LineTo(Target::Point(Point2::new(0.006, 0.003))),
            Step::LineTo(Target::Point(Point2::new(0.006, 0.006))),
            Step::LineTo(Target::Point(Point2::new(0.003, 0.006))),
            Step::LineTo(Target::Start),
        ]
        .into_iter()
        .map(fresh)
        .collect(),
    );
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
            loops: lowered(&loops),
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
/// editor writes such an argument back as its expression, but the op
/// is the API's too: a program that moves the driven argument refuses
/// with the slot field's affordance, naming the slot; one that leaves
/// it as it was lands, and the expression is still what drives it.
#[test]
fn a_kept_driven_argument_is_not_written_over() {
    let loops = [ProfileShape::Path {
        steps: square(0.0, 0.01),
    }];
    let (mut session, profile) = with_profile(&loops, Notation::CANONICAL);
    let out = session.perform(SessionOp::DeclareVar {
        name: VarName::from_static("side"),
        value: FreeVar::continuous(Dimension::Length, 0.01),
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
    let committed: Vec<LoopProgram<Formula>> = program(&session, profile)
        .loops
        .iter()
        .map(LoopProgram::authored)
        .collect();
    let mut moved = committed.clone();
    let mut probe = Node::Profile(ProfileProgram {
        loops: moved,
        ids: Vec::new(),
        on: None,
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
        loops: committed,
        ids: Vec::new(),
        on: None,
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

/// **A driven argument loads as its expression.** The held step is
/// written as the program writes it, so the argument is the expression
/// itself; its view at plain numbers reads the expression's value
/// beside it — which is what the preview draws — and has none without
/// one; and the view written back over it, the number its reading,
/// keeps the expression. A node that is no profile still refuses.
#[test]
fn a_driven_argument_loads_as_its_expression() {
    let loops = [ProfileShape::Path {
        steps: square(0.0, 0.01),
    }];
    let (mut session, profile) = with_profile(&loops, Notation::CANONICAL);
    let out = session.perform(SessionOp::DeclareVar {
        name: VarName::from_static("side"),
        value: FreeVar::continuous(Dimension::Length, 0.013),
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
    let held = held(&session, profile);
    let corner = &held[0][2];
    let driven = |step: &HeldStep| {
        step.args()
            .into_iter()
            .find(|(arg, _)| *arg == StepArg::TargetX)
            .is_some_and(|(_, formula)| formula.literal_value().is_none())
    };
    assert!(driven(corner), "held as its expression");
    let reading = |arg| (arg == StepArg::TargetX).then_some(0.013);
    let view = corner
        .step(&reading, Tol::witness())
        .expect("a view beside its reading");
    match view {
        Step::LineTo(Target::Point(point)) => assert_eq!(
            (point.x, point.y),
            (0.013, 0.01),
            "the view holds what the expression reads"
        ),
        other => panic!("the corner step viewed as {other:?}"),
    }
    assert!(
        matches!(
            corner.step(&|_| None, Tol::witness()),
            Err(HeldStepFault::Unread(StepArg::TargetX))
        ),
        "with no reading there is no number to view"
    );
    let back = corner
        .written(view, &reading, Notation::CANONICAL, &|_| None)
        .expect("finite");
    assert!(driven(&back), "written back, still driven");
    assert!(matches!(
        sketch::held_loops(session.committed_doc(), RecipeNodeId::new(0, 0)),
        Err(HeldRefusal::NotAProfile { .. })
    ));
}

/// **A profile with a step that runs to a written coordinate opens,
/// and its other steps edit.** `line_to(at_x)` lowers against the pen
/// its prefix leaves, which only an evaluation replays, so the editor
/// has no view of it at plain numbers and holds it as written; the
/// steps beside it are views like any other. Two corners moved, Apply
/// is one `SetProgram` keeping every step, the pen step among them as
/// it was written.
#[test]
fn a_step_running_to_a_written_coordinate_opens_and_its_neighbours_edit() {
    let len = common::len;
    let chain = LoopProgram::Chain(vec![
        ProgramStep::At([len(0.0), len(0.0)]),
        ProgramStep::Angle(Formula::literal(0.0, Dimension::Angle).expect("finite")),
        ProgramStep::LineTo(ProgramTarget::AtX(len(0.01))),
        ProgramStep::LineTo(ProgramTarget::Point([len(0.01), len(0.01)])),
        ProgramStep::LineTo(ProgramTarget::Point([len(0.0), len(0.01)])),
        ProgramStep::LineTo(ProgramTarget::Start),
    ]);
    let mut session = session(Tol::witness());
    let profile = session_insert(&mut session, SessionOp::AddProfile { loops: vec![chain] });
    let mut held = held(&session, profile);
    let pen = &held[0][2];
    assert!(
        matches!(pen.step(&|_| None, Tol::witness()), Err(HeldStepFault::Pen)),
        "the pen step has no view outside an evaluation"
    );
    assert!(pen.words().starts_with("line_to"), "{}", pen.words());
    let canonical = Notation::CANONICAL;
    set(
        &mut held,
        (0, 3),
        Step::LineTo(Target::Point(Point2::new(0.01, 0.02))),
        canonical,
    );
    set(
        &mut held,
        (0, 4),
        Step::LineTo(Target::Point(Point2::new(0.0, 0.02))),
        canonical,
    );
    let kept = program(&session, profile).kept_in_place();
    let out = session.perform(edit_of(&session, profile, lowered(&held)));
    assert!(out.refusal.is_none(), "{:?}", out.refusal);
    assert!(
        matches!(
            out.committed.as_slice(),
            [DocEdit::SetProgram { ids, .. }] if *ids == kept
        ),
        "one whole-program edit keeping every step: {:?}",
        out.committed
    );
    let written = sketch::written_program(session.committed_doc(), program(&session, profile));
    let LoopProgram::Chain(steps) = &written.loops[0] else {
        panic!("a chain")
    };
    assert!(
        matches!(&steps[2], ProgramStep::LineTo(ProgramTarget::AtX(x)) if x.literal_value() == Some(0.01)),
        "the pen step is kept as written: {:?}",
        steps[2]
    );
    let far_y = SlotId::Profile {
        loop_: 0,
        step: 3,
        arg: StepArg::TargetY,
    };
    assert_eq!(
        committed_expr(&session, profile, far_y).literal_value(),
        Some(0.02)
    );
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
    let mut held = held(&session, profile);
    // Swap the two far corners: the loop crosses itself.
    let canonical = Notation::CANONICAL;
    set(
        &mut held,
        (0, 2),
        Step::LineTo(Target::Point(Point2::new(0.0, 0.01))),
        canonical,
    );
    set(
        &mut held,
        (0, 3),
        Step::LineTo(Target::Point(Point2::new(0.01, 0.01))),
        canonical,
    );
    let out = session.perform(edit_of(&session, profile, lowered(&held)));
    match out.refusal {
        Some(Refusal::Edit(error)) => assert!(
            matches!(&*error, EditError::ProfileProgramRefused { node, .. } if node.id() == profile),
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
        value: common::len(0.02).into(),
        fresh: Vec::new(),
    };
    assert!(
        matches!(
            apply(session.committed_doc(), &first, tol),
            Err(EditError::ProfileProgramRefused { .. })
        ),
        "the premise: the first corner alone makes a crossed loop"
    );
    let moved = square(0.02, 0.01);
    let state = session.history().current();
    let out = session.perform(edit_of(
        &session,
        profile,
        templated(std::slice::from_ref(&moved), Notation::CANONICAL),
    ));
    assert!(out.refusal.is_none(), "{:?}", out.refusal);
    assert!(
        matches!(out.committed.as_slice(), [DocEdit::SetProgram { .. }]),
        "one whole-program edit: {:?}",
        out.committed
    );
    let held = held(&session, profile);
    assert!(sketch::authors_same_loops(
        &sketch::path_shapes(&viewed(&held)),
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
    let block = common::xy_box_in(&mut session, [0.01, 0.01, 0.01]);
    let out = session.perform(SessionOp::EditProfile {
        node: block,
        base: ProfileProgram {
            loops: Vec::new(),
            ids: Vec::new(),
            on: None,
        },
        loops: Vec::new(),
        ids: Vec::new(),
    });
    assert!(
        matches!(&out.refusal, Some(Refusal::WrongNodeKind { node, .. }) if node.id() == block),
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
    session.perform(SessionOp::SetLabel {
        node: profile,
        label: Some(pncad::document::Label::new("outline").expect("a label")),
    });
    let loaded = sketch::written_program(session.committed_doc(), program(&session, profile));
    let mut held = held(&session, profile);
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
    let moved_to = Step::LineTo(Target::Point(Point2::new(0.015, 0.0)));
    set(&mut held, (0, 1), moved_to, Notation::CANONICAL);
    let out = session.perform(SessionOp::EditProfile {
        node: profile,
        ids: loaded.kept_in_place(),
        base: loaded,
        loops: lowered(&held),
    });
    assert!(
        matches!(&out.refusal, Some(Refusal::ProfileEditStale { node }) if node.id() == profile
            && node.to_string().starts_with("Profile \"outline\" (")),
        "{:?}",
        out.refusal
    );
    assert!(session.committed_doc().bit_eq(&between));
}
