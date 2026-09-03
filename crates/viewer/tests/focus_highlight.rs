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

use crate::common;

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

// ---- the die: a feature's extent is what it MADE ------------------

/// The tour's composed die, as `editor-core`'s corpus committed it:
/// the document `demos/tour/src/diefillet.rs` authors — 21 pips cut
/// from a cube in one grouped tool, its twelve box edges blended, then
/// its 21 pip rims banded — replayed from the edit log that corpus
/// registers.
///
/// Read from the corpus asset rather than copied beside this suite. A
/// second copy of a 400 kB document is a second thing to regenerate
/// when the scene moves, and the corpus carries the CI gate that keeps
/// the bytes current with the tour.
const DIE_DOCUMENT: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../editor-core/tests/corpus/tour/die_composed_tour.pncad"
);

/// The die document, ε re-stamped for this run.
///
/// A saved snapshot records the ε it was written under and `load`
/// refuses one that is not the process's ("one process, one ε"), while
/// the CI matrix sweeps ε — so the ONE ε line is replaced with what
/// this run's serializer writes, exactly as `doc_io` does for the
/// gallery ring. The recipe below it is ε-free: the die's two fillet
/// selections travel as stored names.
fn die_document(tol: Tol) -> pncad::document::Doc<pncad::document::ProfileProgram> {
    let recourse = "regenerate it: cd demos/tour && cargo run --release -- die-corpus \
                    ../../crates/editor-core/tests/corpus/tour/die_composed_tour.pncad";
    let text = std::fs::read_to_string(DIE_DOCUMENT)
        .unwrap_or_else(|e| panic!("the corpus die document is unreadable: {e} — {recourse}"));
    let probe: pncad::document::Doc<pncad::document::ProfileProgram> =
        pncad::document::Doc::empty_derived("gui-focus-epsilon-probe", tol);
    let probe_text = pncad::document::save(&probe, &[], tol).expect("an empty document saves");
    let is_epsilon = |line: &str| line.trim_start().starts_with("\"epsilon\":");
    let wanted = probe_text
        .lines()
        .find(|line| is_epsilon(line))
        .expect("a saved document records its ε");
    assert_eq!(
        text.lines().filter(|l| is_epsilon(l)).count(),
        1,
        "the die document must carry exactly one ε line"
    );
    let restamped: String = text
        .lines()
        .map(|line| if is_epsilon(line) { wanted } else { line })
        .collect::<Vec<&str>>()
        .join("\n");
    pncad::document::load(&restamped, tol)
        .unwrap_or_else(|e| panic!("the die document refuses: {e:?} — {recourse}"))
        .doc
}

/// The die's nodes, found by STRUCTURE — the recipe's shape, never a
/// transcribed id, so the fixture follows the scene instead of pinning
/// numbers that would go quietly wrong.
struct Die {
    session: DocSession,
    /// The rim-band fillet: the last node, and the only drawn root.
    composed: RecipeNodeId,
    /// The box-edge fillet it stands on.
    box_blend: RecipeNodeId,
    /// The subtract that cut the pips.
    cut: RecipeNodeId,
    /// The cube's extrude.
    cube: RecipeNodeId,
    /// The master ball's revolve — every pip cavity's origin.
    ball: RecipeNodeId,
    /// One pip placement: a `Transform`, which mints nothing and
    /// contributes no role segment to any name.
    pip: RecipeNodeId,
}

fn die(tol: Tol) -> Die {
    use pncad::document::Node;
    type DieDoc = pncad::document::Doc<pncad::document::ProfileProgram>;
    let doc = die_document(tol);
    let input_of = |doc: &DieDoc, id| {
        doc.node(id)
            .expect("an ordered node exists")
            .inputs()
            .first()
            .copied()
            .expect("the die's chain is unbroken")
    };
    let first = |doc: &DieDoc, want: fn(&Node<pncad::document::ProfileProgram>) -> bool| {
        doc.order()
            .iter()
            .copied()
            .find(|&id| want(doc.node(id).expect("an ordered node exists")))
            .expect("the die has this node kind")
    };
    let composed = *doc.order().last().expect("the die has nodes");
    let box_blend = input_of(&doc, composed);
    let cut = input_of(&doc, box_blend);
    let cube = input_of(&doc, cut);
    assert!(
        matches!(doc.node(composed), Some(Node::Fillet { .. }))
            && matches!(doc.node(box_blend), Some(Node::Fillet { .. }))
            && matches!(doc.node(cut), Some(Node::Boolean { .. }))
            && matches!(doc.node(cube), Some(Node::Extrude { .. })),
        "the die is rim-blend over box-blend over cut over extrude"
    );
    let ball = first(&doc, |n| matches!(n, Node::Revolve { .. }));
    let pip = first(&doc, |n| matches!(n, Node::Transform { .. }));
    let mut session = DocSession::inline(doc, tol);
    session.pump();
    Die {
        session,
        composed,
        box_blend,
        cut,
        cube,
        ball,
        pip,
    }
}

/// The die's faces, by what made them — the geometry, counted, not a
/// number read off a run.
///
/// - the cube's extrude makes SIX planar faces (two caps, four walls);
///   cutting a dimple into one punches a hole in it and mints no face;
/// - the box-edge fillet makes TWENTY: a quarter-cylinder blend for
///   each of the cube's twelve edges, a sphere octant at each of its
///   eight corners;
/// - the master ball's revolve makes the pip cavities. Each cavity is
///   TWO half-cap faces meeting at the two meridians the tour's
///   `(Sphere, Sphere)` note names, which is why the rim fillet's
///   stored selection is forty-two arcs for twenty-one pips;
/// - the rim fillet makes ONE torus band per pip rim — a rim is a
///   closed chain, and a closed chain is one band.
const DIE_FLATS: usize = 6;
const DIE_BLENDS: usize = 12 + 8;
const DIE_CAVITIES: usize = 21 * 2;
const DIE_BANDS: usize = 21;
const DIE_FACES: usize = DIE_FLATS + DIE_BLENDS + DIE_CAVITIES + DIE_BANDS;

/// **The reported bug, as a value.** Every face of the die is drawn
/// under the outer fillet, so "the patches drawn under the selected
/// node" made that fillet answer the whole die — pips, flats and all.
/// What it made is the torus bands.
///
/// Each of the four makers answers its own faces, the four are
/// disjoint, and together they are the body: every drawn face has
/// exactly one maker.
#[test]
fn each_die_face_is_marked_by_the_feature_that_made_it() {
    let tol = Tol::witness();
    let die = die(tol);
    let index = index_of(&die.session);
    let drawn: BTreeSet<u32> = index.ids_of_node(die.composed).into_iter().collect();
    assert_eq!(drawn.len(), DIE_FACES, "the die's drawn faces");

    let of = |node| pick::focus(&index, die.session.doc(), &Selection::Node(node));
    let bands = of(die.composed);
    let blends = of(die.box_blend);
    let flats = of(die.cube);
    let cavities = of(die.ball);

    assert_eq!(bands.len(), DIE_BANDS, "the rim fillet made the bands");
    assert!(
        bands.len() < drawn.len(),
        "and strictly fewer patches than the body it filleted"
    );
    assert!(bands.is_subset(&drawn));
    assert_eq!(blends.len(), DIE_BLENDS, "the box-edge fillet's own faces");
    assert_eq!(flats.len(), DIE_FLATS, "the cube's own faces");
    assert_eq!(cavities.len(), DIE_CAVITIES, "the ball's own faces");

    let parts = [&bands, &blends, &flats, &cavities];
    for (i, left) in parts.iter().enumerate() {
        for right in parts.iter().skip(i + 1) {
            assert!(
                left.intersection(right).next().is_none(),
                "a face has one maker, not two"
            );
        }
    }
    let union: BTreeSet<u32> = parts.iter().flat_map(|s| s.iter().copied()).collect();
    assert_eq!(union, drawn, "four makers account for the whole die");
}

/// **A node that made nothing drawn marks what passed through it**, by
/// the name where the op re-named what it carried and by the recipe
/// where it did not.
///
/// The cut mints no face of its own — a subtract carries A's faces and
/// B's — so it marks exactly those: the cube's six and the cavities'
/// forty-two. The blends are NOT among them: they were made above the
/// cut, out of it, and a name records what an op carried, not what was
/// later built on top.
///
/// A pip placement is a `Transform`, which contributes no role segment
/// at all, so no name mentions it and the recipe answers instead: what
/// it carries is what was minted below it. That is the master ball's
/// cavities — every pip's, not this pip's, because all twenty-one
/// placements copy ONE ball and a copy belongs to the master.
#[test]
fn a_node_that_made_nothing_marks_what_passed_through_it() {
    let tol = Tol::witness();
    let die = die(tol);
    let index = index_of(&die.session);
    let of = |node| pick::focus(&index, die.session.doc(), &Selection::Node(node));
    let drawn: BTreeSet<u32> = index.ids_of_node(die.composed).into_iter().collect();

    let carried_by_the_cut: BTreeSet<u32> = of(die.cube).union(&of(die.ball)).copied().collect();
    assert_eq!(of(die.cut), carried_by_the_cut);
    assert_eq!(of(die.cut).len(), DIE_FLATS + DIE_CAVITIES);
    assert!(
        of(die.cut).len() < drawn.len(),
        "not the whole die: the blends are above the cut, not through it"
    );

    assert_eq!(of(die.pip), of(die.ball), "a copy belongs to the master");
    assert_eq!(of(die.pip).len(), DIE_CAVITIES);
}

/// **The click path, which is the one Evan reported.** Every face of
/// the die is drawn under the outer fillet, so a pick that inverted to
/// "whose body did the ray meet" answered that fillet for a flat, a
/// blend and a band alike. It inverts to the feature that MADE the
/// face instead, and the tree highlight, the property rows and the
/// picture all read that one answer.
#[test]
fn clicking_a_die_face_reaches_the_feature_that_made_it() {
    let tol = Tol::witness();
    let mut die = die(tol);
    let index = index_of(&die.session);
    let of = |node| pick::focus(&index, die.session.doc(), &Selection::Node(node));
    let flats = of(die.cube);
    let bands = of(die.composed);

    let selection_for = |id: u32| {
        let patch = index.ids().key_of(id).expect("the id maps back");
        assert_eq!(
            patch.node, die.composed,
            "every patch of the die is DRAWN under the outer fillet"
        );
        Selection::Face(viewer::session::FaceSelection {
            node: patch.node,
            body: patch.body,
            name: index
                .name_of(id)
                .and_then(|n| n.as_ref().ok())
                .cloned()
                .expect("a drawn patch is named"),
        })
    };

    // A flat: made by the cube's extrude, carried through the cut and
    // both fillets.
    let flat = selection_for(*flats.first().expect("the die has flats"));
    die.session.perform(SessionOp::Select(flat));
    assert_eq!(
        die.session.selection().node(),
        Some(die.cube),
        "the flat's feature is the extrude that swept it"
    );
    let lit = pick::focus(&index, die.session.doc(), die.session.selection());
    assert_eq!(lit, flats, "and the picture marks that feature's faces");
    assert!(
        lit.intersection(&bands).next().is_none(),
        "clicking a flat lights no band"
    );

    // A band: made by the rim fillet, which also happens to draw it.
    let band = selection_for(*bands.first().expect("the die has bands"));
    die.session.perform(SessionOp::Select(band));
    assert_eq!(die.session.selection().node(), Some(die.composed));
    assert_eq!(
        pick::focus(&index, die.session.doc(), die.session.selection()),
        bands,
        "the fillet's extent is the blends it made"
    );
}
