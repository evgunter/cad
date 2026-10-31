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
//! The hand-made rows split a wedge's axis edge and a drum's seam by
//! `Body::split_edge`, since no door builds a vertex with no profile
//! constraint, or one whose only surface is a cylinder; a hand-split
//! edge is construction state (tier 3's check 11), which `shell`'s
//! at-rest gate refuses before the axial door reads it, so those rows
//! read the door's arms at the door itself, `offset_charts_together`,
//! which takes construction state.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{FRAC_PI_2, PI};

use geom::{Curve3, Surface};
use geom_core::{Point2, Vec3};
use profile::test_support::bulge_loop;
use sweep::Revolution;
use topo::{Body, ReplaceFaceError};

use super::common::shell_operands::vessel;
use super::shell7_common::*;
use crate::common::charts::{charts, hollow_moves};
use crate::common::latitude_seam::{ring_on_cap, ring_on_wall};
use crate::common::oracles::{frustum_wedge_cavity, frustum_wedge_cavity_area};
use crate::common::stations::construction_state;
use crate::common::torus_walls::{assert_hollow_volume, disc_cut_volume};
use sweep::test_support::finished;

const R: f64 = 2.0;
const SMALL_R: f64 = 0.5;
const T: f64 = 0.05;

/// The same torus from the revolve door, its two seam vertices at the
/// minor angles `v` and `v + π` (radians from the outer equator, up).
fn revolved_torus(v: f64) -> Body<f64> {
    let (s, c) = v.sin_cos();
    let a = Point2::new(R + SMALL_R * c, SMALL_R * s);
    let b = Point2::new(R - SMALL_R * c, -SMALL_R * s);
    revolved(bulge_loop(vec![(a, 1.0), (b, 1.0)]), Revolution::Full)
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
    let out = topo::shell(&finished("the operand", body.clone(), tol()), T, tol())
        .unwrap_or_else(|e| panic!("{what}: the full torus shells, got {e}"));
    assert_eq!(
        topo::validate_geometric(&out.body, tol()),
        Ok(()),
        "{what}: tier 3"
    );
    assert_eq!(out.body.shells().count(), 2, "{what}: outer + cavity");
    let props = topo::mass_properties(
        &topo::test_support::finished("the measured body", topo::Body::clone(&out.body), tol()),
        tol(),
        topo::test_support::pinned_resolution(tol()),
    )
    .expect("props");
    let want = wall_volume(SMALL_R, T);
    assert!(
        (props.enclosure.volume - want).abs() <= 1e-9 + props.enclosure.volume_pad,
        "{what}: wall volume got {} (pad {}), want {want}",
        props.enclosure.volume,
        props.enclosure.volume_pad
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

/// **Row 5: the refusal that remains is reachable, and true — at the
/// direct door.** A vertex with NO profile constraint — hand-made,
/// since no door builds one: the wedge's axis edge, between its two
/// meridian caps, split at its midpoint. A hand-split edge is
/// construction state (tier 3's check 11), so `shell`'s at-rest gate
/// refuses it before the axial door reads it; the door itself takes
/// construction state, and there the moved caps meet in a line parallel
/// to the axis and the vertex's station along it is nobody's, so it
/// refuses at exactly that vertex with the corrected words.
#[test]
fn a_corner_with_no_profile_constraint_refuses_typed_at_the_direct_door() {
    let mut body = wedge(1.0, 2.0, FRAC_PI_2);
    let axis_edge = line_edge(&body, |o, d, _| {
        (o.x * o.x + o.z * o.z).sqrt() <= 1e-15 && d.dot(Vec3::unit_y()).abs() >= 1.0 - 1e-15
    });
    let split = split_mid(&mut body, axis_edge);
    assert_eq!(construction_state(&body, tol()), vec![split]);
    let error = direct_door(&body, T).expect_err("the split wedge refuses");
    let ReplaceFaceError::TogetherAxialCorner {
        vertex,
        surfaces,
        what,
    } = error
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

/// The axial door on its own, over a clone of `body` moved inward by
/// `t` on every chart: the moved body and the door's outcome. The door
/// takes construction state, which is what lets a hand-split operand
/// reach its arms.
fn direct_door(
    body: &Body<f64>,
    t: f64,
) -> Result<(Body<f64>, topo::OffsetOutcome), ReplaceFaceError<f64>> {
    let mut moved = body.clone();
    let band = geom_core::Band::linear(tol()).expect("band");
    topo::offset_charts_together(&mut moved, &hollow_moves(body, t), band, tol())
        .map(|outcome| (moved, outcome))
}

/// **Row 6: the LINE arm, on the operand that reaches it — the direct
/// door.** No door builds a vertex whose only surface is a cylinder,
/// cone or cap plane, so the drum's cylinder seam is split by hand at
/// mid-height: the new vertex's faces are the one cylinder, its profile
/// is the wall's line `ρ = r`, no plane contains the axis at it, and its
/// image is the perpendicular foot on the moved line, the azimuth
/// carried, with the two half-seams translated radially onto the moved
/// wall. A hand-split edge is construction state (tier 3's check 11), so
/// `shell`'s at-rest gate refuses it before the axial door reads it; the
/// door itself takes construction state, so the row reads the arm there.
/// The door ends with the join (`docs/DESIGN.md`, maximal edges), which
/// takes the image only because the two half-seams lie on one generator
/// of the moved wall; so the row reads the arm through that join and
/// the seam it leaves whole, at the split vertex's azimuth. The image's
/// height is not read: it does not survive the join.
///
/// A hand-made body owes its pcurve rows: `Body::split_edge` mints none
/// for its two children, so it is finished with `topo::mint_pcurves`
/// first, and check 11 is all that is left. The split changes no
/// geometry, so the moved body's volume is the unsplit drum's closed
/// form, `π(r − T)²(h − 2T)`.
#[test]
fn the_line_arm_carries_a_hand_split_drum_seam_to_its_foot_at_the_direct_door() {
    let (r, h) = (1.0, 2.0);
    let mut body = vessel(r, h);
    let seam = line_edge(&body, |o, d, same_surface| {
        same_surface && o.x > 0.0 && d.dot(Vec3::unit_y()).abs() >= 1.0 - 1e-15
    });
    let split = split_mid(&mut body, seam);
    topo::mint_pcurves(&mut body, tol()).expect("the split operand's pcurves mint");
    assert_eq!(construction_state(&body, tol()), vec![split], "the operand");
    let at = point(&body, split);
    let (rho0, h0) = axial(at);
    assert!((rho0 - r).abs() <= 1e-15 && (h0 - h / 2.0).abs() <= 1e-15);
    let (moved, outcome) =
        direct_door(&body, T).unwrap_or_else(|e| panic!("the split drum moves, got {e}"));
    assert_eq!(topo::validate_geometric(&moved, tol()), Ok(()), "tier 3");
    // The door ends with the join: the image stands between the two
    // half-seams on one generator of the moved wall, so the join takes
    // it, and the seam is whole again on the moved wall.
    let [join] = outcome.joins[..] else {
        panic!("one join, the split vertex's image: {:?}", outcome.joins);
    };
    assert!(moved.get_vertex(join.vertex).is_none(), "the image is dead");
    let (c, (t0, t1)) = carrier(&moved, join.kept);
    let Curve3::Line { origin, dir } = c else {
        panic!("a cylinder seam is a line, got {c:?}");
    };
    assert!(
        (axial(origin).0 - (r - T)).abs() <= 1e-15 && dir.dot(Vec3::unit_y()).abs() >= 1.0 - 1e-15,
        "the joined seam is a generator on the moved wall, got {c:?}"
    );
    // The azimuth is carried: the moved generator stands in the split
    // vertex's own meridian half-plane.
    let azimuth = |p: geom_core::Point3<f64>| p.z.atan2(p.x);
    assert!(
        (azimuth(origin) - azimuth(at)).abs() <= 1e-15,
        "the seam keeps the split vertex's azimuth: {} against {}",
        azimuth(origin),
        azimuth(at)
    );
    let he = moved.get_edge(join.kept).expect("the kept seam").he_plus;
    let ends = (
        point(&moved, moved.get_half_edge(he).expect("he").start),
        point(&moved, moved.half_edge_end(he).expect("end")),
    );
    assert!(c.eval(t0).distance(ends.0) <= 1e-13 && c.eval(t1).distance(ends.1) <= 1e-13);
    let span = (axial(ends.0).1 - axial(ends.1).1).abs();
    assert!(
        (span - (h - 2.0 * T)).abs() <= 1e-13,
        "the whole moved seam, cap to cap: got {span}"
    );
    let props = topo::mass_properties(
        &topo::test_support::finished("the measured body", topo::Body::clone(&moved), tol()),
        tol(),
        topo::test_support::pinned_resolution(tol()),
    )
    .expect("props");
    let want = PI * (r - T) * (r - T) * (h - 2.0 * T);
    assert!(
        (props.enclosure.volume - want).abs() <= 1e-9 + props.enclosure.volume_pad,
        "the moved drum: got {} (pad {}), want {want}",
        props.enclosure.volume,
        props.enclosure.volume_pad
    );
}

// ---------------------------------------------------------------------
// The seam-posture class, wider than the torus: every same-surface
// circle centred on the axis in a plane normal to it is a latitude
// circle, whatever its surface. The sphere's is DOOR-BUILT (two
// cocircular arcs keep a wall each). A line run's is not — a full
// revolve keeps no entity for a station inside a run — so the drum's
// and the frustum's rings are cut by hand through the Euler door on the
// door-built body (`common::latitude_seam`). Fixtures from the R1
// review lane (the collinear drum, the two-arc sphere), measured there
// as refusing at the seam arms.
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
    let out = topo::shell(&finished("the operand", body.clone(), tol()), t, tol())
        .unwrap_or_else(|e| panic!("{what}: shells, got {e}"));
    assert_eq!(
        topo::validate_geometric(&out.body, tol()),
        Ok(()),
        "{what}: tier 3"
    );
    assert_eq!(out.body.shells().count(), 2, "{what}: outer + cavity");
    let props = topo::mass_properties(
        &topo::test_support::finished("the measured body", topo::Body::clone(&out.body), tol()),
        tol(),
        topo::test_support::pinned_resolution(tol()),
    )
    .expect("props");
    assert!(
        (props.enclosure.volume - want).abs() <= 1e-9 + props.enclosure.volume_pad,
        "{what}: volume got {} (pad {}), want {want}",
        props.enclosure.volume,
        props.enclosure.volume_pad
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

/// The surface key of `body`'s faces wearing `pred`'s surface.
fn wall_key(body: &Body<f64>, pred: impl Fn(&Surface<f64>) -> bool) -> topo::SurfaceKey {
    body.faces()
        .find(|(_, f)| body.get_surface(f.surface).is_some_and(&pred))
        .expect("the wall")
        .1
        .surface
}

/// **A drum with a ring on its wall** (R1's fixture): the wall is one
/// cylinder in four faces, the mid-height ring is a same-surface
/// latitude circle, and the mid vertices' only surface is the cylinder
/// — the LINE arm. Shells to the drum's closed form; each mid vertex
/// moves to its foot `(r − t, h/2)`.
#[test]
fn a_collinear_wall_vertex_drum_shells_through_the_line_arm() {
    let (r, h, t) = (1.0, 2.0, 0.05);
    let mut body = polyline(&[(0.0, 0.0), (r, 0.0), (r, h), (0.0, h)], Revolution::Full);
    let wall = wall_key(&body, |s| matches!(s, Surface::Cylinder { .. }));
    ring_on_wall(&mut body, wall);
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
    let props = topo::mass_properties(
        &topo::test_support::finished("the measured body", topo::Body::clone(&cavity), tol()),
        tol(),
        topo::test_support::pinned_resolution(tol()),
    )
    .expect("props");
    assert!(
        (props.enclosure.volume - want).abs() <= 1e-9 + props.enclosure.volume_pad,
        "{what}: cavity volume got {} (pad {}), want {want}",
        props.enclosure.volume,
        props.enclosure.volume_pad
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

/// **A cap with a ring — the door takes it, and `shell` closes it.**
/// The top cap is one plane in three faces, its mid-radius ring a
/// same-surface latitude circle on a PLANE, and the ring's vertices'
/// only surface is that plane: the station-line arm.
/// Through the direct door the cavity is tier-3 valid at
/// `π(r−t)²(h−2t)` with the ring at its foot `(r/2, h − t)`; through
/// `shell` the same cavity is inserted and the thin solid closes at
/// `π r² h − π(r−t)²(h−2t)`, tier-3 valid and watertight. The void
/// door's graft re-certifies the reverted cavity's images, and
/// `Body::revert` mirrors the ring's plane `Chart` images with the
/// plane's frame (`v_ref = normal × u_ref` negates with the normal), so
/// the two half-circles — the only plane images with a non-zero `v`
/// channel — certify on the reverted plane (`shell9_probe`, the drum
/// rows; `revert_plane_charts` for the reversal itself).
#[test]
fn a_collinear_cap_vertex_drum_shells_to_its_closed_form() {
    let (r, h, t) = (1.0, 2.0, 0.05);
    let mut body = polyline(&[(0.0, 0.0), (r, 0.0), (r, h), (0.0, h)], Revolution::Full);
    ring_on_cap(&mut body, h, r, r / 2.0);
    let ring_foot = |(rho, hh): (f64, f64)| {
        ((hh - h).abs() <= 1e-12 && (rho - r / 2.0).abs() <= 1e-12).then_some((r / 2.0, h - t))
    };
    let cavity = PI * (r - t) * (r - t) * (h - 2.0 * t);
    cavity_at_closed_form("collinear cap", &body, t, cavity, ring_foot);
    let want = PI * r * r * h - cavity;
    let out = shells_with_one_surface_vertices("collinear cap", &body, t, want, ring_foot);
    let props = topo::mass_properties(
        &topo::test_support::finished("the measured body", topo::Body::clone(&out.body), tol()),
        tol(),
        topo::test_support::pinned_resolution(tol()),
    )
    .expect("props");
    assert!(
        (props.enclosure.volume - want).abs() <= 1e-12,
        "collinear cap: volume {} vs the closed form {want}",
        props.enclosure.volume
    );
    let mesh = mesh::tessellate(&out.body, 1e-3, tol()).expect("the thin drum tessellates");
    mesh::validate::check_mesh(&mesh).expect("watertight");
}

/// **A frustum with a ring on its wall**: the wall is one cone in four
/// faces, the mid ring a same-surface latitude circle on a CONE, and the
/// ring's vertices' only surface is that cone — the generator-line arm. The moved cone is the same cone with
/// its apex slid by `t / sin α` along the axis, so the image is the
/// foot of the old vertex on the moved generator.
#[test]
fn a_collinear_generator_vertex_frustum_shells_through_the_generator_arm() {
    let (r0, r1, h, t) = (1.0, 0.5, 2.0, 0.05);
    let mut body = polyline(
        &[(0.0, 0.0), (r0, 0.0), (r1, h), (0.0, h)],
        Revolution::Full,
    );
    let wall = wall_key(&body, |s| matches!(s, Surface::Cone { .. }));
    ring_on_wall(&mut body, wall);
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

/// **A sphere revolved from a run of arcs hollows to its closed form**, and a torus wall from one hollows.
/// The run is one meridian arc of a half turn (`crates/sweep/README.md`,
/// "Walls: one per run"), whose end the run's last arc states, so its
/// centre and ends sit an ulp off the exact half turn; the axial door
/// re-authors it about the moved circle on the turn it had, not on
/// whichever side of `atan2`'s cut the moved ends land — a sphere's
/// half turn at four stations, and a torus wall's arc run at, either
/// side of and well past the half turn.
#[test]
fn a_sphere_from_an_arc_run_hollows_to_its_closed_form() {
    let (r, t) = (1.0, 0.05);
    for v in [PI / 4.0, -PI / 4.0, 0.0, PI / 3.0] {
        let (s, c) = v.sin_cos();
        let body = revolved(
            bulge_loop(vec![
                (Point2::new(0.0, -r), ((FRAC_PI_2 + v) / 4.0).tan()),
                (Point2::new(r * c, r * s), ((FRAC_PI_2 - v) / 4.0).tan()),
                (Point2::new(0.0, r), 0.0),
            ]),
            Revolution::Full,
        );
        assert_eq!(
            body.faces().count(),
            2,
            "station {v}: one wall in two π-bands"
        );
        let mut cavity = body.clone();
        let band = geom_core::Band::linear(tol()).expect("band");
        topo::offset_charts_together(&mut cavity, &hollow_moves(&body, t), band, tol())
            .unwrap_or_else(|e| panic!("station {v}: the door takes it, got {e}"));
        assert_eq!(
            topo::validate_geometric(&cavity, tol()),
            Ok(()),
            "station {v}: tier 3"
        );
        let props = topo::mass_properties(
            &topo::test_support::finished("the measured body", topo::Body::clone(&cavity), tol()),
            tol(),
            topo::test_support::pinned_resolution(tol()),
        )
        .expect("props");
        let want = 4.0 / 3.0 * PI * (r - t).powi(3);
        assert!(
            (props.enclosure.volume - want).abs() <= 1e-9 + props.enclosure.volume_pad,
            "station {v}: cavity volume {} vs {want}",
            props.enclosure.volume
        );
    }
    // A torus wall's meridian run at, just under and just over the half
    // turn, and well past it: the re-author keeps each on its turn.
    for k in 1..=5usize {
        for (what, total) in [
            ("half turn", PI),
            ("under", PI - 1e-9),
            ("over", PI + 1e-9),
            ("major", 1.5 * PI),
            ("minor", 0.5 * PI),
        ] {
            let label = format!("torus {what} of {k} arcs");
            let mut chain =
                sweep::test_support::arc_run(Point2::new(2.0, 0.0), 1.0, -total / 2.0, total, k);
            chain.rotate_left(1);
            let body = revolved(bulge_loop(chain), Revolution::Full);
            let mut cavity = body.clone();
            let band = geom_core::Band::linear(tol()).expect("band");
            topo::offset_charts_together(&mut cavity, &hollow_moves(&body, t), band, tol())
                .unwrap_or_else(|e| panic!("{label}: the door takes it, got {e}"));
            assert_eq!(
                topo::validate_geometric(&cavity, tol()),
                Ok(()),
                "{label}: tier 3"
            );
            topo::mass_properties(
                &topo::test_support::finished(
                    "the measured body",
                    topo::Body::clone(&cavity),
                    tol(),
                ),
                tol(),
                topo::test_support::pinned_resolution(tol()),
            )
            .unwrap_or_else(|e| panic!("{label}: props {e:?}"));
        }
    }
}

/// **A sphere authored as two cocircular arcs shells to its closed
/// form.** R1's fixture ([`crate::common::latitude_seam::two_arc_sphere`],
/// the seam cut by hand): one sphere in four faces with a same-surface
/// LATITUDE seam at `v = π/4`, which the sphere's own seam arm could
/// not take and the latitude posture does. Through the direct door the
/// cavity is tier-3 valid at `4/3·π(r−t)³` (`3.591364001828731` in
/// f64; measured `…733`) with the seam vertices moved concentrically;
/// through `shell` the thin solid is tier-3 valid at
/// `4/3·π(r³ − (r−t)³) = 0.5974262029576595` to `1e-12` with the same
/// vertices at the same images, and tessellates watertight. What sits
/// between the two is the void door's `Transfers` posture: the
/// reverted cavity's stored pcurve rows arrive on the twins verbatim
/// and continuous — the reversal moves each loop's anchor with its
/// direction, so the grafted body is tier-3 valid on the carried rows
/// (`shell9_probe`, the sphere rows) — and the verb's closing mint
/// re-derives them all the same, the `Transfers` row's contract.
#[test]
fn a_two_arc_sphere_shells_to_its_closed_form() {
    let (r, t) = (1.0, 0.05);
    let s = (PI / 4.0).sin();
    let body = crate::common::latitude_seam::two_arc_sphere();
    let concentric = |(rho, hh): (f64, f64)| {
        (rho > 1e-6 && (hh - r * s).abs() <= 1e-9).then(|| {
            let n = rho.hypot(hh);
            (rho / n * (r - t), hh / n * (r - t))
        })
    };
    let cavity = 4.0 / 3.0 * PI * (r - t).powi(3);
    cavity_at_closed_form("two-arc sphere", &body, t, cavity, concentric);
    let want = 4.0 / 3.0 * PI * (r.powi(3) - (r - t).powi(3));
    let out = shells_with_one_surface_vertices("two-arc sphere", &body, t, want, concentric);
    let props = topo::mass_properties(
        &topo::test_support::finished("the measured body", topo::Body::clone(&out.body), tol()),
        tol(),
        topo::test_support::pinned_resolution(tol()),
    )
    .expect("props");
    assert!(
        (props.enclosure.volume - want).abs() <= 1e-12,
        "two-arc sphere: volume {} vs the closed form {want}",
        props.enclosure.volume
    );
    let mesh = mesh::tessellate(&out.body, 1e-3, tol()).expect("the thin sphere tessellates");
    mesh::validate::check_mesh(&mesh).expect("watertight");
}

// ---------------------------------------------------------------------
// The refusals that remain, and a standing one made visible.
// ---------------------------------------------------------------------

/// **A line profile beside one meridian cap refuses, where its circle
/// twin is carried** (R1's row, R2's `p1` — the same operand from both
/// review lanes): the quarter-turn wedge's wall/cap generator, split
/// by hand at mid-height, is a vertex whose surfaces are exactly the
/// cylinder and one meridian cap. A hand-split edge is construction
/// state (tier 3's check 11), so `shell`'s at-rest gate refuses it
/// before the axial door reads it; at the door itself, which takes
/// construction state, carrying its station along the line is a
/// convention the door declines, and it says so.
#[test]
fn a_line_profile_beside_one_meridian_cap_refuses_at_the_direct_door() {
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
    assert_eq!(construction_state(&body, tol()), vec![split]);
    let error = direct_door(&body, 0.05).expect_err("refuses");
    let ReplaceFaceError::TogetherAxialCorner {
        vertex,
        surfaces,
        what,
    } = error
    else {
        panic!("not a corner refusal: {error}");
    };
    assert_eq!(vertex, split);
    assert_eq!(surfaces, 2);
    assert!(
        what.starts_with("a line profile and a plane parallel to the axis meet here off the axis"),
        "got {what:?}"
    );
}

/// **A partial two-arc torus hollows to its measured volume** (R1's
/// row, flipped again): the quarter-turn elbow of the two-arc profile
/// has a [torus, meridian cap] corner whose azimuth the MOVED cap fixes
/// — off the sketch plane — and the rim edge between the torus and the
/// cap is minted as the spiric it is, its window read forward of its
/// start (`offset_axial_edge_window`). Its two equator seams are
/// `RevolvedPoint` declarations whose ends the moved caps turn about
/// the axis, and they re-author onto the moved corners. The hollow
/// passes tier 3, check 7 measuring its spiric-bounded faces, and its
/// volume is the disc's quarter turn less the inset disc's cut revolve.
///
/// The old doors, verbatim: check 7's `VolumeUncomputable` at a
/// spiric-bounded cap (`Unimplemented`); before it `Face { error: Op {
/// edge: None, error: RechartFalsifies { edge: EdgeKey(1v1), error:
/// IntervalNotForward { margin: −1.413716694115407 } } } }` — the rim
/// window read backwards by `π·r′` on `atan2`'s branch cut; before it
/// the seams' re-author (`offset_axial_reauthor_plane`), and before
/// that the latitude mint's `offset_axial_centre`.
#[test]
fn a_partial_two_arc_torus_hollows_to_its_measured_volume() {
    let (big_r, r, t) = (2.0, 0.5, 0.05);
    let body = revolved(
        bulge_loop(vec![
            (Point2::new(big_r + r, 0.0), 1.0),
            (Point2::new(big_r - r, 0.0), 1.0),
        ]),
        Revolution::Partial(FRAC_PI_2),
    );
    assert_eq!(topo::validate_geometric(&body, tol()), Ok(()));
    let hollow = topo::shell(&finished("the operand", body.clone(), tol()), t, tol())
        .unwrap_or_else(|e| panic!("the two-arc torus hollows, got {e:?}"))
        .body;
    assert_hollow_volume(
        "the partial two-arc torus",
        &hollow,
        disc_cut_volume(FRAC_PI_2, t, 0.0, big_r, r)
            - disc_cut_volume(FRAC_PI_2, t, 2.0, big_r, r - t),
    );
}

/// **A three-quarter-turn cone frustum hollows**: the cavity moves the
/// meridian caps off the axis, so each wall/cap edge's moved section is
/// a hyperbola, minted as the moved pair's arc between the moved
/// corners; it rests because the plane×cone section states the bounds
/// that retire its tube. The hollow passes
/// tier 3 and measures the operand less the reflex wedge's cavity
/// ([`frustum_wedge_cavity`]).
#[test]
fn a_three_quarter_turn_cone_frustum_hollows() {
    let (r0, r1, h, t) = (1.0, 0.5, 2.0, 0.05);
    let turn = 3.0 * FRAC_PI_2;
    let body = polyline(
        &[(0.0, 0.0), (r0, 0.0), (r1, h), (0.0, h)],
        Revolution::Partial(turn),
    );
    assert_eq!(topo::validate_geometric(&body, tol()), Ok(()));
    let hollow = topo::shell(&finished("the operand", body.clone(), tol()), t, tol())
        .unwrap_or_else(|e| panic!("the three-quarter frustum hollows, got {e}"))
        .body;
    assert_eq!(topo::validate_geometric(&hollow, tol()), Ok(()), "tier 3");
    let frustum = core::f64::consts::PI * h * (r0 * r0 + r0 * r1 + r1 * r1) / 3.0;
    assert_hollow_volume(
        "the three-quarter frustum",
        &hollow,
        turn / core::f64::consts::TAU * frustum - frustum_wedge_cavity(r0, r1, h, turn, t),
    );
    // The boundary's area, the cavity's cone cap bounded by the
    // hyperbolic arcs read through the certified rim lane about its apex.
    let tol = tol();
    let measured = topo::test_support::pinned_volume(
        "props",
        &topo::test_support::finished("the measured body", topo::Body::clone(&hollow), tol),
        tol,
    );
    let area = frustum_wedge_cavity_area(r0, r1, h, turn, 0.0)
        + frustum_wedge_cavity_area(r0, r1, h, turn, t);
    println!(
        "[measured] the three-quarter frustum: area {} oracle {area} pad {:e}",
        measured.surface_area, measured.area_pad
    );
    assert!(
        (measured.surface_area - area).abs() <= measured.area_pad + 1e-12 * area,
        "the three-quarter frustum's boundary is {area}, got {}",
        measured.surface_area
    );
}

/// **A partial frustum moving only its cone keeps every cone–cap edge a
/// generator line.** The meridian caps stay at `0` and hold the axis, so
/// each holds the moved cone's apex too, and the moved pair's section is
/// its generator pair: the axial door's cone-and-cap arm picks the line
/// the old edge stood beside and runs it the old way, read at the door
/// itself (`offset_charts_together`). At a quarter turn
/// and a three-quarter turn, outward and inward, the body builds, passes
/// tier 3, carries a `Line` on every edge between the cone and a
/// meridian cap, and holds the frustum's closed form at the moved radii
/// `r + d/cos α` over its share `turn/2π` of the full turn.
#[test]
fn a_partial_frustum_moving_only_its_cone_keeps_its_generators() {
    let (r0, r1, h) = (1.0_f64, 0.5_f64, 2.0_f64);
    let alpha = ((r0 - r1) / h).atan();
    for (what, turn) in [
        ("a quarter turn", FRAC_PI_2),
        ("a three-quarter turn", 3.0 * FRAC_PI_2),
    ] {
        for d in [0.05, -0.05] {
            let mut body = polyline(
                &[(0.0, 0.0), (r0, 0.0), (r1, h), (0.0, h)],
                Revolution::Partial(turn),
            );
            let surface = |body: &Body<f64>, f: topo::FaceKey| {
                body.get_surface(body.get_face(f).unwrap().surface)
                    .unwrap()
                    .clone()
            };
            // Every chart at `0` but the cone's, moved `d` out of the
            // material along its stored normal.
            let moves: Vec<topo::ChartMove<f64>> = charts(&body)
                .into_iter()
                .map(|faces| {
                    let face = body.get_face(faces[0]).expect("a live face");
                    let distance = match body.get_surface(face.surface) {
                        Some(Surface::Cone { .. }) if face.sense => d,
                        Some(Surface::Cone { .. }) => -d,
                        _ => 0.0,
                    };
                    topo::ChartMove { faces, distance }
                })
                .collect();
            assert_eq!(
                moves.iter().filter(|m| m.distance != 0.0).count(),
                1,
                "{what}: one cone chart"
            );
            let band = geom_core::Band::linear(tol()).expect("band");
            topo::offset_charts_together(&mut body, &moves, band, tol())
                .unwrap_or_else(|e| panic!("{what}, d {d}: the cone-only move refused: {e}"));
            assert_eq!(
                topo::validate_geometric(&body, tol()),
                Ok(()),
                "{what}, d {d}: tier 3"
            );
            let mut generators = 0;
            for (edge, data) in body.edges() {
                let [f, g] = topo::edge_faces(&body, edge);
                let (sf, sg) = (surface(&body, f), surface(&body, g));
                let meridian = |s: &Surface<f64>| matches!(s, Surface::Plane { normal, .. } if normal.y.abs() <= 1e-12);
                let cone_cap = (matches!(sf, Surface::Cone { .. }) && meridian(&sg))
                    || (matches!(sg, Surface::Cone { .. }) && meridian(&sf));
                if cone_cap {
                    generators += 1;
                    let carrier = body
                        .get_curve_geom(data.curve)
                        .and_then(topo::CurveGeom::certified)
                        .map(|c| c.carrier().clone());
                    assert!(
                        matches!(carrier, Some(Curve3::Line { .. })),
                        "{what}, d {d}: the cone–cap edge {edge:?} is a generator line, got \
                         {carrier:?}"
                    );
                }
            }
            assert_eq!(generators, 2, "{what}, d {d}: two cone–cap edges");
            let (m0, m1) = (r0 + d / alpha.cos(), r1 + d / alpha.cos());
            let want = turn * h * (m0 * m0 + m0 * m1 + m1 * m1) / 6.0;
            let got = topo::test_support::pinned_volume(
                "props",
                &finished("the moved body", body, tol()),
                tol(),
            )
            .volume;
            println!("[measured] {what}, d {d}: volume {got} against {want}");
            assert!(
                (got - want).abs() <= 1e-12 * want,
                "{what}, d {d}: volume {got} against the closed form {want}"
            );
        }
    }
}
