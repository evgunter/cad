//! **What an evaluation scalar declares about itself as a LANE**: its
//! name, and how a bracket it carries reads as one `f64` when a
//! refusal has to cross into the scalar-free document vocabulary.
//!
//! Every per-scalar capability seam — `MinClearanceLane`, `ShellLane`
//! — needs both, and each used to spell the name for itself; this is
//! the one home, so `"f64"`, `"Probe"`, `"Interval"`, `"Sym"` and
//! `"Dual"` are written once and a refusal that names a lane cannot
//! disagree with its neighbour about what the lane is called.
//!
//! # The bracket end, declared by the field and read by the lane
//!
//! A kernel refusal's number is a `T`; the document layer's refusal
//! vocabulary is scalar-free. The fold that crosses the seam declares
//! at EACH FIELD which end of a bracket is the honest witness
//! ([`BracketEnd`] — a refused wall thickness is its infimum, a
//! clearance two offsets NEED is its supremum), and the lane says what
//! an end means for its own scalar: a point scalar has one end, a
//! bracket scalar two, a wrapped scalar reads its base's. Neither half
//! decides anything — the number is displayed and tagged, never
//! compared — which is why no lane here needs a bracket bound: the
//! bracket scalar reads its own ends by name.

use geom_core::Real;

/// Which end of a bracket a folded refusal number reports. Declared
/// per field by the fold that crosses the seam; read per lane by
/// [`Lane::end`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BracketEnd {
    /// The bracket's infimum: the number at its smallest.
    Infimum,
    /// The bracket's supremum: the number at its largest.
    Supremum,
}

/// A scalar's lane identity (module docs).
pub trait Lane: Real {
    /// This lane's own name, for the refusal that names it.
    const NAME: &'static str;

    /// The `f64` this scalar reports for a refused number at the given
    /// bracket end. A point scalar answers its value for either end.
    fn end(x: Self, end: BracketEnd) -> f64;
}

impl Lane for f64 {
    const NAME: &'static str = "f64";

    fn end(x: Self, _end: BracketEnd) -> f64 {
        x
    }
}

/// The recording scalar is `f64` with a sink attached, so it carries
/// exactly what `f64` carries.
#[cfg(feature = "probe")]
impl Lane for geom_core::Probe {
    const NAME: &'static str = "Probe";

    fn end(x: Self, _end: BracketEnd) -> f64 {
        x.0
    }
}

/// The certified interval scalar has two ends, and reports the one the
/// field asked for.
#[cfg(feature = "interval")]
impl Lane for geom_core::Interval {
    const NAME: &'static str = "Interval";

    fn end(x: Self, end: BracketEnd) -> f64 {
        use geom_core::Bounds;
        match end {
            BracketEnd::Infimum => x.lo(),
            BracketEnd::Supremum => x.hi(),
        }
    }
}

/// The symbolic tier reads its base scalar's end: provenance is not a
/// number.
impl<T: Lane> Lane for geom_core::Sym<T>
where
    geom_core::Sym<T>: Real,
{
    const NAME: &'static str = "Sym";

    fn end(x: Self, end: BracketEnd) -> f64 {
        T::end(x.value, end)
    }
}

/// A dual reads its VALUE channel's end: the tangent is a derivative,
/// not a number a refusal reports.
impl<T: Lane> Lane for geom_core::Dual<T>
where
    geom_core::Dual<T>: Real,
{
    const NAME: &'static str = "Dual";

    fn end(x: Self, end: BracketEnd) -> f64 {
        T::end(x.value, end)
    }
}
