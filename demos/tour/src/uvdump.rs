//! **UV trim-loop dump** — every face's chart domain, drawn flat as SVG.
//!
//! A `Surface` in this kernel is unbounded ("the infinite plane", "the
//! infinite cylinder"); a `Face` is the patch of one that its boundary
//! **loops** cut out, and those loops live in the surface's own `(u, v)`
//! chart as [`Pcurve`]s. That chart is already a 2-D drawing, so it
//! needs no projection, no camera, and no silhouettes to be rendered
//! faithfully — which is exactly why this lane exists next to the two
//! 3-D render lanes (`demos/render.sh`): it is the ONE picture the
//! kernel can draw of itself with no external renderer at all.
//!
//! # What this is for
//!
//! It is a **diagnostic**, not a depiction of the part. The failures it
//! makes visible are the ones that are miserable in a debugger:
//!
//! - a loop that does not close (the [`closure gap`] is printed per
//!   face, not eyeballed);
//! - a loop wound the wrong way, so the even-odd interior is the
//!   complement of the intended one (the signed chart area is printed,
//!   and its sign named);
//! - a seam crossing on a periodic chart — a cylinder's `u` is
//!   2π-periodic, and a loop that crosses the seam draws as a jump
//!   across the strip;
//! - a half-edge carrying no stored pcurve cache, or carrying a
//!   [`Pcurve::Fitted`] image where the tessellator's trim walk accepts
//!   only closed forms (`mesh::trimmed` refuses both, typed — here they
//!   are drawn and labeled instead, because a diagnostic that refuses
//!   the broken input shows you nothing).
//!
//! [`closure gap`]: FaceDump
//!
//! # Deliberate choices, stated
//!
//! - **Sampled, not exact.** Each half-edge's chart image is drawn as a
//!   dense polyline ([`SAMPLES`] points; 2 for a straight carrier).
//!   SVG could carry a line or an ellipse arc exactly, but the point
//!   here is to show the pcurve as *evaluated* — an exact arc command
//!   would draw the curve we believe is there rather than the one
//!   [`Pcurve::eval`] returns.
//! - **Denser than the tessellator.** These samples are uniform in the
//!   carrier parameter and unrelated to `mesh`'s chord parameters, so a
//!   cell shows the pcurve, not the triangulation's view of it.
//! - **Non-uniform scale.** Each face is fitted to its cell box
//!   independently, so a cylinder chart (`u` in radians ≈ 6.28, `v` in
//!   metres ≈ 0.05) is legible rather than a sliver. Angles are
//!   therefore NOT preserved; the printed axis ranges are the truth.
//! - **Every face gets a cell**, planar ones included, and a face whose
//!   loops cannot be walked gets a cell naming the reason — never a
//!   silent gap. The montage composer selects a subset and says how
//!   many it dropped.
//!
//! # The failure arm has no fixture (stated, not hidden)
//!
//! [`draw_failure`] is unexercised by the corpus: all 982 faces of the
//! M7 tour walk clean, and forging one that does not would mean
//! hand-building a corrupt body here, at the demo layer, purely to feed
//! a drawing routine. The arm stands as the tripwire for a real
//! regression rather than as tested code — the same posture
//! `mesh::trimmed` takes about its own `SelfTouchingTrimLoop` refusal.
//! What IS tested is the composer's side of the contract (failures sort
//! first, nothing is dropped silently): `compose_uv_montage.py
//! --selftest`, an always-run CI row.

use std::fmt::Write as _;

use pncad::geom_brep::Pcurve;
use pncad::geom_core::{Band, Point2};
use pncad::geom_curves::Curve3;
use pncad::geom_surfaces::Surface;
use pncad::topo::{Body, HalfEdgeKey, LoopBoundary, LoopKey};

/// Samples per curved half-edge image (a straight carrier draws with 2).
const SAMPLES: usize = 96;

const CELL_W: f64 = 380.0;
const CELL_H: f64 = 320.0;
const PAD_L: f64 = 54.0;
const PAD_R: f64 = 14.0;
const PAD_T: f64 = 48.0;
const PAD_B: f64 = 48.0;

/// Stroke color per pcurve form — the legend of every cell.
const COLOR_HARMONIC: &str = "#1f4e79";
const COLOR_ISOLINE: &str = "#1b7a3d";
const COLOR_FITTED: &str = "#c1590a";

/// One face's manifest entry, written to `<outdir>/uv.json` for the
/// montage composer.
pub struct FaceDump {
    /// The tour scene the body belongs to.
    pub scene: String,
    /// The body's tour label (its STL/STEP stem).
    pub body: String,
    /// The face's ordinal in `body.faces()` order.
    pub face: usize,
    /// The chart kind: `plane`, `cylinder`, …, `nurbs`.
    pub chart: &'static str,
    /// The SVG's filename, relative to `<outdir>/uv/`.
    pub svg: String,
    /// Whether the chart is curved — the montage's selection key (a
    /// plane chart's picture is the face's own outline, which the 3-D
    /// lanes already show).
    pub curved: bool,
    /// The measured facts of the walk (all-zero when it failed).
    pub stats: FaceStats,
    /// Set when the loops could not be walked: the reason, drawn into
    /// the cell and carried here so the composer can never drop it
    /// quietly.
    pub note: Option<String>,
}

/// One half-edge's sampled chart image, with the provenance the cell
/// draws.
struct Traversal {
    pts: Vec<(f64, f64)>,
    form: &'static str,
    /// `false` when the pcurve was derived on demand rather than read
    /// from the body's stored cache.
    stored: bool,
}

/// The chart kind's name and whether it is curved.
fn chart_of(surface: &Surface<f64>) -> (&'static str, bool) {
    match surface {
        Surface::Plane { .. } => ("plane", false),
        Surface::Cylinder { .. } => ("cylinder", true),
        Surface::Cone { .. } => ("cone", true),
        Surface::Sphere { .. } => ("sphere", true),
        Surface::Torus { .. } => ("torus", true),
        Surface::Nurbs(_) => ("nurbs", true),
    }
}

/// The chart's axis units, for the printed ranges. `u` is an angle on
/// every analytic chart but the plane; `v` is an angle only on the
/// sphere (polar) and the torus (the tube angle).
fn units_of(chart: &str) -> (&'static str, &'static str) {
    match chart {
        "plane" => ("m", "m"),
        "cylinder" | "cone" => ("rad", "m"),
        "sphere" | "torus" => ("rad", "rad"),
        _ => ("", ""),
    }
}

/// Walks one loop, returning each half-edge's sampled chart image in
/// traversal order.
///
/// Unlike `mesh::trimmed`'s walk this accepts **every** pcurve form and
/// falls back to `topo::pcurve_of`'s derive-on-demand when the body
/// carries no stored cache — a face the tessellator refuses is exactly
/// the face worth looking at.
fn walk_loop(body: &Body<f64>, lk: LoopKey, band: Band) -> Result<Vec<Traversal>, String> {
    let lp = body.get_loop(lk).ok_or("loop key resolves to nothing")?;
    let LoopBoundary::Cycle { first } = lp.boundary else {
        return Err("loop has no cycle (empty boundary)".to_string());
    };
    let cycle = body.loop_cycle(first).ok_or("loop cycle does not close")?;
    let mut out = Vec::with_capacity(cycle.len());
    for hek in cycle {
        out.push(traverse(body, hek, band)?);
    }
    Ok(out)
}

/// One half-edge's chart image, sampled in traversal order.
fn traverse(body: &Body<f64>, hek: HalfEdgeKey, band: Band) -> Result<Traversal, String> {
    let he = body.get_half_edge(hek).ok_or("dangling half-edge key")?;
    let edge = body.get_edge(he.edge).ok_or("half-edge names no edge")?;
    let curve = body
        .get_curve_geom(edge.curve)
        .ok_or("edge names no curve entry")?
        .certified()
        .ok_or("edge carries null scaffolding, not a certified carrier")?;
    let (t0, t1) = curve.params();
    // A straight carrier's chart image is straight on an affine chart
    // and a graph on a swept one; two samples are honest for the
    // former, so only genuinely straight-in-UV forms take the shortcut.
    let straight = matches!(curve.carrier(), Curve3::Line { .. });

    // The stored cache is what the tessellator reads; the derive is the
    // fallback so that a cache-less face still draws (and says so).
    let (pcurve, stored) = match body.pcurve(hek) {
        Some(cache) => (cache.pcurve().clone(), true),
        None => (
            pncad::topo::pcurve_of(body, hek, band)
                .map_err(|e| format!("no stored pcurve and the derive refused: {e:?}"))?,
            false,
        ),
    };
    let form = match &pcurve {
        Pcurve::Harmonic { .. } => "harmonic",
        Pcurve::IsoLine { .. } => "isoline",
        Pcurve::Fitted(_) => "fitted",
    };
    let n = if straight || matches!(pcurve, Pcurve::IsoLine { .. }) {
        2
    } else {
        SAMPLES
    };

    // Traversal direction: `he_plus` runs the carrier forward.
    let forward = edge.he_plus == hek;
    let mut pts = Vec::with_capacity(n);
    for k in 0..n {
        let s = k as f64 / (n - 1) as f64;
        let s = if forward { s } else { 1.0 - s };
        let uv: Point2<f64> = pcurve.eval(t0 + (t1 - t0) * s);
        pts.push((uv.x, uv.y));
    }
    Ok(Traversal { pts, form, stored })
}

/// Shoelace signed area of a closed chart loop (positive = CCW in the
/// chart's own orientation).
fn signed_area(pts: &[(f64, f64)]) -> f64 {
    let mut acc = 0.0;
    for i in 0..pts.len() {
        let (x0, y0) = pts[i];
        let (x1, y1) = pts[(i + 1) % pts.len()];
        acc += x0 * y1 - x1 * y0;
    }
    acc / 2.0
}

/// The measured facts of one walked face — drawn into its cell, and
/// carried into `uv.json` so the montage's choice of representative is
/// made from data rather than from ordinal luck.
#[derive(Default)]
pub struct FaceStats {
    /// Boundary loops: the outer one plus the rings.
    pub loops: usize,
    /// Half-edges across all loops.
    pub half_edges: usize,
    /// How many of those read a **stored** pcurve cache (the rest were
    /// derived on demand — the tessellator would refuse them).
    pub cached: usize,
    /// Which pcurve forms appear, in `harmonic`/`isoline`/`fitted`
    /// order.
    pub forms: Vec<&'static str>,
    /// The outer loop's signed chart area (negative = wound CW).
    pub area: f64,
    /// The worst gap between one traversal's last chart point and the
    /// next traversal's first — the `LoopNotClosed` measurement.
    pub gap: f64,
}

/// The per-loop chart rings (each traversal contributing all but its
/// last point, exactly as the tessellator's trim walk composes its
/// polygon) plus the measured diagnostics over them.
fn measure(loops: &[Vec<Traversal>]) -> (Vec<Vec<(f64, f64)>>, FaceStats) {
    let mut rings: Vec<Vec<(f64, f64)>> = Vec::new();
    for l in loops {
        let mut ring = Vec::new();
        for tr in l {
            ring.extend(tr.pts.iter().copied().take(tr.pts.len() - 1));
        }
        rings.push(ring);
    }
    let mut gap: f64 = 0.0;
    for l in loops {
        for i in 0..l.len() {
            let (ax, ay) = *l[i].pts.last().expect("traversal has samples");
            let (bx, by) = l[(i + 1) % l.len()].pts[0];
            gap = gap.max(((ax - bx).powi(2) + (ay - by).powi(2)).sqrt());
        }
    }
    let forms = ["harmonic", "isoline", "fitted"]
        .into_iter()
        .filter(|f| loops.iter().flatten().any(|t| t.form == *f))
        .collect();
    let stats = FaceStats {
        loops: loops.len(),
        half_edges: loops.iter().flatten().count(),
        cached: loops.iter().flatten().filter(|t| t.stored).count(),
        forms,
        area: signed_area(&rings[0]),
        gap,
    };
    (rings, stats)
}

/// Emits one SVG per face of `body` under `<outdir>/uv/`, returning the
/// manifest entries.
pub fn emit(scene: &str, label: &str, body: &Body<f64>, outdir: &str) -> Vec<FaceDump> {
    let dir = format!("{outdir}/uv");
    std::fs::create_dir_all(&dir).expect("create uv dir");
    let band = Band::linear().expect("the run tolerance yields a valid linear band");

    let mut dumps = Vec::new();
    for (ord, (fk, face)) in body.faces().enumerate() {
        let svg_name = format!("{label}.f{ord:03}.svg");
        let (chart, curved) = match body.get_surface(face.surface) {
            Some(s) => chart_of(s),
            None => ("missing", true),
        };

        // Outer loop first, then the rings — the even-odd order the
        // tessellator's interior classification assumes.
        let mut lks = vec![face.outer];
        lks.extend(face.rings.iter().copied());
        let walked: Result<Vec<Vec<Traversal>>, String> = lks
            .iter()
            .map(|&lk| walk_loop(body, lk, band))
            .collect::<Result<Vec<_>, _>>();

        let (svg, stats, note) = match walked {
            Ok(loops) if loops.iter().all(|l| !l.is_empty()) => {
                let (rings, stats) = measure(&loops);
                (
                    draw_face(label, ord, chart, &loops, &rings, &stats),
                    stats,
                    None,
                )
            }
            Ok(_) => {
                let why = "a loop walked to zero half-edges".to_string();
                let svg = draw_failure(label, ord, chart, &why);
                (svg, FaceStats::default(), Some(why))
            }
            Err(why) => {
                let why = format!("{why} (face {fk:?})");
                let svg = draw_failure(label, ord, chart, &why);
                (svg, FaceStats::default(), Some(why))
            }
        };
        std::fs::write(format!("{dir}/{svg_name}"), svg).expect("write uv svg");
        dumps.push(FaceDump {
            scene: scene.to_string(),
            body: label.to_string(),
            face: ord,
            chart,
            svg: svg_name,
            curved,
            stats,
            note,
        });
    }
    dumps
}

/// The cell frame, title band, and legend shared by both cell kinds.
fn cell_header(out: &mut String, label: &str, ord: usize, chart: &str, subtitle: &str) {
    let _ = writeln!(
        out,
        r##"<svg xmlns="http://www.w3.org/2000/svg" width="{CELL_W}" height="{CELL_H}" viewBox="0 0 {CELL_W} {CELL_H}">
<rect x="0" y="0" width="{CELL_W}" height="{CELL_H}" fill="#ffffff" stroke="#b8b8b8" stroke-width="1"/>
<text x="10" y="19" font-family="monospace" font-size="12" font-weight="bold" fill="#222">{}  ·  face {ord}</text>
<text x="10" y="34" font-family="monospace" font-size="10.5" fill="#555">{} chart — {}</text>"##,
        escape(label),
        chart,
        escape(subtitle)
    );
}

/// A cell for a face whose loops could not be walked: the reason, drawn.
fn draw_failure(label: &str, ord: usize, chart: &str, why: &str) -> String {
    let mut out = String::new();
    cell_header(&mut out, label, ord, chart, "TRIM WALK FAILED");
    let _ = writeln!(
        out,
        r##"<rect x="{PAD_L}" y="{PAD_T}" width="{}" height="{}" fill="#fdf0ec" stroke="#c1590a" stroke-width="1.2"/>"##,
        CELL_W - PAD_L - PAD_R,
        CELL_H - PAD_T - PAD_B
    );
    // Hand-wrapped: no text layout engine here, and a clipped reason
    // would be exactly the silent drop this lane exists to prevent.
    for (i, line) in wrap(why, 40).into_iter().enumerate() {
        let _ = writeln!(
            out,
            r##"<text x="{}" y="{}" font-family="monospace" font-size="10" fill="#8a3c05">{}</text>"##,
            PAD_L + 10.0,
            PAD_T + 24.0 + 13.0 * i as f64,
            escape(&line)
        );
    }
    out.push_str("</svg>\n");
    out
}

/// A cell for a face that walked: the even-odd interior, the per-form
/// strokes, traversal arrows, junctions, and the measured diagnostics.
fn draw_face(
    label: &str,
    ord: usize,
    chart: &str,
    loops: &[Vec<Traversal>],
    rings: &[Vec<(f64, f64)>],
    stats: &FaceStats,
) -> String {
    // Chart bounds over every sample of every loop.
    let (mut u0, mut u1, mut v0, mut v1) = (f64::MAX, f64::MIN, f64::MAX, f64::MIN);
    for tr in loops.iter().flatten() {
        for &(u, v) in &tr.pts {
            u0 = u0.min(u);
            u1 = u1.max(u);
            v0 = v0.min(v);
            v1 = v1.max(v);
        }
    }
    // A chart extent can legitimately be zero in one axis (a degenerate
    // image); widen so the map stays finite rather than dividing by it.
    let (du, dv) = ((u1 - u0).max(1e-12), (v1 - v0).max(1e-12));
    let (pu, pv) = (du * 0.06, dv * 0.06);
    let (u0, u1, v0, v1) = (u0 - pu, u1 + pu, v0 - pv, v1 + pv);
    let bw = CELL_W - PAD_L - PAD_R;
    let bh = CELL_H - PAD_T - PAD_B;
    // The map is done here, in Rust, rather than by an SVG transform:
    // the fit is non-uniform (module docs), and a transform would carry
    // that distortion into the arrowheads and junction dots too.
    let sx = |u: f64| PAD_L + (u - u0) / (u1 - u0) * bw;
    let sy = |v: f64| PAD_T + bh - (v - v0) / (v1 - v0) * bh;

    let (uu, vu) = units_of(chart);
    let subtitle = format!(
        "{} loop(s), {} half-edge(s), {} cached, {}",
        stats.loops,
        stats.half_edges,
        stats.cached,
        stats.forms.join("+")
    );
    let mut out = String::new();
    cell_header(&mut out, label, ord, chart, &subtitle);

    // Plot frame.
    let _ = writeln!(
        out,
        r##"<rect x="{PAD_L}" y="{PAD_T}" width="{bw}" height="{bh}" fill="#fbfbfb" stroke="#d0d0d0" stroke-width="1"/>"##
    );

    // Seam markers: a 2π multiple falling inside the drawn range means
    // the loop reaches the chart's periodic seam.
    if uu == "rad" {
        let tau = std::f64::consts::TAU;
        let k0 = (u0 / tau).ceil() as i64;
        let k1 = (u1 / tau).floor() as i64;
        for k in k0..=k1 {
            let x = sx(k as f64 * tau);
            // A chart drawn over a full period puts a seam on each
            // edge; flip the label inboard past the midline so the
            // right-hand one is not clipped off the cell.
            let (anchor, tx) = if x > PAD_L + bw * 0.5 {
                ("end", x - 3.0)
            } else {
                ("start", x + 3.0)
            };
            let _ = writeln!(
                out,
                r##"<line x1="{x:.2}" y1="{PAD_T}" x2="{x:.2}" y2="{}" stroke="#b03060" stroke-width="1" stroke-dasharray="4 3"/>
<text x="{:.2}" y="{}" text-anchor="{anchor}" font-family="monospace" font-size="9" fill="#b03060">seam u={k}&#183;2&#960;</text>"##,
                PAD_T + bh,
                tx,
                PAD_T + 11.0
            );
        }
    }

    // The even-odd interior: outer loop and rings as one path, which is
    // the same rule `mesh::trimmed` uses to pick the material side.
    out.push_str(r##"<path fill-rule="evenodd" fill="#dbe9f6" stroke="none" d=""##);
    for ring in rings {
        let _ = write!(out, "M{:.2},{:.2}", sx(ring[0].0), sy(ring[0].1));
        for &(u, v) in &ring[1..] {
            let _ = write!(out, "L{:.2},{:.2}", sx(u), sy(v));
        }
        out.push('Z');
    }
    out.push_str("\"/>\n");

    // Per-half-edge strokes, colored by pcurve form; a derived (not
    // cached) image is dashed, because "the tessellator would refuse
    // this" is the thing worth seeing at a glance.
    for l in loops {
        for tr in l {
            let color = match tr.form {
                "isoline" => COLOR_ISOLINE,
                "fitted" => COLOR_FITTED,
                _ => COLOR_HARMONIC,
            };
            let dash = if tr.stored {
                ""
            } else {
                r##" stroke-dasharray="5 3""##
            };
            let _ = write!(
                out,
                r##"<path fill="none" stroke="{color}" stroke-width="1.6"{dash} d="M"##
            );
            let mut first = true;
            for &(u, v) in &tr.pts {
                if !first {
                    out.push('L');
                }
                first = false;
                let _ = write!(out, "{:.2},{:.2}", sx(u), sy(v));
            }
            out.push_str("\"/>\n");
            arrow(&mut out, tr, &sx, &sy, color);
        }
    }

    // Junction dots: where one traversal hands off to the next.
    for l in loops {
        for tr in l {
            let (u, v) = tr.pts[0];
            let _ = writeln!(
                out,
                r##"<circle cx="{:.2}" cy="{:.2}" r="2.4" fill="#ffffff" stroke="#333" stroke-width="1"/>"##,
                sx(u),
                sy(v)
            );
        }
    }

    // Axis ranges (the truth the non-uniform fit discards) and the
    // measured diagnostics.
    let _ = writeln!(
        out,
        r##"<text x="{PAD_L}" y="{}" font-family="monospace" font-size="9.5" fill="#444">u &#8712; [{:.5}, {:.5}] {uu}</text>
<text x="{PAD_L}" y="{}" font-family="monospace" font-size="9.5" fill="#444">v &#8712; [{:.5}, {:.5}] {vu}</text>
<text x="{PAD_L}" y="{}" font-family="monospace" font-size="9.5" fill="{}">outer winding {} (A={:.4})   closure gap {:.1e}</text>"##,
        CELL_H - PAD_B + 14.0,
        u0 + pu,
        u1 - pu,
        CELL_H - PAD_B + 26.0,
        v0 + pv,
        v1 - pv,
        CELL_H - PAD_B + 38.0,
        if stats.area > 0.0 { "#444" } else { "#b03060" },
        if stats.area > 0.0 { "CCW" } else { "CW" },
        stats.area,
        stats.gap
    );
    out.push_str("</svg>\n");
    out
}

/// A traversal-direction arrowhead at the image's midpoint.
fn arrow(
    out: &mut String,
    tr: &Traversal,
    sx: &impl Fn(f64) -> f64,
    sy: &impl Fn(f64) -> f64,
    color: &str,
) {
    let n = tr.pts.len();
    // The sample pair straddling the image's midpoint: (0, 1) for a
    // two-point straight image, the middle pair otherwise.
    let i = (n - 1) / 2;
    let (a, b) = (tr.pts[i], tr.pts[i + 1]);
    let (ax, ay, bx, by) = (sx(a.0), sy(a.1), sx(b.0), sy(b.1));
    let (dx, dy) = (bx - ax, by - ay);
    let len = (dx * dx + dy * dy).sqrt();
    if len < 1e-9 {
        return;
    }
    let (ux, uy) = (dx / len, dy / len);
    let (mx, my) = ((ax + bx) / 2.0, (ay + by) / 2.0);
    let (s, w) = (5.0, 2.8);
    let _ = writeln!(
        out,
        r##"<polygon fill="{color}" points="{:.2},{:.2} {:.2},{:.2} {:.2},{:.2}"/>"##,
        mx + ux * s,
        my + uy * s,
        mx - ux * s * 0.4 - uy * w,
        my - uy * s * 0.4 + ux * w,
        mx - ux * s * 0.4 + uy * w,
        my - uy * s * 0.4 - ux * w
    );
}

/// Greedy word wrap for the failure cell's reason text.
fn wrap(text: &str, cols: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let mut cur = String::new();
    for word in text.split_whitespace() {
        if !cur.is_empty() && cur.len() + 1 + word.len() > cols {
            lines.push(std::mem::take(&mut cur));
        }
        if !cur.is_empty() {
            cur.push(' ');
        }
        cur.push_str(word);
    }
    if !cur.is_empty() {
        lines.push(cur);
    }
    lines
}

/// XML text escaping. The inputs are body labels and kernel error
/// spellings — controlled, but a dump that emitted invalid XML would
/// fail the composer rather than itself, which is the wrong place.
fn escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

/// The manifest the montage composer reads (hand-rolled JSON, matching
/// `scenes.json`'s house style).
pub fn manifest_json(dumps: &[FaceDump]) -> String {
    let entries: Vec<String> = dumps
        .iter()
        .map(|d| {
            let note = match &d.note {
                Some(n) => format!("\"{}\"", n.replace('"', "'")),
                None => "null".to_string(),
            };
            let forms: Vec<String> = d.stats.forms.iter().map(|f| format!("\"{f}\"")).collect();
            format!(
                "  {{\"scene\": \"{}\", \"body\": \"{}\", \"face\": {}, \"chart\": \"{}\", \
                 \"svg\": \"{}\", \"curved\": {}, \"loops\": {}, \"half_edges\": {}, \
                 \"cached\": {}, \"forms\": [{}], \"area\": {:e}, \"gap\": {:e}, \
                 \"note\": {note}}}",
                d.scene,
                d.body,
                d.face,
                d.chart,
                d.svg,
                d.curved,
                d.stats.loops,
                d.stats.half_edges,
                d.stats.cached,
                forms.join(", "),
                d.stats.area,
                d.stats.gap
            )
        })
        .collect();
    format!("[\n{}\n]\n", entries.join(",\n"))
}
