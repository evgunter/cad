//! **From nothing to a solid, replayed headlessly** (GAUTH-1): the
//! creation op vocabulary driving the real `DocSession` with no
//! renderer — the new-document door, the datum/profile/extrude/revolve
//! inserts with their typed refusals, the revolve tool's held picks,
//! and the ring acceptance stream.
//!
//! # The ring row's comparison, and its strength
//!
//! The acceptance stream — `NewDocument("hollow-ring")`, one profile
//! of two concentric circle loops, an axis datum, a full-turn
//! revolve — is asserted against the committed gallery fixture
//! (`gallery_ring.pncad`: the ring demo's document as the
//! exporter saved it, ε re-stamped per `common::gallery_ring_at`).
//! The comparator is **`Doc::bit_eq`** — spec D7's replay-identity
//! comparator, the strongest equality the document layer supports:
//! every float compares by bits, identity/order/roots structurally.
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

use common::{ang, body_volume, len, len2, len3, near, scl, scl2, scl3, session_insert, shape};
use pncad::document::{
    Datum, Dimension, DimensionError, Doc, DocumentId, Expr, LoopProgram, Node, ProfileProgram,
    RecipeNodeId, RecordedProgramError, SlotId,
};
use pncad::geom_core::Tol;
use pncad::prelude::{CapEnd, EntityKind, RoleSeg, StableName, ValuePayload};
use pncad::quantity::{WrittenAngle, WrittenLength};
use viewer::props;
use viewer::revolvetool::RevolveTool;
use viewer::seats::{Seat, SeatError, SeatEvent};
use viewer::session::{
    DatumSpec, DocSession, FaceSelection, Hovered, NodeKindWanted, ProfilePlane, ProfileShape,
    Refusal, Selection, SessionOp, Step,
};
use viewer::sketch::Notation;

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
    let plane = common::xy_frame_in(&mut session);
    let profile = session_insert(
        &mut session,
        SessionOp::AddProfile {
            plane: ProfilePlane::Existing(plane),
            loops: vec![
                shape(&ProfileShape::Circle {
                    centre: [R, 0.0],
                    radius: RO,
                }),
                shape(&ProfileShape::Circle {
                    centre: [R, 0.0],
                    radius: RI,
                }),
            ],
        },
    );
    let axis = session_insert(
        &mut session,
        SessionOp::AddDatum {
            // The revolve's axis, in the sketch it turns: the frame's
            // v is world +Y, so this is its own +y through (0, 0).
            datum: DatumSpec::AxisInPlane {
                plane,
                origin: len2([0.0, 0.0]),
                direction: scl2([0.0, 1.0]),
            },
        },
    );
    let revolve = session_insert(
        &mut session,
        SessionOp::AddRevolve {
            profile,
            axis,
            angle: ang(TAU),
        },
    );
    (session, revolve)
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
    assert!(
        session.committed_doc().bit_eq(&fixture),
        "the ops-authored ring and the demo's saved document are one \
         document under D7's replay-identity comparator"
    );
    assert_eq!(
        session.committed_doc().roots(),
        &[revolve],
        "the revolve is the one product root"
    );

    // End to end: the authored document evaluates, and the revolve's
    // body is the demo's two-shell hollow ring.
    session.pump();
    let eval = session.evaluation().expect("the inline seam landed");
    match &eval.value(revolve).expect("the revolve evaluated").payload {
        ValuePayload::Body(body) => {
            assert_eq!(
                body.shells().count(),
                2,
                "outer torus + toroidal cavity, in one solid"
            );
        }
        other => panic!("expected a body, got {other:?}"),
    }
}

#[test]
fn the_authored_ring_survives_the_snapshot_and_log_door() {
    let tol = Tol::witness();
    let (mut session, _) = authored_ring(tol);
    let dir = common::tempdir("gauth1-ring");
    let path = dir.join("hollow-ring.pncad");

    let saved = session.perform(SessionOp::Save(path.clone()));
    assert!(saved.refusal.is_none(), "{:?}", saved.refusal);
    let authored = session.committed_doc().clone();

    // Reload into the same session through the ordinary open door —
    // the file's log replays through `apply` on the way in.
    let opened = session.perform(SessionOp::Open(path));
    assert!(opened.refusal.is_none(), "{:?}", opened.refusal);
    assert!(
        session.committed_doc().bit_eq(&authored),
        "save → reload is bit-identity on the authored document"
    );
}

/// A different part than the ring through the same doors — datum
/// plane, rectangle profile, extrude — with the volume as the oracle,
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
    assert_eq!(session.committed_doc().id(), DocumentId::derive("bracket"));

    // A datum plane (unused downstream — a root of its own).
    let _plane = session_insert(
        &mut session,
        SessionOp::AddDatum {
            datum: DatumSpec::Plane {
                origin: len3([0.0, 0.0, 0.005]),
                normal: scl3([0.0, 0.0, 1.0]),
            },
        },
    );
    // Rectangle profile → extrude: a 40 × 20 × 10 mm block.
    let plane = common::xy_frame_in(&mut session);
    let profile = session_insert(
        &mut session,
        SessionOp::AddProfile {
            plane: ProfilePlane::Existing(plane),
            loops: vec![shape(&ProfileShape::Rectangle {
                width: 0.04,
                height: 0.02,
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
    let v = body_volume(&mut session, extrude, tol);
    let want = 0.04 * 0.02 * 0.01;
    assert!(
        ((v - want) / want).abs() < 1e-12,
        "extruded block volume: {v} vs {want}"
    );

    // Save → reload → bit identity, and the SOLID is the same too.
    let dir = common::tempdir("gauth1-bracket");
    let path = dir.join("bracket.pncad");
    let saved = session.perform(SessionOp::Save(path.clone()));
    assert!(saved.refusal.is_none(), "{:?}", saved.refusal);
    let authored = session.committed_doc().clone();
    let opened = session.perform(SessionOp::Open(path));
    assert!(opened.refusal.is_none(), "{:?}", opened.refusal);
    assert!(session.committed_doc().bit_eq(&authored));
    let v2 = body_volume(&mut session, extrude, tol);
    assert_eq!(v.to_bits(), v2.to_bits(), "same volume after reload");

    // Undo walks back across the creations one at a time; redo
    // restores the whole document.
    assert!(session.perform(SessionOp::Undo).refusal.is_none());
    assert!(session.committed_doc().node(extrude).is_none());
    assert!(session.committed_doc().node(profile).is_some());
    // Four creations now, not three: the sketch frame is a node the
    // profile names, so authoring it was its own undoable step.
    for _ in 0..3 {
        assert!(session.perform(SessionOp::Undo).refusal.is_none());
    }
    assert!(session.committed_doc().order().is_empty(), "back to empty");
    let at_root = session.perform(SessionOp::Undo);
    assert!(matches!(
        at_root.refusal,
        Some(Refusal::NothingToDo {
            direction: Step::Undo
        })
    ));
    for _ in 0..4 {
        assert!(session.perform(SessionOp::Redo).refusal.is_none());
    }
    assert!(session.committed_doc().bit_eq(&authored), "redo restores");
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
    let plane = common::xy_frame_in(&mut session);
    let profile = session_insert(
        &mut session,
        SessionOp::AddProfile {
            plane: ProfilePlane::Existing(plane),
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
fn new_document_derives_its_id_and_clears_the_session() {
    let tol = Tol::witness();
    let mut session = session(tol);
    // Give the session things to clear: a selection, a hover, and —
    // via Save — a backing path and its directory resolver.
    let plane = common::xy_frame_in(&mut session);
    let profile = session_insert(
        &mut session,
        SessionOp::AddProfile {
            plane: ProfilePlane::Existing(plane),
            loops: vec![shape(&ProfileShape::Rectangle {
                width: 0.02,
                height: 0.01,
            })],
        },
    );
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
    assert_eq!(
        doc.id(),
        DocumentId::derive("fresh-part"),
        "the id is authored at creation from the typed name"
    );
    assert!(doc.order().is_empty(), "an empty document");
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

    // The trim is part of the identity rule: surrounding whitespace
    // is a typing accident, so " fresh-part " derives the SAME id.
    let outcome = session.perform(SessionOp::NewDocument {
        name: "  fresh-part  ".to_owned(),
    });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    assert_eq!(
        session.committed_doc().id(),
        DocumentId::derive("fresh-part")
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
    let plane = common::xy_frame_in(&mut session);
    let profile = session_insert(
        &mut session,
        SessionOp::AddProfile {
            plane: ProfilePlane::Existing(plane),
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
        SessionOp::AddDatum {
            datum: DatumSpec::Point {
                position: len3([0.0; 3]),
            },
        },
        SessionOp::AddProfile {
            plane: ProfilePlane::Existing(plane),
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
            axis: extrude,
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
fn each_datum_form_inserts_its_variant_with_literal_slots() {
    let tol = Tol::witness();
    let mut session = session(tol);

    let plane = session_insert(
        &mut session,
        SessionOp::AddDatum {
            datum: DatumSpec::Plane {
                origin: len3([0.0, 0.0, 0.01]),
                normal: scl3([0.0, 0.0, 1.0]),
            },
        },
    );
    let axis = session_insert(
        &mut session,
        SessionOp::AddDatum {
            datum: DatumSpec::Axis {
                origin: len3([0.0; 3]),
                direction: scl3([0.0, 0.0, 1.0]),
            },
        },
    );
    let point = session_insert(
        &mut session,
        SessionOp::AddDatum {
            datum: DatumSpec::Point {
                position: len3([0.01, 0.02, 0.03]),
            },
        },
    );
    let doc = session.committed_doc();
    let expect_bit_eq = |id: RecipeNodeId, want: Node<ProfileProgram>| {
        assert!(
            doc.node(id).expect("the datum is live").bit_eq(&want),
            "the inserted node is the literal spelling of the form"
        );
    };
    expect_bit_eq(
        plane,
        Node::Datum(Datum::Plane {
            origin: [len(0.0), len(0.0), len(0.01)],
            normal: [scl(0.0), scl(0.0), scl(1.0)],
        }),
    );
    expect_bit_eq(
        axis,
        Node::Datum(Datum::Axis {
            origin: [len(0.0), len(0.0), len(0.0)],
            direction: [scl(0.0), scl(0.0), scl(1.0)],
        }),
    );
    expect_bit_eq(
        point,
        Node::Datum(Datum::Point {
            position: [len(0.01), len(0.02), len(0.03)],
        }),
    );

    // A non-finite component refuses at the literal door, typed — and
    // that door is now BEFORE the op: an `Expr` cannot hold one, so
    // the datum spec has no spelling for a NaN origin.
    assert!(matches!(
        Expr::literal(f64::NAN, Dimension::Length),
        Err(DimensionError::NonFiniteLiteral)
    ));

    // The refusal the door itself still owns: an expression of the
    // wrong dimension for the slot, refused by the edit door.
    let refused = session.perform(SessionOp::AddDatum {
        datum: DatumSpec::Point {
            position: scl3([0.0; 3]),
        },
    });
    assert!(
        matches!(refused.refusal, Some(Refusal::Edit(_))),
        "{:?}",
        refused.refusal
    );
}

#[test]
fn the_rectangle_template_is_the_centred_polygon() {
    let tol = Tol::witness();
    let mut session = session(tol);
    let plane = common::xy_frame_in(&mut session);
    let profile = session_insert(
        &mut session,
        SessionOp::AddProfile {
            plane: ProfilePlane::Existing(plane),
            loops: vec![shape(&ProfileShape::Rectangle {
                width: 0.04,
                height: 0.02,
            })],
        },
    );
    let want = Node::Profile(ProfileProgram {
        plane,
        loops: vec![
            LoopProgram::polygon([(-0.02, -0.01), (0.02, -0.01), (0.02, 0.01), (-0.02, 0.01)])
                .expect("finite corners"),
        ],
        // The session's only profile: its five steps (the start and
        // four legs) are the document's first minted.
        ids: vec![(0..5).map(pncad::document::StepId).collect()],
    });
    assert!(
        session
            .committed_doc()
            .node(profile)
            .expect("the profile is live")
            .bit_eq(&want),
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
    // The frame lands FIRST and is not part of what this row measures:
    // the states counted are the ones a REFUSED profile might leave.
    let plane = common::xy_frame_in(&mut session);
    let states = session.history().len();

    // No loops: the profile layer's own refusal ("no loops — nothing
    // to sweep"), through the edit door.
    let empty = session.perform(SessionOp::AddProfile {
        plane: ProfilePlane::Existing(plane),
        loops: vec![],
    });
    assert!(
        matches!(empty.refusal, Some(Refusal::Edit(_))),
        "{:?}",
        empty.refusal
    );

    // Degenerate loops — zero and negative radius — refuse the same
    // way; no rule about them is restated in the session.
    for radius in [0.0, -0.01] {
        let degenerate = session.perform(SessionOp::AddProfile {
            plane: ProfilePlane::Existing(plane),
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
    // The frame comes FIRST now: the axis is written in it.
    let plane = common::xy_frame_in(&mut session);
    let axis = session_insert(
        &mut session,
        SessionOp::AddDatum {
            // The revolve's axis, in the sketch it turns: the frame's
            // v is world +Y, so this is its own +y through (0, 0).
            datum: DatumSpec::AxisInPlane {
                plane,
                origin: len2([0.0, 0.0]),
                direction: scl2([0.0, 1.0]),
            },
        },
    );
    let states = session.history().len();
    for op in [
        SessionOp::NewDocument {
            name: String::new(),
        },
        // The datum door's own refusal is now a DIMENSION one, not a
        // non-finite one: the op carries `Expr`s, so a non-finite
        // component cannot be built into it, while a dimensionless
        // expression in a Length slot can — and refuses at the edit
        // door, one rule for authored and hand-written documents.
        SessionOp::AddDatum {
            datum: DatumSpec::Point {
                position: scl3([0.0; 3]),
            },
        },
        SessionOp::AddProfile {
            plane: ProfilePlane::Existing(plane),
            loops: vec![],
        },
        SessionOp::AddExtrude {
            profile: axis,
            distance: len(0.01),
        },
        SessionOp::AddRevolve {
            profile: axis,
            axis,
            angle: ang(TAU),
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
    let plane = common::xy_frame_in(&mut session);
    let profile = session_insert(
        &mut session,
        SessionOp::AddProfile {
            plane: ProfilePlane::Existing(plane),
            loops: vec![shape(&ProfileShape::Circle {
                centre: [R, 0.0],
                radius: RO,
            })],
        },
    );
    let axis = session_insert(
        &mut session,
        SessionOp::AddDatum {
            // The revolve's axis, in the sketch it turns: the frame's
            // v is world +Y, so this is its own +y through (0, 0).
            datum: DatumSpec::AxisInPlane {
                plane,
                origin: len2([0.0, 0.0]),
                direction: scl2([0.0, 1.0]),
            },
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
    for wrong in [extrude, RecipeNodeId(999)] {
        let refused = session.perform(SessionOp::AddExtrude {
            profile: wrong,
            distance: len(0.02),
        });
        assert!(
            matches!(
                refused.refusal,
                Some(Refusal::WrongNodeKind { node, wanted: NodeKindWanted::Profile })
                    if node == wrong
            ),
            "{:?}",
            refused.refusal
        );
    }

    // The revolve door, both seats: a non-profile profile pick, then
    // a non-axis axis pick (a plane datum is not an axis either).
    let refused = session.perform(SessionOp::AddRevolve {
        profile: axis,
        axis,
        angle: ang(TAU),
    });
    assert!(
        matches!(
            refused.refusal,
            Some(Refusal::WrongNodeKind { node, wanted: NodeKindWanted::Profile })
                if node == axis
        ),
        "{:?}",
        refused.refusal
    );
    let plane = session_insert(
        &mut session,
        SessionOp::AddDatum {
            datum: DatumSpec::Plane {
                origin: len3([0.0; 3]),
                normal: scl3([0.0, 0.0, 1.0]),
            },
        },
    );
    for wrong in [extrude, plane] {
        let refused = session.perform(SessionOp::AddRevolve {
            profile,
            axis: wrong,
            angle: ang(TAU),
        });
        assert!(
            matches!(
                refused.refusal,
                Some(Refusal::WrongNodeKind { node, wanted: NodeKindWanted::SketchAxis })
                    if node == wrong
            ),
            "{:?}",
            refused.refusal
        );
    }

    // The add-datum door, for the axis a revolve takes: its frame is
    // a pick, and a plane datum or a feature is not a frame. Nothing
    // lands.
    let before = session.committed_doc().order().len();
    for wrong in [extrude, plane] {
        let refused = session.perform(SessionOp::AddDatum {
            datum: DatumSpec::AxisInPlane {
                plane: wrong,
                origin: len2([0.0, 0.0]),
                direction: scl2([0.0, 1.0]),
            },
        });
        assert!(
            matches!(
                refused.refusal,
                Some(Refusal::WrongNodeKind { node, wanted: NodeKindWanted::Frame })
                    if node == wrong
            ),
            "{:?}",
            refused.refusal
        );
    }
    assert_eq!(session.committed_doc().order().len(), before);

    // The happy path inserts the revolve with both references.
    let revolve = session_insert(
        &mut session,
        SessionOp::AddRevolve {
            profile,
            axis,
            angle: ang(TAU),
        },
    );
    assert!(matches!(
        session.committed_doc().node(revolve),
        Some(Node::Revolve { .. })
    ));
}

#[test]
fn the_revolve_tool_holds_two_picks_and_survives_a_vanished_one() {
    let tol = Tol::witness();
    let mut session = session(tol);
    let plane = common::xy_frame_in(&mut session);
    let profile = session_insert(
        &mut session,
        SessionOp::AddProfile {
            plane: ProfilePlane::Existing(plane),
            loops: vec![shape(&ProfileShape::Circle {
                centre: [R, 0.0],
                radius: RO,
            })],
        },
    );
    let axis = session_insert(
        &mut session,
        SessionOp::AddDatum {
            // The revolve's axis, in the sketch it turns: the frame's
            // v is world +Y, so this is its own +y through (0, 0).
            datum: DatumSpec::AxisInPlane {
                plane,
                origin: len2([0.0, 0.0]),
                direction: scl2([0.0, 1.0]),
            },
        },
    );

    let mut tool = RevolveTool::new();
    assert!(
        matches!(
            tool.op(ang(TAU)),
            Err(SeatError::Empty {
                seat: Seat::RevolveProfile
            })
        ),
        "no picks, no op"
    );
    tool.pick(session.committed_doc(), profile);
    assert_eq!(tool.profile(), Some(profile));
    assert!(
        matches!(
            tool.op(ang(TAU)),
            Err(SeatError::Empty {
                seat: Seat::RevolveAxis
            })
        ),
        "one pick, no op"
    );
    tool.pick(session.committed_doc(), axis);
    assert_eq!(tool.axis(), Some(axis));

    // The tool's op commits exactly one insert through the session.
    let op = tool.op(ang(TAU)).expect("both seats filled");
    let revolve = session_insert(&mut session, op);
    assert!(matches!(
        session.committed_doc().node(revolve),
        Some(Node::Revolve { .. })
    ));

    // Survival: delete the axis (which takes the revolve with it) and
    // reconcile — the axis seat empties with a typed event NAMING the
    // seat and the node, the profile stays held, and the next pick
    // refills the empty seat.
    let mut tool = RevolveTool::new();
    tool.pick(session.committed_doc(), profile);
    tool.pick(session.committed_doc(), axis);
    let deleted = session.perform(SessionOp::DeleteNode { node: axis });
    assert!(deleted.refusal.is_none(), "{:?}", deleted.refusal);
    let events = tool.reconcile(session.committed_doc());
    assert_eq!(events.len(), 1, "one drop, reported");
    assert!(
        matches!(
            events.first(),
            Some(SeatEvent::PickLost {
                seat: Seat::RevolveAxis,
                node
            }) if *node == axis
        ),
        "the event names the emptied seat and the vanished node: {events:?}"
    );
    assert_eq!(tool.profile(), Some(profile), "the live pick survives");
    assert_eq!(tool.axis(), None, "the vanished pick is dropped");
    let axis = session_insert(
        &mut session,
        SessionOp::AddDatum {
            // The revolve's axis, in the sketch it turns: the frame's
            // v is world +Y, so this is its own +y through (0, 0).
            datum: DatumSpec::AxisInPlane {
                plane,
                origin: len2([0.0, 0.0]),
                direction: scl2([0.0, 1.0]),
            },
        },
    );
    tool.pick(session.committed_doc(), axis);
    assert_eq!(tool.axis(), Some(axis), "the next pick refills the seat");
    assert!(tool.op(ang(TAU)).is_ok());
}

/// The seats are ROLES: a dropped profile leaves the axis IN the axis
/// seat (no promotion — deliberately divergent from the mate tool's
/// pair semantics; both module docs state it), and the next pick
/// refills the profile seat.
#[test]
fn a_dropped_profile_does_not_promote_the_axis() {
    let tol = Tol::witness();
    let mut session = session(tol);
    let plane = common::xy_frame_in(&mut session);
    let profile = session_insert(
        &mut session,
        SessionOp::AddProfile {
            plane: ProfilePlane::Existing(plane),
            loops: vec![shape(&ProfileShape::Circle {
                centre: [R, 0.0],
                radius: RO,
            })],
        },
    );
    let axis = session_insert(
        &mut session,
        SessionOp::AddDatum {
            // The revolve's axis, in the sketch it turns: the frame's
            // v is world +Y, so this is its own +y through (0, 0).
            datum: DatumSpec::AxisInPlane {
                plane,
                origin: len2([0.0, 0.0]),
                direction: scl2([0.0, 1.0]),
            },
        },
    );
    let mut tool = RevolveTool::new();
    tool.pick(session.committed_doc(), profile);
    tool.pick(session.committed_doc(), axis);
    // Delete the profile alone: nothing consumes it, so the cascade
    // is just the profile.
    let deleted = session.perform(SessionOp::DeleteNode { node: profile });
    assert!(deleted.refusal.is_none(), "{:?}", deleted.refusal);
    let events = tool.reconcile(session.committed_doc());
    assert!(
        matches!(
            events.first(),
            Some(SeatEvent::PickLost {
                seat: Seat::RevolveProfile,
                node
            }) if *node == profile
        ),
        "{events:?}"
    );
    assert_eq!(tool.profile(), None, "the profile seat is empty");
    assert_eq!(tool.axis(), Some(axis), "the axis STAYS in the axis seat");
    assert!(
        matches!(
            tool.op(ang(TAU)),
            Err(SeatError::Empty {
                seat: Seat::RevolveProfile
            })
        ),
        "one seat empty, no op"
    );
    // The next pick refills the PROFILE seat, not the axis.
    let plane = common::xy_frame_in(&mut session);
    let profile = session_insert(
        &mut session,
        SessionOp::AddProfile {
            plane: ProfilePlane::Existing(plane),
            loops: vec![shape(&ProfileShape::Circle {
                centre: [R, 0.0],
                radius: RI,
            })],
        },
    );
    tool.pick(session.committed_doc(), profile);
    assert_eq!(tool.profile(), Some(profile));
    assert_eq!(tool.axis(), Some(axis));
    assert!(tool.op(ang(TAU)).is_ok());

    // clear() empties both seats — the chrome's start-over door.
    tool.clear();
    assert_eq!(tool.profile(), None);
    assert_eq!(tool.axis(), None);
}

/// A NewDocument under held picks: a reconciling consumer hears both
/// drops, typed. (A consumer that SKIPS reconcile is not reliably
/// caught — fresh inserts re-mint the same small ids and the stale
/// picks alias the new nodes; the module docs state the hazard and
/// issue #1384 tracks the class.)
#[test]
fn reconcile_drops_both_picks_across_a_new_document() {
    let tol = Tol::witness();
    let mut session = session(tol);
    let plane = common::xy_frame_in(&mut session);
    let profile = session_insert(
        &mut session,
        SessionOp::AddProfile {
            plane: ProfilePlane::Existing(plane),
            loops: vec![shape(&ProfileShape::Circle {
                centre: [R, 0.0],
                radius: RO,
            })],
        },
    );
    let axis = session_insert(
        &mut session,
        SessionOp::AddDatum {
            // The revolve's axis, in the sketch it turns: the frame's
            // v is world +Y, so this is its own +y through (0, 0).
            datum: DatumSpec::AxisInPlane {
                plane,
                origin: len2([0.0, 0.0]),
                direction: scl2([0.0, 1.0]),
            },
        },
    );
    let mut tool = RevolveTool::new();
    tool.pick(session.committed_doc(), profile);
    tool.pick(session.committed_doc(), axis);
    let out = session.perform(SessionOp::NewDocument {
        name: "fresh".to_owned(),
    });
    assert!(out.refusal.is_none(), "{:?}", out.refusal);
    let events = tool.reconcile(session.committed_doc());
    assert_eq!(events.len(), 2, "both picks dropped, loudly: {events:?}");
    assert_eq!(tool.profile(), None);
    assert_eq!(tool.axis(), None);
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
/// (`AddExtrude`), a vector one (`AddDatum`, whose origin the form
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
    let extrude_distance = Expr::written_length(WrittenLength::canonical_in(0.01, mm.length))
        .expect("10 mm is a length");
    let plane = common::xy_frame_in(&mut session);
    let profile = session_insert(
        &mut session,
        SessionOp::AddProfile {
            plane: ProfilePlane::Existing(plane),
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
        props::field_text(&row),
        "10",
        "and the field reads 10, not 0.01"
    );
    // The canonical value is untouched by any of it.
    assert_eq!(
        row.value.expect("a value").as_f64().to_bits(),
        0.01_f64.to_bits()
    );

    // The profile's literals took the same notation through the
    // template lowering — the form's job, not the session's.
    let radius = props::slot_rows(session.committed_doc(), profile)
        .into_iter()
        .find(|row| matches!(row.value, Ok(ref v) if v.as_f64() == 0.005))
        .expect("the circle's radius");
    assert_eq!(radius.unit.map(|u| u.symbol()), Some("mm"));

    // A vector slot: one picker, three components, all three written
    // in it — the panel folds them into one row and one unit.
    let datum = session_insert(
        &mut session,
        SessionOp::AddDatum {
            datum: DatumSpec::Point {
                position: [0.001, 0.002, 0.003].map(|metres| {
                    Expr::written_length(WrittenLength::canonical_in(metres, mm.length))
                        .expect("a finite length")
                }),
            },
        },
    );
    for row in props::slot_rows(session.committed_doc(), datum) {
        assert_eq!(
            row.unit.map(|u| u.symbol()),
            Some("mm"),
            "every component of the vector, not just the first"
        );
    }

    // And an ANGLE authored in degrees, the other half of the picker
    // pair — a right angle reads as 90, not as 1.5707963267948966.
    let axis = session_insert(
        &mut session,
        SessionOp::AddDatum {
            // The circle is drawn on `plane`, so the revolve's axis is
            // written there too — its +y through (0, 0).
            datum: DatumSpec::AxisInPlane {
                plane,
                origin: len2([0.0, 0.0]),
                direction: scl2([0.0, 1.0]),
            },
        },
    );
    let revolve = session_insert(
        &mut session,
        SessionOp::AddRevolve {
            profile,
            axis,
            angle: Expr::written_angle(WrittenAngle::canonical_in(
                core::f64::consts::FRAC_PI_2,
                mm.angle,
            ))
            .expect("a right angle"),
        },
    );
    let row = props::slot_rows(session.committed_doc(), revolve)
        .into_iter()
        .find(|row| row.slot == SlotId::RevolveAngle)
        .expect("the revolve has an angle");
    assert_eq!(row.unit.map(|u| u.symbol()), Some("deg"));
    assert_eq!(props::field_text(&row), "90");
}

/// **From nothing to a boss on a picked face, headlessly** — the
/// gesture AUTH-1 exists for, driven through the op vocabulary alone.
///
/// A document, a frame, a square, an extrude; then the box's top cap
/// is PICKED the way the viewport picks it (a `FaceSelection` carrying
/// the name and the node whose body the ray met), the gate is asked
/// the question the button asks, `AddDatum` mints the frame on it, and
/// a profile is drawn on that frame and extruded.
///
/// **The boss's own volume discriminates nothing** — a frame on the
/// bottom cap, on a side, or at the wrong height extrudes the same
/// cylinder — so what pins the frame is its landed POSE, asserted
/// whole: origin at the cap's centre, normal along the cap's outward
/// normal, and sketch +x where a zero spin puts it. A frame read off
/// the bottom cap has the opposite normal and an origin 10 mm below;
/// one read off a side has neither; one read through a node the name
/// does not live in does not resolve at all. The volume stays as the
/// evidence that a profile drew and extruded on it.
///
/// **Not the union of block and boss.** A boss drawn on the face
/// frame is FLUSH with the block at that face by construction, and
/// this kernel refuses an undeclared coincident contact
/// (`ValidationError::UndeclaredContact`); the declaration is a
/// `Declare` node, which `SessionOp::AddBoolean` has no seat for. The
/// sum-of-volumes assertion is therefore not authorable through the
/// op vocabulary this row drives.
///
/// It is still a TWO-FORM trip for a person — add the datum, then draw
/// on it — which is the residue
/// `work/author/add-profile-mints-no-frame.md` carries.
#[test]
fn a_boss_is_authored_on_a_picked_face() {
    let tol = Tol::witness();
    let mut session = session(tol);
    let plane = common::xy_frame_in(&mut session);
    let profile = session_insert(
        &mut session,
        SessionOp::AddProfile {
            plane: ProfilePlane::Existing(plane),
            loops: vec![shape(&ProfileShape::Rectangle {
                width: 0.04,
                height: 0.02,
            })],
        },
    );
    let block = session_insert(
        &mut session,
        SessionOp::AddExtrude {
            profile,
            distance: len(0.01),
        },
    );
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
    let (at, face) = viewer::session::face_frame_seat(session.landed_pair(), Some(&picked))
        .expect("the top cap is planar and resolves");
    let frame = session_insert(
        &mut session,
        SessionOp::AddDatum {
            datum: DatumSpec::FaceFrame {
                at,
                face,
                spin: ang(0.0),
            },
        },
    );

    let boss_profile = session_insert(
        &mut session,
        SessionOp::AddProfile {
            plane: ProfilePlane::Existing(frame),
            loops: vec![shape(&ProfileShape::Circle {
                centre: [0.0, 0.0],
                radius: 0.005,
            })],
        },
    );
    let boss = session_insert(
        &mut session,
        SessionOp::AddExtrude {
            profile: boss_profile,
            distance: len(0.004),
        },
    );
    let v = body_volume(&mut session, boss, tol);
    let want = core::f64::consts::PI * 0.005 * 0.005 * 0.004;
    assert!(near(v, want), "the boss drew and extruded: {v} vs {want}");

    // The frame IS the cap, as a pose: this is what "the frame was
    // read off that face" means geometrically, and it is the assertion
    // a frame on another face, another node or another height fails.
    let ev = session.evaluation().expect("the document evaluated");
    let placed = viewer::sketch::frame_placement(session.committed_doc(), ev, frame)
        .expect("a drawable frame");
    let close = |got: [f64; 3], want: [f64; 3], what: &str| {
        for (g, w) in got.into_iter().zip(want) {
            assert!((g - w).abs() <= 1e-12, "{what}: {got:?} vs {want:?}");
        }
    };
    let origin = placed.origin();
    close(
        [origin.x, origin.y, origin.z],
        [0.0, 0.0, 0.01],
        "the frame's origin is the cap's centre, 10 mm up",
    );
    let n = placed.normal();
    close(
        [n.x, n.y, n.z],
        [0.0, 0.0, 1.0],
        "and its normal is the cap's outward normal, not the base's",
    );
    let u = placed.u();
    close(
        [u.x, u.y, u.z],
        [1.0, 0.0, 0.0],
        "and a zero spin leaves sketch +x on the carrier's u-reference",
    );
}

// ---------------------------------------------------------------
// AUTH-3: the add-profile door mints the frame it draws on.
// ---------------------------------------------------------------

/// **An empty document authors a sketch in one submit**
/// (`ProfilePlane::NewXy`): two inserts, one committed action, one
/// undo — and the frame is an ordinary node the profile NAMES.
#[test]
fn a_new_xy_plane_inserts_the_frame_and_the_profile_as_one_action() {
    let tol = Tol::witness();
    let mut session = session(tol);
    let outcome = session.perform(SessionOp::AddProfile {
        plane: ProfilePlane::NewXy,
        loops: vec![shape(&ProfileShape::Circle {
            centre: [0.0, 0.0],
            radius: RO,
        })],
    });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    assert_eq!(
        outcome.committed.len(),
        2,
        "the frame and the profile are one action's two edits"
    );
    let doc = session.committed_doc();
    let order = doc.order().to_vec();
    assert_eq!(order.len(), 2, "two nodes and no more");
    let (frame, profile) = (order[0], order[1]);
    assert!(
        matches!(doc.node(frame), Some(Node::Datum(Datum::Frame { .. }))),
        "the frame is a `Datum::Frame` node, visible and editable — not an implicit plane"
    );
    let Some(Node::Profile(program)) = doc.node(profile) else {
        panic!("the second insert is the profile")
    };
    assert_eq!(
        program.plane, frame,
        "the profile names the node the action actually minted"
    );

    // ONE undo takes both away: `commit_action`'s history group is
    // what the row's "one submit, one undo" asks for, and two commits
    // would leave the frame standing here.
    let outcome = session.perform(SessionOp::Undo);
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    assert!(
        session.committed_doc().order().is_empty(),
        "one undo took the whole gesture, frame included"
    );
}

/// **A refused profile leaves no orphan frame.** The door is
/// all-or-nothing, so the empty loop list the insert door refuses
/// takes the frame down with it.
#[test]
fn a_refused_new_xy_profile_leaves_the_document_untouched() {
    let tol = Tol::witness();
    let mut session = session(tol);
    let outcome = session.perform(SessionOp::AddProfile {
        plane: ProfilePlane::NewXy,
        loops: vec![],
    });
    assert!(
        matches!(outcome.refusal, Some(Refusal::Edit(_))),
        "an empty loop list refuses at the insert door: {:?}",
        outcome.refusal
    );
    assert!(
        outcome.committed.is_empty(),
        "nothing was recorded: {:?}",
        outcome.committed
    );
    assert!(
        session.committed_doc().order().is_empty(),
        "and no frame was left behind for the refusal to strand"
    );
}

/// **The minted frame lands where its preview drew.**
///
/// `ProfilePlane::NewXy` states the world XY frame twice — once as the
/// node's literals (`ProfilePlane::world_xy`) and once as the
/// placement the chrome previews on before that node exists
/// (`ProfilePlane::xy_placement`). This is the row that holds the two
/// ends together; it goes red the moment either moves alone.
#[test]
fn a_new_xy_frame_lands_where_its_preview_drew() {
    let tol = Tol::witness();
    let mut session = session(tol);
    let outcome = session.perform(SessionOp::AddProfile {
        plane: ProfilePlane::NewXy,
        loops: vec![shape(&ProfileShape::Circle {
            centre: [0.0, 0.0],
            radius: RO,
        })],
    });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    let frame = session.committed_doc().order()[0];
    session.pump();
    let ev = session.evaluation().expect("the document evaluated");
    let landed = viewer::sketch::frame_placement(session.committed_doc(), ev, frame)
        .expect("the minted frame draws");
    let previewed = ProfilePlane::xy_placement();
    let (o, po) = (landed.origin(), previewed.origin());
    let triples = [
        ([o.x, o.y, o.z], [po.x, po.y, po.z], "origin"),
        (xyz(landed.u()), xyz(previewed.u()), "sketch +x"),
        (xyz(landed.v()), xyz(previewed.v()), "sketch +y"),
        (xyz(landed.normal()), xyz(previewed.normal()), "normal"),
    ];
    for (got, want, what) in triples {
        for (g, w) in got.into_iter().zip(want) {
            assert!(
                (g - w).abs() <= 1e-15,
                "the landed frame and the previewed placement are one plane \
                 ({what}): {got:?} vs {want:?}"
            );
        }
    }
}

/// A direction's components, for the comparison above.
fn xyz(v: pncad::geom_core::Vec3<f64>) -> [f64; 3] {
    [v.x, v.y, v.z]
}

/// **The frame is minted BEFORE the profile that names it**, and the
/// action says so on its outcome.
///
/// The half of `Drafts::accepted`'s `NewXy` settle that lives at the
/// door: the form takes `minted.first()` as the frame it drew on, and
/// this is what makes that reading true rather than a convention two
/// modules apart agree on by luck.
#[test]
fn a_new_xy_action_mints_the_frame_before_the_profile() {
    let tol = Tol::witness();
    let mut session = session(tol);
    let outcome = session.perform(SessionOp::AddProfile {
        plane: ProfilePlane::NewXy,
        loops: vec![shape(&ProfileShape::Circle {
            centre: [0.0, 0.0],
            radius: RO,
        })],
    });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    let doc = session.committed_doc();
    let order = doc.order().to_vec();
    assert_eq!(
        outcome.minted, order,
        "both ids, in the order the action applied them: {:?}",
        outcome.minted
    );
    assert!(
        matches!(
            doc.node(outcome.minted[0]),
            Some(Node::Datum(Datum::Frame { .. }))
        ),
        "the FIRST is the frame"
    );
    assert!(
        matches!(doc.node(outcome.minted[1]), Some(Node::Profile(_))),
        "the second is the profile"
    );
}

/// An add on an existing frame mints exactly the profile — the arm
/// whose `minted.first()` is NOT a frame, which is why the form's
/// settle is gated on the choice rather than on the list.
#[test]
fn an_add_on_an_existing_frame_mints_only_the_profile() {
    let tol = Tol::witness();
    let mut session = session(tol);
    let plane = common::xy_frame_in(&mut session);
    let outcome = session.perform(SessionOp::AddProfile {
        plane: ProfilePlane::Existing(plane),
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
