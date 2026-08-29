//! **SHELLFIX PR-2b's opening measurement** — what the curved-corner
//! fixtures actually refuse, and which corner FORMS they need.
//!
//! This file is print-only instrumentation, not a pin: it dumps the
//! refusal each curved fixture takes, and then the corner/edge census
//! that decides what the simultaneous door has to be able to solve.
//! The census is the input to the build: the general
//! plane∩curved∩curved case is not built on the presumption that the
//! corpus needs it — this is where the corpus says.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, dead_code)]

use geom_brep::SurfaceKind;
use geom_brep::intersect::route;
use geom_core::{Band, Point2, Tol, Vec2};
use profile::{Profile, ProfileLoop, ProfileVertex, RawLoop, SketchPlane};
use sweep::{Revolution, RevolveAxis, revolve};
use topo::Body;

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

const FIT_TOL: f64 = 1e-6;

/// A meridian loop revolved a full turn about the `y` axis.
fn revolved(lp: ProfileLoop<f64>) -> Body<f64> {
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .expect("the meridian validates");
    revolve(
        &profile,
        RevolveAxis {
            origin: p2(0.0, 0.0),
            dir: Vec2::new(0.0, 1.0),
        },
        Revolution::Full,
        Tol::witness(),
    )
    .expect("the meridian revolves")
    .body
}

/// A meridian whose belly is an arc from `(r0, y0)` to `(r1, y1)`
/// about a centre ON the axis — a SPHERE zone. `sign` flips the bulge,
/// and the flipped one is a TORUS: the differential this file exists
/// to keep honest.
fn bellied(r0: f64, y0: f64, r1: f64, y1: f64, cy: f64, sign: f64) -> Body<f64> {
    let c = p2(0.0, cy);
    let (dx0, dy0) = (r0 - c.x, y0 - c.y);
    let (dx1, dy1) = (r1 - c.x, y1 - c.y);
    let sweep = (dx0 * dy1 - dy0 * dx1).atan2(dx0 * dx1 + dy0 * dy1);
    let bulge = sign * (sweep / 4.0).tan();
    revolved(RawLoop::new(vec![
        ProfileVertex::new(p2(0.0, 0.0), 0.0),
        ProfileVertex::new(p2(r0, y0), bulge),
        ProfileVertex::new(p2(r1, y1), 0.0),
        ProfileVertex::new(p2(0.0, y1), 0.0),
    ]))
}

/// **The sphere-zone vase**: a bulged belly between two planar caps.
fn sphere_zone_vase() -> Body<f64> {
    bellied(3.0 / 64.0, 0.0, 3.0 / 64.0, 8.0 / 64.0, 4.0 / 64.0, 1.0)
}

/// The same vase bulged the OTHER way: the centre leaves the axis and
/// the wall is a TORUS.
fn torus_belly_vase() -> Body<f64> {
    bellied(3.0 / 64.0, 0.0, 3.0 / 64.0, 8.0 / 64.0, 4.0 / 64.0, -1.0)
}

/// **The teapot's bellied pot**, rebuilt here from the scene's own
/// numbers: base disc, FOOT CYLINDER, sphere-zone belly, mouth disc.
fn bellied_pot() -> Body<f64> {
    let (foot, y_foot, r_neck, y_mouth) = (4.0 / 64.0, 1.0 / 64.0, 3.0 / 64.0, 8.0 / 64.0);
    let c = p2(0.0, y_mouth / 2.0);
    let (dx0, dy0) = (foot - c.x, y_foot - c.y);
    let (dx1, dy1) = (r_neck - c.x, y_mouth - c.y);
    let sweep = (dx0 * dy1 - dy0 * dx1).atan2(dx0 * dx1 + dy0 * dy1);
    revolved(RawLoop::new(vec![
        ProfileVertex::new(p2(0.0, 0.0), 0.0),
        ProfileVertex::new(p2(foot, 0.0), 0.0),
        ProfileVertex::new(p2(foot, y_foot), (sweep / 4.0).tan()),
        ProfileVertex::new(p2(r_neck, y_mouth), 0.0),
        ProfileVertex::new(p2(0.0, y_mouth), 0.0),
    ]))
}

/// **The tangent bullet**: a hemisphere tangent to its cylinder.
fn bullet() -> Body<f64> {
    let (r, h) = (3.0 / 64.0, 8.0 / 64.0);
    revolved(
        <ProfileLoop<f64> as RawLoop<f64>>::new(vec![
            ProfileVertex::new(p2(0.0, 0.0), 0.0),
            ProfileVertex::new(p2(r, 0.0), 0.0),
            ProfileVertex::new(p2(r, h), (std::f64::consts::FRAC_PI_2 / 4.0).tan()),
            ProfileVertex::new(p2(0.0, h + r), 0.0),
        ])
        .with_tangent_joints(vec![2]),
    )
}

/// A cylinder between two caps normal to its axis — the drum, which
/// hollowed at base and must keep hollowing.
fn drum() -> Body<f64> {
    revolved(ProfileLoop::new(vec![
        ProfileVertex::new(p2(0.0, 0.0), 0.0),
        ProfileVertex::new(p2(3.0 / 64.0, 0.0), 0.0),
        ProfileVertex::new(p2(3.0 / 64.0, 8.0 / 64.0), 0.0),
        ProfileVertex::new(p2(0.0, 8.0 / 64.0), 0.0),
    ]))
}

/// **The cone frustum** between two planar caps.
fn cone_frustum() -> Body<f64> {
    revolved(ProfileLoop::new(vec![
        ProfileVertex::new(p2(0.0, 0.0), 0.0),
        ProfileVertex::new(p2(4.0 / 64.0, 0.0), 0.0),
        ProfileVertex::new(p2(2.0 / 64.0, 8.0 / 64.0), 0.0),
        ProfileVertex::new(p2(0.0, 8.0 / 64.0), 0.0),
    ]))
}

/// **The partial-revolve wedge**: a quarter turn of the drum's own
/// meridian, so its meridian caps are planes CONTAINING the axis.
fn wedge() -> Body<f64> {
    let (r, h) = (3.0 / 64.0, 8.0 / 64.0);
    let profile = Profile::new(
        SketchPlane::xy(),
        vec![ProfileLoop::new(vec![
            ProfileVertex::new(p2(0.0, 0.0), 0.0),
            ProfileVertex::new(p2(r, 0.0), 0.0),
            ProfileVertex::new(p2(r, h), 0.0),
            ProfileVertex::new(p2(0.0, h), 0.0),
        ])],
    )
    .validate(Tol::witness())
    .expect("the meridian validates");
    revolve(
        &profile,
        RevolveAxis {
            origin: p2(0.0, 0.0),
            dir: Vec2::new(0.0, 1.0),
        },
        Revolution::Partial(std::f64::consts::FRAC_PI_2),
        Tol::witness(),
    )
    .expect("a quarter revolve")
    .body
}

/// The distinct incident surface kinds at every vertex, as a sorted
/// name list — the corner FORM the simultaneous door would have to
/// solve there.
fn corner_forms(body: &Body<f64>) -> Vec<(String, usize)> {
    let mut tally: Vec<(String, usize)> = Vec::new();
    for (v, vertex) in body.vertices() {
        let Some(emanating) = vertex.emanating else {
            continue;
        };
        let orbit = body.vertex_orbit(emanating).expect("orbit");
        let mut keys: Vec<topo::SurfaceKey> = Vec::new();
        for he in orbit {
            let lp = body.get_half_edge(he).expect("he").parent_loop;
            let face = body.get_loop(lp).expect("loop").face;
            let key = body.get_face(face).expect("face").surface;
            if !keys.contains(&key) {
                keys.push(key);
            }
        }
        let mut names: Vec<&'static str> = keys
            .iter()
            .map(|k| SurfaceKind::of(body.get_surface(*k).expect("surface")).name())
            .collect();
        names.sort_unstable();
        let form = format!("{} [{}]", names.len(), names.join(" ∩ "));
        let _ = v;
        match tally.iter_mut().find(|(f, _)| *f == form) {
            Some((_, n)) => *n += 1,
            None => tally.push((form, 1)),
        }
    }
    tally
}

/// Every edge's surface PAIR and the C5 arm it would route to.
fn edge_pairs(body: &Body<f64>) -> Vec<(String, usize)> {
    let mut tally: Vec<(String, usize)> = Vec::new();
    for (e, _) in body.edges() {
        let mut keys: Vec<topo::SurfaceKey> = Vec::new();
        // The two faces of the edge, read off its two half-edges.
        let data = body.get_edge(e).expect("edge");
        for he in [data.he_plus, data.he_minus] {
            let lp = body.get_half_edge(he).expect("he").parent_loop;
            let face = body.get_loop(lp).expect("loop").face;
            let key = body.get_face(face).expect("face").surface;
            if !keys.contains(&key) {
                keys.push(key);
            }
        }
        let kinds: Vec<SurfaceKind> = keys
            .iter()
            .map(|k| SurfaceKind::of(body.get_surface(*k).expect("surface")))
            .collect();
        let carrier = body
            .get_edge(e)
            .and_then(|d| body.get_curve_geom(d.curve))
            .and_then(topo::CurveGeom::certified)
            .map(|c| match c.carrier() {
                geom::Curve3::Line { .. } => "Line",
                geom::Curve3::Circle { .. } => "Circle",
                geom::Curve3::Ellipse { .. } => "Ellipse",
                _ => "other",
            })
            .unwrap_or("none");
        let desc = body
            .get_edge(e)
            .and_then(|d| body.get_curve_geom(d.curve))
            .and_then(topo::CurveGeom::certified)
            .map(|c| match c.description() {
                geom_brep::EdgeGeometry::Intersection { .. } => "Intersection",
                geom_brep::EdgeGeometry::TangentIntersection { .. } => "TangentIntersection",
                geom_brep::EdgeGeometry::Seam { .. } => "Seam",
                geom_brep::EdgeGeometry::IsoCurve { .. } => "IsoCurve",
                geom_brep::EdgeGeometry::MappedCurve(m) => match m {
                    geom_brep::MappedCurve::PlacedSegment { .. } => "Mapped/PlacedSegment",
                    geom_brep::MappedCurve::ExtrudedPoint { .. } => "Mapped/ExtrudedPoint",
                    geom_brep::MappedCurve::RevolvedPoint { .. } => "Mapped/RevolvedPoint",
                },
            })
            .unwrap_or("none");
        let form = match kinds[..] {
            [a] => format!("SEAM ({}) carrier {carrier} desc {desc}", a.name()),
            [a, b] => {
                let r = route(a, b);
                format!(
                    "{} × {} carrier {carrier} desc {desc} → {} implemented={}",
                    a.name(),
                    b.name(),
                    r.rung.name(),
                    r.implemented
                )
            }
            _ => "??".to_string(),
        };
        match tally.iter_mut().find(|(f, _)| *f == form) {
            Some((_, n)) => *n += 1,
            None => tally.push((form, 1)),
        }
    }
    tally
}

/// The cavity the axial door builds, reported directly: the moved
/// corners and the volume, which is what a wrong corner shows up in.
fn cavity_report(what: &str, body: &Body<f64>, t: f64) {
    let tol = Tol::witness();
    let mut charts: Vec<(topo::SurfaceKey, Vec<topo::FaceKey>, bool)> = Vec::new();
    for (k, f) in body.faces() {
        match charts.iter_mut().find(|(s, _, _)| *s == f.surface) {
            Some((_, v, _)) => v.push(k),
            None => charts.push((f.surface, vec![k], f.sense)),
        }
    }
    let moves: Vec<topo::ChartMove<f64>> = charts
        .into_iter()
        .map(|(_, faces, sense)| topo::ChartMove {
            faces,
            distance: if sense { -t } else { t },
        })
        .collect();
    // What the PER-CHART door does to each chart on its own, with the
    // same signed distance: the control that says whether a wrong
    // direction is this door's or the offset mint's.
    for m in &moves {
        let mut one = body.clone();
        let before = one
            .get_face(m.faces[0])
            .and_then(|f| one.get_surface(f.surface))
            .cloned();
        match topo::replace_faces_offset(&mut one, &m.faces, m.distance, 1e-6, band(), tol) {
            Ok(()) => {
                let after = one
                    .get_face(m.faces[0])
                    .and_then(|f| one.get_surface(f.surface))
                    .cloned();
                println!("  per-chart d={}: {before:?} -> {after:?}", m.distance);
            }
            Err(e) => println!("  per-chart d={} REFUSED: {e:?}", m.distance),
        }
    }
    let mut cavity = body.clone();
    match topo::offset_charts_together(&mut cavity, &moves, band(), tol) {
        Err(e) => println!("  cavity REFUSED: {e:?}"),
        Ok(()) => {
            let outer = topo::mass_properties(body, tol).map(|p| p.volume);
            let inner = topo::mass_properties(&cavity, tol).map(|p| p.volume);
            println!("  {what}: outer {outer:?} cavity {inner:?}");
            for (k, v) in cavity.vertices() {
                if let Some(p) = cavity.get_point(v.point) {
                    println!("    {k:?} -> [{:.6}, {:.6}, {:.6}]", p.x, p.y, p.z);
                }
            }
        }
    }
}

/// **The teapot's own two arms**, measured: the SEALED hollow and the
/// OPENED one, on the bellied pot the scene wants to ship.
#[test]
fn sf2b_bellied_pot_sealed_and_opened() {
    let tol = Tol::witness();
    let t = 1.0 / 128.0;
    let body = bellied_pot();
    match topo::shell(&body, t, FIT_TOL, band(), tol) {
        Ok(p) => println!(
            "[pot] SEALED hollows: {} shells, props {:?}",
            p.shells().count(),
            topo::mass_properties(&p, tol).map(|x| (x.volume, x.surface_area))
        ),
        Err(e) => println!("[pot] SEALED REFUSED: {e:?}"),
    }
    let mouth: Vec<topo::FaceKey> = body
        .faces()
        .filter(|(_, f)| {
            matches!(body.get_surface(f.surface),
                Some(geom::Surface::Plane { origin, .. }) if (origin.y - 8.0 / 64.0).abs() < 1e-12)
        })
        .map(|(k, _)| k)
        .collect();
    println!("[pot] mouth chart: {} face(s)", mouth.len());
    match topo::shell_open(&body, t, &mouth, FIT_TOL, band(), tol) {
        Ok(p) => println!(
            "[pot] OPENED: {} shells, props {:?}",
            p.shells().count(),
            topo::mass_properties(&p, tol).map(|x| (x.volume, x.surface_area))
        ),
        Err(e) => println!("[pot] OPENED REFUSED: {e:?}"),
    }
}

/// **The measurement.** Run at head, before any 2b code.
#[test]
fn sf2b_head_measurement() {
    let tol = Tol::witness();
    let t = 1.0 / 128.0;
    for (what, body) in [
        ("the sphere-zone vase", sphere_zone_vase()),
        ("the torus-belly vase", torus_belly_vase()),
        ("the cone frustum", cone_frustum()),
        ("the partial-revolve wedge", wedge()),
        ("the teapot's bellied pot", bellied_pot()),
        ("the tangent bullet", bullet()),
        ("the drum", drum()),
    ] {
        println!("=== {what} ===");
        match topo::shell(&body, t, FIT_TOL, band(), tol) {
            Ok(_) => println!("  HOLLOWS"),
            Err(e) => {
                println!("  Display: {e}");
                println!("  Debug:   {e:?}");
            }
        }
        for (k, f) in body.faces() {
            let _ = k;
            println!(
                "  chart sense={}: {:?}",
                f.sense,
                body.get_surface(f.surface).expect("surface")
            );
        }
        for (form, n) in corner_forms(&body) {
            println!("  corner form x{n}: {form}");
        }
        for (form, n) in edge_pairs(&body) {
            println!("  edge      x{n}: {form}");
        }
        cavity_report(what, &body, t);
    }
}
