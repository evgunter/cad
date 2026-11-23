//! **A band's or patch's frame refuses typed where its input
//! degenerates**, at each public arm helper that mints one: the input
//! fed is the degenerate configuration the helper's doc names, and the
//! refusal is the frame mint's (`OrthonormalizeError`, under the axis
//! that degenerated). A neighbour clear of it mints, so each row's
//! refusal is the degeneracy's and not the helper's.
//!
//! The ruled arm's seam (the ball's foot on its first support) and the
//! private mints (a junction's band chart, a closed rim's seamed band, a
//! mixed corner's torus, a mitre's cylinders, a cut-off's cap circle)
//! route through the same two doors and the same refusal map, pinned in
//! `blend::arms` and `blend` beside them.

#![allow(clippy::expect_used, clippy::panic)]

use geom::Surface;
use geom_core::{Band, OrthoAxis, OrthonormalizeError, Point3, Tol, UnitVec3, UnitVec3Error, Vec3};
use sweep::blend::Convexity;
use sweep::blend::arms::{
    Meridian, chamfer_corner_patch, chamfer_strip, corner_ball, plane_plane_blend,
    plane_sphere_blend,
};

fn band() -> Band {
    Band::linear(Tol::witness()).expect("the band")
}

fn o() -> Point3<f64> {
    Point3::new(0.0, 0.0, 0.0)
}

/// The refusal is the definite zero under `axis`.
fn degenerate(e: Option<OrthonormalizeError>, axis: OrthoAxis, what: &str) {
    let e = e.unwrap_or_else(|| panic!("{what}: minted a frame from a degenerate input"));
    assert_eq!(e.axis, axis, "{what}: the axis that degenerated");
    assert!(
        matches!(e.error, UnitVec3Error::Degenerate),
        "{what}: a definite zero, got {:?}",
        e.error
    );
}

/// **The plane–plane cylinder's seam is a support's normal off the
/// edge**: a normal along the edge has none.
#[test]
fn a_plane_plane_cylinder_refuses_a_normal_along_its_edge() {
    let tau = UnitVec3::exact_z();
    let (n_a, n_b) = (Vec3::new(0.0, 0.0, 1.0), Vec3::new(1.0, 0.0, 0.0));
    let r = plane_plane_blend(o(), tau, n_a, n_b, 0.1, Convexity::Convex, band());
    degenerate(r.err(), OrthoAxis::V, "n_a along tau");
    let clear = plane_plane_blend(
        o(),
        tau,
        Vec3::new(0.0, 1.0, 0.0),
        n_b,
        0.1,
        Convexity::Convex,
        band(),
    );
    assert!(clear.is_ok(), "a square edge mints: {:?}", clear.err());
}

/// **The chamfer strip's normal is `n_a + n_b`**: supports back to back
/// have none.
#[test]
fn a_chamfer_strip_refuses_supports_back_to_back() {
    let tau = UnitVec3::exact_z();
    let n = Vec3::new(0.0, 1.0, 0.0);
    let r = chamfer_strip(o(), tau, n, -n, 0.1, band());
    degenerate(r.err(), OrthoAxis::U, "n_a = −n_b");
    let clear = chamfer_strip(o(), tau, n, Vec3::new(1.0, 0.0, 0.0), 0.1, band());
    assert!(clear.is_ok(), "a square edge mints: {:?}", clear.err());
}

/// **The plane–sphere torus is seamed along the plane's own `u_ref`**:
/// a `u_ref` along the plane's normal has no part square to it.
#[test]
fn a_plane_sphere_torus_refuses_a_seam_along_its_axis() {
    let n = Vec3::new(0.0, 0.0, 1.0);
    let sphere_c = Point3::new(0.0, 0.0, -0.5);
    let mint = |u_ref| {
        plane_sphere_blend(
            o(),
            n,
            u_ref,
            sphere_c,
            1.0,
            0.1,
            true,
            Convexity::Convex,
            band(),
        )
    };
    degenerate(mint(n).err(), OrthoAxis::V, "u_ref along n");
    let clear = mint(Vec3::new(1.0, 0.0, 0.0));
    assert!(clear.is_ok(), "the dome's rim mints: {:?}", clear.err());
}

/// **The corner ball's chart pole is the third foot turned by the
/// feet's turn**: three dependent normals turn by zero.
#[test]
fn a_corner_ball_refuses_dependent_normals() {
    let ball = |n3: Vec3<f64>| {
        corner_ball(
            [o(); 3],
            [Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0), n3],
            0.1,
            Convexity::Convex,
            band(),
        )
    };
    let flat = Vec3::new(1.0, 1.0, 0.0) * core::f64::consts::FRAC_1_SQRT_2;
    degenerate(ball(flat).err(), OrthoAxis::U, "three normals in one plane");
    let clear = ball(Vec3::new(0.0, 0.0, 1.0));
    assert!(clear.is_ok(), "a cube corner mints: {:?}", clear.err());
}

/// **The chamfer's corner patch is the plane through its three feet**:
/// three feet on one line span none.
#[test]
fn a_chamfer_corner_patch_refuses_collinear_feet() {
    let outward = Vec3::new(1.0, 1.0, 1.0);
    let feet = |z: f64| {
        [
            Point3::new(0.1, 0.0, 0.0),
            Point3::new(0.0, 0.1, 0.0),
            Point3::new(0.05, 0.05, z),
        ]
    };
    // The normal's fold is `ĝ·(ĝ·outward)`, `g` over its own square: at
    // collinear feet that is `0/0`, so the axis has no finite length.
    let e = chamfer_corner_patch(feet(0.0), outward, band()).expect_err("collinear feet");
    assert_eq!(
        e.axis,
        OrthoAxis::U,
        "collinear feet: the axis that degenerated"
    );
    assert!(
        matches!(e.error, UnitVec3Error::NonFiniteLength),
        "collinear feet: no finite length, got {:?}",
        e.error
    );
    let clear = chamfer_corner_patch(
        [
            Point3::new(0.1, 0.0, 0.0),
            Point3::new(0.0, 0.1, 0.0),
            Point3::new(0.0, 0.0, 0.1),
        ],
        outward,
        band(),
    );
    assert!(
        matches!(clear, Ok(Surface::Plane { .. })),
        "a cube corner's patch mints: {:?}",
        clear.err()
    );
}

/// **The meridian sheet's radial is the rim's offset off the axis**: a
/// rim on the axis has none.
#[test]
fn a_meridian_refuses_a_rim_on_its_axis() {
    let axis = geom::Line {
        origin: o(),
        dir: UnitVec3::exact_y(),
    };
    degenerate(
        Meridian::new(axis, Point3::new(0.0, 0.7, 0.0), band()).err(),
        OrthoAxis::V,
        "rim on the axis",
    );
    let sheet =
        Meridian::new(axis, Point3::new(0.8, 0.0, 0.0), band()).expect("a rim off the axis");
    let u = sheet.radial();
    assert_eq!(
        [u.x, u.y, u.z],
        [1.0, 0.0, 0.0],
        "the radial of a rim on the x axis, bit for bit"
    );
}
