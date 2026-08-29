//! **open(path) / save(path) round-trip, with no dialog anywhere.**
//!
//! The file dialog is the one thing in this unit that escapes headless
//! testing. These rows are the reason that costs nothing: the
//! operations the dialog calls are ordinary typed functions over a
//! `Path`, and everything a user gets from Open… and Save… is
//! exercised here without a window.

// Panicking is a test's failure mechanism (workspace lint note).
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

mod common;

use pncad::document::SlotId;
use pncad::geom_core::Tol;
use viewer::docio::{self, DocIoError};
use viewer::props::SlotValue;
use viewer::session::{DocSession, Refusal, SessionOp};

fn distance(
    doc: &pncad::document::Doc<pncad::document::ProfileProgram>,
    node: pncad::document::RecipeNodeId,
) -> SlotValue {
    viewer::props::slot_rows(doc, node)
        .into_iter()
        .find(|row| row.slot == SlotId::Distance)
        .expect("the extrude carries a distance")
        .value
        .expect("the distance evaluates")
}

#[test]
fn a_document_round_trips_through_save_and_open() {
    let tol = Tol::witness();
    let (doc, _profile, extrude) = common::parametric_plate(tol);
    let mut session = DocSession::inline(doc, tol);
    session.perform(SessionOp::SetParam {
        name: common::thickness_param(),
        value: SlotValue::Continuous(0.020),
    });
    assert_eq!(
        distance(session.committed_doc(), extrude),
        SlotValue::Continuous(0.010)
    );

    let dir = common::tempdir("gui3-round-trip");
    let file = dir.join("plate.pncad");
    assert!(
        session
            .perform(SessionOp::Save(file.clone()))
            .refusal
            .is_none()
    );
    assert_eq!(session.path(), Some(file.as_path()));

    // Opening into a fresh session reproduces the document AND its
    // history: the file's log is the reopened session's current path.
    let mut reopened = DocSession::inline(common::parametric_plate(tol).0, tol);
    assert!(
        reopened
            .perform(SessionOp::Open(file.clone()))
            .refusal
            .is_none()
    );
    assert_eq!(
        distance(reopened.committed_doc(), extrude),
        SlotValue::Continuous(0.010)
    );
    assert_eq!(reopened.history().path_edits().len(), 1);
    assert_eq!(reopened.path(), Some(file.as_path()));

    // And the reopened session can still undo the edit the file
    // carried, because the log became history rather than being
    // flattened into the snapshot.
    reopened.perform(SessionOp::Undo);
    assert_eq!(
        distance(reopened.committed_doc(), extrude),
        SlotValue::Continuous(0.004)
    );

    std::fs::remove_dir_all(&dir).expect("the fixture directory is removable");
}

#[test]
fn opening_a_missing_file_refuses_typed_and_leaves_the_session_alone() {
    let tol = Tol::witness();
    let (doc, _profile, extrude) = common::parametric_plate(tol);
    let mut session = DocSession::inline(doc, tol);
    let before = distance(session.committed_doc(), extrude);
    let outcome = session.perform(SessionOp::Open(
        std::env::temp_dir().join("gui3-no-such-document.pncad"),
    ));
    match outcome.refusal {
        Some(Refusal::Io(ref error)) => assert!(matches!(**error, DocIoError::Read { .. })),
        ref other => panic!("expected a read refusal, got {other:?}"),
    }
    assert_eq!(distance(session.committed_doc(), extrude), before);
    assert!(session.path().is_none());
}

#[test]
fn opening_a_file_that_is_not_a_document_refuses_at_the_persistence_door() {
    let tol = Tol::witness();
    let dir = common::tempdir("gui3-not-a-document");
    let file = dir.join("junk.pncad");
    std::fs::write(&file, "not a document").expect("the fixture file is writable");
    match docio::open(&file, tol) {
        Err(DocIoError::Persist(_)) => {}
        other => panic!("expected a persistence refusal, got {other:?}"),
    }
    std::fs::remove_dir_all(&dir).expect("the fixture directory is removable");
}

/// **A real gallery document, opened through the typed door.**
///
/// The file is the tour's `gallery` mode output for the ring scene,
/// committed verbatim — so this row is the acceptance walk (open a
/// gallery `.pncad`, see its feature tree with live statuses, save it
/// back) with the dialog and the window taken out.
///
/// It is version-stamped in its name, as `pncad`'s own fixture is: a
/// schema break makes this file unreadable, and the fix is to
/// regenerate it from `demo-tour gallery` and rename, never to teach
/// the loader about an old shape.
const GALLERY_RING: &str = include_str!("gallery_ring.v16.pncad");

/// **ε is a run parameter, and a saved document records the one it was
/// decided at** — "one process, one ε", which `load` enforces by
/// refusing a file whose recorded ε is not the process's
/// (`PersistError::ToleranceConflict`). The CI matrix sweeps ε, so a
/// committed document fixture is loadable at exactly one of its
/// points and refuses at the others; this row went red at 1e-12 for
/// precisely that reason.
///
/// So the fixture is re-stamped with THIS run's ε before it is opened.
/// The new ε line comes from `save` itself, via a throwaway document at
/// the process tolerance: spelling a float the way the serializer
/// spells it is the serializer's job, not this file's.
///
/// **What this function does NOT do is check its own work.** An earlier
/// version asserted that the re-stamp changed only the ε line by
/// comparing its own output's non-ε lines against its own input's — an
/// assertion true by construction of the `map` that built them, which
/// is to say no assertion at all. The real claim (a re-stamped fixture
/// is byte-for-byte what the exporter writes at this ε) is measured by
/// [`the_restamped_fixture_is_what_the_serializer_writes_at_this_eps`],
/// which puts the bytes back through `save` rather than through this
/// function's own arithmetic.
fn gallery_ring_at(tol: Tol) -> String {
    let probe: pncad::document::Doc<pncad::document::ProfileProgram> =
        pncad::document::Doc::empty_derived("gui3-epsilon-probe", tol);
    let probe_text = pncad::document::save(&probe, &[], tol).expect("an empty document saves");
    let is_epsilon = |line: &str| line.trim_start().starts_with("\"epsilon\":");
    let wanted = probe_text
        .lines()
        .find(|line| is_epsilon(line))
        .expect("a saved document records its ε");
    assert_eq!(
        GALLERY_RING.lines().filter(|l| is_epsilon(l)).count(),
        1,
        "the fixture must carry exactly one ε line"
    );
    let mut text: String = GALLERY_RING
        .lines()
        .map(|line| if is_epsilon(line) { wanted } else { line })
        .collect::<Vec<&str>>()
        .join("\n");
    text.push('\n');
    text
}

/// **The ε claim, measured through the serializer rather than through
/// the re-stamp's own arithmetic.**
///
/// The claim the fixture rests on is that ε is the file's ONLY
/// ε-dependent byte, so re-stamping one line reproduces what
/// `demo-tour gallery` would write at this run's tolerance. This row
/// takes the re-stamped bytes, loads them, and asks `save` — the same
/// door the exporter calls — to write the document back at this ε. If
/// the format ever grows a second ε-dependent byte the two texts
/// disagree and this goes red, which is exactly what the old
/// self-check could not do.
///
/// It measures at whichever point of the ε matrix the run drew, not
/// only at the one the fixture was born at, because the tolerance it
/// uses is the run's own witness.
#[test]
fn the_restamped_fixture_is_what_the_serializer_writes_at_this_eps() {
    let tol = Tol::witness();
    let restamped = gallery_ring_at(tol);
    let loaded = pncad::document::load(&restamped, tol).expect("the re-stamped fixture loads");
    assert!(
        loaded.edits.is_empty(),
        "the gallery ships state, not a log"
    );
    let rewritten =
        pncad::document::save(&loaded.snapshot, &loaded.edits, tol).expect("and saves back");
    assert_eq!(
        rewritten, restamped,
        "re-stamping ε reproduced the serializer's own output at this ε, \
         so ε is the file's only ε-dependent byte"
    );
}

#[test]
fn a_gallery_document_opens_evaluates_and_saves_back() {
    let tol = Tol::witness();
    let dir = common::tempdir("gui3-gallery");
    let file = dir.join("ring.pncad");
    std::fs::write(&file, gallery_ring_at(tol)).expect("the fixture is writable");

    let history = docio::open(&file, tol).expect("the gallery document opens");
    let mut session = DocSession::new(
        history.doc().clone(),
        tol,
        Box::new(viewer::InlineEvaluator::new()),
    );
    session.pump();

    let rows = session.tree_rows();
    assert_eq!(rows.len(), 3, "profile, axis datum, revolve");
    assert!(
        !viewer::tree::has_faults(&rows),
        "a gallery document evaluates clean: {:?}",
        rows.iter().map(|r| &r.status).collect::<Vec<_>>()
    );
    assert!(
        rows.iter().any(|row| row.kind == "Revolve" && row.root),
        "the revolve is the product root"
    );

    // Round-trip: opened, saved, and opened again is the same document.
    let out = dir.join("ring-again.pncad");
    assert!(
        session
            .perform(SessionOp::Save(out.clone()))
            .refusal
            .is_none()
    );
    let reopened = docio::open(&out, tol).expect("the re-saved document opens");
    assert!(
        reopened.doc().bit_eq(history.doc()),
        "bit-identical round trip"
    );

    std::fs::remove_dir_all(&dir).expect("the fixture directory is removable");
}

#[test]
fn a_saved_file_is_byte_identical_when_nothing_changed_between_saves() {
    // Save is a function of the history's current path, so saving the
    // same path twice must produce the same bytes — the property that
    // makes "did this document change" answerable at all.
    let tol = Tol::witness();
    let (doc, _profile, _extrude) = common::parametric_plate(tol);
    let mut session = DocSession::inline(doc, tol);
    session.perform(SessionOp::SetParam {
        name: common::thickness_param(),
        value: SlotValue::Continuous(0.020),
    });
    let dir = common::tempdir("gui3-stable-save");
    let a = dir.join("a.pncad");
    let b = dir.join("b.pncad");
    session.perform(SessionOp::Save(a.clone()));
    session.perform(SessionOp::Save(b.clone()));
    assert_eq!(
        std::fs::read_to_string(&a).expect("a is readable"),
        std::fs::read_to_string(&b).expect("b is readable")
    );
    std::fs::remove_dir_all(&dir).expect("the fixture directory is removable");
}

/// INVARIANT: a document whose product roots occupy the same space
/// still DRAWS, and the session carries the finding that says so.
///
/// This is the diefillet gallery bug, as a session row. The two roots
/// gather into one product whose picture looks almost right — the
/// second root's material fills the first's cavities and z-fights its
/// outer faces — and every local battery passes, because each root's
/// body is individually perfect. The report is the only thing that
/// says otherwise, so it has to land with the evaluation, and the
/// scene has to keep building alongside it (report, never gate: a
/// modeller cannot fix what the viewer refuses to show).
#[test]
fn overlapping_roots_still_draw_and_land_a_finding() {
    let tol = Tol::witness();
    // Two extrudes over the same square: two sinks, so two product
    // roots, exactly on top of each other.
    let mut doc = pncad::document::Doc::empty_derived("gui-overlap", tol);
    let mut roots = Vec::new();
    for _ in 0..2 {
        let profile = insert_node(
            &mut doc,
            pncad::document::Node::Profile(pncad::document::ProfileProgram {
                plane: pncad::prelude::SketchPlane::xy(),
                loops: vec![
                    pncad::prelude::LoopProgram::polygon([
                        (0.0, 0.0),
                        (1.0, 0.0),
                        (1.0, 1.0),
                        (0.0, 1.0),
                    ])
                    .expect("a square"),
                ],
            }),
            tol,
        );
        roots.push(insert_node(
            &mut doc,
            pncad::document::Node::Extrude {
                profile,
                distance: pncad::document::Expr::literal(1.0, pncad::document::Dimension::Length)
                    .expect("a length"),
            },
            tol,
        ));
    }
    assert_eq!(doc.roots().len(), 2, "two sinks, two product roots");

    let mut session = DocSession::new(doc, tol, Box::new(viewer::InlineEvaluator::new()));
    session.pump();

    // It draws. The scene is the thing the modeller needs in order to
    // see what is wrong with it.
    let (landed_doc, landed_ev) = session.landed_pair().expect("an evaluation landed");
    let scene = viewer::scene::scene_of_evaluation(
        landed_doc,
        landed_ev,
        viewer::DisplayTolerance::new(5e-3).expect("a display delta"),
        tol,
    )
    .expect("an overlapping product still tessellates");
    assert!(scene.stats().triangles > 0, "the picture is not empty");

    // And the finding landed with it, naming both roots.
    let report = session.checks().expect("the registry ran");
    let separation: Vec<_> = report
        .findings
        .iter()
        .filter(|f| f.check == pncad::document::CheckId::Separation)
        .collect();
    assert_eq!(separation.len(), 1, "one pair, one finding: {report}");
    let rendered = separation[0].to_string();
    for root in &roots {
        assert!(
            rendered.contains(&format!("root {}", root.0)),
            "the finding names both roots: {rendered}"
        );
    }
}

/// Insert one node, returning its minted id.
fn insert_node(
    doc: &mut pncad::document::Doc<pncad::document::ProfileProgram>,
    node: pncad::document::Node<pncad::document::ProfileProgram>,
    tol: Tol,
) -> pncad::document::RecipeNodeId {
    let applied = pncad::document::apply(doc, &pncad::document::DocEdit::InsertNode { node }, tol)
        .expect("the edit applies");
    *doc = applied.doc;
    applied.record.minted.expect("insert mints an id")
}
