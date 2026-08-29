//! **What the picture marks because of what the panel is showing**
//! (`pick::focus`), as values rather than as pixels.
//!
//! The GPU's contribution to this feature is one `mix` against one
//! flag bit; everything that could be WRONG about it is upstream of
//! that and is a pure function — which ids a selection is responsible
//! for, and whether the scene carries the flag on exactly those
//! corners. Both are asserted here, headlessly, exactly as the
//! free-move probe's own distinctness marker is.

// Panicking is a test's failure mechanism (workspace lint note).
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

mod common;

use std::collections::BTreeSet;

use pncad::document::RecipeNodeId;
use pncad::geom_core::Tol;
use viewer::display::DisplayView;
use viewer::pick::{self, PickIndex};
use viewer::scene::{self, DisplayTolerance, SceneMesh};
use viewer::session::{DocSession, Selection, SessionOp};

fn delta() -> DisplayTolerance {
    DisplayTolerance::new(0.0005).expect("a positive δ")
}

/// A session over the spike plate, evaluated and landed.
fn plate_session(tol: Tol) -> (DocSession, RecipeNodeId) {
    let (doc, extrude) = scene::plate_with_hole(tol).expect("the plate authors");
    let mut session = DocSession::inline(doc, tol);
    session.pump();
    (session, extrude)
}

fn index_of(session: &DocSession) -> PickIndex {
    let (doc, eval) = session
        .landed_pair()
        .expect("the inline seam lands its first evaluation");
    let generation = session
        .landed_generation()
        .expect("a landed evaluation has a generation");
    PickIndex::build(doc, eval, generation, delta(), session.tol()).expect("the plate indexes")
}

/// Selecting nothing marks nothing — and the empty answer is the same
/// value the "no selection" case produces, not a separate path.
#[test]
fn an_empty_selection_marks_nothing() {
    let tol = Tol::witness();
    let (session, _extrude) = plate_session(tol);
    let index = index_of(&session);
    let focus = pick::focus(&index, session.doc(), &Selection::None);
    assert!(focus.is_empty());
    let scene = index
        .scene_focused(&DisplayView::none(), &focus)
        .expect("a scene builds");
    assert_eq!(scene.stats().focus_patches, 0);
    assert!(
        scene.flags().iter().all(|f| f & SceneMesh::FLAG_FOCUS == 0),
        "no corner is marked"
    );
}

/// **Selecting a feature marks every patch it drew** — not one, and
/// not the whole picture.
#[test]
fn selecting_a_feature_marks_every_patch_it_drew() {
    let tol = Tol::witness();
    let (mut session, extrude) = plate_session(tol);
    session.perform(SessionOp::Select(Selection::Node(extrude)));
    let index = index_of(&session);

    let focus = pick::focus(&index, session.doc(), session.selection());
    let drawn: BTreeSet<u32> = index.ids_of_node(extrude).into_iter().collect();
    assert!(!drawn.is_empty(), "the plate draws patches");
    assert_eq!(focus, drawn, "exactly the feature's own patches");

    let scene = index
        .scene_focused(&DisplayView::none(), &focus)
        .expect("a scene builds");
    assert_eq!(
        scene.stats().focus_patches,
        drawn.len(),
        "one marked patch per focused id"
    );
    // The flag lands on the corners of those ids and on no others —
    // the property the shader's `mix` rests on.
    for (flag, id) in scene.flags().iter().zip(scene.ids()) {
        assert_eq!(
            flag & SceneMesh::FLAG_FOCUS != 0,
            focus.contains(id),
            "corner of id {id} is marked {flag}"
        );
    }
}

/// A face pick marks the feature the face belongs to — the SAME set
/// selecting that feature in the tree marks.
///
/// That is the whole "highlight what the side panel is showing" claim:
/// the panel shows the owning node's slots for a face pick, so the
/// picture marks the owning node's extent, and which route the user
/// took to get there does not change what is drawn.
#[test]
fn a_face_pick_marks_its_owning_feature() {
    let tol = Tol::witness();
    let (mut session, extrude) = plate_session(tol);
    let index = index_of(&session);
    let id = index
        .ids_of_node(extrude)
        .first()
        .copied()
        .expect("the plate draws at least one patch");
    let patch = index.ids().key_of(id).expect("the id maps back");
    let name = index
        .name_of(id)
        .and_then(|n| n.as_ref().ok())
        .cloned()
        .expect("a drawn patch is named");
    session.perform(SessionOp::Select(Selection::Face(
        viewer::session::FaceSelection {
            node: patch.node,
            body: patch.body,
            name,
        },
    )));
    let by_face = pick::focus(&index, session.doc(), session.selection());
    session.perform(SessionOp::Select(Selection::Node(extrude)));
    let by_node = pick::focus(&index, session.doc(), session.selection());
    assert_eq!(by_face, by_node, "one feature, one extent");
    assert!(by_face.contains(&id));
}

/// **A feature that draws nothing marks what was built from it.**
///
/// A profile is not a drawn root — it has no body — so the empty answer
/// would be technically true and useless. What the user is looking at
/// when they select a profile is the shape of the walls above it, and
/// that is what lights up.
#[test]
fn a_profile_marks_the_body_built_from_it() {
    let tol = Tol::witness();
    let (doc, profile, extrude) = common::parametric_plate(tol);
    let mut session = DocSession::inline(doc, tol);
    session.pump();
    let index = index_of(&session);
    assert!(
        index.ids_of_node(profile).is_empty(),
        "a profile draws no patches of its own"
    );

    session.perform(SessionOp::Select(Selection::Node(profile)));
    let by_profile = pick::focus(&index, session.doc(), session.selection());
    session.perform(SessionOp::Select(Selection::Node(extrude)));
    let by_extrude = pick::focus(&index, session.doc(), session.selection());
    assert!(
        !by_profile.is_empty(),
        "the profile marks the extrude's body"
    );
    assert_eq!(by_profile, by_extrude);
}

/// **Selecting a document parameter marks what that number moves** —
/// every feature whose expressions read it, including through
/// arithmetic.
///
/// The fixture's extrude distance is `thickness / 2`, so the parameter
/// is reached through a division rather than named bare, which is the
/// case a shallow "is this expression the parameter" test would miss.
#[test]
fn selecting_a_parameter_marks_the_features_it_drives() {
    let tol = Tol::witness();
    let (doc, _profile, extrude) = common::parametric_plate(tol);
    let mut session = DocSession::inline(doc, tol);
    session.pump();
    let index = index_of(&session);

    session.perform(SessionOp::Select(Selection::Param(
        common::thickness_param(),
    )));
    let driven = pick::focus(&index, session.doc(), session.selection());
    let extrude_ids: BTreeSet<u32> = index.ids_of_node(extrude).into_iter().collect();
    assert!(!extrude_ids.is_empty());
    assert_eq!(driven, extrude_ids, "the parameter drives the extrude");

    // A parameter nothing reads marks nothing — the honest answer, not
    // "everything" and not a panic.
    let unused = pncad::document::ParamName::new("unused");
    let quiet = pick::focus(&index, session.doc(), &Selection::Param(unused));
    assert!(quiet.is_empty());
}

/// An id the scene does not draw is ignored rather than refused: a
/// hidden part's ids are legitimately absent from the picture, and a
/// focus is a request to mark what is there.
#[test]
fn a_focus_on_an_undrawn_id_marks_nothing_and_refuses_nothing() {
    let tol = Tol::witness();
    let (session, _extrude) = plate_session(tol);
    let index = index_of(&session);
    let absent: BTreeSet<u32> = BTreeSet::from([u32::MAX]);
    let scene = index
        .scene_focused(&DisplayView::none(), &absent)
        .expect("an unmatched focus is not an error");
    assert_eq!(scene.stats().focus_patches, 0);
}

/// The unfocused door and the focused one with an empty set are the
/// same picture — so `scene_for`'s existing callers cannot have been
/// changed by this feature.
#[test]
fn the_unfocused_scene_is_the_focused_one_with_nothing_focused() {
    let tol = Tol::witness();
    let (session, _extrude) = plate_session(tol);
    let index = index_of(&session);
    let plain = index.scene_for(&DisplayView::none()).expect("a scene");
    let empty = index
        .scene_focused(&DisplayView::none(), &BTreeSet::new())
        .expect("a scene");
    assert_eq!(plain.flags(), empty.flags());
    assert_eq!(plain.ids(), empty.ids());
    assert_eq!(plain.stats().focus_patches, 0);
}
