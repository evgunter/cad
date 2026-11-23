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

    /// **How far this frame's stored axes may be from the rotation they
    /// hold**, at this scalar: `(axis, seam)`, bounds on `‖axis − Rẑ‖` and
    /// `‖u_ref − Rx̂‖`. `R` is the polar factor of the stored axes, the
    /// rotation a frame means at every scalar
    /// ([`RigidMotion::rounding`], whose doc carries the argument: zero
    /// at an enclosure scalar, measured at a point scalar). A reader that
    /// levers a frame charges these in place of reading the frame's
    /// convention off its entries (fork 274 decision 1).
    #[must_use]
    pub fn rounding(&self) -> (T, T) {
        self.motion.rounding()
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
    /// **A frame's `f64` rounding covers its distance from its polar factor
    /// at every depth of composition, and is tight** ([`Frame::rounding`]):
    /// 16 chains of random turns composed to depth 64. Each frame's
    /// measured `(axis, seam)` is at least its exact distance from the
    /// polar factor of its stored axes
    /// ([`geom_core::test_support::polar_distance`], read independently of
    /// the bound), and the median ratio of the two is at most `1.5`.
    /// Composition adds rounding at every step, so no constant bounds it;
    /// the measure follows it. Red, below, where the invariants are read
    /// in plain `f64` with no charge, as `frame_defect` read them on main;
    /// red, above, where the distance is read as `|A − 1|` and not halved.
    #[test]
    fn a_frames_rounding_covers_its_defect_through_a_chain_of_turns() {
        let mut seed = 0x9e37_79b9_7f4a_7c15_u64;
        let mut next = || {
            seed ^= seed << 13;
            seed ^= seed >> 7;
            seed ^= seed << 17;
            #[allow(clippy::cast_precision_loss)]
            let x = (seed >> 11) as f64 / (1u64 << 53) as f64;
            x
        };
        let band = Band::linear(Tol::witness()).unwrap();
        let mut worst = (0.0f64, 0.0f64);
        let mut ratios: Vec<(f64, f64)> = Vec::new();
        // The oracle's own error: a few ulps of its size.
        let under = |bound: f64, truth: f64| bound < truth * (1.0 - 8.0 * f64::EPSILON);
        for chain in 0..16 {
            let mut frame = tilted();
            for depth in 0..64 {
                let axis = Vec3::new(next() - 0.5, next() - 0.5, next() - 0.5);
                let axis = UnitVec3::new(axis, "test_chain_axis", band).unwrap();
                let turn = RigidMotion::rotation(axis, 6.0 * next()).unwrap();
                frame = frame.moved(&turn);
                let (d_axis, d_seam) = frame.rounding();
                let (t_axis, t_seam) = geom_core::test_support::polar_distance(&frame.motion());
                worst = (worst.0.max(t_axis), worst.1.max(t_seam));
                if t_axis > 0.0 {
                    ratios.push((d_axis / t_axis, f64::NAN));
                }
                if t_seam > 0.0 {
                    ratios.push((f64::NAN, d_seam / t_seam));
                }
                assert!(
                    !under(d_axis, t_axis) && !under(d_seam, t_seam),
                    "chain {chain}, depth {depth}: rounding ({d_axis:e}, {d_seam:e}) under the \
                     polar distance ({t_axis:e}, {t_seam:e})"
                );
            }
        }
        assert!(
            worst.0 > 0.0 || worst.1 > 0.0,
            "the chains carry a defect to cover"
        );
        let median = |mut v: Vec<f64>| {
            v.retain(|x| x.is_finite());
            v.sort_by(f64::total_cmp);
            v[v.len() / 2]
        };
        let (axis, seam) = (
            median(ratios.iter().map(|r| r.0).collect()),
            median(ratios.iter().map(|r| r.1).collect()),
        );
        println!("rounding over polar distance: axis median {axis:.3}, seam median {seam:.3}");
        assert!(
            axis <= 1.5 && seam <= 1.5,
            "the measure is tight: median ratios ({axis:.3}, {seam:.3}) past 1.5"
        );
    }

    /// **Whether an enclosure frame can hold a rotation**: each axis's
    /// squared length can be `1` and each pair's dot product `0` over the
    /// box, every range read exactly off the brackets' ends (each product
    /// an exact [`geom_core::exact::Expansion`], the least and greatest of
    /// a bracket product chosen among its four end products by their
    /// values, which a tie within `10⁻³⁰` cannot turn). A necessary
    /// condition: a box holding a rotation passes it, and a box of points
    /// that are a rotation's `f64` rounding fails it. It does not catch a
    /// widened box: a wider box still holds the rotation, so widening is
    /// a question of width, not of soundness, and not this check's.
    fn can_hold_a_rotation(f: &Frame<geom_core::Interval>) -> bool {
        use geom_core::Bounds;
        use geom_core::exact::Expansion;
        let ends = |x: geom_core::Interval| (x.lo(), x.hi());
        let value = |e: &Expansion| e.components().iter().sum::<f64>();
        // The exact least and greatest of `x·y` over the two brackets.
        let range = |x: (f64, f64), y: (f64, f64)| {
            let mut cands = [
                Expansion::product(x.0, y.0),
                Expansion::product(x.0, y.1),
                Expansion::product(x.1, y.0),
                Expansion::product(x.1, y.1),
            ];
            cands.sort_by(|a, b| value(a).total_cmp(&value(b)));
            let [lo, _, _, hi] = cands;
            (lo, hi)
        };
        let square = |x: (f64, f64)| {
            let (lo, hi) = range(x, x);
            if x.0 <= 0.0 && x.1 >= 0.0 {
                (Expansion::product(0.0, 0.0), hi)
            } else {
                (lo, hi)
            }
        };
        let spans = |terms: [(Expansion, Expansion); 3], target: f64| {
            let (mut lo, mut hi) = (
                Expansion::product(-target, 1.0),
                Expansion::product(-target, 1.0),
            );
            for (l, h) in terms {
                lo.add_expansion(&l);
                hi.add_expansion(&h);
            }
            value(&lo) <= 0.0 && value(&hi) >= 0.0
        };
        let axes = [f.axis(), f.u_ref(), f.v_ref()];
        let col = |v: Vec3<geom_core::Interval>| [ends(v.x), ends(v.y), ends(v.z)];
        let unit = axes.iter().all(|&v| {
            let c = col(v);
            spans([square(c[0]), square(c[1]), square(c[2])], 1.0)
        });
        let square_pairs = [(0, 1), (0, 2), (1, 2)].iter().all(|&(i, j)| {
            let (a, b) = (col(axes[i]), col(axes[j]));
            spans(
                [range(a[0], b[0]), range(a[1], b[1]), range(a[2], b[2])],
                0.0,
            )
        });
        unit && square_pairs
    }

    /// **At an enclosure scalar every mint can hold the rotation it
    /// stands for** ([`Frame::rounding`] is zero there by that argument):
    /// for each mint path the enclosure passes [`can_hold_a_rotation`],
    /// read exactly with no pad. The paths: the polar read of an `f64`
    /// frame (`map_scalar`), a stated frame squared (`Frame::new`), a turn
    /// by a boxed angle, a composition, an inverse, the turn helpers and a
    /// reversal. Red where the polar read is replaced by the stated
    /// entries as points, which a frame rounded at `f64` is not.
    #[test]
    fn every_enclosure_mint_encloses_the_rotation_it_stands_for() {
        use geom_core::{Interval, Real};
        let box_ = |lo: f64, hi: f64| Interval::from_bounds(lo, hi);
        let holds = |name: &str, i: &Frame<Interval>| {
            assert!(
                can_hold_a_rotation(i),
                "{name}: the enclosure holds no rotation: {i:?}"
            );
        };
        let f = tilted();
        holds("polar read", &f.map_scalar(Interval::from_f64));
        let o = Point3::origin().map(Interval::from_f64);
        let lift = |v: Vec3<f64>| v.map(Interval::from_f64);
        let boxed = Frame::new(
            o,
            Vec3::new(box_(2.0, 2.0), box_(2.0, 2.0), box_(0.52, 0.54)),
            lift(Vec3::new(1.0, -2.0, 2.0)),
        );
        holds("stated and squared", &boxed);
        let turn = RigidMotion::rotation(UnitVec3::<Interval>::exact_y(), box_(0.5, 0.52)).unwrap();
        let base = Frame::new(
            o,
            lift(Vec3::new(2.0, 2.0, 0.52)),
            lift(Vec3::new(1.0, -2.0, 2.0)),
        );
        holds("a boxed turn", &Frame::of_motion(&turn));
        holds("composed", &base.moved(&turn));
        holds("inverted", &Frame::of_motion(&turn.inverse().unwrap()));
        holds("quarter turned", &base.quarter_turned());
        holds("half turned", &base.half_turned());
        holds("reversed", &base.reversed());
    }
}
