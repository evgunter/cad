//! **Bored bodies and the plane cuts through them** — the fixtures the
//! split suites cut (`pis_cut_cavity`, `split_section_rings`) and what
//! those suites read off a half: its section faces.
//!
//! Body authoring through the public doors (the [`super::cavity`]
//! constructors and `topo::subtract`), plus one reader. The bored
//! cylinder is `sweep::test_support::bored_cylinder`, a library
//! fixture, and is not repeated here.
//!
//! **Deliberately not absorbed**, and the whole of it:
//!
//! - editor-core's `cutter` (`gather_placed_under_two_roots.rs`), which
//!   [`u_cut`] builds the same body as through `topo` directly: it is
//!   a recipe in another crate's test tree, which this one cannot
//!   reach;
//! - `pis_cut_cavity`'s `Cut`, which carries the closed-form side test
//!   its truth table reads; it is built from [`tilted`].

use geom_core::{Point2, Point3, Vec3};
use topo::splitting::{SplitPart, SplitPlane, split};
use topo::{Body, FaceKey};

use super::cavity::{brick, cut, prism, rod};

/// The brick `[−2, 2]² × [0, 2.5]` less a rod of radius `r` parallel to
/// `z` about `(cx, cy)`, running past both caps.
pub fn bored_brick(cx: f64, cy: f64, r: f64) -> Body<f64> {
    let block = brick(Point3::new(-2.0, -2.0, 0.0), Point3::new(2.0, 2.0, 2.5));
    cut("bore", &block, &rod(Point2::new(cx, cy), r, -0.5, 3.5))
}

/// The 4³ block less a U-cutter whose two prongs, `y ∈ [1, 1.5]` and
/// `[2.5, 3]`, run from `x = 2` out through the `x = 4` wall, `z ∈
/// [1, 3]`.
pub fn u_cut() -> Body<f64> {
    let block = brick(Point3::new(0.0, 0.0, 0.0), Point3::new(4.0, 4.0, 4.0));
    let outline = [
        (2.0, 1.0),
        (6.0, 1.0),
        (6.0, 3.0),
        (2.0, 3.0),
        (2.0, 2.5),
        (5.0, 2.5),
        (5.0, 1.5),
        (2.0, 1.5),
    ]
    .map(|(x, y)| Point2::new(x, y));
    cut("U-cutter", &block, &prism(&outline, 1.0, 3.0))
}

/// The plane through `(0, 0, z)` with normal `(sin t, 0, cos t)`,
/// negated when `flip`.
pub fn tilted(z: f64, t: f64, flip: bool) -> SplitPlane<f64> {
    let s = if flip { -1.0 } else { 1.0 };
    topo::test_support::split_plane(
        Point3::new(0.0, 0.0, z),
        Vec3::new(s * t.sin(), 0.0, s * t.cos()),
        geom_core::Tol::witness(),
    )
}

/// The planar faces of `half` lying in `plane`: its section faces.
pub fn section_faces(half: &Body<f64>, plane: &SplitPlane<f64>) -> Vec<FaceKey> {
    half.faces()
        .filter(|(_, f)| {
            matches!(
                half.get_surface(f.surface),
                Some(geom::Surface::Plane { origin, normal, .. })
                    if normal.cross(plane.normal.get()).norm() < 1e-12
                        && (*origin - plane.origin).dot(plane.normal.get()).abs() < 1e-12
            )
        })
        .map(|(k, _)| k)
        .collect()
}

/// `body` split by `plane` into `[below, above]`, each asserted to hold
/// material and to pass tiers 1 and 3.
pub fn halves_at_rest(what: &str, body: &Body<f64>, plane: &SplitPlane<f64>) -> [Body<f64>; 2] {
    let tol = geom_core::Tol::witness();
    let result = split(body, plane, tol).unwrap_or_else(|e| panic!("{what}: splits: {e:?}"));
    [("below", result.below), ("above", result.above)].map(|(side, part)| {
        let SplitPart::Body(half) = part else {
            panic!("{what} {side}: material on both sides");
        };
        assert_eq!(topo::validate(&half), Ok(()), "{what} {side}: tier 1");
        assert_eq!(
            topo::validate_geometric(&half, tol),
            Ok(()),
            "{what} {side}: tier 3"
        );
        half
    })
}
