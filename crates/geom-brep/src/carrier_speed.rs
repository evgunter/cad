//! **A carrier's speed**, the metres one step of its parameter covers.
//! [`speed_range`] bounds it on every kind and is its one spelling: a
//! line's `|dir|`, a conic's semi-axes ([`Conic`], which
//! reads them as magnitudes in either stored order), a spiric's and a
//! spline's bounds. [`speed_at`] and [`speed_hi`] answer the line and
//! the conics alone, `None` for a spiric or spline carrier, which their
//! callers read as a door that stays.

use crate::Conic;
use geom_core::{InfSpeed, Real, SpanLocate, SupSpeed, Vec3};

/// The speed `|C′(t)|` at `t`: a line's `|dir|`, a conic's
/// [`Conic::speed_at`] held to its semi-axes, so a circle's is its
/// radius exactly at every scalar.
pub fn speed_at<T: SpanLocate>(carrier: &geom::Curve3<T>, t: T) -> Option<T> {
    match Kind::of(carrier) {
        kind @ Kind::Line(_) => kind.closed_hi(),
        Kind::Conic(c) => Some(c.speed_at(t).max(c.speed_lo()).min(c.speed_hi())),
        Kind::Spiric { .. } | Kind::Nurbs(_) => None,
    }
}

/// The greatest speed over every `t`, [`speed_at`]'s bound: an
/// ellipse's larger semi-axis.
pub fn speed_hi<T: SpanLocate>(carrier: &geom::Curve3<T>) -> Option<T> {
    Kind::of(carrier).closed_hi()
}

/// [`speed_hi`] and [`curvature_hi`] together, `None` past
/// [`speed_hi`]'s kinds.
pub fn speed_and_curvature_hi<T: SpanLocate>(carrier: &geom::Curve3<T>) -> Option<(T, T)> {
    let kind = Kind::of(carrier);
    Some((kind.closed_hi()?, kind.curvature_hi()?))
}

/// Bounds on `|C′(t)|` over every `t`, on every kind: exact on a line
/// and a circle, an ellipse's semi-axes, a spiric's minor radius `r`
/// below (`|C′|² = r²(1 + (d/f)²·sin²v)`) and [`geom::spiric_rate_bounds`]
/// over a whole period above, and a spline's control-net bounds
/// ([`geom::NurbsCurve3::speed_lower_bound`], whose floor may be zero
/// or poison). A rational spline's bounds are certified at `Interval`;
/// at `f64` they are the same expressions in round-to-nearest, not a
/// bound at extreme weights.
pub fn speed_range<T: SpanLocate>(carrier: &geom::Curve3<T>) -> (InfSpeed<T>, SupSpeed<T>) {
    Kind::of(carrier).range()
}

/// A bound on `|C″(t)|` over every `t`: zero on a line, a conic's
/// larger semi-axis, a spiric's [`geom::spiric_period_rate_bounds`];
/// `None` on a spline, which has no bound here at every scalar.
pub fn accel_hi<T: SpanLocate>(carrier: &geom::Curve3<T>) -> Option<T> {
    Kind::of(carrier).accel_hi()
}

/// A bound on the carrier's curvature over every `t`: zero on a line,
/// [`Conic::curvature_hi`] on a conic, and `|C″|/|C′|²` from
/// [`accel_hi`] and the minor radius on a spiric (the curvature
/// `|C′ × C″|/|C′|³` is at most that); `None` on a spline.
pub fn curvature_hi<T: SpanLocate>(carrier: &geom::Curve3<T>) -> Option<T> {
    Kind::of(carrier).curvature_hi()
}

/// A carrier read by what bounds its rates: a conic by its frame.
enum Kind<'a, T: Real> {
    Line(Vec3<T>),
    Conic(Conic<T>),
    Spiric { major: T, minor: T, offset: T },
    Nurbs(&'a geom::NurbsCurve3<T>),
}

impl<'a, T: SpanLocate> Kind<'a, T> {
    fn of(carrier: &'a geom::Curve3<T>) -> Self {
        match carrier {
            geom::Curve3::Line { dir, .. } => Self::Line(*dir),
            geom::Curve3::Circle { .. } | geom::Curve3::Ellipse { .. } => Conic::of(carrier)
                .map_or_else(
                    || unreachable!("Conic::of reads every circle and ellipse"),
                    Self::Conic,
                ),
            geom::Curve3::Spiric {
                major_radius,
                minor_radius,
                offset,
                ..
            } => Self::Spiric {
                major: *major_radius,
                minor: *minor_radius,
                offset: *offset,
            },
            geom::Curve3::Nurbs(n) => Self::Nurbs(n),
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
                geom::spiric_period_rate_bounds(major, minor, offset).0,
            ),
            Self::Nurbs(n) => return (n.speed_lower_bound(), n.speed_upper_bound()),
        };
        (InfSpeed::new(lo), SupSpeed::new(hi))
    }

    /// The greatest speed of a line or a conic; `None` on a spiric or a
    /// spline, which [`speed_at`]'s callers read as a door that stays.
    fn closed_hi(&self) -> Option<T> {
        match self {
            Self::Line(_) | Self::Conic(_) => Some(self.range().1.get()),
            Self::Spiric { .. } | Self::Nurbs(_) => None,
        }
    }

    fn accel_hi(&self) -> Option<T> {
        match *self {
            Self::Line(_) => Some(T::zero()),
            Self::Conic(c) => Some(c.speed_hi()),
            Self::Spiric {
                major,
                minor,
                offset,
            } => Some(geom::spiric_period_rate_bounds(major, minor, offset).1),
            Self::Nurbs(_) => None,
        }
    }

    fn curvature_hi(&self) -> Option<T> {
        match *self {
            Self::Line(_) => Some(T::zero()),
            Self::Conic(c) => Some(c.curvature_hi()),
            Self::Spiric { minor, .. } => Some(self.accel_hi()? / minor.powi(2)),
            Self::Nurbs(_) => None,
        }
    }
}
