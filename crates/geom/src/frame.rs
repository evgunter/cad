//! **A carrier's placement**, [`Frame`]: a point and a right-handed
//! orthonormal basis `(u_ref, v_ref, axis)`, the third axis
//! `v_ref = axis × u_ref` derived, never stored.
//!
//! `axis` and `u_ref` are unit and orthogonal by convention, unchecked
//! here and certified at rest. What each one means on a carrier (a
//! conic's plane normal and seam, a surface's axis of revolution) is
//! that kind's own docs'. The readings of a placement alone below are
//! its one home: a reader holding a `Frame` asks it for them rather than
//! spelling them again from the fields.
//!
//! A [`RigidMotion`] is a different fact about the same shape of data:
//! a motion is proven rigid where it is minted, a frame is stated data
//! certified at rest. [`Frame::of_motion`] is the one bridge from the
//! first to the second, and [`Frame::moved`] is a frame's image under
//! one. An improper map has no reading here: its image is a per-kind
//! rule (a conic's axis, a plane's outward normal), stated where the
//! map is applied.

use geom_core::{Point3, Real, RigidMotion, Vec3};

use crate::{Line, SquareFrame};

/// **A placement**: a point and a right-handed orthonormal basis
/// `(u_ref, v_ref, axis)` ([module docs](self)).
#[derive(Clone, Copy, Debug)]
pub struct Frame<T: Real> {
    /// The point the basis is placed at.
    pub origin: Point3<T>,
    /// The third axis, unit (conventional, unchecked).
    pub axis: Vec3<T>,
    /// The first axis, unit and square to `axis` (conventional,
    /// unchecked).
    pub u_ref: Vec3<T>,
}

impl<T: Real> Frame<T> {
    /// The frame at `origin` with third axis `axis` and first axis
    /// `u_ref`.
    #[must_use]
    pub fn new(origin: Point3<T>, axis: Vec3<T>, u_ref: Vec3<T>) -> Self {
        Self {
            origin,
            axis,
            u_ref,
        }
    }

    /// **The frame a motion places**: its origin, `w` as the axis and
    /// `u` as the first axis, so the derived `v_ref` is the motion's `v`
    /// to rounding.
    #[must_use]
    pub fn of_motion(motion: &RigidMotion<T>) -> Self {
        Self::new(motion.origin(), motion.w().get(), motion.u().get())
    }

    /// **The unit radial direction at azimuth `u`**, `u_ref·cos u +
    /// v_ref·sin u`: the azimuthal frame's own reading
    /// ([`crate::azimuth`]'s `Radial`), for a caller outside the crate.
    #[must_use]
    pub fn radial(&self, u: T) -> Vec3<T> {
        crate::azimuth::frame(self, u).radial.0
    }

    /// `axis × u_ref`, the second axis.
    #[must_use]
    pub fn v_ref(&self) -> Vec3<T> {
        self.axis.cross(self.u_ref)
    }

    /// The components of `p − origin` along `u_ref` and `v_ref`.
    #[must_use]
    pub fn in_plane(&self, p: Point3<T>) -> (T, T) {
        let w = p - self.origin;
        (w.dot(self.u_ref), w.dot(self.v_ref()))
    }

    /// **`p`'s direction about the axis**: [`Self::in_plane`] read as an
    /// angle from `u_ref` toward `v_ref` ([`crate::Azimuth`]), the one
    /// in-frame angle read of a point. A point on the axis has no
    /// direction ([`crate::Azimuth::near`] says what it reads).
    #[must_use]
    pub fn azimuth(&self, p: Point3<T>) -> crate::Azimuth<T> {
        let (x, y) = self.in_plane(p);
        crate::Azimuth::of(y, x)
    }

    /// `p`'s height above the frame's plane: `(p − origin)·axis`, the
    /// [`plane_offset`] of the plane through `origin` normal to `axis`.
    #[must_use]
    pub fn height(&self, p: Point3<T>) -> T {
        plane_offset(self.origin, self.axis, p)
    }

    /// **`p`'s local coordinates** `(x, y, h)`: [`Self::in_plane`] and
    /// [`Self::height`].
    #[must_use]
    pub fn local(&self, p: Point3<T>) -> (T, T, T) {
        let (x, y) = self.in_plane(p);
        (x, y, self.height(p))
    }

    /// The point at local coordinates `(x, y, h)`:
    /// `origin + u_ref·x + v_ref·y + axis·h`, [`Self::local`]'s inverse.
    #[must_use]
    pub fn at_local(&self, (x, y, h): (T, T, T)) -> Point3<T> {
        self.origin + self.u_ref * x + self.v_ref() * y + self.axis * h
    }

    /// **`p`'s foot on the frame's plane**, the plane through `origin`
    /// square to `axis` ([`Line::square_foot`] on [`Self::axis_line`]).
    #[must_use]
    pub fn foot(&self, p: Point3<T>) -> Point3<T> {
        self.axis_line().square_foot(p)
    }

    /// The line through `origin` along `axis`.
    #[must_use]
    pub fn axis_line(&self) -> Line<T> {
        Line {
            origin: self.origin,
            dir: self.axis,
        }
    }

    /// The same placement run back: the axis flipped, `u_ref` kept, so
    /// the derived `v_ref` flips with it.
    #[must_use]
    pub fn reversed(&self) -> Self {
        Self {
            axis: -self.axis,
            ..*self
        }
    }

    /// The same placement moved by `by`.
    #[must_use]
    pub fn translated(&self, by: Vec3<T>) -> Self {
        Self {
            origin: self.origin + by,
            ..*self
        }
    }

    /// **The image under a rigid motion**: the origin moved, both
    /// stored axes turned by the motion's linear part
    /// ([`RigidMotion::transform_point`], [`RigidMotion::transform_vec`]).
    #[must_use]
    pub fn moved(&self, motion: &RigidMotion<T>) -> Self {
        Self::new(
            motion.transform_point(self.origin),
            motion.transform_vec(self.axis),
            motion.transform_vec(self.u_ref),
        )
    }

    /// The same placement with every scalar mapped by `f`.
    #[must_use]
    pub fn map_scalar<U: Real>(&self, f: impl Fn(T) -> U) -> Frame<U> {
        Frame {
            origin: self.origin.map(&f),
            axis: self.axis.map(&f),
            u_ref: self.u_ref.map(&f),
        }
    }

    /// **The basis squared** ([`SquareFrame`]): the unit axis along a
    /// stored `axis` of any length, and the unit seam square to it toward
    /// the stored `u_ref`, taken as `a × (u_ref × a)` over the unit `a`.
    #[must_use]
    pub fn square(&self) -> SquareFrame<T> {
        let axis = self.axis.normalize();
        let across = axis.cross(self.u_ref.cross(axis));
        let len = across.norm();
        SquareFrame {
            axis,
            u_ref: across / len,
            width: len / self.u_ref.norm(),
        }
    }
}

/// **A point's signed offset from a plane, in metres**: `q` against the
/// plane through `origin` with unit normal `normal`, `(q − origin)·normal`,
/// positive on the normal's side. A point needs no lever: its distance is
/// what a margin measures. The one spelling of the formula;
/// [`Frame::height`] reads it on a frame's plane. A plane with no seam has
/// no type of its own here (a [`Frame`] carries a seam): whether it gets
/// one is the placement decision's kind-struct stage, so a holder of a
/// bare `(origin, normal)` reads this.
#[must_use]
pub fn plane_offset<T: Real>(origin: Point3<T>, normal: Vec3<T>, q: Point3<T>) -> T {
    (q - origin).dot(normal)
}

/// [`plane_offset`] with a first-order bound on its own rounding, its
/// inputs exact ([`geom_core::running`]): the value is bit-identical to
/// [`plane_offset`]'s, so a door that decides on one charges the other.
#[must_use]
pub fn plane_offset_rounded<T: Real>(
    origin: Point3<T>,
    normal: Vec3<T>,
    q: Point3<T>,
) -> geom_core::running::Rounded<T> {
    use geom_core::running::{self, Rounded};
    let d = [(q.x, origin.x), (q.y, origin.y), (q.z, origin.z)]
        .map(|(a, b)| Rounded::exact(a) - Rounded::exact(b));
    running::dot(d, running::exact_vec(normal))
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::float_cmp)]
mod tests {
    use geom_core::{Band, Point3, RigidMotion, Tol, UnitVec3, Vec3};

    use super::Frame;

    fn tilted() -> Frame<f64> {
        Frame::new(
            Point3::new(0.5, -1.0, 2.0),
            Vec3::new(2.0, 2.0, 1.0) / 3.0,
            Vec3::new(1.0, -2.0, 2.0) / 3.0,
        )
    }

    #[test]
    fn local_coordinates_round_trip_and_split_into_foot_and_height() {
        let f = tilted();
        let p = Point3::new(1.25, 0.5, -0.75);
        let (x, y, h) = f.local(p);
        let back = f.at_local((x, y, h));
        assert!(
            (back - p).norm() < 1e-14,
            "at_local inverts local: {back:?}"
        );
        let foot = f.foot(p);
        assert!(f.height(foot).abs() < 1e-14, "the foot lies on the plane");
        assert!(
            (foot + f.axis * h - p).norm() < 1e-14,
            "the foot is p less its height along the axis"
        );
        assert!(
            (f.axis_line().foot(p) - f.at_local((0.0, 0.0, h))).norm() < 1e-14,
            "the axis line's foot sits at the point's height"
        );
    }

    #[test]
    fn a_motion_moves_a_frame_as_it_moves_the_points_the_frame_places() {
        let band = Band::linear(Tol::witness()).unwrap();
        let axis = UnitVec3::new(Vec3::new(1.0, 2.0, -0.5), "test", band).unwrap();
        let m =
            RigidMotion::rotation_then_translation(axis, 0.7, Vec3::new(0.3, -0.2, 1.1)).unwrap();
        let f = tilted();
        let moved = f.moved(&m);
        let at = (0.4, -1.3, 0.9);
        let image = m.transform_point(f.at_local(at));
        assert!(
            (moved.at_local(at) - image).norm() < 1e-14,
            "the moved frame places the image of each point the frame placed"
        );
        let placed = Frame::of_motion(&m);
        assert!(
            (placed.v_ref() - m.v().get()).norm() < 1e-15,
            "the frame a motion places derives the motion's own v"
        );
    }
}
