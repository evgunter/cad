//! **Does `classify_kind`'s nearest fit put a spurious finding in the
//! chart-coherence report?** A circle carrier is a rim when its axis
//! is within 60° of the chart axis and a meridian otherwise; only a
//! circle that is a genuine iso curve deserves either answer. A plane
//! cutting a sphere off its axis leaves a non-iso CIRCLE, so that is
//! where a nearest-fit false positive would show. A plane cutting a
//! cylinder or a cone off-perpendicular leaves an ellipse (or a
//! parabola/hyperbola) instead, which reads as no iso side at all.
//!
//! The row PRINTS every cut's carriers and report and asserts nothing,
//! so it is `#[ignore]`d:
//!
//! ```text
//! cargo nextest run -p mesh --test all tesslie_coherence_nearest_fit \
//!     --run-ignored only --no-capture
//! ```
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::print_stdout
)]

use crate::common;
use common::*;
use geom::Curve3;
use geom::Surface;
use geom_brep::EdgeCurveSpec;
use geom_core::{Point2, Point3, Tol, Vec3};
use profile::{Profile, SketchPlane, test_support::bulge_loop};
use sweep::{ExtrudeSide, Extrusion, extrude};
use topo::construction::{Construct, FaceSurface, MefSite, MevSite};
use topo::{Body, Chart, LoopBoundary};

/// A unit sphere (axis z) cut along ONE oblique circle: the plane with
/// unit normal at `tilt` from z (toward x), `offset` from the centre.
/// Two arcs of that circle bound two sphere faces, the cap and the
/// rest. The split door refuses a sphere, so this is the Euler doors'
/// spelling of the same section; the arcs' endpoints are where a
/// plane's intersection with a sphere puts them.
fn oblique_cap(tilt: f64, offset: f64) -> Option<Body<f64>> {
    let tol = Tol::witness();
    let n = Vec3::new(tilt.sin(), 0.0, tilt.cos());
    let c = Point3::new(0.0, 0.0, 0.0) + n * offset;
    let r = (1.0 - offset * offset).sqrt();
    let y = Vec3::new(0.0, 1.0, 0.0);
    let circle = Curve3::circle(c, n, r, y);
    let a = circle.eval(0.0);
    let b = circle.eval(core::f64::consts::PI);
    let mut body = Body::<f64>::new();
    let seed = body.mvfs(a, true).ok()?;
    body.set_face_surface(
        seed.face,
        FaceSurface::New {
            surface: Surface::Sphere {
                center: Point3::new(0.0, 0.0, 0.0),
                radius: 1.0,
                axis: Vec3::new(0.0, 0.0, 1.0),
                u_ref: Vec3::new(1.0, 0.0, 0.0),
            },
            sense: true,
        },
    )
    .ok()?;
    let e = body
        .mev(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            b,
            EdgeCurveSpec::arc_of_circle(circle.clone(), 0.0, core::f64::consts::PI)?,
            tol,
        )
        .ok()?;
    body.mef(
        MefSite::Chords {
            he1: e.he_minus,
            he2: e.he_plus,
        },
        EdgeCurveSpec::arc_of_circle(circle, core::f64::consts::PI, core::f64::consts::TAU)?,
        FaceSurface::Inherit,
        tol,
    )
    .ok()?;
    Some(body)
}

fn cylinder() -> Body<f64> {
    let lp = bulge_loop(vec![
        (Point2::new(-1.0, 0.0), 1.0),
        (Point2::new(1.0, 0.0), 1.0),
    ]);
    let disc = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .unwrap();
    extrude(
        &disc,
        Extrusion::Distance {
            depth: 2.5,
            side: ExtrudeSide::Along,
        },
        Tol::witness(),
    )
    .unwrap()
    .body
    .into_body()
}

/// Each curved face's edges: the carrier kind and, for a circle, the
/// |cos| of its axis against the chart axis (the 0.5 split).
fn carriers(body: &Body<f64>) -> Vec<String> {
    let mut out = Vec::new();
    for (fk, _) in body.faces() {
        let surface = body.face_surface(fk).unwrap();
        let Some(chart) = Chart::of(surface) else {
            continue;
        };
        let mut kinds = Vec::new();
        for (_, lp, _) in body.face(fk).unwrap().loops() {
            let LoopBoundary::Cycle { first } = lp.boundary else {
                continue;
            };
            for hek in body.loop_cycle(first).unwrap() {
                let he = body.get_half_edge(hek).unwrap();
                let e = body.get_edge(he.edge).unwrap();
                let Some(c) = body.get_curve_geom(e.curve).unwrap().certified() else {
                    kinds.push("scaffold".to_owned());
                    continue;
                };
                kinds.push(match c.carrier() {
                    Curve3::Line(_) => "line".to_owned(),
                    Curve3::Circle(ci) => {
                        format!(
                            "circle|cos|={:.3}",
                            ci.frame.axis.dot(chart.frame().axis).abs()
                        )
                    }
                    Curve3::Ellipse(..) => "ellipse".to_owned(),
                    other => format!("{:?}", other).chars().take(12).collect(),
                });
            }
        }
        let s = format!("{surface:?}");
        out.push(format!(
            "{}: [{}]",
            s.split([' ', '{', '(']).next().unwrap(),
            kinds.join(", ")
        ));
    }
    out
}

fn report(label: &str, body: &Body<f64>) {
    let tol = Tol::witness();
    let r = topo::examine_chart_coherence(body, tol);
    let meshes = mesh::tessellate(body, 0.05, tol).is_ok();
    println!(
        "CUT {label}: meshes={meshes} findings={} unexamined={}",
        r.findings.len(),
        r.unexamined.len()
    );
    for c in carriers(body) {
        println!("    face {c}");
    }
    for f in &r.findings {
        println!(
            "    FINDING {:?} gap={:e} lever={:e} metres={:e}",
            f.condition, f.gap, f.lever, f.metres
        );
    }
    for u in &r.unexamined {
        println!("    unexamined {:?}", u.why);
    }
}

/// Splits `body` by the plane through `offset * n` with unit normal
/// `n` at `tilt` radians from the chart axis `axis` (toward `side`),
/// and reports both halves.
fn cut(name: &str, body: &Body<f64>, axis: Vec3<f64>, side: Vec3<f64>, tilt: f64, offset: f64) {
    let tol = Tol::witness();
    let n = axis * tilt.cos() + side * tilt.sin();
    let finished = topo::test_support::finished(name, body.clone(), tol);
    let plane = topo::test_support::split_plane(Point3::new(0.0, 0.0, 0.0) + n * offset, n, tol);
    let label = format!("{name} tilt={:.1}deg offset={offset}", tilt.to_degrees());
    match topo::splitting::split(&finished, &plane, tol) {
        Ok(r) => {
            for (half, part) in [("above", &r.above), ("below", &r.below)] {
                match part {
                    topo::splitting::SplitPart::Body(b) => {
                        report(&format!("{label} {half}"), &Body::clone(b))
                    }
                    _ => println!("CUT {label} {half}: no material"),
                }
            }
        }
        Err(e) => println!("CUT {label}: split refused: {e:?}"),
    }
}

#[test]
#[ignore = "measurement; run command in the module docs"]
fn the_nearest_fit_on_oblique_cuts() {
    let deg = |d: f64| d.to_radians();
    let y = Vec3::new(0.0, 1.0, 0.0);
    let z = Vec3::new(0.0, 0.0, 1.0);
    let x = Vec3::new(1.0, 0.0, 0.0);
    // The ball's chart axis is y (revolved about it), radius 1.
    for t in [0.0, 30.0, 59.0, 61.0, 75.0, 90.0] {
        for d in [0.3, 0.0] {
            match oblique_cap(deg(t), d) {
                Some(b) => report(&format!("sphere-cap tilt={t}deg offset={d}"), &b),
                None => println!("CUT sphere-cap tilt={t}deg offset={d}: did not build"),
            }
        }
    }
    let ball = ball();
    for t in [0.0, 30.0, 55.0, 59.0, 61.0, 65.0, 80.0, 90.0] {
        cut("ball", &ball, y, x, deg(t), 0.3);
    }
    cut("ball", &ball, y, x, deg(90.0), 0.0);
    cut("ball", &ball, y, x, deg(75.0), 0.0);
    // The cylinder's axis is z, radius 1, from z = 0 to 2.5.
    let cyl = cylinder();
    for t in [0.0, 17.2, 59.0, 61.0, 80.0] {
        let n = z * deg(t).cos() + x * deg(t).sin();
        let tol = Tol::witness();
        let finished = topo::test_support::finished("cyl", cyl.clone(), tol);
        let plane = topo::test_support::split_plane(Point3::new(0.0, 0.0, 1.25), n, tol);
        let label = format!("cylinder tilt={t}deg");
        match topo::splitting::split(&finished, &plane, tol) {
            Ok(r) => {
                for (half, part) in [("above", &r.above), ("below", &r.below)] {
                    if let topo::splitting::SplitPart::Body(b) = part {
                        report(&format!("{label} {half}"), &Body::clone(b));
                    }
                }
            }
            Err(e) => println!("CUT {label}: split refused: {e:?}"),
        }
    }
    // The cone's axis is y, apex at (0,1,0), base radius 1 at y = 0.
    let cone = cone();
    for t in [0.0, 30.0, 59.0, 61.0, 90.0] {
        let n = y * deg(t).cos() + x * deg(t).sin();
        let tol = Tol::witness();
        let finished = topo::test_support::finished("cone", cone.clone(), tol);
        let plane = topo::test_support::split_plane(Point3::new(0.1, 0.4, 0.0), n, tol);
        let label = format!("cone tilt={t}deg");
        match topo::splitting::split(&finished, &plane, tol) {
            Ok(r) => {
                for (half, part) in [("above", &r.above), ("below", &r.below)] {
                    if let topo::splitting::SplitPart::Body(b) = part {
                        report(&format!("{label} {half}"), &Body::clone(b));
                    }
                }
            }
            Err(e) => println!("CUT {label}: split refused: {e:?}"),
        }
    }
    let _ = z;
}

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

/// **The neutral fact table for the exact-iso question**, this crate's
/// witnesses (`docs/tesslie-coherence-iso-facts.md`): one row per
/// boundary traversal of a chart-bearing face. Prints and asserts
/// nothing:
///
/// ```text
/// cargo nextest run -p mesh --test all tesslie_iso_facts --run-ignored only --no-capture
/// ```
#[test]
#[ignore = "measurement; run command in the doc"]
fn tesslie_iso_facts_over_the_witnesses() {
    use crate::mesh8r1_probes::tilted_meridian_sliver;
    use crate::mesh8r2_probes::{
        ellipse_trimmed_with_a_conic_chord, ellipse_trimmed_with_a_rim_chord, tilted_halves,
        wobbled_sliver,
    };
    use common::witness_bodies::{keyway, oblique_lens, slit};
    let tol = Tol::witness();
    let eps = tol.eps();
    let euler = format!("Euler doors, ε = {eps:e}");
    let split = format!("extrude + split, ε = {eps:e}");
    let (above, below) = tilted_halves();
    let mut bodies: Vec<(String, String, Body<f64>)> = vec![
        (
            "tilted_meridian_sliver(4 ε) (mesh8r1)".into(),
            euler.clone(),
            tilted_meridian_sliver(4.0 * eps / 0.105),
        ),
        (
            "tilted_meridian_sliver(1024 ε) (mesh8r1)".into(),
            euler.clone(),
            tilted_meridian_sliver(1024.0 * eps / 0.105),
        ),
        ("tilted half above (mesh8r2)".into(), split.clone(), above),
        ("tilted half below (mesh8r2)".into(), split.clone(), below),
        (
            "ellipse_trimmed_with_a_rim_chord(1024 ε) (pin)".into(),
            format!("extrude + split + split_edge + mef, ε = {eps:e}"),
            ellipse_trimmed_with_a_rim_chord(1024.0 * eps),
        ),
        (
            "ellipse_trimmed_with_a_conic_chord (pin)".into(),
            format!("extrude + split + split_edge + mef, ε = {eps:e}"),
            ellipse_trimmed_with_a_conic_chord().0,
        ),
        ("keyway (r2r)".into(), euler.clone(), keyway().0),
        ("oblique_lens (r2r)".into(), euler.clone(), oblique_lens().0),
        ("slit (r2r)".into(), euler.clone(), slit().0),
    ];
    // The nist_ftc_09 class: a cylinder sliver whose meridian side is a
    // line with endpoints stated non-co-azimuthally, at the traced
    // 21.4 pm and at 1024 ε (`wobbled_sliver`, radius 0.05 m).
    for (what, metres) in [("21.4 pm, traced", 21.4e-12), ("1024 ε", 1024.0 * eps)] {
        match wobbled_sliver(metres) {
            Some(b) => bodies.push((
                format!("wobbled_sliver({what}) (nist_ftc_09 class)"),
                euler.clone(),
                b,
            )),
            None => println!("wobbled_sliver({what}): did not build"),
        }
    }
    println!("{ISO_FACT_HEADER}");
    for (label, built, body) in &bodies {
        for row in iso_fact_rows(label, built, body, tol) {
            println!("{row}");
        }
    }
}
