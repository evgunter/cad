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

use editor_core::{DocEdit, ProfileDoc};
use pncad::geom_core::Tol;
use viewer::readout;
use viewer::scene::{self, DisplayTolerance, ProbeStop, TRIANGLE_BUDGET};
use viewer::session::DocSession;

use crate::corpus;

/// The δ the application starts on (`app::INITIAL_DELTA`, which is
/// `cfg`-gated behind the `app` feature and so is restated here; the
/// rows below say what the gallery ring and the startup plate cost AT
/// this δ, so a change to the number that matters moves them).
const INITIAL_DELTA: f64 = 1.0e-4;

/// How much coarser than the δ it prices a rung of the fit's ladder
/// runs (`scene`'s `PROBE_FACTOR`, which is private to it and so is
/// restated here; the rung bound below is read against this number, so
/// a change to it moves what this row asserts).
const PROBE_FACTOR: f64 = 8.0;

/// A request the gallery ring exceeds the budget at: a decade finer
/// than the starting δ, where the ring's ~1.6·10⁵ triangles at 0.1 mm
/// become ~1.6·10⁶ (the 1/δ law `fit_delta` solves). The budget rows
/// need a document that is actually over budget, and since the torus
/// sizing spends its chord bound without slack the ring is not one at
/// the starting δ any more.
const OVER_BUDGET_DELTA: f64 = 1.0e-5;

/// The tour's gallery ring, as the committed fixture.
fn gallery_ring(tol: Tol) -> DocSession {
    let mut session = DocSession::inline(gallery_ring_doc(tol), tol);
    session.pump();
    session
}

/// The same fixture as a document, for the rows that open every
/// document the same way.
fn gallery_ring_doc(tol: Tol) -> ProfileDoc {
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
    loaded.snapshot
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

/// Every document the budget is asked about, in one place: every
/// parametric corpus document (the ones the application's own latency
/// rows drive) and the tour's gallery ring. `die_composed_tour` is the
/// other gallery document and is already in the corpus.
fn budgeted_documents(tol: Tol) -> Vec<(&'static str, ProfileDoc)> {
    let mut out: Vec<(&'static str, ProfileDoc)> = corpus::documents()
        .into_iter()
        .filter(|c| matches!(c.bump, DocEdit::SetParam { .. }))
        .map(|c| (c.name, c.doc))
        .collect();
    out.push(("gallery_ring", gallery_ring_doc(tol)));
    out
}

/// What the budget answered for one document at one request, and what
/// answering cost: the largest single rung the ladder ran (the number
/// the fit's invariant is about) and the sum over all of them (the
/// number a person waits for).
#[derive(Clone, Copy, Debug, PartialEq)]
struct Answer {
    document: &'static str,
    requested: f64,
    delta: f64,
    predicted: usize,
    requested_cost: Option<usize>,
    largest_probe: usize,
    probe_triangles: usize,
    stop: ProbeStop,
}

/// **The budget's answer is a value, and this is the whole of it** —
/// every parametric corpus document and both gallery documents, at the
/// δ the application starts on and at the decade finer where the
/// budget binds on the curved ones.
///
/// Twenty-eight documents, two requests each, and seven rows where
/// the budget binds. **The δ is the answer to a question about the
/// body and the budget**, so the work of finding it may change and
/// these may not: a row that moves is a change to what the viewer
/// opens documents at, and has to be argued as one. The forty-nine
/// rows the budget does not bind hold the tighter half of that — the
/// δ asked for, drawn as asked, priced off the very mesh a single
/// probe at [`scene::fit_delta`]'s probe factor reads.
///
/// The two counts and the stop reason are here for the same purpose:
/// what the fit costs is as much a behaviour as what it answers, and
/// a rung count that grows — or a body that stops being read as flat
/// — moves a number in this table rather than going unnoticed.
const ANSWERS: &[Answer] = &[
    Answer {
        document: "die",
        requested: 0.0001,
        delta: 0.0001,
        predicted: 348,
        requested_cost: None,
        largest_probe: 348,
        probe_triangles: 1044,
        stop: ProbeStop::Flat,
    },
    Answer {
        document: "die",
        requested: 1e-5,
        delta: 1e-5,
        predicted: 348,
        requested_cost: None,
        largest_probe: 348,
        probe_triangles: 1044,
        stop: ProbeStop::Flat,
    },
    Answer {
        document: "corner_table",
        requested: 0.0001,
        delta: 0.0001,
        predicted: 76,
        requested_cost: None,
        largest_probe: 76,
        probe_triangles: 228,
        stop: ProbeStop::Flat,
    },
    Answer {
        document: "corner_table",
        requested: 1e-5,
        delta: 1e-5,
        predicted: 76,
        requested_cost: None,
        largest_probe: 76,
        probe_triangles: 228,
        stop: ProbeStop::Flat,
    },
    Answer {
        document: "heat_sink",
        requested: 0.0001,
        delta: 0.0001,
        predicted: 152,
        requested_cost: None,
        largest_probe: 152,
        probe_triangles: 456,
        stop: ProbeStop::Flat,
    },
    Answer {
        document: "heat_sink",
        requested: 1e-5,
        delta: 1e-5,
        predicted: 152,
        requested_cost: None,
        largest_probe: 152,
        probe_triangles: 456,
        stop: ProbeStop::Flat,
    },
    Answer {
        document: "crossing_slots",
        requested: 0.0001,
        delta: 0.0001,
        predicted: 60,
        requested_cost: None,
        largest_probe: 60,
        probe_triangles: 180,
        stop: ProbeStop::Flat,
    },
    Answer {
        document: "crossing_slots",
        requested: 1e-5,
        delta: 1e-5,
        predicted: 60,
        requested_cost: None,
        largest_probe: 60,
        probe_triangles: 180,
        stop: ProbeStop::Flat,
    },
    Answer {
        document: "nested_islands_105",
        requested: 0.0001,
        delta: 0.0001,
        predicted: 60,
        requested_cost: None,
        largest_probe: 60,
        probe_triangles: 180,
        stop: ProbeStop::Flat,
    },
    Answer {
        document: "nested_islands_105",
        requested: 1e-5,
        delta: 1e-5,
        predicted: 60,
        requested_cost: None,
        largest_probe: 60,
        probe_triangles: 180,
        stop: ProbeStop::Flat,
    },
    Answer {
        document: "nested_islands_106_depth1",
        requested: 0.0001,
        delta: 0.0001,
        predicted: 32,
        requested_cost: None,
        largest_probe: 32,
        probe_triangles: 64,
        stop: ProbeStop::Flat,
    },
    Answer {
        document: "nested_islands_106_depth1",
        requested: 1e-5,
        delta: 1e-5,
        predicted: 32,
        requested_cost: None,
        largest_probe: 32,
        probe_triangles: 96,
        stop: ProbeStop::Flat,
    },
    Answer {
        document: "nested_islands_106_depth2",
        requested: 0.0001,
        delta: 0.0001,
        predicted: 44,
        requested_cost: None,
        largest_probe: 44,
        probe_triangles: 132,
        stop: ProbeStop::Flat,
    },
    Answer {
        document: "nested_islands_106_depth2",
        requested: 1e-5,
        delta: 1e-5,
        predicted: 44,
        requested_cost: None,
        largest_probe: 44,
        probe_triangles: 132,
        stop: ProbeStop::Flat,
    },
    Answer {
        document: "declared_tangency",
        requested: 0.0001,
        delta: 0.0001,
        predicted: 1600,
        requested_cost: None,
        largest_probe: 200,
        probe_triangles: 388,
        stop: ProbeStop::AtTheRequest,
    },
    Answer {
        document: "declared_tangency",
        requested: 1e-5,
        delta: 1e-5,
        predicted: 4352,
        requested_cost: None,
        largest_probe: 544,
        probe_triangles: 732,
        stop: ProbeStop::AtTheRequest,
    },
    Answer {
        document: "kitchen_sink",
        requested: 0.0001,
        delta: 0.0001,
        predicted: 3392,
        requested_cost: None,
        largest_probe: 424,
        probe_triangles: 804,
        stop: ProbeStop::AtTheRequest,
    },
    Answer {
        document: "kitchen_sink",
        requested: 1e-5,
        delta: 1e-5,
        predicted: 8032,
        requested_cost: None,
        largest_probe: 1004,
        probe_triangles: 1384,
        stop: ProbeStop::AtTheRequest,
    },
    Answer {
        document: "cut_cylinder",
        requested: 0.0001,
        delta: 0.0001,
        predicted: 36352,
        requested_cost: None,
        largest_probe: 4544,
        probe_triangles: 7848,
        stop: ProbeStop::AtTheRequest,
    },
    Answer {
        document: "cut_cylinder",
        requested: 1e-5,
        delta: 1e-5,
        predicted: 327488,
        requested_cost: None,
        largest_probe: 40936,
        probe_triangles: 44240,
        stop: ProbeStop::AtTheRequest,
    },
    Answer {
        document: "measured_web",
        requested: 0.0001,
        delta: 0.0001,
        predicted: 12,
        requested_cost: None,
        largest_probe: 12,
        probe_triangles: 24,
        stop: ProbeStop::Flat,
    },
    Answer {
        document: "measured_web",
        requested: 1e-5,
        delta: 1e-5,
        predicted: 12,
        requested_cost: None,
        largest_probe: 12,
        probe_triangles: 36,
        stop: ProbeStop::Flat,
    },
    Answer {
        document: "boss_union",
        requested: 0.0001,
        delta: 0.0001,
        predicted: 2208,
        requested_cost: None,
        largest_probe: 276,
        probe_triangles: 528,
        stop: ProbeStop::AtTheRequest,
    },
    Answer {
        document: "boss_union",
        requested: 1e-5,
        delta: 1e-5,
        predicted: 6816,
        requested_cost: None,
        largest_probe: 852,
        probe_triangles: 1104,
        stop: ProbeStop::AtTheRequest,
    },
    Answer {
        document: "die_fillet",
        requested: 0.0001,
        delta: 0.0001,
        predicted: 16608,
        requested_cost: None,
        largest_probe: 2076,
        probe_triangles: 3316,
        stop: ProbeStop::AtTheRequest,
    },
    Answer {
        document: "die_fillet",
        requested: 1e-5,
        delta: 1e-5,
        predicted: 160224,
        requested_cost: None,
        largest_probe: 20028,
        probe_triangles: 21268,
        stop: ProbeStop::AtTheRequest,
    },
    Answer {
        document: "die_chamfer",
        requested: 0.0001,
        delta: 0.0001,
        predicted: 44,
        requested_cost: None,
        largest_probe: 44,
        probe_triangles: 88,
        stop: ProbeStop::Flat,
    },
    Answer {
        document: "die_chamfer",
        requested: 1e-5,
        delta: 1e-5,
        predicted: 44,
        requested_cost: None,
        largest_probe: 44,
        probe_triangles: 132,
        stop: ProbeStop::Flat,
    },
    Answer {
        document: "die_pips",
        requested: 0.0001,
        delta: 0.0001,
        predicted: 4240,
        requested_cost: None,
        largest_probe: 530,
        probe_triangles: 576,
        stop: ProbeStop::AtTheRequest,
    },
    Answer {
        document: "die_pips",
        requested: 1e-5,
        delta: 1e-5,
        predicted: 39408,
        requested_cost: None,
        largest_probe: 4926,
        probe_triangles: 5642,
        stop: ProbeStop::AtTheRequest,
    },
    Answer {
        document: "heat_sink_fins",
        requested: 0.0001,
        delta: 0.0001,
        predicted: 60,
        requested_cost: None,
        largest_probe: 60,
        probe_triangles: 180,
        stop: ProbeStop::Flat,
    },
    Answer {
        document: "heat_sink_fins",
        requested: 1e-5,
        delta: 1e-5,
        predicted: 60,
        requested_cost: None,
        largest_probe: 60,
        probe_triangles: 180,
        stop: ProbeStop::Flat,
    },
    Answer {
        document: "die_tool",
        requested: 0.0001,
        delta: 0.0001,
        predicted: 24960,
        requested_cost: None,
        largest_probe: 3120,
        probe_triangles: 4296,
        stop: ProbeStop::AtTheRequest,
    },
    Answer {
        document: "die_tool",
        requested: 1e-5,
        delta: 1e-5,
        predicted: 235967,
        requested_cost: None,
        largest_probe: 29496,
        probe_triangles: 30672,
        stop: ProbeStop::AtTheRequest,
    },
    Answer {
        document: "face_sketch",
        requested: 0.0001,
        delta: 0.0001,
        predicted: 12,
        requested_cost: None,
        largest_probe: 12,
        probe_triangles: 24,
        stop: ProbeStop::Flat,
    },
    Answer {
        document: "face_sketch",
        requested: 1e-5,
        delta: 1e-5,
        predicted: 12,
        requested_cost: None,
        largest_probe: 12,
        probe_triangles: 36,
        stop: ProbeStop::Flat,
    },
    Answer {
        document: "part_select",
        requested: 0.0001,
        delta: 0.0001,
        predicted: 32,
        requested_cost: None,
        largest_probe: 32,
        probe_triangles: 96,
        stop: ProbeStop::Flat,
    },
    Answer {
        document: "part_select",
        requested: 1e-5,
        delta: 1e-5,
        predicted: 32,
        requested_cost: None,
        largest_probe: 32,
        probe_triangles: 96,
        stop: ProbeStop::Flat,
    },
    Answer {
        document: "die_composed",
        requested: 0.0001,
        delta: 0.0001,
        predicted: 20272,
        requested_cost: None,
        largest_probe: 2534,
        probe_triangles: 3580,
        stop: ProbeStop::AtTheRequest,
    },
    Answer {
        document: "die_composed",
        requested: 1e-5,
        delta: 1e-5,
        predicted: 177280,
        requested_cost: None,
        largest_probe: 22160,
        probe_triangles: 23206,
        stop: ProbeStop::AtTheRequest,
    },
    Answer {
        document: "die_composed_tour",
        requested: 0.0001,
        delta: 0.0001,
        predicted: 208464,
        requested_cost: None,
        largest_probe: 26058,
        probe_triangles: 29824,
        stop: ProbeStop::AtTheRequest,
    },
    Answer {
        document: "die_composed_tour",
        requested: 1e-5,
        delta: 1.954846211277421e-5,
        predicted: 1000000,
        requested_cost: Some(1954846),
        largest_probe: 119934,
        probe_triangles: 177654,
        stop: ProbeStop::Converged,
    },
    Answer {
        document: "plate_param",
        requested: 0.0001,
        delta: 0.0001,
        predicted: 3936,
        requested_cost: None,
        largest_probe: 492,
        probe_triangles: 836,
        stop: ProbeStop::AtTheRequest,
    },
    Answer {
        document: "plate_param",
        requested: 1e-5,
        delta: 1e-5,
        predicted: 11616,
        requested_cost: None,
        largest_probe: 1452,
        probe_triangles: 1796,
        stop: ProbeStop::AtTheRequest,
    },
    Answer {
        document: "kiss_carry",
        requested: 0.0001,
        delta: 0.0001,
        predicted: 48,
        requested_cost: None,
        largest_probe: 48,
        probe_triangles: 144,
        stop: ProbeStop::Flat,
    },
    Answer {
        document: "kiss_carry",
        requested: 1e-5,
        delta: 1e-5,
        predicted: 48,
        requested_cost: None,
        largest_probe: 48,
        probe_triangles: 144,
        stop: ProbeStop::Flat,
    },
    Answer {
        document: "tube_ring",
        requested: 0.0001,
        delta: 0.0001,
        predicted: 641056,
        requested_cost: None,
        largest_probe: 80132,
        probe_triangles: 89368,
        stop: ProbeStop::AtTheRequest,
    },
    Answer {
        document: "tube_ring",
        requested: 1e-5,
        delta: 6.414133008947617e-5,
        predicted: 1000000,
        requested_cost: Some(6414133),
        largest_probe: 120376,
        probe_triangles: 129612,
        stop: ProbeStop::Converged,
    },
    Answer {
        document: "tube_arc",
        requested: 0.0001,
        delta: 0.0001,
        predicted: 105023,
        requested_cost: None,
        largest_probe: 13128,
        probe_triangles: 23728,
        stop: ProbeStop::AtTheRequest,
    },
    Answer {
        document: "tube_arc",
        requested: 1e-5,
        delta: 1.0310142490965582e-5,
        predicted: 1000000,
        requested_cost: Some(1031014),
        largest_probe: 120536,
        probe_triangles: 131136,
        stop: ProbeStop::Converged,
    },
    Answer {
        document: "hollow_tube_elbow",
        requested: 0.0001,
        delta: 0.0001,
        predicted: 284192,
        requested_cost: None,
        largest_probe: 35524,
        probe_triangles: 46092,
        stop: ProbeStop::AtTheRequest,
    },
    Answer {
        document: "hollow_tube_elbow",
        requested: 1e-5,
        delta: 2.8078062883669132e-5,
        predicted: 1000000,
        requested_cost: Some(2807806),
        largest_probe: 121272,
        probe_triangles: 131840,
        stop: ProbeStop::Converged,
    },
    Answer {
        document: "hollow_tube_ring",
        requested: 0.0001,
        delta: 0.0001164492484410271,
        predicted: 1000000,
        requested_cost: Some(1164492),
        largest_probe: 120900,
        probe_triangles: 129388,
        stop: ProbeStop::Converged,
    },
    Answer {
        document: "hollow_tube_ring",
        requested: 1e-5,
        delta: 0.0001164492484410271,
        predicted: 1000000,
        requested_cost: Some(11644924),
        largest_probe: 120900,
        probe_triangles: 129388,
        stop: ProbeStop::Converged,
    },
    Answer {
        document: "gallery_ring",
        requested: 0.0001,
        delta: 0.0001,
        predicted: 165920,
        requested_cost: None,
        largest_probe: 20740,
        probe_triangles: 28960,
        stop: ProbeStop::AtTheRequest,
    },
    Answer {
        document: "gallery_ring",
        requested: 1e-5,
        delta: 1.6372398767083734e-5,
        predicted: 1000000,
        requested_cost: Some(1637239),
        largest_probe: 118840,
        probe_triangles: 127060,
        stop: ProbeStop::Converged,
    },
];

#[test]
fn the_budget_commits_the_delta_it_always_has() {
    let tol = Tol::witness();
    let mut answers: Vec<Answer> = Vec::new();
    for (document, doc) in budgeted_documents(tol) {
        let mut session = DocSession::inline(doc, tol);
        session.pump();
        let body = session
            .landed_body()
            .unwrap_or_else(|| panic!("{document} gathers"));
        for requested in [INITIAL_DELTA, OVER_BUDGET_DELTA] {
            let fitted = scene::fit_delta(body, delta(requested), tol)
                .unwrap_or_else(|error| panic!("{document} fits at {requested}: {error}"));
            // The per-rung bound, on every document the fit is ever
            // asked about: a rung is PLACED at
            // `TRIANGLE_BUDGET / PROBE_FACTOR` triangles, and what it
            // counts meets that up to the law's own error. The 1% is
            // that error and nothing else — the largest rung the
            // corpus produces is 121_272, under the placement itself.
            // Sizing a probe off the REQUEST had no bound at all: at
            // 0.01 mm `hollow_tube_ring` ran one of 1_452_960.
            #[allow(clippy::cast_precision_loss)]
            let placed = TRIANGLE_BUDGET as f64 / PROBE_FACTOR;
            #[allow(clippy::cast_precision_loss)]
            let largest = fitted.largest_probe as f64;
            assert!(
                largest <= placed * 1.01,
                "{document} at δ = {requested}: a single rung tessellated {} triangles, past \
                 the {placed} a rung is placed at by more than the law's error",
                fitted.largest_probe
            );
            answers.push(Answer {
                document,
                requested,
                delta: fitted.delta.get(),
                predicted: fitted.predicted,
                requested_cost: fitted.requested_cost,
                largest_probe: fitted.largest_probe,
                probe_triangles: fitted.probe_triangles,
                stop: fitted.stop,
            });
        }
    }
    let committed: Vec<Answer> = ANSWERS.to_vec();
    if answers != committed {
        let rows: Vec<String> = answers.iter().map(source_of).collect();
        panic!(
            "the budget answers differently than it did. What it answers now, \
             as this table's own source:\n{}",
            rows.join("\n")
        );
    }
}

/// One row as its own source, so a run that moves the table hands back
/// the table to paste.
fn source_of(a: &Answer) -> String {
    format!(
        "    Answer {{ document: {:?}, requested: {:?}, delta: {:?}, predicted: {}, \
         requested_cost: {:?}, largest_probe: {}, probe_triangles: {}, stop: ProbeStop::{:?} }},",
        a.document,
        a.requested,
        a.delta,
        a.predicted,
        a.requested_cost,
        a.largest_probe,
        a.probe_triangles,
        a.stop
    )
}

/// **No PROBE is larger than the picture it sizes**, on every document
/// the budget is asked about and at both requests.
///
/// The claim is per rung, not over the ladder: the sum across rungs is
/// a cost and can exceed a small picture (an all-planar body pays two
/// rungs of the same mesh). What cannot happen is a single
/// tessellation larger than the one the answer draws — no rung runs at
/// a δ finer than the committed one, and the count is non-increasing
/// in δ.
///
/// The defect this closes lived on the other side of the bind: a probe
/// sized off the REQUEST costs
/// `budget · δ_fitted / (PROBE_FACTOR · δ_requested)`, which passes the
/// picture the moment the budget has to coarsen δ by more than
/// `PROBE_FACTOR`. `hollow_tube_ring` at 0.01 mm tessellated 1_452_960
/// triangles to size a 1_002_536-triangle picture.
///
/// **And a flat reading is exact.** Where the ladder answers
/// [`ProbeStop::Flat`] it has ruled the 1/δ law out and taken the
/// measured count as the prediction — a claim about a δ it never
/// probed, so this row checks it against the picture that gets drawn,
/// for every flat document in the corpus.
///
/// The picture is the tessellation at the committed δ, counted through
/// `mesh::tessellate` so the row weighs triangles rather than GPU
/// vertices.
#[test]
fn no_probe_out_tessellates_the_picture_it_sizes() {
    let tol = Tol::witness();
    let mut flat_rows = 0_usize;
    for (document, doc) in budgeted_documents(tol) {
        let mut session = DocSession::inline(doc, tol);
        session.pump();
        let body = session
            .landed_body()
            .unwrap_or_else(|| panic!("{document} gathers"));
        for requested in [INITIAL_DELTA, OVER_BUDGET_DELTA] {
            let fitted = scene::fit_delta(body, delta(requested), tol)
                .unwrap_or_else(|error| panic!("{document} fits at {requested}: {error}"));
            let picture = triangles_at(body, fitted.delta.get(), tol);
            assert!(
                fitted.largest_probe <= picture,
                "{document} at δ = {requested}: a rung tessellated {} triangles to size a \
                 picture of {picture}",
                fitted.largest_probe
            );
            if fitted.stop == ProbeStop::Flat {
                flat_rows += 1;
                assert_eq!(
                    fitted.predicted, picture,
                    "{document} at δ = {requested} was read as flat, so its prediction is the \
                     count it measured — and the picture is what it actually draws"
                );
            }
        }
    }
    assert!(
        flat_rows >= 8,
        "only {flat_rows} rows were read as flat; the corpus carries a dozen all-planar \
         documents and the exactness half of this row is measuring nothing"
    );
}

/// The tour's two gallery documents — the multi-root ones the display
/// budget was written for.
fn gallery_documents(tol: Tol) -> Vec<(&'static str, ProfileDoc)> {
    let tour = corpus::documents()
        .into_iter()
        .find(|c| c.name == "die_composed_tour")
        .expect("the tour's die is a corpus document")
        .doc;
    vec![
        ("die_composed_tour", tour),
        ("gallery_ring", gallery_ring_doc(tol)),
    ]
}

/// What a body tessellates to at one δ, in triangles.
fn triangles_at(body: &pncad::topo::Body<f64>, delta: f64, tol: Tol) -> usize {
    let mesh = pncad::mesh::tessellate(body, delta, tol)
        .unwrap_or_else(|error| panic!("a body tessellates at δ = {delta}: {error:?}"));
    mesh.patches.iter().map(|patch| patch.triangles.len()).sum()
}

/// The mesh's own extent — the diagonal of its points' bounding box,
/// the way `scene::fit_delta` reads it off the scale probe.
fn extent_of(body: &pncad::topo::Body<f64>, delta: f64, tol: Tol) -> f64 {
    let mesh = pncad::mesh::tessellate(body, delta, tol).expect("a body tessellates");
    let (mut lo, mut hi) = ([f64::INFINITY; 3], [f64::NEG_INFINITY; 3]);
    for p in &mesh.positions {
        for (axis, value) in [p.x, p.y, p.z].into_iter().enumerate() {
            lo[axis] = lo[axis].min(value);
            hi[axis] = hi[axis].max(value);
        }
    }
    let span: Vec<f64> = (0..3).map(|axis| hi[axis] - lo[axis]).collect();
    (span[0].powi(2) + span[1].powi(2) + span[2].powi(2)).sqrt()
}

/// **The count a body tessellates to has stopped falling by the time δ
/// reaches the body's own extent** — the fact `scene::fit_delta` rests
/// its first rung on, since it reads a count at the scale probe's δ
/// and prices it at the extent.
///
/// So the interval this samples is the one the code pairs: the extent
/// itself, a little above it, and on up to the scale probe's own δ. A
/// body that still subdivided anywhere in there would make the first
/// rung's constant an under-count, and the rung below it dearer than
/// the ladder priced.
///
/// It also holds the other half of the fit's invariant — the count is
/// non-increasing in δ — between the coarsest rung and the picture.
#[test]
fn a_bodys_count_has_stopped_falling_by_its_own_extent() {
    let tol = Tol::witness();
    for (document, doc) in budgeted_documents(tol) {
        let mut session = DocSession::inline(doc, tol);
        session.pump();
        let body = session
            .landed_body()
            .unwrap_or_else(|| panic!("{document} gathers"));
        // The δ the scale probe runs at (`scene`'s SCALE_PROBE_DELTA,
        // which is private; the row states the number it depends on).
        let scale = 1.0e9;
        let extent = extent_of(body, scale, tol);
        assert!(
            extent > 0.0 && extent.is_finite(),
            "{document} has no extent to start a ladder at"
        );
        let floor = triangles_at(body, scale, tol);
        for delta in [extent, extent * 1.5, extent * 10.0, extent * 1.0e3] {
            assert_eq!(
                triangles_at(body, delta, tol),
                floor,
                "{document} (extent {extent} m) tessellates differently at δ = {delta} m than \
                 at the scale probe's {scale} m, so the first rung's count is not the count \
                 the ladder prices at the extent"
            );
        }
        assert!(
            floor <= triangles_at(body, INITIAL_DELTA, tol),
            "{document} tessellates to MORE triangles at its own extent than at the starting δ"
        );
    }
}

/// **A budget δ is an answer about the body and the budget, and about
/// nothing else** — ask for a decade finer and the same document opens
/// at the same δ.
///
/// That is what a fit whose probe is placed by the ANSWER buys, and it
/// is not a restatement of the table above: a probe placed by the
/// REQUEST reads the 1/δ constant at a different place for every
/// request, so the same body under the same budget answers a different
/// δ each time it is asked, and the finer the request the more it
/// tessellates to say so.
///
/// The rows are the two gallery documents at the three requests they
/// are opened at — the application's starting δ, and the two decades
/// below it — over the requests where the budget binds, which is where
/// there is an answer to be independent.
#[test]
fn a_budget_delta_does_not_depend_on_the_delta_asked_for() {
    let tol = Tol::witness();
    for (document, doc) in gallery_documents(tol) {
        let mut session = DocSession::inline(doc, tol);
        session.pump();
        let body = session
            .landed_body()
            .unwrap_or_else(|| panic!("{document} gathers"));
        let mut bound: Vec<(f64, f64)> = Vec::new();
        for requested in [INITIAL_DELTA, OVER_BUDGET_DELTA, OVER_BUDGET_DELTA / 10.0] {
            let fitted = scene::fit_delta(body, delta(requested), tol)
                .unwrap_or_else(|error| panic!("{document} fits at {requested}: {error}"));
            if fitted.requested_cost.is_some() {
                bound.push((requested, fitted.delta.get()));
            }
        }
        assert!(
            bound.len() >= 2,
            "{document} is inside the budget at every request here, so it has no answer for \
             this row to hold still"
        );
        let (first_requested, answer) = bound[0];
        for &(requested, delta) in &bound[1..] {
            assert_eq!(
                delta, answer,
                "{document} opens at {delta} when {requested} is asked for and at {answer} \
                 when {first_requested} is, though the budget and the body are the same"
            );
        }
    }
}
