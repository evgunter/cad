//! A carrier's held knots and weights as the knot algebra's plan
//! constructors read them (C6's interim): the one funnel, apart from
//! the certification files, through which a carrier's knot operation
//! takes their `f64` vectors.

use super::algebra::KnotAlgebraError;
use super::knot_values::Knots;
use super::knots::{KnotVector, SplineError};
use super::locate::SpanLocate;
use super::weights::Weights;

/// **A carrier's held knots and weights as the plan constructors read
/// them**: their `f64` vectors, which exist where every knot and weight
/// is a point (C6's interim; S5 moves the constructors onto the held
/// values). The one place the knot algebra takes those vectors, so a
/// carrier's knot operation refuses a wide input typed, at the run or
/// weight the door names, rather than reaching a loud interim read.
///
/// # Errors
///
/// [`KnotAlgebraError::Structure`] carrying
/// [`SplineError::CarrierKnotNotAPoint`] at the first run whose value is
/// wider than a point, or [`SplineError::CarrierWeightNotAPoint`] at the
/// first such weight.
pub fn point_inputs<'a, T: SpanLocate>(
    knots: &'a Knots<T>,
    weights: &'a Weights<T>,
) -> Result<(&'a KnotVector, &'a [f64]), KnotAlgebraError> {
    let kv = point_knots(knots)?;
    if let Some(index) = weights.first_wide() {
        return Err(KnotAlgebraError::Structure(
            SplineError::CarrierWeightNotAPoint { index },
        ));
    }
    Ok((kv, weights.weights_f64_interim()))
}

/// [`point_inputs`]' knot half: a carrier's held knots as the plan
/// constructors read them, for a net whose weights reach the
/// constructor another way (a surface's column).
///
/// # Errors
///
/// [`KnotAlgebraError::Structure`] carrying
/// [`SplineError::CarrierKnotNotAPoint`] at the first run whose value is
/// wider than a point.
pub fn point_knots<T: SpanLocate>(knots: &Knots<T>) -> Result<&KnotVector, KnotAlgebraError> {
    if let Some(index) = knots.first_wide_run() {
        return Err(KnotAlgebraError::Structure(
            SplineError::CarrierKnotNotAPoint { index },
        ));
    }
    Ok(knots.structure_f64_interim())
}
