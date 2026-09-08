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

use core::f64::consts::{FRAC_PI_2, PI};

use geom::{Curve3, Surface};
use geom_core::Vec3;
use profile::{ProfileLoop, ProfileVertex, RawLoop};
use sweep::Revolution;
use topo::{Body, ReplaceFaceError, ShellError};

use super::shell7_common::*;

const R: f64 = 2.0;
const SMALL_R: f64 = 0.5;
const T: f64 = 0.05;

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
    shelled_to_closed_form("tube torus", &tube_torus(R, SMALL_R));
}

/// **Row 3: the seam vertex moved exactly.** Each of the tube's two
/// vertices lands at meridian distance `r − t` from the tube centre
/// `(R, 0)` at its OLD azimuth, and the four cavity edges are the two
/// circles the module names — the meridian seams concentric about the
/// tube centre at `r − t`, the equator seams about the axis at
/// `R ± (r − t)` — with their endpoints on them.
#[test]
fn the_seam_vertex_moves_concentrically_at_its_old_azimuth() {
    let body = tube_torus(R, SMALL_R);
    let out = shelled_to_closed_form("tube torus", &body);
    let cavity = &out.body;
    assert_eq!(out.naming.inner_vertices.len(), 2);
    for &(new, old) in &out.naming.inner_vertices {
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
    for &(e, _) in &out.naming.inner_edges {
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
        for &(new, old) in &out.naming.inner_vertices {
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

/// **Row 5: the refusal that remains is reachable, and true.** A vertex
/// with NO profile constraint — hand-made, since no door builds one:
/// the wedge's axis edge, between its two meridian caps, split at its
/// midpoint. The moved caps meet in a line parallel to the axis and
/// the vertex's station along it is nobody's, so the door refuses at
/// exactly that vertex with the corrected words.
#[test]
fn a_corner_with_no_profile_constraint_refuses_typed_on_a_hand_split_wedge() {
    let mut body = wedge(1.0, 2.0, FRAC_PI_2);
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
/// exactly, the azimuth carried — with the two half-seams translated
/// radially onto the moved wall. The split changes no geometry, so the
/// wall's volume is the unsplit drum's closed form.
///
/// A hand-made operand owes what every door's operand has: its pcurve
/// rows. `Body::split_edge` mints none for its two children, and
/// `shell`'s closing tier 3 says so (`Pcurve MissingCache` on both,
/// measured) — so the operand is finished with `topo::mint_pcurves`,
/// the pass every builder runs last, before it is shelled.
#[test]
fn the_line_arm_carries_a_hand_split_drum_seam_to_its_foot() {
    let (r, h) = (1.0, 2.0);
    let mut body = drum(r, h);
    let seam = line_edge(&body, |o, d, same_surface| {
        same_surface && o.x > 0.0 && d.dot(Vec3::unit_y()).abs() >= 1.0 - 1e-15
    });
    let split = split_mid(&mut body, seam);
    topo::mint_pcurves(&mut body, tol()).expect("the split operand's pcurves mint");
    assert_eq!(
        topo::validate_geometric(&body, tol()),
        Ok(()),
        "the operand"
    );
    let (rho0, h0) = axial(point(&body, split));
    assert!((rho0 - r).abs() <= 1e-15 && (h0 - h / 2.0).abs() <= 1e-15);
    let out =
        topo::shell(&body, T, tol()).unwrap_or_else(|e| panic!("the split drum shells, got {e}"));
    let cavity = &out.body;
    assert_eq!(topo::validate_geometric(cavity, tol()), Ok(()), "tier 3");
    let (new, _) = out
        .naming
        .inner_vertices
        .iter()
        .copied()
        .find(|(_, old)| *old == split)
        .expect("the split vertex has an image");
    let (rho, hh) = axial(point(cavity, new));
    assert!(
        (rho - (r - T)).abs() <= 1e-15 && hh == h / 2.0,
        "the foot on the moved wall: got ({rho}, {hh}), want ({}, {})",
        r - T,
        h / 2.0
    );
    // The two half-seams: generator lines on the moved wall, ending at
    // the split vertex's image.
    let mut halves = 0;
    for (e, data) in cavity.edges() {
        let start = cavity.get_half_edge(data.he_plus).expect("he").start;
        let end = cavity.half_edge_end(data.he_plus).expect("end");
        if start != new && end != new {
            continue;
        }
        halves += 1;
        let (c, (t0, t1)) = carrier(cavity, e);
        let Curve3::Line { origin, dir } = c else {
            panic!("{e:?}: a cylinder seam is a line, got {c:?}");
        };
        assert!(
            (axial(origin).0 - (r - T)).abs() <= 1e-15
                && dir.dot(Vec3::unit_y()).abs() >= 1.0 - 1e-15,
            "{e:?}: a generator on the moved wall, got {c:?}"
        );
        let ends = (point(cavity, start), point(cavity, end));
        assert!(c.eval(t0).distance(ends.0) <= 1e-13 && c.eval(t1).distance(ends.1) <= 1e-13);
    }
    assert_eq!(halves, 2, "two half-seams end at the split vertex");
    let props = topo::mass_properties(cavity, tol()).expect("props");
    let want = PI * (r * r * h - (r - T) * (r - T) * (h - 2.0 * T));
    assert!(
        (props.volume - want).abs() <= 1e-9 + props.volume_pad,
        "the drum's wall: got {} (pad {}), want {want}",
        props.volume,
        props.volume_pad
    );
}

// ---------------------------------------------------------------------
// The seam-posture class, wider than the torus: every same-surface
// circle centred on the axis in a plane normal to it is a latitude
// circle, whatever its surface. The fixtures below are DOOR-BUILT
// (a collinear profile vertex is a legal same-carrier continuation),
// so each is also a door-built row for the carried corner arm on that
// profile kind. Fixtures from the R1 review lane (the collinear drum,
// the two-arc sphere), measured there as refusing at the seam arms.
// ---------------------------------------------------------------------

/// Tier 3, two shells, the volume closed form, and every vertex at
/// `mid_h` moved to its foot / concentric point — the shape every
/// collinear-vertex row asserts.
fn shells_with_one_surface_vertices(
    what: &str,
    body: &Body<f64>,
    t: f64,
    want: f64,
    mid: impl Fn((f64, f64)) -> Option<(f64, f64)>,
) -> topo::Shelled<f64> {
    assert_eq!(
        topo::validate_geometric(body, tol()),
        Ok(()),
        "{what}: operand"
    );
    let out = topo::shell(body, t, tol()).unwrap_or_else(|e| panic!("{what}: shells, got {e}"));
    assert_eq!(
        topo::validate_geometric(&out.body, tol()),
        Ok(()),
        "{what}: tier 3"
    );
    assert_eq!(out.body.shells().count(), 2, "{what}: outer + cavity");
    let props = topo::mass_properties(&out.body, tol()).expect("props");
    assert!(
        (props.volume - want).abs() <= 1e-9 + props.volume_pad,
        "{what}: volume got {} (pad {}), want {want}",
        props.volume,
        props.volume_pad
    );
    let mut seen = 0;
    for &(new, old) in &out.naming.inner_vertices {
        let Some(image) = mid(axial(point(body, old))) else {
            continue;
        };
        seen += 1;
        assert_eq!(
            distinct_surfaces_at(body, old),
            1,
            "{what}: {old:?} is a one-surface vertex"
        );
        let (rho, h) = axial(point(&out.body, new));
        assert!(
            (rho - image.0).abs() <= 1e-15 && (h - image.1).abs() <= 1e-15,
            "{what}: {old:?} → ({rho}, {h}), want {image:?}"
        );
    }
    assert!(seen >= 1, "{what}: at least one one-surface vertex");
    out
}

/// **A drum with a collinear wall vertex** (R1's fixture): the wall is
/// one cylinder in four faces, the mid-height ring is a same-surface
/// latitude circle, and the mid vertices' only surface is the cylinder
/// — the LINE arm through a door-built operand. Shells to the drum's
/// closed form; each mid vertex moves to its foot `(r − t, h/2)`.
#[test]
fn a_collinear_wall_vertex_drum_shells_through_the_line_arm() {
    let (r, h, t) = (1.0, 2.0, 0.05);
    let body = polyline(
        &[(0.0, 0.0), (r, 0.0), (r, h / 2.0), (r, h), (0.0, h)],
        Revolution::Full,
    );
    let want = PI * (r * r * h - (r - t) * (r - t) * (h - 2.0 * t));
    shells_with_one_surface_vertices("collinear drum", &body, t, want, |(rho, hh)| {
        ((hh - h / 2.0).abs() <= 1e-12 && (rho - r).abs() <= 1e-12).then_some((r - t, h / 2.0))
    });
}

/// The cavity of a one-solid operand through the direct door, tier 3
/// and its closed-form volume, with the one-surface vertices at their
/// images — the door's own half of a row whose `shell` half stops
/// downstream of it.
fn cavity_at_closed_form(
    what: &str,
    body: &Body<f64>,
    t: f64,
    want: f64,
    mid: impl Fn((f64, f64)) -> Option<(f64, f64)>,
) -> Body<f64> {
    assert_eq!(
        topo::validate_geometric(body, tol()),
        Ok(()),
        "{what}: operand"
    );
    let mut cavity = body.clone();
    let band = geom_core::Band::linear(tol()).expect("band");
    topo::offset_charts_together(&mut cavity, &hollow_moves(body, t), band, tol())
        .unwrap_or_else(|e| panic!("{what}: the door takes it, got {e}"));
    assert_eq!(
        topo::validate_geometric(&cavity, tol()),
        Ok(()),
        "{what}: cavity tier 3"
    );
    let props = topo::mass_properties(&cavity, tol()).expect("props");
    assert!(
        (props.volume - want).abs() <= 1e-9 + props.volume_pad,
        "{what}: cavity volume got {} (pad {}), want {want}",
        props.volume,
        props.volume_pad
    );
    let mut seen = 0;
    for (v, _) in body.vertices() {
        let Some(image) = mid(axial(point(body, v))) else {
            continue;
        };
        seen += 1;
        assert_eq!(
            distinct_surfaces_at(body, v),
            1,
            "{what}: {v:?} is a one-surface vertex"
        );
        let (rho, h) = axial(point(&cavity, v));
        assert!(
            (rho - image.0).abs() <= 1e-15 && (h - image.1).abs() <= 1e-15,
            "{what}: {v:?} → ({rho}, {h}), want {image:?}"
        );
    }
    assert!(seen >= 1, "{what}: at least one one-surface vertex");
    cavity
}

/// **A cap with a collinear vertex — the door takes it, `shell` stops
/// at void insertion (measured, a STOP).** The top cap is one plane in
/// four faces, its mid-radius ring a same-surface latitude circle on a
/// PLANE, and the ring's vertices' only surface is that plane: the
/// station-line arm, door-built. Through the direct door the cavity is
/// tier-3 valid at `π(r−t)²(h−2t)` with the ring at its foot
/// `(r/2, h − t)`. Through `shell` the same cavity is refused by the
/// void-insertion door's graft re-certification (`ChartResidual`), a
/// gap downstream of the corner and the carrier —
/// `work/shell/void-insertion-refuses-a-cavity-with-a-same-surface-
/// latitude-seam.md` — pinned here rather than widened around.
#[test]
fn a_collinear_cap_vertex_drum_is_taken_by_the_door_and_stops_at_void_insertion() {
    let (r, h, t) = (1.0, 2.0, 0.05);
    let body = polyline(
        &[(0.0, 0.0), (r, 0.0), (r, h), (r / 2.0, h), (0.0, h)],
        Revolution::Full,
    );
    let want = PI * (r - t) * (r - t) * (h - 2.0 * t);
    cavity_at_closed_form("collinear cap", &body, t, want, |(rho, hh)| {
        ((hh - h).abs() <= 1e-12 && (rho - r / 2.0).abs() <= 1e-12).then_some((r / 2.0, h - t))
    });
    let e = topo::shell(&body, t, tol()).expect_err("measured: stops at void insertion");
    assert!(matches!(e, ShellError::Insert { .. }), "got {e}");
}

/// **A frustum with a collinear generator vertex**: the wall is one
/// cone in four faces, the mid ring a same-surface latitude circle on
/// a CONE, and the ring's vertices' only surface is that cone — the
/// generator-line arm, door-built. The moved cone is the same cone with
/// its apex slid by `t / sin α` along the axis, so the image is the
/// foot of the old vertex on the moved generator.
#[test]
fn a_collinear_generator_vertex_frustum_shells_through_the_generator_arm() {
    let (r0, r1, h, t) = (1.0, 0.5, 2.0, 0.05);
    let body = polyline(
        &[
            (0.0, 0.0),
            (r0, 0.0),
            ((r0 + r1) / 2.0, h / 2.0),
            (r1, h),
            (0.0, h),
        ],
        Revolution::Full,
    );
    // The cavity: the frustum's own closed form at the inset radii.
    let tan_a = (r0 - r1) / h;
    let alpha = tan_a.atan();
    let apex = r0 / tan_a;
    let apex_in = apex - t / alpha.sin();
    let (c0, c1) = ((apex_in - t) * tan_a, (apex_in - (h - t)) * tan_a);
    let frustum = |a: f64, b: f64, hh: f64| PI * hh / 3.0 * (a * a + a * b + b * b);
    let want = frustum(r0, r1, h) - frustum(c0, c1, h - 2.0 * t);
    // The foot of `(ρ, h/2)` on the moved generator `ρ cos α − (h_apex' − h) sin α = 0`
    // read as `n·(ρ, h) = c` with `n = (cos α, sin α)`, `c = h_apex' sin α`.
    let (sin_a, cos_a) = alpha.sin_cos();
    shells_with_one_surface_vertices("collinear frustum", &body, t, want, |(rho, hh)| {
        ((hh - h / 2.0).abs() <= 1e-12 && (rho - (r0 + r1) / 2.0).abs() <= 1e-12).then(|| {
            let gap = cos_a * rho + sin_a * hh - apex_in * sin_a;
            (rho - gap * cos_a, hh - gap * sin_a)
        })
    });
}

/// **A sphere authored as two cocircular arcs — the door takes it,
/// `shell` stops at the assembly (measured, a STOP).** R1's fixture:
/// one sphere in four faces with a same-surface LATITUDE seam at
/// `v = π/4`, which the sphere's own seam arm could not take and the
/// latitude posture does. Through the direct door the cavity is
/// tier-3 valid at `4/3·π(r−t)³` with the seam vertices moved
/// concentrically. Through `shell` the assembled thin solid fails tier
/// 3 with a pcurve `LoopDiscontinuity` on a grafted half-edge — the
/// same downstream gap as the collinear cap's, same item — pinned
/// here rather than widened around.
#[test]
fn a_two_arc_sphere_is_taken_by_the_door_and_stops_at_the_assembly() {
    let (r, t) = (1.0, 0.05);
    let v = PI / 4.0;
    let (s, c) = v.sin_cos();
    let body = revolved(
        ProfileLoop::new(vec![
            ProfileVertex::new(p2(0.0, -r), ((FRAC_PI_2 + v) / 4.0).tan()),
            ProfileVertex::new(p2(r * c, r * s), ((FRAC_PI_2 - v) / 4.0).tan()),
            ProfileVertex::new(p2(0.0, r), 0.0),
        ]),
        Revolution::Full,
    );
    let want = 4.0 / 3.0 * PI * (r - t).powi(3);
    cavity_at_closed_form("two-arc sphere", &body, t, want, |(rho, hh)| {
        (rho > 1e-6 && (hh - r * s).abs() <= 1e-9).then(|| {
            let n = rho.hypot(hh);
            (rho / n * (r - t), hh / n * (r - t))
        })
    });
    let e = topo::shell(&body, t, tol()).expect_err("measured: stops at the assembly");
    let ShellError::NotValid { errors } = &e else {
        panic!("expected the assembled body's tier 3, got {e}");
    };
    assert!(
        errors
            .iter()
            .any(|f| format!("{f:?}").contains("LoopDiscontinuity")),
        "the measured pcurve loop discontinuity, got {errors:?}"
    );
}

// ---------------------------------------------------------------------
// The refusals that remain, and a standing one made visible.
// ---------------------------------------------------------------------

/// **A line profile beside one meridian cap refuses, where its circle
/// twin is carried** (R1's row, R2's `p1` — the same operand from both
/// review lanes): the quarter-turn wedge's wall/cap generator, split
/// by hand at mid-height, is a vertex whose surfaces are exactly the
/// cylinder and one meridian cap. Carrying its station along the line
/// is a convention the door declines, and it says so.
#[test]
fn a_line_profile_beside_one_meridian_cap_refuses_on_a_hand_split_wedge() {
    let (r, h) = (1.0, 2.0);
    let mut body = wedge(r, h, FRAC_PI_2);
    let generator = line_edge(&body, |o, d, same| {
        !same
            && o.x > 0.5
            && o.z.abs() < 1e-12
            && (axial(o).0 - r).abs() <= 1e-12
            && d.dot(Vec3::unit_y()).abs() >= 1.0 - 1e-15
    });
    let split = split_mid(&mut body, generator);
    topo::mint_pcurves(&mut body, tol()).expect("pcurves");
    assert_eq!(distinct_surfaces_at(&body, split), 2, "wall + cap");
    let e = topo::shell(&body, 0.05, tol()).expect_err("refuses");
    let (vertex, surfaces, what) =
        corner_refusal(&e).unwrap_or_else(|| panic!("not a corner refusal: {e}"));
    assert_eq!(vertex, split);
    assert_eq!(surfaces, 2);
    assert!(
        what.starts_with("a line profile and a plane containing the axis meet here off the axis"),
        "got {what:?}"
    );
}

/// **A partial two-arc torus stops at its spiric rim** (R1's row): the
/// quarter-turn elbow of the two-arc profile has a [torus, meridian
/// cap] corner whose azimuth the MOVED cap fixes — off the sketch
/// plane — but the rim edge between the torus and the cap has no
/// carrier (the klein elbow's spiric wall), and the door refuses there
/// before any latitude seam or its re-author is reached. So no
/// door-built operand reaches the re-author's out-of-plane decide; its
/// presence is its row.
#[test]
fn a_partial_two_arc_torus_refuses_at_its_spiric_rim() {
    let (big_r, r) = (2.0, 0.5);
    let body = revolved(
        RawLoop::new(vec![
            ProfileVertex::new(p2(big_r + r, 0.0), 1.0),
            ProfileVertex::new(p2(big_r - r, 0.0), 1.0),
        ]),
        Revolution::Partial(FRAC_PI_2),
    );
    assert_eq!(topo::validate_geometric(&body, tol()), Ok(()));
    let e = topo::shell(&body, 0.05, tol()).expect_err("refuses");
    let (_, what) = edge_refusal(&e).unwrap_or_else(|| panic!("not an edge refusal: {e}"));
    assert_eq!(
        what,
        "a circular edge between two charts whose centre is off the axis"
    );
}

/// **A three-quarter-turn cone frustum refuses `TogetherEdgeDisagreement`**
/// (R1's finding, `work/shell/partial-cone-frustum-three-quarter-turn-
/// refuses-edge-disagreement.md`): `sf2b_axial` only turns the frustum
/// a quarter, and at three quarters the wall/cap generator's two ends
/// disagree by more than a millimetre — pinned here so the standing
/// refusal is visible, with its measured gap.
#[test]
fn a_three_quarter_turn_cone_frustum_refuses_edge_disagreement() {
    let body = polyline(
        &[(0.0, 0.0), (1.0, 0.0), (0.5, 2.0), (0.0, 2.0)],
        Revolution::Partial(3.0 * FRAC_PI_2),
    );
    assert_eq!(topo::validate_geometric(&body, tol()), Ok(()));
    let e = topo::shell(&body, 0.05, tol()).expect_err("measured: refuses");
    let ShellError::Face { error, .. } = &e else {
        panic!("expected the axial door's refusal, got {e}");
    };
    let ReplaceFaceError::TogetherEdgeDisagreement { gap, .. } = **error else {
        panic!("expected TogetherEdgeDisagreement, got {error}");
    };
    println!("[measured] three-quarter frustum: edge disagreement gap = {gap}");
    assert!(
        (1.0e-3..2.0e-3).contains(&gap),
        "the measured millimetre-scale gap, got {gap}"
    );
}
