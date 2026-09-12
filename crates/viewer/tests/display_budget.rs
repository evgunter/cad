//! **The picture the application opens on has a cost, and it is
//! bounded.**
//!
//! The behaviour of Open was never the defect. `docio::open` and
//! `SessionOp::Open` are exercised in `doc_io.rs` and always were; the
//! typed door works, the resolver rebinds, the log replays, the round
//! trip is byte-stable. What went untested was what opening a document
//! COSTS: a request the budget does not bind must be drawn as asked,
//! and one it binds must be coarsened to a picture inside the budget
//! rather than left to freeze the window on millions of triangles.
//! The tour's gallery ring is the fixture for both: ~1.6·10⁵
//! triangles at the starting 0.1 mm (inside the budget, drawn as
//! asked) and ~1.6·10⁶ at 0.01 mm (bound, and coarsened).
//!
//! These rows are that gap. Triangle counts are deterministic (D9:
//! byte-identical mesh for identical `(body, chordal)`), so the cost
//! is a fact a test can hold, and `scene::fit_delta` is the policy
//! that keeps it under `scene::TRIANGLE_BUDGET`.

// Panicking is a test's failure mechanism (workspace lint note).
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use pncad::geom_core::Tol;
use viewer::readout;
use viewer::scene::{self, DisplayTolerance, TRIANGLE_BUDGET};
use viewer::session::DocSession;

/// The δ the application starts on (`app::INITIAL_DELTA`, which is
/// `cfg`-gated behind the `app` feature and so is restated here; the
/// rows below say what the gallery ring and the startup plate cost AT
/// this δ, so a change to the number that matters moves them).
const INITIAL_DELTA: f64 = 1.0e-4;

/// A request the gallery ring exceeds the budget at: a decade finer
/// than the starting δ, where the ring's ~1.6·10⁵ triangles at 0.1 mm
/// become ~1.6·10⁶ (the 1/δ law `fit_delta` solves). The budget rows
/// need a document that is actually over budget, and since the torus
/// sizing spends its chord bound without slack the ring is not one at
/// the starting δ any more.
const OVER_BUDGET_DELTA: f64 = 1.0e-5;

/// The tour's gallery ring, as the committed fixture.
fn gallery_ring(tol: Tol) -> DocSession {
    let text = include_str!("gallery_ring.pncad");
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
/// At a δ the ring asks for far more than the budget at, the fit must
/// move δ; and what it moves to must be inside the budget, which is
/// the whole claim. Both halves matter: a fit that never coarsened
/// would leave the freeze, and a fit that coarsened without bound
/// would answer a cube.
#[test]
fn the_gallery_ring_is_drawn_inside_the_budget() {
    let tol = Tol::witness();
    let session = gallery_ring(tol);
    // The body the LANDING gathered — the same one the application
    // fits on, asked for the same way (`DocSession::landed_body`), so
    // this row costs the gather the landing already paid and no other.
    let body = session.landed_body().expect("the ring gathers");
    let requested = delta(OVER_BUDGET_DELTA);
    let fitted = scene::fit_delta(body, requested, tol).expect("the ring fits");

    let over = fitted.requested_cost.expect(
        "the ring at a decade under the startup δ is over budget — that is this row's premise",
    );
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
    let mesh =
        scene::scene_of_body(body, fitted.delta, tol).expect("the ring draws at the fitted δ");
    let triangles = mesh.stats().triangles;
    #[allow(clippy::cast_precision_loss)]
    let ratio = triangles as f64 / TRIANGLE_BUDGET as f64;
    assert!(
        ratio < 1.1,
        "the drawn picture is {triangles} triangles, {ratio:.3}× the budget — \
         the 1/δ prediction has drifted further than its measured few percent"
    );
}

/// **The ring at the δ the application starts on is INSIDE the
/// budget, and is drawn as asked** — the display-side statement of
/// the torus chart sizing: `mesh::sizing::torus_grid_steps` spends the
/// doubly-curved chord bound with no slack in its constant, and what
/// that buys the viewer is the gallery ring opening at 0.1 mm rather
/// than at whatever the budget could afford. A count over a quarter of
/// the budget here is the torus sizing gone loose by an order of
/// magnitude, not noise.
#[test]
fn the_gallery_ring_at_the_starting_delta_is_inside_the_budget() {
    let tol = Tol::witness();
    let session = gallery_ring(tol);
    let body = session.landed_body().expect("the ring gathers");
    let requested = delta(INITIAL_DELTA);
    let fitted = scene::fit_delta(body, requested, tol).expect("the ring fits");
    assert_eq!(fitted.delta, requested, "the ring is drawn as asked");
    assert_eq!(fitted.requested_cost, None, "nothing was over budget");
    let mesh = scene::scene_of_body(body, requested, tol).expect("the ring draws at 0.1 mm");
    let triangles = mesh.stats().triangles;
    assert!(
        triangles < TRIANGLE_BUDGET / 4,
        "the ring at the starting δ is {triangles} triangles — over a quarter of the \
         {TRIANGLE_BUDGET} budget, which is the torus sizing gone loose again"
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
    let body = session.landed_body().expect("the plate gathers");
    let requested = delta(INITIAL_DELTA);
    let fitted = scene::fit_delta(body, requested, tol).expect("the plate fits");

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
/// budget, because a picture opened at a δ nobody asked for has to say
/// what it is and why (#1097's posture: a door that cannot open says
/// so) — and it has to say that the budget is not a cap, which is the
/// next question a reader has.
#[test]
fn a_coarsened_picture_says_so_in_both_numbers() {
    let tol = Tol::witness();
    let session = gallery_ring(tol);
    let body = session.landed_body().expect("the ring gathers");
    let fitted = scene::fit_delta(body, delta(OVER_BUDGET_DELTA), tol).expect("fits");
    let wording = fitted
        .wording()
        .expect("a coarsened picture has a sentence");
    for needle in [
        // The renders, not a second formatting of them: a needle built
        // with a format string of its own would hold the sentence to
        // that string rather than to the δ, which is the defect the
        // render exists to close (`no_delta_renders_as_a_number_a_
        // delta_cannot_be`).
        &fitted.delta.render_mm(),
        &delta(OVER_BUDGET_DELTA).render_mm(),
        &TRIANGLE_BUDGET.to_string(),
        &"not a cap".to_owned(),
    ] {
        assert!(
            wording.contains(needle.as_str()),
            "the sentence does not carry {needle}: {wording}"
        );
    }
}

/// **No δ renders as a number a δ cannot be.** The field, the badge and
/// the sentence above all show δ as millimetres of text, and `{:.3}`
/// over millimetres reads `0.000` below half a micrometre — a value
/// [`DisplayTolerance::new`] refuses, in every place a user reads the δ
/// in force as a number they can act on. All three go through
/// [`DisplayTolerance::render_mm`] now, so this row covers all three.
///
/// The property is the whole range, so the row sweeps it: every δ from
/// `f64`'s smallest subnormal to a kilometre renders as text that fits
/// the bound, reads back through the millimetre conversion the δ field
/// commits with, and lands on a δ the door accepts within the render's
/// own stated accuracy.
///
/// **This is where the δ door's own acceptance is checked**, and it is
/// deliberately not checked a second time inside the render. The render
/// asks one question — does this text read back as the value — and for
/// a strictly positive δ that implies the rest; the implication is what
/// this row measures, over the whole type, rather than something the
/// render restates as a predicate no input can falsify.
#[test]
fn no_delta_renders_as_a_number_a_delta_cannot_be() {
    let reads_back_as_a_delta = |value: f64| {
        let d = delta(value);
        let text = d.render_mm();
        let mm = d.get() * 1.0e3;
        assert!(
            text.chars().count() <= readout::MAX_CHARS,
            "δ {mm} mm renders as {text}, past the {} character bound",
            readout::MAX_CHARS
        );
        let read: f64 = text.parse().unwrap_or_else(|error| {
            panic!("δ {mm} mm renders as {text}, which is not a number at all: {error}")
        });
        assert!(
            DisplayTolerance::new(read * 1.0e-3).is_ok(),
            "δ {mm} mm renders as {text}, which is not a δ this door accepts"
        );
        assert!(
            (read - mm).abs() <= readout::REL_TOLERANCE * mm,
            "δ {mm} mm renders as {text}, further from it than the render's own accuracy"
        );
    };
    // The grid the round-trip measurement used, a decade below the
    // kernel's finest to a decade above the coarsest δ a user types.
    let mut sampled: u32 = 0;
    let mut value = 1.0e-12_f64;
    while value < 1.0e-1 {
        reads_back_as_a_delta(value);
        sampled += 1;
        value *= 1.01;
    }
    assert!(sampled > 2_000, "the sweep covered only {sampled} δ");
    // And the ends of the type, which a geometric grid does not reach:
    // the smallest subnormal, the smallest normal, a kilometre.
    for end in [5.0e-324, f64::MIN_POSITIVE, 1.0e3] {
        reads_back_as_a_delta(end);
    }
}

/// The two δ the field used to lie about, by the numbers the item that
/// filed it named: 0.4 µm read `0.000` and 1.6 µm read `0.002`.
#[test]
fn the_two_deltas_the_fixed_three_decimal_render_lied_about() {
    assert_eq!(delta(0.4e-6).render_mm(), "0.0004");
    assert_eq!(delta(1.6e-6).render_mm(), "0.0016");
    // Both are exact here, which is what a decimal spelling buys where
    // it fits at all: the field shows the δ in force rather than a
    // rounding of it.
    assert_eq!("0.000", format!("{:.3}", 0.4e-6 * 1.0e3), "the old render");
    assert_eq!("0.002", format!("{:.3}", 1.6e-6 * 1.0e3), "and the other");
}

/// **What the render does to a budget δ's seventeen significant
/// figures: it shows four.** `fit_delta` solves `constant / budget`, so
/// a δ the budget chose is a quotient with no short spelling at all —
/// and no field is wide enough for one. Four figures is what the
/// character bound buys, which is why this text is a render and never a
/// commit path.
#[test]
fn a_budget_delta_renders_as_four_significant_figures() {
    // A constant in triangle·metres, exactly as `fit_delta` forms it.
    let constant = 0.374_612_345_678_901_2_f64;
    #[allow(clippy::cast_precision_loss)]
    let solved = constant / TRIANGLE_BUDGET as f64;
    let d = delta(solved);
    let exact = format!("{}", d.get() * 1.0e3);
    assert_eq!(exact, "0.0003746123456789012", "seventeen figures");
    assert!(
        exact.chars().count() > readout::MAX_CHARS,
        "and no field this crate has is that wide"
    );
    assert_eq!(d.render_mm(), "0.0003746", "four of them");
}
