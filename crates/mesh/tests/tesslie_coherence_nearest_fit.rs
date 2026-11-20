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
    for (_, face) in body.faces() {
        let surface = body.get_surface(face.surface).unwrap();
        let Some(chart) = Chart::of(surface) else {
            continue;
        };
        let mut kinds = Vec::new();
        for lk in std::iter::once(face.outer).chain(face.rings.iter().copied()) {
            let LoopBoundary::Cycle { first } = body.get_loop(lk).unwrap().boundary else {
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
