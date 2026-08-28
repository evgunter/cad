//! Certified-conservative [`Aabb`] constructors for surface carriers
//! (C10, M5 PR 8) — the surface-side sibling of [`crate::curves::boxes`]
//! (its module docs carry the placement rationale: the `bvh` crate
//! stays below the geometry crates; constructors sit next to the
//! invariants they cite — and the containment contract, including why
//! a box's docs must not price its own looseness). Certified-box driver code — a **sole**-bound
//! [`Bounds`] seam under the 2026-07-29 amendment (geom-core `real.rs`,
//! Bounds scope rule), not an allowlisted one: the sibling module's
//! docs carry why the distinction is not pedantry.

use bvh::Aabb;
use geom_core::Bounds;

use crate::surfaces::nurbs::NurbsSurface;

/// The certified-conservative box of a NURBS surface: the AABB of its
/// control-net brackets. Sound by the convex-hull property — every
/// surface point is a convex combination of control points because the
/// tensor-product basis is nonnegative and the weights are **strictly
/// positive by construction** (the PR 3 positive-weights invariant,
/// enforced by [`NurbsSurface::new`]; negative weights would void
/// convexity, Book p. 293). Valid over the whole (u, v) domain, a
/// fortiori over any cell (cell-tight hulls via knot refinement are
/// PR 7's sharpening; a wider box still contains the locus). No
/// arithmetic — brackets only. All-poison control points (a
/// placeholder) yield the poison box, which overlaps everything.
pub fn nurbs_surface_aabb<T: Bounds>(surface: &NurbsSurface<T>) -> Aabb {
    Aabb::from_points(surface.control().iter().copied()).unwrap_or_else(Aabb::poison)
}
