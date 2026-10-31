//! **The measurement door's predicate** (`docs/DESIGN.md`
//! `[tolerance.measure]`): what a caller asks of a volume's enclosure,
//! as closed data, and how one round's enclosure answers it.
//!
//! A deciding predicate ([`Settle::Sign`], [`Settle::AtMost`],
//! [`Settle::AtLeast`], [`Settle::Contains`]) is a named funnel decision: it settles when the
//! enclosure decides it, refines while the enclosure straddles its
//! threshold, stops when the enclosure lies wholly in the band, and is
//! recorded at every round it is asked. [`Settle::Resolution`] is a
//! sizing: it decides nothing, is compared rather than decided, and is
//! not recorded.

use geom_core::{Band, Decide, Indeterminate, Margin, Real, Sign};
use quantity::Length;

use super::{MassProperties, RoleNames, ShellRole, VolumeEnclosure};

/// **What a measurement is asked**, at the door
/// ([`super::measure_to`]). There is no default: every caller states
/// one.
#[derive(Clone, Copy, Debug)]
pub enum Settle<T: Real> {
    /// The volume's sign, as the role it gives a closed surface:
    /// positive is [`ShellRole::Outer`], negative [`ShellRole::Void`].
    Sign(SignNames),
    /// The volume at most a bound of the body's scalar, as an assertion
    /// states it. The margin `bound − V` is a volume, metered over the
    /// surface area to a mean boundary displacement and decided against
    /// the band.
    AtMost {
        /// The bound, a volume.
        bound: T,
        /// The funnel predicate the decision is recorded under.
        name: &'static str,
    },
    /// The volume at least a bound of the body's scalar: the mirror of
    /// [`Settle::AtMost`].
    AtLeast {
        /// The bound, a volume.
        bound: T,
        /// The funnel predicate the decision is recorded under.
        name: &'static str,
    },
    /// The volume within `[lo, hi]`: both ends of the claim at once, a
    /// [`Settle::AtLeast`] of `lo` and a [`Settle::AtMost`] of `hi` under
    /// `name`.
    Contains {
        /// The least volume claimed.
        lo: T,
        /// The greatest volume claimed.
        hi: T,
        /// The funnel predicate the decisions are recorded under.
        name: &'static str,
    },
    /// The mean boundary displacement a face's enclosure may span: each
    /// face refines until its own width, `width(flux)/(3·area)`, is at
    /// most this length. Never finer than ε, which is the most the model
    /// defines a measure to.
    Resolution(Length),
}

/// **Which side of a bound a relation claims** (`≤`, `≥`): the reading
/// [`Settle::AtMost`] and [`Settle::AtLeast`] share.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum RelOp {
    /// At most the bound.
    Le,
    /// At least the bound.
    Ge,
}

/// **The names a sign is decided under**: the walk's own sums under
/// `walk`, and the round re-derived in interval arithmetic under
/// `certified` ([`super::Round::certify`]).
#[derive(Clone, Copy, Debug)]
pub struct SignNames {
    /// The names on the walk's sums.
    pub walk: RoleNames,
    /// The names on the interval re-derivation.
    pub certified: RoleNames,
}

/// **What a deciding predicate decided.**
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SettleVerdict {
    /// [`Settle::Sign`]'s: the role the sign gives.
    Sign(ShellRole),
    /// [`Settle::AtMost`]'s, [`Settle::AtLeast`]'s and
    /// [`Settle::Contains`]'s: whether the claim holds.
    Holds(bool),
}

/// **Why a measurement stopped** (`[tolerance.measure]`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stop {
    /// A deciding predicate settled.
    Settled(SettleVerdict),
    /// Every face met the resolution.
    Met,
    /// Refinement stopped paying on every face still short of the
    /// question.
    Floor,
    /// A face still short of the question reached the work wall.
    Wall,
    /// The enclosure lies wholly in the band of a deciding predicate's
    /// threshold, where no narrower enclosure decides it.
    Band,
}

impl Stop {
    /// **The stop as its word**: its variant's name in snake case, the
    /// one spelling every surface that names a stop reads (Python's
    /// `MassMeasurement.stop`, which the binding census holds to this
    /// list and to the variants' order).
    #[must_use]
    pub const fn word(self) -> &'static str {
        match self {
            Self::Settled(_) => "settled",
            Self::Met => "met",
            Self::Floor => "floor",
            Self::Wall => "wall",
            Self::Band => "band",
        }
    }
}

/// **What a measurement reached** (`[tolerance.measure]`): the
/// narrowest enclosure each face reached, summed, and why the
/// measurement stopped. A number is a [`Settle::Resolution`]'s
/// enclosure, read with its stop: [`Stop::Met`] is the resolution
/// reached, and [`Stop::Floor`] or [`Stop::Wall`] the bracket where it
/// went unmet, at the width the enclosure states.
#[derive(Clone, Copy, Debug)]
pub struct MassMeasurement<T: Real> {
    /// The volume and area enclosures: midpoints and half-widths
    /// ([`MassProperties`]).
    pub enclosure: MassProperties<T>,
    /// Why the measurement stopped.
    pub stop: Stop,
}

impl<T: Real> MassMeasurement<T> {
    /// **The enclosure as a number**: where the measurement met its
    /// resolution ([`Stop::Met`]), the enclosure every face met it at;
    /// otherwise the stop it reached instead. A reader taking the
    /// midpoint as THE volume reads it through this, so a bracket that
    /// stopped short is never read as a number met. Python's
    /// `MassMeasurement.met()` is this.
    ///
    /// # Errors
    ///
    /// [`Unmet`], carrying the stop, where the stop is anything but
    /// [`Stop::Met`].
    pub fn met(&self) -> Result<MassProperties<T>, Unmet> {
        match self.stop {
            Stop::Met => Ok(self.enclosure),
            stop => Err(Unmet { stop }),
        }
    }

    /// **What a deciding predicate decided**: the verdict
    /// [`Stop::Settled`] carries, and `None` on every other stop.
    /// Python's `MassMeasurement.verdict` is this.
    #[must_use]
    pub fn verdict(&self) -> Option<SettleVerdict> {
        match self.stop {
            Stop::Settled(verdict) => Some(verdict),
            _ => None,
        }
    }
}

/// **A number asked of a measurement that did not meet its
/// resolution** ([`MassMeasurement::met`]): the stop it reached instead.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Unmet {
    /// The stop the measurement reached instead of [`Stop::Met`].
    pub stop: Stop,
}

impl core::fmt::Display for Unmet {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "the measurement did not meet its resolution: it stopped at `{}`, so its \
             enclosure is a bracket and not a number met. Recourse: read the bracket, or \
             ask for a coarser resolution",
            self.stop.word()
        )
    }
}

impl std::error::Error for Unmet {}

/// **One round's answer to a deciding predicate** over a volume
/// enclosure.
#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Step {
    /// The predicate is decided.
    Settled(bool),
    /// The enclosure straddles the threshold: a narrower one may decide
    /// it.
    Refine,
    /// The enclosure lies wholly in the band.
    Band,
    /// A margin was poisoned: no enclosure decides it.
    Poisoned(Indeterminate),
    /// The claim names no volume at all: a [`Settle::Contains`] whose
    /// least volume definitely exceeds its greatest.
    Empty,
}

/// **A relation read off a volume enclosure**: `V op bound`, the margin
/// `bound − V` at each end of the enclosure, metered over the surface
/// area and decided under `name`.
///
/// Each end is decided as the volume's least favourable reading for the
/// side it settles: the claim `V ≤ b` holds when the margin at the HIGH
/// end of the volume is definitely positive, and fails when the margin
/// at its LOW end is definitely negative (`≥` the mirror). The end that
/// can make the claim hold is decided first and the other only where it
/// does not.
pub(crate) fn relation_step<T: Decide>(
    e: VolumeEnclosure<T>,
    op: RelOp,
    bound: T,
    name: &'static str,
    band: Band,
) -> Step {
    let decide = |volume: T| {
        geom_core::k_stats::decide_reported(
            name,
            Margin::over_lever(bound - volume, e.surface_area),
            band,
        )
        .map(|decided| decided.sign)
    };
    let poisoned = |d: &Result<Sign, Indeterminate>| match d {
        Err(cause) if cause.margin.is_invalid() => Some(*cause),
        _ => None,
    };
    // The margin at the volume's high end (`low`, the margin's low end)
    // settles `≤` as held; at its low end (`high`), `≥` as held.
    let (first, second) = match op {
        RelOp::Le => (e.volume_hi, e.volume_lo),
        RelOp::Ge => (e.volume_lo, e.volume_hi),
    };
    let first = decide(first);
    if let Some(cause) = poisoned(&first) {
        return Step::Poisoned(cause);
    }
    let holds = |d: &Result<Sign, Indeterminate>| match op {
        RelOp::Le => d.as_ref().ok() == Some(&Sign::Positive),
        RelOp::Ge => d.as_ref().ok() == Some(&Sign::Negative),
    };
    if holds(&first) {
        return Step::Settled(true);
    }
    let second = decide(second);
    if let Some(cause) = poisoned(&second) {
        return Step::Poisoned(cause);
    }
    let (low, high) = match op {
        RelOp::Le => (first.ok(), second.ok()),
        RelOp::Ge => (second.ok(), first.ok()),
    };
    let below = low == Some(Sign::Positive);
    let above = high == Some(Sign::Negative);
    match op {
        RelOp::Le if above => Step::Settled(false),
        RelOp::Ge if below => Step::Settled(false),
        _ => {
            // Undecided at both ends: wholly in the band where neither end
            // reads a definite side, else straddling it.
            let in_band = |s: Option<Sign>| s.is_none_or(|s| s == Sign::Zero);
            if in_band(low) && in_band(high) {
                Step::Band
            } else {
                Step::Refine
            }
        }
    }
}

/// **A containment read off a volume enclosure**: `lo ≤ V ≤ hi`, each end
/// a [`relation_step`]. It fails where either end fails, holds where both
/// hold, refines where either straddles, and is otherwise in the band.
pub(crate) fn contains_step<T: Decide>(
    e: VolumeEnclosure<T>,
    lo: T,
    hi: T,
    name: &'static str,
    band: Band,
) -> Step {
    // `hi − lo`, metered over the surface area as the relations' margins
    // are: definitely negative, and no volume lies in the claim.
    match geom_core::k_stats::decide_reported(
        name,
        Margin::over_lever(hi - lo, e.surface_area),
        band,
    ) {
        Ok(decided) if decided.sign == Sign::Negative => return Step::Empty,
        Err(cause) if cause.margin.is_invalid() => return Step::Poisoned(cause),
        _ => {}
    }
    let at_least = relation_step(e, RelOp::Ge, lo, name, band);
    let at_most = relation_step(e, RelOp::Le, hi, name, band);
    match (at_least, at_most) {
        (Step::Poisoned(cause), _) | (_, Step::Poisoned(cause)) => Step::Poisoned(cause),
        (Step::Settled(false), _) | (_, Step::Settled(false)) => Step::Settled(false),
        (Step::Settled(true), Step::Settled(true)) => Step::Settled(true),
        (Step::Refine, _) | (_, Step::Refine) => Step::Refine,
        _ => Step::Band,
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use geom_core::{Band, Tol};

    use super::{RelOp, Step, VolumeEnclosure, contains_step, relation_step};

    fn band() -> Band {
        Band::linear(Tol::witness()).unwrap()
    }

    /// A unit-area enclosure `[lo, hi]`: its ends are mean boundary
    /// displacements as they stand.
    fn e(lo: f64, hi: f64) -> VolumeEnclosure<f64> {
        VolumeEnclosure {
            volume_lo: lo,
            volume_hi: hi,
            surface_area: 1.0,
        }
    }

    /// **A relation settles when definite, refines while it straddles,
    /// and stops in the band**, read at each of its operators.
    #[test]
    fn a_relation_settles_refines_or_stops_in_the_band() {
        let eps = Tol::witness().eps();
        let (b, band) = (1.0, band());
        for (what, enclosure, le, ge) in [
            (
                "wholly below",
                e(0.5, 0.6),
                Step::Settled(true),
                Step::Settled(false),
            ),
            (
                "wholly above",
                e(1.4, 1.5),
                Step::Settled(false),
                Step::Settled(true),
            ),
            ("straddling", e(0.5, 1.5), Step::Refine, Step::Refine),
            (
                "in the band",
                e(b - 0.1 * eps, b + 0.1 * eps),
                Step::Band,
                Step::Band,
            ),
        ] {
            assert_eq!(
                relation_step(enclosure, RelOp::Le, b, "t", band),
                le,
                "{what}: ≤"
            );
            assert_eq!(
                relation_step(enclosure, RelOp::Ge, b, "t", band),
                ge,
                "{what}: ≥"
            );
        }
    }

    /// **A containment holds where both ends hold and fails where either
    /// fails**, refining where an end straddles.
    #[test]
    fn a_containment_reads_both_ends() {
        let band = band();
        let read = |lo, hi| contains_step(e(lo, hi), 1.0, 2.0, "t", band);
        assert_eq!(read(1.2, 1.8), Step::Settled(true), "inside");
        assert_eq!(read(2.2, 2.4), Step::Settled(false), "above");
        assert_eq!(read(0.2, 0.4), Step::Settled(false), "below");
        assert_eq!(read(0.5, 1.5), Step::Refine, "straddling the low end");
        assert_eq!(read(0.5, 2.5), Step::Refine, "straddling both");
    }

    /// **A poisoned enclosure decides nothing**: no narrower one would.
    #[test]
    fn a_poisoned_enclosure_is_no_answer() {
        assert!(matches!(
            relation_step(e(f64::NAN, f64::NAN), RelOp::Le, 1.0, "t", band()),
            Step::Poisoned(_)
        ));
    }
}
