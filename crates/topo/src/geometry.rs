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
//!   [`geom_brep::EdgeDescription`] description plus its certified
//!   `geom::Curve3` carrier cache and certification record
//!   (D4 ¶2). Constructible only through certification
//!   (`EdgeCurve::certify`), so an uncertified carrier can never enter
//!   the arena; the Euler operators run that gate at attachment and the
//!   tier-3 validator re-runs it at rest.
//! - **Surfaces**: [`SurfaceSlot<T>`] — a described `geom::Surface<T>`
//!   (D3's closed analytic enum, or a spline) or **pending**. A face's
//!   surface arrives from its construction (operator surface
//!   parameters, [`crate::construction::FaceSurface`]; a sweep's planes
//!   are its stations' frames) or, for `mvfs`'s seed face and the faces
//!   minted from it, is pending: the face exists and waits for the
//!   description a describing door gives it. A pending slot is legal
//!   mid-construction and refused by the tier-3 validator at rest.
//!   Every door that hands a surface to geometry refuses it with
//!   [`SurfaceMiss::Pending`], so geometry is never handed one.
//!
//! The key types are **defined in `geom-brep`** (its descriptions
//! reference surfaces by arena key — see `geom_brep::keys` for the
//! layering rationale) and re-exported here unchanged; a key's
//! [stale/foreign semantics](crate::body#key-validity-stale-vs-foreign)
//! and its [lineage scoping](crate::Body) are documented with the arena.

pub use geom_brep::{CurveKey, PointKey, SurfaceKey, SurfaceMiss};

use geom::Surface;
use geom_core::Real;

/// **A body's surface slot**: the surface a face is a region of, or
/// pending. A pending slot is the state of a face a construction has
/// minted before it says which surface the face lies on (`mvfs`'s seed
/// face, the faces split from it, a sweep's transient disc); a
/// describing door moves the face onto a described slot. Geometry is
/// only ever handed a described surface: the doors that resolve a slot
/// for it refuse a pending one with [`SurfaceMiss::Pending`].
#[derive(Clone, Debug)]
pub enum SurfaceSlot<T: Real> {
    /// The surface the slot's faces are regions of.
    Described(Surface<T>),
    /// No surface yet.
    Pending,
}

impl<T: Real> SurfaceSlot<T> {
    /// The described surface, or [`SurfaceMiss::Pending`].
    ///
    /// # Errors
    ///
    /// [`SurfaceMiss::Pending`] for a pending slot.
    pub fn described(&self) -> Result<&Surface<T>, SurfaceMiss> {
        match self {
            Self::Described(s) => Ok(s),
            Self::Pending => Err(SurfaceMiss::Pending),
        }
    }

    /// Whether the slot is pending.
    pub fn is_pending(&self) -> bool {
        matches!(self, Self::Pending)
    }
}

/// **A resolved face's one surface miss**: its slot is pending. A read
/// through a face the body resolved ([`crate::live::FaceView::surface`])
/// cannot meet a stale surface key — a dangling link panics (D2 row 4)
/// — so this is the whole of what it refuses.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Pending;

impl From<Pending> for SurfaceMiss {
    fn from(Pending: Pending) -> Self {
        SurfaceMiss::Pending
    }
}

impl<T: Real> From<Surface<T>> for SurfaceSlot<T> {
    fn from(surface: Surface<T>) -> Self {
        Self::Described(surface)
    }
}
