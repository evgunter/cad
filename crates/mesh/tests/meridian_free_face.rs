//! **A curved face whose single loop carries no meridian refuses
//! typed** ([`TessellateError::MeridianFreeCurvedFace`] — its doc is the
//! home of what the state is and why), before anything is emitted, and
//! the seamed statement of the same solid meshes watertight.
//!
//! Without the refusal `tessellate` answers with a hole where the face
//! is: caught by the cross-face census where debug assertions run,
//! returned as `Ok` where they do not. Every refusal row below therefore
//! goes red with the guard removed, either way — except the torus row,
//! which says so itself.
//!
//! The bodies come through the Euler doors
//! (`witness_bodies::one_circle_cut`); the STEP route to the same state
//! is `step-import/tests/meridian_free_cap.rs`. What this file pins
//! about the kernel's own verbs is the control row and no more: the
//! revolved dome and one plane-cut ball are stated on meridians through
//! a pole vertex.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use common::witness_bodies::one_circle_cut;
use common::*;
use core::f64::consts::{FRAC_PI_4, PI};
use geom::{Curve3, Surface};
use geom_brep::SurfaceKind;
use geom_core::{Point3, Tol, Vec3};
use mesh::TessellateError;
use profile::{ProfileLoop, RawLoop, test_support::bulge_loop};
use sweep::{Extrusion, Revolution, extrude, revolve};
use topo::{Body, FaceKey};

fn p3(x: f64, y: f64, z: f64) -> Point3<f64> {
    Point3::new(x, y, z)
}
fn z_axis() -> Vec3<f64> {
    Vec3::new(0.0, 0.0, 1.0)
}
fn x_axis() -> Vec3<f64> {
    Vec3::new(1.0, 0.0, 0.0)
}

/// The sphere of radius `r` about +Z at the origin.
fn sphere(r: f64) -> Surface<f64> {
    Surface::Sphere {
        center: p3(0.0, 0.0, 0.0),
        radius: r,
        axis: z_axis(),
        u_ref: x_axis(),
    }
}

/// The plane `z = z0`, outward normal up or down.
fn plane(z0: f64, up: bool) -> Surface<f64> {
    Surface::Plane {
        origin: p3(0.0, 0.0, z0),
        normal: if up { z_axis() } else { -z_axis() },
        u_ref: x_axis(),
    }
}

/// The rim about +Z at height `z`, radius `r`.
fn rim(z: f64, r: f64) -> Curve3<f64> {
    Curve3::Circle {
        center: p3(0.0, 0.0, z),
        axis: z_axis(),
        radius: r,
        u_ref: x_axis(),
    }
}

/// The unit sphere's rim at height `z`.
fn unit_rim(z: f64) -> Curve3<f64> {
    rim(z, (1.0 - z * z).sqrt())
}

/// The first face, in arena order, on a surface of `kind` — the face
/// `tessellate`'s fold answers for when several refuse.
fn first_face_on(body: &Body<f64>, kind: SurfaceKind) -> FaceKey {
    body.faces()
        .find(|(_, f)| SurfaceKind::of(body.get_surface(f.surface).unwrap()) == kind)
        .map(|(fk, _)| fk)
        .expect("the body carries a face of that kind")
}

/// `tessellate` refuses `body` with the meridian-free arm, naming the
/// first face on a `kind` surface and that kind.
fn assert_refuses_meridian_free(name: &str, body: &Body<f64>, kind: SurfaceKind, delta: f64) {
    assert_eq!(
        mesh::tessellate(body, delta, Tol::witness()).map(|_| ()),
        Err(TessellateError::MeridianFreeCurvedFace {
            face: first_face_on(body, kind),
            surface: kind,
        }),
        "{name}"
    );
}

/// **The sphere cap closed by its disc, at both poles and four
/// latitudes.** The cap ABOVE `z` holds the north pole in its interior,
/// the ball BELOW `z` the south pole; the equator, a shallow cap and a
/// cap past the equator all refuse alike, because nothing about the
/// refusal reads the latitude.
///
/// Each body passes tier 3 and measures its closed-form volume as the
/// kernel stands, and both are asserted: that is why the mesh lane has
/// to refuse the face itself — no door in front of it does yet.
#[test]
fn a_rim_only_sphere_cap_refuses_at_either_pole_and_any_latitude() {
    let tol = Tol::witness();
    for z in [0.0_f64, 0.5, 0.9, -0.9] {
        let above = one_circle_cut(&unit_rim(z), sphere(1.0), Some(plane(z, false)));
        assert_eq!(topo::validate_geometric(&above, tol), Ok(()), "z = {z}");
        let exact = PI * (1.0 - z).powi(2) * (2.0 + z) / 3.0;
        let volume = topo::mass_properties(&above, tol).unwrap().volume;
        assert!(
            (volume - exact).abs() <= 1e-12 * exact,
            "cap above z = {z}: volume {volume} vs {exact}"
        );
        assert_refuses_meridian_free(
            &format!("cap above z = {z}"),
            &above,
            SurfaceKind::Sphere,
            0.01,
        );
    }
    for z in [0.0_f64, 0.5, -0.9] {
        let below = one_circle_cut(&unit_rim(z), plane(z, true), Some(sphere(1.0)));
        assert_eq!(topo::validate_geometric(&below, tol), Ok(()), "z = {z}");
        let exact = PI * (1.0 + z).powi(2) * (2.0 - z) / 3.0;
        let volume = topo::mass_properties(&below, tol).unwrap().volume;
        assert!(
            (volume - exact).abs() <= 1e-10 * exact,
            "ball below z = {z}: volume {volume} vs {exact}"
        );
        assert_refuses_meridian_free(
            &format!("ball below z = {z}"),
            &below,
            SurfaceKind::Sphere,
            0.01,
        );
    }
}

/// **A sphere stated as two rim-only caps on one rim** — both faces are
/// members, so the fold answers with the first in arena order. The last
/// body has the proportions of `topo`'s `two_level_rim_cap` at
/// `Δv = 0` (R = 10 mm, the rim at latitude 0.5 rad); that fixture's own
/// row is in `topo/tests/mesh12_rim_row_reach.rs`.
#[test]
fn a_sphere_of_two_rim_only_caps_refuses_on_its_first_face() {
    for z in [0.0_f64, 0.5] {
        let body = one_circle_cut(&unit_rim(z), sphere(1.0), None);
        assert_refuses_meridian_free(
            &format!("two caps on the rim at z = {z}"),
            &body,
            SurfaceKind::Sphere,
            0.01,
        );
    }
    let (r, v) = (0.010_f64, 0.5_f64);
    let body = one_circle_cut(&rim(r * v.sin(), r * v.cos()), sphere(r), None);
    assert_refuses_meridian_free("R = 10 mm, v = 0.5", &body, SurfaceKind::Sphere, 1e-4);
}

/// **The class is not the sphere's**: a cone's apex cap and a
/// cylinder's one-rim face walk to the same zero-height domain and
/// refuse on the same fact, each naming its own kind. The cone is built
/// both ways round so the refusing face is the seed once and the made
/// face once.
#[test]
fn a_cone_apex_cap_and_a_one_rim_cylinder_face_refuse_by_kind() {
    let cone = Surface::Cone {
        apex: p3(0.0, 0.0, 0.0),
        axis: z_axis(),
        half_angle: FRAC_PI_4,
        u_ref: x_axis(),
    };
    for (name, seed, made) in [
        ("cone seed, disc made", cone.clone(), plane(1.0, true)),
        ("disc seed, cone made", plane(1.0, true), cone.clone()),
    ] {
        let body = one_circle_cut(&rim(1.0, 1.0), seed, Some(made));
        assert_refuses_meridian_free(name, &body, SurfaceKind::Cone, 0.05);
    }
    let cylinder = Surface::Cylinder {
        origin: p3(0.0, 0.0, 0.0),
        axis: z_axis(),
        radius: 1.0,
        u_ref: x_axis(),
    };
    let body = one_circle_cut(&rim(1.0, 1.0), cylinder, Some(plane(1.0, true)));
    assert_refuses_meridian_free("one-rim cylinder face", &body, SurfaceKind::Cylinder, 0.05);
}

/// The hemisphere as revolve states it: a quarter disc about +Y.
fn dome() -> Body<f64> {
    let lp = bulge_loop(vec![
        (p2(0.0, 0.0), 0.0),
        (p2(1.0, 0.0), (PI / 8.0).tan()),
        (p2(0.0, 1.0), 0.0),
    ]);
    revolve(
        &validated(vec![lp]),
        axis_y(),
        Revolution::Full,
        Tol::witness(),
    )
    .unwrap()
    .body
}

/// The block `|x|, |z| ≤ 2`, `−2 ≤ y ≤ y0`.
fn slab_below_y(y0: f64) -> Body<f64> {
    let lp = ProfileLoop::polygon([p2(-2.0, -2.0), p2(2.0, -2.0), p2(2.0, y0), p2(-2.0, y0)]);
    let block = extrude(
        &validated(vec![lp]),
        Extrusion::Distance(4.0),
        Tol::witness(),
    )
    .unwrap()
    .body;
    topo::transform_rigid(
        &block,
        &geom_core::Affine3::translation(Vec3::new(0.0, 0.0, -2.0)),
        Tol::witness(),
    )
    .unwrap()
}

/// **The positive control: the SEAMED statements of the same two solids
/// mesh watertight.** The revolved dome is the cap above the equator,
/// and `ball ∩ slab(y ≤ ½)` is the ball below `½`; each measures the
/// volume its rim-only twin measures, so the pair is one solid stated
/// two ways and the refusal separates the statements, not the solids.
#[test]
fn the_seamed_twins_of_the_refused_caps_mesh_watertight() {
    let tol = Tol::witness();
    let cut = topo::boolean_op_with(
        topo::BooleanOp::Intersect,
        &ball(),
        &slab_below_y(0.5),
        &topo::BooleanDeclarations::default(),
        topo::SweepStrategy::Realized,
        tol,
    )
    .unwrap();
    let cut = &cut.body().expect("the ball meets the slab").body;
    for (name, seamed, rim_only, exact) in [
        (
            "dome",
            &dome(),
            one_circle_cut(&unit_rim(0.0), sphere(1.0), Some(plane(0.0, false))),
            (2.0 * PI / 3.0, 3.0 * PI),
        ),
        (
            "ball below 1/2",
            cut,
            one_circle_cut(&unit_rim(0.5), plane(0.5, true), Some(sphere(1.0))),
            (1.125 * PI, 3.75 * PI),
        ),
    ] {
        let (a, b) = (
            topo::mass_properties(seamed, tol).unwrap().volume,
            topo::mass_properties(&rim_only, tol).unwrap().volume,
        );
        assert!(
            (a - b).abs() <= 1e-12 * exact.0 && (a - exact.0).abs() <= 1e-12 * exact.0,
            "{name}: one solid, two statements — seamed {a}, rim-only {b}, exact {}",
            exact.0
        );
        let mesh = check_mesh_acceptance(seamed, 0.05, Some(exact));
        assert!(
            mesh.patches.iter().all(|p| !p.triangles.is_empty()),
            "{name}: every face of the seamed statement is emitted"
        );
    }
}

/// **The torus member is refused one door earlier, on the same fact.**
/// A torus face whose loop is rims only never reaches the walk: props'
/// torus parse — the prologue of the shape door this lane cites —
/// refuses a face with no meridian by that name, so the answer is
/// [`TessellateError::UnsupportedCurvedShape`] and the walk's guard is
/// behind it. Measured on the outer equator and on the top rim.
///
/// **A disposition pin, not a row for the walk's guard**: removing
/// `walk::require_a_meridian` cannot redden it, because the walk is
/// never called. What reddens it is the shape door ceasing to refuse
/// this face — the day the walk's guard would start answering for a
/// torus.
#[test]
fn a_rim_only_torus_face_refuses_at_the_shape_door_on_the_same_fact() {
    let torus = Surface::Torus {
        center: p3(0.0, 0.0, 0.0),
        axis: z_axis(),
        major_radius: 2.0,
        minor_radius: 0.5,
        u_ref: x_axis(),
    };
    for (name, circle) in [("outer equator", rim(0.0, 2.5)), ("top", rim(0.5, 2.0))] {
        let body = one_circle_cut(&circle, torus.clone(), None);
        assert_eq!(
            mesh::tessellate(&body, 0.05, Tol::witness()).map(|_| ()),
            Err(TessellateError::UnsupportedCurvedShape {
                face: first_face_on(&body, SurfaceKind::Torus),
                source: geom_brep::props::PropsError::NotIsoRectangle {
                    what: "torus face without a meridian",
                },
            }),
            "{name}"
        );
    }
}
