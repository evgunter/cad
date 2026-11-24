//! **A solved value the body stores, admitted by its check** (the crate
//! README, DL5.2, the paragraphs after the table).
//!
//! An `f64` search proposes a candidate where the solve has no closed
//! form (a crossing, a meeting, a root); [`decide_admit`] enters it at
//! the scalar the body is built at, runs the residual the caller hands
//! it there, decides that residual against the caller's band, and hands
//! back the admitted value or the refusal.
//!
//! What the door holds: the value exists only after the residual ran on
//! it and its margin read in the band's zero. What it does not hold:
//! that the residual measures the candidate's point (a residual that
//! ignores its argument admits anything), or that the band is the run's
//! (both are the caller's). So an admitted value claims that its
//! residual read on, never that it is the true root.
//!
//! What the admitted value is at each scalar the door admits:
//!
//! - `f64`: the candidate, its residual read on within the band;
//! - an enclosure scalar: the candidate as one point for the whole
//!   family, every member's residual there read on within the band;
//!   where no one number reads on for every member, the refusal;
//! - `Sym`: an opaque unknown, which claims nothing
//!   ([`Real::from_computed`](crate::Real::from_computed)).
//!
//! A dual is not admitted: it does not certify ([`Certifies`]), and a
//! candidate carries no tangent to give it.
//!
//! ```compile_fail,E0277
//! use geom_core::{Band, Dual64, Margin, Tol};
//! let band = Band::linear(Tol::witness()).unwrap();
//! let _ = geom_core::solved::decide_admit("row", 0.5, band, |t: Dual64| (Margin::of(t), ()));
//! ```
//!
//! ```compile_fail,E0277
//! use geom_core::{Band, DualInterval, Margin, Tol};
//! let band = Band::linear(Tol::witness()).unwrap();
//! let _ = geom_core::solved::decide_admit("row", 0.5, band, |t: DualInterval| {
//!     (Margin::of(t), ())
//! });
//! ```
//!
//! The same calls at the scalars that certify compile:
//!
//! ```
//! use geom_core::{Band, Interval, Margin, Tol};
//! let band = Band::linear(Tol::witness()).unwrap();
//! let _ = geom_core::solved::decide_admit("row", 0.5, band, |t: f64| (Margin::of(t), ()));
//! let _ = geom_core::solved::decide_admit("row", 0.5, band, |t: Interval| (Margin::of(t), ()));
//! ```
//!
//! The door is one entry of this module; an entry for a value of another
//! shape (a fitted net and its check) sits beside it. It is spelled
//! `decide_*` so the predicate censuses read the row each caller hands
//! it as decided, as they read any funnel's
//! (`test_utils::source::predicate_census`).

use crate::predicate::{Band, Indeterminate, Margin, Sign};
use crate::real::Certifies;

/// **Why [`decide_admit`] refused a candidate**: its residual read
/// definitely past the band, or the band could not decide it. The site
/// that asked words the refusal; the door knows only which of the two
/// it was.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Unadmitted {
    /// The candidate's residual (`row`) read definitely past the band.
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
pub fn decide_admit<T: Certifies, W>(
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
