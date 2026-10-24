//! **The measurement door's predicate** (`docs/DESIGN.md`
//! `[tolerance.measure]`): what a caller asks of a volume's enclosure,
//! as closed data, and how one round's enclosure answers it.
//!
//! A deciding predicate ([`Settle::Sign`], [`Settle::Relation`],
//! [`Settle::Contains`]) is a named funnel decision: it settles when the
//! enclosure decides it, refines while the enclosure straddles its
//! threshold, stops when the enclosure lies wholly in the band, and is
//! recorded at every round it is asked. [`Settle::Resolution`] is a
//! sizing: it decides nothing, is compared rather than decided, and is
//! not recorded.

use geom_core::{Band, Decide, Indeterminate, Margin, Real, Sign};
use quantity::Length;

use super::{RoleNames, ShellRole, VolumeEnclosure};

/// **What a measurement is asked**, at the door
/// ([`super::measure_to`]). There is no default: every caller states
/// one.
#[derive(Clone, Copy, Debug)]
pub enum Settle<T: Real> {
    /// The volume's sign, as the role it gives a closed surface:
    /// positive is [`ShellRole::Outer`], negative [`ShellRole::Void`].
    Sign(SignNames),
    /// The volume against a bound of the body's scalar, as an assertion
    /// states it. The margin is a volume, metered over the surface area
    /// to a mean boundary displacement and decided against the band.
    Relation {
        /// The relation the volume is claimed to stand in.
        op: RelOp,
        /// The bound, a volume.
        bound: T,
        /// The funnel predicate the decision is recorded under.
        name: &'static str,
    },
    /// The volume within `[lo, hi]`: both ends of the claim at once, each
    /// a [`Settle::Relation`] under `name`.
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

/// **A relation an assertion states** (`[assertions]`: `≤`, `≥`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RelOp {
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
pub enum Verdict {
    /// [`Settle::Sign`]'s: the role the sign gives.
    Sign(ShellRole),
    /// [`Settle::Relation`]'s and [`Settle::Contains`]'s: whether the
    /// claim holds.
    Holds(bool),
}

/// **Why a measurement stopped** (`[tolerance.measure]`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stop {
    /// A deciding predicate settled.
    Settled(Verdict),
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
}

/// **A relation read off a volume enclosure**: `V op bound`, the margin
/// `bound − V` at each end of the enclosure, metered over the surface
/// area and decided under `name` ([`relation_step_by`]).
pub(crate) fn relation_step<T: Decide>(
    e: VolumeEnclosure<T>,
    op: RelOp,
    bound: T,
    name: &'static str,
    band: Band,
) -> Step {
    relation_step_by(e, op, false, |volume| {
        geom_core::k_stats::decide_reported(
            name,
            Margin::over_lever(bound - volume, e.surface_area),
            band,
        )
        .map(|decided| decided.sign)
    })
}

/// **A relation read off a volume enclosure, by a stated decider**:
/// `V op bound`, where `decide(v)` is the sign of the claim's margin
/// `bound − V` at the volume end `v`. The one reading of a relation; its
/// deciders differ only in the lane they record on and how they spell
/// the margin (the funnel's [`relation_step`], and the backstop's
/// operand gate on the invariant lane).
///
/// Each end is decided as the volume's least favourable reading for the
/// side it settles: the claim `V ≤ b` holds when the margin at the HIGH
/// end of the volume is definitely positive, and fails when the margin
/// at its LOW end is definitely negative (`≥` the mirror). The end that
/// can make the claim hold is decided first and the other only where it
/// does not; an `exact` enclosure (both ends one value) is decided once.
pub(crate) fn relation_step_by<T: Decide>(
    e: VolumeEnclosure<T>,
    op: RelOp,
    exact: bool,
    decide: impl Fn(T) -> Result<Sign, Indeterminate>,
) -> Step {
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
    let second = if exact { first } else { decide(second) };
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
