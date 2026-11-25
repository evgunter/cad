//! **A solved value the body stores, admitted by its check** (the crate
//! README, DL5.2, the paragraphs after the table).
//!
//! An `f64` search proposes a candidate where the solve has no closed
//! form (a crossing, a meeting, a root); [`decide_admit`] enters it at
//! the scalar the body is built at, runs the residual the caller hands
//! it there, decides that residual against the caller's band, and hands
//! back the admitted value or the refusal.
//!
//! What the door holds: the value exists only after the check ran on
//! it, its residual's margin read in the band's zero and the check
//! bounded how far another candidate it would admit could lie
//! ([`Reach`]). The value leaves with that bound ([`Admitted`]), so a
//! reader that decides where the value stands decides for every
//! candidate ([`decide_reaching`], [`Reach::bands`]) and not for the one
//! drawn. What it does not hold: that the residual measures the
//! candidate's point (a residual that ignores its argument admits
//! anything), that the reach the check states is one, or that the band
//! is the run's (all three are the caller's). So an admitted value
//! claims that its residual read on and that every candidate the check
//! admits lies within its reach, never that it is the true root.
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
//! let _ = geom_core::solved::decide_admit("row", 0.5, band, |t: Dual64| {
//!     geom_core::solved::Checked { residual: Margin::of(t), reach: 0.0, witness: () }
//! });
//! ```
//!
//! ```compile_fail,E0277
//! use geom_core::{Band, DualInterval, Margin, Tol};
//! let band = Band::linear(Tol::witness()).unwrap();
//! let _ = geom_core::solved::decide_admit("row", 0.5, band, |t: DualInterval| {
//!     geom_core::solved::Checked { residual: Margin::of(t), reach: 0.0, witness: () }
//! });
//! ```
//!
//! The same calls at the scalars that certify compile:
//!
//! ```
//! use geom_core::solved::{Checked, decide_admit};
//! use geom_core::{Band, Interval, Margin, Tol};
//! let band = Band::linear(Tol::witness()).unwrap();
//! let _ = decide_admit("row", 0.5, band, |t: f64| Checked {
//!     residual: Margin::of(t),
//!     reach: 0.0,
//!     witness: (),
//! });
//! let _ = decide_admit("row", 0.5, band, |t: Interval| Checked {
//!     residual: Margin::of(t),
//!     reach: 0.0,
//!     witness: (),
//! });
//! ```
//!
//! The door is one entry of this module; an entry for a value of another
//! shape (a fitted net and its check) sits beside it and returns the same
//! [`Admitted`], its bound the certificate's envelope. It is spelled
//! `decide_*` so the predicate censuses read the row each caller hands
//! it as decided, as they read any funnel's
//! (`test_utils::source::predicate_census`).

use crate::predicate::{Band, BandError, Decide, Indeterminate, Margin, Sign};
use crate::real::Certifies;

/// **An admitted value with what its check bounded.** Only a door of
/// this module makes one, and the value leaves it only beside the bound
/// ([`Self::into_parts`]): `B` is [`Reach`] for a number, and the
/// certificate's envelope for a fitted image.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Admitted<V, B> {
    value: V,
    bound: B,
}

impl<V, B> Admitted<V, B> {
    /// The value and the bound its check gave.
    pub fn into_parts(self) -> (V, B) {
        (self.value, self.bound)
    }

    /// What the check bounded.
    pub const fn bound(&self) -> &B {
        &self.bound
    }
}

/// **What a number's check bounded**, in metres, in each direction a
/// reader decides on: every candidate the check admits has its point
/// within `across` of what the residual measures against (a crossing's
/// face), and within `along` of the admitted candidate's point (for a
/// crossing parameter, along the edge).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Reach {
    across: f64,
    along: f64,
}

impl Reach {
    /// How far a candidate's point lies from what the residual measures
    /// against: the zero of the band the residual was decided in.
    #[must_use]
    pub const fn across(self) -> f64 {
        self.across
    }

    /// How far another admissible candidate's point lies from the
    /// admitted one's.
    #[must_use]
    pub const fn along(self) -> f64 {
        self.along
    }

    /// **The bands a reading taken at the admitted point is decided
    /// against, for a reader that takes a band and not a margin**: under
    /// the first a length decided past the band is past `band` at every
    /// candidate's point, a length moving by at most [`Self::along`]
    /// between them; under the second a length decided zero is zero at
    /// every candidate's too. The second is `None` where the reach is
    /// not inside the band's zero: no zero reading there is every
    /// candidate's.
    ///
    /// # Errors
    ///
    /// [`Band::new`]'s, for a reach that takes the escalation past the
    /// finite range.
    pub fn bands(self, band: Band) -> Result<(Band, Option<Band>), BandError> {
        let escalate = band.escalate() + self.along;
        let clear = Band::new(band.zero(), escalate)?;
        let on = if self.along < band.zero() {
            Some(Band::new(band.zero() - self.along, escalate)?)
        } else {
            None
        };
        Ok((clear, on))
    }
}

/// **A distance read at an admitted value's point, decided for every
/// candidate within `reach` of it**: past the band where `length` less
/// the reach is, zero where `length` and the reach together are, and
/// otherwise the band's escalation, a straddle of the two under the
/// same `row` where each decided ([`crate::k_stats::straddle`]: the two
/// bounds are one reading's, so the straddle is that reading's).
/// `length` is a distance, never negative.
///
/// # Errors
///
/// The band's escalation where the reading is not every candidate's.
pub fn decide_reaching<T: Decide>(
    row: &'static str,
    length: Margin<T>,
    reach: f64,
    band: Band,
) -> Result<Sign, Indeterminate> {
    let nearest = crate::k_stats::decide_reported(row, length.less_floored(reach), band)?;
    if nearest.sign == Sign::Positive {
        return Ok(Sign::Positive);
    }
    let farthest = crate::k_stats::decide_reported(row, length.less(-reach), band)?;
    if farthest.sign == nearest.sign {
        return Ok(nearest.sign);
    }
    Err(crate::k_stats::straddle(row, nearest, farthest, band))
}

/// **What a candidate's check read** ([`decide_admit`]): the residual's
/// margin at the candidate, how far along another candidate the check
/// would admit could lie from it ([`Reach::along`]), in metres, and
/// whatever else the check computed there (the point it read, say).
pub struct Checked<T, W> {
    /// The residual at the candidate, decided against the door's band.
    pub residual: Margin<T>,
    /// The check's bound on [`Reach::along`].
    pub reach: f64,
    /// What else the check computed at the candidate.
    pub witness: W,
}

/// **Why [`decide_admit`] refused a candidate**: its residual read
/// definitely past the band, the band could not decide it, or the check
/// bounded no reach. The site that asked words the refusal; the door
/// knows only which of the three it was.
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
    /// The candidate's residual read on, and the check gave no finite
    /// bound on how far another candidate it would admit could lie.
    Unbounded {
        /// The check's predicate name.
        row: &'static str,
        /// The candidate the search proposed.
        candidate: f64,
    },
}

/// **The one door a stored solved number enters `T` by**: the `f64`
/// `candidate` an `f64` search proposed, and the `check` the site holds
/// it to, read at `T` at the candidate. The check's residual is decided
/// against `band` under `row`; a reading in the band's zero, with a
/// finite reach, admits the candidate with that bound ([`Reach`]: the
/// band's zero across, the check's reach along) and whatever else the
/// check computed there (`W`); any other reading refuses
/// ([`Unadmitted`]).
///
/// # Errors
///
/// [`Unadmitted::Off`] where the residual reads definitely past the
/// band, [`Unadmitted::Undecided`] where it is in its escalation,
/// [`Unadmitted::Unbounded`] where the check bounded no reach.
// `!(r >= 0)` is deliberate: a NaN reach bounds nothing.
#[allow(clippy::neg_cmp_op_on_partial_ord)]
pub fn decide_admit<T: Certifies, W>(
    row: &'static str,
    candidate: f64,
    band: Band,
    check: impl FnOnce(T) -> Checked<T, W>,
) -> Result<(Admitted<T, Reach>, W), Unadmitted> {
    let value = T::from_computed(candidate);
    let Checked {
        residual,
        reach,
        witness,
    } = check(value);
    match crate::k_stats::decide(row, residual, band) {
        Ok(Sign::Zero) if !(reach.is_finite() && reach >= 0.0) => {
            Err(Unadmitted::Unbounded { row, candidate })
        }
        Ok(Sign::Zero) => Ok((
            Admitted {
                value,
                bound: Reach {
                    across: band.zero(),
                    along: reach,
                },
            },
            witness,
        )),
        Ok(Sign::Positive | Sign::Negative) => Err(Unadmitted::Off { row, candidate }),
        Err(why) => Err(Unadmitted::Undecided(why)),
    }
}

#[cfg(test)]
mod rows;
