//! **The import corpora's half of the neutral fact table for the
//! exact-iso question** (`docs/tesslie-coherence-iso-facts.md`;
//! `work/tesslie/chart-coherence-reads-a-non-iso-circle-as-an-iso-side.md`).
//! One summary line per fixture of the three committed corpora, and
//! the full rows for any fixture holding a circle carrier the
//! examination reads off its iso curve by more than ε. Prints and
//! asserts nothing:
//!
//! ```text
//! cargo nextest run -p step-import --test all tesslie_iso_facts --run-ignored only --no-capture
//! ```
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::print_stdout
)]

use crate::common::{
    FREECAD_FIXTURES, SOLID_FIXTURES, fixture, freecad_fixture, own_import_options,
};
use crate::wild::{WILD_IMPORTS, wild};
use geom_core::{Point3, Tol};
use step_import::{ImportOptions, StepImport, import_step};
use topo::Body;

/// **One fact row per boundary traversal of every chart-bearing face**
/// of `body`, as a markdown table row: what the carrier is, how far it
/// and its two vertices sit from the face's surface, how far it strays
/// from the iso curve the examination reads it as and from the other
/// iso family, and what the examination reports on it.
///
/// Distances are maxima over 65 evenly spaced carrier parameters of
/// the edge's own span, each measured to the surface point the chart's
/// inversion names for it (`Chart::uv_of`, then `Surface::eval`).
/// Deviation from a meridian is `radial · separation(u, u_ref)`; from a
/// rim, `v_lever · |v − v_ref|`, wrapped where v wraps. The door's
/// family takes the reference the examination reads (`classify_kind`'s
/// `u_raw` or `v_raw`); the other family takes the carrier's
/// mid-parameter point. A DECLARED COPY in three test crates (topo,
/// mesh, step-import), which cannot share a test helper; the copies
/// are print-only and assert nothing.
fn iso_fact_rows(label: &str, built: &str, body: &Body<f64>, tol: Tol) -> Vec<String> {
    let report = topo::examine_chart_coherence(body, tol);
    let mut rows = Vec::new();
    for (fk, _) in body.faces() {
        let Some(surface) = body.face_surface(fk) else {
            continue;
        };
        let Some(chart) = topo::Chart::of(surface) else {
            continue;
        };
        let off = |p: Point3<f64>| {
            let uv = chart.uv_of(p);
            (p - surface.eval(uv.x, uv.y)).norm()
        };
        let v_gap = |a: f64, b: f64| {
            if chart.v_domain_wraps() {
                geom::separation(a, b)
            } else {
                (a - b).abs()
            }
        };
        let view = body.face(fk).expect("a live face");
        for (_, lp, _) in view.loops() {
            let topo::LoopBoundary::Cycle { first } = lp.boundary else {
                continue;
            };
            for h in body.loop_cycle(first).expect("a cycle") {
                let he = body.get_half_edge(h).expect("a half-edge");
                let edge = body.get_edge(he.edge).expect("an edge");
                let Some(curve) = body.get_curve_geom(edge.curve).and_then(|g| g.certified())
                else {
                    rows.push(format!(
                        "| {label} | {built} | {fk:?} | scaffold | | | | | | |"
                    ));
                    continue;
                };
                let carrier = curve.carrier();
                let kind = format!("{carrier:?}");
                let kind = kind.split(['(', ' ', '{']).next().unwrap_or("?").to_owned();
                let (t0, t1) = curve.params();
                let samples: Vec<Point3<f64>> = (0..=64)
                    .map(|i| carrier.eval(t0 + (t1 - t0) * f64::from(i) / 64.0))
                    .collect();
                let mid = carrier.eval(0.5 * (t0 + t1));
                let max = |f: &dyn Fn(Point3<f64>) -> f64| {
                    samples.iter().map(|&p| f(p)).fold(0.0_f64, f64::max)
                };
                let meridian_dev =
                    |u_ref: f64| max(&|p| chart.radial(p) * geom::separation(chart.u_of(p), u_ref));
                let rim_dev = |v_ref: f64| max(&|p| chart.v_lever() * v_gap(chart.v_of(p), v_ref));
                let (class, door_dev, other_dev) = match topo::classify_kind(&chart, curve) {
                    Some(topo::TravKind::Meridian { u_raw }) => (
                        "Meridian",
                        format!("{:.3e}", meridian_dev(u_raw)),
                        format!("rim {:.3e}", rim_dev(chart.v_of(mid))),
                    ),
                    Some(topo::TravKind::Rim { v_raw }) => (
                        "Rim",
                        format!("{:.3e}", rim_dev(v_raw)),
                        format!("meridian {:.3e}", meridian_dev(chart.u_of(mid))),
                    ),
                    None => (
                        "none (run boundary)",
                        "—".to_owned(),
                        format!(
                            "meridian {:.3e}, rim {:.3e}",
                            meridian_dev(chart.u_of(mid)),
                            rim_dev(chart.v_of(mid))
                        ),
                    ),
                };
                let ends = [Some(he.start), body.half_edge_end(h)]
                    .into_iter()
                    .flatten()
                    .filter_map(|v| body.get_vertex(v).and_then(|x| body.get_point(x.point)))
                    .map(|&p| off(p))
                    .fold(0.0_f64, f64::max);
                let found: Vec<String> = report
                    .findings
                    .iter()
                    .filter(|f| f.face == fk && f.edge == he.edge)
                    .map(|f| {
                        let what = match f.condition {
                            topo::CoherenceCondition::MeridianClosure { .. } => "closure",
                            topo::CoherenceCondition::RimContinuation { .. } => "rim-cont",
                            topo::CoherenceCondition::MeridianContinuation { .. } => "mer-cont",
                        };
                        format!("{what} {:.3e}", f.metres)
                    })
                    .collect();
                rows.push(format!(
                    "| {label} | {built} | {fk:?} | {kind} | {:.3e} | {ends:.3e} | {class} | {door_dev} | {other_dev} | {} |",
                    max(&off),
                    if found.is_empty() { "—".to_owned() } else { found.join(", ") }
                ));
            }
        }
    }
    rows
}

/// The table header [`iso_fact_rows`] fills.
const ISO_FACT_HEADER: &str = "| body | built by, ε | face | carrier | max dist. from surface (m) | vertices' max dist. from surface (m) | door reads it as | deviation from that iso curve (m) | deviation from the other family (m) | door reports on this edge (m) |\n|---|---|---|---|---|---|---|---|---|---|";

#[test]
#[ignore = "measurement; run command in the module docs"]
fn tesslie_iso_facts_over_the_import_corpora() {
    let tol = Tol::witness();
    let eps = tol.eps();
    let mut sources: Vec<(String, String)> = Vec::new();
    for &name in SOLID_FIXTURES {
        sources.push((format!("own/{name}"), fixture(name, "step")));
    }
    for name in FREECAD_FIXTURES {
        sources.push((format!("freecad/{name}"), freecad_fixture(name)));
    }
    for name in WILD_IMPORTS {
        sources.push((format!("wild/{name}"), wild(name)));
    }
    let mut detail = Vec::new();
    println!(
        "| fixture | ε_in (adoption) | traversals on chart faces | circles | circles read > ε off their iso curve | max such deviation (m) | max carrier dist. from surface (m) | findings |"
    );
    println!("|---|---|---|---|---|---|---|---|");
    for (label, text) in &sources {
        let options = if label.starts_with("own/") {
            own_import_options(false)
        } else {
            ImportOptions::default()
        };
        let imported = match import_step(text, &options, tol) {
            Ok(i) => i,
            Err(e) => {
                println!(
                    "| {label} | refused: {} | | | | | | |",
                    e.render().lines().next().unwrap_or("")
                );
                continue;
            }
        };
        let eps_in = imported.eps_in();
        let StepImport::Solid { body, .. } = imported else {
            println!("| {label} | {eps_in:e} | wireframe | | | | | |");
            continue;
        };
        let body = body.into_body();
        let built = format!("import, ε_in = {eps_in:e}, ε = {eps:e}");
        let rows = iso_fact_rows(label, &built, &body, tol);
        let cell = |row: &str, i: usize| row.split('|').nth(i).unwrap_or("").trim().to_owned();
        let num = |s: String| s.parse::<f64>().unwrap_or(0.0);
        let circles: Vec<&String> = rows.iter().filter(|r| cell(r, 4) == "Circle").collect();
        let off: Vec<&&String> = circles.iter().filter(|r| num(cell(r, 8)) > eps).collect();
        let max_off = off.iter().map(|r| num(cell(r, 8))).fold(0.0_f64, f64::max);
        let max_surf = rows.iter().map(|r| num(cell(r, 5))).fold(0.0_f64, f64::max);
        let findings = topo::examine_chart_coherence(&body, tol).findings.len();
        println!(
            "| {label} | {eps_in:e} | {} | {} | {} | {max_off:.3e} | {max_surf:.3e} | {findings} |",
            rows.len(),
            circles.len(),
            off.len()
        );
        if !off.is_empty() {
            detail.extend(
                rows.iter()
                    .filter(|r| cell(r, 4) == "Circle" && num(cell(r, 8)) > eps)
                    .cloned(),
            );
        }
    }
    println!();
    println!("{ISO_FACT_HEADER}");
    for row in detail {
        println!("{row}");
    }
}
