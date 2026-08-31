//! **The frame loop's decisions, replayed.**
//!
//! Three rules used to live inside `app`-gated code that no test can
//! execute: when a batch clears the status line, when the id pass is
//! asked, and when the two picking paths are reported as disagreeing.
//! A fourth — the rebuild-on-stale loop — was written at its one
//! consumer and retried forever when a build refused. Each is a value
//! now, and each is driven here the way the frame loop drives it.
//!
//! The rows that could not exist before are the point: every one of
//! these is a rule whose failure is silent in a running application.

// Panicking is a test's failure mechanism (workspace lint note).
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

mod common;

use pncad::document::{Doc, Node, ParamName, ProfileProgram, RecipeNodeId, SlotId};
use pncad::geom_core::{Point3, Tol, Vec3};
use pncad::select::Ray;
use viewer::camera::{Camera, CameraOp};
use viewer::display::DisplayView;
use viewer::evalseam::Generation;
use viewer::frame::{self, IdQueryLog, IdStep, StatusUpdate};
use viewer::input::{self, InputMap, ViewportSize};
use viewer::pick::{CacheStep, IdMap, PickCache, PickIndex};
use viewer::props::SlotValue;
use viewer::scene::{self, DisplayTolerance, PLATE_EXTENT};
use viewer::session::{DocSession, FaceSelection, Hovered, Refusal, Selection, SessionOp};

fn delta() -> DisplayTolerance {
    DisplayTolerance::new(2.0e-4).expect("a positive delta")
}

fn plate_session(tol: Tol) -> (DocSession, RecipeNodeId) {
    let (doc, extrude) = scene::plate_with_hole(tol).expect("the plate authors");
    let mut session = DocSession::inline(doc, tol);
    session.pump();
    (session, extrude)
}

fn down_at(x: f64, y: f64) -> Ray {
    Ray {
        origin: Point3::new(x, y, 1.0),
        dir: Vec3::new(0.0, 0.0, -1.0),
    }
}

fn index_of(session: &DocSession) -> PickIndex {
    let (doc, eval) = session.landed_pair().expect("a landed pair");
    PickIndex::build(
        doc,
        eval,
        session.landed_generation().expect("a generation"),
        delta(),
        session.tol(),
    )
    .expect("the plate indexes")
}

// --- the status-line policy ----------------------------------------

#[test]
fn a_hover_only_batch_leaves_the_status_line_alone() {
    // The defect this rule exists for: the pointer drifting over the
    // viewport wiped the ratified expression-driven affordance, because
    // a hover makes the batch non-empty on every frame it changes what
    // the cursor is over.
    let hover_only = [SessionOp::Hover(None)];
    assert_eq!(
        frame::batch_status(&hover_only, None),
        StatusUpdate::Keep,
        "a hover is not an action on the document"
    );
}

#[test]
fn a_clean_action_clears_and_a_refusal_shows_even_from_a_hover_batch() {
    let acted = [SessionOp::Select(Selection::None)];
    assert_eq!(frame::batch_status(&acted, None), StatusUpdate::Clear);

    // A refusal always reaches the line, whatever the batch was: a
    // hover cannot refuse today, and silence would be the wrong answer
    // if one ever did.
    let refusal = viewer::session::Refusal::NothingToDo;
    let shown = frame::batch_status(&[SessionOp::Hover(None)], Some(&refusal));
    assert_eq!(shown, StatusUpdate::Show(refusal.to_string()));
    assert!(!refusal.to_string().is_empty());
}

#[test]
fn the_chooser_probe_is_confident_only_with_neither_backend_reading() {
    // The first-light defect (#1097, a WSL distro): with no portal
    // service and no zenity, `rfd` returns the same bare `None` a
    // cancel does, and the app dropped it — Open/Save As "silently do
    // nothing". The probe's decision logic is a pure function of the
    // two readings, so these rows hold whatever is on the CI box's
    // PATH.
    use frame::ChooserBackend;
    assert_eq!(
        frame::chooser_backend_of(true, false),
        ChooserBackend::ZenityPresent
    );
    assert_eq!(
        frame::chooser_backend_of(true, true),
        ChooserBackend::ZenityPresent,
        "zenity needs no portal"
    );
    assert_eq!(
        frame::chooser_backend_of(false, true),
        ChooserBackend::PortalPossible,
        "a session bus makes a portal POSSIBLE — a hint, never a verdict"
    );
    assert_eq!(
        frame::chooser_backend_of(false, false),
        ChooserBackend::Absent
    );
    assert!(ChooserBackend::ZenityPresent.usable());
    assert!(ChooserBackend::PortalPossible.usable());
    assert!(
        !ChooserBackend::Absent.usable(),
        "the one arm the chrome disables the dialogs over"
    );
}

#[test]
fn an_empty_dialog_is_loud_only_under_a_confidently_absent_backend() {
    use frame::ChooserBackend;
    // Confident absence: the loud arm, naming the remedy and the
    // dialog-free workaround. (The chrome disables the controls before
    // any click can reach this; the policy stays honest regardless.)
    let StatusUpdate::Show(message) = frame::dialog_status(ChooserBackend::Absent, false) else {
        panic!("an empty-handed dialog with no backend reaches the status line");
    };
    assert_eq!(message, frame::NO_CHOOSER_BACKEND);
    assert!(message.contains("zenity"));
    assert!(message.contains("xdg-desktop-portal"));
    assert!(message.contains("command line"));
    // A plausibly-present backend reads `None` as a genuine cancel,
    // which should not nag.
    for backend in [
        ChooserBackend::ZenityPresent,
        ChooserBackend::PortalPossible,
    ] {
        assert_eq!(frame::dialog_status(backend, false), StatusUpdate::Keep);
    }
    // A chosen path is never this policy's business: the Open/Save
    // batch it feeds owns the line through `batch_status`.
    for backend in [
        ChooserBackend::ZenityPresent,
        ChooserBackend::PortalPossible,
        ChooserBackend::Absent,
    ] {
        assert_eq!(frame::dialog_status(backend, true), StatusUpdate::Keep);
    }
}

#[test]
fn an_empty_batch_and_a_pure_cursor_stream_move_no_camera() {
    // The other half of the same defect: the event stream now carries
    // cursor events, so "the stream was non-empty" stopped meaning "the
    // camera moved" — and landing a fold that moved nothing cleared the
    // status line on every hovering frame.
    let camera = common::framed(4.0 / 3.0);
    let viewport = ViewportSize {
        width_px: 800.0,
        height_px: 600.0,
    };
    let map = InputMap::default();
    let cursor_only = [
        input::ViewportEvent::Hover {
            pos_px: [10.0, 20.0],
        },
        input::ViewportEvent::Leave,
    ];
    let folded = input::fold_events(&map, &camera, viewport, &cursor_only);
    assert!(
        !frame::folded_moved(&folded),
        "cursor events denote no camera operation"
    );

    let orbit = [input::ViewportEvent::Drag {
        button: input::PointerButton::Middle,
        shift: false,
        alt: false,
        delta_px: [12.0, 0.0],
    }];
    let moved = input::fold_events(&map, &camera, viewport, &orbit);
    assert!(frame::folded_moved(&moved));
    assert!(matches!(
        moved.applied.first(),
        Some(CameraOp::Orbit { .. })
    ));
}

// --- the id query's bookkeeping ------------------------------------

#[test]
fn the_id_query_is_asked_once_per_cursor_and_re_asked_when_the_picture_moves() {
    let mut log = IdQueryLog::new();
    let generation = Some(Generation::FIRST);
    let IdStep::Ask { serial: first } = log.step(Some([10.0, 20.0]), generation) else {
        panic!("the first look at a cursor asks");
    };
    assert_eq!(log.outstanding(), Some(first));
    assert_eq!(
        log.step(Some([10.0, 20.0]), generation),
        IdStep::Hold,
        "a still cursor over an unchanged picture asks nothing"
    );
    let IdStep::Ask { serial: moved } = log.step(Some([11.0, 20.0]), generation) else {
        panic!("a moved cursor asks again");
    };
    assert_ne!(moved, first);
    // The picture changing under a STILL cursor is also a new question:
    // the answer is about what is drawn, not only about the pointer.
    let IdStep::Ask { serial: repainted } =
        log.step(Some([11.0, 20.0]), Some(Generation::FIRST.next()))
    else {
        panic!("a new generation re-asks");
    };
    assert_ne!(repainted, moved);
}

#[test]
fn leaving_the_pane_voids_the_outstanding_answer() {
    // The defect: leaving issued no query, reset no serial, and left
    // the last answer matched — so the comparison fired every frame
    // against a hover that had been cleared, printing the exact message
    // issue #1097 §4 tells the operator to read as a clear-value fault.
    let mut log = IdQueryLog::new();
    let generation = Some(Generation::FIRST);
    assert!(matches!(
        log.step(Some([10.0, 20.0]), generation),
        IdStep::Ask { .. }
    ));
    assert!(log.outstanding().is_some());
    assert_eq!(log.step(None, generation), IdStep::Void);
    assert_eq!(
        log.outstanding(),
        None,
        "nothing is outstanding once the pointer is gone"
    );
    // And coming back asks fresh rather than reusing the void answer.
    assert!(matches!(
        log.step(Some([10.0, 20.0]), generation),
        IdStep::Ask { .. }
    ));
}

// --- the two paths' agreement, by name ------------------------------

/// The channel word the GPU side writes.
fn answer(serial: u32, id: u32) -> u64 {
    u64::from(serial) << 32 | u64::from(id)
}

#[test]
fn the_agreement_check_compares_names_and_ignores_answers_nobody_asked_for() {
    let tol = Tol::witness();
    let (session, _) = plate_session(tol);
    let index = index_of(&session);
    let hit = index
        .pick(session.evaluation().expect("landed"), &down_at(0.01, 0.01))
        .expect("no refusal")
        .expect("a hit");
    let id = *index
        .ids_of(&hit.name)
        .first()
        .expect("the picked face is drawn");

    // Agreement: same face, no verdict.
    assert_eq!(
        frame::disagreement(&index, answer(7, id), Some(7), Some(&hit.name)),
        None
    );
    // A stale answer is not a verdict at all — nor is one with nothing
    // outstanding, which is the leave case.
    assert_eq!(
        frame::disagreement(&index, answer(6, id), Some(7), None),
        None
    );
    assert_eq!(frame::disagreement(&index, answer(7, id), None, None), None);
    // Nothing under the cursor on both sides is agreement.
    assert_eq!(
        frame::disagreement(&index, answer(7, IdMap::NOTHING), Some(7), None),
        None
    );
    // A real disagreement reports both sides.
    let report = frame::disagreement(&index, answer(7, IdMap::NOTHING), Some(7), Some(&hit.name))
        .expect("nothing vs a face is a disagreement");
    assert_eq!(report.from_gpu, None);
    assert_eq!(report.from_ray.as_ref(), Some(&hit.name));
    assert!(report.to_string().contains("disagree"));
}

/// **The diagnostic's subject is the PATCH under the cursor, and the
/// hover stopped being that answer when edges became pickable.**
///
/// The id buffer can only ever name a patch. With the cursor inside
/// the edge radius the hover names an EDGE, so feeding the hover
/// compares two different questions and reports a disagreement on
/// every such frame — which is what the first half below pins, as the
/// defect rather than as the behaviour. The second half is the fix:
/// `face_under_cursor` answers the id buffer's own question, and there
/// is no verdict.
#[test]
fn an_edge_hover_is_not_a_disagreement_because_the_face_is_what_is_compared() {
    let tol = Tol::witness();
    let (session, extrude) = plate_session(tol);
    let index = index_of(&session);
    let eval = session.evaluation().expect("landed");
    let pane = ViewportSize {
        width_px: 1280.0,
        height_px: 720.0,
    };
    let aspect = pane.aspect().expect("a positive aspect");
    let camera = common::framed(aspect);

    // A cursor ON a drawn edge of the plate: the priority rule answers
    // with the edge, and the GPU would answer with the patch behind it.
    let (cursor, edge) = index
        .edges_in(extrude, 0)
        .iter()
        .find_map(|&id| {
            let points = index.edge_polyline_for(id, &DisplayView::none());
            if points.len() < 2 {
                return None;
            }
            let at = (points.len() - 1) / 2;
            let mid = Point3::new(
                (points[at].x + points[at + 1].x) * 0.5,
                (points[at].y + points[at + 1].y) * 0.5,
                (points[at].z + points[at + 1].z) * 0.5,
            );
            let ndc = camera.project(mid, aspect).ok()??;
            let cursor = pane.cursor_of([ndc[0], ndc[1]])?;
            let hovered = index.hovered_at(eval, &camera, pane, cursor).ok()?;
            match hovered {
                Some(Hovered::Edge(edge)) => Some((cursor, edge)),
                _ => None,
            }
        })
        .expect("some drawn edge of the plate is hoverable from its own midpoint");

    let face = index
        .face_under_cursor(eval, &camera, pane, cursor, &DisplayView::none())
        .expect("the cursor un-projects")
        .expect("an edge is only reachable where its body is, so a face is under it too");
    let id = *index
        .ids_of_target(&face)
        .first()
        .expect("the face under the cursor is drawn");

    // The defect, pinned: the hover's name against the patch's.
    assert!(
        frame::disagreement(&index, answer(7, id), Some(7), Some(&edge.name)).is_some(),
        "an edge name against a patch name is two questions, and the check cannot know it"
    );
    // The fix: the ray side answers the question the id buffer asked.
    assert_eq!(
        frame::disagreement(&index, answer(7, id), Some(7), Some(&face.name)),
        None,
        "the face under the cursor is what the id buffer named"
    );
}

#[test]
fn one_name_drawn_twice_is_not_a_disagreement() {
    // The false positive the id comparison used to produce: the same
    // face drawn under two roots has two ids, and comparing ids called
    // that a fault. Names are the currency the property is about.
    let tol = Tol::witness();
    let (doc, _left, right) = two_placements(tol);
    let mut session = DocSession::inline(doc, tol);
    session.pump();
    let index = index_of(&session);
    let hit = index
        .pick(session.evaluation().expect("landed"), &down_at(0.115, 0.01))
        .expect("no refusal")
        .expect("the right placement is hit");
    assert_eq!(hit.node, right);
    let ids = index.ids_of(&hit.name);
    assert!(
        ids.len() > 1,
        "the fixture must draw one name twice, got {ids:?}"
    );
    // The GPU answers the OTHER copy's id; both name the same face.
    let other = ids
        .iter()
        .copied()
        .find(|id| !index.ids_of_target(&face_of(&hit)).contains(id))
        .expect("a second occurrence");
    assert_eq!(
        frame::disagreement(&index, answer(3, other), Some(3), Some(&hit.name)),
        None,
        "two ids of one name are the same answer"
    );
}

fn face_of(hit: &pncad::select::PickHit) -> FaceSelection {
    FaceSelection {
        name: hit.name.clone(),
        node: hit.node,
        body: hit.body,
    }
}

/// Two `Transform` roots over one extrude: both drawn copies carry the
/// extrude's names, because a transform contributes no role segment.
fn two_placements(tol: Tol) -> (Doc<ProfileProgram>, RecipeNodeId, RecipeNodeId) {
    let doc: Doc<ProfileProgram> = Doc::empty_derived("frame-two-placements", tol);
    let (doc, profile) = common::inserted(&doc, common::square(0.02), tol);
    let (doc, extrude) = common::inserted(
        &doc,
        Node::Extrude {
            profile,
            distance: common::len(0.01),
        },
        tol,
    );
    let place = |doc: &Doc<ProfileProgram>, x: f64| {
        common::inserted(
            doc,
            Node::Transform {
                input: extrude,
                translation: [common::len(x), common::len(0.0), common::len(0.0)],
                rotation_axis: [common::scl(0.0), common::scl(0.0), common::scl(1.0)],
                rotation_angle: pncad::document::Expr::literal(
                    0.0,
                    pncad::document::Dimension::Angle,
                )
                .expect("a finite angle"),
            },
            tol,
        )
    };
    let (doc, left) = place(&doc, 0.0);
    let (doc, right) = place(&doc, 0.1);
    (doc, left, right)
}

// --- the highlight, scoped ------------------------------------------

#[test]
fn the_highlight_narrows_a_twice_drawn_name_to_exactly_one_id() {
    let tol = Tol::witness();
    let (doc, left, right) = two_placements(tol);
    let mut session = DocSession::inline(doc, tol);
    session.pump();
    let index = index_of(&session);
    let hit = index
        .pick(session.evaluation().expect("landed"), &down_at(0.115, 0.01))
        .expect("no refusal")
        .expect("a hit on the right placement");
    let face = face_of(&hit);
    // The narrowing is to ONE id, not merely to a smaller set: a node's
    // name table is a bijection, so a name denotes at most one entity of
    // one body. A wider answer here would be a naming-emission bug, and
    // this row is where it would show.
    assert_eq!(
        index.ids_of_target(&face).len(),
        1,
        "the name table's bijection makes the scoped answer unique"
    );
    assert!(
        index.ids_of(&face.name).len() > 1,
        "the name is drawn twice"
    );
    let marked = viewer::pick::highlight(&index, &Selection::Face(face.clone()), None);
    let key = index
        .ids()
        .key_of(marked.selected)
        .expect("the mark names a patch");
    assert_eq!((key.node, key.body), (right, hit.body));
    assert_ne!(key.node, left);

    // And every id in the right placement belongs to it.
    for id in index.ids_in(right, hit.body) {
        let key = index.ids().key_of(*id).expect("a drawn id");
        assert_eq!(key.node, right);
    }
    assert!(!index.ids_in(right, hit.body).is_empty());
    assert!(index.ids_in(RecipeNodeId(9999), 0).is_empty());
}

// --- the rebuild loop -----------------------------------------------

#[test]
fn a_refused_index_is_attempted_once_per_generation_and_not_once_per_frame() {
    // The defect: a failed or poisoned root is an ordinary editing
    // state, and the app's guard stayed false forever afterwards — so
    // every repainted frame re-tessellated every healthy root before
    // reaching the failing one, behind a picture already stale.
    let tol = Tol::witness();
    let (doc, extrude) = scene::plate_with_hole(tol).expect("the plate authors");
    let mut session = DocSession::inline(doc, tol);
    session.pump();
    let mut cache = PickCache::new();
    assert_eq!(cache.sync(&session, delta()), CacheStep::Rebuilt);
    assert_eq!(cache.sync(&session, delta()), CacheStep::Current);
    assert!(cache.index().is_some());
    assert!(cache.error().is_none());

    // Break the document so the root refuses to evaluate.
    let outcome = session.perform(SessionOp::SetSlot {
        node: extrude,
        slot: SlotId::Distance,
        value: SlotValue::Continuous(0.0),
    });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    session.pump();

    assert_eq!(
        cache.sync(&session, delta()),
        CacheStep::Refused,
        "a failed root refuses the index"
    );
    assert!(cache.error().is_some(), "the refusal is readable");
    // The frame after, and the frame after that: HELD. This is the
    // whole row — before the fix, both of these were another full
    // rebuild attempt.
    assert_eq!(
        cache.sync(&session, delta()),
        CacheStep::Held,
        "a refused build is not retried on the next frame"
    );
    assert_eq!(cache.sync(&session, delta()), CacheStep::Held);

    // The same state is what the gather refuses on, and the tree badges
    // the node — two channels, both saying something true.
    assert!(
        session.product_fault().is_some(),
        "a failed root is a gather refusal too"
    );
    assert!(
        session
            .tree_rows()
            .iter()
            .any(|row| matches!(row.status, viewer::tree::RowStatus::Failed { .. })),
        "and the node itself badges"
    );
}

#[test]
fn a_new_generation_or_a_new_delta_earns_one_fresh_attempt() {
    let tol = Tol::witness();
    let (mut session, extrude) = plate_session(tol);
    let mut cache = PickCache::new();
    assert_eq!(cache.sync(&session, delta()), CacheStep::Rebuilt);
    // δ is part of the key: the parts are the tessellations the picture
    // is drawn from.
    let coarser = delta().scaled(2.0).expect("a positive delta");
    assert_eq!(cache.sync(&session, coarser), CacheStep::Rebuilt);
    assert_eq!(cache.sync(&session, coarser), CacheStep::Current);

    session.perform(SessionOp::SetSlot {
        node: extrude,
        slot: SlotId::Distance,
        value: SlotValue::Continuous(PLATE_EXTENT[2] * 1.5),
    });
    session.pump();
    assert_eq!(cache.sync(&session, coarser), CacheStep::Rebuilt);
}

#[test]
fn a_cache_with_nothing_landed_has_nothing_to_do() {
    let tol = Tol::witness();
    let (doc, _) = scene::plate_with_hole(tol).expect("the plate authors");
    // No `pump`, so nothing has landed.
    let session = DocSession::inline(doc, tol);
    let mut cache = PickCache::new();
    assert_eq!(cache.sync(&session, delta()), CacheStep::Nothing);
    assert!(cache.index().is_none());
}

// --- the pairing sweep ----------------------------------------------

#[test]
fn the_feature_tree_reads_the_landed_pair_not_the_shown_document() {
    // The sibling the landed-pair fix missed: a row's badge is a
    // statement about what a run said, so reading it off a document
    // that run never saw describes a run that never happened.
    let tol = Tol::witness();
    let (doc, extrude) = scene::plate_with_hole(tol).expect("the plate authors");
    let mut session = DocSession::inline(doc, tol);
    session.pump();
    let before = session.tree_rows().len();
    assert!(before >= 2);

    // Hold the seam: submit an edit and do NOT pump, so the document
    // has moved past the landed evaluation.
    session.perform(SessionOp::DeleteNode { node: extrude });
    assert!(session.busy(), "a run is outstanding");
    let rows = session.tree_rows();
    let (landed_doc, _) = session.landed_pair().expect("a landed pair");
    assert_eq!(
        rows.len(),
        landed_doc.order().len(),
        "the tree describes the landed document, which is what is drawn"
    );
    session.pump();
    assert_eq!(
        session.tree_rows().len(),
        before - 1,
        "and then it catches up"
    );
}

#[test]
fn opening_a_document_drops_the_previous_ones_landed_run() {
    let tol = Tol::witness();
    let (session, _) = plate_session(tol);
    assert!(session.landed_pair().is_some());

    let dir = std::env::temp_dir().join("gui2-frame-open");
    std::fs::create_dir_all(&dir).expect("a scratch dir");
    let path = dir.join("plate.pncad");
    let mut session = session;
    let saved = session.perform(SessionOp::Save(path.clone()));
    assert!(saved.refusal.is_none(), "{:?}", saved.refusal);

    let opened = session.perform(SessionOp::Open(path.clone()));
    assert!(opened.refusal.is_none(), "{:?}", opened.refusal);
    // Before the new run lands there is NO pair: the old one answered
    // the previous document and would have rendered its tree and
    // resolved the new document's names against it.
    assert!(
        session.landed_pair().is_none(),
        "the previous document's run does not describe this one"
    );
    assert!(session.product_fault().is_none());
    assert!(
        session
            .tree_rows()
            .iter()
            .all(|row| matches!(row.status, viewer::tree::RowStatus::Unevaluated))
    );
    session.pump();
    assert!(session.landed_pair().is_some(), "and the fresh run lands");
    let _ = std::fs::remove_file(&path);
}

#[test]
fn a_well_formed_product_reports_no_fault_and_the_verdict_is_computed_once() {
    // The gather-level verdict no per-node badge can carry. Nothing in
    // the gallery refuses, so the row asserts the honest half: the
    // verdict exists, is `None` for a good document, and is `None`
    // before anything lands (which is not the same as "well formed").
    let tol = Tol::witness();
    let (doc, _) = scene::plate_with_hole(tol).expect("the plate authors");
    let mut session = DocSession::inline(doc, tol);
    assert!(session.landed_pair().is_none());
    assert!(session.product_fault().is_none());
    session.pump();
    assert!(session.landed_pair().is_some());
    assert!(
        session.product_fault().is_none(),
        "the plate's product gathers: {:?}",
        session.product_fault()
    );
}

// --- the conversion's one home --------------------------------------

#[test]
fn the_pixel_to_ndc_conversion_round_trips_and_flips_y_once() {
    let viewport = ViewportSize {
        width_px: 800.0,
        height_px: 600.0,
    };
    // The corners, named: top-left is (−1, +1) because pixels count
    // down and normalized device coordinates count up.
    assert_eq!(viewport.ndc_of([0.0, 0.0]), Some([-1.0, 1.0]));
    assert_eq!(viewport.ndc_of([800.0, 600.0]), Some([1.0, -1.0]));
    assert_eq!(viewport.ndc_of([400.0, 300.0]), Some([0.0, 0.0]));
    for px in [[0.0, 0.0], [123.0, 45.0], [800.0, 600.0]] {
        let ndc = viewport.ndc_of(px).expect("a positive area");
        let back = viewport.cursor_of(ndc).expect("a positive area");
        assert!((back[0] - px[0]).abs() < 1e-9 && (back[1] - px[1]).abs() < 1e-9);
    }
    let empty = ViewportSize {
        width_px: 0.0,
        height_px: 600.0,
    };
    assert_eq!(empty.ndc_of([1.0, 1.0]), None);
    assert_eq!(empty.cursor_of([0.0, 0.0]), None);

    // And the camera's un-projection agrees with it, because it goes
    // through it: the pane centre is the view axis.
    let camera: Camera = common::framed(4.0 / 3.0);
    let ray = camera
        .ray_through([400.0, 300.0], viewport)
        .expect("the centre un-projects");
    let forward = camera.forward();
    assert!((ray.dir.x - forward.x).abs() < 1e-12);
    assert!((ray.dir.y - forward.y).abs() < 1e-12);
    assert!((ray.dir.z - forward.z).abs() < 1e-12);
}

// --- the refuse-then-offer pair for an unknown parameter ------------

/// **The unknown-parameter refusal carries its offer, and the frame
/// policies hand both to the chrome.** An expression naming an
/// undeclared parameter refuses at the parse door (typo-safety — text
/// never creates a parameter); `creation_offer` extracts the name to
/// prefill the add-parameter affordance, and `retype_draft` hands the
/// refused text back so acting on the offer does not cost the very
/// expression that raised it.
#[test]
fn an_unknown_parameter_refusal_offers_creation_and_returns_the_draft() {
    let tol = Tol::witness();
    let doc: Doc<ProfileProgram> = Doc::empty_derived("frame-offer", tol);
    let (doc, profile) = common::inserted(&doc, common::square(0.04), tol);
    let (doc, extrude) = common::inserted(
        &doc,
        Node::Extrude {
            profile,
            distance: common::len(0.008),
        },
        tol,
    );
    let mut session = DocSession::inline(doc, tol);

    // The frame loop's exact shape: perform the batch, collect the
    // preferred refusal, then ask the policies.
    let batch = vec![SessionOp::SetSlotExpression {
        node: extrude,
        slot: SlotId::Distance,
        text: "margin * 2.0".to_owned(),
    }];
    let mut refusal = None;
    for op in batch.clone() {
        if let Some(next) = session.perform(op).refusal {
            refusal = Refusal::preferred(refusal, next);
        }
    }
    assert_eq!(
        frame::creation_offer(refusal.as_ref()),
        Some(ParamName::new("margin")),
        "the offer is the undeclared name"
    );
    assert_eq!(
        frame::retype_draft(&batch, refusal.as_ref()),
        Some((extrude, SlotId::Distance, "margin * 2.0".to_owned())),
        "and the refused draft comes back"
    );

    // A parse refusal that names NO parameter restores the draft but
    // offers nothing to create.
    let batch = vec![SessionOp::SetSlotExpression {
        node: extrude,
        slot: SlotId::Distance,
        text: "0.008 *".to_owned(),
    }];
    let mut refusal = None;
    for op in batch.clone() {
        if let Some(next) = session.perform(op).refusal {
            refusal = Refusal::preferred(refusal, next);
        }
    }
    assert!(refusal.is_some(), "the malformed text refuses");
    assert_eq!(frame::creation_offer(refusal.as_ref()), None);
    assert!(frame::retype_draft(&batch, refusal.as_ref()).is_some());

    // And a clean batch offers and restores nothing.
    let batch = vec![SessionOp::SetSlotExpression {
        node: extrude,
        slot: SlotId::Distance,
        text: "8 mm * 2.0".to_owned(),
    }];
    let mut refusal = None;
    for op in batch.clone() {
        if let Some(next) = session.perform(op).refusal {
            refusal = Refusal::preferred(refusal, next);
        }
    }
    assert!(refusal.is_none(), "{refusal:?}");
    assert_eq!(frame::creation_offer(refusal.as_ref()), None);
    assert_eq!(frame::retype_draft(&batch, refusal.as_ref()), None);
}

/// The XDG rules for where preferences live, held as a table.
///
/// Pure like the chooser probe beside it, and for the same reason:
/// the ambient read is one line, and everything that can be WRONG
/// here is the resolution. These rows hold whatever the CI box has in
/// its environment.
#[test]
fn the_preferences_path_follows_the_xdg_rules() {
    use std::ffi::OsStr;
    use std::path::PathBuf;

    let xdg = |c: Option<&str>, h: Option<&str>| {
        frame::prefs_path_in(c.map(OsStr::new), h.map(OsStr::new))
    };
    let tail = PathBuf::from("pncad").join("viewer.toml");

    // Set and non-empty wins outright — $HOME is not consulted.
    assert_eq!(
        xdg(Some("/cfg"), Some("/home/someone")),
        Some(PathBuf::from("/cfg").join(&tail)),
    );
    assert_eq!(
        xdg(Some("/cfg"), None),
        Some(PathBuf::from("/cfg").join(&tail))
    );

    // Unset falls back to $HOME/.config, as the spec says.
    assert_eq!(
        xdg(None, Some("/home/someone")),
        Some(PathBuf::from("/home/someone/.config").join(&tail)),
    );

    // EMPTY counts as unset — the spec says so in as many words, and
    // this is the row that matters: taking `""` as the base would
    // build a RELATIVE path and write preferences into whatever
    // directory the viewer was launched from.
    assert_eq!(
        xdg(Some(""), Some("/home/someone")),
        Some(PathBuf::from("/home/someone/.config").join(&tail)),
        "an empty XDG_CONFIG_HOME is unset, not a relative base",
    );
    // The same trap one variable over.
    assert_eq!(
        xdg(Some(""), Some("")),
        None,
        "an empty HOME is unset too — it would be a relative path as well",
    );

    // With neither, nothing is invented. The caller's store is then
    // unusable and says so, which is how a person learns their
    // preferences are not being kept.
    assert_eq!(xdg(None, None), None);

    // Whatever it resolves to is absolute, which is the property the
    // two empty-string rows above exist to protect.
    for path in [
        xdg(Some("/cfg"), None),
        xdg(None, Some("/home/someone")),
        xdg(Some(""), Some("/home/someone")),
    ]
    .into_iter()
    .flatten()
    {
        assert!(path.is_absolute(), "{} is not absolute", path.display());
    }
}
