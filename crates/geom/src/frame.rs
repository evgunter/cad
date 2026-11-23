//! **A carrier's placement**, [`Frame`]: a point and a right-handed
//! orthonormal basis `(u_ref, v_ref, axis)`, held as the
//! [`RigidMotion`] it is, so its axes are unit and square as a property
//! of the type. `v_ref` is read as `axis × u_ref`.
//!
//! What each axis means on a carrier (a conic's plane normal and seam,
//! a surface's axis of revolution) is that kind's own docs'. The
//! readings of a placement alone below are its one home: a reader
//! holding a `Frame` asks it for them rather than spelling them again
//! from the axes.
//!
//! A frame is placed by a motion its construction mints
//! ([`Frame::of_motion`]), or stated as an origin and two directions and
//! squared with no decision ([`Frame::new`], [`RigidMotion::squared`]).
//! [`Frame::moved`] is its image under a motion. At another scalar it
//! is carried within one witness kind and re-minted across kinds
//! ([`Frame::map_scalar`]). An improper map has no reading here: its
//! image is a per-kind rule (a conic's axis, a plane's outward normal),
//! stated where the map is applied.

use geom_core::{Point3, Real, RigidMotion, UnitVec3, Vec3};

use crate::{Line, SquareFrame};

/// **A placement**: a point and a right-handed orthonormal basis
/// `(u_ref, v_ref, axis)` ([module docs](self)), held as the
/// [`RigidMotion`] it is.
///
/// Its `Debug` renders the frame's data, `origin`, `axis` and `u_ref`,
/// the three a frame is stated by and every reader reads; `v_ref` is
/// `axis × u_ref` by the motion's rigidity and is not rendered.
#[derive(Clone, Copy)]
pub struct Frame<T: Real> {
    motion: RigidMotion<T>,
}

impl<T: Real> core::fmt::Debug for Frame<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Frame")
            .field("origin", &self.origin())
            .field("axis", &self.axis())
            .field("u_ref", &self.u_ref())
            .finish()
    }
}

impl<T: Real> Frame<T> {
    /// The frame at `origin` with third axis `axis` and first axis
    /// `u_ref`, squared with no decision ([`RigidMotion::squared`]): the
    /// axis over its length, `u_ref`'s part square to it over its own.
    /// A construction that can decide its frame mints it and places it
    /// with [`Frame::of_motion`].
    #[must_use]
    pub fn new(origin: Point3<T>, axis: Vec3<T>, u_ref: Vec3<T>) -> Self {
        Self::of_motion(&RigidMotion::squared(origin, axis, u_ref))
    }

    /// **The frame a motion places**: its origin, `w` as the axis and
    /// `u` as the first axis, so the derived `v_ref` is the motion's `v`
    /// to rounding.
    #[must_use]
    pub fn of_motion(motion: &RigidMotion<T>) -> Self {
        Self { motion: *motion }
    }

    /// The motion this frame is.
    #[must_use]
    pub fn motion(&self) -> RigidMotion<T> {
        self.motion
    }

    /// The point the basis is placed at.
    #[must_use]
    pub fn origin(&self) -> Point3<T> {
        self.motion.origin()
    }

    /// The third axis.
    #[must_use]
    pub fn axis(&self) -> Vec3<T> {
        self.motion.w().get()
    }

    /// The third axis, as the witness it is.
    #[must_use]
    pub fn axis_unit(&self) -> UnitVec3<T> {
        self.motion.w()
    }

    /// The first axis, square to [`Self::axis`].
    #[must_use]
    pub fn u_ref(&self) -> Vec3<T> {
        self.motion.u().get()
    }

    /// The first axis, as the witness it is.
    #[must_use]
    pub fn u_ref_unit(&self) -> UnitVec3<T> {
        self.motion.u()
    }

    /// The same axes placed at `origin`.
    #[must_use]
    pub fn with_origin(&self, origin: Point3<T>) -> Self {
        Self::of_motion(&self.motion.with_origin(origin))
    }

    /// **The unit radial direction at azimuth `u`**, `u_ref·cos u +
    /// v_ref·sin u`: the azimuthal frame's own reading
    /// ([`crate::azimuth`]'s `Radial`), for a caller outside the crate.
    #[must_use]
    pub fn radial(&self, u: T) -> Vec3<T> {
        crate::azimuth::frame(self, u).radial.0
    }

    /// **The radial direction's derivative in the azimuth**, `u_ref·(−sin
    /// u) + v_ref·cos u`: the azimuthal frame's `Tangential`, beside
    /// [`Self::radial`].
    #[must_use]
    pub fn tangential(&self, u: T) -> Vec3<T> {
        crate::azimuth::frame(self, u).tangential.0
    }

    /// `axis × u_ref`, the second axis.
    #[must_use]
    pub fn v_ref(&self) -> Vec3<T> {
        self.axis().cross(self.u_ref())
    }

    /// The components of `p − origin` along `u_ref` and `v_ref`.
    #[must_use]
    pub fn in_plane(&self, p: Point3<T>) -> (T, T) {
        let w = p - self.origin();
        (w.dot(self.u_ref()), w.dot(self.v_ref()))
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
        plane_offset(self.origin(), self.axis(), p)
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
        self.origin() + self.u_ref() * x + self.v_ref() * y + self.axis() * h
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
            origin: self.origin(),
            dir: self.axis_unit(),
        }
    }

    /// The same placement run back: the axis flipped, `u_ref` kept, so
    /// the derived `v_ref` flips with it.
    #[must_use]
    pub fn reversed(&self) -> Self {
        Self::of_motion(&self.motion.reversed())
    }

    /// **The same placement turned a quarter turn, its seam the
    /// normalized `v_ref`** ([`RigidMotion::quarter_turned_renormalized`]):
    /// the axis kept bit for bit and `u_ref` set to `v_ref().normalize()`
    /// as given, nothing decided and nothing re-squared.
    #[must_use]
    pub fn quarter_turned_renormalized(&self) -> Self {
        Self::of_motion(&self.motion.quarter_turned_renormalized())
    }

    /// The same placement turned a quarter turn about its axis: `u_ref`
    /// becomes the old `v_ref`, and `v_ref` the old `−u_ref`.
    #[must_use]
    pub fn quarter_turned(&self) -> Self {
        Self::of_motion(&self.motion.quarter_turned())
    }

    /// The same placement turned a half turn about its axis: `u_ref` and
    /// `v_ref` both reversed.
    #[must_use]
    pub fn half_turned(&self) -> Self {
        Self::of_motion(&self.motion.half_turned())
    }

    /// **One of two placements, by the sign of `d`**
    /// ([`RigidMotion::select_le_zero`]): `when_le` where `d ≤ 0`,
    /// `when_gt` where it is positive.
    #[must_use]
    pub fn select_le_zero(d: T, when_le: Self, when_gt: Self) -> Self {
        Self::of_motion(&RigidMotion::select_le_zero(
            d,
            when_le.motion,
            when_gt.motion,
        ))
    }

    /// The same placement moved by `by`.
    #[must_use]
    pub fn translated(&self, by: Vec3<T>) -> Self {
        self.with_origin(self.origin() + by)
    }

    /// **The image under a rigid motion**: the origin moved, every axis
    /// turned by the motion's linear part ([`RigidMotion::carried_by`]).
    #[must_use]
    pub fn moved(&self, motion: &RigidMotion<T>) -> Self {
        Self::of_motion(&self.motion.carried_by(*motion))
    }

    /// **The same placement at another scalar** ([`RigidMotion::map_scalar`]):
    /// within one witness kind its entries carried through `f` as they
    /// are; from `f64` to `Interval` read as the enclosure of its polar
    /// factor. No read carries a frame into `Dual` or `Sym` from another
    /// scalar.
    #[must_use]
    pub fn map_scalar<U: Real>(&self, f: impl Fn(T) -> U) -> Frame<U> {
        Frame::of_motion(&self.motion.map_scalar(f))
    }

    /// **The `f64` placement this one's value channel holds**
    /// ([`RigidMotion::point_value`]), or `None` where an entry has no
    /// single finite `f64`.
    #[must_use]
    pub fn point_value(&self) -> Option<Frame<f64>>
    where
        T: geom_core::spline::SpanLocate,
    {
        Some(Frame::of_motion(&self.motion.point_value()?))
    }

    /// **The basis squared** ([`SquareFrame`]): the axis over its length
    /// and the seam taken as `a × (u_ref × a)` over the unit `a`. The
    /// stored axes are already unit and square, so this moves them only
    /// by rounding; it is kept as the arithmetic its readers' pins were
    /// taken on.
    #[must_use]
    pub fn square(&self) -> SquareFrame<T> {
        let axis = self.axis().normalize();
        let across = axis.cross(self.u_ref().cross(axis));
        let ((u_ref, len), (_, u_len)) = (
            across.direction_and_length(),
            self.u_ref().direction_and_length(),
        );
        SquareFrame {
            axis,
            u_ref,
            width: len / u_len,
        }
    }
}

#[cfg(any(test, feature = "test-support"))]
impl Frame<f64> {
    /// **A stated frame admitted as it is, at `f64`**: `axis` and `u_ref`
    /// kept bit for bit where the basis `(u_ref, axis × u_ref, axis)` is
    /// rigid within its own rounding ([`RigidMotion::check_rigid`]), and
    /// `None` where it is not. The door for a caller whose `f64` entries
    /// are a frame already, and whose readings are pinned to those very
    /// bits (an oracle computed off them); [`Frame::new`] squares instead.
    /// Test builds only: a production frame is minted by its construction.
    #[must_use]
    pub fn admitted(origin: Point3<f64>, axis: Vec3<f64>, u_ref: Vec3<f64>) -> Option<Self> {
        let map = geom_core::Affine3 {
            linear: geom_core::Mat3::from_cols(u_ref, axis.cross(u_ref), axis),
            translation: origin - Point3::origin(),
        };
        let motion = RigidMotion::check_rigid(&map).ok()?;
        Some(Self::of_motion(&motion))
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
    T::difference_dot(
        [q.x, q.y, q.z],
        [origin.x, origin.y, origin.z],
        [normal.x, normal.y, normal.z],
    )
}

/// The plain formula `(q − origin)·normal` with a first-order bound on
/// its own rounding, its inputs exact ([`geom_core::running`]).
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
            (foot + f.axis() * h - p).norm() < 1e-14,
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
