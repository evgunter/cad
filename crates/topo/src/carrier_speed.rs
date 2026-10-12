//! **A carrier's speed**, the metres one step of its parameter covers:
//! `|dir|` on a line, the radius on a circle, and
//! [`geom_brep::Conic::speed_at`] on an ellipse, whose semi-axes it reads
//! as magnitudes in either stored order. A spiric or spline carrier has
//! no closed form here and answers `None`.

use geom_brep::Conic;
use geom_core::Real;

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
