//! Geometry-arena element types — the only `T`-carrying storage in a
//! [`Body`](crate::Body).
//!
//! The genericity boundary (Q1 in `docs/DESIGN.md`) runs exactly here:
//! topology entities ([`crate::entity`]) are scalar-free, and everything
//! scalar-valued lives in three arenas keyed by [`PointKey`],
//! [`CurveKey`], and [`SurfaceKey`]. An interval replay materializes a
//! `Body<Interval>` with **identical topology keys** and interval-valued
//! geometry — topology never branches on the scalar type.
//!
//! # M0 placeholders
//!
//! Vertex geometry is real already: [`geom_core::Point3`], which exists
//! since the linalg PR. Curve and surface geometry are **deliberately-stub
//! one-variant enums** ([`CurveGeom`] / [`SurfaceGeom`]): the real variant
//! taxonomies (line/circle/…/NURBS; plane/cylinder/…/NURBS) are M2's
//! design work against D2/D3, but the arena plumbing, key discipline, and
//! validator shape have to be real *now*. M2 replaces the placeholder
//! variants with the real ones; the enums stay **closed** per D3
//! (intersection dispatch needs compile-time exhaustiveness — no open
//! trait objects).

use geom_core::{Point3, Real};
use slotmap::new_key_type;

new_key_type! {
    /// Typed key into a body's point arena (vertex geometry).
    pub struct PointKey;

    /// Typed key into a body's curve arena (edge geometry).
    pub struct CurveKey;

    /// Typed key into a body's surface arena (face geometry).
    pub struct SurfaceKey;
}

/// Edge geometry — M0 placeholder enum.
///
/// M2 replaces the variant set with the real curve taxonomy (D2's
/// intensional descriptions over D3's closed analytic-plus-NURBS kinds);
/// the enum stays closed per D3. The single M0 variant exists so the
/// curve arena, its key discipline, and the validator are exercised for
/// real before any curve evaluation exists.
#[derive(Clone, Debug)]
pub enum CurveGeom<T: Real> {
    /// The M0 placeholder variant. Carries a point so the arena element
    /// genuinely holds scalar values of type `T` (an interval replay
    /// stores interval coordinates here); the point has **no geometric
    /// meaning** — it is arena ballast, not a curve description.
    Placeholder {
        /// Scalar ballast; no geometric meaning at M0.
        anchor: Point3<T>,
    },
}

/// Face geometry — M0 placeholder enum.
///
/// M2 replaces the variant set with the real surface taxonomy
/// (plane/cylinder/cone/sphere/torus/NURBS, D3); the enum stays closed
/// per D3. See [`CurveGeom`] for why the placeholder exists at all.
#[derive(Clone, Debug)]
pub enum SurfaceGeom<T: Real> {
    /// The M0 placeholder variant. Same role as
    /// [`CurveGeom::Placeholder`]: keeps the surface arena genuinely
    /// `T`-valued with no geometric meaning attached.
    Placeholder {
        /// Scalar ballast; no geometric meaning at M0.
        anchor: Point3<T>,
    },
}
