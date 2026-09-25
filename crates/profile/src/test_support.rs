//! Fixture helpers, compiled under `test` / `test-support` only.

use geom_core::{Point2, Real};

use crate::ProfileLoop;

/// The loop a chain of (position, bulge) pairs lowers to — each vertex
/// with the bulge of the segment leaving it, the last one's closing
/// back to the first — with no declared-tangent joints.
///
/// It forwards to the lowering the lattice's emission layer uses for
/// `arc_to(Bulge)` and computes nothing of its own, so a loop written
/// here stores the same vertices, segments and bulges, bit for bit, as
/// the same chain emitted by the lattice. A bulge of exactly zero
/// (either sign) is a line; any other bulge is an arc, finite or not,
/// and [`crate::Profile::validate`] decides what the table is.
pub fn bulge_loop<T: Real>(chain: Vec<(Point2<T>, T)>) -> ProfileLoop<T> {
    ProfileLoop::lower(&chain, Vec::new())
}
