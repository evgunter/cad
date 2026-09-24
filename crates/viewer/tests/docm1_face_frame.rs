//! **DOCM-1 at the viewer** — acceptance row A7's chrome half: a
//! derived frame (`Datum::FaceFrame`) is a frame BY VALUE to every
//! viewer door that consumes one. The frame seat admits it, a profile
//! is drawn on it through `SessionOp::AddProfile`, the sketch drawer
//! reads its landed placement, the plane picker lists it, and the
//! feature tree names it apart from an authored frame.
//!
//! **And the chrome MINTS one**: the add-datum form's `frame on face`
//! seat carries the two picks a face frame wants — the node whose body
//! the ray met and the frozen face name — plus the spin, and
//! `SessionOp::AddDatum` lowers it to the same node a library consumer
//! authors through the document door. The rows here drive both routes
//! and compare the documents.
//!
//! The gate above that seat (`session::face_frame_seat`) is the
//! affordance, never the safety: it answers the same questions the
//! node answers at evaluation — is the carrier planar, does the name
//! still resolve, is `at` one body — one step earlier, so the author
//! learns before the node lands.

// Panicking is a test's failure mechanism (workspace lint note).
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use crate::common;
use common::{insert, inserted, len};

use pncad::document::{Datum, Doc, Expr, LoopProgram, Node, ProfileProgram, RecipeNodeId};
use pncad::geom_core::Tol;
use pncad::prelude::{CapEnd, EntityKind, RoleSeg, StableName, SurfaceKind, attribute};
use pncad::select::{InterrogateError, all_faces, face_carrier_kind};
use viewer::session::{
    DatumSpec, DocSession, FaceFrameFault, FaceSelection, NodeKindWanted, ProfilePlane, Refusal,
    SessionOp, admits, face_frame_seat,
};
use viewer::{sketch, tree};

/// A 20 mm square box, 10 mm tall, through the document door — the
/// body every row here picks a face on.
fn boxed(tol: Tol) -> (Doc<ProfileProgram>, RecipeNodeId) {
    let doc = Doc::empty_derived("docm1-viewer", tol);
    let (doc, plane) = inserted(&doc, common::xy_frame(), tol);
    let (doc, profile) = inserted(&doc, common::square(plane, 0.02), tol);
    inserted(
        &doc,
        Node::Extrude {
            profile,
            distance: len(0.01),
        },
        tol,
    )
}

/// The box's top cap, named as the extrude emits it.
fn cap_of(cube: RecipeNodeId) -> StableName {
    StableName {
        kind: EntityKind::Face,
        node: cube,
        path: vec![RoleSeg::Cap(CapEnd::End)],
    }
}

/// A box and a frame derived from its top cap, authored through the
/// document door.
fn boxed_with_face_frame(tol: Tol) -> (Doc<ProfileProgram>, RecipeNodeId, RecipeNodeId) {
    let (doc, cube) = boxed(tol);
    let (doc, frame) = inserted(
        &doc,
        Node::Datum(Datum::FaceFrame {
            at: cube,
            face: cap_of(cube),
            spin: common::ang(0.0),
        }),
        tol,
    );
    (doc, cube, frame)
}

/// **The frame seat accepts a derived frame, a profile draws on it,
/// and the drawer, the picker and the tree all see it as a frame.**
#[test]
fn a7_the_viewer_takes_a_derived_frame_by_value() {
    let tol = Tol::witness();
    let (doc, _cube, frame) = boxed_with_face_frame(tol);
    let node = doc.node(frame).expect("the frame is live");
    assert!(
        admits(Some(node), NodeKindWanted::Frame),
        "the frame seat admits it"
    );
    assert!(
        !admits(Some(node), NodeKindWanted::Plane),
        "and only the frame seat"
    );
    assert_eq!(tree::node_kind(node), "Datum frame (on face)");
    assert!(
        sketch::frames(&doc).contains(&frame),
        "the plane picker lists it"
    );

    let mut session = DocSession::inline(doc, tol);
    let boss = insert(
        &mut session,
        SessionOp::AddProfile {
            plane: ProfilePlane::Existing(frame),
            loops: vec![
                LoopProgram::polygon([
                    (-0.005, -0.005),
                    (0.005, -0.005),
                    (0.005, 0.005),
                    (-0.005, 0.005),
                ])
                .expect("finite corners"),
            ],
        },
    );
    session.pump();
    let ev = session.evaluation().expect("the document evaluated");
    assert!(
        ev.value(boss).is_some(),
        "the profile on the derived frame evaluated"
    );
    // The drawer reads the LANDED placement: sketch (0, 0) is the cap
    // plane's origin at the box's height.
    let placed = sketch::frame_placement(session.doc(), ev, frame).expect("a drawable frame");
    assert!((placed.placement.translation.z - 0.01).abs() <= 1e-12);
}

/// **The chrome mints the node the document door mints.** One
/// `SessionOp::AddDatum` carrying the add-datum form's sixth seat, on
/// the document a library consumer would hand-author on, and the two
/// documents compare bit for bit (`Doc::bit_eq`, spec D7's replay
/// identity — every float by bits).
///
/// The spin is deliberately NOT zero: a lowering that dropped the
/// slot, or swapped `at` and the face's own node, passes at zero and
/// fails here.
#[test]
fn the_chrome_mints_what_the_document_door_mints() {
    let tol = Tol::witness();
    let (doc, cube) = boxed(tol);
    let spin = common::ang(0.3);
    let (hand, _) = inserted(
        &doc,
        Node::Datum(Datum::FaceFrame {
            at: cube,
            face: cap_of(cube),
            spin: spin.clone(),
        }),
        tol,
    );

    let mut session = DocSession::inline(doc, tol);
    let minted = insert(
        &mut session,
        SessionOp::AddDatum {
            datum: DatumSpec::FaceFrame {
                at: cube,
                face: cap_of(cube),
                spin,
            },
        },
    );
    assert!(
        session.committed_doc().bit_eq(&hand),
        "the op's document is the document door's, bit for bit"
    );
    assert!(
        admits(session.committed_doc().node(minted), NodeKindWanted::Frame),
        "and what it minted is a frame to the seats"
    );
}

/// A box with every edge filleted, so the box's flats survive as
/// SHRUNK faces of a later feature's body — the shape the `at`
/// question is actually about.
fn filleted(tol: Tol) -> (DocSession, RecipeNodeId, RecipeNodeId) {
    let (doc, cube) = boxed(tol);
    let mut session = DocSession::inline(doc, tol);
    session.pump();
    let edges = pncad::select::all_edges(session.evaluation().expect("the box evaluated"), cube);
    assert!(!edges.is_empty(), "the box has edges to blend");
    let (doc, fillet) = inserted(
        session.committed_doc(),
        Node::fillet(cube, len(0.001), edges),
        tol,
    );
    let mut session = DocSession::inline(doc, tol);
    session.pump();
    (session, cube, fillet)
}

/// **One face of `at`'s body passing `want`**, with its name — the
/// rows below each want a face of a particular carrier, and picking it
/// out of the evaluation is how they avoid hand-writing a name the
/// fillet's emission owns.
fn face_where(
    session: &DocSession,
    at: RecipeNodeId,
    want: impl Fn(SurfaceKind, &StableName) -> bool,
) -> StableName {
    let ev = session.evaluation().expect("the document evaluated");
    all_faces(ev, at)
        .into_iter()
        .find(|name| face_carrier_kind(ev, at, name).is_ok_and(|kind| want(kind, name)))
        .expect("a face of the wanted kind")
}

/// **C2: `at` is the node whose body the ray met, not the feature that
/// minted the face.** A flat swept by the extrude and shrunk by the
/// fillet is picked on the FILLET's body; the seat answers the fillet,
/// while the same pick's `feature()` answers the extrude — so the two
/// are demonstrably different values here and the seat takes the right
/// one.
#[test]
fn at_is_the_node_the_ray_met_and_not_the_feature() {
    let tol = Tol::witness();
    let (session, cube, fillet) = filleted(tol);
    let flat = face_where(&session, fillet, |kind, name| {
        kind == SurfaceKind::Plane && attribute(name).minted_by() == Some(cube)
    });
    let picked = FaceSelection {
        name: flat.clone(),
        node: fillet,
        body: 0,
    };
    assert_eq!(
        picked.feature(),
        cube,
        "the flat is still the extrude's face"
    );
    assert_ne!(cube, fillet, "and the two questions have different answers");
    let (at, name) =
        face_frame_seat(session.landed_pair(), Some(&picked)).expect("a planar face admits");
    assert_eq!(at, fillet, "the seat takes the body the ray met");
    assert_eq!(name, flat);

    // And the node that seat authors evaluates: a frame read through
    // the extrude would name a face of a different size, and one read
    // through a node the name does not live in would not resolve at
    // all.
    let mut session = session;
    let frame = insert(
        &mut session,
        SessionOp::AddDatum {
            datum: DatumSpec::FaceFrame {
                at,
                face: name,
                spin: common::ang(0.0),
            },
        },
    );
    session.pump();
    let ev = session.evaluation().expect("the document evaluated");
    assert!(
        ev.value(frame).is_some(),
        "the frame on the shrunk flat evaluated"
    );
}

/// **C3: a curved carrier is declined, and the refusal names the kind
/// it found.** The fillet's own rounded face is a cylinder; the gate
/// says so as a VALUE, and the sentence it renders carries the
/// kernel's word for that kind.
#[test]
fn the_gate_declines_a_curved_carrier_and_names_it() {
    let tol = Tol::witness();
    let (session, _cube, fillet) = filleted(tol);
    let round = face_where(&session, fillet, |kind, _| kind == SurfaceKind::Cylinder);
    let picked = FaceSelection {
        name: round,
        node: fillet,
        body: 0,
    };
    let fault = face_frame_seat(session.landed_pair(), Some(&picked))
        .expect_err("a cylinder carries no sketch frame");
    assert_eq!(
        fault,
        FaceFrameFault::NotPlanar {
            carrier: SurfaceKind::Cylinder
        }
    );
    let said = fault.to_string();
    assert!(
        said.contains(SurfaceKind::Cylinder.name()),
        "the sentence names the carrier it found: {said}"
    );
}

/// **C4: a name that no longer resolves is declined, carrying the
/// interrogation door's refusal** rather than a sentence composed
/// here.
#[test]
fn the_gate_carries_the_interrogation_refusal() {
    let tol = Tol::witness();
    let (doc, cube) = boxed(tol);
    let mut session = DocSession::inline(doc, tol);
    session.pump();
    let picked = FaceSelection {
        // A well-formed face name of this node that its table does not
        // hold — a selection an upstream edit took away.
        name: StableName {
            kind: EntityKind::Face,
            node: cube,
            path: vec![RoleSeg::Cap(CapEnd::Start), RoleSeg::Cap(CapEnd::End)],
        },
        node: cube,
        body: 0,
    };
    assert_eq!(
        face_frame_seat(session.landed_pair(), Some(&picked)),
        Err(FaceFrameFault::Unresolved {
            error: InterrogateError::NoSuchName
        })
    );
}

/// **The two states that are not about the face at all**: nothing
/// picked, and nothing landed. Each is its own value, because the
/// recourse differs — pick something, or wait.
#[test]
fn the_gate_separates_no_pick_from_no_evaluation() {
    let tol = Tol::witness();
    let (doc, cube) = boxed(tol);
    let mut session = DocSession::inline(doc, tol);
    assert_eq!(
        face_frame_seat(session.landed_pair(), None),
        Err(FaceFrameFault::NoFace),
        "no pick is answered before the evaluation is even looked for"
    );
    let picked = FaceSelection {
        name: cap_of(cube),
        node: cube,
        body: 0,
    };
    assert_eq!(
        face_frame_seat(None, Some(&picked)),
        Err(FaceFrameFault::NotLanded)
    );
    session.pump();
    assert!(
        face_frame_seat(session.landed_pair(), Some(&picked)).is_ok(),
        "and the same pick admits once the document has answered"
    );
}

/// **A face picked on a node whose value is SEVERAL bodies is declined
/// — by the form and by the op door alike.** A split's sides are two
/// bodies, and the frame node reads its face through the evaluator's
/// single-body operand door, so minting one there would be minting a
/// node that refuses after the edit lands.
#[test]
fn several_bodies_is_no_seat_for_a_face_frame() {
    let tol = Tol::witness();
    let (doc, cube) = boxed(tol);
    let (doc, knife) = inserted(
        &doc,
        Node::Datum(Datum::Plane {
            origin: common::len3([0.0, 0.0, 0.005]),
            normal: common::scl3([0.0, 0.0, 1.0]),
        }),
        tol,
    );
    let (doc, split) = inserted(
        &doc,
        Node::Split {
            target: cube,
            tool: knife,
        },
        tol,
    );
    let mut session = DocSession::inline(doc, tol);
    session.pump();
    let face = face_where(&session, split, |kind, _| kind == SurfaceKind::Plane);
    let picked = FaceSelection {
        name: face.clone(),
        node: split,
        body: 0,
    };
    assert_eq!(
        face_frame_seat(session.landed_pair(), Some(&picked)),
        Err(FaceFrameFault::NotOneBody { at: split }),
        "the form declines it"
    );
    let refused = session.perform(SessionOp::AddDatum {
        datum: DatumSpec::FaceFrame {
            at: split,
            face,
            spin: common::ang(0.0),
        },
    });
    assert!(
        matches!(
            refused.refusal,
            Some(Refusal::WrongNodeKind {
                node,
                wanted: NodeKindWanted::Body
            }) if node == split
        ),
        "and so does the op door, if the form is bypassed: {:?}",
        refused.refusal
    );
    assert!(refused.committed.is_empty(), "nothing was inserted");
}

/// **A placer over a pattern is a body-denoting NODE whose VALUE is
/// several bodies, and the seat reads the value.** `Node::Transform`
/// is shape-preserving over its input, so a transform of a pattern
/// evaluates to `Instances` while every node-kind predicate over it
/// answers "a body". The frame node's `at` goes through the
/// evaluator's single-body operand door, which reads the VALUE, so a
/// seat that asked the node kind would admit this pick and mint a node
/// that refuses on arrival.
#[test]
fn a_transform_of_a_pattern_is_no_seat_for_a_face_frame() {
    let tol = Tol::witness();
    let (doc, cube) = boxed(tol);
    let (doc, pattern) = inserted(
        &doc,
        Node::Pattern {
            input: cube,
            count: Expr::count(2),
            kind: pncad::document::PatternKind::Linear {
                direction: common::scl3([1.0, 0.0, 0.0]),
                spacing: len(0.05),
            },
        },
        tol,
    );
    let (doc, placed) = inserted(
        &doc,
        Node::Transform {
            input: pattern,
            translation: common::len3([0.0, 0.0, 0.001]),
            rotation_axis: common::scl3([0.0, 0.0, 1.0]),
            rotation_angle: common::ang(0.0),
        },
        tol,
    );
    let mut session = DocSession::inline(doc, tol);
    session.pump();
    let face = face_where(&session, placed, |kind, _| kind == SurfaceKind::Plane);
    let picked = FaceSelection {
        name: face,
        node: placed,
        body: 0,
    };
    assert_eq!(
        face_frame_seat(session.landed_pair(), Some(&picked)),
        Err(FaceFrameFault::NotOneBody { at: placed }),
        "the transform's value is several bodies"
    );
}

/// **A pick whose feature an undo took away is refused for being
/// GONE, not for being several bodies.** The latch outlives the node:
/// a face is picked, the extrude that carried it is undone, and the
/// arm is still showing. "Project the one you mean first" would be
/// advice about a feature that no longer exists, so the refusal is the
/// interrogation door's own word for a node this document has no
/// result for.
#[test]
fn a_pick_whose_node_an_undo_took_away_is_refused_as_gone() {
    let tol = Tol::witness();
    // Built through the session's own ops, so the undo has a state to
    // step back to — the gesture this row is about is an author's, not
    // a document-door edit's.
    let mut session = DocSession::inline(Doc::empty_derived("docm1-viewer", tol), tol);
    let plane = common::xy_frame_in(&mut session);
    let profile = insert(
        &mut session,
        SessionOp::AddProfile {
            plane: ProfilePlane::Existing(plane),
            loops: vec![common::shape(&viewer::session::ProfileShape::Rectangle {
                width: 0.02,
                height: 0.02,
            })],
        },
    );
    let cube = insert(
        &mut session,
        SessionOp::AddExtrude {
            profile,
            distance: len(0.01),
        },
    );
    session.pump();
    let picked = FaceSelection {
        name: cap_of(cube),
        node: cube,
        body: 0,
    };
    assert!(
        face_frame_seat(session.landed_pair(), Some(&picked)).is_ok(),
        "the cap seats while its feature is there"
    );

    assert!(
        session.perform(SessionOp::Undo).refusal.is_none(),
        "the extrude is undone"
    );
    session.pump();
    assert!(
        session.committed_doc().node(cube).is_none(),
        "the document no longer holds the node the pick names"
    );
    assert_eq!(
        face_frame_seat(session.landed_pair(), Some(&picked)),
        Err(FaceFrameFault::Unresolved {
            error: InterrogateError::NodeNotEvaluated { node: cube },
        }),
        "the face is gone, and that is what it is told"
    );
}
