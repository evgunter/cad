//! **Cursor in, selection out** — the viewport-selection unit's
//! headless half.
//!
//! Everything about picking except the pixels is a value here: the
//! un-projection is a camera operation, the ray path is the shipped
//! hit-test service, the id buffer's alphabet is a pure function pair,
//! and the selection is a typed layer-3 value the panels and the
//! viewport share. So the rows below drive synthetic cursors through
//! the same functions the application drives and assert on the
//! selection state and the names that came out.
//!
//! What is NOT here, and cannot be: the GPU pass itself. Its geometry
//! is checked through [`viewer::cursor_projection`] — the transform
//! that decides which pixel it samples — and the rest is issue #1097's
//! hardware checklist.

// Panicking is a test's failure mechanism (workspace lint note).
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

mod common;

use pncad::document::{
    Doc, Evaluation, Expr, Node, PatternKind, ProfileProgram, RecipeNodeId, SlotId,
};
use pncad::geom_core::{Point3, Tol, Vec3};
use pncad::select::{Ray, Resolution};
use viewer::camera::Camera;
use viewer::input::{InputMap, PickAction, PointerButton, ViewportEvent, ViewportSize};
use viewer::pick::{IdMap, PatchId, PickIndex};
use viewer::props::SlotValue;
use viewer::scene::{self, PLATE_EXTENT};
use viewer::session::{DocSession, Selection, SessionOp, Standing};
use viewer::{cursor_projection, input, pick};

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

/// The display tolerance every row here uses. Coarse enough to keep
/// the suite cheap, fine enough that the hole is a ring of facets
/// rather than a polygon that misses the ray.
fn delta() -> scene::DisplayTolerance {
    scene::DisplayTolerance::new(2.0e-4).expect("a positive delta")
}

/// A ray straight down onto the plate at `(x, y)`.
fn down_at(x: f64, y: f64) -> Ray {
    Ray {
        origin: Point3::new(x, y, 1.0),
        dir: Vec3::new(0.0, 0.0, -1.0),
    }
}

/// A pattern of `count` small blocks — the fixture for the rows that
/// need SEVERAL bodies under one node, and for the structural edit
/// that consumes one of them.
///
/// Answers the document, the extrude node and the pattern node.
fn patterned_blocks(tol: Tol, count: i64) -> (Doc<ProfileProgram>, RecipeNodeId, RecipeNodeId) {
    let doc: Doc<ProfileProgram> = Doc::empty_derived("gui2-pattern", tol);
    let (doc, profile) = common::inserted(&doc, common::square(0.02), tol);
    let (doc, extrude) = common::inserted(
        &doc,
        Node::Extrude {
            profile,
            distance: common::len(0.01),
        },
        tol,
    );
    let (doc, pattern) = common::inserted(
        &doc,
        Node::Pattern {
            input: extrude,
            count: Expr::count(count),
            kind: PatternKind::Linear {
                direction: [common::scl(1.0), common::scl(0.0), common::scl(0.0)],
                spacing: common::len(0.05),
            },
        },
        tol,
    );
    (doc, extrude, pattern)
}

// --- the un-projection --------------------------------------------

#[test]
fn a_projected_point_unprojects_to_the_ray_through_it() {
    let viewport = ViewportSize {
        width_px: 1280.0,
        height_px: 720.0,
    };
    let aspect = viewport.aspect().expect("a positive aspect");
    let camera = common::framed(aspect);
    let bounds = common::plate_bounds();
    for point in common::corners(&bounds) {
        let ndc = camera
            .project(point, aspect)
            .expect("the projection is defined")
            .expect("a framed corner is in front of the eye");
        // NDC → the pixel the cursor would be at, `+y` down.
        let cursor = [
            (ndc[0] + 1.0) * 0.5 * viewport.width_px,
            (1.0 - ndc[1]) * 0.5 * viewport.height_px,
        ];
        let ray = camera
            .ray_through(cursor, viewport)
            .expect("a cursor inside the viewport un-projects");
        let to_point = point - ray.origin;
        // The point lies on the ray: no perpendicular component, and
        // in front of the eye rather than behind it.
        let along = ray.dir.dot(to_point);
        assert!(along > 0.0, "the point is in front of the eye");
        let perpendicular = to_point - ray.dir * along;
        let residual = perpendicular.dot(perpendicular).sqrt();
        assert!(
            residual < 1.0e-12 * along.max(1.0),
            "the un-projected ray misses its own projected point by {residual}"
        );
    }
}

#[test]
fn the_pane_centre_unprojects_to_the_view_axis() {
    let viewport = ViewportSize {
        width_px: 800.0,
        height_px: 600.0,
    };
    let camera = common::framed(viewport.aspect().expect("a positive aspect"));
    let ray = camera
        .ray_through([400.0, 300.0], viewport)
        .expect("the centre un-projects");
    let forward = camera.forward();
    for (got, want) in [
        (ray.dir.x, forward.x),
        (ray.dir.y, forward.y),
        (ray.dir.z, forward.z),
    ] {
        assert!((got - want).abs() < 1.0e-12, "{got} vs {want}");
    }
}

#[test]
fn a_viewport_with_no_area_and_a_non_finite_cursor_are_both_refused() {
    let camera = common::framed(1.0);
    let empty = ViewportSize {
        width_px: 0.0,
        height_px: 600.0,
    };
    assert!(camera.ray_through([1.0, 1.0], empty).is_err());
    let viewport = ViewportSize {
        width_px: 800.0,
        height_px: 600.0,
    };
    assert!(camera.ray_through([f64::NAN, 1.0], viewport).is_err());
}

// --- the id mapping ------------------------------------------------

#[test]
fn every_id_round_trips_to_the_patch_it_names() {
    let tol = Tol::witness();
    let (session, _) = plate_session(tol);
    let index = index_of(&session);
    let ids = index.ids();
    assert!(ids.len() >= 3, "the plate draws at least three patches");
    for id in ids.ids() {
        let key = ids.key_of(id).expect("an assigned id names a patch");
        assert_eq!(ids.id_of(key), Some(id), "the round trip is the identity");
    }
}

#[test]
fn distinct_patches_never_share_an_id_across_bodies() {
    let tol = Tol::witness();
    let (doc, _extrude, pattern) = patterned_blocks(tol, 3);
    let mut session = DocSession::inline(doc, tol);
    session.pump();
    let index = index_of(&session);
    let ids = index.ids();

    let mut bodies = std::collections::BTreeSet::new();
    let mut seen_ids = std::collections::BTreeSet::new();
    for id in ids.ids() {
        let key = ids.key_of(id).expect("an assigned id names a patch");
        assert_eq!(key.node, pattern, "every drawn part belongs to the root");
        bodies.insert(key.body);
        assert!(seen_ids.insert(id), "ids are distinct");
    }
    assert_eq!(bodies.len(), 3, "three instances, three output bodies");
    // The collision the id map exists to make impossible: two patches
    // at the same position in DIFFERENT bodies must not answer to one
    // id.
    for body in &bodies {
        let key = PatchId {
            node: pattern,
            body: *body,
            patch: 0,
        };
        assert!(ids.id_of(key).is_some(), "patch 0 of body {body} is drawn");
    }
    let first: Vec<Option<u32>> = bodies
        .iter()
        .map(|body| {
            ids.id_of(PatchId {
                node: pattern,
                body: *body,
                patch: 0,
            })
        })
        .collect();
    let distinct: std::collections::BTreeSet<_> = first.iter().collect();
    assert_eq!(distinct.len(), first.len(), "one id per body's patch 0");
}

#[test]
fn nothing_is_reserved_and_a_repeated_patch_is_refused() {
    let key = PatchId {
        node: RecipeNodeId(1),
        body: 0,
        patch: 0,
    };
    let map = IdMap::build([key]).expect("one key assigns");
    assert_eq!(map.key_of(IdMap::NOTHING), None, "0 names no patch");
    assert_eq!(map.id_of(key), Some(1), "ids start at 1");
    assert!(
        IdMap::build([key, key]).is_err(),
        "a repeated patch is a pairing bug, not a deduplication"
    );
}

#[test]
fn re_indexing_one_generation_gives_the_same_ids() {
    let tol = Tol::witness();
    let (session, _) = plate_session(tol);
    let first = index_of(&session);
    let second = index_of(&session);
    assert_eq!(
        first.ids(),
        second.ids(),
        "the same evaluation at the same delta re-tessellates to the same id map"
    );
}

// --- the ray path --------------------------------------------------

#[test]
fn a_ray_down_the_hole_axis_is_a_typed_miss() {
    let tol = Tol::witness();
    let (session, _) = plate_session(tol);
    let index = index_of(&session);
    let [width, depth, _] = PLATE_EXTENT;
    let hit = index
        .pick(eval_of(&session), &down_at(width * 0.5, depth * 0.5))
        .expect("the hit test does not refuse");
    assert!(
        hit.is_none(),
        "the hole's axis meets no triangle — a typed miss, not an error"
    );
}

#[test]
fn a_ray_onto_the_top_face_names_it_and_the_name_resolves() {
    let tol = Tol::witness();
    let (session, extrude) = plate_session(tol);
    let index = index_of(&session);
    let hit = index
        .pick(eval_of(&session), &down_at(0.01, 0.01))
        .expect("the hit test does not refuse")
        .expect("a ray onto the plate hits it");
    assert_eq!(hit.node, extrude, "the plate's body is the extrude's");
    let (doc, eval) = session.landed_pair().expect("a landed pair");
    assert!(
        matches!(
            pncad::select::resolve(pncad::select::RunCtx { doc, eval }, &hit.name),
            Resolution::Resolved(_)
        ),
        "a just-picked name resolves in the run it was picked from"
    );
    let [_, _, thickness] = PLATE_EXTENT;
    assert!(
        (hit.point.z - thickness).abs() < 1.0e-9,
        "the nearest hit is the TOP face, not the bottom one"
    );
}

#[test]
fn two_cursors_on_one_face_agree_and_a_wall_is_a_different_face() {
    let tol = Tol::witness();
    let (session, _) = plate_session(tol);
    let index = index_of(&session);
    let eval = eval_of(&session);
    let a = index
        .pick(eval, &down_at(0.01, 0.01))
        .expect("no refusal")
        .expect("a hit");
    let b = index
        .pick(eval, &down_at(0.05, 0.03))
        .expect("no refusal")
        .expect("a hit");
    assert_eq!(a.name, b.name, "one planar cap face, two cursors");

    let [_, depth, thickness] = PLATE_EXTENT;
    let wall = Ray {
        origin: Point3::new(-1.0, depth * 0.5, thickness * 0.5),
        dir: Vec3::new(1.0, 0.0, 0.0),
    };
    let side = index
        .pick(eval, &wall)
        .expect("no refusal")
        .expect("a hit on the wall");
    assert_ne!(a.name, side.name, "a wall is not the cap");
}

#[test]
fn the_ray_paths_answer_is_the_id_maps_inverse() {
    let tol = Tol::witness();
    let (session, _) = plate_session(tol);
    let index = index_of(&session);
    let hit = index
        .pick(eval_of(&session), &down_at(0.01, 0.01))
        .expect("no refusal")
        .expect("a hit");
    // The id path, inverted: the name the RAY answered is drawn under
    // some id, and that id names a patch whose own name is the same.
    // This is the headless expression of "where both paths answer the
    // same query, they agree".
    let ids = index.ids_of(&hit.name);
    assert!(!ids.is_empty(), "the picked face is drawn under some id");
    for id in ids {
        let key = index
            .ids()
            .key_of(*id)
            .expect("a drawn id names a drawn patch");
        assert_eq!(key.node, hit.node);
        assert_eq!(key.body, hit.body);
        let name = index
            .name_of(*id)
            .expect("a drawn id has a name slot")
            .as_ref()
            .expect("the face is named");
        assert_eq!(*name, hit.name, "the id map inverts to the ray's answer");
    }
}

#[test]
fn the_id_passs_transform_samples_the_pixel_the_ray_was_cast_through() {
    // The GPU pass renders into a 1×1 target through
    // `cursor_projection`. If that transform is right, the world point
    // the ray path hit lands at the CENTRE of that target — which is
    // the geometric half of the two paths agreeing, checkable with no
    // GPU.
    let tol = Tol::witness();
    let (session, _) = plate_session(tol);
    let index = index_of(&session);
    let viewport = ViewportSize {
        width_px: 1024.0,
        height_px: 768.0,
    };
    let aspect = viewport.aspect().expect("a positive aspect");
    let camera = common::framed(aspect);
    // A cursor aimed at a point that is certainly on the plate's top
    // face: derived from the projection rather than guessed, so the row
    // tests the transform and not the framing.
    let ndc_of_target = camera
        .project(Point3::new(0.008, 0.006, PLATE_EXTENT[2]), aspect)
        .expect("defined")
        .expect("in front of the eye");
    let cursor = [
        (ndc_of_target[0] + 1.0) * 0.5 * viewport.width_px,
        (1.0 - ndc_of_target[1]) * 0.5 * viewport.height_px,
    ];
    let ray = camera
        .ray_through(cursor, viewport)
        .expect("the cursor un-projects");
    let hit = index
        .pick(eval_of(&session), &ray)
        .expect("no refusal")
        .expect("the cursor is aimed at a point on the plate");

    let matrix = camera
        .view_projection(aspect)
        .expect("the projection is defined");
    let vp = matrix.map(|column| column.map(|v| v as f32));
    let ndc = [
        (2.0 * cursor[0] / viewport.width_px - 1.0) as f32,
        (1.0 - 2.0 * cursor[1] / viewport.height_px) as f32,
    ];
    let sampled = cursor_projection(
        &vp,
        ndc,
        [viewport.width_px as f32, viewport.height_px as f32],
    );
    let clip = mul_point(&sampled, hit.point);
    assert!(clip[3] > 0.0, "the hit is in front of the eye");
    // Within half a target pixel of the centre: the target is one
    // pixel wide, so its whole extent is ±1 in this space.
    assert!(
        (clip[0] / clip[3]).abs() < 1.0 && (clip[1] / clip[3]).abs() < 1.0,
        "the hit point lands inside the sampled pixel: {clip:?}"
    );
}

/// A column-major matrix applied to a world point.
fn mul_point(m: &[[f32; 4]; 4], p: Point3<f64>) -> [f32; 4] {
    let v = [p.x as f32, p.y as f32, p.z as f32, 1.0];
    let mut out = [0.0f32; 4];
    for (row, slot) in out.iter_mut().enumerate() {
        *slot = m[0][row] * v[0] + m[1][row] * v[1] + m[2][row] * v[2] + m[3][row] * v[3];
    }
    out
}

// --- the selection-op fold -----------------------------------------

/// A viewport whose pixels are the cursor coordinates the rows below
/// use, and the camera framed on it.
fn viewport_and_camera() -> (ViewportSize, Camera) {
    let viewport = ViewportSize {
        width_px: 1024.0,
        height_px: 768.0,
    };
    let camera = common::framed(viewport.aspect().expect("a positive aspect"));
    (viewport, camera)
}

/// The cursor of the pane's centre — over the plate, and over its hole.
fn centre_cursor(viewport: ViewportSize) -> [f64; 2] {
    [viewport.width_px * 0.5, viewport.height_px * 0.5]
}

#[test]
fn the_primary_button_is_the_select_binding_and_moves_no_camera() {
    let map = InputMap::default();
    assert_eq!(map.select_button, PointerButton::Primary);
    let click = ViewportEvent::Click {
        button: PointerButton::Primary,
        pos_px: [10.0, 20.0],
    };
    let (viewport, camera) = viewport_and_camera();
    assert_eq!(map.map(&click, viewport, &camera), None, "no camera move");
    assert_eq!(map.pick(&click), Some(PickAction::Select([10.0, 20.0])));
    // A click on a button that is not the select binding picks nothing.
    let other = ViewportEvent::Click {
        button: PointerButton::Middle,
        pos_px: [10.0, 20.0],
    };
    assert_eq!(map.pick(&other), None);
}

#[test]
fn an_event_stream_selects_a_face_and_a_click_on_nothing_clears_it() {
    let tol = Tol::witness();
    let (mut session, extrude) = plate_session(tol);
    let index = index_of(&session);
    let (viewport, camera) = viewport_and_camera();
    let map = InputMap::default();

    // A cursor onto the plate's near corner region, found by projecting
    // a point that is on the body and off the hole.
    let aspect = viewport.aspect().expect("a positive aspect");
    let on_face = Point3::new(0.005, 0.005, PLATE_EXTENT[2]);
    let ndc = camera
        .project(on_face, aspect)
        .expect("defined")
        .expect("in front of the eye");
    let cursor = [
        (ndc[0] + 1.0) * 0.5 * viewport.width_px,
        (1.0 - ndc[1]) * 0.5 * viewport.height_px,
    ];
    // Far outside the pane: the ray leaves the frustum and meets
    // nothing, which is the "click on empty space" case.
    let empty = [-4000.0, -4000.0];

    let events = [
        ViewportEvent::Click {
            button: PointerButton::Primary,
            pos_px: cursor,
        },
        ViewportEvent::Hover { pos_px: cursor },
    ];
    let actions = input::pick_stream(&map, &events);
    assert_eq!(actions.len(), 2, "a click and a hover, in that order");
    for action in actions {
        let op = index
            .op_for(eval_of(&session), &camera, viewport, action)
            .expect("the cursor un-projects and the hit test does not refuse");
        session.perform(op);
    }
    let face = session
        .selection()
        .face()
        .expect("the click selected a face")
        .clone();
    assert_eq!(face.node, extrude);
    assert_eq!(
        session.hover().map(|h| h.name.clone()),
        Some(face.name.clone()),
        "the hover names the same face the click did"
    );

    let clear = index
        .op_for(
            eval_of(&session),
            &camera,
            viewport,
            PickAction::Select(empty),
        )
        .expect("an off-model cursor is a miss, not a refusal");
    session.perform(clear);
    assert_eq!(
        *session.selection(),
        Selection::None,
        "a click that hits nothing clears the selection"
    );
}

#[test]
fn hovering_never_touches_the_selection_and_leaving_clears_only_the_hover() {
    let tol = Tol::witness();
    let (mut session, _) = plate_session(tol);
    let index = index_of(&session);
    let (viewport, camera) = viewport_and_camera();

    let aspect = viewport.aspect().expect("a positive aspect");
    let ndc = camera
        .project(Point3::new(0.005, 0.005, PLATE_EXTENT[2]), aspect)
        .expect("defined")
        .expect("in front");
    let cursor = [
        (ndc[0] + 1.0) * 0.5 * viewport.width_px,
        (1.0 - ndc[1]) * 0.5 * viewport.height_px,
    ];
    let eval_ptr = eval_of(&session);
    let select = index
        .op_for(eval_ptr, &camera, viewport, PickAction::Select(cursor))
        .expect("no refusal");
    session.perform(select);
    let selected = session.selection().clone();
    assert!(matches!(selected, Selection::Face(_)));

    // Hover somewhere else, then leave.
    let elsewhere = index
        .op_for(
            eval_of(&session),
            &camera,
            viewport,
            PickAction::Hover([-4000.0, -4000.0]),
        )
        .expect("no refusal");
    session.perform(elsewhere);
    assert_eq!(session.hover(), None, "a hover onto nothing is no hover");
    assert_eq!(*session.selection(), selected, "the selection is untouched");

    let leave = index
        .op_for(eval_of(&session), &camera, viewport, PickAction::ClearHover)
        .expect("clearing the hover needs no ray");
    session.perform(leave);
    assert_eq!(session.hover(), None);
    assert_eq!(*session.selection(), selected);
}

#[test]
fn selecting_twice_keeps_exactly_one_selection() {
    let tol = Tol::witness();
    let (mut session, extrude) = plate_session(tol);
    let index = index_of(&session);
    let cap = index
        .face_at(eval_of(&session), &down_at(0.01, 0.01))
        .expect("no refusal")
        .expect("a hit");
    let [_, depth, thickness] = PLATE_EXTENT;
    let wall = index
        .face_at(
            eval_of(&session),
            &Ray {
                origin: Point3::new(-1.0, depth * 0.5, thickness * 0.5),
                dir: Vec3::new(1.0, 0.0, 0.0),
            },
        )
        .expect("no refusal")
        .expect("a hit");
    assert_ne!(cap.name, wall.name);

    session.perform(SessionOp::Select(Selection::Face(cap)));
    session.perform(SessionOp::Select(Selection::Face(wall.clone())));
    // Single-select: the second pick REPLACES the first. There is no
    // set here and nothing accumulates.
    assert_eq!(*session.selection(), Selection::Face(wall));
    assert_eq!(session.selection().node(), Some(extrude));
}

// --- one selection value, two panels -------------------------------

#[test]
fn a_face_pick_selects_the_owning_node_in_the_tree() {
    let tol = Tol::witness();
    let (mut session, extrude) = plate_session(tol);
    let index = index_of(&session);
    let face = index
        .face_at(eval_of(&session), &down_at(0.01, 0.01))
        .expect("no refusal")
        .expect("a hit");
    session.perform(SessionOp::Select(Selection::Face(face)));

    // What the feature tree highlights, and what the property panel
    // shows rows for, are both this one inversion.
    assert_eq!(session.selection().node(), Some(extrude));
    assert!(
        session.tree_rows().iter().any(|row| row.id == extrude),
        "the owning node is a row in the tree"
    );
    assert!(
        !session.slot_rows().is_empty(),
        "the property panel shows the owning feature's slots"
    );
}

#[test]
fn a_tree_selection_needs_no_viewport_pick() {
    let tol = Tol::witness();
    let (mut session, extrude) = plate_session(tol);
    session.perform(SessionOp::Select(Selection::Node(extrude)));
    assert_eq!(session.selection().node(), Some(extrude));
    assert_eq!(session.selection().face(), None, "no face is implied");
    assert!(!session.slot_rows().is_empty());
    assert!(session.standing().live());
}

#[test]
fn the_highlight_is_a_function_of_the_scene_and_the_selection() {
    let tol = Tol::witness();
    let (mut session, _) = plate_session(tol);
    let index = index_of(&session);
    let face = index
        .face_at(eval_of(&session), &down_at(0.01, 0.01))
        .expect("no refusal")
        .expect("a hit");

    let nothing = pick::highlight(&index, &Selection::None, None);
    assert_eq!(nothing.selected, IdMap::NOTHING);
    assert_eq!(nothing.hovered, IdMap::NOTHING);

    session.perform(SessionOp::Select(Selection::Face(face.clone())));
    session.perform(SessionOp::Hover(Some(face.clone())));
    let lit = pick::highlight(&index, session.selection(), session.hover());
    assert_ne!(lit.selected, IdMap::NOTHING, "the selected patch is marked");
    assert_eq!(lit.hovered, lit.selected, "the same patch is under both");
    assert_eq!(
        index.ids_of(&face.name).first().copied(),
        Some(lit.selected),
        "the mark is the id the name is drawn under"
    );

    // Recomputing from the same inputs gives the same answer, which is
    // what "pure function" means here and why no widget retains it.
    assert_eq!(
        lit,
        pick::highlight(&index, session.selection(), session.hover())
    );
}

// --- survival ------------------------------------------------------

#[test]
fn deleting_the_selected_feature_leaves_a_typed_unresolved_selection() {
    let tol = Tol::witness();
    let (doc, _extrude, pattern) = patterned_blocks(tol, 2);
    let mut session = DocSession::inline(doc, tol);
    session.pump();
    let index = index_of(&session);
    let face = index
        .face_at(eval_of(&session), &down_at(0.01, 0.01))
        .expect("no refusal")
        .expect("the first block is under this ray");
    assert_eq!(face.node, pattern);
    session.perform(SessionOp::Select(Selection::Face(face.clone())));
    assert!(session.standing().live());

    let outcome = session.perform(SessionOp::DeleteNode { node: pattern });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    session.pump();

    // No crash, no silent clear: the selection is still the name that
    // was picked, and its standing says it denotes nothing.
    assert_eq!(session.selection().face(), Some(&face));
    let standing = session.standing();
    assert!(!standing.live(), "a deleted feature's face is not live");
    assert!(
        standing.unresolved().is_some(),
        "the failure is a typed verdict, not an absence: {standing:?}"
    );
    assert!(
        session.slot_rows().is_empty(),
        "dependent affordances are off"
    );
}

#[test]
fn a_structural_edit_that_consumes_the_selected_face_leaves_it_unresolved() {
    let tol = Tol::witness();
    let (doc, _extrude, pattern) = patterned_blocks(tol, 3);
    let mut session = DocSession::inline(doc, tol);
    session.pump();
    let index = index_of(&session);
    // The third instance sits two spacings along +x.
    let face = index
        .face_at(eval_of(&session), &down_at(0.11, 0.01))
        .expect("no refusal")
        .expect("the third block is under this ray");
    assert_eq!(face.body, 2, "the third instance is output body 2");
    session.perform(SessionOp::Select(Selection::Face(face.clone())));
    assert!(session.standing().live());

    // The parameter edit that consumes it: two instances, no third.
    let outcome = session.perform(SessionOp::SetSlot {
        node: pattern,
        slot: SlotId::Count,
        value: SlotValue::Count(2),
    });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    session.pump();

    assert_eq!(session.selection().face(), Some(&face));
    let standing = session.standing();
    assert!(
        !standing.live(),
        "the consumed face is not live: {standing:?}"
    );
    assert!(standing.unresolved().is_some());
}

#[test]
fn undo_across_the_selections_birth_leaves_it_unresolved() {
    let tol = Tol::witness();
    let (doc, _extrude, pattern) = patterned_blocks(tol, 2);
    let mut session = DocSession::inline(doc, tol);
    session.pump();

    // Grow the pattern, then pick a face that exists only because of
    // that edit — the selection is BORN here.
    session.perform(SessionOp::SetSlot {
        node: pattern,
        slot: SlotId::Count,
        value: SlotValue::Count(3),
    });
    session.pump();
    let index = index_of(&session);
    let face = index
        .face_at(eval_of(&session), &down_at(0.11, 0.01))
        .expect("no refusal")
        .expect("the third block exists now");
    session.perform(SessionOp::Select(Selection::Face(face.clone())));
    assert!(session.standing().live());

    session.perform(SessionOp::Undo);
    session.pump();
    assert_eq!(session.selection().face(), Some(&face));
    let standing = session.standing();
    assert!(
        !standing.live(),
        "undoing past the selection's birth un-resolves it: {standing:?}"
    );
    assert!(standing.unresolved().is_some());

    // And redoing brings it back: the selection was never destroyed,
    // so nothing has to be re-picked.
    session.perform(SessionOp::Redo);
    session.pump();
    assert!(
        session.standing().live(),
        "the name resolves again once its referent is back"
    );
}

#[test]
fn a_selection_with_no_evaluation_behind_it_is_not_reported_as_live() {
    let tol = Tol::witness();
    let (mut session, extrude) = plate_session(tol);
    // A name that never came from a pick: the standing machinery must
    // answer, not panic, and "cannot tell" is not "yes".
    session.perform(SessionOp::Select(Selection::Node(extrude)));
    assert!(matches!(session.standing(), Standing::Node { .. }));
    assert!(session.standing().live());
    let deleted = session.perform(SessionOp::DeleteNode { node: extrude });
    assert!(deleted.refusal.is_none(), "{:?}", deleted.refusal);
    assert!(
        !session.standing().live(),
        "a node selection survives the node's deletion as a dead standing"
    );
}

// --- generation invalidation ---------------------------------------

#[test]
fn a_pick_index_from_an_older_generation_is_not_current() {
    let tol = Tol::witness();
    let (doc, _extrude, pattern) = patterned_blocks(tol, 2);
    let mut session = DocSession::inline(doc, tol);
    session.pump();
    let index = index_of(&session);
    assert!(
        index.current_for(session.landed_generation(), delta()),
        "freshly built, it describes the run on screen"
    );

    session.perform(SessionOp::SetSlot {
        node: pattern,
        slot: SlotId::Count,
        value: SlotValue::Count(3),
    });
    session.pump();
    assert!(
        !index.current_for(session.landed_generation(), delta()),
        "a re-evaluation invalidates the index — it is DISCARDED, not repaired"
    );
    // A different display tolerance invalidates it too: the parts are
    // the tessellations the picture is drawn from.
    let coarser = delta().scaled(2.0).expect("a positive delta");
    let rebuilt = index_of(&session);
    assert!(rebuilt.current_for(session.landed_generation(), delta()));
    assert!(!rebuilt.current_for(session.landed_generation(), coarser));
}

#[test]
fn the_index_and_the_picture_are_one_tessellation() {
    let tol = Tol::witness();
    let (session, _) = plate_session(tol);
    let index = index_of(&session);
    let scene = index.scene().expect("the index draws");
    let triangles: usize = index
        .parts()
        .iter()
        .map(|part| {
            part.mesh()
                .patches
                .iter()
                .map(|patch| patch.triangles.len())
                .sum::<usize>()
        })
        .sum();
    assert_eq!(
        scene.positions().len(),
        triangles * 3,
        "the drawn corners are the indexed triangles' corners"
    );
    assert_eq!(scene.ids().len(), scene.positions().len());
    // Every drawn corner carries the id of a patch this index holds.
    for id in scene.ids() {
        assert!(
            index.ids().key_of(*id).is_some(),
            "a drawn corner's id names a patch"
        );
    }
    assert_eq!(
        scene.stats().faces,
        index.ids().len(),
        "one id per drawn patch"
    );
}

#[test]
fn a_product_scene_carries_no_ids_and_is_therefore_unpickable() {
    // The startup scene, drawn from the gathered product before the
    // first evaluation lands: its patches belong to the aggregate and
    // to no node, so nothing in it is addressable.
    let tol = Tol::witness();
    let (doc, _) = scene::plate_with_hole(tol).expect("the plate authors");
    let scene = scene::scene_of(&doc, delta(), tol).expect("the plate tessellates");
    assert!(!scene.ids().is_empty());
    assert!(
        scene.ids().iter().all(|id| *id == IdMap::NOTHING),
        "an unowned part draws under no id"
    );
}
