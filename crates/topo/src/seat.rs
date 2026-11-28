//! **A body read from another operand's frame** (`[placement.frame]`).
//!
//! An operand in another frame is read into its reader's through their
//! relative placement, which moves where the operand stands and none of
//! its data. A [`Seat`] is that reading: the body untouched, and the
//! motion each datum taken off it for a reading between two operands is
//! mapped through.
//!
//! **What a seat promises its readers, and what a door does with it.**
//! To a reader that relates two operands (a measure, a clearance, a
//! carrier verdict) a seat moves nothing: each datum it takes is mapped
//! at the reading, and no question of the moved body is asked there.
//! The one door that builds data out of a seated body is the placed
//! graft ([`crate::boolean::graft_placed`]): an aggregate's image is the
//! aggregate's data, so that door moves the body whole and asks the
//! session box of the image, as every move does.

use geom_core::{RigidMotion, RigidMotionError};

use crate::body::Body;

/// **A body read from another operand's frame** (`[placement.frame]`):
/// the body in its own coordinates, untouched, and the rigid motion from
/// them into the frame its reader computes in, `None` where the two are
/// one frame. A reading within the body is made in its own coordinates;
/// a reading between it and another operand maps the datum it takes
/// (a point, a direction, a carrier) through the motion at the reading,
/// and moves none of the body's data.
#[derive(Debug, Clone, Copy)]
pub struct Seat<'b, T: geom_core::Real> {
    body: &'b Body<T>,
    /// The motion into the reader's frame.
    at: Option<RigidMotion<T>>,
}

impl<'b, T: geom_core::Real> Seat<'b, T> {
    /// `body` read in its own frame.
    #[must_use]
    pub fn own(body: &'b Body<T>) -> Self {
        Self::from_motion(body, None)
    }

    /// `body` read through `motion` into its reader's frame.
    #[must_use]
    pub fn at(body: &'b Body<T>, motion: RigidMotion<T>) -> Self {
        Self::from_motion(body, Some(motion))
    }

    /// `body` read through `motion` into its reader's frame, `None` where
    /// the two are one frame.
    #[must_use]
    pub fn from_motion(body: &'b Body<T>, motion: Option<RigidMotion<T>>) -> Self {
        Self { body, at: motion }
    }

    /// The body, in its own coordinates.
    #[must_use]
    pub fn body(&self) -> &'b Body<T> {
        self.body
    }

    /// The motion into the reader's frame, `None` for the identity.
    #[must_use]
    pub fn motion(&self) -> Option<&RigidMotion<T>> {
        self.at.as_ref()
    }

    /// A point of the body, in the reader's frame.
    #[must_use]
    pub fn point(&self, p: geom_core::Point3<T>) -> geom_core::Point3<T> {
        self.at.map_or(p, |m| m.transform_point(p))
    }

    /// A direction of the body, in the reader's frame.
    #[must_use]
    pub fn vec(&self, v: geom_core::Vec3<T>) -> geom_core::Vec3<T> {
        self.at.map_or(v, |m| m.transform_vec(v))
    }

    /// A surface of the body, in the reader's frame
    /// ([`crate::transform::surface_at`]); `None` where the map refuses.
    #[must_use]
    pub fn surface(&self, s: &geom::Surface<T>) -> Option<geom::Surface<T>> {
        match &self.at {
            None => Some(s.clone()),
            Some(m) => crate::transform::surface_at(m, s).ok(),
        }
    }

    /// **The map from `other`'s body coordinates into this one's**,
    /// through the reader's frame the two seats share
    /// ([`RigidMotion::relative`]).
    ///
    /// # Errors
    ///
    /// The inverse's or the composition's overflow.
    pub(crate) fn relative(
        &self,
        other: &Seat<'_, T>,
    ) -> Result<Option<RigidMotion<T>>, RigidMotionError> {
        RigidMotion::relative(self.at.as_ref(), other.at.as_ref())
    }

    /// **A point of the reader's frame, in the body's own coordinates**:
    /// where a reading within the body (a vertex's link) is asked about
    /// another operand's point. The inverse is taken here, so a reader
    /// that asks no such point cannot refuse on it.
    ///
    /// # Errors
    ///
    /// The inverse's overflow.
    pub fn into_own(
        &self,
        p: geom_core::Point3<T>,
    ) -> Result<geom_core::Point3<T>, RigidMotionError> {
        Ok(match &self.at {
            None => p,
            Some(m) => m.inverse()?.transform_point(p),
        })
    }
}
