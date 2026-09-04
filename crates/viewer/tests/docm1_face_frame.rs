//! **DOCM-1 at the viewer** — acceptance row A7's chrome half: a
//! derived frame (`Datum::FaceFrame`) is a frame BY VALUE to every
//! viewer door that consumes one. The frame seat admits it, a profile
//! is drawn on it through `SessionOp::AddProfile`, the sketch drawer
//! reads its landed placement, the plane picker lists it, and the
//! feature tree names it apart from an authored frame.
//!
//! No chrome MINTS one here — that is CHROME's build
//! (`add-profile-placement-on-picked-face-frame`); the node arrives in
//! the document through the document door, as a library consumer
//! authors it.

// Panicking is a test's failure mechanism (workspace lint note).
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use crate::common;
use common::{insert, inserted, len};

use pncad::document::{Datum, Dimension, Doc, Expr, LoopProgram, Node, RecipeNodeId};
use pncad::geom_core::Tol;
use pncad::prelude::{CapEnd, EntityKind, RoleSeg, StableName};
use viewer::session::{DocSession, NodeKindWanted, SessionOp, admits};
use viewer::{sketch, tree};

/// A box and a frame derived from its top cap, authored through the
/// document door.
fn boxed_with_face_frame(
    tol: Tol,
) -> (
    Doc<pncad::document::ProfileProgram>,
    RecipeNodeId,
    RecipeNodeId,
) {
    let doc = Doc::empty_derived("docm1-viewer", tol);
    let (doc, plane) = inserted(&doc, common::xy_frame(), tol);
    let (doc, profile) = inserted(&doc, common::square(plane, 0.02), tol);
    let (doc, cube) = inserted(
        &doc,
        Node::Extrude {
            profile,
            distance: len(0.01),
        },
        tol,
    );
    let (doc, frame) = inserted(
        &doc,
        Node::Datum(Datum::FaceFrame {
            at: cube,
            face: StableName {
                kind: EntityKind::Face,
                node: cube,
                path: vec![RoleSeg::Cap(CapEnd::Top)],
            },
            spin: Expr::literal(0.0, Dimension::Angle).expect("an angle"),
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
            plane: frame,
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
