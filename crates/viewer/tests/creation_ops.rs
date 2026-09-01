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
//! (`gallery_ring.v19.pncad`: the ring demo's document as the
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

mod common;

use core::f64::consts::TAU;

use common::{body_volume, insert, near};
use pncad::document::{
    Datum, Dimension, Doc, DocumentId, Expr, LoopProgram, Node, ProfileProgram, RecipeNodeId,
    SlotId,
};
use pncad::geom_core::Tol;
use pncad::prelude::{EntityKind, StableName, ValuePayload};
use pncad::profile::SketchPlane;
use viewer::revolvetool::RevolveTool;
use viewer::seats::{Seat, SeatError, SeatEvent};
use viewer::session::{
    DatumSpec, DocSession, FaceSelection, Hovered, NodeKindWanted, ProfileShape, Refusal,
    Selection, SessionOp,
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
    let profile = insert(
        &mut session,
        SessionOp::AddProfile {
            plane: SketchPlane::xy(),
            loops: vec![
                ProfileShape::Circle {
                    centre: [R, 0.0],
                    radius: RO,
                },
                ProfileShape::Circle {
                    centre: [R, 0.0],
                    radius: RI,
                },
            ],
        },
    );
    let axis = insert(
        &mut session,
        SessionOp::AddDatum {
            datum: DatumSpec::Axis {
                origin: [0.0; 3],
                direction: [0.0, 1.0, 0.0],
            },
        },
    );
    let revolve = insert(
        &mut session,
        SessionOp::AddRevolve {
            profile,
            axis,
            angle: TAU,
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
    let _plane = insert(
        &mut session,
        SessionOp::AddDatum {
            datum: DatumSpec::Plane {
                origin: [0.0, 0.0, 0.005],
                normal: [0.0, 0.0, 1.0],
            },
        },
    );
    // Rectangle profile → extrude: a 40 × 20 × 10 mm block.
    let profile = insert(
        &mut session,
        SessionOp::AddProfile {
            plane: SketchPlane::xy(),
            loops: vec![ProfileShape::Rectangle {
                width: 0.04,
                height: 0.02,
            }],
        },
    );
    let extrude = insert(
        &mut session,
        SessionOp::AddExtrude {
            profile,
            distance: 0.01,
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
    assert!(session.perform(SessionOp::Undo).refusal.is_none());
    assert!(session.perform(SessionOp::Undo).refusal.is_none());
    assert!(session.committed_doc().order().is_empty(), "back to empty");
    let at_root = session.perform(SessionOp::Undo);
    assert!(matches!(at_root.refusal, Some(Refusal::NothingToDo)));
    for _ in 0..3 {
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
    let profile = insert(
        &mut session,
        SessionOp::AddProfile {
            plane: SketchPlane::xy(),
            loops: vec![
                ProfileShape::Rectangle {
                    width: 0.06,
                    height: 0.03,
                },
                ProfileShape::Circle {
                    centre: [-0.015, 0.0],
                    radius: 0.005,
                },
                ProfileShape::Circle {
                    centre: [0.015, 0.0],
                    radius: 0.005,
                },
            ],
        },
    );
    let extrude = insert(
        &mut session,
        SessionOp::AddExtrude {
            profile,
            distance: 0.01,
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
    let profile = insert(
        &mut session,
        SessionOp::AddProfile {
            plane: SketchPlane::xy(),
            loops: vec![ProfileShape::Rectangle {
                width: 0.02,
                height: 0.01,
            }],
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
    let profile = insert(
        &mut session,
        SessionOp::AddProfile {
            plane: SketchPlane::xy(),
            loops: vec![ProfileShape::Circle {
                centre: [0.0, 0.0],
                radius: 0.01,
            }],
        },
    );
    let extrude = insert(
        &mut session,
        SessionOp::AddExtrude {
            profile,
            distance: 0.01,
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
            datum: DatumSpec::Point { position: [0.0; 3] },
        },
        SessionOp::AddProfile {
            plane: SketchPlane::xy(),
            loops: vec![ProfileShape::Circle {
                centre: [0.0, 0.0],
                radius: 0.01,
            }],
        },
        SessionOp::AddExtrude {
            profile,
            distance: 0.01,
        },
        SessionOp::AddRevolve {
            profile,
            axis: extrude,
            angle: TAU,
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
    let len = |v: f64| Expr::literal(v, Dimension::Length).expect("a length");
    let scl = |v: f64| Expr::literal(v, Dimension::Scalar).expect("a scalar");

    let plane = insert(
        &mut session,
        SessionOp::AddDatum {
            datum: DatumSpec::Plane {
                origin: [0.0, 0.0, 0.01],
                normal: [0.0, 0.0, 1.0],
            },
        },
    );
    let axis = insert(
        &mut session,
        SessionOp::AddDatum {
            datum: DatumSpec::Axis {
                origin: [0.0; 3],
                direction: [0.0, 0.0, 1.0],
            },
        },
    );
    let point = insert(
        &mut session,
        SessionOp::AddDatum {
            datum: DatumSpec::Point {
                position: [0.01, 0.02, 0.03],
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

    // A non-finite component refuses at the literal door, typed.
    let refused = session.perform(SessionOp::AddDatum {
        datum: DatumSpec::Point {
            position: [f64::NAN, 0.0, 0.0],
        },
    });
    assert!(
        matches!(refused.refusal, Some(Refusal::Dimension(_))),
        "{:?}",
        refused.refusal
    );
}

#[test]
fn the_rectangle_template_is_the_centred_polygon() {
    let tol = Tol::witness();
    let mut session = session(tol);
    let profile = insert(
        &mut session,
        SessionOp::AddProfile {
            plane: SketchPlane::xy(),
            loops: vec![ProfileShape::Rectangle {
                width: 0.04,
                height: 0.02,
            }],
        },
    );
    let want = Node::Profile(ProfileProgram {
        plane: SketchPlane::xy(),
        loops: vec![
            LoopProgram::polygon([(-0.02, -0.01), (0.02, -0.01), (0.02, 0.01), (-0.02, 0.01)])
                .expect("finite corners"),
        ],
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
    let states = session.history().len();

    // No loops: the profile layer's own refusal ("no loops — nothing
    // to sweep"), through the edit door.
    let empty = session.perform(SessionOp::AddProfile {
        plane: SketchPlane::xy(),
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
            plane: SketchPlane::xy(),
            loops: vec![ProfileShape::Circle {
                centre: [0.0, 0.0],
                radius,
            }],
        });
        assert!(
            matches!(degenerate.refusal, Some(Refusal::Edit(_))),
            "{:?}",
            degenerate.refusal
        );
    }

    // A non-finite field refuses at the literal door.
    let non_finite = session.perform(SessionOp::AddProfile {
        plane: SketchPlane::xy(),
        loops: vec![ProfileShape::Rectangle {
            width: f64::INFINITY,
            height: 0.01,
        }],
    });
    assert!(
        matches!(non_finite.refusal, Some(Refusal::Dimension(_))),
        "{:?}",
        non_finite.refusal
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
    let axis = insert(
        &mut session,
        SessionOp::AddDatum {
            datum: DatumSpec::Axis {
                origin: [0.0; 3],
                direction: [0.0, 1.0, 0.0],
            },
        },
    );
    let states = session.history().len();
    for op in [
        SessionOp::NewDocument {
            name: String::new(),
        },
        SessionOp::AddDatum {
            datum: DatumSpec::Point {
                position: [f64::NAN, 0.0, 0.0],
            },
        },
        SessionOp::AddProfile {
            plane: SketchPlane::xy(),
            loops: vec![],
        },
        SessionOp::AddExtrude {
            profile: axis,
            distance: 0.01,
        },
        SessionOp::AddRevolve {
            profile: axis,
            axis,
            angle: TAU,
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
    let profile = insert(
        &mut session,
        SessionOp::AddProfile {
            plane: SketchPlane::xy(),
            loops: vec![ProfileShape::Circle {
                centre: [R, 0.0],
                radius: RO,
            }],
        },
    );
    let axis = insert(
        &mut session,
        SessionOp::AddDatum {
            datum: DatumSpec::Axis {
                origin: [0.0; 3],
                direction: [0.0, 1.0, 0.0],
            },
        },
    );
    let extrude = insert(
        &mut session,
        SessionOp::AddExtrude {
            profile,
            distance: 0.02,
        },
    );

    // The extrude door: an extrude node is not a profile, and neither
    // is an id the document never held.
    for wrong in [extrude, RecipeNodeId(999)] {
        let refused = session.perform(SessionOp::AddExtrude {
            profile: wrong,
            distance: 0.02,
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
        angle: TAU,
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
    let plane = insert(
        &mut session,
        SessionOp::AddDatum {
            datum: DatumSpec::Plane {
                origin: [0.0; 3],
                normal: [0.0, 0.0, 1.0],
            },
        },
    );
    for wrong in [extrude, plane] {
        let refused = session.perform(SessionOp::AddRevolve {
            profile,
            axis: wrong,
            angle: TAU,
        });
        assert!(
            matches!(
                refused.refusal,
                Some(Refusal::WrongNodeKind { node, wanted: NodeKindWanted::Axis })
                    if node == wrong
            ),
            "{:?}",
            refused.refusal
        );
    }

    // The happy path inserts the revolve with both references.
    let revolve = insert(
        &mut session,
        SessionOp::AddRevolve {
            profile,
            axis,
            angle: TAU,
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
    let profile = insert(
        &mut session,
        SessionOp::AddProfile {
            plane: SketchPlane::xy(),
            loops: vec![ProfileShape::Circle {
                centre: [R, 0.0],
                radius: RO,
            }],
        },
    );
    let axis = insert(
        &mut session,
        SessionOp::AddDatum {
            datum: DatumSpec::Axis {
                origin: [0.0; 3],
                direction: [0.0, 1.0, 0.0],
            },
        },
    );

    let mut tool = RevolveTool::new();
    assert!(
        matches!(
            tool.op(TAU),
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
            tool.op(TAU),
            Err(SeatError::Empty {
                seat: Seat::RevolveAxis
            })
        ),
        "one pick, no op"
    );
    tool.pick(session.committed_doc(), axis);
    assert_eq!(tool.axis(), Some(axis));

    // The tool's op commits exactly one insert through the session.
    let op = tool.op(TAU).expect("both seats filled");
    let revolve = insert(&mut session, op);
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
    let axis = insert(
        &mut session,
        SessionOp::AddDatum {
            datum: DatumSpec::Axis {
                origin: [0.0; 3],
                direction: [0.0, 1.0, 0.0],
            },
        },
    );
    tool.pick(session.committed_doc(), axis);
    assert_eq!(tool.axis(), Some(axis), "the next pick refills the seat");
    assert!(tool.op(TAU).is_ok());
}

/// The seats are ROLES: a dropped profile leaves the axis IN the axis
/// seat (no promotion — deliberately divergent from the mate tool's
/// pair semantics; both module docs state it), and the next pick
/// refills the profile seat.
#[test]
fn a_dropped_profile_does_not_promote_the_axis() {
    let tol = Tol::witness();
    let mut session = session(tol);
    let profile = insert(
        &mut session,
        SessionOp::AddProfile {
            plane: SketchPlane::xy(),
            loops: vec![ProfileShape::Circle {
                centre: [R, 0.0],
                radius: RO,
            }],
        },
    );
    let axis = insert(
        &mut session,
        SessionOp::AddDatum {
            datum: DatumSpec::Axis {
                origin: [0.0; 3],
                direction: [0.0, 1.0, 0.0],
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
            tool.op(TAU),
            Err(SeatError::Empty {
                seat: Seat::RevolveProfile
            })
        ),
        "one seat empty, no op"
    );
    // The next pick refills the PROFILE seat, not the axis.
    let profile = insert(
        &mut session,
        SessionOp::AddProfile {
            plane: SketchPlane::xy(),
            loops: vec![ProfileShape::Circle {
                centre: [R, 0.0],
                radius: RI,
            }],
        },
    );
    tool.pick(session.committed_doc(), profile);
    assert_eq!(tool.profile(), Some(profile));
    assert_eq!(tool.axis(), Some(axis));
    assert!(tool.op(TAU).is_ok());

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
    let profile = insert(
        &mut session,
        SessionOp::AddProfile {
            plane: SketchPlane::xy(),
            loops: vec![ProfileShape::Circle {
                centre: [R, 0.0],
                radius: RO,
            }],
        },
    );
    let axis = insert(
        &mut session,
        SessionOp::AddDatum {
            datum: DatumSpec::Axis {
                origin: [0.0; 3],
                direction: [0.0, 1.0, 0.0],
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
