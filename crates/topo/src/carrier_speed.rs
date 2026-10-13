//! **A carrier's speed**, the metres one step of its parameter covers:
//! `|dir|` on a line, the radius on a circle, and
//! [`geom_brep::Conic::speed_at`] on an ellipse, whose semi-axes it reads
//! as magnitudes in either stored order. A spiric or spline carrier has
//! no closed form here and answers `None`; [`speed_range`] bounds every
//! kind.

use geom_brep::Conic;
use geom_core::{InfSpeed, Real, SpanLocate, SupSpeed};

/// The speed `|C′(t)|` at `t`.
pub(crate) fn speed_at<T: Real>(carrier: &geom::Curve3<T>, t: T) -> Option<T> {
    match *carrier {
        geom::Curve3::Line { dir, .. } => Some(dir.norm()),
        geom::Curve3::Circle { radius, .. } => Some(radius),
        _ => Conic::of(carrier).map(|c| c.speed_at(t)),
    }
}

/// The greatest speed over every `t`, [`speed_at`]'s bound: an
/// ellipse's larger semi-axis.
pub(crate) fn speed_hi<T: Real>(carrier: &geom::Curve3<T>) -> Option<T> {
    match *carrier {
        geom::Curve3::Line { dir, .. } => Some(dir.norm()),
        geom::Curve3::Circle { radius, .. } => Some(radius),
        _ => Conic::of(carrier).map(|c| c.speed_hi()),
    }
}

/// Bounds on `|C′(t)|` over every `t`, on every kind: exact on a line
/// and a circle, an ellipse's semi-axes, a spiric's minor radius `r`
/// below (`|C′|² = r²(1 + (d/f)²·sin²v)`) and [`geom::spiric_rate_bounds`]
/// over a whole period above, and a spline's control-net bounds, whose
/// floor may be zero or poison.
pub(crate) fn speed_range<T: SpanLocate>(carrier: &geom::Curve3<T>) -> (InfSpeed<T>, SupSpeed<T>) {
    let (lo, hi) = match carrier {
        geom::Curve3::Line { dir, .. } => (dir.norm(), dir.norm()),
        geom::Curve3::Circle { radius, .. } => (*radius, *radius),
        geom::Curve3::Ellipse { major, minor, .. } => {
            (major.abs().min(minor.abs()), major.abs().max(minor.abs()))
        }
        geom::Curve3::Spiric {
            major_radius,
            minor_radius,
            offset,
            ..
        } => (
            *minor_radius,
            geom::spiric_rate_bounds(
                *minor_radius,
                *offset,
                (*major_radius - *minor_radius, *major_radius + *minor_radius),
                T::one(),
            )
            .0,
        ),
        geom::Curve3::Nurbs(n) => {
            return (n.speed_lower_bound(), n.speed_upper_bound());
        }
    };
    (InfSpeed::new(lo), SupSpeed::new(hi))
}
