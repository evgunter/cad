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

use crate::common;

use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

use common::asm;
use pncad::document::{
    CheckEvidence, CheckFinding, CheckId, ChecksReport, Doc, Frame, Node, ParamName, ProductError,
    ProfileProgram, RecipeNodeId, SlotId,
};
use pncad::geom_core::{Point3, Tol, Vec3};
use pncad::select::{ContactClass, Ray};
use viewer::camera::{Camera, CameraOp};
use viewer::display::{DisplayFault, DisplayView};
use viewer::evalseam::{Generation, IndexDone, IndexRequest, IndexService, InlineIndexer};
use viewer::frame::{self, IdQueryLog, IdStep, StatusUpdate};
use viewer::input::{self, InputMap, ViewportSize};
use viewer::pick::{self, CacheStep, IdMap, IndexLanding, PickCache, PickIndex};
use viewer::props::SlotValue;
use viewer::scene::{self, DisplayTolerance, FittedDelta, PLATE_EXTENT};
use viewer::session::{
    AtRestBadge, DocSession, FaceSelection, Hovered, Refusal, Selection, SessionOp,
};

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
    assert_eq!(
        shown,
        StatusUpdate::Show(frame::Message::new(
            frame::Subject::Document,
            refusal.to_string()
        )),
        "the document's answer to the act is about the document"
    );
    assert!(!refusal.to_string().is_empty());
}

/// **A tool's notice is not erased by the batch that carried its own
/// pick** — the composition seam, as the rule that closes it.
///
/// The defect this row exists for: the blend tool declines a pick on a
/// second body and says so, but the declined click is still a `Select`
/// that the session performs cleanly. `batch_status` sees an acting op
/// and no refusal, answers `Clear`, and the explanation is wiped in
/// the same frame it was written — net effect, the selection jumps to
/// another body and the sentence saying why it did not join the blend
/// is shown for zero frames.
///
/// Both halves are asserted, because the first is what makes the
/// second necessary rather than decorative.
#[test]
fn a_tool_notice_survives_the_batch_that_carried_its_own_pick() {
    let declined = [SessionOp::Select(Selection::None)];
    let notice = frame::Message::new(
        frame::Subject::Document,
        "blend tool: the held edges are on feature 3 body 0",
    );

    // The batch policy alone: the frame acted, nothing refused, so the
    // line is cleared. This is the seam.
    assert_eq!(
        frame::batch_status(&declined, None),
        StatusUpdate::Clear,
        "a declined pick still performs cleanly, so the batch alone clears"
    );

    // The frame policy: the notice is what the line shows.
    assert_eq!(
        frame::frame_status(std::slice::from_ref(&notice), &declined, None),
        StatusUpdate::Show(notice.clone())
    );

    // A refusal outranks it — the answer to what the user asked the
    // DOCUMENT for is the louder of the two.
    let refusal = Refusal::NothingToDo;
    assert_eq!(
        frame::frame_status(std::slice::from_ref(&notice), &declined, Some(&refusal)),
        StatusUpdate::Show(frame::Message::new(
            frame::Subject::Document,
            refusal.to_string()
        ))
    );

    // With no notices the frame policy is the batch policy, verdict
    // for verdict — including the hover rule, which must not become a
    // second opinion here.
    for ops in [
        vec![SessionOp::Hover(None)],
        vec![SessionOp::Select(Selection::None)],
        vec![],
    ] {
        assert_eq!(
            frame::frame_status(&[], &ops, None),
            frame::batch_status(&ops, None),
            "{ops:?}"
        );
    }

    // SEVERAL notices in one frame are all shown, joined. Assigning
    // `status` from each in turn keeps the last and loses the rest,
    // which is the keep-last defect the batch policy already exists to
    // stop for refusals.
    let second = frame::Message::new(
        frame::Subject::Document,
        "blend tool: an edit removed 6 of the picked edges",
    );
    let both = frame::frame_status(&[notice.clone(), second.clone()], &declined, None);
    let StatusUpdate::Show(line) = &both else {
        panic!("two notices are shown, got {both:?}");
    };
    assert!(
        line.text.contains(&notice.text) && line.text.contains(&second.text),
        "{line}"
    );
    assert_eq!(
        line.text,
        [notice.text.as_str(), second.text.as_str()].join(frame::NOTICE_SEPARATOR)
    );
    assert_eq!(
        line.subject,
        frame::Subject::Document,
        "notices that agree on a subject are joined under it, so the \
         joined line still knows what retires it"
    );
}

/// **Every badge's silence is a row now**, which is the whole reason
/// the family became a vocabulary: the checks badge's "only when there
/// are findings" rule used to be an `&&` inside a `ui` closure, where
/// no test could reach it and nothing but a human eye said whether it
/// was right.
#[test]
fn a_badge_that_has_nothing_to_say_says_nothing() {
    assert_eq!(frame::at_rest_badge(None), None, "no assembly, no verdict");
    assert_eq!(
        frame::checks_badge(None),
        None,
        "nothing landed, or the registry refused"
    );
    assert_eq!(
        frame::checks_badge(Some(&ChecksReport {
            findings: Vec::new(),
            skipped: Vec::new(),
        })),
        None,
        "checked and fine is not a finding — the rule that lived in a \
         `ui` closure until this type existed"
    );
    assert_eq!(
        frame::checks_badge(Some(&ChecksReport {
            findings: Vec::new(),
            skipped: vec![CheckId::Connectedness],
        })),
        None,
        "and a SKIPPED check does not light it either: not checked and \
         checked-and-found-something are different answers, and the \
         window is where that distinction is drawn"
    );
    assert_eq!(frame::delta_badge(None), None, "the user's own δ");
    assert_eq!(frame::product_badge(None), None);
}

/// The tone split is the actionable-or-not rule, stated by a value
/// rather than picked at four call sites — the rule `pane::features`
/// argues explicitly for poisoned rows and that nothing used to say.
#[test]
fn a_badge_states_whether_a_reader_has_anything_to_do_about_it() {
    let certified = frame::at_rest_badge(Some(&AtRestBadge::Certified { minted: 4 }))
        .expect("a certified assembly badges");
    assert_eq!(
        certified.tone,
        frame::Tone::Advisory,
        "good news is a report: {}",
        certified.label
    );
    assert!(
        certified.label.starts_with("at rest: "),
        "{}",
        certified.label
    );
    assert!(certified.label.contains('4'), "{}", certified.label);

    let refused = frame::at_rest_badge(Some(&AtRestBadge::Refused {
        message: "the gate declined to certify".to_owned(),
    }))
    .expect("a refusal badges");
    assert_eq!(
        refused.tone,
        frame::Tone::Actionable,
        "and the reader is the only one who can answer a refusal"
    );
    assert!(
        refused.label.ends_with("the gate declined to certify"),
        "the typed refusal's own words, unaltered: {}",
        refused.label
    );

    let fitted = FittedDelta::as_requested(DisplayTolerance::new(1.0e-3).expect("a positive δ"));
    assert_eq!(
        frame::delta_badge(Some(&fitted)),
        None,
        "a fit with nothing to say is the second half of this badge's \
         `None`, and it used to be a second condition at the call site"
    );
}

/// The checks badge is a BUTTON, and that is ratified rather than
/// incidental: a tooltip is the wrong home for text a reader needs to
/// keep open while they act on it, because it is gone the moment the
/// pointer moves toward the feature it names. A uniformity pass over
/// the family must not flatten it, so the value says so.
#[test]
fn the_checks_badge_is_a_control_and_the_rest_are_labels() {
    let report = ChecksReport {
        findings: vec![CheckFinding {
            check: CheckId::Connectedness,
            root: RecipeNodeId(3),
            output_ix: 0,
            evidence: CheckEvidence::Connectedness {
                actual: 2,
                expected: 1,
            },
        }],
        skipped: Vec::new(),
    };
    let badge = frame::checks_badge(Some(&report)).expect("a finding badges");
    assert_eq!(badge.affordance, frame::Affordance::Opens);
    assert_eq!(badge.tone, frame::Tone::Actionable);
    assert_eq!(badge.label, "checks: 1 finding(s)");
    assert!(
        badge.detail.is_some(),
        "and it says what opening it does — the hover is the invitation, \
         never the findings themselves"
    );
    assert!(
        !badge.label.contains("component"),
        "the findings' own sentences live in the window it opens, so the \
         badge composes no prose about them: {}",
        badge.label
    );

    for label in [
        frame::at_rest_badge(Some(&AtRestBadge::Certified { minted: 0 })),
        frame::product_badge(Some(&ProductError::NoBodyRoots)),
    ]
    .into_iter()
    .flatten()
    {
        assert_eq!(label.affordance, frame::Affordance::Read, "{}", label.label);
    }
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
    assert_eq!(message.text, frame::NO_CHOOSER_BACKEND);
    assert!(message.text.contains("zenity"));
    assert!(message.text.contains("xdg-desktop-portal"));
    assert!(message.text.contains("command line"));
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
    let (doc, profile) = common::framed_square(&doc, 0.02, tol);
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

// --- the rebuild loop, across the index seam -------------------------

/// An [`InlineIndexer`] that counts what reaches it.
///
/// The receipt for *at most one attempt per (generation, δ)*: with the
/// build behind a seam, a cache that submits on every frame and a
/// cache that submits once are indistinguishable from the outside —
/// both end with one index — unless something counts the submits.
struct CountingIndexer {
    inner: InlineIndexer,
    submits: Arc<AtomicUsize>,
}

impl CountingIndexer {
    fn new() -> (Box<Self>, Arc<AtomicUsize>) {
        let submits = Arc::new(AtomicUsize::new(0));
        (
            Box::new(Self {
                inner: InlineIndexer::new(),
                submits: Arc::clone(&submits),
            }),
            submits,
        )
    }
}

impl IndexService for CountingIndexer {
    fn submit(&mut self, request: IndexRequest) {
        self.submits.fetch_add(1, Ordering::Relaxed);
        self.inner.submit(request);
    }

    fn poll(&mut self) -> Option<IndexDone> {
        self.inner.poll()
    }

    fn busy(&self) -> bool {
        self.inner.busy()
    }
}

#[test]
fn a_refused_index_is_attempted_once_per_generation_and_not_once_per_frame() {
    // The defect: a failed or poisoned root is an ordinary editing
    // state, and the app's guard stayed false forever afterwards — so
    // every repainted frame re-tessellated every healthy root before
    // reaching the failing one, behind a picture already stale.
    //
    // The policy now has a seam under it, so the row counts SUBMITS:
    // the frames a stalled cache would have spent are frames it must
    // not put work on the worker for either.
    let tol = Tol::witness();
    let (doc, extrude) = scene::plate_with_hole(tol).expect("the plate authors");
    let mut session = DocSession::inline(doc, tol);
    session.pump();
    let (seam, submits) = CountingIndexer::new();
    let mut cache = PickCache::new(seam);
    assert_eq!(cache.sync(&session, delta()), CacheStep::Submitted);
    assert_eq!(cache.pump(), vec![IndexLanding::Built]);
    assert_eq!(cache.sync(&session, delta()), CacheStep::Current);
    assert!(cache.index().is_some());
    assert!(cache.error().is_none());
    assert_eq!(submits.load(Ordering::Relaxed), 1);

    // Break the document so the root refuses to evaluate.
    let outcome = session.perform(SessionOp::SetSlot {
        node: extrude,
        slot: SlotId::Distance,
        value: SlotValue::Continuous(0.0),
    });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    session.pump();

    assert_eq!(cache.sync(&session, delta()), CacheStep::Submitted);
    assert_eq!(
        cache.pump(),
        vec![IndexLanding::Refused],
        "a failed root refuses the index"
    );
    assert!(cache.error().is_some(), "the refusal is readable");
    assert!(
        cache.index().is_none(),
        "and the index built for the document before the break is gone"
    );
    // The frame after, and the frame after that: HELD. This is the
    // whole row — before the fix, both of these were another full
    // rebuild attempt.
    assert_eq!(
        cache.sync(&session, delta()),
        CacheStep::Held,
        "a refused build is not retried on the next frame"
    );
    assert_eq!(cache.sync(&session, delta()), CacheStep::Held);
    assert!(cache.pump().is_empty(), "and nothing was sent to answer");
    assert_eq!(
        submits.load(Ordering::Relaxed),
        2,
        "two pictures, two attempts — not one per frame"
    );

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
    let (seam, submits) = CountingIndexer::new();
    let mut cache = PickCache::new(seam);
    assert_eq!(cache.sync(&session, delta()), CacheStep::Submitted);
    assert_eq!(cache.pump(), vec![IndexLanding::Built]);
    // δ is part of the key: the parts are the tessellations the picture
    // is drawn from.
    let coarser = delta().scaled(2.0).expect("a positive delta");
    assert_eq!(cache.sync(&session, coarser), CacheStep::Submitted);
    assert_eq!(cache.pump(), vec![IndexLanding::Built]);
    assert_eq!(cache.sync(&session, coarser), CacheStep::Current);

    session.perform(SessionOp::SetSlot {
        node: extrude,
        slot: SlotId::Distance,
        value: SlotValue::Continuous(PLATE_EXTENT[2] * 1.5),
    });
    session.pump();
    assert_eq!(cache.sync(&session, coarser), CacheStep::Submitted);
    assert_eq!(cache.pump(), vec![IndexLanding::Built]);
    assert_eq!(submits.load(Ordering::Relaxed), 3);
}

#[test]
fn a_cache_with_nothing_landed_has_nothing_to_do() {
    let tol = Tol::witness();
    let (doc, _) = scene::plate_with_hole(tol).expect("the plate authors");
    // No `pump`, so nothing has landed.
    let session = DocSession::inline(doc, tol);
    let mut cache = PickCache::inline();
    assert_eq!(cache.sync(&session, delta()), CacheStep::Nothing);
    assert!(cache.index().is_none());
    assert!(!cache.indexing());
}

/// **The window the seam creates, and what is in it.** Between the
/// submit and the answer the cache holds NO index — not the previous
/// one — so there is nothing a pick could be answered from, which is
/// the whole reason the answer is allowed to arrive late.
#[test]
fn between_a_submit_and_its_answer_there_is_no_index_to_pick_from() {
    let tol = Tol::witness();
    let (mut session, extrude) = plate_session(tol);
    let mut cache = PickCache::inline();
    assert_eq!(cache.sync(&session, delta()), CacheStep::Submitted);
    assert_eq!(cache.pump(), vec![IndexLanding::Built]);
    let first = cache.index().expect("the plate indexes").generation();

    session.perform(SessionOp::SetSlot {
        node: extrude,
        slot: SlotId::Distance,
        value: SlotValue::Continuous(PLATE_EXTENT[2] * 1.5),
    });
    session.pump();
    assert_eq!(cache.sync(&session, delta()), CacheStep::Submitted);
    assert!(
        cache.index().is_none(),
        "the index for the previous document is dropped at the SUBMIT, \
         not when its replacement lands"
    );
    assert!(cache.indexing(), "and the chrome has something to say");
    // Asked again on the next frame: still waiting, and nothing is
    // resubmitted.
    assert_eq!(cache.sync(&session, delta()), CacheStep::Indexing);

    assert_eq!(cache.pump(), vec![IndexLanding::Built]);
    assert!(!cache.indexing());
    let second = cache
        .index()
        .expect("the edited plate indexes")
        .generation();
    assert_ne!(first, second, "and the index that landed is the new one");
}

/// **The one arm where "current or absent, never behind" could break,
/// and the only one that can produce a CONFIDENTLY WRONG pick.**
///
/// Every other transition swaps one picture's key for another's, so a
/// late answer meets a key it does not match and is discarded. A
/// `New document` (or an `Open`) under a build that is still with the
/// seam leaves NO next key: the session's landed run is gone. Before
/// `PickCache::forget`, the vanished picture's key stayed on the cache
/// and the in-flight build matched it on arrival — installing the
/// index of the document that was just replaced, over the scene of the
/// one before THAT, with nothing running, nothing on the status line
/// and no way to self-correct until the newly-opened document landed.
#[test]
fn a_build_in_flight_when_the_document_is_replaced_installs_nothing() {
    let tol = Tol::witness();
    let (mut session, extrude) = plate_session(tol);
    let mut cache = PickCache::inline();
    assert_eq!(cache.sync(&session, delta()), CacheStep::Submitted);
    assert_eq!(cache.pump(), vec![IndexLanding::Built]);

    // An edit lands, and its index build is submitted but not answered
    // — the window this seam creates.
    session.perform(SessionOp::SetSlot {
        node: extrude,
        slot: SlotId::Distance,
        value: SlotValue::Continuous(PLATE_EXTENT[2] * 1.5),
    });
    session.pump();
    assert_eq!(cache.sync(&session, delta()), CacheStep::Submitted);
    assert!(cache.index().is_none());

    // The document is replaced mid-build. The session's landed run
    // goes with it, so there is no picture for the outstanding build
    // to be the index OF.
    session.perform(SessionOp::NewDocument {
        name: "fresh".to_owned(),
    });
    assert!(session.landed_generation().is_none());
    assert_eq!(cache.sync(&session, delta()), CacheStep::Nothing);
    assert!(
        !cache.indexing(),
        "nothing the chrome should promise an answer for",
    );

    // The build finishes. It must be discarded, not installed.
    assert_eq!(
        cache.pump(),
        vec![IndexLanding::Stale],
        "an index of the replaced document is not an index of anything on screen",
    );
    assert!(
        cache.index().is_none(),
        "and no pick can be answered from it",
    );
    assert!(cache.error().is_none());

    // The new document lands: an ordinary fresh attempt, not a state
    // the cache has to be talked out of.
    session.pump();
    assert_eq!(cache.sync(&session, delta()), CacheStep::Submitted);
}

/// **The ordinary half of the same replacement**, which the row above
/// cannot reach: a CURRENT index and no build in flight when the
/// document is replaced.
///
/// The two rows constrain different halves of `PickCache::forget`.
/// There, `index` and `error` are already `None` — the submit that
/// opened the window cleared them — so only the attempt's two fields
/// are doing anything. Here nothing is outstanding and the index is
/// the live one, so it is the `index` field alone that decides whether
/// a pick after the replacement is answered from the document that was
/// just replaced.
#[test]
fn replacing_the_document_drops_a_current_index_with_no_build_in_flight() {
    let tol = Tol::witness();
    let (session, _extrude) = plate_session(tol);
    let mut cache = PickCache::inline();
    assert_eq!(cache.sync(&session, delta()), CacheStep::Submitted);
    assert_eq!(cache.pump(), vec![IndexLanding::Built]);
    assert_eq!(cache.sync(&session, delta()), CacheStep::Current);
    assert!(cache.index().is_some());

    let mut session = session;
    session.perform(SessionOp::NewDocument {
        name: "fresh".to_owned(),
    });
    assert!(session.landed_generation().is_none());
    assert!(!cache.indexing(), "nothing was outstanding to begin with");

    assert_eq!(cache.sync(&session, delta()), CacheStep::Nothing);
    assert!(
        cache.index().is_none(),
        "the picture is the replaced document's, and a pick answered from \
         it would name entities of a document nobody can see",
    );
}

/// **The confidently-wrong answer this seam makes possible**, refused.
/// A build finishing for a generation the session has moved past is
/// exactly what restart-without-cancel produces, and installing it
/// would answer picks against a document that is gone.
#[test]
fn an_answer_for_a_superseded_generation_is_discarded_not_installed() {
    let tol = Tol::witness();
    let (mut session, extrude) = plate_session(tol);
    let stale = index_of(&session);
    assert_eq!(
        stale.generation(),
        session.landed_generation().expect("a generation")
    );

    let mut cache = PickCache::inline();
    session.perform(SessionOp::SetSlot {
        node: extrude,
        slot: SlotId::Distance,
        value: SlotValue::Continuous(PLATE_EXTENT[2] * 1.5),
    });
    session.pump();
    assert_eq!(cache.sync(&session, delta()), CacheStep::Submitted);

    let landing = cache.land(IndexDone {
        generation: stale.generation(),
        delta: delta(),
        index: Ok(stale),
    });
    assert_eq!(landing, IndexLanding::Stale);
    assert!(
        cache.index().is_none(),
        "a pick answered from that index would name entities of a document \
         nobody is looking at"
    );
    assert!(
        cache.indexing(),
        "and the build actually asked for is still owed"
    );
}

/// The same refusal on the sharper half of the key: the document is
/// RIGHT and only the δ is wrong, so a check that compared generations
/// alone would install this one without complaint.
#[test]
fn an_answer_built_at_another_delta_is_discarded_too() {
    let tol = Tol::witness();
    let (session, _extrude) = plate_session(tol);
    let coarse = index_of(&session);
    let generation = session.landed_generation().expect("a generation");

    let mut cache = PickCache::inline();
    let finer = delta().scaled(0.5).expect("a positive delta");
    assert_eq!(cache.sync(&session, finer), CacheStep::Submitted);
    let landing = cache.land(IndexDone {
        generation,
        delta: delta(),
        index: Ok(coarse),
    });
    assert_eq!(
        landing,
        IndexLanding::Stale,
        "same generation, other δ — the picture asked for is not this one"
    );
    assert!(cache.index().is_none());
}

/// **Not indexed yet is not nothing under the cursor.** A click during
/// the window is news; the hover pushed on every frame the pointer is
/// inside the pane is not.
#[test]
fn a_click_with_no_index_refuses_typed_and_a_hover_stays_quiet() {
    let click = [input::PickAction::Select([10.0, 10.0])];
    assert_eq!(
        pick::unindexed(&click, true),
        Some(pick::NotIndexed::Building),
    );
    assert_eq!(
        pick::unindexed(&click, false),
        Some(pick::NotIndexed::Absent),
        "a refused build is not a build that is still running, and the \
         sentence must not promise an answer that is not coming",
    );
    for indexing in [true, false] {
        assert_eq!(
            pick::unindexed(
                &[
                    input::PickAction::Hover([10.0, 10.0]),
                    input::PickAction::ClearHover,
                ],
                indexing,
            ),
            None,
            "an observation asked every frame is not a refusal to report",
        );
        assert_eq!(pick::unindexed(&[], indexing), None);
    }
    assert_ne!(
        pick::NotIndexed::Building.to_string(),
        pick::NotIndexed::Absent.to_string(),
    );
    for refusal in [pick::NotIndexed::Building, pick::NotIndexed::Absent] {
        assert!(
            refusal.to_string().contains("index"),
            "and each sentence says which of the two answers it is",
        );
    }
}

/// One indicator for one wait, and the ranking that decides which.
#[test]
fn the_chrome_has_one_progress_state_and_evaluation_outranks_indexing() {
    // busy, running, indexing.
    assert_eq!(frame::progress(false, false, false), None);
    assert_eq!(
        frame::progress(true, true, false),
        Some(frame::Progress::Evaluating)
    );
    assert_eq!(
        frame::progress(true, false, false),
        Some(frame::Progress::Canceled { indexing: false }),
        "a spinner over no running work would be a lie",
    );
    assert_eq!(
        frame::progress(true, false, true),
        Some(frame::Progress::Canceled { indexing: true }),
        "a cancel with an index build still running is one state that \
         carries the work, not a second indicator beside it",
    );
    assert_eq!(
        frame::progress(false, false, true),
        Some(frame::Progress::Indexing)
    );
    assert_eq!(
        frame::progress(true, true, true),
        Some(frame::Progress::Evaluating),
        "an index for a superseded generation is about to be discarded",
    );
    // The last two of the eight. `busy` is "the picture is older than
    // the document" and `running` is "the seam has work", so a seam
    // with work outstanding always has a generation the picture has
    // not caught up to: NOT busy while running is unreachable through
    // `DocSession`. The function is total anyway, and what it answers
    // there is written down rather than left to be discovered.
    for indexing in [false, true] {
        assert_eq!(
            frame::progress(false, true, indexing),
            frame::progress(false, false, indexing),
            "with the picture current, a running evaluation the session \
             cannot report changes nothing",
        );
    }
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
    let (doc, profile) = common::framed_square(&doc, 0.04, tol);
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

/// **A superseded free move reaches the line — driven the way the
/// frame loop drives it.**
///
/// The value the session computes here is the one the chrome used to
/// drop: `OpOutcome::superseded`, which nothing outside the test suite
/// read. This row fails if it goes silent again, and it is built from
/// a REAL outcome — a real mate landing on a real assembly — because a
/// hand-made `OpOutcome` would assert about a shape rather than about
/// the path.
#[test]
fn a_superseded_free_move_is_news_the_ranking_shows() {
    let tol = Tol::witness();
    let bench = asm::bench("framepolicy-supersede", tol);
    let mut session = asm::open_bench(&bench, tol);

    // The user places the unconstrained post by hand and COMMITS it —
    // only a committed placement is ever reported superseded.
    for op in [
        SessionOp::BeginFreeMove {
            instance: bench.post_b,
        },
        SessionOp::PreviewFreeMove {
            frame: Frame::translation([0.04, 0.0, 0.0]),
        },
        SessionOp::CommitFreeMove,
    ] {
        let outcome = session.perform(op);
        assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    }
    assert!(
        session.display().free_move_of(bench.post_b).is_some(),
        "the placement is held, so there is something to discard"
    );

    // Then they mate it, and that placement is discarded under them.
    let mate = SessionOp::AddMate {
        a: asm::in_part(bench.post_b, &bench.post_top),
        b: asm::in_part(bench.shelf_i, &bench.shelf_bottom),
        class: ContactClass::Rest,
        alignment: asm::seat_alignment(asm::SHELF_LENGTH / 2.0, None),
    };
    let outcome = session.perform(mate.clone());
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    let [superseded] = &outcome.superseded[..] else {
        panic!(
            "exactly one placement is superseded: {:?}",
            outcome.superseded
        )
    };
    assert_eq!(
        superseded.instance, bench.post_b,
        "the mate discards the hand placement"
    );
    // The PAYLOAD, not the variant: the variant is what the op this row
    // just performed already implies.
    let DisplayFault::MateConstrained { instance, mates } = &superseded.cause else {
        panic!(
            "a mate landing supersedes with its own fault: {}",
            superseded.cause
        )
    };
    assert_eq!(*instance, bench.post_b, "the fault names the same instance");
    assert!(
        !mates.is_empty(),
        "and names the mate that landed, which is what the line then reads"
    );

    // What the chrome does with it, in `perform_batch`'s order. This
    // is a HAND-WRITTEN MIRROR of app-gated code no row can reach, so
    // it has to model every producer that feeds the notices there —
    // both withdrawal channels, not just the one this row provokes. A
    // half-mirror would pass while the real loop dropped the other.
    let notices: Vec<frame::Message> = frame::Withdrawal::superseded(&outcome.superseded)
        .into_iter()
        .chain(frame::Withdrawal::dropped_hide(&outcome.dropped_hides))
        .map(|withdrawal| withdrawal.notice())
        .collect();
    assert_eq!(
        notices.len(),
        1,
        "a mate landing on a probed instance withdraws a placement and no \
         hide, so the second producer is silent here rather than absent"
    );
    let update = frame::frame_status(
        &notices,
        core::slice::from_ref(&mate),
        outcome.refusal.as_ref(),
    );
    let StatusUpdate::Show(message) = update else {
        panic!("a discarded placement is news, not silence: {update:?}");
    };
    assert!(
        message
            .text
            .contains(&format!("instance {}", bench.post_b.0)),
        "the line names which of the user's placements went: {message}"
    );
    assert_eq!(
        message.subject,
        frame::Subject::Document,
        "and it is about the document that superseded it"
    );

    // The counterfactual, so this row is not asserting about a frame
    // where nothing had to survive anything: the SAME frame, with the
    // supersession dropped on the floor as it used to be, clears the
    // line instead of saying any of that.
    assert_eq!(
        frame::frame_status(&[], core::slice::from_ref(&mate), outcome.refusal.as_ref()),
        StatusUpdate::Clear,
    );
}
