//! **Test fixtures**, behind the `test-support` feature (on only through
//! dev-dependency edges): the array spellings of this crate's two input
//! values — a box from its two corners and a ray from its origin and
//! direction — shared by `bvh`'s suites and by the pick suites of
//! `editor-core` and `viewer`, whose ray IS this crate's [`Ray`].
//!
//! Neither door carries an oracle: each is the struct literal with its
//! fields read out of two arrays, and a row that built the wrong box or
//! ray reds on the answer it expected from the one it meant.
//!
//! `tests/ray_r2.rs` adds an integer-grid pair (`box_of` / `ray_of`
//! over `i64`) that delegates here: its exactness argument is that every
//! coordinate IS an integer, so the conversion is its own.

use geom_core::{Point3, Vec3};

use crate::{Aabb, Ray};

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
