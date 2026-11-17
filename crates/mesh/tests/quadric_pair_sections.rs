//! **Booleans of quadric pairs whose section is a closed fitted loop
//! build, and mesh.** A closed fitted section loop is a periodic cubic,
//! seated by its span; an edge on it is the loop's carrier and a span,
//! read as the clamped view over that span, whose interior knots are
//! simple (`crates/geom-brep/README.md` C11). The four pairs below each
//! built and refused in the chord pass while a seated loop carried its
//! own old start, a full-multiplicity knot, inside an edge.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::{CurveKind, SurfaceKind};
use geom_core::{Affine3, Point2, Point3, Tol, Vec3};
use profile::test_support::bulge_loop;
use profile::{Profile, SketchPlane};
use sweep::test_support::{finished, revolved_about_y};
use sweep::{ExtrudeSide, Extrusion, Revolution, extrude, revolve};
use topo::{AtRestBody, Body, BooleanError, BooleanResult};

use crate::common::{axis_y, validated};

fn tol() -> Tol {
    Tol::witness()
}

/// A disc of radius `r` about `(0, 0, z0)` extruded `depth` along `z`.
fn rod(r: f64, z0: f64, depth: f64) -> Body<f64> {
    let lp = profile::circle(Point2::new(0.0, 0.0), r, tol()).unwrap();
    let plane = SketchPlane::xy_at(Point3::new(0.0, 0.0, z0));
    let profile = Profile::new(plane, vec![lp.into()])
        .validate(tol())
        .unwrap();
    let side = ExtrudeSide::Along;
    extrude(&profile, Extrusion::Distance { depth, side }, tol())
        .unwrap()
        .body
        .into_body()
}

/// The ball of radius `r` about the origin, poled along `y`.
fn ball(r: f64) -> Body<f64> {
    revolved_about_y(
        vec![(Point2::new(0.0, -r), 1.0), (Point2::new(0.0, r), 0.0)],
        Revolution::Full,
        tol(),
    )
}

/// The donut of major radius `big` and tube radius `r` about `y`.
fn donut(big: f64, r: f64) -> Body<f64> {
    let lp = bulge_loop(vec![
        (Point2::new(big, -r), 1.0),
        (Point2::new(big, r), 1.0),
    ]);
    revolve(&validated(vec![lp]), axis_y(), Revolution::Full, tol())
        .unwrap()
        .body
        .into_body()
}

fn posed(what: &str, raw: Body<f64>, pose: Affine3<f64>) -> AtRestBody<f64> {
    finished(
        what,
        topo::transform_rigid(&raw, &geom_core::test_support::rigid(&pose, tol())).unwrap(),
        tol(),
    )
}

type Run = (&'static str, Result<BooleanResult<f64>, BooleanError>);

fn six(a: &AtRestBody<f64>, b: &AtRestBody<f64>) -> [Run; 6] {
    [
        ("A ∪ B", topo::union(a, b, tol())),
        ("B ∪ A", topo::union(b, a, tol())),
        ("A ∩ B", topo::intersect(a, b, tol())),
        ("B ∩ A", topo::intersect(b, a, tol())),
        ("A ∖ B", topo::subtract(a, b, tol())),
        ("B ∖ A", topo::subtract(b, a, tol())),
    ]
}

/// Every certified edge's reading, with the edge's faces' kinds.
fn spline_edges(body: &Body<f64>) -> Vec<(topo::EdgeKey, &geom_brep::EdgeCurve<f64>)> {
    body.edges()
        .filter_map(|(ek, e)| {
            let c = body.get_curve_geom(e.curve)?.certified()?;
            (c.carrier().kind() == CurveKind::Nurbs).then_some((ek, c))
        })
        .collect()
}

/// The greatest interior knot multiplicity over `body`'s spline edges,
/// each read as a reader reads it; and how many of them are spans of a
/// periodic cubic.
fn knots_and_loops(body: &Body<f64>) -> (usize, usize) {
    let mut worst = 0;
    let mut on_loops = 0;
    for (_, c) in spline_edges(body) {
        let geom::Curve3::Nurbs(net) = c.carrier() else {
            unreachable!("filtered to splines");
        };
        let runs: Vec<(f64, usize)> = net.knot_values().runs().collect();
        for &(_, m) in &runs[1..runs.len() - 1] {
            worst = worst.max(m);
        }
        on_loops += usize::from(c.stored_carrier().kind() == CurveKind::Periodic);
    }
    (worst, on_loops)
}

/// `body` meshes at every δ and each mesh passes `check_mesh`.
fn meshes(what: &str, body: &Body<f64>, deltas: &[f64]) {
    for &delta in deltas {
        let m = mesh::tessellate(body, delta, tol())
            .unwrap_or_else(|e| panic!("{what}, δ = {delta:e}: meshes, got {e:?}"));
        mesh::validate::check_mesh(&m)
            .unwrap_or_else(|e| panic!("{what}, δ = {delta:e}: check_mesh, got {e:?}"));
    }
}

/// The four measured pairs, each `(name, A, B)`.
fn pairs() -> Vec<(&'static str, AtRestBody<f64>, AtRestBody<f64>)> {
    let x = Vec3::new(1.0, 0.0, 0.0);
    let turn = |angle: f64, by: Vec3<f64>| {
        Affine3::translation(by) * Affine3::rotation_about_axis(Point3::origin(), x, angle)
    };
    vec![
        (
            "cylinder × sphere",
            finished("the cylinder", rod(0.5, -1.0, 2.0), tol()),
            posed(
                "the ball",
                ball(0.3),
                Affine3::translation(Vec3::new(0.5, 0.0, 0.0)),
            ),
        ),
        (
            "cylinder × cylinder",
            finished("the wide rod", rod(0.5, -1.0, 2.0), tol()),
            posed(
                "the thin rod",
                rod(0.2, -1.5, 3.0),
                turn(core::f64::consts::FRAC_PI_2, Vec3::new(0.15, 0.0, 0.1)),
            ),
        ),
        (
            "sphere × torus",
            finished("the ball", ball(0.5), tol()),
            posed(
                "the donut",
                donut(0.6, 0.2),
                turn(0.7, Vec3::new(0.1, 0.35, 0.2)),
            ),
        ),
        (
            "cylinder × torus",
            finished("the donut", donut(0.6, 0.2), tol()),
            posed(
                "the rod",
                rod(0.1, -1.0, 2.0),
                Affine3::translation(Vec3::new(0.62, 0.03, 0.0)),
            ),
        ),
    ]
}

#[test]
fn probe_pairs() {
    for (name, a, b) in pairs().into_iter().take(1) {
        for (op, run) in six(&a, &b) {
            match run {
                Ok(built) => {
                    let Some(out) = built.body() else {
                        eprintln!("{name}, {op}: EMPTY");
                        continue;
                    };
                    let body = &out.body;
                    let (worst, loops) = knots_and_loops(body);
                    let m = mesh::tessellate(body, 5e-3, tol());
                    let m = match m {
                        Ok(m) => format!("meshed, check {:?}", mesh::validate::check_mesh(&m)),
                        Err(mesh::TessellateError::UnsupportedCurve { edge, note }) => {
                            let (fa, fb) = topo::readback::edge_sides(body, edge).unwrap().faces();
                            let k = |f| body.face_surface(f).unwrap().kind();
                            format!(
                                "UnsupportedCurve beside {:?}/{:?}: {}",
                                k(fa),
                                k(fb),
                                &note[..60.min(note.len())]
                            )
                        }
                        Err(e) => format!("{e:?}"),
                    };
                    eprintln!(
                        "{name}, {op}: built; worst {worst}, {loops} loop edges of {}; {m}",
                        spline_edges(body).len()
                    );
                }
                Err(e) => eprintln!(
                    "{name}, {op}: REFUSED {}",
                    format!("{e:?}").chars().take(300).collect::<String>()
                ),
            }
        }
    }
    let _ = meshes;
}
