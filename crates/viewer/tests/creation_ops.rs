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
//! revolve — is asserted against an inline twin of the ring demo's
//! document (`demos/tour/src/ring.rs::document`, restated here because
//! the tour deliberately lives outside the workspace). The comparator
//! is **`Doc::bit_eq`** — spec D7's replay-identity comparator, the
//! strongest equality the document layer supports: every float
//! compares by bits, identity/order/roots structurally. `PartialEq`
//! would conflate `±0.0`; nothing weaker would be a claim about the
//! same document.

// Panicking is a test's failure mechanism (workspace lint note).
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

mod common;

use core::f64::consts::TAU;

use pncad::document::{
    Datum, Dimension, Doc, DocEdit, DocumentId, Expr, LoopProgram, Node, ProfileProgram,
    RecipeNodeId, SlotId, apply,
};
use pncad::geom_core::Tol;
use pncad::prelude::ValuePayload;
use pncad::profile::SketchPlane;
use viewer::revolvetool::{RevolveTool, RevolveToolError};
use viewer::session::{DatumSpec, DocSession, ProfileShape, Refusal, Selection, SessionOp};

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

/// Perform one op that must commit exactly one edit, answering the id
/// of the node it inserted (the newest node in the document).
fn insert(session: &mut DocSession, op: SessionOp) -> RecipeNodeId {
    let outcome = session.perform(op);
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    assert_eq!(outcome.committed.len(), 1, "exactly one committed edit");
    assert!(matches!(
        outcome.committed.first(),
        Some(DocEdit::InsertNode { .. })
    ));
    *session
        .committed_doc()
        .order()
        .last()
        .expect("the insert landed")
}

/// The ring demo's document, restated node for node from
/// `demos/tour/src/ring.rs::document` (the tour is outside the
/// workspace, so the twin lives here; if the demo's recipe ever
/// changes, this is the copy to re-sync).
fn ring_twin(tol: Tol) -> Doc<ProfileProgram> {
    let len = |v: f64| Expr::literal(v, Dimension::Length).expect("a length");
    let mut doc: Doc<ProfileProgram> = Doc::empty_derived("hollow-ring", tol);
    let insert = |doc: &mut Doc<ProfileProgram>, node| -> RecipeNodeId {
        let applied = apply(doc, &DocEdit::InsertNode { node }, tol).expect("the edit applies");
        *doc = applied.doc;
        applied.record.minted.expect("insert mints an id")
    };
    let circle = |r: f64| LoopProgram::Circle {
        centre: [len(R), len(0.0)],
        radius: len(r),
    };
    let profile = insert(
        &mut doc,
        Node::Profile(ProfileProgram {
            plane: SketchPlane::xy(),
            loops: vec![circle(RO), circle(RI)],
        }),
    );
    let axis = insert(
        &mut doc,
        Node::Datum(Datum::Axis {
            origin: [len(0.0), len(0.0), len(0.0)],
            direction: [
                Expr::literal(0.0, Dimension::Scalar).expect("a scalar"),
                Expr::literal(1.0, Dimension::Scalar).expect("a scalar"),
                Expr::literal(0.0, Dimension::Scalar).expect("a scalar"),
            ],
        }),
    );
    insert(
        &mut doc,
        Node::Revolve {
            profile,
            axis,
            angle: Expr::literal(TAU, Dimension::Angle).expect("an angle"),
        },
    );
    doc
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
fn the_ring_stream_reproduces_the_demo_document_bit_for_bit() {
    let tol = Tol::witness();
    let (mut session, revolve) = authored_ring(tol);

    let twin = ring_twin(tol);
    assert!(
        session.committed_doc().bit_eq(&twin),
        "the ops-authored ring and the demo's recipe are one document \
         under D7's replay-identity comparator"
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

#[test]
fn new_document_derives_its_id_and_clears_the_session() {
    let tol = Tol::witness();
    let mut session = session(tol);
    // Give the session things to clear: a selection and a backing
    // path.
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
    let dir = common::tempdir("gauth1-new");
    let saved = session.perform(SessionOp::Save(dir.join("old.pncad")));
    assert!(saved.refusal.is_none(), "{:?}", saved.refusal);
    assert!(session.path().is_some());

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
    assert!(session.path().is_none(), "no backing file until saved");
    assert!(
        !session.history().can_undo(),
        "the old document's history is gone, not underneath"
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

    // Mid-gesture, every creation door refuses.
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

#[test]
fn profile_refusals_are_typed_at_the_door() {
    let tol = Tol::witness();
    let mut session = session(tol);
    let states = session.history().len();

    // No loops: nothing to insert, said here rather than downstream.
    let empty = session.perform(SessionOp::AddProfile {
        plane: SketchPlane::xy(),
        loops: vec![],
    });
    assert!(
        matches!(empty.refusal, Some(Refusal::EmptyProfile)),
        "{:?}",
        empty.refusal
    );

    // A degenerate loop refuses through the edit door's own
    // authoring-time check — the same refusal a hand-written program
    // gets, not a rule restated in the session.
    let degenerate = session.perform(SessionOp::AddProfile {
        plane: SketchPlane::xy(),
        loops: vec![ProfileShape::Circle {
            centre: [0.0, 0.0],
            radius: 0.0,
        }],
    });
    assert!(
        matches!(degenerate.refusal, Some(Refusal::Edit(_))),
        "{:?}",
        degenerate.refusal
    );

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
            matches!(refused.refusal, Some(Refusal::NotAProfile { node }) if node == wrong),
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
        matches!(refused.refusal, Some(Refusal::NotAProfile { node }) if node == axis),
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
            matches!(refused.refusal, Some(Refusal::NotAnAxis { node }) if node == wrong),
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
        matches!(tool.op(TAU), Err(RevolveToolError::NotTwoPicks)),
        "no picks, no op"
    );
    tool.pick(profile);
    assert_eq!(tool.profile(), Some(profile));
    assert!(
        matches!(tool.op(TAU), Err(RevolveToolError::NotTwoPicks)),
        "one pick, no op"
    );
    tool.pick(axis);
    assert_eq!(tool.axis(), Some(axis));

    // The tool's op commits exactly one insert through the session.
    let op = tool.op(TAU).expect("both seats filled");
    let revolve = insert(&mut session, op);
    assert!(matches!(
        session.committed_doc().node(revolve),
        Some(Node::Revolve { .. })
    ));

    // Survival: delete the axis (which takes the revolve with it) and
    // reconcile — the axis seat empties with a typed event, the
    // profile stays held, and the next pick refills the empty seat.
    let mut tool = RevolveTool::new();
    tool.pick(profile);
    tool.pick(axis);
    let deleted = session.perform(SessionOp::DeleteNode { node: axis });
    assert!(deleted.refusal.is_none(), "{:?}", deleted.refusal);
    let events = tool.reconcile(session.committed_doc());
    assert_eq!(events.len(), 1, "one drop, reported");
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
    tool.pick(axis);
    assert_eq!(tool.axis(), Some(axis), "the next pick refills the seat");
    assert!(tool.op(TAU).is_ok());
}
