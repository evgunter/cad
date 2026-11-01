//! **From nothing to a solid, replayed headlessly** (GAUTH-1): the
//! creation op vocabulary driving the real `DocSession` with no
//! renderer — the new-document door, the profile/extrude/revolve/sketch-on-face
//! inserts with their typed refusals, the revolve tool's held pick,
//! and the ring acceptance stream.
//!
//! # The ring row's comparison, and its strength
//!
//! The acceptance stream — `NewDocument("hollow-ring")`, one profile
//! of two concentric circle loops, a full-turn revolve about an axis
//! in the profile's own plane — is asserted against the committed gallery fixture
//! (`gallery_ring.pncad`: the ring demo's document as the
//! exporter saved it, ε re-stamped per `common::gallery_ring_at`).
//! The authored document is compared under the fixture's id, because
//! the door mints a random one where the demo derives its own.
//! The comparator is **`Doc::bit_eq`** — spec D7's replay-identity
//! comparator, the strongest equality the document layer supports:
//! every float compares by bits, identity/order/placements structurally.
//! `PartialEq` would conflate `±0.0`; nothing weaker would be a claim
//! about the same document. The row goes red when the fixture is
//! regenerated from a deliberately changed demo recipe — and then the
//! question is whether the creation vocabulary still spells the new
//! recipe, never how to get the old bytes back.

// Panicking is a test's failure mechanism (workspace lint note).
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use crate::common;

use core::f64::consts::TAU;

use common::{ang, body_volume, len, len2, near, scl, session_insert, shape};
use pncad::document::{
    Dimension, DimensionError, Doc, DocumentId, Formula, LoopProgram, Node, ProfileProgram,
    RecipeNodeId, RecordedProgramError, SlotId,
};
use pncad::geom_core::Tol;
use pncad::prelude::{CapEnd, EntityKind, RoleSeg, StableName, ValuePayload};
use pncad::quantity::{WrittenAngle, WrittenLength};
use viewer::props;
use viewer::props::Notation;
use viewer::revolvetool::RevolveTool;
use viewer::seats::{Seat, SeatError, SeatEvent};
use viewer::session::{
    DocSession, FaceSelection, Hovered, NodeKindWanted, ProfileShape, Refusal, Selection,
    SessionOp, Step,
};

/// The ring demo's constants (`demos/tour/src/ring.rs`): mean radius,
/// tube outer radius, bore radius.
const R: f64 = 0.30;
const RO: f64 = 0.07;
const RI: f64 = 0.05;

/// A session over a throwaway document — creation starts from an
/// arbitrary "whatever was open".
fn session(tol: Tol) -> DocSession {
    DocSession::inline(Doc::empty_derived("creation-start", tol), tol)
}

/// A synthetic face selection for the hover row: `Hover` stores the
/// value without resolving it, so any well-formed name will do.
fn synthetic_face(node: RecipeNodeId) -> FaceSelection {
    FaceSelection {
        name: StableName {
            kind: EntityKind::Face,
            node,
            path: vec![],
        },
        node,
        body: 0,
    }
}

/// The acceptance stream: the ring, authored through the ops.
/// Answers the session and the revolve node.
fn authored_ring(tol: Tol) -> (DocSession, RecipeNodeId) {
    let mut session = session(tol);
    let outcome = session.perform(SessionOp::NewDocument {
        name: "hollow-ring".to_owned(),
    });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    let profile = session_insert(
        &mut session,
        SessionOp::AddProfile {
            loops: [RO, RI]
                .map(|radius| LoopProgram::Circle {
                    centre: [len(R), Formula::zero(Dimension::Length)],
                    radius: len(radius),
                })
                .into(),
        },
    );
    let revolve = session_insert(
        &mut session,
        SessionOp::AddRevolve {
            profile,
            // The revolve's axis, in the sketch it turns: the frame's
            // v is world +Y, so this is its own +y through (0, 0).
            axis_through: [
                Formula::zero(Dimension::Length),
                Formula::zero(Dimension::Length),
            ],
            axis_heading: Formula::turn_over(4).expect("a quarter turn"),
            angle: ang(TAU),
        },
    );
    (session, revolve)
}

/// **A revolve lands unplaced, whatever its axis reads**: the gesture
/// inserts the revolve alone — no placement, so nothing reads the ring
/// and it is drawn in its own space where it was made — and the outcome
/// carries no notice. Breaks if the gesture writes a placement beside
/// it, or says something about the ordinary answer.
#[test]
fn a_revolve_lands_unplaced_whatever_its_axis_reads() {
    let tol = Tol::witness();
    let mut session = session(tol);
    let profile = session_insert(
        &mut session,
        SessionOp::AddProfile {
            loops: vec![shape(&ProfileShape::Circle {
                centre: [R, 0.0],
                radius: RO,
            })],
        },
    );
    let outcome = session.perform(SessionOp::AddRevolve {
        profile,
        axis_through: [len(0.0), len(0.0)],
        axis_heading: Formula::turn_over(4).expect("a quarter turn"),
        angle: ang(TAU),
    });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    let [revolve] = outcome.minted[..] else {
        panic!("the revolve alone: {:?}", outcome.minted)
    };
    assert_eq!(outcome.made, Some(revolve));
    assert_eq!(
        viewer::frame::outcome_notices(&outcome).count(),
        0,
        "the ordinary answer carries no notice"
    );
    let doc = session.committed_doc();
    assert!(doc.placements().is_empty(), "nothing is placed");
    assert!(
        viewer::world::is_drawn(doc, revolve),
        "the ring is drawn in its own space"
    );
}

#[test]
fn the_ring_stream_reproduces_the_gallery_document_bit_for_bit() {
    let tol = Tol::witness();
    let (mut session, revolve) = authored_ring(tol);

    // The record of comparison: the committed gallery fixture — the
    // demo's own document through the exporter — not a hand twin that
    // could drift from both (the module header carries the strength
    // argument).
    let fixture = pncad::document::load(&common::gallery_ring_at(tol), tol)
        .expect("the gallery fixture loads at this run's ε")
        .snapshot;
    // The demo places the ring into the world and makes it the product;
    // a new body lands unplaced, so the ops stream's document gets the
    // same two edits by hand.
    let (placed, placement) = common::inserted(
        session.committed_doc(),
        common::world_place(pncad::document::Operand::output(revolve, 0)),
        tol,
    );
    let (placed, _) = common::edited(
        &placed,
        pncad::document::DocEdit::SetProduct {
            product: Some(pncad::document::Operand::Node(placement)),
        },
        tol,
    );
    assert!(
        placed.clone().under_identity(fixture.id()).bit_eq(&fixture),
        "the ops-authored ring, placed by hand, and the demo's saved \
         document are one document's content under D7's replay-identity \
         comparator"
    );
    assert_eq!(
        common::world(&placed),
        [revolve],
        "placed by hand, the world is the revolve"
    );

    // End to end: the authored document evaluates, and the revolve's
    // body is the demo's two-shell hollow ring.
    session.pump();
    let eval = session.evaluation().expect("the inline seam landed");
    match &eval.value(revolve).expect("the revolve evaluated").payload {
        ValuePayload::Body(body) => {
            assert_eq!(
                body.body().shells().count(),
                2,
                "outer torus + toroidal cavity, in one solid"
            );
        }
        other => panic!("expected a body, got {other:?}"),
    }
}

/// **A new body lands unplaced** (Ev, 2026-10-31): the extrude gesture
/// is the extrude alone, one recorded action — no placement, so the
/// body is drawn in its own space, outside the product — and a fresh
/// extrude raises only `unused`: nothing reads it, and the product
/// door is its recourse. ONE undo takes it. Breaks if the gesture
/// writes a world placement (a second insert, a placed badge), or the
/// fresh body opens with any other finding.
#[test]
fn a_fresh_extrude_lands_unplaced_and_raises_only_unused() {
    let tol = Tol::witness();
    let mut session = session(tol);
    let profile = common::rectangle_in(&mut session, 0.02, 0.01);
    let before = session.committed_doc().clone();
    let outcome = session.perform(SessionOp::AddExtrude {
        profile,
        distance: len(0.005),
    });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    let [extrude] = outcome.minted[..] else {
        panic!("one insert: {:?}", outcome.minted);
    };
    assert_eq!(outcome.made, Some(extrude), "the extrude is what it made");
    assert_eq!(outcome.committed.len(), 1, "the extrude alone");
    let doc = session.committed_doc();
    assert!(matches!(doc.node(extrude), Some(Node::Extrude { .. })));
    assert!(doc.placements().is_empty(), "nothing is placed");
    assert_eq!(
        viewer::world::drawn_nodes(doc),
        [extrude],
        "the extrude is drawn, in its own space"
    );
    assert!(
        !common::row_of(&session.tree_rows(), extrude).placed,
        "no world badge"
    );
    session.pump();
    let report = session.checks().expect("the registry ran");
    let found: Vec<_> = report.findings.iter().map(|f| f.check).collect();
    assert_eq!(
        found,
        [pncad::document::CheckId::Unused],
        "a fresh extrude raises only `unused`: {:?}",
        report.findings
    );
    assert!(session.perform(SessionOp::Undo).refusal.is_none());
    assert!(
        session.committed_doc().bit_eq(&before),
        "one undo takes the body"
    );
}

#[test]
fn the_authored_ring_survives_save_and_reload() {
    let tol = Tol::witness();
    let (mut session, _) = authored_ring(tol);
    let dir = common::tempdir("gauth1-ring");
    let path = dir.join("hollow-ring.pncad");

    let saved = session.perform(SessionOp::Save(path.clone()));
    assert!(saved.refusal.is_none(), "{:?}", saved.refusal);
    let authored = session.committed_doc().clone();

    // Reload into the same session through the ordinary open door.
    let opened = session.perform(SessionOp::Open(path));
    assert!(opened.refusal.is_none(), "{:?}", opened.refusal);
    assert!(
        session.committed_doc().bit_eq(&authored),
        "save → reload is bit-identity on the authored document"
    );
}

/// A different part than the ring through the same doors — rectangle
/// profile, extrude — with the volume as the oracle,
/// then save/reload (the volume bit-identical across it) and an
/// undo/redo walk over the creations. Promoted from the review lane's
/// e2e exercise.
#[test]
fn a_bracket_block_authors_saves_reloads_and_undoes() {
    let tol = Tol::witness();
    let mut session = session(tol);
    let out = session.perform(SessionOp::NewDocument {
        name: "bracket".to_owned(),
    });
    assert!(out.refusal.is_none(), "{:?}", out.refusal);

    // Rectangle profile → extrude: a 40 × 20 × 10 mm block.
    let (profile, extrude) = common::box_in(&mut session, [0.04, 0.02, 0.01]);
    let v = body_volume(&mut session, extrude, tol);
    let want = 0.04 * 0.02 * 0.01;
    assert!(
        ((v - want) / want).abs() < 1e-12,
        "extruded block volume: {v} vs {want}"
    );

    // Undo walks the session's actions back to the empty document, and
    // redo restores the whole of it.
    let authored = session.committed_doc().clone();
    let mut steps = 0;
    while session.perform(SessionOp::Undo).refusal.is_none() {
        steps += 1;
    }
    assert!(steps > 0, "the authoring is undoable");
    assert!(session.committed_doc().is_empty(), "back to empty");
    assert!(session.committed_doc().node(profile).is_none());
    for _ in 0..steps {
        assert!(session.perform(SessionOp::Redo).refusal.is_none());
    }
    assert!(session.committed_doc().bit_eq(&authored), "redo restores");

    // Save → reload → bit identity, and the SOLID is the same too.
    let dir = common::tempdir("gauth1-bracket");
    let path = dir.join("bracket.pncad");
    let saved = session.perform(SessionOp::Save(path.clone()));
    assert!(saved.refusal.is_none(), "{:?}", saved.refusal);
    let opened = session.perform(SessionOp::Open(path));
    assert!(opened.refusal.is_none(), "{:?}", opened.refusal);
    assert!(session.committed_doc().bit_eq(&authored));
    let v2 = body_volume(&mut session, extrude, tol);
    assert_eq!(v.to_bits(), v2.to_bits(), "same volume after reload");

    // The file is the document's state: undo ends at the open.
    assert!(matches!(
        session.perform(SessionOp::Undo).refusal,
        Some(Refusal::NothingToDo {
            direction: Step::Undo
        })
    ));
    std::fs::remove_dir_all(&dir).expect("the fixture directory is removable");
}

/// The op vocabulary deliberately exceeds the chrome's templates — a
/// rectangle outer with two circle holes is spellable through
/// `AddProfile` though no form says it — and what it authors is real:
/// the extruded plate's volume matches the closed form. Promoted from
/// the review lane's e2e exercise (the disclosed loop-list deviation,
/// probed at its full scope).
#[test]
fn the_op_vocabulary_exceeds_the_chrome_templates_and_that_works() {
    let tol = Tol::witness();
    let mut session = session(tol);
    let profile = session_insert(
        &mut session,
        SessionOp::AddProfile {
            loops: vec![
                shape(&ProfileShape::Rectangle {
                    width: 0.06,
                    height: 0.03,
                }),
                shape(&ProfileShape::Circle {
                    centre: [-0.015, 0.0],
                    radius: 0.005,
                }),
                shape(&ProfileShape::Circle {
                    centre: [0.015, 0.0],
                    radius: 0.005,
                }),
            ],
        },
    );
    let extrude = session_insert(
        &mut session,
        SessionOp::AddExtrude {
            profile,
            distance: len(0.01),
        },
    );
    let v = body_volume(&mut session, extrude, tol);
    let want = (0.06 * 0.03 - 2.0 * core::f64::consts::PI * 0.005 * 0.005) * 0.01;
    assert!(near(v, want), "plate with two bores: {v} vs {want}");
}

#[test]
fn new_document_mints_its_id_and_clears_the_session() {
    let tol = Tol::witness();
    let mut session = session(tol);
    // Give the session things to clear: a selection, a hover, and —
    // via Save — a backing path and its directory resolver.
    let profile = common::rectangle_in(&mut session, 0.02, 0.01);
    session.perform(SessionOp::Select(Selection::Node(profile)));
    session.perform(SessionOp::Hover(Some(Hovered::Face(synthetic_face(
        profile,
    )))));
    let dir = common::tempdir("gauth1-new");
    let saved = session.perform(SessionOp::Save(dir.join("old.pncad")));
    assert!(saved.refusal.is_none(), "{:?}", saved.refusal);
    assert!(session.path().is_some());
    assert!(session.resolve_dir().is_some(), "save bound the resolver");
    assert!(session.hover().is_some());

    let outcome = session.perform(SessionOp::NewDocument {
        name: "fresh-part".to_owned(),
    });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    assert!(outcome.committed.is_empty(), "a replacement, not an edit");
    let doc = session.committed_doc();
    let fresh = doc.id();
    assert_ne!(
        fresh,
        DocumentId::derive("fresh-part"),
        "the id is minted, not derived from the typed name"
    );
    assert!(doc.is_empty(), "an empty document");
    assert_eq!(session.selection(), &Selection::None);
    assert!(session.hover().is_none(), "hover cleared");
    assert!(session.path().is_none(), "no backing file until saved");
    assert!(
        session.resolve_dir().is_none(),
        "no resolver: references refuse typed until the document is saved"
    );
    assert!(
        !session.history().can_undo(),
        "the old document's history is gone, not underneath"
    );

    // One name twice is two documents.
    let outcome = session.perform(SessionOp::NewDocument {
        name: "fresh-part".to_owned(),
    });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    assert_ne!(
        session.committed_doc().id(),
        fresh,
        "a second document of one name is a second identity"
    );
}

#[test]
fn new_document_refuses_a_blank_name_and_a_gesture_in_flight() {
    let tol = Tol::witness();
    let mut session = session(tol);
    let outcome = session.perform(SessionOp::NewDocument {
        name: "   ".to_owned(),
    });
    assert!(
        matches!(outcome.refusal, Some(Refusal::EmptyName)),
        "{:?}",
        outcome.refusal
    );

    // Mid-gesture, every creation door refuses — and so does Open,
    // which shares NewDocument's policy (both replace the document a
    // drag is previewing against).
    let profile = session_insert(
        &mut session,
        SessionOp::AddProfile {
            loops: vec![shape(&ProfileShape::Circle {
                centre: [0.0, 0.0],
                radius: 0.01,
            })],
        },
    );
    let extrude = session_insert(
        &mut session,
        SessionOp::AddExtrude {
            profile,
            distance: len(0.01),
        },
    );
    let begun = session.perform(SessionOp::BeginGesture {
        node: extrude,
        slot: SlotId::Distance,
    });
    assert!(begun.refusal.is_none(), "{:?}", begun.refusal);
    for op in [
        SessionOp::NewDocument {
            name: "mid-gesture".to_owned(),
        },
        SessionOp::Open(std::env::temp_dir().join("gauth1-never-read.pncad")),
        SessionOp::SketchOnFace {
            face: FaceSelection {
                name: StableName {
                    kind: EntityKind::Face,
                    node: extrude,
                    path: vec![RoleSeg::Cap(CapEnd::End)],
                },
                node: extrude,
                body: 0,
            },
            loops: vec![common::rectangle_loop([0.0, 0.0], 0.002, 0.002)],
            distance: len(0.002),
            spin: Formula::zero(Dimension::Angle),
        },
        SessionOp::AddProfile {
            loops: vec![shape(&ProfileShape::Circle {
                centre: [0.0, 0.0],
                radius: 0.01,
            })],
        },
        SessionOp::AddExtrude {
            profile,
            distance: len(0.01),
        },
        SessionOp::AddRevolve {
            profile,
            axis_through: [
                Formula::zero(Dimension::Length),
                Formula::zero(Dimension::Length),
            ],
            axis_heading: Formula::turn_over(4).expect("a quarter turn"),
            angle: ang(TAU),
        },
    ] {
        let refused = session.perform(op);
        assert!(
            matches!(refused.refusal, Some(Refusal::GestureInFlight)),
            "{:?}",
            refused.refusal
        );
    }
}

#[test]
fn the_rectangle_template_is_the_centred_polygon() {
    let tol = Tol::witness();
    let mut session = session(tol);
    let profile = session_insert(
        &mut session,
        SessionOp::AddProfile {
            loops: vec![shape(&ProfileShape::Rectangle {
                width: 0.04,
                height: 0.02,
            })],
        },
    );
    // The session's only profile: its five steps (the start and four
    // legs) are every id the document has minted.
    let doc = session.committed_doc();
    let Some(Node::Profile(minted)) = doc.node(profile) else {
        panic!("the profile is live");
    };
    assert_eq!(
        minted
            .ids
            .iter()
            .flatten()
            .copied()
            .collect::<std::collections::BTreeSet<_>>(),
        doc.mint().steps().collect(),
        "the profile's steps are the document's only mints"
    );
    assert_eq!(minted.ids.iter().flatten().count(), 5, "five steps");
    let want = Node::Profile(ProfileProgram {
        loops: vec![
            LoopProgram::polygon([(-0.02, -0.01), (0.02, -0.01), (0.02, 0.01), (-0.02, 0.01)])
                .expect("finite corners"),
        ],
        ids: minted.ids.clone(),
        on: None,
    });
    let doc = session.committed_doc();
    assert_eq!(
        doc.node(profile).expect("the profile is live").written(doc),
        want,
        "corners at (±w/2, ±h/2), counter-clockwise from lower-left"
    );
}

/// Every profile refusal but the non-finite literal is the EDIT
/// DOOR's — the authoring-time check replaying the program — so the
/// sentence a hand-written program gets and the sentence the template
/// gets are one sentence.
#[test]
fn profile_refusals_are_typed_at_the_door() {
    let tol = Tol::witness();
    let mut session = session(tol);
    let states = session.history().len();

    // No loops: the profile layer's own refusal ("no loops — nothing
    // to sweep"), through the edit door.
    let empty = session.perform(SessionOp::AddProfile { loops: vec![] });
    assert!(
        matches!(empty.refusal, Some(Refusal::Edit(_))),
        "{:?}",
        empty.refusal
    );

    // Degenerate loops — zero and negative radius — refuse the same
    // way; no rule about them is restated in the session.
    for radius in [0.0, -0.01] {
        let degenerate = session.perform(SessionOp::AddProfile {
            loops: vec![shape(&ProfileShape::Circle {
                centre: [0.0, 0.0],
                radius,
            })],
        });
        assert!(
            matches!(degenerate.refusal, Some(Refusal::Edit(_))),
            "{:?}",
            degenerate.refusal
        );
    }

    // A non-finite field refuses at the literal door — which is now
    // BEFORE the op exists, not inside it. The op carries `Expr`s, and
    // an `Expr` cannot hold a non-finite value, so a session that
    // refused this would be refusing a state nothing can build: the
    // lowering is where the number is judged, and the form puts its
    // refusal on the status line.
    let non_finite = viewer::sketch::loop_program(
        &ProfileShape::Rectangle {
            width: f64::INFINITY,
            height: 0.01,
        },
        Notation::CANONICAL,
    );
    assert!(
        matches!(
            non_finite,
            Err(RecordedProgramError::Literal(
                DimensionError::NonFiniteLiteral
            ))
        ),
        "{non_finite:?}"
    );

    assert_eq!(
        session.history().len(),
        states,
        "a refused creation leaves no history state behind"
    );
}

/// One refusal from each of the five creation doors, and none of them
/// commits an edit or leaves a history state.
#[test]
fn a_refusal_at_any_creation_door_leaves_no_history_state() {
    let tol = Tol::witness();
    let mut session = session(tol);
    // A body: no profile to extrude or revolve, and its face is on a
    // body, so the sketch on it refuses on its depth instead.
    let block = common::xy_box_in(&mut session, [0.01, 0.01, 0.01]);
    let states = session.history().len();
    for op in [
        SessionOp::NewDocument {
            name: String::new(),
        },
        SessionOp::AddProfile { loops: vec![] },
        SessionOp::AddExtrude {
            profile: block,
            distance: len(0.01),
        },
        SessionOp::AddRevolve {
            profile: block,
            axis_through: [
                Formula::zero(Dimension::Length),
                Formula::zero(Dimension::Length),
            ],
            axis_heading: Formula::turn_over(4).expect("a quarter turn"),
            angle: ang(TAU),
        },
        // A dimensionless depth in a Length slot refuses at the edit
        // door, one rule for authored and hand-written documents.
        SessionOp::SketchOnFace {
            face: FaceSelection {
                name: StableName {
                    kind: EntityKind::Face,
                    node: block,
                    path: vec![RoleSeg::Cap(CapEnd::End)],
                },
                node: block,
                body: 0,
            },
            loops: vec![common::rectangle_loop([0.0, 0.0], 0.002, 0.002)],
            distance: scl(0.01),
            spin: Formula::zero(Dimension::Angle),
        },
    ] {
        let outcome = session.perform(op);
        assert!(outcome.refusal.is_some(), "the door refuses");
        assert!(outcome.committed.is_empty(), "and commits nothing");
    }
    assert_eq!(session.history().len(), states, "no history state minted");
}

#[test]
fn extrude_and_revolve_require_their_node_kinds() {
    let tol = Tol::witness();
    let mut session = session(tol);
    let profile = session_insert(
        &mut session,
        SessionOp::AddProfile {
            loops: vec![shape(&ProfileShape::Circle {
                centre: [R, 0.0],
                radius: RO,
            })],
        },
    );
    let extrude = session_insert(
        &mut session,
        SessionOp::AddExtrude {
            profile,
            distance: len(0.02),
        },
    );

    // The extrude door: an extrude node is not a profile, and neither
    // is an id the document never held.
    for wrong in [extrude, RecipeNodeId::new(0, 999)] {
        let refused = session.perform(SessionOp::AddExtrude {
            profile: wrong,
            distance: len(0.02),
        });
        assert!(
            matches!(
                &refused.refusal,
                Some(Refusal::WrongNodeKind { node, wanted: NodeKindWanted::Profile })
                    if node.id() == wrong
            ),
            "{:?}",
            refused.refusal
        );
    }

    // The revolve door's one seat: a feature or a placement is not a
    // profile. The axis is no pick — two coordinate pairs in the
    // profile's own plane — so there is no second seat to refuse.
    // A copy in the world, added to the product: a new body lands unplaced.
    let (mut session, copy) = common::world_copy_in(&session, extrude, tol);
    for wrong in [extrude, copy] {
        let refused = session.perform(SessionOp::AddRevolve {
            profile: wrong,
            axis_through: [
                Formula::zero(Dimension::Length),
                Formula::zero(Dimension::Length),
            ],
            axis_heading: Formula::turn_over(4).expect("a quarter turn"),
            angle: ang(TAU),
        });
        assert!(
            matches!(
                &refused.refusal,
                Some(Refusal::WrongNodeKind { node, wanted: NodeKindWanted::Profile })
                    if node.id() == wrong
            ),
            "{:?}",
            refused.refusal
        );
    }

    // The happy path inserts the revolve, its axis on the node itself.
    let revolve = session_insert(
        &mut session,
        SessionOp::AddRevolve {
            profile,
            // The frame's v is world +Y, so this is its own +y
            // through (0, 0).
            axis_through: [
                Formula::zero(Dimension::Length),
                Formula::zero(Dimension::Length),
            ],
            axis_heading: Formula::turn_over(4).expect("a quarter turn"),
            angle: ang(TAU),
        },
    );
    assert!(matches!(
        session.committed_doc().node(revolve),
        Some(Node::Revolve { .. })
    ));
}

/// The revolve tool's axis, as the chrome's default writes it: the
/// profile frame's own +y through (0, 0).
fn tool_op(tool: &RevolveTool) -> Result<SessionOp, SeatError> {
    tool.op(
        len2([0.0, 0.0]),
        Formula::turn_over(4).expect("a quarter turn"),
        ang(TAU),
    )
}

#[test]
fn the_revolve_tool_holds_its_profile_pick_and_survives_its_loss() {
    let tol = Tol::witness();
    let mut session = session(tol);
    let profile = session_insert(
        &mut session,
        SessionOp::AddProfile {
            loops: vec![shape(&ProfileShape::Circle {
                centre: [R, 0.0],
                radius: RO,
            })],
        },
    );

    let mut tool = RevolveTool::new();
    assert!(
        matches!(
            tool_op(&tool),
            Err(SeatError::Empty {
                seat: Seat::RevolveProfile
            })
        ),
        "no pick, no op"
    );
    tool.pick(session.committed_doc(), profile);
    assert_eq!(tool.profile(), Some(profile));

    // The tool's op commits exactly one insert through the session.
    let op = tool_op(&tool).expect("the seat is filled");
    let revolve = session_insert(&mut session, op);
    assert!(matches!(
        session.committed_doc().node(revolve),
        Some(Node::Revolve { .. })
    ));

    // Survival: delete the profile (which takes the revolve with it)
    // and reconcile — the seat empties with a typed event NAMING the
    // seat and the node, and the next pick refills it.
    let deleted = session.perform(SessionOp::DeleteNode { node: profile });
    assert!(deleted.refusal.is_none(), "{:?}", deleted.refusal);
    let events = tool.reconcile(session.committed_doc());
    assert_eq!(events.len(), 1, "one drop, reported");
    assert!(
        matches!(
            events.first(),
            Some(SeatEvent::PickLost {
                seat: Seat::RevolveProfile,
                node
            }) if node.id() == profile
        ),
        "the event names the emptied seat and the vanished node: {events:?}"
    );
    assert_eq!(tool.profile(), None, "the vanished pick is dropped");
    assert!(
        matches!(
            tool_op(&tool),
            Err(SeatError::Empty {
                seat: Seat::RevolveProfile
            })
        ),
        "the seat is empty, no op"
    );
    let profile = session_insert(
        &mut session,
        SessionOp::AddProfile {
            loops: vec![shape(&ProfileShape::Circle {
                centre: [R, 0.0],
                radius: RI,
            })],
        },
    );
    tool.pick(session.committed_doc(), profile);
    assert_eq!(
        tool.profile(),
        Some(profile),
        "the next pick refills the seat"
    );
    assert!(tool_op(&tool).is_ok());

    // clear() empties the seat — the chrome's start-over door.
    tool.clear();
    assert_eq!(tool.profile(), None);
}

/// A NewDocument under a held pick: a reconciling consumer hears the
/// drop, typed. (A consumer that SKIPS reconcile is not reliably
/// caught — fresh inserts re-mint the same small ids and the stale
/// pick aliases the new node; the module docs state the hazard and
/// issue #1384 tracks the class.)
#[test]
fn reconcile_drops_the_pick_across_a_new_document() {
    let tol = Tol::witness();
    let mut session = session(tol);
    let profile = session_insert(
        &mut session,
        SessionOp::AddProfile {
            loops: vec![shape(&ProfileShape::Circle {
                centre: [R, 0.0],
                radius: RO,
            })],
        },
    );
    let mut tool = RevolveTool::new();
    tool.pick(session.committed_doc(), profile);
    let out = session.perform(SessionOp::NewDocument {
        name: "fresh".to_owned(),
    });
    assert!(out.refusal.is_none(), "{:?}", out.refusal);
    let events = tool.reconcile(session.committed_doc());
    assert_eq!(events.len(), 1, "the pick dropped, loudly: {events:?}");
    assert_eq!(tool.profile(), None);
}

/// **The reported defect, end to end: a form authoring in millimetres
/// produces a document that reads back in millimetres.**
///
/// Before this, every creation op carried a bare canonical `f64`, so
/// the form's unit picker moved what was on SCREEN and nothing else:
/// the session minted a unit-less literal and the panel rendered it in
/// metres until the user reached for the panel's own picker. The op
/// vocabulary is what closed it — an `Expr` carries the notation the
/// form built it with — so this row drives the same door the chrome
/// does and asserts on what the PANEL then says, not on the op.
///
/// It walks all three shapes the report covers: a scalar slot
/// (`AddExtrude`), a vector one (`AddRevolve`, whose axis point the form
/// writes from one picker), and a profile, whose template is lowered
/// in the form's notation rather than in the session.
#[test]
fn a_form_authoring_in_millimetres_reads_back_in_millimetres() {
    let tol = Tol::witness();
    let mut session = session(tol);
    let mm = Notation {
        length: pncad::quantity::MM,
        angle: pncad::quantity::DEG,
    };

    // The extrude form's one field, authored the way the chrome does:
    // the draft is canonical, the picker says how it is written.
    let extrude_distance = Formula::written_length(WrittenLength::canonical_in(0.01, mm.length))
        .expect("10 mm is a length");
    let profile = session_insert(
        &mut session,
        SessionOp::AddProfile {
            loops: vec![
                viewer::sketch::loop_program(
                    &ProfileShape::Circle {
                        centre: [0.0, 0.0],
                        radius: 0.005,
                    },
                    mm,
                )
                .expect("a 5 mm circle"),
            ],
        },
    );
    let extrude = session_insert(
        &mut session,
        SessionOp::AddExtrude {
            profile,
            distance: extrude_distance,
        },
    );

    let row = props::slot_rows(session.committed_doc(), extrude)
        .into_iter()
        .find(|row| row.slot == SlotId::Distance)
        .expect("the extrude has a distance");
    assert_eq!(
        row.unit.map(|u| u.symbol()),
        Some("mm"),
        "the panel row remembers the form's unit, with no picker touched"
    );
    assert_eq!(
        props::field_text(&row, Notation::DEFAULT),
        "10",
        "and the field reads 10, not 0.01"
    );
    // The canonical value is untouched by any of it.
    assert_eq!(
        row.value
            .expect("a value")
            .one()
            .expect("one value")
            .as_f64()
            .to_bits(),
        0.01_f64.to_bits()
    );

    // The profile's literals took the same notation through the
    // template lowering — the form's job, not the session's.
    let radius = props::slot_rows(session.committed_doc(), profile)
        .into_iter()
        .find(|row| matches!(row.value, Ok(ref v) if v.one().expect("one value").as_f64() == 0.005))
        .expect("the circle's radius");
    assert_eq!(radius.unit.map(|u| u.symbol()), Some("mm"));

    // A vector slot: one picker, every component written in it — the
    // revolve's axis point, in the profile's own plane, clear of the
    // circle. And an ANGLE authored in degrees, the other half of the
    // picker pair — a right angle reads as 90, not as
    // 1.5707963267948966.
    let revolve = session_insert(
        &mut session,
        SessionOp::AddRevolve {
            profile,
            axis_through: [0.006, 0.001].map(|metres| {
                Formula::written_length(WrittenLength::canonical_in(metres, mm.length))
                    .expect("a finite length")
            }),
            axis_heading: Formula::turn_over(4).expect("a quarter turn"),
            angle: Formula::written_angle(WrittenAngle::canonical_in(
                core::f64::consts::FRAC_PI_2,
                mm.angle,
            ))
            .expect("a right angle"),
        },
    );
    let point: Vec<props::SlotRow> = props::slot_rows(session.committed_doc(), revolve)
        .into_iter()
        .filter(|row| matches!(row.slot, SlotId::Origin(_)))
        .collect();
    assert_eq!(point.len(), 2, "the axis point's two components");
    for row in point {
        assert_eq!(
            row.unit.map(|u| u.symbol()),
            Some("mm"),
            "every component of the vector, not just the first"
        );
    }
    let row = props::slot_rows(session.committed_doc(), revolve)
        .into_iter()
        .find(|row| row.slot == SlotId::RevolveAngle)
        .expect("the revolve has an angle");
    assert_eq!(row.unit.map(|u| u.symbol()), Some("deg"));
    assert_eq!(props::field_text(&row, Notation::DEFAULT), "90");
}

/// **From nothing to a boss on a picked face, headlessly** — the
/// gesture AUTH-1 exists for, driven through the op vocabulary alone.
///
/// A document, a square, an extrude; then the box's top cap is PICKED
/// the way the viewport picks it (a `FaceSelection` carrying the name
/// and the node whose body the ray met), and `SketchOnFace` draws a
/// circle on it, extrudes it and fuses it on — one action.
///
/// **The boss's own volume discriminates nothing** — a boss on the
/// bottom cap, on a side, or at the wrong height extrudes the same
/// cylinder — so what pins it is its placement's POSE in the block's
/// coordinates, asserted whole: origin at the cap's centre, its own z
/// along the cap's outward normal, and its own +x where a zero spin
/// puts it. A boss on the bottom cap has the opposite z and an origin
/// 10 mm below; one on a side has neither. The union's volume is the
/// evidence that the profile drew and extruded on it, flush.
#[test]
fn a_boss_is_authored_on_a_picked_face() {
    let tol = Tol::witness();
    let mut session = session(tol);
    let block = common::xy_box_in(&mut session, [0.04, 0.02, 0.01]);
    session.pump();

    // The pick, as the viewport makes it: the name, and the node whose
    // body the ray met.
    let cap = StableName {
        kind: EntityKind::Face,
        node: block,
        path: vec![RoleSeg::Cap(CapEnd::End)],
    };
    let picked = FaceSelection {
        name: cap,
        node: block,
        body: 0,
    };
    let outcome = session.perform(SessionOp::SketchOnFace {
        face: picked,
        loops: vec![shape(&ProfileShape::Circle {
            centre: [0.0, 0.0],
            radius: 0.005,
        })],
        distance: len(0.004),
        spin: Formula::zero(Dimension::Angle),
    });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    let doc = session.committed_doc();
    let of_kind = |want: fn(&Node<ProfileProgram>) -> bool| {
        outcome
            .minted
            .iter()
            .copied()
            .find(|&id| doc.node(id).is_some_and(want))
            .expect("the action minted it")
    };
    let placed = of_kind(|node| matches!(node, Node::Place { .. }));
    let union = of_kind(|node| matches!(node, Node::Union { .. }));
    let v = body_volume(&mut session, union, tol);
    let want = 0.04 * 0.02 * 0.01 + core::f64::consts::PI * 0.005 * 0.005 * 0.004;
    assert!(
        near(v, want),
        "the boss drew, extruded and fused: {v} vs {want}"
    );

    // The copy stands ON the cap: this is what "drawn on that face"
    // means geometrically, and it is the assertion a boss on another
    // face or at another height fails.
    let poses = pncad::document::solve_document(
        session.committed_doc(),
        &pncad::document::EvalOptions::default(),
        tol,
    );
    let at = poses
        .relative(placed)
        .expect("the boss stands in the block's space");
    let close = |got: [f64; 3], want: [f64; 3], what: &str| {
        for (g, w) in got.into_iter().zip(want) {
            assert!((g - w).abs() <= 1e-12, "{what}: {got:?} vs {want:?}");
        }
    };
    close(
        at.translation,
        [0.0, 0.0, 0.01],
        "the boss's origin is the cap's centre, 10 mm up",
    );
    close(
        at.columns[2],
        [0.0, 0.0, 1.0],
        "and its own z is the cap's outward normal, not the base's",
    );
    close(
        at.columns[0],
        [1.0, 0.0, 0.0],
        "and a zero spin leaves its own +x on the carrier's u-reference",
    );
}

/// **An add mints exactly the profile**: one node, the profile, on its
/// own plane.
#[test]
fn an_add_profile_mints_only_the_profile() {
    let tol = Tol::witness();
    let mut session = session(tol);
    let outcome = session.perform(SessionOp::AddProfile {
        loops: vec![shape(&ProfileShape::Circle {
            centre: [0.0, 0.0],
            radius: RO,
        })],
    });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    assert_eq!(outcome.minted.len(), 1, "{:?}", outcome.minted);
    assert!(matches!(
        session.committed_doc().node(outcome.minted[0]),
        Some(Node::Profile(_))
    ));
}

/// **A viewport pick on a revolve's body is that body, not a half**: a
/// revolve defines a body and an axis, so the pick seats its body's
/// port; the seat line says no half, and the duplicate lands.
#[test]
fn a_viewport_picked_revolve_is_its_body_not_a_split_half() {
    use viewer::combine::DuplicateTool;
    let tol = Tol::witness();
    let (mut session, revolve) = authored_ring(tol);
    session.pump();
    let doc = session.committed_doc().clone();
    let mut tool = DuplicateTool::new();
    tool.pick_at(&doc, revolve, Some(0));
    let line = viewer::seats::seat_line(tool.seats(), &doc);
    assert!(
        !line.contains("half"),
        "a revolve is said as a split half: {line}"
    );
    let outcome = session.perform(tool.op().expect("seated"));
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
}

/// **A document made in the viewer says what it makes**: two fresh
/// boxes land unplaced and are no product until the author adds them,
/// one by one, through the one door; the product is then their copies'
/// `⊔`, and taking one out leaves the other alone. The `⊔` the product
/// read before goes with the old answer.
#[test]
fn the_product_is_what_the_author_adds_to_it() {
    let tol = Tol::witness();
    let mut session = DocSession::inline(
        pncad::document::Doc::empty_derived("viewer-product-gesture", tol),
        tol,
    );
    let big = common::xy_box_in(&mut session, [0.04, 0.02, 0.01]);
    let small = common::xy_box_in(&mut session, [0.02, 0.01, 0.006]);
    assert!(
        viewer::world::product_reads(session.committed_doc()).is_empty(),
        "nothing joins the product by being made"
    );
    for body in [big, small] {
        let outcome = session.perform(SessionOp::AddToProduct {
            node: body,
            body: None,
        });
        assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    }
    let [big, small] = [big, small].map(|body| {
        let doc = session.committed_doc();
        viewer::world::world_copy(doc, common::copy_of(doc, body)).expect("a world copy")
    });
    let doc = session.committed_doc();
    assert_eq!(viewer::world::product_reads(doc), vec![big, small]);
    let assembles = |doc: &pncad::document::Doc<pncad::document::ProfileProgram>| {
        doc.ids()
            .into_iter()
            .filter(|&id| matches!(doc.node(id), Some(pncad::document::Node::Assemble { .. })))
            .count()
    };
    assert_eq!(assembles(doc), 1, "one `⊔`, the product's");
    let bodies = viewer::world::product_without(doc, big);
    let outcome = session.perform(SessionOp::SetProduct { bodies });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    let doc = session.committed_doc();
    assert_eq!(viewer::world::product_reads(doc), vec![small]);
    assert_eq!(assembles(doc), 0, "the old `⊔` went with the old answer");
}
