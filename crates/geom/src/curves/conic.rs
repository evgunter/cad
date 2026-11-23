//! **The conics: a circle and an ellipse carrier, and what they share.**
//!
//! [`Curve3::Circle`] and [`Curve3::Ellipse`] are two kinds: a circle is
//! not degraded into an ellipse with equal semi-axes, and an ellipse is
//! not a circle with a stretch. What the two share is named once and
//! held by each:
//!
//! - **The placement**, [`Frame`]: `axis` the unit plane normal and
//!   `u_ref` the unit seam, `θ = 0` on it, read at the conic's centre
//!   ([`Conic::frame`]). A reading of the placement alone (a point's
//!   in-plane components, the frame reversed or moved) is [`Frame`]'s.
//! - **The readings either kind answers**, the [`Conic`] trait: the point
//!   and the derivatives at `θ`, the angle of a point and its parameter on
//!   a branch, the speed bounds, the major frame. A reader that treats the
//!   two kinds alike takes a `&dyn Conic` ([`Curve3::conic`]); it never
//!   assembles its own `(centre, axis, u_ref, a, b)` from the fields.
//!
//! Each kind keeps its own shape beside its frame: a [`Circle`] its
//! `radius`, an [`Ellipse`] its `semi_u` along `u_ref` and `semi_v` along
//! `v_ref`, **in stored order**. An ellipse's `u_ref` is a principal
//! direction along either semi-axis, so the order is where the seam sits,
//! not which semi-axis is larger; which one is larger is a reading of the
//! ellipse ([`Ellipse::semi_axes_by_size`], [`Ellipse::major_frame`]), and
//! on a near-circle it is rounding, so a reader that needs the major
//! direction takes it from the surface the ellipse is a section of
//! ([`Ellipse::section_frame`]).
//!
//! # Where a conic is held
//!
//! A conic stores ONE point. A circle's is its centre. An ellipse's is
//! its centre or a point of its arc, and its private placement says
//! which; the other is derived by name on each read.
//!
//! - **Held at its centre** ([`Circle::new`], [`Ellipse::new`], and the
//!   doors over them): the placement stands at the centre as given, and
//!   a point is `center + u_ref·(a·cos θ) + v_ref·(b·sin θ)`. Every bit
//!   a file or a caller stated is kept, and a point at a quarter turn is
//!   the centre moved by one semi-axis exactly.
//! - **Held on its arc** ([`Ellipse::anchored`]): the placement stands
//!   at the **anchor**, a point of the ellipse, with the pair
//!   `(cos θ₀, sin θ₀)` of the anchor's eccentric anomaly from `u_ref`
//!   beside it. A point is summed from the anchor,
//!   `anchor + u_ref·a·(cos θ − cos θ₀) + v_ref·b·(sin θ − sin θ₀)`, the
//!   first offset read as `−(2·sin²(θ/2) − (1 − cos θ₀))`. The centre is
//!   `anchor − u_ref·(a·cos θ₀) − v_ref·(b·sin θ₀)` ([`Conic::center`],
//!   [`Conic::frame`]), derived on each read and so rounded at the
//!   semi-axes' scale; a reader that wants a centre as a datum reads
//!   that, and nothing stores it.
//!
//! An ellipse is minted on its arc with the anchor at the seam's own
//! vertex, `θ₀ = 0`, the pair `(1, 0)`: a point is then
//! `anchor − u_ref·a·2·sin²(θ/2) + v_ref·b·sin θ`, with no cancellation
//! at any `θ`, and a point near the anchor holds to the rounding of its
//! own coordinates however long the semi-axes. A section whose centre
//! lies a megametre off the arc it serves is stored at the arc.
//!
//! **A re-seam moves parameter zero and never the anchor.** Turning the
//! seam a quarter or a half turn ([`Ellipse::seamed_near`]) permutes
//! `u_ref`, the semi-axes and the pair exactly, and the anchor is the
//! stored one bit for bit; so the anchor is the point at `θ = 0` only
//! until the seam is turned. The parameter stays the eccentric anomaly
//! from `u_ref`, which is what every reader pairs with the principal
//! directions. With the anchor a quarter or a half turn from the seam
//! the parameter itself carries that turn, and a point near the anchor
//! holds to the semi-axis's rounding, not its own. The derived centre is
//! read from the same anchor at every seam, but not to the same bits: at
//! `f64` a quarter turn takes the turned seam renormalized
//! ([`Frame::quarter_turned_renormalized`]), a rounding apart from the
//! frame's own `v`, and the centre read through the turned frame moves
//! by a few roundings of the semi-axis, more as the seam handed in
//! nears the axis; across a half turn, which is exact, it does not move.
//!
//! The placement is private to this module, and nothing outside it asks
//! which point a conic holds. Where a conic is, against a foreign point
//! or along a direction, is read through it from the point it stores:
//! a point's height over its plane ([`Conic::height_of`]), its level and
//! direction ([`Conic::level_at`], [`Conic::azimuth`],
//! [`Conic::unit_in_plane`]), and the conic's height along a direction
//! as a first harmonic about that point, posed per placement inside one
//! opaque value that answers at a parameter, over a window, at its
//! extremes and at its roots ([`Conic::height_along`], [`Height`]). The
//! centre itself is read as the centre
//! ([`Conic::center`], the origin of [`Conic::frame`]), and the stored
//! data are walked by name ([`Curve3::data`], whose datum for the stored
//! point is [`CurveDatum::Center`] or [`CurveDatum::Anchor`]). A motion
//! of the conic maps its one placement ([`Circle::placed`],
//! [`Ellipse::mapped`]), so the image is held as the conic was.
//!
//! A reader that asks where the conic is asks the conic: its point
//! ([`Conic::point_at`]), a point's parameter ([`Conic::azimuth`]), its
//! extent along a direction over a span ([`Conic::extent_along`]) and
//! its cover over a span ([`Conic::cover_over`]), each stated from the
//! stored point. The directions ([`Conic::axis`], [`Conic::u_ref`],
//! [`Conic::v_ref`]) are stored and derive nothing.
//!
//! Each conic evaluates one way, [`Conic::point_at`], and
//! [`Conic::running_point`] is that evaluation operation for operation
//! with its bound. The derivatives read no position and are the same
//! however the conic is held.
//!
//! [`CurveDatum::Center`]: crate::CurveDatum::Center
//! [`CurveDatum::Anchor`]: crate::CurveDatum::Anchor
//!
//! # Conventions
//!
//! - **The parameter** `θ` is in radians, domain ℝ, period 2π, and winds
//!   counterclockwise viewed from the tip of `axis` (right-hand rule). A
//!   circle's is its angle, `|dP/dθ| = radius`; an ellipse's is its
//!   **eccentric anomaly**, not arc length and not the polar angle of the
//!   point: `|dP/dθ|² = semi_u²sin²θ + semi_v²cos²θ` varies between the
//!   two semi-axes.
//! - **A unit frame, by its type.** The frame is a `RigidMotion`, its
//!   axes unit and square by the mint that made it, so nothing that reads
//!   a conic normalizes it: the sizes are the lengths every speed and
//!   curvature bound is stated at, and the frame's axes carry no length
//!   of their own to rescale them by.
//! - **Non-negative sizes.** A radius and both semi-axes are lengths,
//!   in metres, and a negative one is unrepresentable: the fields are
//!   private and the constructors ([`Circle::new`], [`Ellipse::new`])
//!   panic on a size that is not finite or is definitely negative
//!   ([`stored_size`]). The other orientation is the frame's, never a
//!   size's sign. An enclosure across zero is representable, and the
//!   speed readers read it soundly; a zero size is a collapsed length,
//!   which certification's span meter refuses ([`speed_floor`]). A door
//!   taking a caller's sizes ([`Curve3::ellipse`]) decides their sign
//!   before it builds and refuses typed.
//! - **Equal semi-axes are an ellipse.** Whether a configuration is a
//!   circle or an ellipse is decided once by its pose (a section table's
//!   tilt reading), never by comparing the semi-axes.
//! - **The seam.** A caller seams an ellipse at the principal vertex
//!   nearest the part of it in use ([`Ellipse::seamed_near`]), so the
//!   parameter there is near zero. On an ellipse held at its centre, or
//!   on its arc at that vertex, the parameter then resolves to the scale
//!   of that part; held on its arc elsewhere, a point there holds to the
//!   semi-axis's rounding ("Where a conic is held").
//! - **One branch rule.** A point's parameter is its direction
//!   ([`Conic::azimuth`]) read within half a turn of a reference
//!   ([`Azimuth::near`]): a parameter the caller holds, or a span's middle
//!   ([`Conic::param_in`]).
//!
//! [`speed_floor`]: crate::carrier_speed::speed_floor

#![allow(
    clippy::disallowed_methods,
    reason = "the conic's own module: the centre is derived here and nowhere else"
)]

use geom_core::{Decide, Point3, Real, Rounded, Vec3};

use crate::azimuth::{self, Azimuth};
use crate::cover::{Cover, ParamWindow};
use crate::curves::Curve3;
use crate::curves::boxes::ArcWindow;
use crate::frame::Frame;

/// **A full circle**, `P(θ) = center + (u_ref·cos θ + v_ref·sin θ)·radius`.
///
/// Its invariants: the axis is unit, the radius never definitely
/// negative ([`Circle::new`]), and the seam (`u_ref`) is carried whether
/// or not a reader reads it.
#[derive(Clone, Copy, Debug)]
pub struct Circle<T: Real> {
    /// The placement, at the centre: `axis` the unit plane normal
    /// (right-hand winding), `u_ref` the seam, where `θ = 0` lives.
    at: Frame<T>,
    /// The radius in metres, never definitely negative ([`Circle::new`]).
    radius: T,
}

/// **Which point an ellipse's placement stands at** ([module
/// docs](self), "Where a conic is held"). Matched in this module only.
#[derive(Clone, Copy, Debug)]
enum Held<T: Real> {
    /// At the centre.
    Centre,
    /// At the anchor, a point of the arc.
    Arc {
        /// `(cos θ₀, sin θ₀)` of the anchor's eccentric anomaly from
        /// `u_ref`: `(1, 0)` as minted, and one of `(±1, 0)`, `(0, ±1)`
        /// wherever the seam's turns were decided. A re-seam turns
        /// `u_ref` and this pair by one exact permutation and leaves the
        /// anchor where it is.
        phase: (T, T),
    },
}

/// **A full ellipse**,
/// `P(θ) = center + u_ref·(semi_u·cos θ) + v_ref·(semi_v·sin θ)`, `u_ref`
/// a principal direction.
#[derive(Clone, Copy, Debug)]
pub struct Ellipse<T: Real> {
    /// The placement: `axis` the unit plane normal (right-hand
    /// winding), `u_ref` the seam, where `θ = 0` lives, a principal
    /// direction along either semi-axis, and `origin` the one point the
    /// ellipse stores, which `held` names.
    at: Frame<T>,
    /// Which point `at` stands at.
    held: Held<T>,
    /// The semi-axis along `u_ref`, in metres, never definitely negative
    /// ([`Ellipse::new`]).
    semi_u: T,
    /// The semi-axis along `axis × u_ref`, likewise.
    semi_v: T,
}

/// An ellipse's principal frame with a chosen principal direction first
/// ([`Conic::major_frame`], [`Ellipse::section_frame`]): the reading of
/// which semi-axis is the major one, for a reader whose formula needs the
/// major vertex (a focus on the major axis, a lever at the farthest reach
/// along it).
#[derive(Clone, Copy, Debug)]
pub struct MajorFrame<T: Real> {
    /// The semi-axis along `major_dir`.
    pub major: T,
    /// The semi-axis along `axis × major_dir`.
    pub minor: T,
    /// The principal direction read as the major one: `u_ref`, or
    /// `axis × u_ref`.
    pub major_dir: Vec3<T>,
    /// The carrier parameter at `center + major_dir·major`: `0`, or
    /// `π/2` where `major_dir` is `axis × u_ref`.
    pub phase: T,
}

/// **The readings a circle and an ellipse both answer.** See the
/// [module docs](self) for the conventions every method reads.
///
/// The evaluators are each kind's own formula, fixed in its evaluation
/// order (D9), and [`Curve3`]'s evaluators call them, so a reader through
/// this trait builds the very bits [`Curve3::eval`] does.
pub trait Conic<T: Real>: core::fmt::Debug {
    /// **The principal frame at the centre**: the stored placement of a
    /// conic held at its centre, and for one held on its arc its
    /// placement moved back to the derived centre ([`Self::center`]).
    fn frame(&self) -> Frame<T>;

    /// The unit plane normal, as stored.
    fn axis(&self) -> Vec3<T>;

    /// The unit seam, as stored: the direction `θ = 0` lies along.
    fn u_ref(&self) -> Vec3<T>;

    /// `axis × u_ref`, crossed on each read as [`Frame::v_ref`] is.
    fn v_ref(&self) -> Vec3<T>;

    /// `(semi_u, semi_v)` in stored order: a circle's are its radius
    /// twice. Both are finite and never definitely negative (the
    /// constructors' [`stored_size`]); an enclosure may reach below zero,
    /// and the speed readers below read it soundly.
    fn semi_axes(&self) -> (T, T);

    /// The point at `theta`.
    fn point_at(&self, theta: T) -> Point3<T>;

    /// `dP/dθ` at `theta`.
    fn derivative_at(&self, theta: T) -> Vec3<T>;

    /// The point and `dP/dθ` at `theta`, each half bit for bit its own
    /// evaluator's.
    fn jet_at(&self, theta: T) -> (Point3<T>, Vec3<T>);

    /// `d²P/dθ²` at `theta`, the negated offset from the centre
    /// (`P + P″ = center` in ℝ).
    fn second_derivative_at(&self, theta: T) -> Vec3<T>;

    /// **The direction of `p` about the centre, in the conic's unit
    /// coordinates**: the angle it is read as ([`Azimuth::near`],
    /// [`Azimuth::principal`]) is the parameter whose point `p` is, for a
    /// point of the conic, and the parameter of its projection along the
    /// centre's ray (scaled per axis on an ellipse) otherwise.
    fn azimuth(&self, p: Point3<T>) -> Azimuth<T>;

    /// The least speed `|P′|` over the carrier: the smaller semi-axis,
    /// floored at zero, which a speed never falls below.
    fn speed_lo(&self) -> T;

    /// The greatest speed `|P′|` over the carrier, and a bound on `|P″|`:
    /// the larger semi-axis, held above the smaller's negation so the
    /// bound holds over an enclosure that reaches below zero.
    fn speed_hi(&self) -> T;

    /// The speed `|P′(θ)|` at `theta`, held to
    /// `[`[`Self::speed_lo`]`, `[`Self::speed_hi`]`]`: a circle's is its
    /// radius's magnitude at every scalar, the radius itself wherever it
    /// is a point.
    fn speed_at(&self, theta: T) -> T;

    /// The principal frame with the larger semi-axis first, whichever the
    /// seam is at; a tie, and a circle, keep the seam's own axis.
    fn major_frame(&self) -> MajorFrame<T>;

    /// **The point at the parameter `theta` with a first-order running
    /// bound on its rounding** ([`Rounded`]): [`Self::point_at`]
    /// operation for operation, so its value is that point bit for bit.
    /// `theta`, the frame, the sizes and the stored centre or anchor are
    /// taken as exact. Every operation is charged its rounding: each sine
    /// or cosine read an ulp, and on an ellipse held on its arc the
    /// half-angle square and the two differences with the anchor's phase
    /// their own.
    fn running_point(&self, theta: T) -> [Rounded<T>; 3];

    /// **The centre**: the frame's origin. Stored where the conic is
    /// held at it; derived from the anchor where it is held on its arc,
    /// and then rounded at the semi-axes' own scale.
    fn center(&self) -> Point3<T> {
        self.frame().origin()
    }

    /// **`p` in the conic's unit coordinates**: its in-plane components
    /// over the semi-axes, `((p − c)·u_ref/a, (p − c)·v_ref/b)`, in which
    /// the conic is the unit circle and its parameter the polar angle.
    /// Held on its arc, read from the anchor, `(cos θ₀ + x/a, sin θ₀ + y/b)`
    /// with `(x, y)` `p`'s components from it; the centre is not formed.
    fn unit_in_plane(&self, p: Point3<T>) -> (T, T) {
        self.unit_of(p - self.frame().origin())
    }

    /// **The `(lo, hi)` of `(P(θ) − origin)·n` over `window`**:
    /// [`Self::height_along`]'s [`Height::extent`]. `n` need not be unit.
    fn extent_along(&self, window: &ArcWindow<T>, origin: Point3<T>, n: Vec3<T>) -> (T, T) {
        self.height_along(origin, n).extent(window)
    }

    /// **The conic's height along `n` from `o`**, `(P(θ) − o)·n`, a first
    /// harmonic in `θ` stated from the point the conic stores ([`Height`]).
    /// `n` need not be unit.
    fn height_along(&self, o: Point3<T>, n: Vec3<T>) -> Height<T>;

    /// **The cover of the arc over `window`**, stated from the point
    /// the conic stores. This body is a conic held at its centre's
    /// ([`Cover::arc`] on its frame); a kind that can be held elsewhere
    /// states its own. `None` for a datum that is not a finite number.
    fn cover_over(&self, window: ParamWindow<T>) -> Option<Cover<T>> {
        Cover::arc(&self.frame(), self.semi_axes(), window)
    }

    /// **`p`'s height over the conic's plane**, `(p − x)·axis`, `x` the
    /// point the conic stores: its centre, or the anchor of an ellipse held
    /// on its arc, which lies in the plane as stored. Read at `p`'s own
    /// distance from that point, so a point beside the anchor reads at
    /// the arc's scale however far the centre lies.
    fn height_of(&self, p: Point3<T>) -> T {
        (p - self.frame().origin()).dot(self.axis())
    }

    /// **The conic's level at the foreign point `p`**, with its
    /// gradient: `F = X²/a² + Y²/b² − 1`, `(X, Y)` the in-plane
    /// components of `p` from the centre along `(u_ref, v_ref)` and
    /// `(a, b)` the stored semi-axes, and `(∂F/∂X, ∂F/∂Y) = (2X/a², 2Y/b²)`.
    /// Negative inside, positive outside, zero on the conic; the height
    /// of `p` over the plane plays no part. Held at its centre, `X` and
    /// `Y` are read from it; held on its arc, from the anchor, and `F` is
    /// formed as `2(cos θ₀·x/a + sin θ₀·y/b) + (x/a)² + (y/b)²`, `(x, y)`
    /// `p`'s components from the anchor, with no cancellation: a point
    /// beside the arc there reads its defect at the arc's scale.
    fn level_at(&self, p: Point3<T>) -> (T, (T, T)) {
        centred_level(self.unit_in_plane(p), self.semi_axes())
    }

    /// **The vector `w` in the conic's unit coordinates**:
    /// `(w·u_ref/a, w·v_ref/b)`, [`Self::unit_in_plane`] for an offset
    /// rather than a point.
    fn unit_of(&self, w: Vec3<T>) -> (T, T) {
        let (a, b) = self.semi_axes();
        (w.dot(self.u_ref()) / a, w.dot(self.v_ref()) / b)
    }

    /// **The conic's harmonic along `n`**: `(a·(u_ref·n), b·(v_ref·n))`,
    /// so `(P(θ) − center)·n = first·cos θ + second·sin θ`. `n` need not
    /// be unit: the pair is linear in it.
    ///
    /// Read off `n`'s part across the conic's plane, `(â × n) × â`, which
    /// the frame's directions see as `n` itself, formed with no
    /// cancellation: as the conic turns parallel to `n`'s plane, `n·û` and
    /// `n·v̂` are differences of nearly equal terms, each off by `u`, so
    /// the harmonic's phase would be `u/sin` off and a graze's point
    /// `r·u/sin` along the conic. `â × n = â × (n ∓ |n|·â)`, and
    /// `n − |n|·â` (or `n + |n|·â`, the conic facing away) is formed to `u`
    /// of its own size, so its cross with `â` cancels nothing. Away from parallel both
    /// forms are as good, so the choice at `â·n = 0` is free.
    fn along(&self, n: Vec3<T>) -> (T, T) {
        let (a, b) = self.semi_axes();
        let axis = self.axis();
        let across = |m: Vec3<T>| axis.cross(m).cross(axis);
        // `|n|·â`, so the difference rounds at `n`'s own scale.
        let scaled = axis * n.norm();
        let (facing, away) = (across(n - scaled), across(n + scaled));
        let key = axis.dot(n);
        let d = Vec3::new(
            key.select_le_zero(away.x, facing.x),
            key.select_le_zero(away.y, facing.y),
            key.select_le_zero(away.z, facing.z),
        );
        // `(â × m) × â` is `m`'s part across the plane only for a unit
        // `â`; the frame's axis is unit by the conic's convention.
        (a * self.u_ref().dot(d), b * self.v_ref().dot(d))
    }

    /// **The parameter of `p` within half a turn of `span`'s middle**:
    /// its [direction](Self::azimuth) read [near](Azimuth::near) the
    /// middle.
    fn param_in(&self, p: Point3<T>, (t0, t1): (T, T)) -> T
    where
        T: Decide,
    {
        self.azimuth(p).near(crate::mid_param(t0, t1))
    }

    /// The semi-axis vectors `semi_u·u_ref` and `semi_v·v_ref`, in stored
    /// order: the conic is `center + cos θ·first + sin θ·second`.
    fn semi_axis_vectors(&self) -> (Vec3<T>, Vec3<T>) {
        let (a, b) = self.semi_axes();
        (self.u_ref() * a, self.v_ref() * b)
    }

    /// A bound on the curvature over the carrier, `speed_hi / speed_lo²`:
    /// an ellipse's tightest bend, at the ends of its larger semi-axis.
    fn curvature_hi(&self) -> T {
        self.speed_hi() / self.speed_lo().powi(2)
    }

    /// A bound on the speed `|P′(θ)|` over the span `(t0, t1)`: `sin θ`
    /// and `cos θ` move by at most the half-width `h` from their values
    /// at the span's middle, so `|sin θ| ≤ min(1, |sin m| + h)` and
    /// likewise `cos`; held to [`Self::speed_hi`].
    fn speed_over(&self, (t0, t1): (T, T)) -> T {
        let (a, b) = self.semi_axes();
        let h = (t1 - t0).abs() * T::from_f64(0.5);
        let (sin, cos) = crate::mid_param(t0, t1).sin_cos();
        let s = (sin.abs() + h).min(T::one());
        let c = (cos.abs() + h).min(T::one());
        ((a * s).powi(2) + (b * c).powi(2))
            .sqrt()
            .min(self.speed_hi())
    }
}

/// A borrowed conic reads as the conic, so a `&dyn Conic` held by value
/// passes where a conic is asked for.
impl<T: Real, C: Conic<T> + ?Sized> Conic<T> for &C {
    fn frame(&self) -> Frame<T> {
        (**self).frame()
    }
    fn axis(&self) -> Vec3<T> {
        (**self).axis()
    }
    fn u_ref(&self) -> Vec3<T> {
        (**self).u_ref()
    }
    fn v_ref(&self) -> Vec3<T> {
        (**self).v_ref()
    }
    fn semi_axes(&self) -> (T, T) {
        (**self).semi_axes()
    }
    fn point_at(&self, theta: T) -> Point3<T> {
        (**self).point_at(theta)
    }
    fn derivative_at(&self, theta: T) -> Vec3<T> {
        (**self).derivative_at(theta)
    }
    fn jet_at(&self, theta: T) -> (Point3<T>, Vec3<T>) {
        (**self).jet_at(theta)
    }
    fn second_derivative_at(&self, theta: T) -> Vec3<T> {
        (**self).second_derivative_at(theta)
    }
    fn azimuth(&self, p: Point3<T>) -> Azimuth<T> {
        (**self).azimuth(p)
    }
    fn speed_lo(&self) -> T {
        (**self).speed_lo()
    }
    fn speed_hi(&self) -> T {
        (**self).speed_hi()
    }
    fn speed_at(&self, theta: T) -> T {
        (**self).speed_at(theta)
    }
    fn major_frame(&self) -> MajorFrame<T> {
        (**self).major_frame()
    }
    fn running_point(&self, theta: T) -> [Rounded<T>; 3] {
        (**self).running_point(theta)
    }
    fn center(&self) -> Point3<T> {
        (**self).center()
    }
    fn extent_along(&self, window: &ArcWindow<T>, origin: Point3<T>, n: Vec3<T>) -> (T, T) {
        (**self).extent_along(window, origin, n)
    }
    fn cover_over(&self, window: ParamWindow<T>) -> Option<Cover<T>> {
        (**self).cover_over(window)
    }
    fn along(&self, n: Vec3<T>) -> (T, T) {
        (**self).along(n)
    }
    fn unit_in_plane(&self, p: Point3<T>) -> (T, T) {
        (**self).unit_in_plane(p)
    }
    fn height_of(&self, p: Point3<T>) -> T {
        (**self).height_of(p)
    }
    fn level_at(&self, p: Point3<T>) -> (T, (T, T)) {
        (**self).level_at(p)
    }
    fn height_along(&self, o: Point3<T>, n: Vec3<T>) -> Height<T> {
        (**self).height_along(o, n)
    }
    fn unit_of(&self, w: Vec3<T>) -> (T, T) {
        (**self).unit_of(w)
    }
    fn param_in(&self, p: Point3<T>, span: (T, T)) -> T
    where
        T: Decide,
    {
        (**self).param_in(p, span)
    }
    fn semi_axis_vectors(&self) -> (Vec3<T>, Vec3<T>) {
        (**self).semi_axis_vectors()
    }
    fn curvature_hi(&self) -> T {
        (**self).curvature_hi()
    }
    fn speed_over(&self, span: (T, T)) -> T {
        (**self).speed_over(span)
    }
}

/// A frame's origin, `u_ref` and `v_ref` entered exactly into running
/// arithmetic, `v_ref` crossed there in [`Vec3::cross`]'s order.
fn running_frame<T: Real>(f: &Frame<T>) -> RunningFrame<T> {
    use geom_core::running::{cross, exact_vec};
    let (n, u) = (exact_vec(f.axis()), exact_vec(f.u_ref()));
    (exact_point(f.origin()), u, cross(n, u))
}

/// A point's coordinates entered exactly into running arithmetic.
fn exact_point<T: Real>(p: Point3<T>) -> [Rounded<T>; 3] {
    geom_core::running::exact_vec(Vec3::new(p.x, p.y, p.z))
}

/// A frame in running arithmetic: origin, `u_ref`, `v_ref`.
type RunningFrame<T> = ([Rounded<T>; 3], [Rounded<T>; 3], [Rounded<T>; 3]);

/// **A conic's size as it may be stored**: `x` itself, where it is a
/// finite length that is not definitely negative.
///
/// `−x` decides above zero ([`Real::select_le_zero`] picks its second
/// candidate) exactly where `x` is negative at every point it carries;
/// an enclosure across zero hulls both candidates, so the second read,
/// with the poison first, tells a decided negative from a straddle,
/// which is representable. At `f64` it is `x < 0`.
///
/// # Panics
///
/// Where `x` is not bounded ([`Real::is_bounded`]) or is definitely
/// negative, naming `door` and `name`. Only kernel code constructs a
/// conic; a door taking a caller's size decides it first.
fn stored_size<T: Real>(x: T, door: &str, name: &str) -> T {
    let poison = T::from_f64(f64::NAN);
    let minus = T::zero() - x;
    let negative = minus.select_le_zero(T::zero(), poison).is_poison()
        && !minus.select_le_zero(poison, T::zero()).is_poison();
    assert!(
        x.is_bounded() && !negative,
        "{door}: {name} {x:?} is not a finite non-negative length \
         (the other placement is the frame's)"
    );
    x
}

/// **The two offsets an arc-held ellipse's point is summed from**, the
/// anchor at the anomaly whose `(cos, sin)` is `phase`:
/// `(cos θ − cos θ₀, sin θ − sin θ₀)`, the first read as
/// `−(2·sin²(θ/2) − (1 − cos θ₀))`. At the seam's own vertex,
/// `phase = (1, 0)`, they are `(−2·sin²(θ/2), sin θ)` to the bit, with no
/// cancellation at any `θ`. Each is a function of one sine of `θ` alone,
/// so an enclosure of `θ` gives the enclosure of each with no dependency
/// between two factors.
fn offsets<T: Real>(theta: T, (c0, s0): (T, T)) -> (T, T) {
    let (sh, _) = (theta * T::from_f64(0.5)).sin_cos();
    let (s, _) = theta.sin_cos();
    (-(T::from_f64(2.0) * sh.powi(2) - (T::one() - c0)), s - s0)
}

/// A sine or a cosine as computed, charged an ulp.
fn transcendental<T: Real>(x: T) -> Rounded<T> {
    Rounded {
        value: x,
        error: T::from_f64(2.0 * geom_core::UNIT_ROUNDOFF) * x.abs(),
    }
}

/// `(sin θ, cos θ)` in running arithmetic, each charged an ulp.
fn running_sin_cos<T: Real>(theta: T) -> (Rounded<T>, Rounded<T>) {
    let (s, c) = theta.sin_cos();
    (transcendental(s), transcendental(c))
}

/// [`offsets`] in running arithmetic: each sine charged an ulp, the
/// square its rounding, the halving, the doubling and the negation
/// exact.
fn running_offsets<T: Real>(theta: T, (c0, s0): (T, T)) -> (Rounded<T>, Rounded<T>) {
    let (sh, _) = (theta * T::from_f64(0.5)).sin_cos();
    let (s, _) = theta.sin_cos();
    let sq = transcendental(sh).square();
    let two = T::from_f64(2.0);
    let doubled = Rounded {
        value: two * sq.value,
        error: two * sq.error,
    };
    // The stored phase enters exact; its two differences are charged
    // their rounding, which is none at the seam's own vertex.
    let first = doubled - (Rounded::exact(T::one()) - Rounded::exact(c0));
    (
        Rounded {
            value: -first.value,
            error: first.error,
        },
        transcendental(s) - Rounded::exact(s0),
    )
}

/// **The `(lo, hi)` of `p·(cos θ − cos θ₀) + q·(sin θ − sin θ₀)` over
/// `window`**, `(cos θ₀, sin θ₀)` the unit pair `phase`: an arc-held
/// conic's harmonic along a direction, less its anchor's height.
///
/// An end reads through [`offsets`], as the point there does. A crest
/// stands `A − k` over the anchor and a trough `A + k` under it,
/// `A = √(p² + q²)` and `k = p·cos θ₀ + q·sin θ₀` ([`over_anchor`]).
fn offset_extent<T: Real>(window: &ArcWindow<T>, p: T, q: T, phase: (T, T)) -> (T, T) {
    let (t0, t1) = window.span();
    let end = |t: T| {
        let (ku, kv) = offsets(t, phase);
        p * ku + q * kv
    };
    let (r0, r1) = (end(t0), end(t1));
    let (trough, crest) = offset_crests(p, q, phase);
    let hi = window.crest_outside(p, q).select_le_zero(crest, r0.max(r1));
    let lo = window
        .crest_outside(-p, -q)
        .select_le_zero(trough, r0.min(r1));
    (r0.min(r1).min(lo), r0.max(r1).max(hi))
}

/// **The trough and the crest of `p·(cos θ − cos θ₀) + q·(sin θ − sin θ₀)`
/// over the whole turn**, `−(A + k)` and `A − k`, each through
/// [`over_anchor`]: [`offset_extent`]'s and [`Height::extremes`]'.
fn offset_crests<T: Real>(p: T, q: T, (c0, s0): (T, T)) -> (T, T) {
    let amp = (p.powi(2) + q.powi(2)).sqrt();
    let k = p * c0 + q * s0;
    let cross = p * s0 - q * c0;
    (-over_anchor(amp, -k, cross), over_anchor(amp, k, cross))
}

/// The row a harmonic's crest angle records its cut side under.
const CREST_ROW: &str = "conic_harmonic_crest";

/// **[`Height::roots`] about the centre**: `c₀ + p·cos θ + q·sin θ`,
/// `c₀` already less the level, about the crest's angle ([`crest_roots`]).
fn centred_roots<T: Decide>(c0: T, p: T, q: T) -> [T; 2] {
    let swing = (p.powi(2) + q.powi(2)).sqrt();
    let (lo, hi) = (c0 - swing, c0 + swing);
    let two = swing + swing;
    let at_hi = Azimuth::of(q, p).principal_as(CREST_ROW, geom_core::Band::least());
    crest_roots(at_hi, ((T::zero() - lo) / two, hi / two))
}

/// **The angle from an extreme of a first harmonic `c₀ + A·cos ψ` to its
/// roots**, where that extreme has crossed zero by `ratio` of the swing
/// `2A` (`ratio ∈ [0, 1]`): `2·asin(√ratio)`, which reads the extreme to
/// its own relative precision.
pub fn arc_past<T: Real>(ratio: T) -> T {
    T::from_f64(2.0) * ratio.sqrt().asin()
}

/// **A first harmonic's two roots about its crest at `at_hi`**,
/// `[at_hi + ψ, at_hi − ψ]` (past the crest, then before it), `ψ` read
/// from the extreme that has crossed zero the less ([`arc_past`] of its
/// crossing, clamped to `[0, 1]`): `cross_lo` and `cross_hi` how far the
/// trough and the crest have crossed zero over the swing, positive where
/// they have. Where neither crosses, the extreme nearer zero, twice. The
/// one home of the rule for every harmonic solved about its crest.
pub fn crest_roots<T: Real>(at_hi: T, (cross_lo, cross_hi): (T, T)) -> [T; 2] {
    let past = |cross: T| arc_past(cross.max(T::zero()).min(T::one()));
    let from_hi = past(cross_hi);
    let from_lo = T::pi() - past(cross_lo);
    let psi = (cross_hi - cross_lo).select_le_zero(from_hi, from_lo);
    [at_hi + psi, at_hi - psi]
}

/// **[`Height::roots`] about an anchor at the anomaly `phase`**:
/// `h + p·(cos θ − cos θ₀) + q·(sin θ − sin θ₀)`, `h` the anchor's
/// height less the level. In `φ = θ − θ₀` it is
/// `h + P·(cos φ − 1) + Q·sin φ`, and with `t = tan(φ/2)`
/// `(h − 2P)·t² + 2Q·t + h = 0`; its roots are read as `w/(h − 2P)` and
/// `h/w`, `w = −(Q + sign(Q)·√(Q² − (h − 2P)·h))`, which cancel nothing,
/// each turned to `φ = 2·atan2(num, den)` without a division. Where the
/// discriminant is not positive the harmonic does not cross zero (or
/// touches it), and both are the extreme nearer it, read from the
/// extremes' own values.
fn anchored_roots<T: Decide>(h: T, p: T, q: T, (c0, s0): (T, T)) -> [T; 2] {
    let two = T::from_f64(2.0);
    let big_p = p * c0 + q * s0;
    let big_q = q * c0 - p * s0;
    let a2 = h - two * big_p;
    let disc = big_q.powi(2) - a2 * h;
    let root = disc.max(T::zero()).sqrt();
    let w = T::zero() - (big_q + big_q.select_le_zero(T::zero() - root, root));
    // The anchor's anomaly within half a turn of the seam, `0` itself
    // at the seam's own vertex.
    let theta0 = Azimuth::of(s0, c0).near(T::zero());
    // `2·atan(num/den)`, read as one direction so no quotient is formed,
    // within half a turn of the anchor.
    let angle = |num: T, den: T| {
        theta0 + Azimuth::of(two * num * den, den.powi(2) - num.powi(2)).near(T::zero())
    };
    let (r1, r2) = (angle(w, a2), angle(h, w));
    // The falling one first: where `dH/dθ = q·cos θ − p·sin θ` is negative.
    let slope = |t: T| {
        let (s, c) = t.sin_cos();
        q * c - p * s
    };
    let falls = slope(r1);
    let (falling, rising) = (falls.select_le_zero(r1, r2), falls.select_le_zero(r2, r1));
    // Where it does not cross (a touch included), the extreme nearer
    // zero, each read as an angle from the anchor: the crest at
    // `φ = atan2(Q, P)`, the trough half a turn on. A tie in the
    // extremes' magnitudes reads the crest.
    let (trough, crest) = offset_crests(p, q, (c0, s0));
    let (lo, hi) = (h + trough, h + crest);
    let crest_at = theta0 + Azimuth::of(big_q, big_p).near(T::zero());
    let trough_at = theta0 + Azimuth::of(T::zero() - big_q, T::zero() - big_p).near(T::zero());
    let nearer = (hi.abs() - lo.abs()).select_le_zero(crest_at, trough_at);
    [
        disc.select_le_zero(nearer, falling),
        disc.select_le_zero(nearer, rising),
    ]
}

/// **[`Conic::level_at`] from unit coordinates** `(X/a, Y/b)`.
fn centred_level<T: Real>((u, v): (T, T), (a, b): (T, T)) -> (T, (T, T)) {
    let two = T::from_f64(2.0);
    (u.powi(2) + v.powi(2) - T::one(), (two * u / a, two * v / b))
}

/// **[`Conic::level_at`] from an anchor at the anomaly `(c₀, s₀)`**,
/// `(du, dv)` the foreign point's unit offsets from it: `F` formed as
/// `2(c₀·du + s₀·dv) + (du² + dv²) + (c₀² + s₀² − 1)`, the last zero for
/// a stored phase, with no cancellation beside the anchor.
fn anchored_level<T: Real>((du, dv): (T, T), (c0, s0): (T, T), (a, b): (T, T)) -> (T, (T, T)) {
    let two = T::from_f64(2.0);
    let (u, v) = (c0 + du, s0 + dv);
    (
        two * (c0 * du + s0 * dv)
            + (du.powi(2) + dv.powi(2))
            + (c0.powi(2) + s0.powi(2) - T::one()),
        (two * u / a, two * v / b),
    )
}

/// **`A − k`, how far a harmonic's crest stands over its anchor**, for
/// `A = √(p² + q²)`, `k` the anchor's own height in it and `cross` with
/// `cross² = A² − k²`. Never negative in ℝ.
///
/// Where `k ≤ 0` the difference adds. Where `k > 0` it cancels as the
/// anchor nears the crest, losing a rounding of `A`, the semi-axis's
/// scale; there it is read as the ratio `cross² / (A + k)`, the same
/// number as a quotient of sums, whose divisor is at least `A`.
///
/// An enclosure of `k` across zero takes both. Along the conic's own
/// normal `p` and `q` enclose zero, so `A + k` does too and the ratio
/// bounds nothing: the divisor is floored at the least positive number,
/// which keeps the ratio a bounded enclosure, and the reading is held
/// under `A + |k|`, which the number never passes and which is small
/// there.
fn over_anchor<T: Real>(amp: T, k: T, cross: T) -> T {
    let ratio = cross.powi(2) / (amp + k).max(T::from_f64(f64::MIN_POSITIVE));
    k.select_le_zero(amp - k, ratio).min(amp + k.abs())
}

impl<T: Real> Circle<T> {
    /// The circle of `radius` placed by `frame`.
    ///
    /// # Panics
    ///
    /// Where `radius` is not a finite length or is definitely negative
    /// ([`stored_size`]): a door taking a caller's radius decides its
    /// sign before it builds one.
    #[must_use]
    pub fn new(frame: Frame<T>, radius: T) -> Self {
        Self {
            at: frame,
            radius: stored_size(radius, "Circle::new", "radius"),
        }
    }

    /// **This circle with its placement mapped by `map`**: its image
    /// under a map of space that keeps its radius (a motion).
    ///
    /// [`Frame::reversed`] handed here is the half turn about the line
    /// through the frame's origin along the seam. A circle's frame
    /// stands at its centre, so that line is a diameter and the half
    /// turn carries the circle onto itself with `θ ↦ −θ`: the same
    /// circle run back, which is how [`Curve3::reversed`] spells it. An
    /// ellipse's frame may stand on its arc, where that is another
    /// ellipse, so an ellipse is run back by its own door
    /// ([`Ellipse::reversed`]).
    #[must_use]
    pub fn placed(&self, map: impl FnOnce(&Frame<T>) -> Frame<T>) -> Self {
        Self {
            at: map(&self.at),
            radius: self.radius,
        }
    }

    /// **This circle at another scalar**: its placement as `frame`
    /// carries it, its radius through `f`.
    pub fn map_scalar<U: Real>(
        &self,
        frame: impl FnOnce(&Frame<T>) -> Frame<U>,
        f: impl Fn(T) -> U,
    ) -> Circle<U> {
        Circle {
            at: frame(&self.at),
            radius: stored_size(f(self.radius), "Circle::map_scalar", "radius"),
        }
    }

    /// **This circle moved so that its centre stands at `center`.**
    #[must_use]
    pub fn centred_at(&self, center: Point3<T>) -> Self {
        self.placed(|at| at.with_origin(center))
    }

    /// The principal frame at the centre ([`Conic::frame`]).
    #[must_use]
    pub fn frame(&self) -> Frame<T> {
        self.at
    }

    /// The unit plane normal ([`Conic::axis`]).
    #[must_use]
    pub fn axis(&self) -> Vec3<T> {
        self.at.axis()
    }

    /// The unit seam ([`Conic::u_ref`]).
    #[must_use]
    pub fn u_ref(&self) -> Vec3<T> {
        self.at.u_ref()
    }

    /// `axis × u_ref` ([`Conic::v_ref`]).
    #[must_use]
    pub fn v_ref(&self) -> Vec3<T> {
        self.at.v_ref()
    }

    /// The radius.
    #[must_use]
    pub fn radius(&self) -> T {
        self.radius
    }

    /// **The circle's period**: its parameter is an angle
    /// ([`crate::conic_period`]), so every circle repeats. Total, where
    /// [`crate::Curve3::period`], which reads it, is `None` on the kinds
    /// that do not repeat.
    #[must_use]
    pub fn period(&self) -> geom_core::period::Period<T> {
        crate::conic_period()
    }

    fn azimuth(&self, theta: T) -> azimuth::AzimuthFrame<T> {
        azimuth::frame(&self.at, theta)
    }
}

impl<T: Real> Conic<T> for Circle<T> {
    fn frame(&self) -> Frame<T> {
        self.at
    }

    fn height_along(&self, o: Point3<T>, n: Vec3<T>) -> Height<T> {
        let (p, q) = self.along(n);
        Height {
            at: (self.at.origin() - o).dot(n),
            p,
            q,
            held: Held::Centre,
        }
    }

    fn axis(&self) -> Vec3<T> {
        self.at.axis()
    }

    fn u_ref(&self) -> Vec3<T> {
        self.at.u_ref()
    }

    fn v_ref(&self) -> Vec3<T> {
        self.at.v_ref()
    }

    fn semi_axes(&self) -> (T, T) {
        (self.radius, self.radius)
    }

    fn point_at(&self, theta: T) -> Point3<T> {
        self.at.origin() + self.azimuth(theta).radial.0 * self.radius
    }

    fn derivative_at(&self, theta: T) -> Vec3<T> {
        self.azimuth(theta).tangential.0 * self.radius
    }

    fn jet_at(&self, theta: T) -> (Point3<T>, Vec3<T>) {
        let frame = self.azimuth(theta);
        (
            self.at.origin() + frame.radial.0 * self.radius,
            frame.tangential.0 * self.radius,
        )
    }

    fn second_derivative_at(&self, theta: T) -> Vec3<T> {
        let ((s, c), v_ref) = azimuth::basis(&self.at, theta);
        (self.at.u_ref() * (-c) + v_ref * (-s)) * self.radius
    }

    /// `(x/r, y/r)`'s direction, read as `(y·r, x·r)`, as an ellipse
    /// reads its own.
    fn azimuth(&self, p: Point3<T>) -> Azimuth<T> {
        self.at.azimuth(p).scaled(self.radius, self.radius)
    }

    fn speed_lo(&self) -> T {
        self.radius.max(T::zero())
    }

    fn speed_hi(&self) -> T {
        self.radius.abs()
    }

    fn speed_at(&self, _theta: T) -> T {
        self.radius.abs()
    }

    fn running_point(&self, theta: T) -> [Rounded<T>; 3] {
        let (c, u, v) = running_frame(&self.at);
        let r = Rounded::exact(self.radius);
        let (sin, cos) = running_sin_cos(theta);
        core::array::from_fn(|i| c[i] + (u[i] * cos + v[i] * sin) * r)
    }

    fn major_frame(&self) -> MajorFrame<T> {
        MajorFrame {
            major: self.radius,
            minor: self.radius,
            major_dir: self.at.u_ref(),
            phase: T::zero(),
        }
    }
}

/// **A conic's height along a direction from a point**, `(P(θ) − o)·n`
/// ([`Conic::height_along`]): a first harmonic in the conic's parameter,
/// stated from the point the conic stores and posed per placement, which
/// no reader sees. Held at its centre it is `c₀ + p·cos θ + q·sin θ`, `c₀`
/// the centre's height; held on its arc
/// `h + p·(cos θ − cos θ₀) + q·(sin θ − sin θ₀)`, `h` the anchor's height,
/// read through the evaluator's offsets, so beside the anchor every
/// reading below holds at the arc's scale however far the centre lies.
/// Opaque: its `Debug` names the type and no field.
#[derive(Clone, Copy)]
pub struct Height<T: Real> {
    /// The stored point's height.
    at: T,
    /// `(a·(u_ref·n), b·(v_ref·n))`, [`Conic::along`].
    p: T,
    q: T,
    held: Held<T>,
}

impl<T: Real> core::fmt::Debug for Height<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Height").finish_non_exhaustive()
    }
}

impl<T: Real> Height<T> {
    /// The height at `theta`.
    #[must_use]
    pub fn at(&self, theta: T) -> T {
        match self.held {
            Held::Centre => {
                let (s, c) = theta.sin_cos();
                self.at + self.p * c + self.q * s
            }
            Held::Arc { phase } => {
                let (ku, kv) = offsets(theta, phase);
                self.at + (self.p * ku + self.q * kv)
            }
        }
    }

    /// **The height at `theta` with a first-order running bound on its
    /// rounding** ([`Rounded`]): [`Self::at`] operation for operation,
    /// its value that one bit for bit, the stored point's height and the
    /// harmonic's parts taken as exact, each sine or cosine charged an ulp
    /// and every product and sum its rounding.
    #[must_use]
    pub fn running_at(&self, theta: T) -> Rounded<T> {
        let (at, p, q) = (
            Rounded::exact(self.at),
            Rounded::exact(self.p),
            Rounded::exact(self.q),
        );
        match self.held {
            Held::Centre => {
                let (s, c) = running_sin_cos(theta);
                at + p * c + q * s
            }
            Held::Arc { phase } => {
                let (ku, kv) = running_offsets(theta, phase);
                at + (p * ku + q * kv)
            }
        }
    }

    /// `dH/dθ` at `theta`, `q·cos θ − p·sin θ`: no position enters it.
    #[must_use]
    pub fn slope(&self, theta: T) -> T {
        let (s, c) = theta.sin_cos();
        self.q * c - self.p * s
    }

    /// The swing `√(p² + q²)`: half the height's range over a turn.
    #[must_use]
    pub fn swing(&self) -> T {
        (self.p.powi(2) + self.q.powi(2)).sqrt()
    }

    /// **The least and the greatest height over the whole conic**: about
    /// the centre its height less and plus the swing, about the anchor its
    /// height plus the trough's and the crest's offsets, each formed with
    /// no cancellation.
    #[must_use]
    pub fn extremes(&self) -> (T, T) {
        match self.held {
            Held::Centre => {
                let swing = self.swing();
                (self.at - swing, self.at + swing)
            }
            Held::Arc { phase } => {
                let (trough, crest) = offset_crests(self.p, self.q, phase);
                (self.at + trough, self.at + crest)
            }
        }
    }

    /// **The `(lo, hi)` of the height over `window`**: an end reads as
    /// [`Self::at`] does, and a crest or trough inside the window as
    /// [`Self::extremes`] does.
    #[must_use]
    pub fn extent(&self, window: &ArcWindow<T>) -> (T, T) {
        match self.held {
            Held::Centre => window.extent(self.at, self.p, self.q),
            Held::Arc { phase } => {
                let (lo, hi) = offset_extent(window, self.p, self.q, phase);
                (self.at + lo, self.at + hi)
            }
        }
    }
}

impl<T: Decide> Height<T> {
    /// The angle of the height's crest, in the angle window
    /// ([`Azimuth::principal_as`]).
    #[must_use]
    pub fn crest(&self) -> T {
        Azimuth::of(self.q, self.p).principal_as(CREST_ROW, geom_core::Band::least())
    }

    /// **Where the height equals `level`**: `[falling, rising]`, the root
    /// past the crest and the one before it, posed per placement. About
    /// the centre each is read from the extreme that has crossed `level`
    /// the less, to that extreme's own relative precision, the crest's
    /// angle plus and less the half-chord. About the anchor the roots
    /// solve the evaluator's own form in `t = tan(φ/2)`, `φ = θ − θ₀` the
    /// angle from the anchor, by the pair that cancels nothing, so a root
    /// beside the anchor reads to its own relative precision however far
    /// the centre lies. Where the height does not reach `level`, both are
    /// its extreme nearer it.
    #[must_use]
    pub fn roots(&self, level: T) -> [T; 2] {
        let h = self.at - level;
        match self.held {
            Held::Centre => centred_roots(h, self.p, self.q),
            Held::Arc { phase } => anchored_roots(h, self.p, self.q, phase),
        }
    }
}

/// **An ellipse's stored placement as its walk reads it**
/// ([`Ellipse::stored`]).
pub(super) struct Stored<T: Real> {
    /// What the stored point is: [`CurveDatum::Center`] or
    /// [`CurveDatum::Anchor`].
    ///
    /// [`CurveDatum::Center`]: super::CurveDatum::Center
    /// [`CurveDatum::Anchor`]: super::CurveDatum::Anchor
    pub(super) datum: super::CurveDatum,
    /// The stored point.
    pub(super) at: Point3<T>,
    /// The stored axis.
    pub(super) axis: Vec3<T>,
    /// The stored seam.
    pub(super) u_ref: Vec3<T>,
    /// Beside an anchor, where on the ellipse it sits: the in-plane unit
    /// direction `(cos θ₀, sin θ₀, 0)`.
    pub(super) phase: Option<Vec3<T>>,
}

impl<T: Real> Held<T> {
    /// This placement's kind with an arc's phase read through `f`.
    fn with_phase<U: Real>(self, f: impl FnOnce((T, T)) -> (U, U)) -> Held<U> {
        match self {
            Held::Centre => Held::Centre,
            Held::Arc { phase } => Held::Arc { phase: f(phase) },
        }
    }
}

impl<T: Real> Ellipse<T> {
    /// **The ellipse's period**: its parameter is the eccentric anomaly,
    /// an angle ([`crate::conic_period`]), so every ellipse repeats.
    /// Total, as [`Circle::period`] is.
    #[must_use]
    pub fn period(&self) -> geom_core::period::Period<T> {
        crate::conic_period()
    }

    /// The ellipse placed by `frame`, at its centre, with `semi_u` along
    /// its `u_ref` and `semi_v` along its `v_ref`.
    ///
    /// # Panics
    ///
    /// Where either semi-axis is not a finite length or is definitely
    /// negative ([`stored_size`]): a door taking a caller's semi-axes
    /// ([`Curve3::ellipse`]) decides their sign before it builds one.
    #[must_use]
    pub fn new(frame: Frame<T>, semi_u: T, semi_v: T) -> Self {
        Self {
            at: frame,
            held: Held::Centre,
            semi_u: stored_size(semi_u, "Ellipse::new", "semi_u"),
            semi_v: stored_size(semi_v, "Ellipse::new", "semi_v"),
        }
    }

    /// **The ellipse held on its arc at its vertex `anchor`** ([module
    /// docs](self), "Where a conic is held"): in the plane of normal
    /// `axis`, with `semi_u` along `toward` (the principal direction
    /// from the centre to the anchor, squared to the axis with no
    /// decision) and `semi_v` across it. The anchor is the datum it
    /// stores, and its point at `θ = 0` until its seam is turned; the
    /// centre, `semi_u` back from it, is derived. Unchecked;
    /// [`Curve3::ellipse_anchored`] is the deciding door.
    #[must_use]
    pub fn anchored(
        anchor: Point3<T>,
        axis: Vec3<T>,
        toward: Vec3<T>,
        semi_u: T,
        semi_v: T,
    ) -> Self {
        Self {
            at: Frame::new(anchor, axis, toward),
            held: Held::Arc {
                phase: (T::one(), T::zero()),
            },
            semi_u: stored_size(semi_u, "Ellipse::anchored", "semi_u"),
            semi_v: stored_size(semi_v, "Ellipse::anchored", "semi_v"),
        }
    }

    /// **This ellipse's image under an isometry of space that keeps its
    /// parameter**, `map` that isometry on a frame (a motion's image of
    /// the frame, or under a reflection the frame the image is read by on
    /// the same parameter): the image's point at each parameter is the
    /// isometry's image of this ellipse's point there, so an anchor stays
    /// at its own anomaly and the image is held as this ellipse is.
    ///
    /// A re-parameterization of the same locus is not such a map:
    /// running the ellipse back is [`Self::reversed`], and
    /// [`Frame::reversed`] handed here is the half turn about the line
    /// through the stored point along the seam, which is the same locus
    /// only where that line passes through the centre.
    #[must_use]
    pub fn mapped(&self, map: impl FnOnce(&Frame<T>) -> Frame<T>) -> Self {
        Self {
            at: map(&self.at),
            ..*self
        }
    }

    /// **This ellipse moved by `by`**: a translation of its one
    /// placement.
    #[must_use]
    pub fn translated(&self, by: Vec3<T>) -> Self {
        self.mapped(|at| at.translated(by))
    }

    /// **The same locus run back** ([`Curve3::reversed`]): the axis
    /// flipped and the seam kept, so `θ ↦ −θ`, and an anchor's anomaly
    /// with it: the anchor stays where it is.
    #[must_use]
    pub fn reversed(&self) -> Self {
        Self {
            at: self.at.reversed(),
            held: self.held.with_phase(|(c0, s0)| (c0, -s0)),
            ..*self
        }
    }

    /// **This ellipse at another scalar**: its placement as `frame`
    /// carries it, its semi-axes and an anchor's phase through `f`.
    pub fn map_scalar<U: Real>(
        &self,
        frame: impl FnOnce(&Frame<T>) -> Frame<U>,
        f: impl Fn(T) -> U,
    ) -> Ellipse<U> {
        Ellipse {
            at: frame(&self.at),
            held: self.held.with_phase(|(c0, s0)| (f(c0), f(s0))),
            semi_u: stored_size(f(self.semi_u), "Ellipse::map_scalar", "semi_u"),
            semi_v: stored_size(f(self.semi_v), "Ellipse::map_scalar", "semi_v"),
        }
    }

    /// **The stored placement, datum by datum**, for the walk of the
    /// stored data alone ([`Curve3::data`]).
    pub(super) fn stored(&self) -> Stored<T> {
        let (datum, phase) = match self.held {
            Held::Centre => (super::CurveDatum::Center, None),
            Held::Arc { phase: (c0, s0) } => (
                super::CurveDatum::Anchor,
                Some(Vec3::new(c0, s0, T::zero())),
            ),
        };
        Stored {
            datum,
            at: self.at.origin(),
            axis: self.at.axis(),
            u_ref: self.at.u_ref(),
            phase,
        }
    }

    /// Whether `other` is held as this ellipse is, on a placement `same`
    /// reads as this one's and, on its arc, at a phase `same_scalar`
    /// reads as this one's ([`Curve3::same_content`]).
    pub(super) fn same_placement(
        &self,
        other: &Self,
        same: impl Fn(&Frame<T>, &Frame<T>) -> bool,
        same_scalar: impl Fn(T, T) -> bool,
    ) -> bool {
        same(&self.at, &other.at)
            && match (self.held, other.held) {
                (Held::Centre, Held::Centre) => true,
                (Held::Arc { phase: p }, Held::Arc { phase: q }) => {
                    same_scalar(p.0, q.0) && same_scalar(p.1, q.1)
                }
                (Held::Centre, Held::Arc { .. }) | (Held::Arc { .. }, Held::Centre) => false,
            }
    }

    /// **This ellipse moved so that its centre stands at `center`**: a
    /// translation of its one placement, exact on an ellipse held at its
    /// centre.
    #[must_use]
    pub fn centred_at(&self, center: Point3<T>) -> Self {
        self.mapped(|at| match self.held {
            Held::Centre => at.with_origin(center),
            Held::Arc { .. } => at.translated(center - Conic::center(self)),
        })
    }

    /// `at_centre`, a frame at this ellipse's centre, stood where this
    /// ellipse's placement stands: itself, or moved to the anchor.
    fn stood(&self, at_centre: Frame<T>) -> Frame<T> {
        match self.held {
            Held::Centre => at_centre,
            Held::Arc { .. } => at_centre.with_origin(self.at.origin()),
        }
    }

    /// The principal frame at the centre ([`Conic::frame`]).
    #[must_use]
    pub fn frame(&self) -> Frame<T> {
        Conic::frame(self)
    }

    /// The unit plane normal ([`Conic::axis`]).
    #[must_use]
    pub fn axis(&self) -> Vec3<T> {
        self.at.axis()
    }

    /// The unit seam ([`Conic::u_ref`]).
    #[must_use]
    pub fn u_ref(&self) -> Vec3<T> {
        self.at.u_ref()
    }

    /// `axis × u_ref` ([`Conic::v_ref`]).
    #[must_use]
    pub fn v_ref(&self) -> Vec3<T> {
        self.at.v_ref()
    }

    /// `(semi_u, semi_v)` in stored order.
    #[must_use]
    pub fn semi_axes(&self) -> (T, T) {
        (self.semi_u, self.semi_v)
    }

    /// The semi-axis along `u_ref`: one stored size, no region.
    #[must_use]
    pub fn semi_u(&self) -> T {
        self.semi_u
    }

    /// The semi-axis along `axis × u_ref`: one stored size, no region.
    #[must_use]
    pub fn semi_v(&self) -> T {
        self.semi_v
    }

    /// **The semi-axes by size**: the larger and the smaller, in that
    /// order. The one reading of "the major semi-axis" off the stored
    /// pair, whose order is free.
    #[must_use]
    pub fn semi_axes_by_size(&self) -> (T, T) {
        (self.semi_u.max(self.semi_v), self.semi_u.min(self.semi_v))
    }

    /// **The ellipse read as a plane section of a surface of revolution
    /// about `axis`** (a cylinder's or a cone's): its principal frame with
    /// the direction of the larger component along `axis` first. Such a
    /// section's minor axis is perpendicular to the axis of revolution, so
    /// in truth its major direction is the one `axis` has a component
    /// along, and the reading holds at a near-circle, where the semi-axes
    /// round past each other and [`Conic::major_frame`] has nothing to
    /// read. A tie (a plane across the axis, whose section is a circle)
    /// keeps the seam's own axis.
    #[must_use]
    pub fn section_frame(&self, axis: Vec3<T>) -> MajorFrame<T> {
        let f = &self.at;
        self.frame_turned_where(f.v_ref().dot(axis).abs() - f.u_ref().dot(axis).abs())
    }

    /// The frame as stored where `turn ≤ 0`, else
    /// [quarter-turned](Self::quarter_turned_where), at its phase.
    fn frame_turned_where(&self, turn: T) -> MajorFrame<T> {
        let e = self.quarter_turned_where(turn);
        MajorFrame {
            major: e.semi_u,
            minor: e.semi_v,
            major_dir: e.at.u_ref(),
            phase: turn.select_le_zero(T::zero(), T::pi() * T::from_f64(0.5)),
        }
    }

    /// **The same ellipse seamed a quarter turn on where `turn > 0`**, as
    /// stored where `turn ≤ 0`, read without a branch. `θ = s + π/2` reads
    /// `center + v·(semi_v·cos s) − u_ref·(semi_u·sin s)` with
    /// `v = axis × u_ref`: the seam turns to `v`, renormalized so the turn
    /// adds no length to the frame, and the semi-axes swap. The turned
    /// frame keeps its axis bit for bit. At an `Inexact` witness kind
    /// (`f64`, `Probe` and the duals and forms over them) its seam is
    /// `v_ref().normalize()` as given, not re-squared to the axis
    /// ([`Frame::quarter_turned_renormalized`], which decides nothing):
    /// the merge base's bits, a rounding apart from the frame's own `v`
    /// (`a_quarter_turned_seam_is_the_base_formula_bit_for_bit`). At
    /// an `Exact` kind (`Interval` and the duals and forms over it) it
    /// takes the exact turn ([`Frame::quarter_turned`], a permutation of
    /// the frame's axes), which encloses the same turned rotation and adds
    /// no width, where re-squaring there would widen every entry by the
    /// squaring's own.
    ///
    /// An ellipse held on its arc keeps its anchor: the seam turns under
    /// it, and its phase turns back by the same quarter.
    ///
    /// # Panics
    ///
    /// On a poison `turn`, which hulls the two sizes to poison: the
    /// sizes are read as the constructors read them ([`stored_size`]),
    /// so it is refused here rather than stored.
    #[must_use]
    pub fn quarter_turned_where(&self, turn: T) -> Self {
        // The turn is read on the frame at the centre, which an ellipse
        // held there stores, so its bits are the stored frame's.
        let f = Conic::frame(self);
        let turned = if T::WITNESS == geom_core::Witness::Inexact {
            f.quarter_turned_renormalized()
        } else {
            f.quarter_turned()
        };
        let door = "Ellipse::quarter_turned_where";
        Self {
            at: self.stood(Frame::select_le_zero(turn, f, turned)),
            // `θ = s + π/2`: the anchor's anomaly from the turned seam
            // is `θ₀ − π/2`, an exact permutation of the stored pair.
            held: self
                .held
                .with_phase(|(c0, s0)| (turn.select_le_zero(c0, s0), turn.select_le_zero(s0, -c0))),
            semi_u: stored_size(
                turn.select_le_zero(self.semi_u, self.semi_v),
                door,
                "semi_u",
            ),
            semi_v: stored_size(
                turn.select_le_zero(self.semi_v, self.semi_u),
                door,
                "semi_v",
            ),
        }
    }

    /// **The same ellipse seamed at the principal vertex nearest the
    /// eccentric anomaly `theta`.** See [`Self::seamed_toward`].
    #[must_use]
    pub fn seamed_near(&self, theta: T) -> Self {
        let (sin, cos) = theta.sin_cos();
        self.seamed_toward(cos, sin)
    }

    /// **The same ellipse seamed at the principal vertex nearest the
    /// direction `(cos, sin)`** in its eccentric anomaly. Of the vertices
    /// at `0`, `π/2`, `π` and `3π/2`, the one on the axis whose component
    /// dominates, a tie keeping the seam's own axis; on that axis, the
    /// vertex the component's sign points to, a zero pointing forward.
    /// Near the new seam the parameter is near zero. An ellipse held on
    /// its arc keeps its anchor through the turn ([module docs](self),
    /// "Where a conic is held").
    #[must_use]
    pub fn seamed_toward(&self, cos: T, sin: T) -> Self {
        let turn = sin.abs() - cos.abs();
        // The direction's cosine in the turned frame, `θ − π/2`, is `sin`.
        let back = T::zero() - turn.select_le_zero(cos, sin);
        let e = self.quarter_turned_where(turn);
        let f = Conic::frame(&e);
        Self {
            at: e.stood(Frame::select_le_zero(back, f, f.half_turned())),
            // A half turn of the seam: the anchor's anomaly less `π`.
            held: e.held.with_phase(|(c0, s0)| {
                (back.select_le_zero(c0, -c0), back.select_le_zero(s0, -s0))
            }),
            ..e
        }
    }

    fn basis(&self, theta: T) -> ((T, T), Vec3<T>) {
        azimuth::basis(&self.at, theta)
    }
}

impl<T: Real> Conic<T> for Ellipse<T> {
    fn frame(&self) -> Frame<T> {
        match self.held {
            Held::Centre => self.at,
            // The centre lies `u·(a·cos θ₀) + v·(b·sin θ₀)` back from the
            // anchor: `u·a` at the seam's own vertex, to the bit.
            Held::Arc { phase: (c0, s0) } => self.at.with_origin(
                self.at.origin()
                    - self.at.u_ref() * (self.semi_u * c0)
                    - self.at.v_ref() * (self.semi_v * s0),
            ),
        }
    }

    fn axis(&self) -> Vec3<T> {
        self.at.axis()
    }

    fn u_ref(&self) -> Vec3<T> {
        self.at.u_ref()
    }

    fn v_ref(&self) -> Vec3<T> {
        self.at.v_ref()
    }

    fn semi_axes(&self) -> (T, T) {
        Ellipse::semi_axes(self)
    }

    fn height_along(&self, o: Point3<T>, n: Vec3<T>) -> Height<T> {
        let (p, q) = self.along(n);
        Height {
            at: (self.at.origin() - o).dot(n),
            p,
            q,
            held: self.held,
        }
    }

    fn height_of(&self, p: Point3<T>) -> T {
        (p - self.at.origin()).dot(self.at.axis())
    }

    fn unit_in_plane(&self, p: Point3<T>) -> (T, T) {
        let (du, dv) = self.unit_of(p - self.at.origin());
        match self.held {
            Held::Centre => (du, dv),
            Held::Arc { phase: (c0, s0) } => (c0 + du, s0 + dv),
        }
    }

    fn level_at(&self, p: Point3<T>) -> (T, (T, T)) {
        let (du, dv) = self.unit_of(p - self.at.origin());
        match self.held {
            Held::Centre => centred_level((du, dv), self.semi_axes()),
            Held::Arc { phase } => anchored_level((du, dv), phase, self.semi_axes()),
        }
    }

    fn cover_over(&self, window: ParamWindow<T>) -> Option<Cover<T>> {
        match self.held {
            Held::Centre => Cover::arc(&self.at, self.semi_axes(), window),
            Held::Arc { .. } => Cover::arc_of(*self, window),
        }
    }

    fn point_at(&self, theta: T) -> Point3<T> {
        match self.held {
            Held::Centre => {
                let ((s, c), v_ref) = self.basis(theta);
                self.at.origin() + (self.at.u_ref() * (self.semi_u * c) + v_ref * (self.semi_v * s))
            }
            Held::Arc { phase } => {
                let (ku, kv) = offsets(theta, phase);
                let (u, v) = (self.at.u_ref(), self.at.v_ref());
                self.at.origin() + (u * (self.semi_u * ku) + v * (self.semi_v * kv))
            }
        }
    }

    fn derivative_at(&self, theta: T) -> Vec3<T> {
        let ((s, c), v_ref) = self.basis(theta);
        self.at.u_ref() * (-(self.semi_u * s)) + v_ref * (self.semi_v * c)
    }

    fn jet_at(&self, theta: T) -> (Point3<T>, Vec3<T>) {
        let ((s, c), v_ref) = self.basis(theta);
        let u_ref = self.at.u_ref();
        (
            self.point_at(theta),
            u_ref * (-(self.semi_u * s)) + v_ref * (self.semi_v * c),
        )
    }

    fn second_derivative_at(&self, theta: T) -> Vec3<T> {
        let ((s, c), v_ref) = self.basis(theta);
        self.at.u_ref() * (-(self.semi_u * c)) + v_ref * (-(self.semi_v * s))
    }

    /// `(x/a, y/b)`'s direction, read as `(y·a, x·b)`: the same
    /// direction scaled by `a·b`, with no division. An ellipse held on
    /// its arc reads `p` from its anchor, `x = a·cos θ₀ + (p − A)·u_ref`
    /// and `y = b·sin θ₀ + (p − A)·v_ref`, so near an anchor at the
    /// seam's own vertex the angle resolves at the arc's scale however
    /// far the derived centre lies.
    fn azimuth(&self, p: Point3<T>) -> Azimuth<T> {
        match self.held {
            Held::Centre => self.at.azimuth(p),
            Held::Arc { phase: (c0, s0) } => {
                let w = p - self.at.origin();
                // `y − (0 − b·sin θ₀)`: the anchor's own coordinates
                // added back, `y` itself (its zero's sign too) at the
                // seam's own vertex.
                Azimuth::of(
                    w.dot(self.at.v_ref()) - (T::zero() - self.semi_v * s0),
                    self.semi_u * c0 + w.dot(self.at.u_ref()),
                )
            }
        }
        .scaled(self.semi_u, self.semi_v)
    }

    fn speed_lo(&self) -> T {
        self.semi_axes_by_size().1.max(T::zero())
    }

    fn speed_hi(&self) -> T {
        let (larger, smaller) = self.semi_axes_by_size();
        larger.max(-smaller)
    }

    fn speed_at(&self, theta: T) -> T {
        let (sin, cos) = theta.sin_cos();
        ((self.semi_u * sin).powi(2) + (self.semi_v * cos).powi(2))
            .sqrt()
            .max(self.speed_lo())
            .min(self.speed_hi())
    }

    fn major_frame(&self) -> MajorFrame<T> {
        self.frame_turned_where(self.semi_v - self.semi_u)
    }

    fn running_point(&self, theta: T) -> [Rounded<T>; 3] {
        let (at, u, v) = running_frame(&self.at);
        let (a, b) = (Rounded::exact(self.semi_u), Rounded::exact(self.semi_v));
        match self.held {
            Held::Centre => {
                let (sin, cos) = running_sin_cos(theta);
                let (ac, bs) = (a * cos, b * sin);
                core::array::from_fn(|i| at[i] + (u[i] * ac + v[i] * bs))
            }
            Held::Arc { phase } => {
                let (ku, kv) = running_offsets(theta, phase);
                let (ku, kv) = (a * ku, b * kv);
                core::array::from_fn(|i| at[i] + (u[i] * ku + v[i] * kv))
            }
        }
    }
}

impl<T: Real> Curve3<T> {
    /// The circle of `radius` at `center` with normal `axis` and seam
    /// `u_ref`.
    #[must_use]
    pub fn circle(center: Point3<T>, axis: Vec3<T>, radius: T, u_ref: Vec3<T>) -> Self {
        Self::Circle(Circle::new(Frame::new(center, axis, u_ref), radius))
    }

    /// **The carrier read as a conic**: a circle or an ellipse as the
    /// readings they share; `None` for every other kind.
    #[must_use]
    pub fn conic(&self) -> Option<&dyn Conic<T>> {
        match self {
            Self::Circle(c) => Some(c),
            Self::Ellipse(e) => Some(e),
            Self::Line(_) | Self::Spiric { .. } | Self::Nurbs(_) | Self::Periodic(_) => None,
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::float_cmp)]
mod tests {

    use geom_core::{Point3, Real, Vec3};

    use crate::Curve3;

    fn carriers() -> [Curve3<f64>; 3] {
        let (c, n) = (Point3::new(0.5, -1.0, 2.0), Vec3::new(2.0, 2.0, 1.0) / 3.0);
        let u = Vec3::new(1.0, -2.0, 2.0) / 3.0;
        [
            Curve3::circle(c, n, 0.7, u),
            Curve3::Ellipse(super::Ellipse::new(super::Frame::new(c, n, u), 0.9, 0.4)),
            Curve3::Ellipse(super::Ellipse::new(super::Frame::new(c, n, u), 0.4, 0.9)),
        ]
    }

    /// The D9 evaluators written out, independent of the trait: a
    /// circle's `c + (u·cos + v·sin)·r` and its derivatives, an ellipse's
    /// `c + (u·(a·cos) + v·(b·sin))` and its, `v = axis × u`. The oracle
    /// for [`the_conic_readings_are_the_carriers_own`].
    fn written_out(carrier: &Curve3<f64>, t: f64) -> [Vec3<f64>; 3] {
        let (s, c) = Real::sin_cos(t);
        let o = Point3::origin();
        match carrier {
            Curve3::Circle(k) => {
                let (u, r) = (k.frame().u_ref(), k.radius());
                let v = k.frame().axis().cross(u);
                [
                    (k.frame().origin() + (u * c + v * s) * r) - o,
                    (u * (-s) + v * c) * r,
                    (u * (-c) + v * (-s)) * r,
                ]
            }
            Curve3::Ellipse(k) => {
                let (u, (a, b)) = (k.frame().u_ref(), k.semi_axes());
                let v = k.frame().axis().cross(u);
                [
                    (k.frame().origin() + (u * (a * c) + v * (b * s))) - o,
                    u * (-(a * s)) + v * (b * c),
                    u * (-(a * c)) + v * (-(b * s)),
                ]
            }
            _ => unreachable!(),
        }
    }

    /// The stored point and the phase of an ellipse held on its arc.
    fn anchor_of(e: &super::Ellipse<f64>) -> Option<(Point3<f64>, (f64, f64))> {
        match e.held {
            super::Held::Centre => None,
            super::Held::Arc { phase } => Some((e.at.origin(), phase)),
        }
    }

    /// **The arc-held chain's bound holds each charge it names.** At the
    /// seam's own vertex the first offset is `−2·sin²(θ/2)` and its bound
    /// is three terms: the half-angle sine's ulp carried through the
    /// square (`8u·sh²`), the square's own rounding (`2u·sh²` once
    /// doubled), and the charge of the difference with `1 − cos θ₀`
    /// (`u·2sh²`; the difference is exact there, and the charge is kept).
    /// The second offset's is the sine's ulp and its difference's charge.
    /// `mul_add` reads the square's rounding exactly, and the charge held
    /// for it covers that reading. Red where the square's rounding is
    /// uncharged, where either sine's ulp is, and where a difference's
    /// charge is dropped.
    #[test]
    fn the_arc_held_offsets_bound_holds_each_charge_it_names() {
        let u = geom_core::UNIT_ROUNDOFF;
        let mut inexact = 0;
        for i in 1..=600 {
            let theta = -3.1 + 0.0103 * f64::from(i);
            let (first, second) = super::running_offsets(theta, (1.0, 0.0));
            let (sh, s) = ((0.5 * theta).sin(), theta.sin());
            let sines_share = 2.0 * (2.0 * sh.abs() * (2.0 * u * sh.abs()));
            let squares = 2.0 * (u * sh * sh);
            let differences = u * (2.0 * sh * sh);
            assert!(
                first.error >= (sines_share + squares + differences) * (1.0 - 8.0 * u),
                "θ {theta}: the first offset's bound {:e} holds the sine's share \
                 {sines_share:e}, the square's {squares:e} and the difference's {differences:e}",
                first.error
            );
            let rounding = sh.mul_add(sh, -(sh * sh)).abs();
            assert!(
                2.0 * rounding <= squares,
                "θ {theta}: the square's charge covers its rounding"
            );
            assert!(
                second.error >= (2.0 * u * s.abs() + u * s.abs()) * (1.0 - 8.0 * u),
                "θ {theta}: the second offset's bound {:e} holds the sine's ulp and its \
                 difference's charge",
                second.error
            );
            inexact += usize::from(rounding > 0.0);
        }
        assert!(inexact > 500, "the square rounds at {inexact} of 600");
    }

    /// **An ellipse held on its arc reads from its anchor.** As minted
    /// its point at `θ = 0` is the anchor bit for bit, its centre is the
    /// anchor less the semi-axis along the seam, and round the turn it
    /// is the ellipse of that centre to the centre's own rounding. Its
    /// derivatives are that centre-held ellipse's bit for bit, a point's
    /// parameter reads back, and the anchor rides a reversal, a
    /// translation and a re-seam, whichever way the seam turns
    /// (`a_re_seam_moves_parameter_zero_and_never_the_anchor`). An
    /// ellipse held at its centre has no anchor.
    #[test]
    fn an_arc_held_ellipse_reads_from_its_anchor() {
        use super::{Conic, Ellipse};
        let bits = |v: Vec3<f64>| [v.x, v.y, v.z].map(f64::to_bits);
        let o = Point3::origin();
        let (at, n) = (Point3::new(3.5, -1.0, 2.0), Vec3::new(2.0, 2.0, 1.0));
        let toward = Vec3::new(1.0, -2.0, 2.0);
        for far in [0.7, 30.0, 1500.0, 1e6] {
            for e in [
                Ellipse::anchored(at, n, toward, far, far * 0.01),
                Ellipse::anchored(at, n, toward, far, far),
            ] {
                let carrier = Curve3::Ellipse(e);
                let k = carrier.conic().unwrap();
                assert_eq!(anchor_of(&e).map(|(p, _)| bits(p - o)), Some(bits(at - o)));
                assert_eq!(bits(k.point_at(0.0) - o), bits(at - o), "{carrier:?}");
                assert_eq!(bits(carrier.eval(0.0) - o), bits(at - o), "{carrier:?}");
                let (a, b) = k.semi_axes();
                let f = k.frame();
                assert_eq!(bits(k.center() - o), bits((at - f.u_ref() * a) - o));
                // The same ellipse held at that centre holds the frame it
                // was given, bit for bit, and has no anchor.
                let centred = Ellipse::new(f, a, b);
                let c: &dyn Conic<f64> = &centred;
                assert!(anchor_of(&centred).is_none());
                let held = c.frame();
                assert_eq!(bits(held.origin() - o), bits(f.origin() - o));
                assert_eq!(bits(held.axis()), bits(f.axis()));
                assert_eq!(bits(held.u_ref()), bits(f.u_ref()));
                assert!(carrier.same_content(&carrier.clone()));
                let scale = (f.origin() - o).norm() + 2.0 * a;
                for t in [-3.0, -1.2, -2e-3, 1e-7, 0.4, 1.55, 2.9] {
                    let gap = (k.point_at(t) - c.point_at(t)).norm();
                    assert!(
                        gap <= 8.0 * f64::EPSILON * scale,
                        "{carrier:?} at {t}: {gap:e} off its centre's ellipse"
                    );
                    assert_eq!(bits(k.derivative_at(t)), bits(c.derivative_at(t)));
                    assert_eq!(
                        bits(k.second_derivative_at(t)),
                        bits(c.second_derivative_at(t))
                    );
                    let (jp, jd) = k.jet_at(t);
                    assert_eq!(bits(jp - o), bits(k.point_at(t) - o));
                    assert_eq!(bits(jd), bits(k.derivative_at(t)));
                    // The parameter reads back at the arc's own scale,
                    // whatever the semi-axis: to the rounding of the
                    // point's offset from the anchor, over the smaller
                    // semi-axis it is divided by.
                    let back = k.param_in(k.point_at(t), (t - 0.3, t + 0.3));
                    assert!(
                        (back - t).abs() <= 8.0 * f64::EPSILON * (1.0 + t.abs()) * (a / b),
                        "{carrier:?}: {t} reads back as {back}"
                    );
                    // Run back, the point at `−t` is this one's at `t`.
                    let reversed = carrier.reversed().unwrap();
                    assert_eq!(
                        bits(reversed.eval(-t) - o),
                        bits(carrier.eval(t) - o),
                        "{carrier:?} reversed at {t}"
                    );
                }
                let by = Vec3::new(0.25, -4.0, 1.5);
                let Curve3::Ellipse(moved) = carrier.translated(by).unwrap() else {
                    unreachable!()
                };
                assert_eq!(
                    anchor_of(&moved).map(|(p, _)| bits(p - o)),
                    Some(bits((at + by) - o)),
                    "{carrier:?}: a translation carries the anchor"
                );
                for seam in [0.3, 1.4, 2.9, -1.5] {
                    assert_eq!(
                        anchor_of(&e.seamed_near(seam)).map(|(p, _)| bits(p - o)),
                        Some(bits(at - o)),
                        "a re-seam near {seam} keeps the anchor"
                    );
                }
                assert!(anchor_of(&centred.seamed_near(1.4)).is_none());
            }
        }
    }

    /// **An arc-held ellipse moved to a centre is that ellipse
    /// translated whole.** [`Ellipse::centred_at`] on an ellipse held on
    /// its arc, as minted and re-seamed: the centre stands at the point
    /// asked for, the anchor has moved by the same step, the phase is
    /// kept, and the point at every parameter is the original's moved by
    /// that step. Held at its centre, the origin is the point asked for
    /// to the bit. Red where the arc-held arm sets the placement's origin
    /// to the centre asked for, which puts the anchor there.
    #[test]
    fn an_arc_held_ellipse_centred_elsewhere_is_translated_whole() {
        use super::{Conic, Ellipse};
        let o = Point3::origin();
        let (at, n) = (Point3::new(3.5, -1.0, 2.0), Vec3::new(2.0, 2.0, 1.0));
        let toward = Vec3::new(1.0, -2.0, 2.0);
        let to = Point3::new(-7.25, 0.5, 11.0);
        for (a, b) in [(0.7, 0.3), (30.0, 41.0), (1500.0, 15.0)] {
            let minted = Ellipse::anchored(at, n, toward, a, b);
            for e in [minted, minted.seamed_near(1.4), minted.seamed_near(2.9)] {
                let step = to - e.center();
                let slack =
                    64.0 * f64::EPSILON * ((to - o).norm() + (at - o).norm() + 2.0 * a.max(b));
                let moved = e.centred_at(to);
                assert!(
                    moved.center().distance(to) <= slack,
                    "({a}, {b}): the centre"
                );
                let ((anchor, phase), (was, was_phase)) =
                    (anchor_of(&moved).unwrap(), anchor_of(&e).unwrap());
                assert_eq!(phase, was_phase);
                assert!(
                    anchor.distance(was + step) <= slack,
                    "({a}, {b}): the anchor"
                );
                for t in [-3.0, -1.2, 0.0, 0.4, 1.55, 2.9] {
                    assert!(
                        moved.point_at(t).distance(e.point_at(t) + step) <= slack,
                        "({a}, {b}) at {t}"
                    );
                }
            }
            let centred = Ellipse::new(minted.frame(), a, b).centred_at(to);
            assert!(anchor_of(&centred).is_none());
            let got = centred.frame().origin();
            assert_eq!(
                [got.x, got.y, got.z].map(f64::to_bits),
                [to.x, to.y, to.z].map(f64::to_bits)
            );
        }
    }

    /// **Two ellipses that differ only in how they are held are two
    /// carriers, and the walk says how.** On one stored frame and one
    /// pair of semi-axes: held at the frame's origin as the centre, held
    /// there on the arc, and held there on the arc at another phase, no
    /// two are the same content ([`Curve3::same_content`]). The walk of
    /// the stored data ([`Curve3::data`]) names the stored point
    /// `Center` or `Anchor`, and beside an anchor its phase, the pair as
    /// stored. Red where `same_content` does not read the kind of point,
    /// or not the phase, and where the walk leaves the phase out.
    #[test]
    fn the_walk_names_an_arc_held_ellipses_anchor_and_phase() {
        use super::{Ellipse, Held};
        use crate::{CurveDatum, DatumValue};
        let (at, n) = (Point3::new(3.5, -1.0, 2.0), Vec3::new(2.0, 2.0, 1.0));
        let toward = Vec3::new(1.0, -2.0, 2.0);
        let arc = Ellipse::anchored(at, n, toward, 30.0, 41.0);
        let centre = Ellipse::new(super::Frame::new(at, n, toward), 30.0, 41.0);
        let turned = Ellipse {
            held: Held::Arc { phase: (0.0, -1.0) },
            ..arc
        };
        let three = [centre, arc, turned].map(Curve3::Ellipse);
        for (i, a) in three.iter().enumerate() {
            for (j, b) in three.iter().enumerate() {
                assert_eq!(a.same_content(b), i == j, "{a:?} against {b:?}");
            }
        }
        let point = |c: &Curve3<f64>, d| match c.data().named(d) {
            Some(DatumValue::Point(p)) => Some((p.x, p.y, p.z)),
            _ => None,
        };
        let at = (at.x, at.y, at.z);
        let phase = |c: &Curve3<f64>| match c.data().named(CurveDatum::Phase) {
            Some(DatumValue::Direction(d)) => Some((d.x, d.y, d.z)),
            _ => None,
        };
        assert_eq!(point(&three[0], CurveDatum::Center), Some(at));
        assert!(point(&three[0], CurveDatum::Anchor).is_none() && phase(&three[0]).is_none());
        for (k, want) in [(1, (1.0, 0.0, 0.0)), (2, (0.0, -1.0, 0.0))] {
            assert_eq!(point(&three[k], CurveDatum::Anchor), Some(at));
            assert!(point(&three[k], CurveDatum::Center).is_none());
            assert_eq!(phase(&three[k]), Some(want));
        }
    }

    /// **A re-seamed arc-held ellipse reads back, lifts and moves as
    /// itself.** Seamed near each of its four principal vertices, and
    /// each of those run back:
    ///
    /// - a point's parameter reads back as the parameter it was
    ///   evaluated at (red where the direction of a point ignores the
    ///   phase's sine, or its cosine);
    /// - lifted to `Interval`, its phase is the stored pair, and its
    ///   point's and its centre's enclosures sit on the `f64` ones (red
    ///   where the lift drops the phase, which reads the anchor as the
    ///   turned seam's vertex);
    /// - moved, it is the same ellipse moved: translated, and carried by
    ///   a rotation, its point at each parameter is the image of the
    ///   original's there (red where the map drops the phase).
    #[test]
    fn a_re_seamed_arc_held_ellipse_reads_back_lifts_and_moves_as_itself() {
        use super::{Conic, Ellipse};
        use geom_core::{Bounds, Interval, RigidMotion};
        let o = Point3::origin();
        let (at, n) = (Point3::new(3.5, -1.0, 2.0), Vec3::new(2.0, 2.0, 1.0));
        let toward = Vec3::new(1.0, -2.0, 2.0);
        let lift = Interval::from_f64;
        let mid = |p: Point3<Interval>| {
            Point3::new(
                0.5 * (p.x.lo() + p.x.hi()),
                0.5 * (p.y.lo() + p.y.hi()),
                0.5 * (p.z.lo() + p.z.hi()),
            )
        };
        let turn = RigidMotion::rotation_about_axis(
            Point3::new(0.5, 2.0, -1.0),
            super::Frame::new(o, Vec3::new(1.0, 2.0, -2.0), Vec3::unit_x()).axis_unit(),
            0.7,
        )
        .unwrap();
        let by = Vec3::new(0.25, -4.0, 1.5);
        for (a, b) in [(0.7, 0.3), (30.0, 41.0), (1500.0, 15.0), (1e6, 1e4)] {
            let e = Ellipse::anchored(at, n, toward, a, b);
            let scale = (e.center() - o).norm() + 2.0 * a.max(b);
            let slack = 64.0 * f64::EPSILON * scale;
            for seam in [0.3, 1.4, 2.9, -1.5] {
                for r in [e.seamed_near(seam), e.seamed_near(seam).reversed()] {
                    let (_, phase) = anchor_of(&r).unwrap();
                    let lifted = r.map_scalar(|f| f.map_scalar(lift), lift);
                    let lifted_phase = match lifted.held {
                        super::Held::Arc { phase } => phase,
                        super::Held::Centre => {
                            unreachable!("the lift keeps the ellipse on its arc")
                        }
                    };
                    assert_eq!(
                        (
                            lifted_phase.0.lo(),
                            lifted_phase.0.hi(),
                            lifted_phase.1.lo(),
                            lifted_phase.1.hi()
                        ),
                        (phase.0, phase.0, phase.1, phase.1),
                        "({a}, {b}) seamed near {seam}: the lifted phase"
                    );
                    assert!(
                        mid(lifted.center()).distance(r.center()) <= slack,
                        "({a}, {b}) seamed near {seam}: the lifted centre"
                    );
                    let translated = r.translated(by);
                    let carried = r.mapped(|f| f.moved(&turn));
                    for t in [-3.0, -1.2, -2e-3, 0.0, 1e-7, 0.4, 1.55, 2.9] {
                        let p = r.point_at(t);
                        let back = r.param_in(p, (t - 0.3, t + 0.3));
                        assert!(
                            (back - t).abs() <= slack / a.min(b),
                            "({a}, {b}) seamed near {seam}: {t} reads back as {back}"
                        );
                        assert!(
                            mid(lifted.point_at(lift(t))).distance(p) <= slack,
                            "({a}, {b}) seamed near {seam}: the lifted point at {t}"
                        );
                        assert!(
                            (translated.point_at(t) - (p + by)).norm() <= slack,
                            "({a}, {b}) seamed near {seam}: translated, at {t}"
                        );
                        assert!(
                            (carried.point_at(t) - turn.transform_point(p)).norm() <= slack,
                            "({a}, {b}) seamed near {seam}: carried by a rotation, at {t}"
                        );
                    }
                }
            }
        }
    }

    /// **Running an ellipse back is [`Ellipse::reversed`], and a frame's
    /// reversal handed to [`Ellipse::mapped`] is another map.** On an
    /// arc-held ellipse seamed a quarter turn on, `reversed` at `−t` is
    /// the ellipse at `t` bit for bit. `mapped(Frame::reversed)` is the
    /// ellipse turned half a turn about the line through its anchor
    /// along its seam: the anchor is kept, each point is that half
    /// turn's image, and it is not the same locus (its point at `−t`
    /// stands up to twice the semi-axis across the seam from the
    /// ellipse's at `t`). Held at its centre the two agree, the line
    /// passing through the centre.
    #[test]
    fn running_an_ellipse_back_is_its_own_door_and_not_a_frames_reversal() {
        use super::{Conic, Ellipse, Frame};
        let bits = |v: Vec3<f64>| [v.x, v.y, v.z].map(f64::to_bits);
        let o = Point3::origin();
        let (at, n) = (Point3::new(3.5, -1.0, 2.0), Vec3::new(2.0, 2.0, 1.0));
        let toward = Vec3::new(1.0, -2.0, 2.0);
        let params = [-3.0, -1.2, 0.0, 0.4, 1.55, 2.9];
        for (a, b) in [(30.0, 41.0), (1500.0, 15.0)] {
            let r = Ellipse::anchored(at, n, toward, a, b).seamed_near(1.4);
            let (anchor, phase) = anchor_of(&r).unwrap();
            assert_eq!(phase, (0.0, -1.0));
            let (back, by_frame) = (r.reversed(), r.mapped(Frame::reversed));
            assert_eq!(
                anchor_of(&by_frame).map(|(p, _)| bits(p - o)),
                Some(bits(anchor - o))
            );
            let u = r.frame().u_ref();
            let slack = 64.0 * f64::EPSILON * ((r.center() - o).norm() + 2.0 * a.max(b));
            let mut apart = 0.0_f64;
            for t in params {
                let p = r.point_at(t);
                assert_eq!(
                    bits(back.point_at(-t) - o),
                    bits(p - o),
                    "({a}, {b}) at {t}"
                );
                let w = p - anchor;
                let half_turned = anchor + (u * (2.0 * w.dot(u)) - w);
                assert!(
                    (by_frame.point_at(t) - half_turned).norm() <= slack,
                    "({a}, {b}) at {t}: the frame's reversal is the half turn about the seam"
                );
                apart = apart.max((by_frame.point_at(-t) - p).norm());
            }
            // The anchor sits a quarter turn from the seam, so the two
            // centres are twice the semi-axis across the seam apart.
            assert!(
                (apart - 2.0 * r.semi_v()).abs() <= slack,
                "({a}, {b}): the frame's reversal stands {apart:e} off, not twice {}",
                r.semi_v()
            );
            let centred = Ellipse::new(r.frame(), r.semi_u(), r.semi_v());
            for t in params {
                assert_eq!(
                    bits(centred.mapped(Frame::reversed).point_at(-t) - o),
                    bits(centred.reversed().point_at(-t) - o)
                );
            }
        }
    }

    /// **A re-seam moves parameter zero and never the anchor.** An
    /// anchored ellipse seamed near each of its four principal vertices:
    ///
    /// - the anchor is the stored one, bit for bit, and the phase is the
    ///   exact quarter turn `(cos, sin)(−k·π/2)` (red where the re-seam
    ///   re-anchors at the turned seam's vertex);
    /// - it is the same ellipse: its point at `t − k·π/2` is the
    ///   original's at `t`, and its centre the original's, to the rounding
    ///   of their own size (red where the phase is dropped, which reads
    ///   the anchor as the new seam's vertex);
    /// - parameter zero is at the new seam's principal vertex,
    ///   `centre + u_ref·semi_u` of the turned frame, so the parameter is
    ///   still the eccentric anomaly from `u_ref` that every reader pairs
    ///   with the principal directions (red where parameter zero is left
    ///   at the anchor);
    /// - run back, its point at `−t` is its own at `t`, bit for bit (red
    ///   where the reversal keeps the phase's sine);
    /// - at `Interval`, its centre is no wider than the original's: the
    ///   centre is read from the stored anchor, and a re-seam re-sums
    ///   nothing (red where the anchor is re-summed, which carries the
    ///   semi-axis into the centre three times).
    #[test]
    fn a_re_seam_moves_parameter_zero_and_never_the_anchor() {
        use super::{Conic, Ellipse};
        use core::f64::consts::FRAC_PI_2;
        use geom_core::{Bounds, Interval};
        let bits = |v: Vec3<f64>| [v.x, v.y, v.z].map(f64::to_bits);
        let o = Point3::origin();
        let (at, n) = (Point3::new(3.5, -1.0, 2.0), Vec3::new(2.0, 2.0, 1.0));
        let toward = Vec3::new(1.0, -2.0, 2.0);
        for (a, b) in [(0.7, 0.3), (30.0, 41.0), (1500.0, 15.0), (1e6, 1e4)] {
            let e = Ellipse::anchored(at, n, toward, a, b);
            let scale = (e.center() - o).norm() + 2.0 * a.max(b);
            let slack = 16.0 * f64::EPSILON * scale;
            for (seam, k) in [(0.3, 0), (1.4, 1), (2.9, 2), (-1.5, 3), (3.3, 2), (4.6, 3)] {
                let r = e.seamed_near(seam);
                let (anchor, phase) = anchor_of(&r).unwrap();
                assert_eq!(bits(anchor - o), bits(at - o));
                let want = [(1.0, 0.0), (0.0, -1.0), (-1.0, 0.0), (0.0, 1.0)][k];
                assert_eq!(phase, want, "({a}, {b}) seamed near {seam}");
                assert!((r.center() - e.center()).norm() <= slack, "the centre");
                let turn = FRAC_PI_2 * f64::from(u32::try_from(k).unwrap());
                for t in [-3.0, -1.2, -2e-3, 0.0, 1e-7, 0.4, 1.55, 2.9] {
                    let gap = (r.point_at(t - turn) - e.point_at(t)).norm();
                    assert!(
                        gap <= slack,
                        "({a}, {b}) seamed near {seam}: {gap:e} off the original at {t}"
                    );
                    let back = Curve3::Ellipse(r).reversed().unwrap();
                    assert_eq!(
                        bits(back.eval(-t) - o),
                        bits(r.point_at(t) - o),
                        "({a}, {b}) seamed near {seam}, reversed at {t}"
                    );
                }
                let f = r.frame();
                let vertex = f.origin() + f.u_ref() * r.semi_u();
                assert!(
                    (r.point_at(0.0) - vertex).norm() <= slack,
                    "({a}, {b}) seamed near {seam}: parameter zero is the seam's vertex"
                );
            }
            let lift = Interval::from_f64;
            let wide = |x: f64| Interval::from_bounds(x - 1e-13 * x.abs(), x + 1e-13 * x.abs());
            let ei = e.map_scalar(|f| f.map_scalar(lift), wide);
            let width = |p: Point3<Interval>| {
                [p.x, p.y, p.z]
                    .into_iter()
                    .map(|c| c.hi() - c.lo())
                    .fold(0.0, f64::max)
            };
            for seam in [0.3, 1.4, 2.9, -1.5] {
                let ri = ei.seamed_near(lift(seam));
                assert!(
                    width(ri.center()) <= 1.01 * width(ei.center()) + 8.0 * f64::EPSILON * scale,
                    "({a}, {b}) seamed near {seam} at Interval: the centre {:e} wide against {:e}",
                    width(ri.center()),
                    width(ei.center())
                );
            }
        }
    }

    /// **A re-seamed ellipse holds a point near its anchor to the
    /// semi-axis's rounding, not the arc's.** As minted, parameter zero
    /// is at the anchor and a point a step `d` along the arc is the
    /// anchor plus `P′·d` to a few roundings of its own coordinates.
    /// Seamed a quarter turn on, the anchor has not moved but its
    /// parameter is `−π/2`, which the parameter itself now carries: the
    /// same step lands a rounding of the SEMI-AXIS off (measured 1.5 to
    /// 3.6 `u·a` over anchors a quarter and a half turn from the seam).
    ///
    /// The second half is a pin on a known shortfall, held from both
    /// sides: red if it gets worse than `8 u·a`, and red if it becomes
    /// arc-scale, which is what
    /// `work/conic/a-section-conic-is-anchored-at-a-general-point-of-its-arc.md`
    /// closes (parameter zero kept at the anchor through a re-seam); the
    /// unit that does it turns this half over.
    #[test]
    fn a_re_seamed_ellipse_resolves_near_its_anchor_at_the_semi_axis() {
        use super::{Conic, Ellipse};
        let u = geom_core::UNIT_ROUNDOFF;
        let (at, n) = (Point3::new(3.5, -1.0, 2.0), Vec3::new(2.0, 2.0, 1.0));
        let toward = Vec3::new(1.0, -2.0, 2.0);
        let own = (at - Point3::origin()).norm();
        for a in [1e4, 1e5, 1e6] {
            let e = Ellipse::anchored(at, n, toward, a, 0.37 * a);
            let worst = |k: &Ellipse<f64>, at_param: f64| {
                let speed = k.derivative_at(at_param);
                [-3e-10, -1e-10, 7e-11, 2e-10, 3.3e-10]
                    .into_iter()
                    .map(|d: f64| (k.point_at(at_param + d) - (at + speed * d)).norm())
                    .fold(0.0, f64::max)
            };
            // The step's own curvature term, `a·d²/2`, is under 1e-13.
            let minted = worst(&e, 0.0);
            assert!(
                minted <= 16.0 * u * own + 1e-13,
                "a {a:e}, as minted: {minted:e} off, past its own rounding"
            );
            let turned = e.seamed_near(1.4);
            assert_eq!(anchor_of(&turned).map(|(_, p)| p), Some((0.0, -1.0)));
            let off = worst(&turned, -core::f64::consts::FRAC_PI_2);
            assert!(
                off <= 8.0 * u * a,
                "a {a:e}, seamed a quarter turn on: {off:e} off, past 8 u·a"
            );
            assert!(
                off >= 0.05 * u * a,
                "a {a:e}, seamed a quarter turn on: {off:e} off is arc-scale now: turn this \
                 half of the row over and close the general anchor's row"
            );
        }
    }

    /// **The conic readings are the D9 evaluators**: the point, the
    /// derivatives and the jet a reader takes through [`Conic`], and
    /// [`Curve3`]'s, are [`written_out`]'s bit for bit, on either kind and
    /// in either stored order, so a change to either kind's formula goes
    /// red here (a circle read with the ellipse formula at equal
    /// semi-axes rounds apart). A point's parameter on a span's branch is
    /// the parameter it was evaluated at, and the speed lies within its
    /// bounds.
    #[test]
    fn the_conic_readings_are_the_carriers_own() {
        let bits = |v: Vec3<f64>| [v.x, v.y, v.z].map(f64::to_bits);
        let o = Point3::origin();
        for carrier in carriers() {
            let k = carrier.conic().unwrap();
            for t in [-2.5, -0.3, 0.0, 0.7, 2.0, 3.1] {
                let [p, d, d2] = written_out(&carrier, t).map(bits);
                assert_eq!(bits(k.point_at(t) - o), p, "{carrier:?} at {t}");
                assert_eq!(bits(carrier.eval(t) - o), p, "{carrier:?} at {t}");
                assert_eq!(bits(k.derivative_at(t)), d, "{carrier:?} at {t}");
                assert_eq!(bits(carrier.deriv(t)), d, "{carrier:?} at {t}");
                assert_eq!(bits(k.second_derivative_at(t)), d2, "{carrier:?} at {t}");
                assert_eq!(bits(carrier.deriv2(t)), d2, "{carrier:?} at {t}");
                let (jp, jd) = k.jet_at(t);
                assert_eq!((bits(jp - o), bits(jd)), (p, d), "{carrier:?} at {t}");
                let span = (t - 0.4, t + 0.9);
                let back = k.param_in(carrier.eval(t), span);
                assert!(
                    (back - t).abs() < 1e-14,
                    "{carrier:?}: {t} reads back as {back}"
                );
                let lo = k.speed_lo();
                assert!(
                    lo <= k.speed_at(t) && k.speed_at(t) <= k.speed_hi(),
                    "{carrier:?}"
                );
                assert!(lo > 0.0 && lo <= k.semi_axes().0.min(k.semi_axes().1));
            }
        }
    }

    /// **A reflected carrier is unrepresentable**: a negative semi-axis
    /// is the other sense of traversal, which is the frame's (`−axis`,
    /// and `−u_ref` for `semi_u`), so the constructor refuses it loudly.
    #[test]
    #[should_panic(expected = "Ellipse::new: semi_u -0.9 is not a finite non-negative length")]
    fn an_ellipse_with_a_negative_semi_axis_panics_at_construction() {
        let f = super::Frame::new(Point3::origin(), Vec3::unit_z(), Vec3::unit_x());
        let _ = super::Ellipse::new(f, -0.9, 0.4);
    }

    /// The circle's constructor refuses a negative radius as the
    /// ellipse's does.
    #[test]
    #[should_panic(expected = "Circle::new: radius -0.7 is not a finite non-negative length")]
    fn a_circle_with_a_negative_radius_panics_at_construction() {
        let f = super::Frame::new(Point3::origin(), Vec3::unit_z(), Vec3::unit_x());
        let _ = super::Circle::new(f, -0.7);
    }

    /// An enclosure entirely below zero is a definite negative.
    #[test]
    #[should_panic(expected = "Ellipse::new: semi_v")]
    fn an_interval_semi_axis_definitely_negative_panics_at_construction() {
        use geom_core::Interval;
        let f = super::Frame::new(Point3::origin(), Vec3::unit_z(), Vec3::unit_x())
            .map_scalar(Interval::from_f64);
        let _ = super::Ellipse::new(
            f,
            Interval::from_f64(0.9),
            Interval::from_bounds(-0.8, -1e-20),
        );
    }

    /// **An enclosure across zero is representable, and the speed
    /// readers bound it**: `semi_u ∈ [−2, 1]`, `semi_v = 0.5` constructs
    /// (its sign is undecided, which a reader takes in band), the least
    /// speed is floored at zero, and the greatest reaches `2`, the
    /// magnitude the enclosure's negative end carries: read as the
    /// larger signed semi-axis it would be `1`, under the speed at
    /// `θ = π/2`. A `−0.0` radius is no negative length.
    #[test]
    fn an_enclosure_across_zero_constructs_and_its_speeds_bound_it() {
        use geom_core::{Bounds, Interval};
        let f = super::Frame::new(Point3::origin(), Vec3::unit_z(), Vec3::unit_x())
            .map_scalar(Interval::from_f64);
        let e = super::Ellipse::new(f, Interval::from_bounds(-2.0, 1.0), Interval::from_f64(0.5));
        let k: &dyn super::Conic<Interval> = &e;
        let (lo, hi) = (k.speed_lo(), k.speed_hi());
        assert!(lo.lo() >= 0.0 && lo.hi() >= 0.5, "speed_lo {lo:?}");
        assert!(hi.hi() >= 2.0, "speed_hi {hi:?} under |semi_u| = 2");
        let at = k.speed_at(Interval::from_f64(core::f64::consts::FRAC_PI_2));
        assert!(at.hi() >= 2.0 - 1e-12, "speed_at(π/2) {at:?}");
        let flat = super::Frame::new(Point3::origin(), Vec3::unit_z(), Vec3::unit_x());
        assert_eq!(
            super::Circle::new(flat, -0.0).radius().to_bits(),
            (-0.0f64).to_bits()
        );
    }

    /// **A circle whose radius straddles zero has sound speeds**:
    /// `radius ∈ [−2, 1]` constructs, its least speed is floored at zero
    /// and its greatest, and its speed at any `θ`, reach `2`, the
    /// magnitude the enclosure's negative end carries. Read as the signed
    /// radius, the greatest would stop at `1`.
    #[test]
    fn a_circle_radius_across_zero_constructs_and_its_speeds_bound_it() {
        use geom_core::{Bounds, Interval};
        let f = super::Frame::new(Point3::origin(), Vec3::unit_z(), Vec3::unit_x())
            .map_scalar(Interval::from_f64);
        let c = super::Circle::new(f, Interval::from_bounds(-2.0, 1.0));
        let k: &dyn super::Conic<Interval> = &c;
        let (lo, hi) = (k.speed_lo(), k.speed_hi());
        assert!(lo.lo() >= 0.0 && lo.hi() >= 1.0, "speed_lo {lo:?}");
        assert!(hi.hi() >= 2.0, "speed_hi {hi:?} under |radius| = 2");
        let at = k.speed_at(Interval::from_f64(0.3));
        assert!(at.lo() >= 0.0 && at.hi() >= 2.0, "speed_at {at:?}");
    }

    /// A poison turn reads poison sizes, which the checked constructor
    /// refuses rather than store.
    #[test]
    #[should_panic(expected = "Ellipse::quarter_turned_where: semi_u NaN")]
    fn a_poison_seam_turn_panics_at_construction() {
        let f = super::Frame::new(Point3::origin(), Vec3::unit_z(), Vec3::unit_x());
        let _ = super::Ellipse::new(f, 0.9, 0.4).seamed_near(f64::NAN);
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::float_cmp)]
mod door_rows;
