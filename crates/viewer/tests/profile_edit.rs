//! **The profile editor's edit door, headlessly**: a committed
//! profile loaded into the add-profile form's currency
//! (`sketch::held_loops`), lowered back, and committed through
//! `SessionOp::EditProfile`.
//!
//! The rows claim three things the one-editor design rests on:
//! everything the create form can author loads into the editor and
//! lowers back to the program it came from (the round trip); an
//! untouched editor applied costs nothing — no edit, no history
//! state (the no-op); and what the editor cannot hold or the door
//! cannot write refuses typed, with the document left alone.

// Panicking is a test's failure mechanism (workspace lint note).
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use crate::common;

use common::{insert, shape};
use pncad::document::{Dimension, Doc, DocParam, ParamName};
use pncad::document::{
    DocEdit, EditError, LoopProgram, Node, ParamEnv, ProfileProgram, RecipeNodeId, SlotId, StepArg,
    apply,
};
use pncad::geom_core::{Point2, Tol};
use pncad::profile::{ArcData, ArcMode, Step, Target, TargetKind, Verb};
use viewer::session::{DocSession, ProfilePlane, ProfileShape, Refusal, SessionOp};
use viewer::sketch::{self, HeldRefusal, Notation, Restructure};

/// A session over a throwaway document.
fn session(tol: Tol) -> DocSession {
    DocSession::inline(Doc::empty_derived("profile-edit", tol), tol)
}

/// A point of the sketch frame, in metres.
fn pt(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

/// The notation a person writing in millimetres and degrees mints.
const MM: Notation = Notation {
    length: pncad::quantity::MM,
    angle: pncad::quantity::DEG,
};

/// A closed square authored the way the form's default chain is.
fn square(x0: f64, side: f64) -> Vec<Step<f64>> {
    vec![
        Step::At(pt(x0, 0.0)),
        Step::LineTo(Target::Point(pt(x0 + side, 0.0))),
        Step::LineTo(Target::Point(pt(x0 + side, side))),
        Step::LineTo(Target::Point(pt(x0, side))),
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
                    Step::At(pt(-0.01, 0.0)),
                    Step::ArcTo(ArcData::Bulge {
                        target: Target::Point(pt(0.01, 0.0)),
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
                    Step::At(pt(0.005, 0.0)),
                    Step::LineTo(Target::Point(pt(0.01, 0.0))),
                    Step::LineTo(Target::Point(pt(0.01, 0.01))),
                    Step::LineTo(Target::Point(pt(0.0, 0.01))),
                    Step::LineTo(Target::Point(pt(0.0, 0.005))),
                    Step::ContinueTo(Target::Point(pt(0.0, 0.0))),
                    Step::LineTo(Target::StartArriving),
                ],
            }],
        ),
        (
            "tangent-arc path",
            vec![ProfileShape::Path {
                steps: vec![
                    Step::At(pt(0.0, 0.0)),
                    Step::Angle(0.0),
                    Step::TangentArcTo(Target::Point(pt(0.0, 0.01))),
                    Step::LineTo(Target::Start),
                ],
            }],
        ),
        (
            "split circle",
            vec![ProfileShape::Path {
                steps: vec![Step::CircleSplit {
                    centre: pt(0.0, 0.0),
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
    let profile = insert(
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
                let edits =
                    sketch::program_edits(program(&session, profile), &lowered(&held, notation))
                        .unwrap_or_else(|why| panic!("{name}: the round trip reshaped it: {why}"));
                assert!(
                    edits.is_empty(),
                    "{name}: an untouched load moved {edits:?}"
                );
            }
            let before = session.committed_doc().clone();
            let state = session.history().current();
            let out = session.perform(SessionOp::EditProfile {
                node: profile,
                base: program(&session, profile).clone(),
                loops: lowered(&held, other),
            });
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
        };
        let held = sketch::held_program(node, &program, &ParamEnv::default())
            .unwrap_or_else(|refusal| panic!("{verb}: {refusal}"));
        let edits = sketch::program_edits(&program, &lowered(&held, MM))
            .unwrap_or_else(|why| panic!("{verb}: {why}"));
        assert!(edits.is_empty(), "{verb}: {edits:?}");
    }
}

/// **A moved number is one undoable edit of exactly that slot.** The
/// square's second corner moves out; the door writes that one
/// argument, as one history state, and undo and redo walk it.
#[test]
fn a_moved_number_is_one_edit_and_undoes() {
    let loops = [ProfileShape::Path {
        steps: square(0.0, 0.01),
    }];
    let (mut session, profile) = with_profile(&loops, MM);
    let original = session.committed_doc().clone();
    let mut held = sketch::held_loops(session.committed_doc(), profile).expect("held");
    held[0][1] = Step::LineTo(Target::Point(pt(0.015, 0.0)));
    let out = session.perform(SessionOp::EditProfile {
        node: profile,
        base: program(&session, profile).clone(),
        loops: lowered(&held, MM),
    });
    assert!(out.refusal.is_none(), "{:?}", out.refusal);
    let slot = SlotId::Profile {
        loop_: 0,
        step: 1,
        arg: StepArg::TargetX,
    };
    assert!(
        matches!(
            out.committed.as_slice(),
            [DocEdit::SetParam { node, slot: written, expr }]
                if *node == profile && *written == slot && expr.literal_value() == Some(0.015)
        ),
        "exactly the moved argument is written: {:?}",
        out.committed
    );
    // Written in the editor's notation, as the picker beside the
    // fields says.
    let Some(DocEdit::SetParam { expr, .. }) = out.committed.first() else {
        unreachable!("matched above")
    };
    assert_eq!(expr.display_unit().map(|unit| unit.symbol()), Some("mm"));
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

/// **A reshaped program refuses, and nothing lands.** The edit
/// vocabulary writes arguments; a step added, a verb changed or a
/// loop dropped is a different program, which no edit door writes.
#[test]
fn a_reshaped_program_refuses_restructure() {
    let loops = [ProfileShape::Path {
        steps: square(0.0, 0.01),
    }];
    let (mut session, profile) = with_profile(&loops, Notation::CANONICAL);
    let before = session.committed_doc().clone();
    let held = sketch::held_loops(session.committed_doc(), profile).expect("held");
    let mut longer = held.clone();
    longer[0].insert(4, Step::LineTo(Target::Point(pt(-0.005, 0.005))));
    let mut reverbed = held.clone();
    reverbed[0][1] = Step::ContinueTo(Target::Point(pt(0.01, 0.0)));
    let mut two = held.clone();
    two.push(square(0.02, 0.005));
    for (name, loops, expected) in [
        ("a step added", longer, Restructure::Loop { loop_: 0 }),
        ("a verb changed", reverbed, Restructure::Loop { loop_: 0 }),
        (
            "a loop added",
            two,
            Restructure::LoopCount { was: 1, now: 2 },
        ),
    ] {
        let out = session.perform(SessionOp::EditProfile {
            node: profile,
            base: program(&session, profile).clone(),
            loops: lowered(&loops, Notation::CANONICAL),
        });
        match out.refusal {
            Some(Refusal::ProfileRestructure { node, why }) => {
                assert_eq!(node, profile, "{name}");
                assert_eq!(why, expected, "{name}");
            }
            other => panic!("{name}: {other:?}"),
        }
        assert!(
            session.committed_doc().bit_eq(&before),
            "{name}: the document moved"
        );
    }
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
        name: ParamName::literal("side"),
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
/// insert door's words.** A bow-tie is not a profile; the door says
/// so about the program rather than about whichever one-slot write
/// first noticed, and nothing lands.
#[test]
fn an_invalid_program_refuses_as_itself() {
    let loops = [ProfileShape::Path {
        steps: square(0.0, 0.01),
    }];
    let (mut session, profile) = with_profile(&loops, Notation::CANONICAL);
    let before = session.committed_doc().clone();
    let mut held = sketch::held_loops(session.committed_doc(), profile).expect("held");
    // Swap the two far corners: the loop crosses itself.
    held[0][2] = Step::LineTo(Target::Point(pt(0.0, 0.01)));
    held[0][3] = Step::LineTo(Target::Point(pt(0.01, 0.01)));
    let out = session.perform(SessionOp::EditProfile {
        node: profile,
        base: program(&session, profile).clone(),
        loops: lowered(&held, Notation::CANONICAL),
    });
    match out.refusal {
        Some(Refusal::Edit(error)) => assert!(
            matches!(*error, EditError::ProfileProgramRefused { node, .. } if node == profile),
            "{error:?}"
        ),
        other => panic!("{other:?}"),
    }
    assert!(session.committed_doc().bit_eq(&before));
}

/// **Numbers that are valid together land even when one write alone
/// is not.** Each one-slot write re-validates the whole program, so a
/// square moved bodily to the right cannot take its first corner
/// first: that corner alone crosses the square. The door holds the
/// refused write back until the others have made it valid, and the
/// whole move is still one action.
#[test]
fn a_move_whose_first_write_alone_crosses_still_lands() {
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
    let out = session.perform(SessionOp::EditProfile {
        node: profile,
        base: program(&session, profile).clone(),
        loops: lowered(std::slice::from_ref(&moved), Notation::CANONICAL),
    });
    assert!(out.refusal.is_none(), "{:?}", out.refusal);
    assert_eq!(
        out.committed.len(),
        4,
        "the four moved x arguments: {:?}",
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
        },
        loops: Vec::new(),
    });
    assert!(
        matches!(out.refusal, Some(Refusal::WrongNodeKind { node, .. }) if node == plane),
        "{:?}",
        out.refusal
    );
}

/// A closed polygon through `points`, in order.
fn polygon(points: &[(f64, f64)]) -> Vec<Step<f64>> {
    let mut steps = vec![Step::At(pt(points[0].0, points[0].1))];
    for &(x, y) in &points[1..] {
        steps.push(Step::LineTo(Target::Point(pt(x, y))));
    }
    steps.push(Step::LineTo(Target::Start));
    steps
}

/// **Numbers valid together that no order of one-slot writes reaches
/// refuse `ProfileEditOrder`**, and nothing lands. The pentagon pair
/// was found by `profile_edit_order`'s search; the door searched every
/// order of its writes before saying so.
#[test]
fn numbers_no_order_reaches_refuse_edit_order() {
    let base = [
        (0.721_070_807_093_289_2, 0.024_685_130_330_262_216),
        (0.106_151_747_868_981_88, 0.953_850_357_115_103_9),
        (-0.675_152_065_374_494_7, 0.226_295_352_329_219_46),
        (-0.335_828_105_609_440_3, -0.486_568_152_102_961_9),
        (0.235_145_486_038_303_8, -0.579_976_097_390_286_7),
    ];
    let target = [
        base[0],
        base[1],
        (0.691_776_936_062_037_2, 0.226_295_352_329_219_46),
        (0.893_949_020_659_274_2, -0.158_427_872_150_871_17),
        base[4],
    ];
    let (mut session, profile) = with_profile(
        &[ProfileShape::Path {
            steps: polygon(&base),
        }],
        Notation::CANONICAL,
    );
    let before = session.committed_doc().clone();
    let state = session.history().current();
    let out = session.perform(SessionOp::EditProfile {
        node: profile,
        base: program(&session, profile).clone(),
        loops: lowered(&[polygon(&target)], Notation::CANONICAL),
    });
    match out.refusal {
        Some(refusal @ Refusal::ProfileEditOrder { .. }) => {
            let said = refusal.to_string();
            assert!(said.contains("every order"), "{said}");
        }
        other => panic!("{other:?}"),
    }
    assert!(session.committed_doc().bit_eq(&before));
    assert_eq!(session.history().current(), state);
}

/// **Past the search cap, a refusal says the search was capped** —
/// not that no order exists. A heptagon turned half a turn moves all
/// fourteen of its coordinates, more than the cap, and its first
/// corner written alone crosses the loop.
#[test]
fn a_move_past_the_search_cap_says_it_was_capped() {
    use viewer::session::ORDER_SEARCH_CAP;
    let corners = |turn: f64| {
        (0..7)
            .map(|i| {
                let a = core::f64::consts::TAU * f64::from(i) / 7.0 + turn;
                (0.01 * a.cos(), 0.01 * a.sin())
            })
            .collect::<Vec<_>>()
    };
    let (mut session, profile) = with_profile(
        &[ProfileShape::Path {
            steps: polygon(&corners(0.0)),
        }],
        Notation::CANONICAL,
    );
    let before = session.committed_doc().clone();
    let out = session.perform(SessionOp::EditProfile {
        node: profile,
        base: program(&session, profile).clone(),
        loops: lowered(
            &[polygon(&corners(core::f64::consts::PI))],
            Notation::CANONICAL,
        ),
    });
    match out.refusal {
        Some(refusal @ Refusal::ProfileEditOrderCapped { writes, cap, .. }) => {
            assert_eq!((writes, cap), (14, ORDER_SEARCH_CAP));
            assert!(writes > cap);
            let said = refusal.to_string();
            assert!(said.contains("capped"), "{said}");
            assert!(!said.contains("every order"), "{said}");
        }
        other => panic!("{other:?}"),
    }
    assert!(session.committed_doc().bit_eq(&before));
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
    held[0][1] = Step::LineTo(Target::Point(pt(0.015, 0.0)));
    let out = session.perform(SessionOp::EditProfile {
        node: profile,
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
