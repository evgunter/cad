//! **"Sketch on this face" at the viewer** (spec §5): a profile drawn
//! on its own plane, extruded away from a picked face, its copy placed
//! against the face's plane and fused with the face's body — one
//! committed action through `SessionOp::SketchOnFace`, which is the
//! kernel's own gesture (`Recording::sketch_on_face`).
//!
//! The face is a PICK, and what it is read through is the node whose
//! body the ray met — never the feature that minted the face: a flat
//! swept by an extrude and shrunk by a later chamfer is drawn on the
//! CHAMFER's body, and reading it through the extrude would name a face
//! of a different size.

// Panicking is a test's failure mechanism (workspace lint note).
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use crate::common;
use common::{inserted, len};

use pncad::document::{Doc, DocEdit, Node, ProfileProgram, RecipeNodeId};
use pncad::geom_core::Tol;
use pncad::prelude::{CapEnd, EntityKind, RoleSeg, StableName, SurfaceKind, attribute};
use pncad::select::{all_faces, face_carrier_kind};
use viewer::session::{DocSession, FaceSelection, NodeKindWanted, Refusal, SessionOp};

/// The box's side: 20 mm square, 10 mm tall.
const SIDE: f64 = 0.02;
/// The box's height.
const HEIGHT: f64 = 0.01;
/// The boss sketched on the box's top: a 5 mm square, 4 mm tall.
const BOSS: [f64; 3] = [0.005, 0.005, 0.004];

/// A 20 mm square box, 10 mm tall, through the document door and
/// placed in the world — the body every row here picks a face on.
fn boxed(tol: Tol) -> (Doc<ProfileProgram>, RecipeNodeId) {
    let doc = Doc::empty_derived("sketch-on-face", tol);
    let (doc, profile) = inserted(&doc, common::square(SIDE), tol);
    let (doc, cube) = inserted(
        &doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(HEIGHT),
        },
        tol,
    );
    let (doc, _) = common::placed(&doc, cube, tol);
    (doc, cube)
}

/// The box's top cap, picked on the box itself.
fn top_of(cube: RecipeNodeId) -> FaceSelection {
    FaceSelection {
        name: StableName {
            kind: EntityKind::Face,
            node: cube,
            path: vec![RoleSeg::Cap(CapEnd::End)],
        },
        node: cube,
        body: 0,
    }
}

/// The boss op on `face`.
fn boss_on(face: FaceSelection) -> SessionOp {
    SessionOp::SketchOnFace {
        face,
        loops: vec![common::rectangle_loop([0.0, 0.0], BOSS[0], BOSS[1])],
        distance: len(BOSS[2]),
        spin: common::ang(0.0),
    }
}

/// **One action, one undo, and the body it makes is the box with the
/// boss on it.** The gesture writes the profile, the extrude, the
/// placement on the face and the union in one recorded action; the
/// box's world placement is re-pointed to the union, so the world is
/// that one body; its volume is the two blocks' sum (the boss stands
/// on the face, flush, meeting it in a face and no volume); and one
/// undo takes the whole action back.
#[test]
fn a_sketch_on_a_face_is_one_action_and_one_body() {
    let tol = Tol::witness();
    let (doc, cube) = boxed(tol);
    let mut session = DocSession::inline(doc, tol);
    session.pump();
    let before = session.committed_doc().clone();
    let steps = session.history().len();
    let out = session.perform(boss_on(top_of(cube)));
    assert!(out.refusal.is_none(), "{:?}", out.refusal);
    assert_eq!(session.history().len(), steps + 1, "one action");
    let inserted: Vec<&str> = out
        .committed
        .iter()
        .filter_map(|edit| match edit {
            DocEdit::InsertNode { node, .. } => Some(pncad::document::node_kind_noun(node)),
            _ => None,
        })
        .collect();
    assert_eq!(
        inserted,
        ["Profile", "Extrude", "Place", "Union"],
        "the profile, the construction, its placement and the combine"
    );
    let doc = session.committed_doc();
    let union = doc
        .ids()
        .into_iter()
        .find(|&id| matches!(doc.node(id), Some(Node::Union { .. })))
        .expect("the union landed");
    assert_eq!(
        common::world(doc),
        [union],
        "the box's placement now places the union"
    );
    let got = common::body_volume(&mut session, union, tol);
    let want = SIDE * SIDE * HEIGHT + BOSS[0] * BOSS[1] * BOSS[2];
    assert!(common::near(got, want), "box plus boss: {got} vs {want}");

    assert!(session.perform(SessionOp::Undo).refusal.is_none());
    assert!(
        session.committed_doc().bit_eq(&before),
        "one undo takes the whole action back"
    );
}

/// **The face is read through the body the ray met, not through the
/// feature that minted it.** A flat swept by the extrude and shrunk by
/// the chamfer is picked on the CHAMFER's body; the pick's `feature()`
/// answers the extrude, so the two are demonstrably different values
/// here — and the union the gesture writes reads the chamfer, and
/// evaluates.
#[test]
fn the_face_is_read_through_the_body_the_ray_met_and_not_the_feature() {
    let tol = Tol::witness();
    let doc = Doc::empty_derived("sketch-on-face-chamfered", tol);
    let (doc, profile) = inserted(&doc, common::square(SIDE), tol);
    let (doc, cube) = inserted(
        &doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(HEIGHT),
        },
        tol,
    );
    let mut session = DocSession::inline(doc, tol);
    session.pump();
    let edges = pncad::select::all_edges(session.evaluation().expect("the box evaluated"), cube);
    assert!(!edges.is_empty(), "the box has edges to bevel");
    let (doc, chamfer) = inserted(
        session.committed_doc(),
        Node::chamfer(cube, len(0.001), edges),
        tol,
    );
    let (doc, _) = common::placed(&doc, chamfer, tol);
    let mut session = DocSession::inline(doc, tol);
    session.pump();
    let flat = {
        let ev = session.evaluation().expect("the chamfer evaluated");
        all_faces(ev, chamfer)
            .into_iter()
            .find(|name| {
                face_carrier_kind(ev, chamfer, name).is_ok_and(|kind| kind == SurfaceKind::Plane)
                    && attribute(name).minted_by() == Some(cube)
                    && name
                        .path
                        .iter()
                        .any(|seg| matches!(seg, RoleSeg::From { .. }))
            })
            .expect("a flat of the box, shrunk by the chamfer")
    };
    let picked = FaceSelection {
        name: flat,
        node: chamfer,
        body: 0,
    };
    assert_eq!(
        picked.feature(),
        cube,
        "the flat is still the extrude's face"
    );
    assert_ne!(
        cube, chamfer,
        "and the two questions have different answers"
    );
    let out = session.perform(boss_on(picked));
    assert!(out.refusal.is_none(), "{:?}", out.refusal);
    let union = {
        let doc = session.committed_doc();
        doc.ids()
            .into_iter()
            .find(|&id| matches!(doc.node(id), Some(Node::Union { .. })))
            .expect("the union landed")
    };
    assert!(
        session.committed_doc().upstream(union).contains(&chamfer),
        "the union reads the body the ray met"
    );
    session.pump();
    let ev = session.evaluation().expect("the document evaluated");
    assert!(
        ev.value(union).is_some(),
        "the sketch on the shrunk flat evaluated: {:?}",
        ev.result(union)
    );
}

/// **A face whose node is no body refuses at the door, typed** — and
/// records nothing.
#[test]
fn a_face_on_no_body_is_refused_at_the_door() {
    let tol = Tol::witness();
    let (doc, cube) = boxed(tol);
    let (doc, profile) = inserted(&doc, common::square(SIDE), tol);
    let mut session = DocSession::inline(doc, tol);
    session.pump();
    let steps = session.history().len();
    let out = session.perform(boss_on(FaceSelection {
        node: profile,
        ..top_of(cube)
    }));
    assert!(
        matches!(
            &out.refusal,
            Some(Refusal::WrongNodeKind { node, wanted: NodeKindWanted::Body }) if node.id() == profile
        ),
        "{:?}",
        out.refusal
    );
    assert_eq!(session.history().len(), steps, "nothing recorded");
}
