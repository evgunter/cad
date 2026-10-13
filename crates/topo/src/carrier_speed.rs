//! **A carrier's speed**, the metres one step of its parameter covers.
//! [`speed_range`] bounds it on every kind and is its one spelling: a
//! line's `|dir|`, a conic's semi-axes ([`geom_brep::Conic`], which
//! reads them as magnitudes in either stored order), a spiric's and a
//! spline's bounds. [`speed_at`] and [`speed_hi`] answer the line and
//! the conics alone, `None` for a spiric or spline carrier, which their
//! callers read as a door that stays.

use geom_brep::Conic;
use geom_core::{InfSpeed, Real, SpanLocate, SupSpeed, Vec3};

/// The speed `|C′(t)|` at `t`: the bound itself where [`speed_range`]
/// is exact (a line, a circle).
pub(crate) fn speed_at<T: SpanLocate>(carrier: &geom::Curve3<T>, t: T) -> Option<T> {
    match carrier {
        geom::Curve3::Line { .. } | geom::Curve3::Circle { .. } => {
            Some(speed_range(carrier).0.get())
        }
        geom::Curve3::Ellipse { .. } => Conic::of(carrier).map(|c| c.speed_at(t)),
        geom::Curve3::Spiric { .. } | geom::Curve3::Nurbs(_) => None,
    }
}

/// The greatest speed over every `t`, [`speed_at`]'s bound: an
/// ellipse's larger semi-axis.
pub(crate) fn speed_hi<T: SpanLocate>(carrier: &geom::Curve3<T>) -> Option<T> {
    match carrier {
        geom::Curve3::Spiric { .. } | geom::Curve3::Nurbs(_) => None,
        _ => Some(speed_range(carrier).1.get()),
    }
}

/// Bounds on `|C′(t)|` over every `t`, on every kind: exact on a line
/// and a circle, an ellipse's semi-axes, a spiric's minor radius `r`
/// below (`|C′|² = r²(1 + (d/f)²·sin²v)`) and [`geom::spiric_rate_bounds`]
/// over a whole period above, and a spline's control-net bounds
/// ([`geom::NurbsCurve3::speed_lower_bound`], whose floor may be zero
/// or poison). A rational spline's bounds are certified at `Interval`;
/// at `f64` they are the same expressions in round-to-nearest, not a
/// bound at extreme weights.
pub(crate) fn speed_range<T: SpanLocate>(carrier: &geom::Curve3<T>) -> (InfSpeed<T>, SupSpeed<T>) {
    let (lo, hi) = match Kind::of(carrier) {
        Kind::Line(dir) => (dir.norm(), dir.norm()),
        Kind::Conic(c) => (c.speed_lo(), c.speed_hi()),
        Kind::Spiric {
            major,
            minor,
            offset,
        } => (
            minor,
            geom::spiric_period_rate_bounds(major, minor, offset).0,
        ),
        Kind::Nurbs(n) => return (n.speed_lower_bound(), n.speed_upper_bound()),
    };
    (InfSpeed::new(lo), SupSpeed::new(hi))
}

/// A bound on `|C″(t)|` over every `t`: zero on a line, a conic's
/// larger semi-axis, a spiric's [`geom::spiric_period_rate_bounds`];
/// `None` on a spline, which has no bound here at every scalar.
pub(crate) fn accel_hi<T: SpanLocate>(carrier: &geom::Curve3<T>) -> Option<T> {
    match Kind::of(carrier) {
        Kind::Line(_) => Some(T::zero()),
        Kind::Conic(c) => Some(c.speed_hi()),
        Kind::Spiric {
            major,
            minor,
            offset,
        } => Some(geom::spiric_period_rate_bounds(major, minor, offset).1),
        Kind::Nurbs(_) => None,
    }
}

/// A bound on the carrier's curvature over every `t`: zero on a line,
/// [`Conic::curvature_hi`] on a conic, and `|C″|/|C′|²` from
/// [`accel_hi`] and the minor radius on a spiric (the curvature
/// `|C′ × C″|/|C′|³` is at most that); `None` on a spline.
pub(crate) fn curvature_hi<T: SpanLocate>(carrier: &geom::Curve3<T>) -> Option<T> {
    match Kind::of(carrier) {
        Kind::Line(_) => Some(T::zero()),
        Kind::Conic(c) => Some(c.curvature_hi()),
        Kind::Spiric { minor, .. } => Some(accel_hi(carrier)? / minor.powi(2)),
        Kind::Nurbs(_) => None,
    }
}

/// A carrier read by what bounds its rates: a conic by its frame.
enum Kind<'a, T: Real> {
    Line(Vec3<T>),
    Conic(Conic<T>),
    Spiric { major: T, minor: T, offset: T },
    Nurbs(&'a geom::NurbsCurve3<T>),
}

impl<'a, T: Real> Kind<'a, T> {
    fn of(carrier: &'a geom::Curve3<T>) -> Self {
        match carrier {
            geom::Curve3::Line { dir, .. } => Self::Line(*dir),
            geom::Curve3::Circle {
                center,
                axis,
                radius,
                u_ref,
            } => Self::Conic(Conic::circle(*center, *axis, *radius, *u_ref)),
            geom::Curve3::Ellipse {
                center,
                axis,
                major,
                minor,
                u_ref,
            } => Self::Conic(Conic {
                center: *center,
                axis: *axis,
                u_ref: *u_ref,
                major: *major,
                minor: *minor,
            }),
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
}
