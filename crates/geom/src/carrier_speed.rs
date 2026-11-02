//! **A carrier's speed**, the metres one step of its parameter covers.
//! [`speed_range`] bounds it over every parameter on every kind: a line's
//! `|dir|`, a conic's semi-axes (read as magnitudes in either stored
//! order, [`crate::Curve3::semi_axes_by_size`]), a spiric's and a
//! spline's bounds. [`turn_defect_rate`] bounds it over one stretch, a
//! window's turn defect, no looser than [`speed_range`] and tight as the
//! defect shrinks. [`speed_at`] and [`speed_hi`] answer the line and the
//! conics alone, `None` for a spiric or spline carrier: the speed at a
//! point a conic's root is metered by, and the closed form a carrier's
//! rounding resolution is summed from.

use geom_core::period::{Bounded, Extent};
use geom_core::{InfSpeed, Real, SpanLocate, SupSpeed, Vec3};

/// The speed `|C′(t)|` at `t`: a line's `|dir|`, a conic's
/// `√((a·sin t)² + (b·cos t)²)` held to its semi-axes, so a circle's is its
/// radius exactly at every scalar.
pub fn speed_at<T: SpanLocate>(carrier: &crate::Curve3<T>, t: T) -> Option<T> {
    match Kind::of(carrier) {
        kind @ Kind::Line(_) => kind.closed_hi(),
        Kind::Conic(c) => Some(c.speed_held(t)),
        Kind::Spiric { .. } | Kind::Nurbs(_) => None,
    }
}

/// The greatest speed over every `t`, [`speed_at`]'s bound: an
/// ellipse's larger semi-axis.
pub fn speed_hi<T: SpanLocate>(carrier: &crate::Curve3<T>) -> Option<T> {
    Kind::of(carrier).closed_hi()
}

/// Bounds on `|C′(t)|` over every `t`, on every kind: exact on a line
/// and a circle, an ellipse's semi-axes, a spiric's minor radius `r`
/// below (`|C′|² = r²(1 + (d/f)²·sin²v)`) and [`crate::spiric_rate_bounds`]
/// over a whole period above, and a spline's control-net bounds
/// ([`crate::NurbsCurve3::speed_lower_bound`], whose floor may be zero,
/// negative or poison: its assemblies are joined by `max`, and each may
/// read below zero where the hodograph runs near the origin). A rational spline's bounds are certified at `Interval`;
/// at `f64` they are the same expressions in round-to-nearest, not a
/// bound at extreme weights.
pub fn speed_range<T: SpanLocate>(carrier: &crate::Curve3<T>) -> (InfSpeed<T>, SupSpeed<T>) {
    Kind::of(carrier).range()
}

/// **The speed floor a span on a periodic carrier is decided at**, the
/// one spelling certification's check 2 and the conic readers share: a
/// circle's radius, an ellipse's `min(|semi_u|, semi_v)` (either stored
/// order, its smaller magnitude; a non-positive `semi_v` keeps the floor
/// non-positive, so no span is forward at it), a spiric's minor radius
/// (`|C′|² = r²(1 + (d/f)²·sin²v)`). `None` on a line or a spline, which
/// have no period.
pub fn speed_floor<T: Real>(carrier: &crate::Curve3<T>) -> Option<InfSpeed<T>> {
    match *carrier {
        crate::Curve3::Circle { radius, .. } => Some(InfSpeed::new(radius)),
        crate::Curve3::Ellipse { semi_u, semi_v, .. } => {
            Some(InfSpeed::new(semi_u.abs().min(semi_v)))
        }
        crate::Curve3::Spiric { minor_radius, .. } => Some(InfSpeed::new(minor_radius)),
        crate::Curve3::Line { .. } | crate::Curve3::Nurbs(_) => None,
    }
}

/// The **turn defect** of the window that starts at `t0` and spans
/// `extent`: the stretch between its end and its start one period on, a
/// gap where the headroom `h = τ − extent` is positive and an overlap
/// where it is negative. Returned as its middle `m = t0 − h/2` (one period
/// off, which no carrier's speed tells apart) and its **reach**, the
/// parameter distance from `m` that every point of it lies within: half
/// its width `|h|`, plus [`DEFECT_ROUNDING`] of `1 + |m|`.
fn turn_defect<T: Real>(t0: T, extent: Extent<T>) -> (T, T) {
    let half = T::from_f64(0.5);
    let gap = T::tau() - extent.get();
    let m = t0 - gap * half;
    let reach = gap.abs() * half + (T::one() + m.abs()) * T::from_f64(DEFECT_ROUNDING);
    (m, reach)
}

/// **The allowance a turn defect's reach carries for rounding**, in units
/// of `1 + |m|`, `m` the defect's middle. At `f64`, `m` is a rounded value
/// and lies a few units in the last place of `1 + |m|` off the true
/// middle, and a rate read at `m` carries a few units of its own; a rate
/// whose slope is at most `k` per radian moves by at most `k` times the
/// first, and `k ≥` the rate itself covers the second once `1 + |m| ≥ 1`
/// is folded in. `2⁻⁴⁶` is 64 to 128 units of `1 + |m|`'s last place; the
/// review of PR 5724 measured the rounding it absorbs at no more than a
/// sixteenth of it over 358k cases.
///
/// It is applied at **every scalar**, though an `Interval` reader needs
/// none (its `m` and its rates are enclosures already): these functions
/// are generic over the scalar and have no way to tell a certified one
/// from round-to-nearest, and the allowance only widens. At `Interval` it
/// widens a bound by `2⁻⁴⁶·(1 + |m|)` of the speed's slope — beneath any
/// band a turn is read in, and the same widening as at `f64`, so the two
/// lanes decide a window at the same place.
const DEFECT_ROUNDING: f64 = 1.0 / (1u64 << 46) as f64;

/// A sup on `|C′|` over the turn defect of the window that starts at `t0`
/// and spans `extent` ([`turn_defect`]). On a conic it is the speed at the
/// defect's middle plus the larger semi-axis (a bound on `|C″|`, so on how
/// fast `|C′|` moves) times the defect's reach, never above the larger
/// semi-axis: never less than the speed anywhere on the defect, at `f64`
/// as at `Interval`, and tight as it shrinks. On a spiric it is
/// [`crate::spiric_window_rate_bounds`] over the defect's reach: never
/// more than the whole period's sup, and finite on a defect clear of a
/// tangent pinch. On a line or a spline, [`speed_range`]'s sup.
pub fn turn_defect_speed_hi<T: SpanLocate>(
    carrier: &crate::Curve3<T>,
    t0: T,
    extent: Extent<T>,
) -> SupSpeed<T> {
    let (m, reach) = turn_defect(t0, extent);
    match Kind::of(carrier) {
        Kind::Conic(c) => SupSpeed::new((c.speed_held(m) + c.speed_hi() * reach).min(c.speed_hi())),
        Kind::Spiric {
            major,
            minor,
            offset,
        } => SupSpeed::new(crate::spiric_window_rate_bounds((major, minor, offset), m, reach).0),
        kind => kind.range().1,
    }
}

/// An inf on `|C′|` over the turn defect of the window that starts at
/// `t0` and spans `extent` ([`turn_defect`]), [`turn_defect_speed_hi`]'s
/// twin. On a conic it is the speed at the defect's middle less the larger
/// semi-axis times the defect's reach, never below the smaller semi-axis:
/// never more than the speed anywhere on the defect, and tight as it
/// shrinks. On every other kind, [`speed_range`]'s inf (a spiric's minor
/// radius, over its whole period).
pub fn turn_defect_speed_lo<T: SpanLocate>(
    carrier: &crate::Curve3<T>,
    t0: T,
    extent: Extent<T>,
) -> InfSpeed<T> {
    let (m, reach) = turn_defect(t0, extent);
    match Kind::of(carrier) {
        Kind::Conic(c) => InfSpeed::new((c.speed_held(m) - c.speed_hi() * reach).max(c.speed_lo())),
        kind => kind.range().0,
    }
}

/// **The rate a turn is read at over its defect**: [`turn_defect_speed_lo`]
/// and [`turn_defect_speed_hi`] as the turn rule's [`Bounded`] pair, the
/// one spelling certification's check 2, `ConicArc::of` and the cone
/// ellipse door share.
pub fn turn_defect_rate<T: SpanLocate>(
    carrier: &crate::Curve3<T>,
    t0: T,
    extent: Extent<T>,
) -> Bounded<T> {
    Bounded::new(
        turn_defect_speed_lo(carrier, t0, extent),
        turn_defect_speed_hi(carrier, t0, extent),
    )
}

/// A bound on `|C″(t)|` over every `t`, read as a Lipschitz constant of
/// `C′` across any window, which is what a second-order bound along the
/// curve needs: zero on a line, a conic's larger semi-axis, a spiric's
/// [`crate::spiric_period_rate_bounds`], a spline's
/// [`crate::NurbsCurve3::accel_upper_bound`] (infinite where a crease
/// lets `C′` jump; certified at `Interval`, poison where its statement
/// refuses; at `f64` the same expression in round-to-nearest, as
/// [`speed_range`]'s).
pub fn accel_hi<T: SpanLocate>(carrier: &crate::Curve3<T>) -> T {
    Kind::of(carrier).accel_hi()
}

/// A bound on the carrier's curvature over every `t`: zero on a line,
/// `speed_hi / speed_lo²` on a conic, and `|C″|/|C′|²` from
/// [`accel_hi`] and [`speed_range`]'s floor on a spiric (its minor
/// radius) and a spline (the curvature `|C′ × C″|/|C′|³` is at most
/// that). A floor not above zero is read as zero, so a spline whose
/// floor certifies nothing (a cusp, or a hodograph near the origin,
/// whose `f64` floor can read negative), or with a crease, bounds no
/// finite curvature: infinite or NaN at `f64`, poison at `Interval` for
/// a crease. At `Interval` a floor whose enclosure straddles zero gives
/// an enclosure unbounded above, `[x, +∞]`, which reads as a length
/// ([`geom_core::is_finite_length`]) but bounds nothing above.
pub fn curvature_hi<T: SpanLocate>(carrier: &crate::Curve3<T>) -> T {
    Kind::of(carrier).curvature_hi()
}

/// **A conic's rates from its semi-axes**, as stored, in either order and
/// sign: the one home of a conic's speed and curvature bounds
/// (`geom_brep`'s `Conic` reads them here).
#[derive(Clone, Copy, Debug)]
pub struct SemiAxes<T> {
    semi_u: T,
    semi_v: T,
}

impl<T: Real> SemiAxes<T> {
    /// The semi-axes along `u_ref` and `axis × u_ref`.
    pub fn new(semi_u: T, semi_v: T) -> Self {
        Self { semi_u, semi_v }
    }

    /// The least speed `|C′|`: the smaller semi-axis magnitude.
    pub fn speed_lo(self) -> T {
        crate::Curve3::semi_axes_by_size(self.semi_u, self.semi_v).1
    }

    /// The greatest speed `|C′|`, and a bound on `|C″|`: the larger
    /// semi-axis magnitude.
    pub fn speed_hi(self) -> T {
        crate::Curve3::semi_axes_by_size(self.semi_u, self.semi_v).0
    }

    /// A bound on the curvature, `speed_hi / speed_lo²`: the tightest
    /// bend, at the ends of the larger semi-axis.
    pub fn curvature_hi(self) -> T {
        self.speed_hi() / self.speed_lo().powi(2)
    }

    /// The speed `|C′(θ)|` at `theta`.
    pub fn speed_at(self, theta: T) -> T {
        let (sin, cos) = theta.sin_cos();
        ((self.semi_u * sin).powi(2) + (self.semi_v * cos).powi(2)).sqrt()
    }

    /// A bound on the speed `|C′(θ)|` over the span `(t0, t1)`: `sin θ`
    /// and `cos θ` move by at most the half-width `h` from their values
    /// at the span's middle, so `|sin θ| ≤ min(1, |sin m| + h)` and
    /// likewise `cos`; held to [`Self::speed_hi`].
    pub fn speed_over(self, (t0, t1): (T, T)) -> T {
        let h = (t1 - t0).abs() * T::from_f64(0.5);
        let (sin, cos) = crate::mid_param(t0, t1).sin_cos();
        let s = (sin.abs() + h).min(T::one());
        let c = (cos.abs() + h).min(T::one());
        ((self.semi_u * s).powi(2) + (self.semi_v * c).powi(2))
            .sqrt()
            .min(self.speed_hi())
    }

    /// [`Self::speed_at`] held to the semi-axes, so a circle's is its
    /// radius exactly at every scalar: the reading [`speed_at`] and the
    /// turn defect's bounds share.
    fn speed_held(self, theta: T) -> T {
        self.speed_at(theta)
            .max(self.speed_lo())
            .min(self.speed_hi())
    }
}

/// A carrier read by what bounds its rates: a conic by its semi-axes.
enum Kind<'a, T: Real> {
    Line(Vec3<T>),
    Conic(SemiAxes<T>),
    Spiric { major: T, minor: T, offset: T },
    Nurbs(&'a crate::NurbsCurve3<T>),
}

impl<'a, T: SpanLocate> Kind<'a, T> {
    fn of(carrier: &'a crate::Curve3<T>) -> Self {
        match carrier {
            crate::Curve3::Line { dir, .. } => Self::Line(*dir),
            crate::Curve3::Circle { radius, .. } => Self::Conic(SemiAxes::new(*radius, *radius)),
            crate::Curve3::Ellipse { semi_u, semi_v, .. } => {
                Self::Conic(SemiAxes::new(*semi_u, *semi_v))
            }
            crate::Curve3::Spiric {
                major_radius,
                minor_radius,
                offset,
                ..
            } => Self::Spiric {
                major: *major_radius,
                minor: *minor_radius,
                offset: *offset,
            },
            crate::Curve3::Nurbs(n) => Self::Nurbs(n),
        }
    }

    fn range(&self) -> (InfSpeed<T>, SupSpeed<T>) {
        let (lo, hi) = match *self {
            Self::Line(dir) => (dir.norm(), dir.norm()),
            Self::Conic(c) => (c.speed_lo(), c.speed_hi()),
            Self::Spiric {
                major,
                minor,
                offset,
            } => (
                minor,
                crate::spiric_period_rate_bounds(major, minor, offset).0,
            ),
            Self::Nurbs(n) => return (n.speed_lower_bound(), n.speed_upper_bound()),
        };
        (InfSpeed::new(lo), SupSpeed::new(hi))
    }

    /// The greatest speed of a line or a conic; `None` on a spiric or a
    /// spline.
    fn closed_hi(&self) -> Option<T> {
        match self {
            Self::Line(_) | Self::Conic(_) => Some(self.range().1.get()),
            Self::Spiric { .. } | Self::Nurbs(_) => None,
        }
    }

    fn accel_hi(&self) -> T {
        match *self {
            Self::Line(_) => T::zero(),
            Self::Conic(c) => c.speed_hi(),
            Self::Spiric {
                major,
                minor,
                offset,
            } => crate::spiric_period_rate_bounds(major, minor, offset).1,
            Self::Nurbs(n) => n.accel_upper_bound(),
        }
    }

    fn curvature_hi(&self) -> T {
        match *self {
            Self::Line(_) => T::zero(),
            Self::Conic(c) => c.curvature_hi(),
            Self::Spiric { .. } | Self::Nurbs(_) => {
                self.accel_hi() / self.range().0.get().max(T::zero()).powi(2)
            }
        }
    }
}
