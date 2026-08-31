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
//! `pick::edge_overlay` — and the pixels are issue #1097's hardware
//! checklist.

// Panicking is a test's failure mechanism (workspace lint note).
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

mod common;

use pncad::document::{Evaluation, Frame, RecipeNodeId};
use pncad::geom_core::{Point3, Tol};
use pncad::select::{Resolution, RunCtx, resolve};
use viewer::camera::Camera;
use viewer::display::DisplayView;
use viewer::input::{PickAction, ViewportSize};
use viewer::pick;
use viewer::pick::{EDGE_PICK_RADIUS_PX, EdgeId, PickIndex, PickKinds};
use viewer::scene::{self, PLATE_EXTENT, PLATE_HOLE_RADIUS};
use viewer::session::{DocSession, EdgeSelection, Hovered, Selection, SessionOp};

/// The pane every row picks in.
fn pane() -> ViewportSize {
    ViewportSize {
        width_px: 1280.0,
        height_px: 720.0,
    }
}

/// The display tolerance every row here uses — the same reading
/// `select_pick` takes, for the same reason: coarse enough to keep the
/// suite cheap, fine enough that the hole is a ring of facets.
fn delta() -> scene::DisplayTolerance {
    scene::DisplayTolerance::new(2.0e-4).expect("a positive delta")
}

/// A session over the spike plate, evaluated and landed.
fn plate_session(tol: Tol) -> (DocSession, RecipeNodeId) {
    let (doc, extrude) = scene::plate_with_hole(tol).expect("the plate authors");
    let mut session = DocSession::inline(doc, tol);
    session.pump();
    (session, extrude)
}

/// The pick index for a session's landed evaluation.
fn index_of(session: &DocSession) -> PickIndex {
    let (doc, eval) = session
        .landed_pair()
        .expect("the inline seam lands its first evaluation");
    let generation = session
        .landed_generation()
        .expect("a landed evaluation has a generation");
    PickIndex::build(doc, eval, generation, delta(), session.tol()).expect("the plate indexes")
}

/// The landed evaluation, for the doors that take one.
fn eval_of(session: &DocSession) -> &Evaluation<f64> {
    session.evaluation().expect("an evaluation has landed")
}

/// Every drawn edge of the plate, with the polyline it is drawn as.
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
    drawn_edges(index, node)
        .into_iter()
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
    let (session, extrude) = plate_session(tol);
    let index = index_of(&session);
    let (doc, eval) = session.landed_pair().expect("a landed pair");
    let edges = drawn_edges(&index, extrude);
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
            matches!(resolve(RunCtx { doc, eval }, name), Resolution::Resolved(_)),
            "a just-drawn edge's name resolves in the run it was drawn from"
        );
        assert!(names.insert(name.clone()), "one name per drawn edge");
    }
}

#[test]
fn an_edge_selection_narrows_to_the_copy_it_was_picked_from() {
    let tol = Tol::witness();
    let (session, extrude) = plate_session(tol);
    let index = index_of(&session);
    let (id, _) = hole_rim(&index, extrude);
    let name = index
        .edge_name_of(id)
        .expect("a drawn edge has a name")
        .clone();
    let picked = EdgeSelection {
        name: name.clone(),
        node: extrude,
        body: 0,
    };
    assert_eq!(index.edges_of_target(&picked), vec![id]);
    // A body this index does not draw carries no copy of the name,
    // which is how a stale or foreign selection lights nothing.
    let elsewhere = EdgeSelection {
        name,
        node: extrude,
        body: 7,
    };
    assert!(index.edges_of_target(&elsewhere).is_empty());
}

// --- aiming ---------------------------------------------------------

#[test]
fn a_cursor_on_a_drawn_edge_picks_that_edge() {
    let tol = Tol::witness();
    let (session, extrude) = plate_session(tol);
    let index = index_of(&session);
    let aspect = pane().aspect().expect("a positive aspect");
    let camera = common::framed(aspect);
    let (id, points) = hole_rim(&index, extrude);
    let (a, b) = middle_segment(&camera, &points);
    let cursor = [(a[0] + b[0]) * 0.5, (a[1] + b[1]) * 0.5];

    let pick = index
        .edge_at(eval_of(&session), &camera, pane(), cursor)
        .expect("the cursor un-projects")
        .expect("a cursor on a drawn edge picks it");
    assert_eq!(pick.id(), id, "the edge under the cursor is the one picked");
    assert_eq!(pick.node, extrude);
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
    let (mut session, extrude) = plate_session(tol);
    let index = index_of(&session);
    let aspect = pane().aspect().expect("a positive aspect");
    let camera = common::framed(aspect);
    let (id, points) = hole_rim(&index, extrude);
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
        Some(extrude),
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
    let (session, extrude) = plate_session(tol);
    let index = index_of(&session);
    let aspect = pane().aspect().expect("a positive aspect");
    let camera = common::framed(aspect);
    let (id, points) = hole_rim(&index, extrude);
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
                assert_eq!(face.node, extrude, "the cursor is still over the plate");
            }
            other => panic!("a cursor over the plate selects something: {other:?}"),
        }
    }
}

#[test]
fn a_cursor_over_the_background_picks_nothing_at_all() {
    let tol = Tol::witness();
    let (session, _) = plate_session(tol);
    let index = index_of(&session);
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
    let (session, extrude) = plate_session(tol);
    let index = index_of(&session);
    let aspect = pane().aspect().expect("a positive aspect");
    let camera = common::framed(aspect);
    let eval = eval_of(&session);

    let mut checked = 0usize;
    for (id, points) in drawn_edges(&index, extrude) {
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
        let Some(front) = index
            .pick_for(eval, &ray, &DisplayView::none())
            .expect("no refusal")
        else {
            continue;
        };
        // How far along the ray the edge's own midpoint sits.
        let midpoint = Point3::new((a.x + b.x) * 0.5, (a.y + b.y) * 0.5, (a.z + b.z) * 0.5);
        let depth = ray.dir.dot(midpoint - ray.origin);
        // The 1e-6 band is this row's OWN reading of "the same depth",
        // spelled here rather than imported from the implementation's
        // `OCCLUSION_SLACK_REL` (which is private, and would make the
        // row agree with the code by construction). An independent
        // number is the point: it says the two populations this row
        // sorts are far enough apart that any sane threshold separates
        // them.
        if front.t >= depth * (1.0 - 1.0e-6) {
            continue; // visible at its own pixel — not this row's subject
        }
        checked += 1;
        let picked = index
            .edge_at(eval, &camera, pane(), cursor)
            .expect("un-projects");
        assert_ne!(
            picked.as_ref().map(viewer::pick::EdgePick::id),
            Some(id),
            "an edge {depth} deep behind a surface at {} was picked through the solid",
            front.t
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
    let (session, extrude) = plate_session(tol);
    let index = index_of(&session);
    let aspect = pane().aspect().expect("a positive aspect");
    let camera = common::framed(aspect);
    let (_, points) = hole_rim(&index, extrude);
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
    let (mut session, extrude) = plate_session(tol);
    let index = index_of(&session);
    let (rim, points) = hole_rim(&index, extrude);
    let selection = EdgeSelection {
        name: index
            .edge_name_of(rim)
            .expect("a drawn edge has a name")
            .clone(),
        node: extrude,
        body: 0,
    };
    let other = drawn_edges(&index, extrude)
        .into_iter()
        .find(|(id, run)| *id != rim && run.len() >= 2)
        .expect("the plate draws more than one edge");
    let hovered = EdgeSelection {
        name: index
            .edge_name_of(other.0)
            .expect("a drawn edge has a name")
            .clone(),
        node: extrude,
        body: 0,
    };

    let empty = pick::edge_overlay(&index, &DisplayView::none(), &Selection::None, None);
    assert!(empty.is_empty(), "nothing selected marks nothing");

    session.perform(SessionOp::Select(Selection::Edge(selection.clone())));
    let marked = pick::edge_overlay(
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
    let both = pick::edge_overlay(
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
    let once = pick::edge_overlay(
        &index,
        &DisplayView::none(),
        session.selection(),
        session.hover(),
    );
    assert!(once.hovered.is_empty(), "the selected edge is marked once");
    assert_eq!(once.segments(), points.len() - 1);
}

#[test]
fn a_face_selection_marks_no_edge_and_an_edge_selection_marks_no_patch() {
    let tol = Tol::witness();
    let (session, extrude) = plate_session(tol);
    let index = index_of(&session);
    let (rim, _) = hole_rim(&index, extrude);
    let edge = Selection::Edge(EdgeSelection {
        name: index
            .edge_name_of(rim)
            .expect("a drawn edge has a name")
            .clone(),
        node: extrude,
        body: 0,
    });
    let lit = pick::highlight(&index, &edge, None);
    assert_eq!(
        lit.selected,
        viewer::pick::IdMap::NOTHING,
        "an edge selection tints no patch"
    );
    let face = index
        .face_at(
            eval_of(&session),
            &pncad::select::Ray {
                origin: Point3::new(0.005, 0.005, 1.0),
                dir: pncad::geom_core::Vec3::new(0.0, 0.0, -1.0),
            },
        )
        .expect("no refusal")
        .expect("a ray onto the plate hits it");
    let overlay = pick::edge_overlay(&index, &DisplayView::none(), &Selection::Face(face), None);
    assert!(overlay.is_empty(), "a face selection marks no edge");
}

// --- survival -------------------------------------------------------

/// The ratified resolution-failure semantics, on the edge arm: the
/// name stays, the verdict changes, the affordances switch off, and
/// the picture marks nothing.
#[test]
fn deleting_the_feature_leaves_the_edge_selection_unresolved() {
    let tol = Tol::witness();
    let (mut session, extrude) = plate_session(tol);
    let index = index_of(&session);
    let (rim, _) = hole_rim(&index, extrude);
    let selection = EdgeSelection {
        name: index
            .edge_name_of(rim)
            .expect("a drawn edge has a name")
            .clone(),
        node: extrude,
        body: 0,
    };
    session.perform(SessionOp::Select(Selection::Edge(selection.clone())));
    assert!(session.standing().live());

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
    let after = index_of(&session);
    let overlay = pick::edge_overlay(
        &after,
        &DisplayView::none(),
        session.selection(),
        session.hover(),
    );
    assert!(overlay.is_empty(), "a vanished edge lights nothing");
}

// --- the display view ------------------------------------------------

/// **The pick obeys the picture.** A hidden root is out of the pick
/// exactly as it is out of the scene, and a free-moved instance is
/// picked WHERE IT IS DRAWN — both statements about the same
/// `DisplayView` the viewport draws under, and neither reachable
/// through the display-view-less wrappers the rows above use.
#[test]
fn a_hidden_root_offers_no_edge_and_a_probed_one_picks_where_it_is_drawn() {
    let tol = Tol::witness();
    let (session, extrude) = plate_session(tol);
    let index = index_of(&session);
    let aspect = pane().aspect().expect("a positive aspect");
    let camera = common::framed(aspect);
    let eval = eval_of(&session);
    let (id, points) = hole_rim(&index, extrude);
    let at = (points.len() - 1) / 2;
    let midpoint = Point3::new(
        (points[at].x + points[at + 1].x) * 0.5,
        (points[at].y + points[at + 1].y) * 0.5,
        (points[at].z + points[at + 1].z) * 0.5,
    );

    // Hidden: nothing is drawn there, so nothing is picked there.
    let hidden = DisplayView {
        hidden_roots: std::collections::BTreeSet::from([extrude]),
        ..DisplayView::none()
    };
    assert!(
        index
            .edge_at_for(eval, &camera, pane(), pixel_of(&camera, midpoint), &hidden)
            .expect("un-projects")
            .is_none(),
        "a hidden root is out of the pick as it is out of the picture"
    );
    assert!(
        index.edge_polyline_for(id, &hidden).is_empty(),
        "and it marks nothing either"
    );

    // Free-moved: the edge is picked at the DISPLACED midpoint, and
    // not at the tessellated one.
    let shift = [0.0, 0.0, 0.05];
    let probed = DisplayView {
        moved_roots: std::collections::BTreeMap::from([(extrude, Frame::translation(shift))]),
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
    let (session, extrude) = plate_session(tol);
    let index = index_of(&session);
    let aspect = pane().aspect().expect("a positive aspect");
    let camera = common::framed(aspect);
    let eval = eval_of(&session);
    let (_, points) = hole_rim(&index, extrude);
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
    assert_eq!(face.node, extrude, "and it is the face the ray hit");
    // The mate tool's feed matches exactly this shape, which is what
    // the filter exists to keep reachable.
    assert_eq!(
        index
            .face_under_cursor(eval, &camera, pane(), cursor, &DisplayView::none())
            .expect("un-projects")
            .map(|under| under.name),
        Some(face.name.clone()),
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
