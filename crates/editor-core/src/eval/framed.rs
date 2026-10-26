//! **A body value and the frame it computes in** (D10
//! `[placement.frame]`).
//!
//! A body value is [`Framed`]: the body in its own coordinates, shared
//! by `Arc`, and one map from them into its space's coordinates. A copy
//! keeps its source's `Arc` under the composed map and is never
//! rebuilt. An operation over operands in one space computes in its
//! first operand's frame and reads every other operand through
//! [`Framed::relative`], the one door between two operands of one
//! space; its result is in that first operand's frame. A construction
//! computes in its own coordinates ([`Frame::Own`]).
//!
//! Nothing here composes a body into its space's coordinates for a
//! reader outside the model: that is `crate::mate::in_space`'s, and only
//! export and the viewer's picture call it.

use std::sync::Arc;

use geom_core::{Decide, Real, RigidMotion, RigidMotionError, Tol};
use topo::Body;

/// **The map from a body's coordinates into its space's.**
#[derive(Debug, Clone)]
pub(crate) enum Frame<T: Real> {
    /// The identity: the body is in its space's coordinates. A
    /// construction's frame, and a copy's under the identity bundle as
    /// written.
    Own,
    /// A rigid map, shared: every value computed in one frame holds the
    /// same `Arc`, so two such values relate by the identity without
    /// arithmetic.
    Map(Arc<RigidMotion<T>>),
}

impl<T: Real> Frame<T> {
    /// **Whether the two are one frame**: both `Own`, or one shared map.
    fn same(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Own, Self::Own) => true,
            (Self::Map(a), Self::Map(b)) => Arc::ptr_eq(a, b),
            _ => false,
        }
    }

    /// The map, `None` for [`Frame::Own`].
    pub(crate) fn map(&self) -> Option<&RigidMotion<T>> {
        match self {
            Self::Own => None,
            Self::Map(m) => Some(m),
        }
    }

    /// **This frame moved by `motion`**: `motion ∘ self`.
    ///
    /// # Errors
    ///
    /// The composition's overflow.
    pub(crate) fn moved(&self, motion: &RigidMotion<T>) -> Result<Self, RigidMotionError> {
        Ok(Self::Map(Arc::new(match self {
            Self::Own => *motion,
            Self::Map(inner) => motion.compose(**inner)?,
        })))
    }

    /// **`self⁻¹ ∘ other`**: the map from `other`'s coordinates into
    /// `self`'s, `None` where the two are one frame.
    ///
    /// # Errors
    ///
    /// The inverse's or the composition's overflow.
    pub(crate) fn relative(
        &self,
        other: &Self,
    ) -> Result<Option<RigidMotion<T>>, RigidMotionError> {
        if self.same(other) {
            return Ok(None);
        }
        Ok(Some(match (self.map(), other.map()) {
            (None, Some(b)) => *b,
            (Some(a), None) => a.inverse()?,
            (Some(a), Some(b)) => a.inverse()?.compose(*b)?,
            (None, None) => unreachable!("two `Own` frames are one frame"),
        }))
    }
}

/// **A body value** (the copies ruling): the body in its own
/// coordinates and the map from them into its space's.
#[derive(Debug, Clone)]
pub struct Framed<T: Decide> {
    body: Arc<Body<T>>,
    frame: Frame<T>,
}

impl<T: Decide> Framed<T> {
    /// A body in its space's coordinates: a construction's, computed in
    /// its own.
    #[must_use]
    pub fn own(body: Arc<Body<T>>) -> Self {
        Self {
            body,
            frame: Frame::Own,
        }
    }

    /// `body` in `frame`: an operation's result, computed in the frame
    /// of the operand it computed in.
    pub(crate) fn in_frame(body: Arc<Body<T>>, frame: Frame<T>) -> Self {
        Self { body, frame }
    }

    /// **The body, in its own coordinates.** A reader that relates it to
    /// another body of its space reads that one through
    /// [`Self::relative`].
    #[must_use]
    pub fn body(&self) -> &Body<T> {
        &self.body
    }

    /// The body, shared.
    #[must_use]
    pub fn shared(&self) -> &Arc<Body<T>> {
        &self.body
    }

    /// **Another body in this one's frame** (`test-support`): for rows
    /// that write a value by hand where an evaluation put one.
    #[cfg(any(test, feature = "test-support"))]
    #[must_use]
    pub fn with_body(&self, body: Arc<Body<T>>) -> Self {
        Self {
            body,
            frame: self.frame.clone(),
        }
    }

    /// The frame it is in.
    pub(crate) fn frame(&self) -> &Frame<T> {
        &self.frame
    }

    /// **Its space's origin, in the body's coordinates**: where a reach
    /// in the space is measured from.
    ///
    /// # Errors
    ///
    /// The inverse's overflow.
    pub(crate) fn space_origin(&self) -> Result<geom_core::Point3<T>, RigidMotionError> {
        Ok(match self.frame.map() {
            None => geom_core::Point3::origin(),
            Some(map) => map.inverse()?.origin(),
        })
    }

    /// **A point of the body, in its space's coordinates**: what a
    /// reader that states a position in the space compares against.
    pub(crate) fn point_in_space(&self, p: geom_core::Point3<T>) -> geom_core::Point3<T> {
        self.frame.map().map_or(p, |map| map.transform_point(p))
    }

    /// **A direction of the body, in its space's coordinates.**
    pub(crate) fn vec_in_space(&self, v: geom_core::Vec3<T>) -> geom_core::Vec3<T> {
        self.frame.map().map_or(v, |map| map.transform_vec(v))
    }

    /// **The same body, its frame moved by `motion`**: a placement's
    /// copy, sharing its source's `Arc`.
    ///
    /// # Errors
    ///
    /// The composition's overflow.
    pub(crate) fn moved(&self, motion: &RigidMotion<T>) -> Result<Self, RigidMotionError> {
        Ok(Self {
            body: Arc::clone(&self.body),
            frame: self.frame.moved(motion)?,
        })
    }

    /// **The one door between two operands of one space**: the map from
    /// `other`'s body coordinates into this one's
    /// (`self.frame⁻¹ ∘ other.frame`), `None` where the two are in one
    /// frame and the map is the identity.
    ///
    /// # Errors
    ///
    /// The inverse's or the composition's overflow.
    pub fn relative(&self, other: &Self) -> Result<Option<RigidMotion<T>>, RigidMotionError> {
        self.frame.relative(&other.frame)
    }
}

impl<T: Decide + topo::AtRestPolicy> Framed<T> {
    /// **`other`'s body in this one's coordinates**, mapped once
    /// ([`Self::relative`]), or `other`'s own `Arc` where the two are in
    /// one frame.
    ///
    /// # Errors
    ///
    /// The relative map's overflow, or the kernel's own
    /// [`topo::transform::TransformError`].
    pub(crate) fn read_in(
        &self,
        other: &Self,
        tol: Tol,
    ) -> Result<Arc<Body<T>>, super::NodeErrorKind> {
        match self.relative(other).map_err(super::NodeErrorKind::Motion)? {
            None => Ok(Arc::clone(&other.body)),
            Some(map) => {
                super::counters::rigid_map();
                topo::transform::transform_rigid(&other.body, &map, tol)
                    .map(Arc::new)
                    .map_err(super::NodeErrorKind::Transform)
            }
        }
    }
}
