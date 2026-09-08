//! **The axial door takes a one-surface corner**: a vertex all of whose
//! faces lie on one surface of revolution is a point OF that surface,
//! and the offset moves it along that surface's own normal — the
//! concentric move on a profile circle, the perpendicular foot on a
//! profile line — with the azimuth carried as every seam's is. The
//! full-period torus is the body that has such vertices: its two
//! half-circle walls share ONE torus, meet along two meridian seams
//! and share two equators, and its two vertices are each incident to
//! the torus and nothing else. Every accepting row here asserts a
//! closed form: the wall's volume `2π²R[r² − (r − t)²]`, the cavity's
//! minor radius `r − t`, and the seam vertex's own image.
//!
//! The two refusal-side rows are HAND-MADE operands, said to be: no
//! door builds a vertex with no profile constraint, or one whose only
//! surface is a cylinder, so a wedge's axis edge and a drum's seam are
//! split by `Body::split_edge` to make them.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use geom::{Curve3, Surface};
use geom_core::{Point2, Point3, Tol, Vec2, Vec3};
use profile::{Profile, ProfileLoop, ProfileVertex, RawLoop, SketchPlane};
use sweep::{Revolution, RevolveAxis, TubeWindow, revolve, tube_along_arc};
use topo::{Body, EdgeKey, ReplaceFaceError, ShellError, VertexKey};

const R: f64 = 2.0;
const SMALL_R: f64 = 0.5;
const T: f64 = 0.05;

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

fn tol() -> Tol {
    Tol::witness()
}

/// The tube door's full torus: `R = 2`, `r = 1/2`, about `y`, seam at
/// `u_ref = x`; its two vertices are the outer and inner equators.
fn tube_torus() -> Body<f64> {
    tube_along_arc::<f64>(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::unit_y(),
        Vec3::unit_x(),
        R,
        TubeWindow::Full,
        SMALL_R,
        tol(),
    )
    .expect("the solid torus builds")
    .body
}

/// A full revolve of a closed loop about `y`.
fn revolved(lp: ProfileLoop<f64>, turn: Revolution<f64>) -> Body<f64> {
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(tol())
        .expect("the meridian validates");
    revolve(
        &profile,
        RevolveAxis {
            origin: p2(0.0, 0.0),
            dir: Vec2::new(0.0, 1.0),
        },
        turn,
        tol(),
    )
    .expect("the meridian revolves")
    .body
}

/// The same torus from the revolve door, its two seam vertices at the
/// minor angles `v` and `v + π` (radians from the outer equator, up).
fn revolved_torus(v: f64) -> Body<f64> {
    let (s, c) = v.sin_cos();
    let a = p2(R + SMALL_R * c, SMALL_R * s);
    let b = p2(R - SMALL_R * c, -SMALL_R * s);
    revolved(
        RawLoop::new(vec![ProfileVertex::new(a, 1.0), ProfileVertex::new(b, 1.0)]),
        Revolution::Full,
    )
}

/// `2π²R[r² − (r − t)²]`, the volume between two coaxial tori.
fn wall_volume(r: f64, t: f64) -> f64 {
    2.0 * PI * PI * R * (r * r - (r - t) * (r - t))
}

fn axial(p: Point3<f64>) -> (f64, f64) {
    ((p.x * p.x + p.z * p.z).sqrt(), p.y)
}

fn point(body: &Body<f64>, v: VertexKey) -> Point3<f64> {
    *body
        .get_point(body.get_vertex(v).expect("vertex").point)
        .expect("point")
}

fn carrier(body: &Body<f64>, e: EdgeKey) -> (Curve3<f64>, (f64, f64)) {
    let c = body
        .get_curve_geom(body.get_edge(e).expect("edge").curve)
        .and_then(|g| g.certified())
        .expect("a certified curve");
    (c.carrier().clone(), c.params())
}

/// The distinct minor radii stored on the body's torus charts.
fn minor_radii(body: &Body<f64>) -> Vec<f64> {
    let mut out: Vec<f64> = body
        .faces()
        .filter_map(|(_, f)| match body.get_surface(f.surface) {
            Some(Surface::Torus { minor_radius, .. }) => Some(*minor_radius),
            _ => None,
        })
        .collect();
    out.sort_by(f64::total_cmp);
    out.dedup();
    out
}

/// Shell to tier 3, two shells, and the wall's closed form.
fn shelled_to_closed_form(what: &str, body: &Body<f64>) -> topo::Shelled<f64> {
    let out = topo::shell(body, T, tol())
        .unwrap_or_else(|e| panic!("{what}: the full torus shells, got {e}"));
    assert_eq!(
        topo::validate_geometric(&out.body, tol()),
        Ok(()),
        "{what}: tier 3"
    );
    assert_eq!(out.body.shells().count(), 2, "{what}: outer + cavity");
    let props = topo::mass_properties(&out.body, tol()).expect("props");
    let want = wall_volume(SMALL_R, T);
    assert!(
        (props.volume - want).abs() <= 1e-9 + props.volume_pad,
        "{what}: wall volume got {} (pad {}), want {want}",
        props.volume,
        props.volume_pad
    );
    assert_eq!(
        minor_radii(&out.body),
        vec![SMALL_R - T, SMALL_R],
        "{what}: the cavity's minor radius is r − t, bit for bit"
    );
    out
}

/// **Row 1: the solid full torus shells.**
#[test]
fn the_solid_full_torus_shells() {
    shelled_to_closed_form("tube torus", &tube_torus());
}

/// **Row 3: the seam vertex moved exactly.** Each of the tube's two
/// vertices lands at meridian distance `r − t` from the tube centre
/// `(R, 0)` at its OLD azimuth, and the four cavity edges are the two
/// circles the module names — the meridian seams concentric about the
/// tube centre at `r − t`, the equator seams about the axis at
/// `R ± (r − t)` — with their endpoints on them.
#[test]
fn the_seam_vertex_moves_concentrically_at_its_old_azimuth() {
    let body = tube_torus();
    let out = shelled_to_closed_form("tube torus", &body);
    let cavity = &out.body;
    assert_eq!(out.naming.inner_vertices.len(), 2);
    for &(old, new) in &out.naming.inner_vertices {
        let (p, q) = (point(&body, old), point(cavity, new));
        let (rho, h) = axial(q);
        let meridian = (rho - R).hypot(h);
        assert!(
            (meridian - (SMALL_R - T)).abs() <= 1e-15,
            "{old:?} → {new:?}: meridian distance {meridian} is not r − t"
        );
        let e_old = Vec3::new(p.x, 0.0, p.z).normalize();
        let e_new = Vec3::new(q.x, 0.0, q.z).normalize();
        let along = e_old.dot(e_new);
        assert!(
            (1.0 - along).abs() <= f64::EPSILON,
            "{old:?} → {new:?}: the azimuth moved, old·new = {along}"
        );
    }
    assert_eq!(out.naming.inner_edges.len(), 4);
    let (mut meridians, mut equators) = (0, 0);
    for &(_, e) in &out.naming.inner_edges {
        let (c, (t0, t1)) = carrier(cavity, e);
        let Curve3::Circle {
            center,
            axis,
            radius,
            ..
        } = c
        else {
            panic!("{e:?}: a torus seam is a circle, got {c:?}");
        };
        let (rho_c, h_c) = axial(center);
        if rho_c <= 1e-15 {
            // An equator seam: centred on the axis, normal along it,
            // radius `R ± (r − t)` at the corner's own station.
            equators += 1;
            assert_eq!(h_c, 0.0, "{e:?}: the equators' station");
            assert!((axis.dot(Vec3::unit_y()).abs() - 1.0).abs() <= 1e-15);
            let want = [R + (SMALL_R - T), R - (SMALL_R - T)];
            assert!(
                want.iter().any(|w| (radius - w).abs() <= 1e-15),
                "{e:?}: equator radius {radius} is neither of {want:?}"
            );
        } else {
            // A meridian seam: concentric about the tube centre.
            meridians += 1;
            assert!(
                (rho_c - R).abs() <= 1e-15 && h_c.abs() <= 1e-15,
                "{e:?}: meridian seam centre ({rho_c}, {h_c})"
            );
            assert_eq!(radius, SMALL_R - T, "{e:?}: meridian seam radius");
        }
        // Endpoints on the carrier, read from the vertices themselves.
        let edge = cavity.get_edge(e).expect("edge");
        let start = point(
            cavity,
            cavity.get_half_edge(edge.he_plus).expect("he").start,
        );
        let end = point(cavity, cavity.half_edge_end(edge.he_plus).expect("end"));
        assert!(
            c.eval(t0).distance(start) <= 1e-13,
            "{e:?}: start off its carrier"
        );
        assert!(
            c.eval(t1).distance(end) <= 1e-13,
            "{e:?}: end off its carrier"
        );
    }
    assert_eq!((meridians, equators), (2, 2));
}

/// **Row 7: the seam vertex at every cardinal minor angle.** The
/// revolve door places the seam vertices where the loop's own vertices
/// are, so one fixture puts them at the outer and inner equators
/// (`v = 0, π` — the tube door's own placement), one at the top and
/// bottom (`v = π/2, 3π/2`), and one off every axis of symmetry
/// (`v = π/4`). Every one shells to the same closed form, and every
/// seam vertex lands on the concentric point `(R + (r − t) cos v,
/// (r − t) sin v)`.
#[test]
fn the_seam_vertex_at_each_minor_angle_shells_to_the_closed_form() {
    for v in [0.0, PI / 2.0, PI / 4.0] {
        let body = revolved_torus(v);
        assert_eq!(body.vertices().count(), 2, "v = {v}: two seam vertices");
        let out = shelled_to_closed_form(&format!("revolved torus, v = {v}"), &body);
        for &(old, new) in &out.naming.inner_vertices {
            let (rho, h) = axial(point(&out.body, new));
            let (rho_old, h_old) = axial(point(&body, old));
            // The concentric point of the OLD corner about `(R, 0)`.
            let (dx, dy) = (rho_old - R, h_old);
            let n = dx.hypot(dy);
            let want = (R + dx / n * (SMALL_R - T), dy / n * (SMALL_R - T));
            assert!(
                (rho - want.0).abs() <= 1e-15 && (h - want.1).abs() <= 1e-15,
                "v = {v}: {old:?} → ({rho}, {h}), want {want:?}"
            );
        }
    }
}

/// The quarter-turn wedge of `sf2b_axial`: a cylinder wall, two caps
/// normal to the axis, two meridian caps, and an axis edge between the
/// meridian caps.
fn wedge(r: f64, h: f64) -> Body<f64> {
    revolved(
        ProfileLoop::new(vec![
            ProfileVertex::new(p2(0.0, 0.0), 0.0),
            ProfileVertex::new(p2(r, 0.0), 0.0),
            ProfileVertex::new(p2(r, h), 0.0),
            ProfileVertex::new(p2(0.0, h), 0.0),
        ]),
        Revolution::Partial(PI / 2.0),
    )
}

/// The full drum of the same rectangle: its cylinder wall is swept in
/// two half-turn bands on one surface, so two generator lines stand
/// between the rims on that one cylinder — either is a seam to the
/// door; the row splits the one at `u_ref`.
fn drum(r: f64, h: f64) -> Body<f64> {
    revolved(
        ProfileLoop::new(vec![
            ProfileVertex::new(p2(0.0, 0.0), 0.0),
            ProfileVertex::new(p2(r, 0.0), 0.0),
            ProfileVertex::new(p2(r, h), 0.0),
            ProfileVertex::new(p2(0.0, h), 0.0),
        ]),
        Revolution::Full,
    )
}

/// The one edge of `body` whose carrier is a LINE satisfying `pick`.
fn line_edge(body: &Body<f64>, pick: impl Fn(Point3<f64>, Vec3<f64>, bool) -> bool) -> EdgeKey {
    let hits: Vec<EdgeKey> = body
        .edges()
        .filter(|(e, data)| {
            // A seam is an edge whose two sides lie on one SURFACE —
            // the door's own test — whether or not they are one face.
            let same_surface = {
                let f = |he| {
                    let face = body
                        .get_loop(body.get_half_edge(he).unwrap().parent_loop)
                        .unwrap()
                        .face;
                    body.get_face(face).unwrap().surface
                };
                f(data.he_plus) == f(data.he_minus)
            };
            match carrier(body, *e).0 {
                Curve3::Line { origin, dir } => pick(origin, dir, same_surface),
                _ => false,
            }
        })
        .map(|(e, _)| e)
        .collect();
    assert_eq!(hits.len(), 1, "exactly one such edge, got {hits:?}");
    hits[0]
}

/// Split `edge` at its parameter midpoint; the new vertex.
fn split_mid(body: &mut Body<f64>, edge: EdgeKey) -> VertexKey {
    let (_, (t0, t1)) = carrier(body, edge);
    body.split_edge(edge, (t0 + t1) * 0.5, tol())
        .expect("the edge splits")
        .vertex
}

/// **Row 5: the refusal that remains is reachable, and true.** A vertex
/// with NO profile constraint — hand-made, since no door builds one:
/// the wedge's axis edge, between its two meridian caps, split at its
/// midpoint. The moved caps meet in a line parallel to the axis and
/// the vertex's station along it is nobody's, so the door refuses at
/// exactly that vertex with the corrected words.
#[test]
fn a_corner_with_no_profile_constraint_refuses_typed_on_a_hand_split_wedge() {
    let mut body = wedge(1.0, 2.0);
    let axis_edge = line_edge(&body, |o, d, _| {
        (o.x * o.x + o.z * o.z).sqrt() <= 1e-15 && d.dot(Vec3::unit_y()).abs() >= 1.0 - 1e-15
    });
    let split = split_mid(&mut body, axis_edge);
    let e = topo::shell(&body, T, tol()).expect_err("the split wedge refuses");
    let ShellError::Face { error, .. } = &e else {
        panic!("expected the axial door's refusal, got {e}");
    };
    let ReplaceFaceError::TogetherAxialCorner {
        vertex,
        surfaces,
        what,
    } = **error
    else {
        panic!("expected TogetherAxialCorner, got {error}");
    };
    assert_eq!(vertex, split, "the refusal names the split vertex");
    assert_eq!(surfaces, 2, "the two meridian caps, and nothing else");
    assert_eq!(
        what,
        "no profile constraint meets here, so no point in the meridian half-plane is determined"
    );
}

/// **Row 6: the LINE arm, on the operand that reaches it.** No door
/// builds a vertex whose only surface is a cylinder, cone or cap
/// plane, so the drum's cylinder seam is split by hand at mid-height:
/// the new vertex's faces are the one cylinder, its profile is the
/// wall's line `ρ = r`, no plane contains the axis at it, and its
/// image is the perpendicular foot on the moved line — `(r − t, h/2)`
/// exactly, the azimuth carried. The split changes no geometry, so the
/// wall's volume is the unsplit drum's closed form.
#[test]
fn the_line_arm_carries_a_hand_split_drum_seam_to_its_foot() {
    let (r, h) = (1.0, 2.0);
    let mut body = drum(r, h);
    let seam = line_edge(&body, |o, _, same_surface| same_surface && o.x > 0.0);
    let split = split_mid(&mut body, seam);
    let (rho0, h0) = axial(point(&body, split));
    assert!((rho0 - r).abs() <= 1e-15 && (h0 - h / 2.0).abs() <= 1e-15);
    let out =
        topo::shell(&body, T, tol()).unwrap_or_else(|e| panic!("the split drum shells, got {e}"));
    assert_eq!(topo::validate_geometric(&out.body, tol()), Ok(()), "tier 3");
    let (_, new) = out
        .naming
        .inner_vertices
        .iter()
        .copied()
        .find(|(old, _)| *old == split)
        .expect("the split vertex has an image");
    let (rho, hh) = axial(point(&out.body, new));
    assert!(
        (rho - (r - T)).abs() <= 1e-15 && hh == h / 2.0,
        "the foot on the moved wall: got ({rho}, {hh}), want ({}, {})",
        r - T,
        h / 2.0
    );
    let props = topo::mass_properties(&out.body, tol()).expect("props");
    let want = PI * (r * r * h - (r - T) * (r - T) * (h - 2.0 * T));
    assert!(
        (props.volume - want).abs() <= 1e-9 + props.volume_pad,
        "the drum's wall: got {} (pad {}), want {want}",
        props.volume,
        props.volume_pad
    );
}
