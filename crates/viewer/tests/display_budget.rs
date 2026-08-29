//! **The picture the application opens on has a cost, and it is
//! bounded.**
//!
//! The behaviour of Open was never the defect. `docio::open` and
//! `SessionOp::Open` are exercised in `doc_io.rs` and always were; the
//! typed door works, the resolver rebinds, the log replays, the round
//! trip is byte-stable. What went untested was what opening a document
//! COSTS, and the answer for the tour's own gallery ring at the δ the
//! application starts on was four million triangles — tens of seconds
//! of tessellation and index build with the window frozen, still
//! showing the previous document.
//!
//! These rows are that gap. Triangle counts are deterministic (D9:
//! byte-identical mesh for identical `(body, chordal)`), so the cost
//! is a fact a test can hold, and `scene::fit_delta` is the policy
//! that keeps it under `scene::TRIANGLE_BUDGET`.

// Panicking is a test's failure mechanism (workspace lint note).
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

mod common;

use pncad::geom_core::Tol;
use viewer::scene::{self, DisplayTolerance, TRIANGLE_BUDGET};
use viewer::session::DocSession;

/// The δ the application starts on (`app::INITIAL_DELTA`, which is
/// `cfg`-gated behind the `app` feature and so is restated here — the
/// row below fails loudly if the two ever disagree about the number
/// that matters, because it asserts the ring is over budget AT this δ).
const INITIAL_DELTA: f64 = 1.0e-4;

/// The tour's gallery ring, as the committed fixture.
fn gallery_ring(tol: Tol) -> DocSession {
    let text = include_str!("gallery_ring.v15.pncad");
    // The fixture is stamped at the ε it was born at; `doc_io.rs` owns
    // the re-stamp and the proof that ε is its only ε-dependent byte.
    // Here the document only has to LOAD, so the born-at ε is fine and
    // the row runs at whatever the draw gave it.
    let loaded = match pncad::document::load(text, tol) {
        Ok(loaded) => loaded,
        Err(_) => {
            // A different ε row: re-stamp exactly as `doc_io` does.
            let probe: pncad::document::Doc<pncad::document::ProfileProgram> =
                pncad::document::Doc::empty_derived("budget-epsilon-probe", tol);
            let probe_text =
                pncad::document::save(&probe, &[], tol).expect("an empty document saves");
            let is_epsilon = |line: &str| line.trim_start().starts_with("\"epsilon\":");
            let wanted = probe_text
                .lines()
                .find(|line| is_epsilon(line))
                .expect("a saved document records its ε");
            let mut restamped: String = text
                .lines()
                .map(|line| if is_epsilon(line) { wanted } else { line })
                .collect::<Vec<&str>>()
                .join("\n");
            restamped.push('\n');
            pncad::document::load(&restamped, tol).expect("the re-stamped fixture loads")
        }
    };
    let mut session = DocSession::inline(loaded.snapshot, tol);
    session.pump();
    session
}

fn delta(value: f64) -> DisplayTolerance {
    DisplayTolerance::new(value).expect("a positive δ")
}

/// **The row the defect would have failed.**
///
/// At the application's starting δ the ring asks for far more than the
/// budget, so the fit must move δ; and what it moves to must be inside
/// the budget, which is the whole claim. Both halves matter: a fit
/// that never coarsened would leave the freeze, and a fit that
/// coarsened without bound would answer a cube.
#[test]
fn the_gallery_ring_is_drawn_inside_the_budget() {
    let tol = Tol::witness();
    let session = gallery_ring(tol);
    let (doc, evaluation) = session.landed_pair().expect("the ring evaluates");
    let requested = delta(INITIAL_DELTA);
    let fitted = scene::fit_delta(doc, evaluation, requested, tol).expect("the ring fits");

    let over = fitted
        .requested_cost
        .expect("the ring at the startup δ is over budget — that is this row's premise");
    assert!(
        over > TRIANGLE_BUDGET,
        "the requested δ was reported as costing {over}, which is not over the \
         {TRIANGLE_BUDGET} budget: then nothing needed fitting and this row is vacuous"
    );
    assert!(
        fitted.delta.get() > requested.get(),
        "δ must have been coarsened: asked {}, drawn {}",
        requested.get(),
        fitted.delta.get()
    );
    assert!(
        fitted.predicted <= TRIANGLE_BUDGET,
        "the drawn δ is predicted at {} triangles, over the {TRIANGLE_BUDGET} budget",
        fitted.predicted
    );

    // And the prediction is about the picture that actually gets
    // built: tessellate at the drawn δ and count. A few percent of
    // slack is the stated contract (`fit_delta` does not verify), so
    // the assertion is a bound with a margin, not an equality.
    let mesh = scene::scene_of_evaluation(doc, evaluation, fitted.delta, tol)
        .expect("the ring draws at the fitted δ");
    let triangles = mesh.stats().triangles;
    #[allow(clippy::cast_precision_loss)]
    let ratio = triangles as f64 / TRIANGLE_BUDGET as f64;
    assert!(
        ratio < 1.1,
        "the drawn picture is {triangles} triangles, {ratio:.3}× the budget — \
         the 1/δ prediction has drifted further than its measured few percent"
    );
}

/// The other side of the same rule: a document the budget does not
/// bind is drawn at exactly the δ it was asked for, and says nothing.
///
/// The startup plate is the application's own first picture, so if the
/// budget ever started moving δ here, every session would open on a
/// coarsened picture with a status line explaining itself.
#[test]
fn a_document_inside_the_budget_is_drawn_as_asked() {
    let tol = Tol::witness();
    let (doc, _root) = viewer::scene::plate_with_hole(tol).expect("the startup document");
    let mut session = DocSession::inline(doc, tol);
    session.pump();
    let (doc, evaluation) = session.landed_pair().expect("the plate evaluates");
    let requested = delta(INITIAL_DELTA);
    let fitted = scene::fit_delta(doc, evaluation, requested, tol).expect("the plate fits");

    assert_eq!(fitted.delta, requested, "the plate is drawn as asked");
    assert_eq!(
        fitted.requested_cost, None,
        "nothing was over budget, so there is nothing to report"
    );
    assert_eq!(
        fitted.wording(),
        None,
        "and the status line stays quiet about a picture drawn as asked"
    );
}

/// The report, when there is one: the sentence names both δ and the
/// budget, because a picture that is not what was asked for has to say
/// what it is and why (#1097's posture: a door that cannot open says
/// so).
#[test]
fn a_coarsened_picture_says_so_in_both_numbers() {
    let tol = Tol::witness();
    let session = gallery_ring(tol);
    let (doc, evaluation) = session.landed_pair().expect("the ring evaluates");
    let fitted = scene::fit_delta(doc, evaluation, delta(INITIAL_DELTA), tol).expect("fits");
    let wording = fitted
        .wording()
        .expect("a coarsened picture has a sentence");
    for needle in [
        &format!("{:.3}", fitted.delta.get() * 1.0e3),
        &format!("{:.3}", INITIAL_DELTA * 1.0e3),
        &TRIANGLE_BUDGET.to_string(),
    ] {
        assert!(
            wording.contains(needle.as_str()),
            "the sentence does not carry {needle}: {wording}"
        );
    }
}
