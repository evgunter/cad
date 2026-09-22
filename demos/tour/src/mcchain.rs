//! **The chain's Monte-Carlo density sheet** — an angular error at
//! four joints, and what it does to the link that has to reach the
//! target.
//!
//! Same document as [`crate::chaintol`] (both take it from
//! [`crate::chain`]), same study: σ = 0.01 rad at every joint,
//! independently. That cell measures what the CERTIFIED lane can say
//! about it. This one draws the population the advisory number is a
//! summary of, and its subject is the one Ev asked for: the dispersion
//! GROWS down the chain, because joint `j`'s error is a rotation of
//! everything below it and the last link has the longest lever.
//!
//! The growth is a prediction, not an impression. With one law at
//! every joint the lateral spread at pin `k` is `L·σ·sqrt(Σ (k−j)²)`
//! over the joints above it, which is `1 : 2.24 : 3.74 : 5.48` for
//! four links — and the cell prints the MEASURED spread beside it, so
//! the sheet's fan is checked against arithmetic rather than admired.
//!
//! # What is actually drawn, and what is not
//!
//! **Every polygon on the sheet is the outline of a bar the kernel
//! built**, walked off that body's own cap face: its outer loop's
//! half-edges in traversal order, each half-edge's start vertex read
//! back through `topo::readback::vertex_point`, projected to the
//! sketch plane. Not the placement the parameters imply, and not a
//! re-derivation of the forward kinematics — if the transform stack
//! moved a link somewhere else, the sheet would draw it there.
//! Likewise every pin is a `Surface::Cylinder` read off the body that
//! sample built, exactly as [`crate::mcplate`]'s holes are.
//!
//! The TARGET pin is drawn once, because it is a literal in the
//! document and does not vary: it is the datum the tip's position is
//! measured against, and a target that moved would be a different
//! study.
//!
//! # Why a second sheet module rather than one generalised drawing
//!
//! [`crate::mcplate`] is untouched by this unit, which is the unit's
//! own scope line and not an accident of laziness: its sheet is a
//! COMMITTED artifact that has to stay byte-identical, and a shared
//! drawing module would put every future edit to this cell's layout
//! one refactor away from moving the plate's pixels. What the two
//! would actually share is `esc`, `text` and a metres-to-pixels map —
//! about sixty lines — while their subjects share nothing: the plate
//! draws two circles per sample at one scale and dimensions a web;
//! this draws four walked polygons and five pins per sample, at a
//! forty-eight millimetre extent, and dimensions a spread per joint.
//! Two modules, and the duplication named rather than hidden.
//!
//! # The claim the picture makes, and how it is checked
//!
//! Not "samples from the same laws" — **this run's samples**. Each one
//! comes from `mc::sample_offsets(analyzed, config, i)`, and the cell
//! then holds itself to it: it summarizes its own replay's tip-position
//! readings and requires the mean, the spread and both extremes to
//! equal `monte_carlo`'s BIT FOR BIT. If they ever differ the tour
//! fails here rather than shipping a picture of a different population.
//!
//! # What was awkward to write, stated rather than smoothed over
//!
//! Per `memories/demo-purpose.md`:
//!
//! 1. **A link's outline has no read-back door.** There is
//!    `readback::face_pose` for a face's frame and
//!    `readback::vertex_point` for a vertex, and nothing in between:
//!    "give me this planar face's boundary points" is assembled here
//!    out of `face.outer`, `Body::get_loop`, `LoopBoundary::Cycle`,
//!    `Body::loop_cycle`, `Body::get_half_edge` and the vertex door —
//!    six calls and a hand-written traversal, at the demo layer, to
//!    ask a solid modeller where the corners of a rectangle went.
//!    [`crate::uvdump`] walks the same six for its chart images.
//! 2. **Which face is the cap is the consumer's problem.** The bars
//!    are placed by rotations about `+z`, so a cap's normal is `±z`
//!    and every side's lies in the sketch plane — two classes that are
//!    exactly known, not measured. The cell separates them at `|n_z| >
//!    1/2`, which is a classification of a known dichotomy rather than
//!    a tolerance decision, and says so here because an unexplained
//!    float threshold in a demo is indistinguishable from one.
//! 3. **A per-sample replay is a full rebuild** — [`crate::mcplate`]'s
//!    note, with a longer document under it: a changed joint angle
//!    changes the content key of every node below it, which for this
//!    chain is most of the chain.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::fmt::Write as _;

use pncad::analysis::{
    AnalysisPolicy, DEFAULT_SAMPLES, McConfig, analyzed_box, monte_carlo, sample_offsets,
};
use pncad::document::{
    CancelToken, DocEdit, DocParamValue, EvalOptions, Evaluation, ParamName, ProfileDoc,
    RecipeNodeId, RefusingReach, ValuePayload, apply, evaluate,
};
use pncad::geom::Surface;
use pncad::geom_core::Tol;
use pncad::topo::{Body, LoopBoundary};

use crate::chain::{
    CERTIFIABLE_FRACTION, CERTIFIED_PIN_BOX, Chain, JOINT_SIGMA, LINK_HEIGHT, LINK_LENGTH, LINKS,
    PIN_RADIUS, POSITION_BOUND, chain,
};

/// Metres to millimetres, for every printed number.
const MM: f64 = 1e3;

// ---- the sheet's geometry, in px --------------------------------

const SHEET_W: f64 = 1120.0;
const SHEET_H: f64 = 920.0;
const WIDE_W: f64 = 1088.0;
const WIDE_H: f64 = 230.0;
const ZOOM_W_PX: f64 = 380.0;
const ZOOM_H_PX: f64 = 340.0;
const MARGIN_X: f64 = 16.0;
const WIDE_Y: f64 = 148.0;
const ZOOM_Y: f64 = WIDE_Y + WIDE_H + 60.0;
/// Where the per-joint table's columns start, in px from the left.
const COL_X: [f64; 7] = [0.0, 40.0, 104.0, 210.0, 352.0, 414.0, 480.0];

/// The tip panel's window WIDTH, in metres: how much of the part the
/// zoom shows, centred on the NOMINAL tip. Sized to hold the whole
/// cloud plus the 1 mm position band with room to read it.
const ZOOM_WINDOW: f64 = 8.0e-3;

/// Per-sample stroke opacity, chosen so a column the fan visits a
/// dozen times reads as grey and one it visits once is nearly
/// invisible — a density, not a silhouette.
const SAMPLE_ALPHA: f64 = 0.04;

/// The floor width, in px, a certified box is drawn at — see
/// `Panel::certified_box` for why it has one.
const CERTIFIED_MIN_PX: f64 = 5.0;

/// One sample, as the sheet needs it: the bars and pins the kernel
/// built, and the tip position it measured.
struct Sample {
    /// Per link, its cap face's corners in the sketch plane, in the
    /// loop's own traversal order.
    bars: Vec<Vec<(f64, f64)>>,
    /// Per joint pin (base first, tip last), its centre — the stored
    /// `Cylinder`'s axis point of the body that sample built.
    pins: Vec<(f64, f64)>,
    /// The tip-position `Measure`'s value at this sample.
    position: f64,
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
        other => panic!("a placed link evaluates to a body, got {other:?}"),
    }
}

/// **The pin, read off the body rather than off the placement.** Its
/// one cylindrical face's stored `Surface::Cylinder` carries the axis
/// point; it is exact, and it is what the kernel built rather than
/// what the transform stack was asked for.
fn pin_centre(body: &Body<f64>) -> (f64, f64) {
    let mut found = None;
    for (_, face) in body.faces() {
        if let Some(Surface::Cylinder { origin, .. }) = body.get_surface(face.surface) {
            let seen = (origin.x, origin.y);
            if let Some(prev) = found {
                assert_eq!(
                    prev, seen,
                    "a pin's cylindrical faces are one cylinder (a seam split, not two walls)"
                );
            }
            found = Some(seen);
        }
    }
    found.expect("a pin extrude has a cylindrical wall")
}

/// **The bar's outline, walked off the body it was built into.** The
/// cap face — the one whose normal is `±z`, at the lower `z` of the
/// two — then its outer loop's cycle, then each half-edge's start
/// vertex, projected to the sketch plane.
fn bar_outline(body: &Body<f64>) -> Vec<(f64, f64)> {
    let mut cap: Option<(f64, Vec<(f64, f64)>)> = None;
    for (_, face) in body.faces() {
        let Some(Surface::Plane { origin, normal, .. }) = body.get_surface(face.surface) else {
            continue;
        };
        // The caps' normal is `+z` or `-z` and every side's lies in the
        // sketch plane, so this separates two exactly-known classes.
        if normal.z.abs() <= 0.5 {
            continue;
        }
        let lp = body.get_loop(face.outer).expect("the face's outer loop");
        let LoopBoundary::Cycle { first } = lp.boundary else {
            panic!("a bar's cap face bounds a cycle, not a lone vertex");
        };
        let cycle = body.loop_cycle(first).expect("the cap's loop closes");
        let corners: Vec<(f64, f64)> = cycle
            .iter()
            .map(|hek| {
                let he = body.get_half_edge(*hek).expect("a live half-edge");
                let p = pncad::topo::readback::vertex_point(body, he.start)
                    .expect("a corner vertex has a stored point");
                (p.x, p.y)
            })
            .collect();
        if cap.as_ref().is_none_or(|(z, _)| origin.z < *z) {
            cap = Some((origin.z, corners));
        }
    }
    let (_, corners) = cap.expect("an extruded bar has a cap face");
    assert_eq!(
        corners.len(),
        4,
        "the bar's cap is the extruded rectangle, four corners"
    );
    corners
}

/// The whole replay: sample `i`'s draws placed by an ordinary value
/// edit, the document evaluated there, and every bar, pin and the
/// tip's position read back out of it.
fn replay(base: &Chain, samples: usize, config: &McConfig, tol: Tol) -> Vec<Sample> {
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
                    &RefusingReach,
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
            let bars = base
                .bars
                .iter()
                .map(|id| bar_outline(&body_at(&ev, *id)))
                .collect();
            let pins = base
                .pins
                .iter()
                .map(|id| pin_centre(&body_at(&ev, *id)))
                .collect();
            let position = match &ev
                .value(base.measure)
                .expect("the measure evaluated")
                .payload
            {
                ValuePayload::Measure { value, .. } => *value,
                other => panic!("the tip-position node is a measure, got {other:?}"),
            };
            Sample {
                bars,
                pins,
                position,
            }
        })
        .collect()
}

/// One joint pin's dispersion over the run, in metres.
struct Spread {
    /// The pin's index along the chain: 0 is the base pin, `LINKS` the
    /// tip.
    index: usize,
    /// The nominal position, which is where the pin sits with every
    /// joint at zero.
    nominal: (f64, f64),
    /// σ of the LATERAL deviation — the component across the chain's
    /// nominal axis, which is what "dispersed" means here.
    sigma_y: f64,
    /// The lateral deviation's extremes.
    lo: f64,
    hi: f64,
}

/// The per-pin spreads, measured over the replay.
fn spreads(samples: &[Sample]) -> Vec<Spread> {
    let pins = samples[0].pins.len();
    (0..pins)
        .map(|index| {
            let nominal = (index as f64 * LINK_LENGTH, 0.0);
            let ys: Vec<f64> = samples.iter().map(|s| s.pins[index].1).collect();
            let (_, sigma_y, lo, hi) = summarize(&ys);
            Spread {
                index,
                nominal,
                sigma_y,
                lo,
                hi,
            }
        })
        .collect()
}

/// The tour's chain density cell: the replay, the bit-equality check
/// against the lane's own report, the narration, and the sheet.
///
/// Returns the SVG so the caller owns where it lands.
pub fn narration(tol: Tol) -> String {
    let base = chain(LINKS, JOINT_SIGMA, POSITION_BOUND, tol);
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
        .expect("the tip-position measure has a row");
    let assertion = report
        .assertions
        .iter()
        .find(|a| a.node == base.assertion)
        .expect("the position assertion has a row");

    let samples = replay(&base, config.samples, &config, tol);
    let positions: Vec<f64> = samples.iter().map(|s| s.position).collect();
    let (mean, sigma, min, max) = summarize(&positions);

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

    let spreads = spreads(&samples);
    // **The claim the sheet is FOR, asserted.** A transform stack
    // nested the other way round would put every joint's error on its
    // own link only, and the fan would be the same width at every pin;
    // this is the runtime value that would make that false.
    for pair in spreads[1..].windows(2) {
        assert!(
            pair[1].sigma_y > pair[0].sigma_y,
            "the dispersion must GROW down the chain — pin {} spreads {:e} m and pin {} \
             spreads {:e} m, which is not more. A joint that moved only its own link \
             would look exactly like this.",
            pair[0].index + 1,
            pair[0].sigma_y,
            pair[1].index + 1,
            pair[1].sigma_y
        );
    }
    println!(
        "   {} samples replayed from the MC lane's own draws (seed {:#x}); the drawn \
         population's tip-position mean, sigma, min and max equal `monte_carlo`'s BIT FOR \
         BIT, so the fan on the sheet is this run's population and not a second one from \
         the same laws",
        config.samples, config.seed
    );
    println!(
        "   study: {LINKS} links of {:.0} mm, σ = {JOINT_SIGMA} rad ({:.3}°) at every joint, \
         independently",
        LINK_LENGTH * MM,
        JOINT_SIGMA.to_degrees()
    );
    // Pin 1 is the base joint: it sits at the world origin under every
    // draw, so its row would be four zeros and a ratio against itself.
    for s in spreads.iter().skip(1) {
        println!(
            "   pin {} (x = {:5.1} mm): lateral σ {:.4} mm, range [{:+.4}, {:+.4}] mm \
             — {:.2}× pin 2's, the accumulation law says {:.2}×",
            s.index + 1,
            s.nominal.0 * MM,
            s.sigma_y * MM,
            s.lo * MM,
            s.hi * MM,
            ratio(s.sigma_y, spreads[1].sigma_y),
            ratio(predicted_sigma(s.index), predicted_sigma(1)),
        );
    }
    println!(
        "   tip position against the target: mean {:.4} mm, sigma {:.4} mm, range \
         [{:.4}, {:.4}] mm, asserted <= {:.4} mm",
        mean * MM,
        sigma * MM,
        min * MM,
        max * MM,
        POSITION_BOUND * MM
    );
    println!(
        "   advisory yield: {} of {} samples hold, {} violate ({:.2}% violated, decided), \
         {} undecided",
        assertion.holds,
        config.samples,
        assertion.violated,
        assertion.violation_fraction().unwrap_or(f64::NAN) * 100.0,
        assertion.unevaluated
    );
    println!(
        "   {:.4}% of the samples fell outside the analyzed box — the empirical twin of \
         the accounting's tail, and the reason the advisory lane exists: the certified \
         answer says nothing about them",
        report.outside_box * 100.0
    );

    sheet(
        &samples,
        &spreads,
        mean,
        sigma,
        min,
        max,
        &report,
        assertion.holds,
        assertion.violated,
        &config,
    )
}

/// The lateral σ the accumulation law predicts at pin `index`, in
/// metres: joint `j` turns everything below it, so pin `index` picks
/// up `(index − j)` link lengths of lever from joint `j + 1`.
fn predicted_sigma(index: usize) -> f64 {
    let quadrature: f64 = (0..index).map(|j| ((index - j) as f64).powi(2)).sum();
    LINK_LENGTH * JOINT_SIGMA * quadrature.sqrt()
}

fn ratio(a: f64, b: f64) -> f64 {
    if b == 0.0 { f64::NAN } else { a / b }
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

/// One panel: a framed window on the part, at its own scale, centred
/// where the panel's subject is.
struct Panel {
    x: f64,
    y: f64,
    w: f64,
    h: f64,
    /// The window's centre in metres — the chain is 48 mm long and its
    /// tip is at one end of it, so the two panels do NOT share a
    /// centre the way the plate's do.
    centre: (f64, f64),
    /// The window's half-width in metres.
    half_w: f64,
}

impl Panel {
    /// One scale for both axes, deliberately: a panel that stretched x
    /// against y would turn the fan into a shape the geometry does not
    /// have.
    fn px_per_m(&self) -> f64 {
        self.w / (2.0 * self.half_w)
    }

    fn map(&self, mx: f64, my: f64) -> (f64, f64) {
        let s = self.px_per_m();
        // +y is up in the part and down in SVG, so the vertical axis
        // flips here and nowhere else.
        (
            self.x + self.w / 2.0 + (mx - self.centre.0) * s,
            self.y + self.h / 2.0 - (my - self.centre.1) * s,
        )
    }

    /// The panel's own clip id — one per panel, derived from its y so
    /// the two can never collide.
    fn clip_id(&self) -> String {
        format!("panel{}", self.y as i64)
    }

    /// Open the panel's clipped group. **Load-bearing rather than
    /// tidy**: the tip panel's window is five millimetres across and a
    /// bar drawn on it is twelve, so without a clip the fan runs off
    /// the sheet.
    fn open(&self, out: &mut String) {
        let _ = writeln!(out, r##"<g clip-path="url(#{})">"##, self.clip_id());
    }

    fn close(out: &mut String) {
        out.push_str("</g>\n");
    }

    fn frame(&self, out: &mut String, title: &str, subtitle: &str) {
        let _ = writeln!(
            out,
            r##"<clipPath id="{}"><rect x="{:.1}" y="{:.1}" width="{:.1}" height="{:.1}"/></clipPath>"##,
            self.clip_id(),
            self.x,
            self.y,
            self.w,
            self.h
        );
        let _ = writeln!(
            out,
            r##"<rect x="{:.1}" y="{:.1}" width="{:.1}" height="{:.1}" fill="#ffffff" stroke="#c9c9c9" stroke-width="1"/>"##,
            self.x, self.y, self.w, self.h
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

    /// Every sample's link outlines, at the density opacity — the fan
    /// itself.
    fn fan(&self, out: &mut String, samples: &[Sample]) {
        let _ = writeln!(
            out,
            r##"<g fill="none" stroke="#1f4e9c" stroke-width="0.8" stroke-opacity="{SAMPLE_ALPHA}">"##
        );
        for sample in samples {
            for corners in &sample.bars {
                let pts: Vec<String> = corners
                    .iter()
                    .map(|&(mx, my)| {
                        let (px, py) = self.map(mx, my);
                        format!("{px:.2},{py:.2}")
                    })
                    .collect();
                let _ = writeln!(out, r##"<polygon points="{}"/>"##, pts.join(" "));
            }
        }
        out.push_str("</g>\n");
    }

    /// Every sample's pins, at the density opacity.
    fn pin_cloud(&self, out: &mut String, samples: &[Sample], pins: std::ops::Range<usize>) {
        let s = self.px_per_m();
        let _ = writeln!(
            out,
            r##"<g fill="none" stroke="#7c2d92" stroke-width="0.8" stroke-opacity="{SAMPLE_ALPHA}">"##
        );
        for sample in samples {
            for &(cx, cy) in &sample.pins[pins.clone()] {
                let (px, py) = self.map(cx, cy);
                let _ = writeln!(
                    out,
                    r##"<circle cx="{px:.3}" cy="{py:.3}" r="{:.3}"/>"##,
                    PIN_RADIUS * s
                );
            }
        }
        out.push_str("</g>\n");
        // …and its CENTRE, which is the quantity the study is about.
        // On the tip panel five hundred overlapping pin circles are one
        // blob; the axis points are the scatter inside it.
        let _ = writeln!(
            out,
            r##"<g fill="#7c2d92" fill-opacity="{:.3}" stroke="none">"##,
            SAMPLE_ALPHA * 6.0
        );
        for sample in samples {
            for &(cx, cy) in &sample.pins[pins.clone()] {
                let (px, py) = self.map(cx, cy);
                let _ = writeln!(out, r##"<circle cx="{px:.3}" cy="{py:.3}" r="1.3"/>"##);
            }
        }
        out.push_str("</g>\n");
    }

    /// The nominal chain — every joint at zero — crisp, so the fan has
    /// something to be a fan AROUND.
    ///
    /// `bars` is off on a panel whose window is narrower than one bar,
    /// where a twelve-millimetre rectangle draws as a single edge
    /// across the whole panel; the pins are always drawn, because they
    /// are the thing every other mark on the sheet is measured from.
    fn nominal(&self, out: &mut String, bars: bool, links: usize) {
        let h = LINK_HEIGHT / 2.0;
        let s = self.px_per_m();
        if bars {
            for k in 0..links {
                let (x0, y0) = self.map(k as f64 * LINK_LENGTH, h);
                let (x1, y1) = self.map((k + 1) as f64 * LINK_LENGTH, -h);
                let _ = writeln!(
                    out,
                    r##"<rect x="{x0:.2}" y="{y0:.2}" width="{:.2}" height="{:.2}" fill="none" stroke="#c2410c" stroke-width="1.1" stroke-dasharray="4 3"/>"##,
                    x1 - x0,
                    y1 - y0
                );
            }
        }
        for k in 0..=links {
            let (px, py) = self.map(k as f64 * LINK_LENGTH, 0.0);
            let _ = writeln!(
                out,
                r##"<circle cx="{px:.3}" cy="{py:.3}" r="{:.3}" fill="none" stroke="#c2410c" stroke-width="1.1" stroke-dasharray="4 3"/>"##,
                PIN_RADIUS * s
            );
        }
    }

    /// The target pin and the position band around it — the
    /// requirement, drawn where it is asserted.
    fn target(&self, out: &mut String, bound: f64) {
        let s = self.px_per_m();
        let (px, py) = self.map(LINKS as f64 * LINK_LENGTH, 0.0);
        let _ = writeln!(
            out,
            r##"<circle cx="{px:.3}" cy="{py:.3}" r="{:.3}" fill="none" stroke="#15803d" stroke-width="1.6"/>"##,
            PIN_RADIUS * s
        );
        let _ = writeln!(
            out,
            r##"<circle cx="{px:.3}" cy="{py:.3}" r="{:.3}" fill="none" stroke="#15803d" stroke-width="1.2" stroke-dasharray="6 4"/>"##,
            bound * s
        );
    }

    /// The spread at one joint, DIMENSIONED: the measured lateral
    /// range as a bar with end ticks, labeled with its σ.
    fn spread_dim(&self, out: &mut String, s: &Spread) {
        let (x, y_hi) = self.map(s.nominal.0, s.hi);
        let (_, y_lo) = self.map(s.nominal.0, s.lo);
        let _ = writeln!(
            out,
            r##"<line x1="{x:.1}" y1="{y_hi:.1}" x2="{x:.1}" y2="{y_lo:.1}" stroke="#7c2d92" stroke-width="1.2"/>"##
        );
        for y in [y_hi, y_lo] {
            let _ = writeln!(
                out,
                r##"<line x1="{:.1}" y1="{y:.1}" x2="{:.1}" y2="{y:.1}" stroke="#7c2d92" stroke-width="1.2"/>"##,
                x - 5.0,
                x + 5.0
            );
        }
        // The label rides the panel's top edge rather than the bar's
        // end: at the first joint the whole range is sixteen pixels
        // tall, and a label pinned to it would sit on the nominal.
        text(
            out,
            x - 30.0,
            self.y + 20.0,
            11.5,
            "#7c2d92",
            "bold",
            &format!("σ {:.3} mm", s.sigma_y * MM),
        );
    }

    /// **The CERTIFIED enclosure at one joint**, over the widest box
    /// that certifies whole — the other half of E11's trade, drawn to
    /// the same scale as the cloud it sits beside.
    ///
    /// The along-the-chain half-width is microns (the reach barely
    /// moves; the deviation is lateral), so at any scale this sheet
    /// can carry, the box is a line. It is drawn at a floor width of
    /// [`CERTIFIED_MIN_PX`] so that it is visible AS a box, and the
    /// true number is in the table — a widened stroke that said
    /// nothing about it would be the drawing lying about a
    /// measurement.
    fn certified_box(&self, out: &mut String, index: usize) {
        let (dx, dy) = CERTIFIED_PIN_BOX[index];
        if dy == 0.0 {
            return;
        }
        let s = self.px_per_m();
        let (x0, y0) = self.map(index as f64 * LINK_LENGTH - dx, dy);
        let w = (2.0 * dx * s).max(CERTIFIED_MIN_PX);
        let _ = writeln!(
            out,
            r##"<rect x="{:.2}" y="{y0:.2}" width="{w:.2}" height="{:.2}" fill="none" stroke="#0f766e" stroke-width="1.8"/>"##,
            x0 - (w - 2.0 * dx * s).max(0.0) / 2.0,
            2.0 * dy * s
        );
    }

    /// A scale bar, so a panel's zoom ratio is readable off the sheet
    /// rather than only stated in the caption.
    fn scale_bar(&self, out: &mut String, metres: f64, label: &str) {
        let s = self.px_per_m();
        let w = metres * s;
        let (x, y) = (self.x + 14.0, self.y + self.h - 16.0);
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
    spreads: &[Spread],
    mean: f64,
    sigma: f64,
    min: f64,
    max: f64,
    report: &pncad::analysis::McReport,
    holds: usize,
    violated: usize,
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
        MARGIN_X,
        30.0,
        19.0,
        "#1a1a1a",
        "bold",
        "the four-link chain: one joint error, four times, and where the tip ends up",
    );
    text(
        &mut out,
        MARGIN_X,
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
        MARGIN_X,
        70.0,
        12.0,
        "#444444",
        "normal",
        &format!(
            "study: {LINKS} links of {:.0} mm, each joint an independent normal at \u{03c3} = {} rad ({:.2}\u{00b0})  \u{2014}  tip asserted within {:.2} mm of the target",
            LINK_LENGTH * MM,
            JOINT_SIGMA,
            JOINT_SIGMA.to_degrees(),
            POSITION_BOUND * MM
        ),
    );
    text(
        &mut out,
        MARGIN_X,
        90.0,
        12.0,
        "#444444",
        "normal",
        &format!(
            "tip position over the draws: mean {:.4} mm, \u{03c3} {:.4} mm, range [{:.4}, {:.4}] mm  \u{2014}  {:.2}% of draws fell outside the analyzed box",
            mean * MM,
            sigma * MM,
            min * MM,
            max * MM,
            report.outside_box * 100.0
        ),
    );

    let wide = Panel {
        x: MARGIN_X,
        y: WIDE_Y,
        w: WIDE_W,
        h: WIDE_H,
        centre: (LINKS as f64 * LINK_LENGTH / 2.0, 0.0),
        half_w: (LINKS as f64 * LINK_LENGTH + 8.0e-3) / 2.0,
    };
    wide.frame(
        &mut out,
        "the whole chain",
        &format!(
            "{:.0} px/mm \u{2014} all {} samples, every link outline walked off the body that sample built; the bar at each joint is that joint's MEASURED lateral range",
            wide.px_per_m() / MM,
            samples.len(),
        ),
    );
    wide.open(&mut out);
    wide.fan(&mut out, samples);
    wide.pin_cloud(&mut out, samples, 0..LINKS + 1);
    wide.nominal(&mut out, true, LINKS);
    wide.target(&mut out, POSITION_BOUND);
    for s in spreads.iter().skip(1) {
        wide.spread_dim(&mut out, s);
        wide.certified_box(&mut out, s.index);
    }
    Panel::close(&mut out);
    wide.scale_bar(&mut out, 1.0e-2, "10 mm");

    let zoom = Panel {
        x: MARGIN_X,
        y: ZOOM_Y,
        w: ZOOM_W_PX,
        h: ZOOM_H_PX,
        centre: (LINKS as f64 * LINK_LENGTH, 0.0),
        half_w: ZOOM_WINDOW / 2.0,
    };
    zoom.frame(
        &mut out,
        "the tip, magnified",
        &format!(
            "{:.0} px/mm \u{2014} the same {} samples' tip pins against the target and its {:.2} mm band",
            zoom.px_per_m() / MM,
            samples.len(),
            POSITION_BOUND * MM
        ),
    );
    zoom.open(&mut out);
    // The tip PIN only. The last link's outline is twelve millimetres
    // long on a seven-millimetre window, so drawing it here would fill
    // the panel with the one thing the panel is not about.
    zoom.pin_cloud(&mut out, samples, LINKS..LINKS + 1);
    zoom.nominal(&mut out, false, LINKS);
    zoom.certified_box(&mut out, LINKS);
    zoom.target(&mut out, POSITION_BOUND);
    Panel::close(&mut out);
    zoom.scale_bar(&mut out, 1.0e-3, "1 mm");

    // The growth, as a table beside the zoom: the measured ratio and
    // the one the accumulation law predicts, so the fan is checked
    // rather than admired.
    let tx = MARGIN_X + ZOOM_W_PX + 40.0;
    text(
        &mut out,
        tx,
        ZOOM_Y + 4.0,
        14.0,
        "#1a1a1a",
        "bold",
        "the dispersion down the chain, measured against the accumulation law",
    );
    text(
        &mut out,
        tx,
        ZOOM_Y + 26.0,
        12.0,
        "#555555",
        "normal",
        &format!(
            "joint j turns links j..{LINKS}, so pin k carries (k\u{2212}j) link lengths of lever from joint j \u{2014} the quadrature sum below"
        ),
    );
    let head = [
        "pin",
        "x",
        "lateral \u{03c3}",
        "measured range",
        "\u{00d7} pin 2",
        "law",
        "CERTIFIED \u{00b1}",
    ];
    for (c, label) in head.iter().enumerate() {
        text(
            &mut out,
            tx + COL_X[c],
            ZOOM_Y + 54.0,
            12.0,
            "#555555",
            "bold",
            label,
        );
    }
    for (i, s) in spreads.iter().skip(1).enumerate() {
        let row = [
            format!("{}", s.index + 1),
            format!("{:.0} mm", s.nominal.0 * MM),
            format!("{:.4} mm", s.sigma_y * MM),
            format!("[{:+.3}, {:+.3}] mm", s.lo * MM, s.hi * MM),
            format!("{:.2}\u{00d7}", ratio(s.sigma_y, spreads[1].sigma_y)),
            format!(
                "{:.2}\u{00d7}",
                ratio(predicted_sigma(s.index), predicted_sigma(1))
            ),
            format!("{:.4} mm", CERTIFIED_PIN_BOX[s.index].1 * MM),
        ];
        for (c, cell) in row.iter().enumerate() {
            text(
                &mut out,
                tx + COL_X[c],
                ZOOM_Y + 78.0 + i as f64 * 22.0,
                12.0,
                "#1a1a1a",
                "normal",
                cell,
            );
        }
    }
    let foot = ZOOM_Y + 78.0 + spreads.len() as f64 * 22.0;
    text(
        &mut out,
        tx,
        foot + 6.0,
        12.0,
        "#1a1a1a",
        "normal",
        &format!(
            "tip against the target: mean {:.4} mm, \u{03c3} {:.4} mm, worst {:.4} mm",
            mean * MM,
            sigma * MM,
            max * MM
        ),
    );
    text(
        &mut out,
        tx,
        foot + 26.0,
        12.0,
        "#1a1a1a",
        "normal",
        &format!(
            "advisory yield against the {:.2} mm band: {} of {} hold, {} violate",
            POSITION_BOUND * MM,
            holds,
            samples.len(),
            violated
        ),
    );

    let legend_y = ZOOM_Y + ZOOM_H_PX + 26.0;
    text(
        &mut out,
        MARGIN_X,
        legend_y,
        12.0,
        "#1f4e9c",
        "bold",
        &format!(
            "blue: one outline per sample per link, at {SAMPLE_ALPHA} opacity \u{2014} each walked off the cap face of the body that sample built"
        ),
    );
    text(
        &mut out,
        MARGIN_X,
        legend_y + 17.0,
        12.0,
        "#7c2d92",
        "bold",
        "purple: the joint pins, each a Surface::Cylinder read off that body, with its axis point as a dot; the bars are the measured lateral ranges",
    );
    text(
        &mut out,
        MARGIN_X,
        legend_y + 34.0,
        12.0,
        "#c2410c",
        "bold",
        "dashed orange: the nominal chain, every joint at zero.   dashed green: the target pin and the asserted position band",
    );
    text(
        &mut out,
        MARGIN_X,
        legend_y + 55.0,
        12.0,
        "#0f766e",
        "bold",
        &format!(
            "teal: the CERTIFIED enclosure per joint \u{2014} exact over a box, and silent outside it. The widest box that certifies THIS chain whole is {:.3} of the study.",
            CERTIFIABLE_FRACTION
        ),
    );
    text(
        &mut out,
        MARGIN_X,
        legend_y + 72.0,
        12.0,
        "#0f766e",
        "normal",
        &format!(
            "\u{2014} across the chain it grows 1 : 3 : 6 : 10, the WORST-CASE lever sum; the advisory \u{03c3} beside it grows 1 : 2.24 : 3.74 : 5.48, the quadrature sum. Along the chain it is {:.1e} m at the tip, so the box draws as a line and is widened to {CERTIFIED_MIN_PX} px to be seen at all.",
            CERTIFIED_PIN_BOX[LINKS].0
        ),
    );
    text(
        &mut out,
        MARGIN_X,
        legend_y + 89.0,
        12.0,
        "#1a1a1a",
        "normal",
        &format!(
            "ADVISORY: draws from the WHOLE distribution, tail included, which is nine times the certified box. Its numbers summarize these {} chains \u{2014} which is what the panels are.",
            samples.len()
        ),
    );

    out.push_str("</svg>\n");
    out
}
