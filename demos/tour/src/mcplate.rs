//! **The Monte-Carlo density plate** — the population an advisory
//! number is a summary of, drawn.
//!
//! Same document as [`crate::tolerance`] (both take it from
//! [`crate::plate`]), same study: ±0.05 mm on the hole spacing, σ =
//! 0.01 mm on each radius. That cell narrates the two lanes' NUMBERS.
//! This one draws the thing the numbers are about, and its subject is
//! the trade E11 names, at the one scale where the trade is visible:
//!
//! * the CERTIFIED lane's answer covers a box, and on this plate the
//!   widest box that certifies whole is `7.81e-7` of the study
//!   ([`crate::plate::CERTIFIABLE_FRACTION`], measured by the
//!   tolerance cell). Drawn to the zoom panel's scale that box is
//!   about a hundred-thousandth of one pixel. **A picture cannot show
//!   it. That is not a drawing problem — it is the finding.**
//! * the ADVISORY lane draws from the WHOLE distribution and reports a
//!   mean, a spread and two extremes. Those four numbers are a summary
//!   of 512 built plates, and 512 built plates are something a picture
//!   can show exactly.
//!
//! # Why it is an SVG and not a render
//!
//! Ev's constraint on this cell was latency: an overlay worth having
//! only if drawing it costs seconds, not minutes. A 3-D render of 512
//! bodies is minutes — the two montage lanes draw one STL per body
//! through an external renderer, and 512 of those in one cell is not
//! what either was built for. So this takes the `uv` lane's road
//! instead (`demos/render-uv.sh`): the geometry here is already 2-D,
//! so the tour writes the picture itself and no renderer is involved
//! at all. The whole cell is a few hundred kilobytes of text and an
//! unchanged re-run is byte-identical.
//!
//! # What is actually drawn, and what is not
//!
//! **Every circle on the sheet is a `Surface::Cylinder` read off a
//! body the kernel built** — its `origin` and its `radius`, exactly,
//! at the sample that built it. Not the parameter that produced it,
//! and not a polygonal approximation of the result: an SVG `<circle>`
//! IS the stored circle, so the drawing is a readback in
//! `fivewall`'s sense rather than a re-derivation. That is also what
//! keeps the file small — 512 samples cost 1024 elements.
//!
//! The plate's own rectangle is drawn once, because it is a literal in
//! the document and does not vary: a study whose plate outline moved
//! would be a different study, and the cell would be lying about which
//! one it drew.
//!
//! # The claim the picture makes, and how it is checked
//!
//! Not "samples from the same laws" — **this run's samples**. Each
//! one comes from `mc::sample_offsets(analyzed, config, i)`, the door
//! `work/m10`'s
//! `mc-lanes-draws-are-not-reproducible-from-outside-the-crate` asked
//! for, and the cell then holds itself to it: it summarizes its own
//! replay's web readings and requires the mean, the spread and both
//! extremes to equal `monte_carlo`'s BIT FOR BIT. If they ever differ
//! the tour fails here rather than shipping a picture of a different
//! population.
//!
//! # What was awkward to write, stated rather than smoothed over
//!
//! Per `memories/demo-purpose.md`:
//!
//! 1. **The lane's own way of placing a sample is not reachable.**
//!    `monte_carlo` puts a draw at `nominal + offset` through a
//!    degenerate `ParamBox` axis, and `ParamBox`/`BoxAxis` are
//!    `interval`-gated in the façade while `monte_carlo` is not. This
//!    cell places each draw with an ordinary
//!    `DocEdit::SetDocParamValue` instead. That the two coincide is
//!    not assumed: `m10_6_mc_draws.rs` pins it in the library, and the
//!    bit-equality above re-checks it here on real geometry. The
//!    gating half of the issue is still open.
//! 2. **A per-sample replay is a full rebuild.** There is no door for
//!    "re-evaluate this document at a different parameter value and
//!    keep everything the value does not reach": the memo is by
//!    content key, and a changed parameter changes the key of every
//!    node below it. 512 replays of this document cost about a second,
//!    which is fine here and would not be on a part with a real
//!    feature tree.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::fmt::Write as _;

use pncad::analysis::{
    AnalysisPolicy, DEFAULT_SAMPLES, McConfig, analyzed_box, monte_carlo, sample_offsets,
};
use pncad::document::{
    CancelToken, DocEdit, DocParamValue, EvalOptions, Evaluation, ParamName, ProfileDoc,
    RecipeNodeId, ValuePayload, apply, evaluate,
};
use pncad::geom::Surface;
use pncad::geom_core::Tol;
use pncad::topo::Body;

use crate::plate::{CERTIFIABLE_FRACTION, Plate, RADIUS, SPACING, WEB, plate};

/// The study the machinist writes down: ±0.05 mm on the hole spacing.
const SPACING_HALF_WIDTH: f64 = 5.0e-5;
/// …and σ = 0.01 mm on each radius. Same two numbers as the tolerance
/// cell's stop 1, because it is the same study.
const RADIUS_SIGMA: f64 = 1.0e-5;

/// Metres to millimetres, for every printed number.
const MM: f64 = 1e3;

// ---- the sheet's geometry, in px --------------------------------

const SHEET_W: f64 = 1000.0;
const SHEET_H: f64 = 640.0;
const BANNER_H: f64 = 126.0;
const PANEL_W: f64 = 470.0;
const PANEL_H: f64 = 380.0;
const PANEL_Y: f64 = BANNER_H + 10.0;
const PANEL_A_X: f64 = 16.0;
const PANEL_B_X: f64 = PANEL_A_X + PANEL_W + 28.0;

/// Panel B's window WIDTH, in metres: how much of the part the zoom
/// shows, centred on the origin (which is the web's own centre). The
/// height follows from the panel's aspect, because both axes share one
/// scale.
const ZOOM_W: f64 = 1.6e-3;

/// Per-sample stroke opacity. Chosen so that a column the cloud
/// visits ~16 times reads at about 0.4 and one it visits once is
/// nearly invisible — a density, not a silhouette.
const SAMPLE_ALPHA: f64 = 0.03;

/// One sample, as the sheet needs it: the two holes the kernel built,
/// and the web it measured.
struct Sample {
    /// `(centre_x, centre_y, radius)` per hole, read off the stored
    /// `Cylinder` of the body that sample built.
    holes: [(f64, f64, f64); 2],
    /// The web `Measure`'s value at this sample.
    web: f64,
}

/// `summarize`'s arithmetic, in the order the MC lane runs it — so the
/// comparison below is over the same reduction and a difference means
/// a difference in the DRAWS.
fn summarize(values: &[f64]) -> (f64, f64, f64, f64) {
    let n = values.len() as f64;
    let mean = values.iter().sum::<f64>() / n;
    let sigma = (values.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / (n - 1.0)).sqrt();
    let min = values.iter().copied().fold(f64::INFINITY, f64::min);
    let max = values.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    (mean, sigma, min, max)
}

/// The body a node evaluated to.
fn body_at(ev: &Evaluation<f64>, id: RecipeNodeId) -> Body<f64> {
    match &ev.value(id).expect("the node evaluated").payload {
        ValuePayload::Body(b) => (**b).clone(),
        other => panic!("a hole extrude evaluates to a body, got {other:?}"),
    }
}

/// **The hole, read off the body rather than off the parameter.** Its
/// one cylindrical face's stored `Surface::Cylinder` carries the axis
/// point and the radius; both are exact, and both are what the kernel
/// built rather than what the document asked for.
fn hole_circle(body: &Body<f64>) -> (f64, f64, f64) {
    let mut found = None;
    for (_, face) in body.faces() {
        if let Some(Surface::Cylinder { origin, radius, .. }) = body.get_surface(face.surface) {
            let seen = (origin.x, origin.y, *radius);
            if let Some(prev) = found {
                assert_eq!(
                    prev, seen,
                    "a hole's cylindrical faces are one cylinder (a seam split, not two walls)"
                );
            }
            found = Some(seen);
        }
    }
    found.expect("a hole extrude has a cylindrical wall")
}

/// The whole replay: sample `i`'s draws placed by an ordinary value
/// edit, the document evaluated there, and the two holes plus the web
/// read back out of it.
fn replay(base: &Plate, samples: usize, config: &McConfig, tol: Tol) -> Vec<Sample> {
    let analyzed = analyzed_box(&base.doc, &AnalysisPolicy::default());
    let nominal: Vec<(ParamName, f64)> = analyzed
        .varying()
        .map(|(name, p)| (name.clone(), p.nominal))
        .collect();

    (0..samples)
        .map(|i| {
            let offsets =
                sample_offsets(&analyzed, config, i).expect("the study's laws are sampleable");
            let mut doc: ProfileDoc = base.doc.clone();
            for (name, value) in &nominal {
                let offset = offsets[name];
                let applied = apply(
                    &doc,
                    &DocEdit::SetDocParamValue {
                        name: name.clone(),
                        value: DocParamValue::Continuous(value + offset),
                    },
                    tol,
                )
                .expect("a nominal moves to a drawn value");
                doc = applied.doc;
            }
            let ev: Evaluation<f64> = evaluate(
                &doc,
                None,
                &CancelToken::new(),
                &EvalOptions::default(),
                tol,
            );
            let holes = [
                hole_circle(&body_at(&ev, base.holes[0])),
                hole_circle(&body_at(&ev, base.holes[1])),
            ];
            let web = match &ev
                .value(base.measure)
                .expect("the measure evaluated")
                .payload
            {
                ValuePayload::Measure { value, .. } => *value,
                other => panic!("the web node is a measure, got {other:?}"),
            };
            Sample { holes, web }
        })
        .collect()
}

/// The tour's MC density cell: the replay, the bit-equality check
/// against the lane's own report, the narration, and the sheet.
///
/// Returns the SVG so the caller owns where it lands.
pub fn narration(tol: Tol) -> String {
    let base = plate(SPACING_HALF_WIDTH, RADIUS_SIGMA, WEB - 1.0e-4, tol);
    let analyzed = analyzed_box(&base.doc, &AnalysisPolicy::default());
    let config = McConfig {
        samples: DEFAULT_SAMPLES,
        parallel: false,
        ..McConfig::default()
    };

    let report = monte_carlo(&base.doc, &analyzed, &config, tol).expect("the nominal builds");
    let row = report
        .measures
        .iter()
        .find(|m| m.node == base.measure)
        .expect("the web measure has a row");

    let samples = replay(&base, config.samples, &config, tol);
    let webs: Vec<f64> = samples.iter().map(|s| s.web).collect();
    let (mean, sigma, min, max) = summarize(&webs);

    // **The picture is this run's population, checked rather than
    // claimed.** Four bitwise equalities: the replay drew what the
    // lane drew, evaluated where the lane evaluated, and read what the
    // lane read.
    for (ours, theirs, what) in [
        (mean, row.mean, "mean"),
        (sigma, row.sigma, "sigma"),
        (min, row.min, "min"),
        (max, row.max, "max"),
    ] {
        assert_eq!(
            ours.to_bits(),
            theirs.to_bits(),
            "the drawn population's {what} must equal the MC report's bit for bit \
             ({ours:e} vs {theirs:e}) — otherwise the sheet is a picture of a \
             different run"
        );
    }

    println!(
        "   {} samples replayed from the MC lane's own draws (seed {:#x}); the drawn \
         population's web mean, sigma, min and max equal `monte_carlo`'s BIT FOR BIT, \
         so the cloud on the sheet is this run's population and not a second one from \
         the same laws",
        config.samples, config.seed
    );
    println!(
        "   web: nominal {:.4} mm, mean {:.6} mm, sigma {:.6} mm, range [{:.6}, {:.6}] mm \
         — a {:.1}% swing on a {:.1} mm feature",
        WEB * MM,
        mean * MM,
        sigma * MM,
        min * MM,
        max * MM,
        (max - min) / WEB * 100.0,
        WEB * MM
    );
    println!(
        "   {:.4}% of the samples fell outside the analyzed box — the empirical twin of \
         the accounting's tail, and the reason the advisory lane exists: the certified \
         answer says nothing about them",
        report.outside_box * 100.0
    );

    let px_per_m_zoom = PANEL_W / ZOOM_W;
    let certified_px = CERTIFIABLE_FRACTION * SPACING_HALF_WIDTH * 2.0 * px_per_m_zoom;
    let cloud_px = (max - min) * px_per_m_zoom;
    println!(
        "   drawn to the zoom panel's scale ({:.0} px/mm) the study's cloud is {:.0} px \
         wide and the widest box that CERTIFIES whole is {:.1e} px — the trade E11 names, \
         at the one scale where a picture can carry it",
        px_per_m_zoom / MM,
        cloud_px,
        certified_px
    );

    sheet(&samples, mean, sigma, min, max, report.outside_box, &config)
}

// ---- the sheet ---------------------------------------------------

fn esc(s: &str) -> String {
    s.replace('&', "&amp;").replace('<', "&lt;")
}

fn text(out: &mut String, x: f64, y: f64, size: f64, fill: &str, weight: &str, s: &str) {
    let _ = writeln!(
        out,
        r##"<text x="{x:.1}" y="{y:.1}" font-family="DejaVu Sans, Helvetica, sans-serif" font-size="{size}" font-weight="{weight}" fill="{fill}">{}</text>"##,
        esc(s)
    );
}

/// One panel: a framed window on the part, at its own scale.
///
/// `half_w`/`half_h` are the window's half-extent in metres about the
/// origin — which is the plate's centre and the web's centre at once,
/// so both panels share a centre and only the scale differs.
struct Panel {
    x: f64,
    y: f64,
    half_w: f64,
}

impl Panel {
    /// One scale for both axes, deliberately: a panel that stretched
    /// x against y would turn every circle here into an ellipse and
    /// the readback into a drawing.
    fn px_per_m(&self) -> f64 {
        PANEL_W / (2.0 * self.half_w)
    }

    fn map(&self, mx: f64, my: f64) -> (f64, f64) {
        let s = self.px_per_m();
        // +y is up in the part and down in SVG, so the vertical axis
        // flips here and nowhere else.
        (
            self.x + PANEL_W / 2.0 + mx * s,
            self.y + PANEL_H / 2.0 - my * s,
        )
    }

    /// The panel's own clip id — one per panel, derived from its x so
    /// the two can never collide.
    fn clip_id(&self) -> String {
        format!("panel{}", self.x as i64)
    }

    /// Open the panel's clipped group. **Load-bearing rather than
    /// tidy**: the zoom panel's window is under two millimetres across
    /// and the circles drawn on it are wider than that, so without a
    /// clip the cloud runs over the neighbouring panel and off the
    /// sheet. SVG has no per-element viewport to stop it.
    fn open(&self, out: &mut String) {
        let _ = writeln!(out, r##"<g clip-path="url(#{})">"##, self.clip_id());
    }

    fn close(out: &mut String) {
        out.push_str("</g>\n");
    }

    fn frame(&self, out: &mut String, title: &str, subtitle: &str) {
        let _ = writeln!(
            out,
            r##"<clipPath id="{}"><rect x="{:.1}" y="{:.1}" width="{PANEL_W}" height="{PANEL_H}"/></clipPath>"##,
            self.clip_id(),
            self.x,
            self.y
        );
        let _ = writeln!(
            out,
            r##"<rect x="{:.1}" y="{:.1}" width="{PANEL_W}" height="{PANEL_H}" fill="#ffffff" stroke="#c9c9c9" stroke-width="1"/>"##,
            self.x, self.y
        );
        text(out, self.x, self.y - 30.0, 15.0, "#1a1a1a", "bold", title);
        text(
            out,
            self.x,
            self.y - 12.0,
            11.5,
            "#555555",
            "normal",
            subtitle,
        );
    }

    /// Every sample's two circles, at the density opacity.
    fn cloud(&self, out: &mut String, samples: &[Sample]) {
        let s = self.px_per_m();
        let _ = writeln!(
            out,
            r##"<g fill="none" stroke="#1f4e9c" stroke-width="1" stroke-opacity="{SAMPLE_ALPHA}">"##
        );
        for sample in samples {
            for &(cx, cy, r) in &sample.holes {
                let (px, py) = self.map(cx, cy);
                let _ = writeln!(
                    out,
                    r##"<circle cx="{px:.3}" cy="{py:.3}" r="{:.3}"/>"##,
                    r * s
                );
            }
        }
        out.push_str("</g>\n");
    }

    /// The nominal, crisp, so the cloud has something to be a cloud
    /// AROUND.
    fn nominal(&self, out: &mut String) {
        let s = self.px_per_m();
        for sign in [-1.0, 1.0] {
            let (px, py) = self.map(sign * SPACING / 2.0, 0.0);
            let _ = writeln!(
                out,
                r##"<circle cx="{px:.3}" cy="{py:.3}" r="{:.3}" fill="none" stroke="#c2410c" stroke-width="1.1" stroke-dasharray="4 3"/>"##,
                RADIUS * s
            );
        }
    }

    /// The plate's own outline — a literal in the document, so it is
    /// drawn once and the panel's clip does the cutting when the zoom
    /// runs past it.
    fn outline(&self, out: &mut String) {
        let (x0, y0) = self.map(-4.0e-3, 2.0e-3);
        let (x1, y1) = self.map(4.0e-3, -2.0e-3);
        let _ = writeln!(
            out,
            r##"<rect x="{x0:.2}" y="{y0:.2}" width="{:.2}" height="{:.2}" fill="none" stroke="#7a7a7a" stroke-width="1.2"/>"##,
            x1 - x0,
            y1 - y0
        );
    }

    /// The web itself, dimensioned between the two NOMINAL rims — so
    /// that the quantity every number on the sheet is about has a
    /// picture too, and a reader can see that the cloud's spread is a
    /// large fraction of it rather than being told.
    fn web_dim(&self, out: &mut String) {
        let (x0, y) = self.map(-SPACING / 2.0 + RADIUS, 0.0);
        let (x1, _) = self.map(SPACING / 2.0 - RADIUS, 0.0);
        let _ = writeln!(
            out,
            r##"<line x1="{x0:.1}" y1="{y:.1}" x2="{x1:.1}" y2="{y:.1}" stroke="#c2410c" stroke-width="1"/>"##
        );
        for x in [x0, x1] {
            let _ = writeln!(
                out,
                r##"<line x1="{x:.1}" y1="{:.1}" x2="{x:.1}" y2="{:.1}" stroke="#c2410c" stroke-width="1"/>"##,
                y - 6.0,
                y + 6.0
            );
        }
        text(
            out,
            (x0 + x1) / 2.0 - 46.0,
            y - 10.0,
            11.5,
            "#c2410c",
            "normal",
            &format!("web {:.2} mm", WEB * MM),
        );
    }

    /// A scale bar, so the two panels' zoom ratio is readable off the
    /// sheet rather than only stated in the caption.
    fn scale_bar(&self, out: &mut String, metres: f64, label: &str) {
        let s = self.px_per_m();
        let w = metres * s;
        let (x, y) = (self.x + 14.0, self.y + PANEL_H - 18.0);
        let _ = writeln!(
            out,
            r##"<line x1="{x:.1}" y1="{y:.1}" x2="{:.1}" y2="{y:.1}" stroke="#1a1a1a" stroke-width="2"/>"##,
            x + w
        );
        text(out, x, y - 6.0, 11.0, "#1a1a1a", "normal", label);
    }
}

#[allow(clippy::too_many_arguments)]
fn sheet(
    samples: &[Sample],
    mean: f64,
    sigma: f64,
    min: f64,
    max: f64,
    outside_box: f64,
    config: &McConfig,
) -> String {
    let mut out = String::new();
    let _ = writeln!(
        out,
        r##"<svg xmlns="http://www.w3.org/2000/svg" width="{SHEET_W}" height="{SHEET_H}" viewBox="0 0 {SHEET_W} {SHEET_H}">"##
    );
    let _ = writeln!(
        out,
        r##"<rect width="{SHEET_W}" height="{SHEET_H}" fill="#fbfbfb"/>"##
    );

    text(
        &mut out,
        PANEL_A_X,
        30.0,
        19.0,
        "#1a1a1a",
        "bold",
        "the two-hole plate: the population an advisory number summarizes",
    );
    text(
        &mut out,
        PANEL_A_X,
        50.0,
        12.0,
        "#444444",
        "normal",
        &format!(
            "{} Monte-Carlo samples, seed {:#x}  \u{2014}  ADVISORY (E11.1): every number here is an estimate over draws, and none of it gates",
            config.samples, config.seed
        ),
    );
    text(
        &mut out,
        PANEL_A_X,
        70.0,
        12.0,
        "#444444",
        "normal",
        &format!(
            "study: hole spacing {:.2} mm \u{00b1}{:.2} mm (uniform), each radius {:.2} mm at \u{03c3} = {:.2} mm  \u{2014}  web = spacing \u{2212} 2r, nominal {:.2} mm",
            SPACING * MM,
            SPACING_HALF_WIDTH * MM,
            RADIUS * MM,
            RADIUS_SIGMA * MM,
            WEB * MM
        ),
    );
    text(
        &mut out,
        PANEL_A_X,
        90.0,
        12.0,
        "#444444",
        "normal",
        &format!(
            "web over the draws: mean {:.5} mm, \u{03c3} {:.5} mm, range [{:.5}, {:.5}] mm  \u{2014}  {:.2}% of draws fell outside the analyzed box",
            mean * MM,
            sigma * MM,
            min * MM,
            max * MM,
            outside_box * 100.0
        ),
    );

    let a = Panel {
        x: PANEL_A_X,
        y: PANEL_Y,
        half_w: 4.6e-3,
    };
    a.frame(
        &mut out,
        "the part",
        &format!(
            "{:.0} px/mm \u{2014} all {} samples are here, and the \u{00b1}{:.2} mm study is {:.1} px wide",
            a.px_per_m() / MM,
            samples.len(),
            SPACING_HALF_WIDTH * MM,
            2.0 * SPACING_HALF_WIDTH * a.px_per_m()
        ),
    );
    a.open(&mut out);
    a.outline(&mut out);
    a.cloud(&mut out, samples);
    a.nominal(&mut out);
    Panel::close(&mut out);
    a.scale_bar(&mut out, 2.0e-3, "2 mm");

    let b = Panel {
        x: PANEL_B_X,
        y: PANEL_Y,
        half_w: ZOOM_W / 2.0,
    };
    let certified_px = CERTIFIABLE_FRACTION * SPACING_HALF_WIDTH * 2.0 * b.px_per_m();
    b.frame(
        &mut out,
        "the web, magnified",
        &format!(
            "{:.0} px/mm \u{2014} the same {} samples, {:.0} px of cloud on a {:.2} mm web",
            b.px_per_m() / MM,
            samples.len(),
            (max - min) * b.px_per_m(),
            WEB * MM
        ),
    );
    b.open(&mut out);
    b.outline(&mut out);
    b.cloud(&mut out, samples);
    b.nominal(&mut out);
    b.web_dim(&mut out);
    Panel::close(&mut out);
    b.scale_bar(&mut out, 2.5e-4, "0.25 mm");

    let legend_y = PANEL_Y + PANEL_H + 26.0;
    text(
        &mut out,
        PANEL_A_X,
        legend_y,
        12.0,
        "#1f4e9c",
        "bold",
        &format!(
            "blue: one stroke per sample per hole, at {SAMPLE_ALPHA} opacity \u{2014} each is a Surface::Cylinder read off the body that sample built"
        ),
    );
    text(
        &mut out,
        PANEL_A_X,
        legend_y + 17.0,
        12.0,
        "#c2410c",
        "bold",
        "dashed orange: the nominal",
    );
    text(
        &mut out,
        PANEL_A_X,
        legend_y + 38.0,
        12.0,
        "#1a1a1a",
        "normal",
        &format!(
            "CERTIFIED: exact over a box, and silent outside it. The widest box that certifies THIS plate whole is {CERTIFIABLE_FRACTION:e} of the study",
        ),
    );
    text(
        &mut out,
        PANEL_A_X,
        legend_y + 55.0,
        12.0,
        "#1a1a1a",
        "normal",
        &format!(
            "\u{2014} {certified_px:.1e} px at the right-hand panel's scale. A picture cannot show it, and that is the finding rather than a drawing problem.",
        ),
    );
    text(
        &mut out,
        PANEL_A_X,
        legend_y + 72.0,
        12.0,
        "#1a1a1a",
        "normal",
        &format!(
            "ADVISORY: draws from the whole distribution, tail included. Its four numbers summarize these {} plates \u{2014} which is what the panel is.",
            samples.len()
        ),
    );

    out.push_str("</svg>\n");
    out
}
