//! The array spellings of `bvh`'s two input values, shared by this
//! crate's suites: a box from its two corners and a ray from its origin
//! and direction.
//!
//! No oracle rides on either. Each is the struct literal with its fields
//! read out of two arrays, and a row that built the wrong box or ray
//! reds on the answer it expected from the one it meant.
//!
//! `ray_r2` keeps its own integer-grid pair (`box_of`/`ray_of` over
//! `i64`): its exactness argument is that every coordinate IS an
//! integer, which these `f64` spellings would leave to the caller.

// One instance per binary; not every suite reads both.
#![allow(dead_code)]
#![allow(unreachable_pub)] // why: root Cargo.toml, the `unreachable_pub` stanza

use bvh::{Aabb, Ray};
use geom_core::{Point3, Vec3};

/// The box with lower corner `min` and upper corner `max`.
pub fn boxed(min: [f64; 3], max: [f64; 3]) -> Aabb {
    Aabb {
        min_x: min[0],
        min_y: min[1],
        min_z: min[2],
        max_x: max[0],
        max_y: max[1],
        max_z: max[2],
    }
}

/// The ray from `origin` along `dir`.
pub fn ray(origin: [f64; 3], dir: [f64; 3]) -> Ray {
    Ray {
        origin: Point3::new(origin[0], origin[1], origin[2]),
        dir: Vec3::new(dir[0], dir[1], dir[2]),
    }
}
