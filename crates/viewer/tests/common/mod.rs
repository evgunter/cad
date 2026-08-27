//! Fixtures shared by this crate's suites, **derived from the scene
//! they test** rather than restated beside it.
//!
//! Why this file exists: the plate's dimensions were hand-copied into
//! three suites, so changing `scene::plate_with_hole` would have left
//! two of them testing a box the scene no longer has — green, and
//! measuring nothing. `viewer::scene` now exports the plate's identity
//! (`PLATE_EXTENT`, `PLATE_HOLE_RADIUS`) and everything here is a
//! function of those constants, so the fixtures cannot drift from the
//! subject.
//!
//! The two review suites (`review_gui0_r1`, `review_gui0_r2`) keep
//! their own fixtures on purpose: a promoted review suite's value is
//! that it is an INDEPENDENT derivation of what the unit claims
//! (`memories/review-and-dependency-policy.md`), and pointing it at
//! the implementation's own constants would spend exactly that.

#![allow(dead_code)] // loaded once per consumer; each uses a subset
#![allow(unreachable_pub)]
// why: root Cargo.toml, the `unreachable_pub` stanza
// Panicking is a test's failure mechanism (workspace lint note).
#![allow(clippy::expect_used)]

use bvh::Aabb;
use pncad::geom_core::Point3;
use viewer::camera::Camera;
use viewer::scene::{PLATE_EXTENT, PLATE_HOLE_RADIUS};

/// The spike plate's bounding box, from the scene's own dimensions.
pub fn plate_bounds() -> Aabb {
    let [width, depth, thickness] = PLATE_EXTENT;
    Aabb {
        min_x: 0.0,
        min_y: 0.0,
        min_z: 0.0,
        max_x: width,
        max_y: depth,
        max_z: thickness,
    }
}

/// The plate's nominal solid volume: the block, less the through hole.
pub fn plate_volume() -> f64 {
    let [width, depth, thickness] = PLATE_EXTENT;
    width * depth * thickness
        - std::f64::consts::PI * PLATE_HOLE_RADIUS * PLATE_HOLE_RADIUS * thickness
}

/// The default framing on the plate at `aspect`.
pub fn framed(aspect: f64) -> Camera {
    Camera::framing(&plate_bounds(), aspect).expect("the plate frames")
}

/// The eight corners of a box.
pub fn corners(b: &Aabb) -> Vec<Point3<f64>> {
    let mut out = Vec::new();
    for x in [b.min_x, b.max_x] {
        for y in [b.min_y, b.max_y] {
            for z in [b.min_z, b.max_z] {
                out.push(Point3::new(x, y, z));
            }
        }
    }
    out
}
