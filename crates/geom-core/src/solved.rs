//! **A solved value the body stores, admitted by its check** (the crate
//! README, DL5.2, the paragraphs after the table).
//!
//! An `f64` search proposes a candidate where the solve has no closed
//! form (a crossing, a meeting, a root); [`admit`] runs the check that
//! the candidate's point reads on within the band, at the scalar the
//! body is built at, and hands back the admitted value or the refusal.
//! A caller cannot have the value without the residual, so what the
//! value claims is what was checked: this number reads on, never that
//! it is the true root.
//!
//! What the admitted value is at each scalar the door admits:
//!
//! - `f64`: the candidate, its point read on within the band;
//! - an enclosure scalar: the candidate as one point for the whole
//!   family, every member's point there read on within the band; where
//!   no one number reads on for every member, the refusal;
//! - `Sym`: an opaque unknown, which claims nothing
//!   ([`Real::from_computed`](crate::Real::from_computed)).
//!
//! A dual is not admitted: it does not certify ([`Certifies`]), and a
//! candidate carries no tangent to give it.
//!
//! ```compile_fail,E0277
//! use geom_core::{Band, Dual64, Margin, Tol};
//! let band = Band::linear(Tol::witness()).unwrap();
//! let _ = geom_core::solved::admit("row", 0.5, band, |t: Dual64| (Margin::of(t), ()));
//! ```

use core::fmt;

use crate::predicate::{Band, Indeterminate, Margin, Sign};
use crate::real::Certifies;

/// **Why [`admit`] refused a candidate**: its residual read definitely
/// past the band, or the band could not decide it.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Unadmitted {
    /// The candidate's residual (`row`) read definitely off: its point
    /// is not on within the band.
    Off {
        /// The check's predicate name.
        row: &'static str,
        /// The candidate the search proposed.
        candidate: f64,
    },
    /// The candidate's residual is in the band's escalation: neither on
    /// nor off at this tolerance.
    Undecided(Indeterminate),
}

impl fmt::Display for Unadmitted {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Off { row, candidate } => write!(
                f,
                "the solved value at parameter {candidate} was not admitted: its point reads \
                 definitely off within the band ({row}), so no one number stands for the \
                 solution there; move the geometry so the solve is better conditioned"
            ),
            Self::Undecided(why) => write!(
                f,
                "the solved value was not admitted: whether its point reads on is undecided \
                 at this tolerance ({why})"
            ),
        }
    }
}

/// **The one door a stored solved value enters `T` by**: the `f64`
/// `candidate` an `f64` search proposed, and the `residual` the site
/// checks it against, read at `T` at the candidate. The residual's
/// margin is decided against `band` under `row`; a reading in the
/// band's zero admits the candidate, with whatever else the residual
/// computed there (`W`: the point it read, say), and any other reading
/// refuses ([`Unadmitted`]).
///
/// # Errors
///
/// [`Unadmitted::Off`] where the residual reads definitely past the
/// band, [`Unadmitted::Undecided`] where it is in its escalation.
pub fn admit<T: Certifies, W>(
    row: &'static str,
    candidate: f64,
    band: Band,
    residual: impl FnOnce(T) -> (Margin<T>, W),
) -> Result<(T, W), Unadmitted> {
    let value = T::from_computed(candidate);
    let (margin, witness) = residual(value);
    match crate::k_stats::decide(row, margin, band) {
        Ok(Sign::Zero) => Ok((value, witness)),
        Ok(Sign::Positive | Sign::Negative) => Err(Unadmitted::Off { row, candidate }),
        Err(why) => Err(Unadmitted::Undecided(why)),
    }
}

#[cfg(test)]
mod rows;
