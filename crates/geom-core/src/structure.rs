//! **Structure decisions** (C6): whether a value holds a property for
//! every value it stands for, asked of the scalar's own seam
//! ([`SpanLocate::structure_positive`]) rather than decided against a
//! band. At a point scalar each reads the value channel; at `Interval`
//! the whole bracket, so a wide value answers `false` wherever its
//! members could take both answers. A spline's span selection is one
//! reader, and any door that must not read a direction or a step it
//! cannot certify is another.

use crate::spline::SpanLocate;

/// Whether `b` is above `a` for every value they stand for, as a
/// structure decision (C6): at points, `a < b` on their value channels,
/// so no arithmetic is formed at a point scalar; otherwise
/// [`SpanLocate::structure_positive`] of `b − a`.
pub fn structure_ascends<T: SpanLocate>(a: T, b: T) -> bool {
    match (a.point_value(), b.point_value()) {
        (Some(a), Some(b)) => a < b,
        _ => (b - a).structure_positive(),
    }
}

/// Whether `x` is nonzero for every value it stands for, as a structure
/// decision (C6): at a point, `x != 0` on its value channel; otherwise
/// [`SpanLocate::structure_positive`] of `x` or of `−x`. Poison is not
/// nonzero.
pub fn structure_nonzero<T: SpanLocate>(x: T) -> bool {
    match x.point_value() {
        Some(v) => v != 0.0,
        None => x.structure_positive() || (-x).structure_positive(),
    }
}
