//! **The conics: a circle and an ellipse carrier, and what they share.**
//!
//! [`Curve3::Circle`] and [`Curve3::Ellipse`] are two kinds: a circle is
//! not degraded into an ellipse with equal semi-axes, and an ellipse is
//! not a circle with a stretch. What the two share is named once and
//! held by each:
//!
//! - **The placement**, [`Frame`] (a rigid placement, not a conic's
//!   alone): a centre and an oriented
//!   orthonormal basis `(u_ref, v_ref, axis)`, `v_ref = axis × u_ref`
//!   (computed, never stored, so the frame is right-handed by
//!   construction). `axis` is the unit plane normal and `u_ref` the unit
//!   reference direction, `u_ref ⊥ axis`: conventional data, unchecked
//!   here, certified at rest by tier 3. `u_ref` carries the seam: `θ = 0`
//!   lives on it. A reading of the placement alone (a point's in-plane
//!   components, the frame reversed or moved) takes a `&Frame`.
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
//! # Conventions
//!
//! - **The parameter** `θ` is in radians, domain ℝ, period 2π, and winds
//!   counterclockwise viewed from the tip of `axis` (right-hand rule). A
//!   circle's is its angle, `|dP/dθ| = radius`; an ellipse's is its
//!   **eccentric anomaly**, not arc length and not the polar angle of the
//!   point: `|dP/dθ|² = semi_u²sin²θ + semi_v²cos²θ` varies between the
//!   two semi-axes.
//! - **A unit frame, unchecked.** Nothing that reads a conic normalizes
//!   its frame, and nothing can afford to: the sizes are the lengths
//!   every speed and curvature bound is stated at, so a non-unit `u_ref`
//!   rescales the curve without rescaling the bound. A caller
//!   synthesising a frame owes the normalization.
//! - **Positive sizes.** A radius and both semi-axes are positive
//!   lengths, in metres. [`Curve3::ellipse`] refuses a non-positive
//!   semi-axis, certification's span meter refuses one ([`speed_floor`]),
//!   and tier 3 certifies each at rest. A struct literal that bypasses the
//!   doors owns the consequences: evaluators consume the fields as given
//!   (well-defined garbage, never poison), and the readings below assume
//!   the sizes positive, reading no magnitudes.
//! - **Equal semi-axes are an ellipse.** Whether a configuration is a
//!   circle or an ellipse is decided once by its pose (a section table's
//!   tilt reading), never by comparing the semi-axes.
//! - **The seam.** A caller seams an ellipse at the principal vertex
//!   nearest the part of it in use ([`Ellipse::seamed_near`]), so the
//!   parameter there is near zero and resolves to the scale of that part.
//! - **One branch rule.** A point's parameter is its principal angle
//!   ([`Conic::angle_of`]) moved by whole turns: to within half a turn of
//!   a parameter (`Conic::param_near`), or onto the branch of a span's
//!   middle ([`on_span_branch`], [`Conic::param_in`]).
//!
//! [`speed_floor`]: crate::carrier_speed::speed_floor

use geom_core::{Decide, Point3, Real, Rounded, Vec3};

use crate::azimuth::{self, Azimuth};
use crate::curves::Curve3;

/// **A rigid placement**: a point and an oriented orthonormal basis
/// `(u_ref, v_ref, axis)`, `v_ref = axis × u_ref` (computed, never
/// stored). `axis` and `u_ref` are unit and orthogonal by convention,
/// unchecked here and certified at rest. A circle and an ellipse each hold
/// one, centred on the conic and seamed at `u_ref` (the
/// [module docs](self)); the same triple places the plane, cylinder and
/// cone surfaces, which do not hold it yet.
#[derive(Clone, Copy, Debug)]
pub struct Frame<T: Real> {
    /// The centre.
    pub center: Point3<T>,
    /// The unit normal of the plane (right-hand winding rule;
    /// conventional, unchecked).
    pub axis: Vec3<T>,
    /// The unit reference direction ⊥ `axis` where `θ = 0` lives — the
    /// seam, carried as conventional data per D2.
    pub u_ref: Vec3<T>,
}

impl<T: Real> Frame<T> {
    /// The frame at `center` with normal `axis` and seam `u_ref`.
    #[must_use]
    pub fn new(center: Point3<T>, axis: Vec3<T>, u_ref: Vec3<T>) -> Self {
        Self {
            center,
            axis,
            u_ref,
        }
    }

    /// `axis × u_ref`, the direction `θ = π/2` points along.
    #[must_use]
    pub fn v_ref(&self) -> Vec3<T> {
        self.axis.cross(self.u_ref)
    }

    /// The components of `p − center` along `u_ref` and `v_ref`.
    #[must_use]
    pub fn in_plane(&self, p: Point3<T>) -> (T, T) {
        let w = p - self.center;
        (w.dot(self.u_ref), w.dot(self.v_ref()))
    }

    /// The same placement run back: the axis flipped, the seam kept, so
    /// `θ ↦ −θ` names the same point.
    #[must_use]
    pub fn reversed(&self) -> Self {
        Self {
            axis: -self.axis,
            ..*self
        }
    }

    /// The same placement with every scalar mapped by `f`.
    #[must_use]
    pub fn map_scalar<U: Real>(&self, f: impl Fn(T) -> U) -> Frame<U> {
        Frame {
            center: self.center.map(&f),
            axis: self.axis.map(&f),
            u_ref: self.u_ref.map(&f),
        }
    }

    /// The same placement moved by `by`.
    #[must_use]
    pub fn translated(&self, by: Vec3<T>) -> Self {
        Self {
            center: self.center + by,
            ..*self
        }
    }
}

/// **A full circle**, `P(θ) = center + (u_ref·cos θ + v_ref·sin θ)·radius`.
#[derive(Clone, Copy, Debug)]
pub struct Circle<T: Real> {
    /// The placement.
    pub frame: Frame<T>,
    /// The radius in metres (positive).
    pub radius: T,
}

/// **A full ellipse**,
/// `P(θ) = center + u_ref·(semi_u·cos θ) + v_ref·(semi_v·sin θ)`, `u_ref`
/// a principal direction.
#[derive(Clone, Copy, Debug)]
pub struct Ellipse<T: Real> {
    /// The placement; `u_ref` is a principal direction, along either
    /// semi-axis.
    pub frame: Frame<T>,
    /// The semi-axis along `u_ref`, in metres (positive).
    pub semi_u: T,
    /// The semi-axis along `axis × u_ref`, in metres (positive).
    pub semi_v: T,
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
    /// The placement.
    fn frame(&self) -> &Frame<T>;

    /// `(semi_u, semi_v)` in stored order: a circle's are its radius
    /// twice.
    fn semi_axes(&self) -> (T, T);

    /// The point at `theta`.
    fn point_at(&self, theta: T) -> Point3<T>;

    /// `dP/dθ` at `theta`.
    fn derivative_at(&self, theta: T) -> Vec3<T>;

    /// The point and `dP/dθ` at `theta` from one `sin_cos`, each half
    /// bit for bit its own evaluator's.
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

    /// The least speed `|P′|` over the carrier: the smaller semi-axis.
    fn speed_lo(&self) -> T;

    /// The greatest speed `|P′|` over the carrier, and a bound on `|P″|`:
    /// the larger semi-axis.
    fn speed_hi(&self) -> T;

    /// The speed `|P′(θ)|` at `theta`, held to
    /// `[`[`Self::speed_lo`]`, `[`Self::speed_hi`]`]`: a circle's is its
    /// radius exactly at every scalar.
    fn speed_at(&self, theta: T) -> T;

    /// The principal frame with the larger semi-axis first, whichever the
    /// seam is at; a tie, and a circle, keep the seam's own axis.
    fn major_frame(&self) -> MajorFrame<T>;

    /// **The point at `θ` with a first-order running bound on its
    /// rounding** ([`Rounded`]), from `sin θ` and `cos θ` already charged:
    /// [`Self::point_at`] operation for operation, so its value is that
    /// point bit for bit. The frame and the sizes are taken as exact.
    fn running_point(&self, sin_cos: (Rounded<T>, Rounded<T>)) -> [Rounded<T>; 3];

    /// **`p` in the conic's unit coordinates**: its in-plane components
    /// over the semi-axes, `((p − c)·u_ref/a, (p − c)·v_ref/b)`, in which
    /// the conic is the unit circle and its parameter the polar angle.
    fn unit_in_plane(&self, p: Point3<T>) -> (T, T) {
        self.unit_of(p - self.frame().center)
    }

    /// **The vector `w` in the conic's unit coordinates**:
    /// `(w·u_ref/a, w·v_ref/b)`, [`Self::unit_in_plane`] for an offset
    /// rather than a point.
    fn unit_of(&self, w: Vec3<T>) -> (T, T) {
        let (f, (a, b)) = (self.frame(), self.semi_axes());
        (w.dot(f.u_ref) / a, w.dot(f.v_ref()) / b)
    }

    /// **The conic's harmonic along `n`**: `(a·(u_ref·n), b·(v_ref·n))`,
    /// so `(P(θ) − center)·n = first·cos θ + second·sin θ`. `n` need not
    /// be unit: the pair is linear in it.
    fn along(&self, n: Vec3<T>) -> (T, T) {
        let (f, (a, b)) = (self.frame(), self.semi_axes());
        (a * f.u_ref.dot(n), b * f.v_ref().dot(n))
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
        let f = self.frame();
        (f.u_ref * a, f.v_ref() * b)
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
    fn frame(&self) -> &Frame<T> {
        (**self).frame()
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
    fn running_point(&self, sin_cos: (Rounded<T>, Rounded<T>)) -> [Rounded<T>; 3] {
        (**self).running_point(sin_cos)
    }
    fn along(&self, n: Vec3<T>) -> (T, T) {
        (**self).along(n)
    }
    fn unit_in_plane(&self, p: Point3<T>) -> (T, T) {
        (**self).unit_in_plane(p)
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

/// A frame's centre, `u_ref` and `v_ref` entered exactly into running
/// arithmetic, `v_ref` crossed there in [`Vec3::cross`]'s order.
fn running_frame<T: Real>(f: &Frame<T>) -> RunningFrame<T> {
    use geom_core::running::{cross, exact_vec};
    let (n, u) = (exact_vec(f.axis), exact_vec(f.u_ref));
    let c = exact_vec(Vec3::new(f.center.x, f.center.y, f.center.z));
    (c, u, cross(n, u))
}

/// A frame in running arithmetic: centre, `u_ref`, `v_ref`.
type RunningFrame<T> = ([Rounded<T>; 3], [Rounded<T>; 3], [Rounded<T>; 3]);

impl<T: Real> Circle<T> {
    /// The circle of `radius` placed by `frame`.
    #[must_use]
    pub fn new(frame: Frame<T>, radius: T) -> Self {
        Self { frame, radius }
    }

    /// The point from an azimuthal frame already built:
    /// `center + radial·radius`, exactly as parenthesized (D9) — the one
    /// spelling of a circle's point, which [`Curve3::circle_at`] and
    /// [`Conic::jet_at`] both reach.
    pub(crate) fn point_on(&self, at: &azimuth::AzimuthFrame<T>) -> Point3<T> {
        self.frame.center + at.radial.0 * self.radius
    }

    fn azimuth(&self, theta: T) -> azimuth::AzimuthFrame<T> {
        azimuth::frame(self.frame.axis, self.frame.u_ref, theta)
    }
}

impl<T: Real> Conic<T> for Circle<T> {
    fn frame(&self) -> &Frame<T> {
        &self.frame
    }

    fn semi_axes(&self) -> (T, T) {
        (self.radius, self.radius)
    }

    fn point_at(&self, theta: T) -> Point3<T> {
        self.point_on(&self.azimuth(theta))
    }

    fn derivative_at(&self, theta: T) -> Vec3<T> {
        self.azimuth(theta).tangential.0 * self.radius
    }

    fn jet_at(&self, theta: T) -> (Point3<T>, Vec3<T>) {
        let at = self.azimuth(theta);
        (self.point_on(&at), at.tangential.0 * self.radius)
    }

    fn second_derivative_at(&self, theta: T) -> Vec3<T> {
        let ((s, c), v_ref) = azimuth::basis(self.frame.axis, self.frame.u_ref, theta);
        (self.frame.u_ref * (-c) + v_ref * (-s)) * self.radius
    }

    /// `(x/r, y/r)`'s direction, read as `(y·r, x·r)`, as an ellipse
    /// reads its own: a circle of negative radius, traversed through the
    /// seam's antipode, reads the parameter that reproduces the point.
    fn azimuth(&self, p: Point3<T>) -> Azimuth<T> {
        let (x, y) = self.frame.in_plane(p);
        Azimuth::of(y * self.radius, x * self.radius)
    }

    fn speed_lo(&self) -> T {
        self.radius
    }

    fn speed_hi(&self) -> T {
        self.radius
    }

    fn speed_at(&self, _theta: T) -> T {
        self.radius
    }

    fn running_point(&self, (sin, cos): (Rounded<T>, Rounded<T>)) -> [Rounded<T>; 3] {
        let (c, u, v) = running_frame(&self.frame);
        let r = Rounded::exact(self.radius);
        core::array::from_fn(|i| c[i] + (u[i] * cos + v[i] * sin) * r)
    }

    fn major_frame(&self) -> MajorFrame<T> {
        MajorFrame {
            major: self.radius,
            minor: self.radius,
            major_dir: self.frame.u_ref,
            phase: T::zero(),
        }
    }
}

impl<T: Real> Ellipse<T> {
    /// The ellipse placed by `frame` with `semi_u` along its `u_ref` and
    /// `semi_v` along its `v_ref`, unchecked; [`Curve3::ellipse`] is the
    /// deciding door.
    #[must_use]
    pub fn new(frame: Frame<T>, semi_u: T, semi_v: T) -> Self {
        Self {
            frame,
            semi_u,
            semi_v,
        }
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
        let f = &self.frame;
        self.frame_turned_where(f.v_ref().dot(axis).abs() - f.u_ref.dot(axis).abs())
    }

    /// The frame as stored where `turn ≤ 0`, else
    /// [quarter-turned](Self::quarter_turned_where), at its phase.
    fn frame_turned_where(&self, turn: T) -> MajorFrame<T> {
        let e = self.quarter_turned_where(turn);
        MajorFrame {
            major: e.semi_u,
            minor: e.semi_v,
            major_dir: e.frame.u_ref,
            phase: turn.select_le_zero(T::zero(), T::pi() * T::from_f64(0.5)),
        }
    }

    /// **The same ellipse seamed a quarter turn on where `turn > 0`**, as
    /// stored where `turn ≤ 0`, read without a branch. `θ = s + π/2` reads
    /// `center + v·(semi_v·cos s) − u_ref·(semi_u·sin s)` with
    /// `v = axis × u_ref`: the seam turns to `v`, renormalized so the turn
    /// adds no length to the frame, and the semi-axes swap.
    #[must_use]
    pub fn quarter_turned_where(&self, turn: T) -> Self {
        let f = self.frame;
        Self {
            frame: Frame {
                u_ref: Vec3::select(turn, f.u_ref, f.v_ref().normalize()),
                ..f
            },
            semi_u: turn.select_le_zero(self.semi_u, self.semi_v),
            semi_v: turn.select_le_zero(self.semi_v, self.semi_u),
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
    /// Near the new seam the parameter is near zero, so it resolves to the
    /// scale of the arc there rather than to `ulp(π/2)` times the
    /// semi-axis across it.
    #[must_use]
    pub fn seamed_toward(&self, cos: T, sin: T) -> Self {
        let turn = sin.abs() - cos.abs();
        // The direction's cosine in the turned frame, `θ − π/2`, is `sin`.
        let back = T::zero() - turn.select_le_zero(cos, sin);
        let e = self.quarter_turned_where(turn);
        Self {
            frame: Frame {
                u_ref: Vec3::select(back, e.frame.u_ref, -e.frame.u_ref),
                ..e.frame
            },
            ..e
        }
    }

    fn basis(&self, theta: T) -> ((T, T), Vec3<T>) {
        azimuth::basis(self.frame.axis, self.frame.u_ref, theta)
    }
}

impl<T: Real> Conic<T> for Ellipse<T> {
    fn frame(&self) -> &Frame<T> {
        &self.frame
    }

    fn semi_axes(&self) -> (T, T) {
        (self.semi_u, self.semi_v)
    }

    fn point_at(&self, theta: T) -> Point3<T> {
        let ((s, c), v_ref) = self.basis(theta);
        self.frame.center + (self.frame.u_ref * (self.semi_u * c) + v_ref * (self.semi_v * s))
    }

    fn derivative_at(&self, theta: T) -> Vec3<T> {
        let ((s, c), v_ref) = self.basis(theta);
        self.frame.u_ref * (-(self.semi_u * s)) + v_ref * (self.semi_v * c)
    }

    fn jet_at(&self, theta: T) -> (Point3<T>, Vec3<T>) {
        let ((s, c), v_ref) = self.basis(theta);
        let u_ref = self.frame.u_ref;
        (
            self.frame.center + (u_ref * (self.semi_u * c) + v_ref * (self.semi_v * s)),
            u_ref * (-(self.semi_u * s)) + v_ref * (self.semi_v * c),
        )
    }

    fn second_derivative_at(&self, theta: T) -> Vec3<T> {
        let ((s, c), v_ref) = self.basis(theta);
        self.frame.u_ref * (-(self.semi_u * c)) + v_ref * (-(self.semi_v * s))
    }

    /// `(x/a, y/b)`'s direction, read as `(y·a, x·b)`: the same
    /// direction scaled by `a·b`, with no division.
    fn azimuth(&self, p: Point3<T>) -> Azimuth<T> {
        let (x, y) = self.frame.in_plane(p);
        Azimuth::of(y * self.semi_u, x * self.semi_v)
    }

    fn speed_lo(&self) -> T {
        self.semi_axes_by_size().1
    }

    fn speed_hi(&self) -> T {
        self.semi_axes_by_size().0
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

    fn running_point(&self, (sin, cos): (Rounded<T>, Rounded<T>)) -> [Rounded<T>; 3] {
        let (c, u, v) = running_frame(&self.frame);
        let (ac, bs) = (
            Rounded::exact(self.semi_u) * cos,
            Rounded::exact(self.semi_v) * sin,
        );
        core::array::from_fn(|i| c[i] + (u[i] * ac + v[i] * bs))
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
            Self::Line(_) | Self::Spiric { .. } | Self::Nurbs(_) => None,
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
                let (u, r) = (k.frame.u_ref, k.radius);
                let v = k.frame.axis.cross(u);
                [
                    (k.frame.center + (u * c + v * s) * r) - o,
                    (u * (-s) + v * c) * r,
                    (u * (-c) + v * (-s)) * r,
                ]
            }
            Curve3::Ellipse(k) => {
                let (u, a, b) = (k.frame.u_ref, k.semi_u, k.semi_v);
                let v = k.frame.axis.cross(u);
                [
                    (k.frame.center + (u * (a * c) + v * (b * s))) - o,
                    u * (-(a * s)) + v * (b * c),
                    u * (-(a * c)) + v * (-(b * s)),
                ]
            }
            _ => unreachable!(),
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
}
