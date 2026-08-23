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
//! # The real element types (M2 PR 3 — the M0 placeholders retired)
//!
//! - **Points**: `geom_core::Point3<T>` (real since M0).
//! - **Curves**: [`geom_brep::EdgeCurve<T>`] — D2's intensional
//!   [`geom_brep::EdgeGeometry`] description plus its certified
//!   `geom::Curve3` carrier cache and certification record
//!   (D4 ¶2). Constructible only through certification
//!   (`EdgeCurve::certify`), so an uncertified carrier can never enter
//!   the arena; the Euler operators run that gate at attachment and the
//!   tier-3 validator re-runs it at rest.
//! - **Surfaces**: `geom::Surface<T>` — D3's closed analytic
//!   enum. A face's surface arrives from its construction (operator
//!   surface parameters, [`crate::FaceSurface`]; Newell-certified
//!   planes via `geom_brep::newell_plane`) or, for `mvfs`'s seed face,
//!   starts as the `Surface::Nurbs` representable-unimplemented
//!   placeholder — the honest "no description yet" state, legal
//!   mid-construction and rejected by the tier-3 validator if it
//!   survives to rest (nothing can be certified against it at M2).
//!
//! The key types are **defined in `geom-brep`** (its descriptions
//! reference surfaces by arena key — see `geom_brep::keys` for the
//! layering rationale) and re-exported here unchanged; a key's lineage
//! scoping and stale/foreign semantics are documented on
//! [`Body`](crate::Body).

pub use geom_brep::{CurveKey, PointKey, SurfaceKey};
