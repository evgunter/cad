//! **Cursor in, edge out** — the edge-pick unit's headless half.
//!
//! Everything about picking an edge except the pixels is a value: the
//! projection is a camera operation, the drawn polylines are the
//! tessellation the picture is built from, the names come out of the
//! shipped inversion, and the marking is a pure function. So the rows
//! below drive synthetic cursors through the same functions the
//! application drives and assert on what came out.
//!
//! **The cursors are DERIVED, never typed in.** Every row projects a
//! drawn edge through the camera and places its cursor relative to the
//! result, so a change to the plate, the camera or the display δ moves
//! the cursors with it rather than leaving a row that measures a pixel
//! nothing is at any more.
//!
//! What is NOT here, and cannot be: the line pass that paints the
//! marks. What that pass draws is checked as the value it consumes —
//! `marks::edge_overlay` — and the pixels are issue #1097's hardware
//! checklist.

// Panicking is a test's failure mechanism (workspace lint note).
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use crate::common;
use crate::common::plate_index;

use pncad::document::{Evaluation, Frame, RecipeNodeId};
use pncad::geom_core::{Point3, Tol};
use pncad::select::{HitTestError, Resolution, RunCtx};
use viewer::camera::Camera;
use viewer::display::DisplayView;
use viewer::input::{PickAction, ViewportSize};
use viewer::marks;
use viewer::pickindex::{EDGE_PICK_RADIUS_PX, EdgeId, PickIndex, PickKinds};
use viewer::scene::{self, PLATE_EXTENT, PLATE_HOLE_RADIUS};
use viewer::session::{DocSession, EdgeSelection, Hovered, Selection, SessionOp};

/// The pane every row picks in.
fn pane() -> ViewportSize {
    ViewportSize {
        width_px: 1280.0,
        height_px: 720.0,
    }
}

/// A session over the spike plate, evaluated and landed, and the
/// plate's world placement: the node its drawn copy is picked on.
fn plate_session(tol: Tol) -> (DocSession, RecipeNodeId) {
    let (doc, _) = scene::plate_with_hole(tol).expect("the plate authors");
    let [copy] = doc.placements()[..] else {
        panic!("the plate is placed once")
    };
    let mut session = DocSession::inline(doc, tol);
    session.pump();
    (session, copy)
}

/// The landed evaluation, for the doors that take one.
fn eval_of(session: &DocSession) -> &Evaluation<f64> {
    session.evaluation().expect("an evaluation has landed")
}

/// Every drawn edge of the plate, with the polyline it is drawn as.
/// How far along `ray` the nearest drawn surface sits, or `None` for
/// a miss.
///
/// The door answers one face, nothing, or a certified TIE between
/// several — and a tie is still a surface: the tied answers pairwise
/// overlap, so the smallest of their parameters is what "is anything
/// in front" reads.
fn nearest_surface(
    index: &PickIndex,
    eval: &Evaluation<f64>,
    ray: &pncad::select::Ray,
) -> Option<f64> {
    match index.pick_for(eval, ray, &DisplayView::none()) {
        Ok(hit) => hit.map(|hit| hit.t),
        Err(HitTestError::Ambiguous { hits }) => {
            hits.iter().map(|hit| hit.t).min_by(f64::total_cmp)
        }
        Err(other) => panic!("no refusal: {other}"),
    }
}

fn drawn_edges(index: &PickIndex, node: RecipeNodeId) -> Vec<(EdgeId, Vec<Point3<f64>>)> {
    index
        .edges_in(node, 0)
        .iter()
        .map(|&id| (id, index.edge_polyline_for(id, &DisplayView::none())))
        .collect()
}

/// The plate's top face is at `z = thickness`; its hole's rim is the
/// drawn edge that lies entirely there at the hole's own radius from
/// the plate's axis.
///
/// Chosen as the subject of the aiming rows because it is the one
/// drawn edge that is unambiguously INSIDE the silhouette from any
/// view that sees the top face: a cursor on it has the plate behind it
/// on both sides, so what the rows measure is the priority rule and
/// never an accident of where the body ends.
fn hole_rim(index: &PickIndex, node: RecipeNodeId) -> (EdgeId, Vec<Point3<f64>>) {
    let [width, depth, thickness] = PLATE_EXTENT;
    let (cx, cy) = (width * 0.5, depth * 0.5);
    // The rim is two half-circle edges. The rows' cursors are laid out
    // against its +y half, so that half is chosen by GEOMETRY — the
    // edge whose points lie above the hole's centre — rather than by
    // the order the edges were minted in.
    drawn_edges(index, node)
        .into_iter()
        .filter(|(_, points)| points.iter().all(|p| p.y >= cy - 1.0e-9))
        .find(|(_, points)| {
            !points.is_empty()
                && points.iter().all(|p| {
                    (p.z - thickness).abs() < 1.0e-9
                        && ((p.x - cx).powi(2) + (p.y - cy).powi(2)).sqrt()
                            < PLATE_HOLE_RADIUS * 1.05
                })
        })
        .expect("the plate's top face carries the hole's rim")
}

/// Where a world point lands in the pane, in physical pixels.
fn pixel_of(camera: &Camera, point: Point3<f64>) -> [f64; 2] {
    let aspect = pane().aspect().expect("a positive aspect");
    let ndc = camera
        .project(point, aspect)
        .expect("the projection is defined")
        .expect("a framed point is in front of the eye");
    pane().cursor_of([ndc[0], ndc[1]]).expect("a positive area")
}

/// A point of the polyline and the two pixels its own segment spans —
/// the midpoint of the segment nearest the middle of the run, so a
/// row's cursor is never at a chord point two segments share.
fn middle_segment(camera: &Camera, points: &[Point3<f64>]) -> ([f64; 2], [f64; 2]) {
    assert!(points.len() >= 2, "a polyline is at least one segment");
    let at = (points.len() - 1) / 2;
    (
        pixel_of(camera, points[at]),
        pixel_of(camera, points[at + 1]),
    )
}

/// The cursor `offset_px` from the midpoint of `a`–`b`, perpendicular
/// to it and on the side `toward` lies.
fn offset_from(a: [f64; 2], b: [f64; 2], toward: [f64; 2], offset_px: f64) -> [f64; 2] {
    let mid = [(a[0] + b[0]) * 0.5, (a[1] + b[1]) * 0.5];
    let (dx, dy) = (b[0] - a[0], b[1] - a[1]);
    let length = (dx.powi(2) + dy.powi(2)).sqrt();
    assert!(
        length > 0.0,
        "a segment that projects to a point has no normal"
    );
    let mut normal = [-dy / length, dx / length];
    let to = [toward[0] - mid[0], toward[1] - mid[1]];
    if normal[0] * to[0] + normal[1] * to[1] < 0.0 {
        normal = [-normal[0], -normal[1]];
    }
    [
        mid[0] + normal[0] * offset_px,
        mid[1] + normal[1] * offset_px,
    ]
}

/// The pane pixel the plate's centre of mass projects to — "into the
/// body", for the rows that offset a cursor off an edge and want to
/// stay over the solid.
fn plate_centre_px(camera: &Camera) -> [f64; 2] {
    let [width, depth, thickness] = PLATE_EXTENT;
    pixel_of(camera, Point3::new(width * 0.5, depth * 0.25, thickness))
}

// --- the drawn edges and their names --------------------------------

#[test]
fn every_drawn_edge_names_an_edge_that_resolves() {
    let tol = Tol::witness();
    let (session, copy) = plate_session(tol);
    let index = plate_index(&session);
    let (doc, eval) = session.landed_pair().expect("a landed pair");
    let edges = drawn_edges(&index, copy);
    assert!(
        edges.len() >= 12,
        "a plate with a hole draws at least a box's worth of edges, got {}",
        edges.len()
    );
    let mut names = std::collections::BTreeSet::new();
    for (id, points) in &edges {
        assert!(
            points.len() >= 2,
            "every drawn edge is at least one segment: {id:?}"
        );
        let name = index.edge_name_of(*id).expect("a drawn edge has a name");
        assert!(
            matches!(
                crate::fixture::resolve_at_mint(RunCtx { doc, eval }, name),
                Resolution::Resolved(_)
            ),
            "a just-drawn edge's name resolves in the run it was drawn from"
        );
        assert!(names.insert(name.clone()), "one name per drawn edge");
    }
}

#[test]
fn an_edge_selection_narrows_to_the_copy_it_was_picked_from() {
    let tol = Tol::witness();
    let (session, copy) = plate_session(tol);
    let index = plate_index(&session);
    let (id, _) = hole_rim(&index, copy);
    let name = index
        .edge_name_of(id)
        .expect("a drawn edge has a name")
        .clone();
    let picked = EdgeSelection {
        name: name.clone(),
        node: copy,
        body: 0,
    };
    assert_eq!(index.edges_of_target(&picked), vec![id]);
    // A body this index does not draw carries no copy of the name,
    // which is how a stale or foreign selection lights nothing.
    let elsewhere = EdgeSelection {
        name,
        node: copy,
        body: 7,
    };
    assert!(index.edges_of_target(&elsewhere).is_empty());
}

// --- aiming ---------------------------------------------------------

#[test]
fn a_cursor_on_a_drawn_edge_picks_that_edge() {
    let tol = Tol::witness();
    let (session, copy) = plate_session(tol);
    let index = plate_index(&session);
    let aspect = pane().aspect().expect("a positive aspect");
    let camera = common::framed(aspect);
    let (id, points) = hole_rim(&index, copy);
    let (a, b) = middle_segment(&camera, &points);
    let cursor = [(a[0] + b[0]) * 0.5, (a[1] + b[1]) * 0.5];

    let pick = index
        .edge_at(eval_of(&session), &camera, pane(), cursor)
        .expect("the cursor un-projects")
        .expect("a cursor on a drawn edge picks it");
    assert_eq!(pick.id(), id, "the edge under the cursor is the one picked");
    assert_eq!(pick.node, copy);
    assert!(
        pick.distance_px <= EDGE_PICK_RADIUS_PX,
        "a pick is inside the radius by construction, got {}",
        pick.distance_px
    );
    assert!(
        pick.distance_px < 1.0e-6,
        "a cursor ON the projected segment is zero pixels from it, got {}",
        pick.distance_px
    );
}

#[test]
fn hover_and_click_answer_one_cursor_the_same_way() {
    let tol = Tol::witness();
    let (mut session, copy) = plate_session(tol);
    let index = plate_index(&session);
    let aspect = pane().aspect().expect("a positive aspect");
    let camera = common::framed(aspect);
    let (id, points) = hole_rim(&index, copy);
    let (a, b) = middle_segment(&camera, &points);
    let cursor = [(a[0] + b[0]) * 0.5, (a[1] + b[1]) * 0.5];

    let hover = index
        .op_for(
            eval_of(&session),
            &camera,
            pane(),
            PickAction::Hover(cursor),
        )
        .expect("the cursor un-projects");
    let select = index
        .op_for(
            eval_of(&session),
            &camera,
            pane(),
            PickAction::Select(cursor),
        )
        .expect("the cursor un-projects");
    let SessionOp::Hover(Some(Hovered::Edge(hovered))) = &hover else {
        panic!("a cursor on an edge hovers the edge, got {hover:?}");
    };
    let SessionOp::Select(Selection::Edge(selected)) = &select else {
        panic!("a cursor on an edge selects the edge, got {select:?}");
    };
    assert_eq!(hovered, selected, "one cursor, one answer");
    assert_eq!(index.edges_of_target(selected), vec![id]);

    session.perform(hover);
    session.perform(select);
    assert!(
        session.standing().live(),
        "a just-picked edge denotes something: {:?}",
        session.standing()
    );
    assert_eq!(
        session.selection().node(),
        Some(viewer::world::seat_of(session.committed_doc(), copy)),
        "an edge selection reaches the feature that made it, as a face does"
    );
    assert!(
        !session.slot_rows().is_empty(),
        "the panel offers the feature's rows for an edge pick too"
    );
}

/// **The radius boundary, from both sides.**
///
/// The cursor is walked off the hole's rim, perpendicular to it and
/// into the solid, so the only thing that changes across the two rows
/// is how many pixels away it is. Inside the radius the edge wins;
/// outside it the face does — and the face is the same face either
/// way, which is what says the second row is the priority rule
/// declining rather than the pick failing.
#[test]
fn the_edge_beats_the_face_exactly_inside_the_radius() {
    let tol = Tol::witness();
    let (session, copy) = plate_session(tol);
    let index = plate_index(&session);
    let aspect = pane().aspect().expect("a positive aspect");
    let camera = common::framed(aspect);
    let (id, points) = hole_rim(&index, copy);
    let (a, b) = middle_segment(&camera, &points);
    let into = plate_centre_px(&camera);

    let inside = offset_from(a, b, into, EDGE_PICK_RADIUS_PX - 0.5);
    let outside = offset_from(a, b, into, EDGE_PICK_RADIUS_PX + 0.5);

    let near = index
        .edge_at(eval_of(&session), &camera, pane(), inside)
        .expect("un-projects")
        .expect("half a pixel inside the radius, the edge wins");
    assert_eq!(near.id(), id);
    assert!(
        (near.distance_px - (EDGE_PICK_RADIUS_PX - 0.5)).abs() < 1.0e-6,
        "the reported distance is the offset that was aimed, got {}",
        near.distance_px
    );
    assert!(
        index
            .edge_at(eval_of(&session), &camera, pane(), outside)
            .expect("un-projects")
            .is_none(),
        "half a pixel outside the radius, no edge is picked"
    );

    // And the op that consumes the rule agrees at both cursors.
    for (cursor, wants_edge) in [(inside, true), (outside, false)] {
        let op = index
            .op_for(
                eval_of(&session),
                &camera,
                pane(),
                PickAction::Select(cursor),
            )
            .expect("un-projects");
        match op {
            SessionOp::Select(Selection::Edge(_)) => {
                assert!(wants_edge, "an edge won outside the radius");
            }
            SessionOp::Select(Selection::Face(face)) => {
                assert!(!wants_edge, "the face won inside the radius");
                assert_eq!(face.node, copy, "the cursor is still over the plate");
            }
            other => panic!("a cursor over the plate selects something: {other:?}"),
        }
    }
}

#[test]
fn a_cursor_over_the_background_picks_nothing_at_all() {
    let tol = Tol::witness();
    let (session, _) = plate_session(tol);
    let index = plate_index(&session);
    let aspect = pane().aspect().expect("a positive aspect");
    let camera = common::framed(aspect);
    // A corner of the pane: the framing fits the plate inside the
    // frustum, so the corners are background.
    let cursor = [2.0, 2.0];
    assert!(
        index
            .edge_at(eval_of(&session), &camera, pane(), cursor)
            .expect("un-projects")
            .is_none(),
        "an edge is reachable only where its own body is"
    );
    let op = index
        .op_for(
            eval_of(&session),
            &camera,
            pane(),
            PickAction::Select(cursor),
        )
        .expect("un-projects");
    assert!(
        matches!(op, SessionOp::Select(Selection::None)),
        "a click on empty space clears, got {op:?}"
    );
}

/// **An edge the solid hides never wins.**
///
/// The occluded cursor is FOUND rather than typed: every drawn edge's
/// middle segment is projected and re-picked, and the row runs at the
/// first one whose own pixel shows a nearer surface. A view that
/// stopped hiding any edge would fail here loudly instead of leaving a
/// green row that measures nothing.
#[test]
fn an_edge_behind_the_solid_does_not_win_at_its_own_pixel() {
    let tol = Tol::witness();
    let (session, copy) = plate_session(tol);
    let index = plate_index(&session);
    let aspect = pane().aspect().expect("a positive aspect");
    let camera = common::framed(aspect);
    let eval = eval_of(&session);

    let mut checked = 0usize;
    for (id, points) in drawn_edges(&index, copy) {
        if points.len() < 2 {
            continue;
        }
        let at = (points.len() - 1) / 2;
        let (a, b) = (points[at], points[at + 1]);
        let cursor = pixel_of(
            &camera,
            Point3::new((a.x + b.x) * 0.5, (a.y + b.y) * 0.5, (a.z + b.z) * 0.5),
        );
        let ray = camera.ray_through(cursor, pane()).expect("un-projects");
        // **A certified tie is still a surface in front.** The ray
        // through an edge's own pixel lands ON that edge, so it very
        // often meets both faces sharing it and the door names both
        // rather than choosing. The question this row asks is how near
        // the nearest surface is, and every tied answer answers it —
        // so the nearest of them is read.
        let Some(front) = nearest_surface(&index, eval, &ray) else {
            continue;
        };
        // How far along the ray the edge's own midpoint sits.
        let midpoint = Point3::new((a.x + b.x) * 0.5, (a.y + b.y) * 0.5, (a.z + b.z) * 0.5);
        let depth = ray.dir.dot(midpoint - ray.origin);
        // The 1e-6 band is this row's OWN reading of "the same depth",
        // an oracle independent of the implementation, which orders
        // certified spans and has no band: it says the two populations
        // this row sorts are far enough apart that any sane threshold
        // separates them.
        if front >= depth * (1.0 - 1.0e-6) {
            continue; // visible at its own pixel — not this row's subject
        }
        checked += 1;
        let picked = index
            .edge_at(eval, &camera, pane(), cursor)
            .expect("un-projects");
        assert_ne!(
            picked.as_ref().map(viewer::pickindex::EdgePick::id),
            Some(id),
            "an edge {depth} deep behind a surface at {front} was picked through the solid"
        );
    }
    assert!(
        checked > 0,
        "no drawn edge of this view is hidden, so this row measured nothing"
    );
}

#[test]
fn one_cursor_answers_the_same_edge_twice() {
    let tol = Tol::witness();
    let (session, copy) = plate_session(tol);
    let index = plate_index(&session);
    let aspect = pane().aspect().expect("a positive aspect");
    let camera = common::framed(aspect);
    let (_, points) = hole_rim(&index, copy);
    let (a, b) = middle_segment(&camera, &points);
    let cursor = offset_from(a, b, plate_centre_px(&camera), 1.5);
    let once = index
        .edge_at(eval_of(&session), &camera, pane(), cursor)
        .expect("un-projects")
        .expect("an edge");
    let twice = index
        .edge_at(eval_of(&session), &camera, pane(), cursor)
        .expect("un-projects")
        .expect("an edge");
    assert_eq!(once.id(), twice.id());
    assert_eq!(
        once.distance_px.to_bits(),
        twice.distance_px.to_bits(),
        "the same cursor against the same state answers bit-identically"
    );
}

// --- the marks ------------------------------------------------------

#[test]
fn the_overlay_marks_the_selected_and_hovered_edges_and_nothing_else() {
    let tol = Tol::witness();
    let (mut session, copy) = plate_session(tol);
    let index = plate_index(&session);
    let (rim, points) = hole_rim(&index, copy);
    let selection = EdgeSelection {
        name: index
            .edge_name_of(rim)
            .expect("a drawn edge has a name")
            .clone(),
        node: copy,
        body: 0,
    };
    let other = drawn_edges(&index, copy)
        .into_iter()
        .find(|(id, run)| *id != rim && run.len() >= 2)
        .expect("the plate draws more than one edge");
    let hovered = EdgeSelection {
        name: index
            .edge_name_of(other.0)
            .expect("a drawn edge has a name")
            .clone(),
        node: copy,
        body: 0,
    };

    let empty = marks::edge_overlay(&index, &DisplayView::none(), &Selection::None, None);
    assert!(empty.is_empty(), "nothing selected marks nothing");

    session.perform(SessionOp::Select(Selection::Edge(selection.clone())));
    let marked = marks::edge_overlay(
        &index,
        &DisplayView::none(),
        session.selection(),
        session.hover(),
    );
    assert_eq!(
        marked.selected.len(),
        (points.len() - 1) * 2,
        "the mark is the drawn polyline, as line-list pairs"
    );
    assert!(marked.hovered.is_empty(), "nothing is hovered");

    session.perform(SessionOp::Hover(Some(Hovered::Edge(hovered))));
    let both = marks::edge_overlay(
        &index,
        &DisplayView::none(),
        session.selection(),
        session.hover(),
    );
    assert_eq!(both.selected.len(), (points.len() - 1) * 2);
    assert_eq!(both.hovered.len(), (other.1.len() - 1) * 2);

    // Hovering what is already selected marks it once: selection is
    // the state the user committed to.
    session.perform(SessionOp::Hover(Some(Hovered::Edge(selection))));
    let once = marks::edge_overlay(
        &index,
        &DisplayView::none(),
        session.selection(),
        session.hover(),
    );
    assert!(once.hovered.is_empty(), "the selected edge is marked once");
    assert_eq!(once.segments(), points.len() - 1);
}

/// **A preview is drawn, and it is not a selection.**
///
/// The overlay carries a third list for segments that are not in the
/// document at all — the wireframe of something a form is composing.
/// It is never derived from a pick (so a selection puts nothing in
/// it), and it counts toward what the overlay has to draw: an overlay
/// carrying only a preview is NOT empty, which is what stops the
/// renderer's "nothing to upload" arm from swallowing it.
#[test]
fn a_preview_is_carried_beside_the_marks_and_derived_from_no_pick() {
    let tol = Tol::witness();
    let (session, copy) = plate_session(tol);
    let index = plate_index(&session);
    let (rim, _) = hole_rim(&index, copy);
    let selection = EdgeSelection {
        name: index
            .edge_name_of(rim)
            .expect("a drawn edge has a name")
            .clone(),
        node: copy,
        body: 0,
    };
    let marked = marks::edge_overlay(
        &index,
        &DisplayView::none(),
        &Selection::Edge(selection),
        None,
    );
    assert!(
        marked.preview.is_empty(),
        "a selection is about the document; a preview is not",
    );

    let mut composing = marks::EdgeOverlay::default();
    assert!(composing.is_empty());
    composing.preview = vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]];
    assert!(!composing.is_empty(), "a preview alone is still a drawing");
    assert_eq!(composing.segments(), 1);
}

#[test]
fn a_face_selection_marks_no_edge_and_an_edge_selection_marks_no_patch() {
    let tol = Tol::witness();
    let (session, copy) = plate_session(tol);
    let index = plate_index(&session);
    let (rim, _) = hole_rim(&index, copy);
    let edge = Selection::Edge(EdgeSelection {
        name: index
            .edge_name_of(rim)
            .expect("a drawn edge has a name")
            .clone(),
        node: copy,
        body: 0,
    });
    let lit = marks::highlight(&index, &edge, None);
    assert_eq!(
        lit.selected,
        viewer::pickindex::IdMap::NOTHING,
        "an edge selection tints no patch"
    );
    let face = index
        .face_at(eval_of(&session), &common::down_at(0.005, 0.005))
        .expect("no refusal")
        .expect("a ray onto the plate hits it");
    let overlay = marks::edge_overlay(&index, &DisplayView::none(), &Selection::Face(face), None);
    assert!(overlay.is_empty(), "a face selection marks no edge");
}

// --- the two halves' opposite conventions ----------------------------

/// **One gesture, two answers to "is this hovered?", and the
/// difference is deliberate.** Hovering what is already selected
/// leaves `Highlight::hovered` carrying the selected patch's id —
/// `gpu`'s `fs_main` tests the selected lane first, so the face path
/// rules downstream where the fragment can see both — while
/// `EdgeOverlay::hovered` comes back EMPTY, because the overlay is a
/// value a test reads and an edge vertex carries exactly one lane, so
/// the selection is settled in the value rather than left to the pass's
/// draw order (`marks::edge_overlay` says why).
///
/// Each half is already pinned on its own, a file apart:
/// `select_pick.rs`'s
/// `the_highlight_is_a_function_of_the_scene_and_the_selection` and
/// `the_overlay_marks_the_selected_and_hovered_edges_and_nothing_else`
/// above. Neither says the other exists, so a lane that reds one of
/// them has no way to learn that the opposite convention next door is
/// the intended one. This row is the pair, asserted together: swapping
/// the two conventions reds it whichever way the swap goes.
///
/// The two negative assertions each carry their own control — a hover
/// on a DIFFERENT edge must still fill `hovered`, and the selected
/// patch's id must not be `NOTHING` — so neither can pass by the mark
/// having stopped working altogether.
#[test]
fn a_hover_on_the_selection_is_kept_by_the_face_mark_and_dropped_by_the_edge_mark() {
    let tol = Tol::witness();
    let (mut session, copy) = plate_session(tol);
    let index = plate_index(&session);

    // The face half: both lanes carry the picked patch.
    let face = index
        .face_at(eval_of(&session), &common::down_at(0.005, 0.005))
        .expect("no refusal")
        .expect("a ray onto the plate hits it");
    session.perform(SessionOp::Select(Selection::Face(face.clone())));
    session.perform(SessionOp::Hover(Some(Hovered::Face(face))));
    let lit = marks::highlight(&index, session.selection(), session.hover());
    assert_ne!(
        lit.selected,
        viewer::pickindex::IdMap::NOTHING,
        "the control: the picked patch is marked at all"
    );
    assert_eq!(
        lit.hovered, lit.selected,
        "the face mark keeps a hover that is the selection; the shader rules"
    );

    // The edge half: the hovered lane is dropped.
    let (rim, _) = hole_rim(&index, copy);
    let selection = EdgeSelection {
        name: index
            .edge_name_of(rim)
            .expect("a drawn edge has a name")
            .clone(),
        node: copy,
        body: 0,
    };
    let other = drawn_edges(&index, copy)
        .into_iter()
        .find(|(id, run)| *id != rim && run.len() >= 2)
        .expect("the plate draws more than one edge");
    session.perform(SessionOp::Select(Selection::Edge(selection.clone())));

    session.perform(SessionOp::Hover(Some(Hovered::Edge(EdgeSelection {
        name: index
            .edge_name_of(other.0)
            .expect("a drawn edge has a name")
            .clone(),
        node: copy,
        body: 0,
    }))));
    let elsewhere = marks::edge_overlay(
        &index,
        &DisplayView::none(),
        session.selection(),
        session.hover(),
    );
    assert!(
        !elsewhere.hovered.is_empty(),
        "the control: a hover off the selection does fill the hovered lane"
    );

    session.perform(SessionOp::Hover(Some(Hovered::Edge(selection))));
    let on_it = marks::edge_overlay(
        &index,
        &DisplayView::none(),
        session.selection(),
        session.hover(),
    );
    assert!(
        !on_it.selected.is_empty(),
        "the control: the selected edge is still marked"
    );
    assert!(
        on_it.hovered.is_empty(),
        "the edge mark drops a hover that is the selection; nothing downstream could"
    );
    assert!(
        !on_it.hovered_probed,
        "the dropped lane carries no probe flag either"
    );
}

// --- survival -------------------------------------------------------

/// The ratified resolution-failure semantics, on the edge arm: the
/// name stays, the verdict changes, the affordances switch off, and
/// the picture marks nothing.
#[test]
fn deleting_the_feature_leaves_the_edge_selection_unresolved() {
    let tol = Tol::witness();
    let (mut session, copy) = plate_session(tol);
    let index = plate_index(&session);
    let (rim, _) = hole_rim(&index, copy);
    let selection = EdgeSelection {
        name: index
            .edge_name_of(rim)
            .expect("a drawn edge has a name")
            .clone(),
        node: copy,
        body: 0,
    };
    session.perform(SessionOp::Select(Selection::Edge(selection.clone())));
    assert!(session.standing().live());

    let extrude = viewer::world::seat_of(session.doc(), copy);
    let outcome = session.perform(SessionOp::DeleteNode { node: extrude });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    session.pump();

    // No crash, no silent clear.
    assert_eq!(session.selection().edge(), Some(&selection));
    let standing = session.standing();
    assert!(!standing.live(), "a deleted feature's edge is not live");
    assert!(
        standing.unresolved().is_some(),
        "the failure is a typed verdict, not an absence: {standing:?}"
    );
    assert!(
        session.slot_rows().is_empty(),
        "dependent affordances are off"
    );
    // The picture is rebuilt from the surviving document, and the
    // vanished selection marks nothing in it.
    //
    // Un-nested deliberately: written as `if let Some(pair)` around
    // the assertion, a session that stopped landing an evaluation
    // would skip the check and the row would stay green having tested
    // nothing. Both steps are expectations, so both are failures.
    let after = plate_index(&session);
    let overlay = marks::edge_overlay(
        &after,
        &DisplayView::none(),
        session.selection(),
        session.hover(),
    );
    assert!(overlay.is_empty(), "a vanished edge lights nothing");
}

// --- the pixel measure ------------------------------------------------

/// **A viewport no pixel distance can be measured in picks no edge,
/// and refuses nothing about the cursor.**
///
/// The edge rule measures PIXELS, and a pixel distance is arithmetic
/// over numbers `ViewportSize` scales: `viewer::pickindex`'s
/// `segment_distance_px` squares a pixel separation, which overflows
/// past about `1.34e154`, and the quotient that follows is `inf/inf`.
/// So the walk can be handed a projection that is not a measurement
/// **with every input to the door finite** — no cursor, placement or
/// mesh position has to be a `NaN`, and the two routes that would
/// need one are closed upstream anyway (`Camera::ray_through` refuses
/// a cursor that is not a number, and `Camera::project` answers
/// `None` for a position that is not).
///
/// Before the admission at `best_segment`, such a segment took
/// neither side of `distance > EDGE_PICK_RADIUS_PX`, was installed as
/// the best, held its boundary against every later candidate, and
/// handed its `NaN` pixel to the occlusion probe — so the door
/// refused with `the camera's cursor x is NaN`, **naming the caller's
/// cursor for a pixel the walk had computed**.
///
/// The rows are a pair, because neither half says anything alone. The
/// scales are one picture in different pixel units: the aspect is
/// `pane()`'s at every one of them, so the camera, the projection and
/// the NDC are identical and the only thing that changes is the size
/// of the number the measure is taken in.
///
/// **What this does NOT claim.** `ViewportSize` is unbounded above and
/// is a door-level input; no screen is 1e155 pixels across and no
/// user-reachable producer of one was found.
#[test]
fn a_viewport_no_pixel_distance_can_be_measured_in_picks_no_edge() {
    let tol = Tol::witness();
    let (session, copy) = plate_session(tol);
    let index = plate_index(&session);
    let eval = eval_of(&session);
    let aspect = pane().aspect().expect("a positive aspect");
    let camera = common::framed(aspect);
    let (rim, points) = hole_rim(&index, copy);
    let at = (points.len() - 1) / 2;
    let wide = |scale: f64| ViewportSize {
        width_px: 1.28 * scale,
        height_px: 0.72 * scale,
    };
    // The cursor is DERIVED at each scale — the midpoint of the two
    // pixels the drawn segment's own endpoints land on, so it is ON
    // the chord the walk measures against whatever the viewport
    // measures in. (The projection of the 3-D midpoint is not: the
    // perspective divide puts it off the chord by a fraction of the
    // chord, and a fraction of `1e100` pixels is not within a
    // six-pixel radius. The row would then be measuring the fixture.)
    let cursor_in = |viewport: ViewportSize| {
        let pixel = |point: Point3<f64>| {
            let ndc = camera
                .project(point, viewport.aspect().expect("a positive aspect"))
                .expect("the projection is defined")
                .expect("a framed point is in front of the eye");
            viewport
                .cursor_of([ndc[0], ndc[1]])
                .expect("a viewport with area names a pixel")
        };
        let (a, b) = (pixel(points[at]), pixel(points[at + 1]));
        [(a[0] + b[0]) * 0.5, (a[1] + b[1]) * 0.5]
    };

    // Measurable: the answer is the rim, at a distance that is one.
    for scale in [1.0e2_f64, 1.0e100, 1.0e150] {
        let viewport = wide(scale);
        let pick = index
            .edge_at_for(
                eval,
                &camera,
                viewport,
                cursor_in(viewport),
                &DisplayView::none(),
            )
            .expect("a finite cursor names a ray whatever the viewport measures in")
            .unwrap_or_else(|| {
                panic!("the cursor is the rim segment's own midpoint, at {scale:e} across")
            });
        assert_eq!(
            pick.id(),
            rim,
            "the same picture answers the same edge at {scale:e} pixels across"
        );
        assert!(
            pick.distance_px.is_finite() && pick.distance_px <= EDGE_PICK_RADIUS_PX,
            "the pick carries a measurement, not {} at {scale:e}",
            pick.distance_px
        );
    }

    // Past the measure: nothing is picked, and nothing is refused
    // about the cursor the caller passed.
    for scale in [1.0e160_f64, 1.0e200] {
        let viewport = wide(scale);
        let answer = index
            .edge_at_for(
                eval,
                &camera,
                viewport,
                cursor_in(viewport),
                &DisplayView::none(),
            )
            .expect("a pixel the walk could not measure is not the caller's cursor");
        assert!(
            answer.as_ref().is_none_or(
                |pick| pick.distance_px.is_finite() && pick.distance_px <= EDGE_PICK_RADIUS_PX
            ),
            "whatever is answered at {scale:e} is within the radius by construction"
        );
    }
}

// --- the display view ------------------------------------------------

/// **The pick obeys the picture.** A hidden copy is out of the pick
/// exactly as it is out of the scene, and a free-moved instance is
/// picked WHERE IT IS DRAWN — both statements about the same
/// `DisplayView` the viewport draws under, and neither reachable
/// through the display-view-less wrappers the rows above use.
#[test]
fn a_hidden_copy_offers_no_edge_and_a_probed_one_picks_where_it_is_drawn() {
    let tol = Tol::witness();
    let (session, copy) = plate_session(tol);
    let index = plate_index(&session);
    let aspect = pane().aspect().expect("a positive aspect");
    let camera = common::framed(aspect);
    let eval = eval_of(&session);
    let (id, points) = hole_rim(&index, copy);
    let at = (points.len() - 1) / 2;
    let midpoint = Point3::new(
        (points[at].x + points[at + 1].x) * 0.5,
        (points[at].y + points[at + 1].y) * 0.5,
        (points[at].z + points[at + 1].z) * 0.5,
    );

    // Hidden: nothing is drawn there, so nothing is picked there.
    let hidden = DisplayView {
        hidden_placements: std::collections::BTreeSet::from([copy]),
        ..DisplayView::none()
    };
    assert!(
        index
            .edge_at_for(eval, &camera, pane(), pixel_of(&camera, midpoint), &hidden)
            .expect("un-projects")
            .is_none(),
        "a hidden copy is out of the pick as it is out of the picture"
    );
    assert!(
        index.edge_polyline_for(id, &hidden).is_empty(),
        "and it marks nothing either"
    );
    // The plate is placed once, so hiding it hides the picture, which
    // then has no extent: a hidden part does not frame.
    let blank = index.scene_for(&hidden).expect("a blank picture");
    assert!(blank.positions().is_empty(), "nothing is drawn");
    assert_eq!(blank.bounds(), None, "and nothing frames");
    assert!(
        index
            .scene_for(&DisplayView::none())
            .expect("the plate draws")
            .bounds()
            .is_some(),
        "while the plate shown frames"
    );

    // Free-moved: the edge is picked at the DISPLACED midpoint, and
    // not at the tessellated one.
    let shift = [0.0, 0.0, 0.05];
    let probed = DisplayView {
        moved_placements: std::collections::BTreeMap::from([(copy, Frame::translation(shift))]),
        ..DisplayView::none()
    };
    let moved = Point3::new(
        midpoint.x + shift[0],
        midpoint.y + shift[1],
        midpoint.z + shift[2],
    );
    let pick = index
        .edge_at_for(eval, &camera, pane(), pixel_of(&camera, moved), &probed)
        .expect("un-projects")
        .expect("the probed edge is picked where it is drawn");
    assert_eq!(pick.id(), id);
    assert!(
        (pick.point.z - moved.z).abs() < 1.0e-9,
        "the pick point rides the probe frame: {} vs {}",
        pick.point.z,
        moved.z
    );
    let marked = index.edge_polyline_for(id, &probed);
    assert!(
        marked
            .iter()
            .zip(&points)
            .all(|(drawn, tessellated)| (drawn.z - tessellated.z - shift[2]).abs() < 1.0e-12),
        "the mark is drawn under the same probe frame the pick used"
    );
}

// --- the kind filter -------------------------------------------------

/// **A faces-only tool gets its face inside the edge radius.**
///
/// Without the filter the mate tool could not take a face wherever the
/// cursor came within `EDGE_PICK_RADIUS_PX` of that face's own
/// boundary — which on a narrow face is everywhere. The row drives the
/// same op door the application drives, at one cursor, under both
/// filters.
#[test]
fn a_faces_only_pick_answers_the_face_where_an_unfiltered_one_answers_the_edge() {
    let tol = Tol::witness();
    let (session, copy) = plate_session(tol);
    let index = plate_index(&session);
    let aspect = pane().aspect().expect("a positive aspect");
    let camera = common::framed(aspect);
    let eval = eval_of(&session);
    let (_, points) = hole_rim(&index, copy);
    let (a, b) = middle_segment(&camera, &points);
    let cursor = offset_from(a, b, plate_centre_px(&camera), 1.0);

    let unfiltered = index
        .op_under(
            eval,
            &camera,
            pane(),
            PickAction::Select(cursor),
            &DisplayView::none(),
            PickKinds::Any,
        )
        .expect("un-projects");
    assert!(
        matches!(unfiltered, SessionOp::Select(Selection::Edge(_))),
        "one pixel from the rim the bare cursor means the edge, got {unfiltered:?}"
    );

    let faces_only = index
        .op_under(
            eval,
            &camera,
            pane(),
            PickAction::Select(cursor),
            &DisplayView::none(),
            PickKinds::FacesOnly,
        )
        .expect("un-projects");
    let SessionOp::Select(Selection::Face(face)) = &faces_only else {
        panic!("a faces-only pick answers a face or nothing, got {faces_only:?}");
    };
    assert_eq!(face.node, copy, "and it is the face the ray hit");
    // The mate tool's feed matches exactly this shape, which is what
    // the filter exists to keep reachable.
    assert_eq!(
        index
            .faces_under_cursor(eval, &camera, pane(), cursor, &DisplayView::none())
            .expect("un-projects")
            .into_iter()
            .map(|under| under.name)
            .collect::<Vec<_>>(),
        vec![face.name.clone()],
        "the filtered answer is the ray path's own face, unnarrowed"
    );

    // Hover obeys the same rule, so the picture and the click agree
    // about what the open tool will get.
    let hovered = index
        .hovered_for(
            eval,
            &camera,
            pane(),
            cursor,
            &DisplayView::none(),
            PickKinds::FacesOnly,
        )
        .expect("un-projects");
    assert!(matches!(hovered, Some(Hovered::Face(_))), "{hovered:?}");
}

/// **The mirror narrowing**: an edges-only cursor answers the edge
/// where the bare cursor does, and answers NOTHING where the bare
/// cursor falls back to the face.
///
/// That is what the blend tool needs: it consumes edges and nothing
/// else, so a cursor that missed every edge must not quietly
/// re-select the wall behind the one the user was aiming at. Hover
/// obeys the same rule, so the picture and the click agree about what
/// the open tool is about to get.
#[test]
fn an_edges_only_pick_answers_nothing_where_an_unfiltered_one_answers_the_face() {
    let tol = Tol::witness();
    let (session, copy) = plate_session(tol);
    let index = plate_index(&session);
    let aspect = pane().aspect().expect("a positive aspect");
    let camera = common::framed(aspect);
    let eval = eval_of(&session);
    let (_, points) = hole_rim(&index, copy);
    let (a, b) = middle_segment(&camera, &points);
    let inward = plate_centre_px(&camera);

    let pick_at = |cursor: [f64; 2], kinds: PickKinds| {
        index
            .op_under(
                eval,
                &camera,
                pane(),
                PickAction::Select(cursor),
                &DisplayView::none(),
                kinds,
            )
            .expect("un-projects")
    };

    // On the rim: both rules answer the same edge. Compared as the
    // typed selections they are — `EdgeSelection` is `PartialEq`, and
    // two `Debug` strings agreeing is a weaker claim that would also
    // hold for two values that merely print alike.
    let near = offset_from(a, b, inward, 1.0);
    let selected = |op: &SessionOp| match op {
        SessionOp::Select(selection) => selection.clone(),
        other => panic!("a click emits a selection, got {other:?}"),
    };
    let bare = selected(&pick_at(near, PickKinds::Any));
    let filtered = selected(&pick_at(near, PickKinds::EdgesOnly));
    assert!(matches!(&bare, Selection::Edge(_)), "{bare:?}");
    assert_eq!(bare, filtered, "on an edge the narrowing changes nothing");

    // Well inside the plate, off every edge: the bare cursor means the
    // top face, and the edges-only cursor means nothing at all.
    let far = offset_from(a, b, inward, EDGE_PICK_RADIUS_PX * 4.0);
    let bare_far = selected(&pick_at(far, PickKinds::Any));
    assert!(
        matches!(&bare_far, Selection::Face(_)),
        "the bare cursor falls back to the face, got {bare_far:?}"
    );
    assert_eq!(
        selected(&pick_at(far, PickKinds::EdgesOnly)),
        Selection::None,
        "an edges-only cursor off every edge picks nothing"
    );
    assert!(
        index
            .hovered_for(
                eval,
                &camera,
                pane(),
                far,
                &DisplayView::none(),
                PickKinds::EdgesOnly,
            )
            .expect("un-projects")
            .is_none(),
        "and hovers nothing, so the picture agrees with the click"
    );
}

/// **A cursor the FACE pick ties on still picks the edge.**
///
/// A cursor on a shared edge is the pixel a user aims an edge with,
/// and it is also where the face pick ties — the ray meets both faces
/// the edge belongs to and the arithmetic orders neither. Seeding the
/// edge search on the face ANSWER therefore refuses the edge pick
/// exactly where an edge is what the cursor means; the seed is a
/// depth instead (`PickIndex::front_of`), and only the face answer
/// refuses.
///
/// The fixture is the shipped plate's own drawn edges, swept at every
/// segment midpoint: the sweep both finds the tied cursors and pins
/// the edge door over all of them. The premise is asserted, so a
/// plate that stopped producing tied cursors reds here rather than
/// passing vacuously.
#[test]
fn a_cursor_the_face_pick_ties_on_still_picks_the_edge() {
    let tol = Tol::witness();
    let (session, copy) = plate_session(tol);
    let index = plate_index(&session);
    let aspect = pane().aspect().expect("a positive aspect");
    let camera = common::framed(aspect);
    let eval = eval_of(&session);
    let mut tied_cursors = 0usize;
    let mut edge_refused = 0usize;
    let mut edge_missed = 0usize;
    let mut cursors = 0usize;
    let mut edges_hit = std::collections::BTreeSet::new();
    let mut edges_all = 0usize;
    let mut example = String::new();
    for (id, points) in drawn_edges(&index, copy) {
        edges_all += 1;
        for pair in points.windows(2) {
            let mid = Point3::new(
                (pair[0].x + pair[1].x) * 0.5,
                (pair[0].y + pair[1].y) * 0.5,
                (pair[0].z + pair[1].z) * 0.5,
            );
            let cursor = pixel_of(&camera, mid);
            cursors += 1;
            let faces = index
                .faces_under_cursor(eval, &camera, pane(), cursor, &DisplayView::none())
                .expect("the cursor un-projects");
            if faces.len() < 2 {
                continue;
            }
            tied_cursors += 1;
            edges_hit.insert(format!("{id:?}"));
            match index.edge_at_for(eval, &camera, pane(), cursor, &DisplayView::none()) {
                Ok(Some(pick)) => {
                    if example.is_empty() {
                        example = format!("tied at {cursor:?}, edge pick answered {pick:?}");
                    }
                }
                Ok(None) => {
                    edge_missed += 1;
                    if example.is_empty() {
                        example = format!("tied at {cursor:?}, edge pick answered NOTHING");
                    }
                }
                Err(error) => {
                    edge_refused += 1;
                    if example.is_empty() {
                        example = format!("tied at {cursor:?}, edge pick REFUSED: {error}");
                    }
                }
            }
        }
    }
    println!(
        "# tied cursors {tied_cursors} of {cursors}, on {} of {edges_all} drawn edges; the \
         edge pick refuses at {edge_refused} of them and answers nothing at {edge_missed}; \
         {example}",
        edges_hit.len()
    );
    assert!(
        tied_cursors > 0,
        "the row's premise: the plate's drawn edges produce cursors the face pick ties on \
         ({cursors} cursors over {edges_all} edges)"
    );
    assert_eq!(
        edge_refused, 0,
        "a cursor the face pick ties on takes the EDGE pick down with it: {example}"
    );
    assert_eq!(
        edge_missed, 0,
        "the cursor is on a drawn edge's own midpoint, so the edge door answers it: {example}"
    );
}

// --- the certified occlusion -----------------------------------------

/// A session over `doc`, evaluated and landed.
fn session_over(doc: pncad::document::Doc<pncad::document::ProfileProgram>) -> DocSession {
    let mut session = DocSession::inline(doc, Tol::witness());
    session.pump();
    session
}

/// `doc` with a prism added and placed where it stands: `corners`
/// (counter-clockwise in the xy plane) extruded `height` up `z`.
/// Answers the document and the placement, the node its drawn copy is
/// picked on.
fn with_prism(
    doc: &pncad::document::Doc<pncad::document::ProfileProgram>,
    corners: &[(f64, f64)],
    height: f64,
) -> (
    pncad::document::Doc<pncad::document::ProfileProgram>,
    RecipeNodeId,
) {
    let tol = Tol::witness();
    let outline =
        pncad::document::LoopProgram::polygon(corners.iter().copied()).expect("finite corners");
    let (doc, profile) = common::inserted(
        doc,
        pncad::document::Node::Profile(pncad::document::ProfileProgram {
            loops: vec![outline],
            ids: Vec::new(),
            on: None,
        }),
        tol,
    );
    let (doc, extrude) = common::inserted(
        &doc,
        pncad::document::Node::Extrude {
            profile: profile.into(),
            distance: common::len(height),
        },
        tol,
    );
    common::placed(&doc, extrude, tol)
}

/// The drawn edge of `node` whose every chord point satisfies `on`.
fn edge_where(
    index: &PickIndex,
    node: RecipeNodeId,
    on: impl Fn(Point3<f64>) -> bool,
) -> (EdgeId, Vec<Point3<f64>>) {
    drawn_edges(index, node)
        .into_iter()
        .find(|(_, points)| !points.is_empty() && points.iter().all(|&p| on(p)))
        .expect("the fixture draws the edge")
}

fn lerp(a: Point3<f64>, b: Point3<f64>, f: f64) -> Point3<f64> {
    Point3::new(
        a.x + (b.x - a.x) * f,
        a.y + (b.y - a.y) * f,
        a.z + (b.z - a.z) * f,
    )
}

/// **A visible concave edge stays visible at grazing incidence.**
///
/// An L-shaped block's inner vertical edge, seen head-on across one of
/// its two faces and edge-on across the other, at ever smaller angles
/// to that second face. The ray through a pixel on the edge misses it
/// by its own rounding, and on the material side of the grazed face
/// that offset puts the face's crossing in front of the edge by the
/// offset over the angle's sine — unbounded as the angle closes. The
/// crossing is of a face the edge bounds, so it is the edge's own, and
/// the edge is picked at every cursor along it.
#[test]
fn a_visible_concave_edge_at_grazing_incidence_is_picked() {
    let (w, h) = (0.04, 0.01);
    let half = w * 0.5;
    let doc = pncad::document::Doc::empty_derived("occl-concave", Tol::witness());
    let (doc, node) = with_prism(
        &doc,
        &[
            (0.0, 0.0),
            (w, 0.0),
            (w, half),
            (half, half),
            (half, w),
            (0.0, w),
        ],
        h,
    );
    let session = session_over(doc);
    let index = plate_index(&session);
    let eval = eval_of(&session);
    let near = |a: f64, b: f64| (a - b).abs() < 1.0e-9;
    let (concave, points) = edge_where(&index, node, |p| near(p.x, half) && near(p.y, half));
    let (bottom, top) = (points[0], points[points.len() - 1]);
    let mut cursors = 0usize;
    for grazing in [1.0e-2, 1.0e-4, 1.0e-6, 1.0e-8] {
        // The eye along +x, tilted `grazing` toward +y: the face x = w/2
        // is seen head-on and the face y = w/2 edge-on from its outside.
        let camera = Camera::new(
            Point3::new(half, half, h * 0.5),
            0.2,
            grazing,
            0.0,
            std::f64::consts::FRAC_PI_4,
            w,
        )
        .expect("a camera");
        for step in 1..10 {
            let at = lerp(bottom, top, f64::from(step) / 10.0);
            let cursor = pixel_of(&camera, at);
            cursors += 1;
            let picked = index
                .edge_at(eval, &camera, pane(), cursor)
                .expect("un-projects")
                .map(|pick| pick.id());
            assert_eq!(
                picked,
                Some(concave),
                "the concave edge at {at:?}, seen at {grazing:e} rad to one of its faces, \
                 is hidden by its own face's crossing"
            );
        }
    }
    assert_eq!(cursors, 36, "every grazing angle measured every cursor");
}

/// **An edge hidden by less than a millionth of its own depth is not
/// picked.**
///
/// The plate, framed from a hundred kilometres with a lens narrow
/// enough to fill the pane: every hidden edge then sits behind its
/// occluder by a gap of millimetres, which is below `1e-6` of its
/// depth, and the depth is still certified to far better than that.
/// The subjects are classified by this row's own oracle — a gap above a
/// micrometre that is under `1e-6` of the depth — and every one is
/// refused.
#[test]
fn an_edge_hidden_within_a_millionth_of_its_depth_is_not_picked() {
    let tol = Tol::witness();
    let (session, copy) = plate_session(tol);
    let index = plate_index(&session);
    let eval = eval_of(&session);
    let [width, depth, thickness] = PLATE_EXTENT;
    let distance = 1.0e5;
    let camera = Camera::new(
        Point3::new(width * 0.5, depth * 0.5, thickness * 0.5),
        distance,
        -std::f64::consts::FRAC_PI_3,
        std::f64::consts::FRAC_PI_6,
        2.0 * (width / distance).atan(),
        distance,
    )
    .expect("a camera");
    let mut hidden = 0usize;
    for (id, points) in drawn_edges(&index, copy) {
        if points.len() < 2 {
            continue;
        }
        let at = (points.len() - 1) / 2;
        let midpoint = lerp(points[at], points[at + 1], 0.5);
        let cursor = pixel_of(&camera, midpoint);
        let ray = camera.ray_through(cursor, pane()).expect("un-projects");
        let Some(front) = nearest_surface(&index, eval, &ray) else {
            continue;
        };
        let along = ray.dir.dot(midpoint - ray.origin) / ray.dir.dot(ray.dir);
        let gap = along - front;
        if !(gap > 1.0e-6 && gap < along * 1.0e-6) {
            continue;
        }
        hidden += 1;
        let picked = index
            .edge_at(eval, &camera, pane(), cursor)
            .expect("un-projects");
        assert_ne!(
            picked.as_ref().map(viewer::pickindex::EdgePick::id),
            Some(id),
            "an edge {gap:e} behind a surface at {front} ({:e} of its depth) was picked \
             through the solid",
            gap / along
        );
    }
    assert!(
        hidden > 0,
        "no drawn edge of this view is hidden by under 1e-6 of its depth, so this row \
         measured nothing"
    );
}

/// **A silhouette edge is picked where the ray through it crosses
/// neither of its faces.**
///
/// The hole's rim, from the default framing: along its near half the
/// top face and the hole's wall both lie on one side of the rim in the
/// picture, so a ray through a rim pixel that its rounding puts on the
/// other side passes into the hole, crosses neither, and meets the far
/// wall behind. The rim's own crossing is then the span of its own
/// point alone, and nothing in front precedes it. The subjects are the
/// chords where the row sees exactly that — no incident triangle
/// crossed, a surface behind — and every one is picked.
#[test]
fn a_silhouette_edge_whose_faces_the_ray_misses_is_picked() {
    let tol = Tol::witness();
    let (session, copy) = plate_session(tol);
    // At 0.7 of the plate δ. Whether a rim chord's midpoint pixel rounds
    // past the rim turns on the chord count's parity against the
    // framing; at the plate δ itself the rim has no such chord, and at
    // 0.7 of it three. Half the plate δ also has them, but that is the
    // rim the δ/2 sizing cut, so it would test the old mesh rather than
    // this one.
    let witness = viewer::scene::DisplayTolerance::new(common::plate_delta().get() * 0.7)
        .expect("a positive delta");
    let index = common::index_of(&session, witness);
    let aspect = pane().aspect().expect("a positive aspect");
    let camera = common::framed(aspect);
    let eval = eval_of(&session);
    let [width, depth, thickness] = PLATE_EXTENT;
    let (cx, cy) = (width * 0.5, depth * 0.5);
    let part = index
        .parts()
        .iter()
        .find(|part| part.node() == copy)
        .expect("the plate is drawn");
    let mesh = part.mesh();
    let mut subjects = 0usize;
    for (id, points) in drawn_edges(&index, copy) {
        let rim = points.iter().all(|p| {
            (p.z - thickness).abs() < 1.0e-9
                && ((p.x - cx).powi(2) + (p.y - cy).powi(2)).sqrt() < PLATE_HOLE_RADIUS * 1.05
        });
        if !rim {
            continue;
        }
        let chord_points = &mesh.boundaries[id.boundary].points;
        for (segment, pair) in points.windows(2).enumerate() {
            let cursor = pixel_of(&camera, lerp(pair[0], pair[1], 0.5));
            let ray = camera.ray_through(cursor, pane()).expect("un-projects");
            let (from, to) = (chord_points[segment], chord_points[segment + 1]);
            let crosses_a_face = mesh
                .patches
                .iter()
                .flat_map(|patch| &patch.triangles)
                .filter(|tri| tri.contains(&from) && tri.contains(&to))
                .any(|tri| {
                    let corners = tri.map(|i| mesh.positions[i as usize]);
                    editor_core::resolve::ray_triangle(&ray, &corners).is_some()
                });
            if crosses_a_face || nearest_surface(&index, eval, &ray).is_none() {
                continue;
            }
            subjects += 1;
            let picked = index
                .edge_at(eval, &camera, pane(), cursor)
                .expect("un-projects")
                .map(|pick| pick.id());
            assert_eq!(
                picked,
                Some(id),
                "the rim's chord {segment} is a silhouette over the far wall and was not picked"
            );
        }
    }
    assert!(
        subjects > 0,
        "no rim chord's ray passes outside both its faces with a surface behind, so this \
         row measured nothing"
    );
}

/// **A tie across two parts offers both parts' edges.**
///
/// Two blocks placed side by side, overlapping by a third of their
/// width, their top faces coincident over the overlap. A cursor there
/// meets both tops and the door names both; the edge search is over
/// both bodies, so a cursor beside EITHER block's edge on the overlap
/// picks that edge — whichever body the list or a rounded parameter
/// would have put first.
#[test]
fn a_tie_across_two_parts_offers_both_parts_edges() {
    let (side, h) = (0.02, 0.01);
    let doc = pncad::document::Doc::empty_derived("occl-two-parts", Tol::witness());
    let square = |x0: f64| [(x0, 0.0), (x0 + side, 0.0), (x0 + side, side), (x0, side)];
    let (doc, left) = with_prism(&doc, &square(0.0), h);
    let (doc, right) = with_prism(&doc, &square(side * 0.5), h);
    let session = session_over(doc);
    let index = plate_index(&session);
    let eval = eval_of(&session);
    let aspect = pane().aspect().expect("a positive aspect");
    let bounds = bvh::Aabb {
        min_x: 0.0,
        min_y: 0.0,
        min_z: 0.0,
        max_x: side * 1.5,
        max_y: side,
        max_z: h,
    };
    let camera = Camera::framing(Some(bounds), aspect).expect("the pair frames");
    let near = |a: f64, b: f64| (a - b).abs() < 1.0e-9;
    // The right block's left top edge lies on the left block's top, and
    // the left block's right top edge on the right block's.
    let (right_edge, right_points) =
        edge_where(&index, right, |p| near(p.x, side * 0.5) && near(p.z, h));
    let (left_edge, left_points) = edge_where(&index, left, |p| near(p.x, side) && near(p.z, h));
    for (name, id, points, other) in [
        ("the right block's", right_edge, &right_points, &left_points),
        ("the left block's", left_edge, &left_points, &right_points),
    ] {
        let (a, b) = middle_segment(&camera, points);
        let (c, d) = middle_segment(&camera, other);
        let toward = [(c[0] + d[0]) * 0.5, (c[1] + d[1]) * 0.5];
        let cursor = offset_from(a, b, toward, 3.0);
        let faces = index
            .faces_under_cursor(eval, &camera, pane(), cursor, &DisplayView::none())
            .expect("un-projects");
        let parts: std::collections::BTreeSet<RecipeNodeId> =
            faces.iter().map(|face| face.node).collect();
        assert_eq!(
            parts.len(),
            2,
            "the premise: {name} edge's cursor is over both coincident tops, not {faces:?}"
        );
        let picked = index
            .edge_at(eval, &camera, pane(), cursor)
            .expect("un-projects")
            .map(|pick| pick.id());
        assert_eq!(
            picked,
            Some(id),
            "{name} edge, 3 px from the cursor on a two-part tie, was not offered"
        );
    }
}

/// **A cursor on a chord's END, over a grazed face, picks an edge
/// through that end.**
///
/// The plate, with each of its five visible faces grazed in turn — the
/// top from a hair above it, each side from a hair outside it — at
/// three angles. Every drawn chord point on the grazed face is aimed at
/// its own pixel, the pixel the edge walk clamps any cursor past a
/// chord's end onto, so the candidate there is a chord at `s → 1` (the
/// one ending at the point: a curved edge keeps its earlier segment at
/// a shared point) or `s → 0` (an edge starting at a vertex). The ray
/// through a chord point misses it by its own rounding, and on the
/// material side of the grazed face that miss is a crossing of a
/// triangle around the point well in front of it — a triangle of the
/// END's fan, which is not always one of the chord's own pair. Every
/// chord point this row's own `1e-6` band calls visible is picked, by
/// an edge whose polyline holds it.
#[test]
fn a_cursor_on_a_chord_end_over_a_grazed_face_picks_an_edge_through_it() {
    let tol = Tol::witness();
    let (session, copy) = plate_session(tol);
    let index = plate_index(&session);
    let eval = eval_of(&session);
    let [width, depth, thickness] = PLATE_EXTENT;
    let edges = drawn_edges(&index, copy);
    let same = |q: Point3<f64>, p: Point3<f64>| {
        [q.x, q.y, q.z].map(f64::to_bits) == [p.x, p.y, p.z].map(f64::to_bits)
    };
    let mut points: Vec<Point3<f64>> = Vec::new();
    for (_, polyline) in &edges {
        for &p in polyline {
            if !points.iter().any(|&q| same(q, p)) {
                points.push(p);
            }
        }
    }
    use std::f64::consts::{FRAC_PI_2, FRAC_PI_3, PI};
    // (yaw, pitch, the grazed face) for an angle `g` off that face.
    type OnFace = fn(Point3<f64>, [f64; 3]) -> bool;
    type View = fn(f64) -> (f64, f64);
    let faces: [(View, OnFace); 10] = [
        (
            |g| (-FRAC_PI_3, g),
            |p, [_, _, t]| (p.z - t).abs() < 1.0e-12,
        ),
        (|g| (-FRAC_PI_3, -g), |p, _| p.z.abs() < 1.0e-12),
        (|g| (-g, 0.5), |p, _| p.y.abs() < 1.0e-12),
        (|g| (-g, -0.5), |p, _| p.y.abs() < 1.0e-12),
        (|g| (g, 0.5), |p, [_, d, _]| (p.y - d).abs() < 1.0e-12),
        (|g| (g, -0.5), |p, [_, d, _]| (p.y - d).abs() < 1.0e-12),
        (|g| (FRAC_PI_2 + g, 0.5), |p, _| p.x.abs() < 1.0e-12),
        (|g| (FRAC_PI_2 + g, -0.5), |p, _| p.x.abs() < 1.0e-12),
        (
            |g| (FRAC_PI_2 - g + PI, 0.5),
            |p, [w, _, _]| (p.x - w).abs() < 1.0e-12,
        ),
        (
            |g| (FRAC_PI_2 - g + PI, -0.5),
            |p, [w, _, _]| (p.x - w).abs() < 1.0e-12,
        ),
    ];
    let mut subjects = 0usize;
    for (view, on_face) in faces {
        for grazing in (3..=10).map(|k| 10f64.powi(-k)) {
            let (yaw, pitch) = view(grazing);
            let camera = Camera::new(
                Point3::new(width * 0.5, depth * 0.5, thickness),
                0.2,
                yaw,
                pitch,
                std::f64::consts::FRAC_PI_4,
                width,
            )
            .expect("a camera");
            for &p in points.iter().filter(|&&p| on_face(p, PLATE_EXTENT)) {
                let cursor = pixel_of(&camera, p);
                let ray = camera.ray_through(cursor, pane()).expect("un-projects");
                let Some(front) = nearest_surface(&index, eval, &ray) else {
                    continue;
                };
                let along = ray.dir.dot(p - ray.origin) / ray.dir.dot(ray.dir);
                if front < along * (1.0 - 1.0e-6) {
                    continue; // hidden at its own pixel — not this row's subject
                }
                subjects += 1;
                let picked = index
                    .edge_at(eval, &camera, pane(), cursor)
                    .expect("un-projects")
                    .map(|pick| pick.id());
                let through: Vec<EdgeId> = edges
                    .iter()
                    .filter(|(_, polyline)| polyline.iter().any(|&q| same(q, p)))
                    .map(|(id, _)| *id)
                    .collect();
                assert!(
                    picked.is_some_and(|id| through.contains(&id)),
                    "the chord point {p:?}, visible {grazing:e} rad off a grazed face (yaw \
                     {yaw}, pitch {pitch}), picked {picked:?} rather than one of the edges \
                     through it {through:?}"
                );
            }
        }
    }
    assert!(
        subjects > 0,
        "no chord point on a grazed face is visible at its own pixel, so this row measured \
         nothing"
    );
}
