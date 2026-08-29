//! **The tightness ceiling** — the half of a bound-domination claim
//! that `bound >= truth` cannot state.
//!
//! # The idiom
//!
//! A row that certifies a bound against a sampled truth has two things
//! to say, and only one of them is usually written. `sup >= max` says
//! the bound is SOUND; it is monotone in the safe direction, so an
//! implementation returning `f64::MAX`, or one that lost a
//! cancellation, satisfies it forever. The row's *name* is normally
//! about the geometry the bound was written to enclose, and that is the
//! one thing domination never constrains.
//!
//! So a domination row states both sides:
//!
//! 1. [`Sup::truth_at_least`] / [`Meter::truth_at_least`] — the
//!    anti-vacuity floor, in the same spirit as
//!    [`crate::vacuity::Exposure`] but on a magnitude rather than a
//!    count: a fixture whose sampled truth collapsed satisfies every
//!    comparison below it for free.
//! 2. [`Sup::dominates`] — soundness, the half that was already there.
//! 3. [`Sup::within`] / [`Meter::gives_away_at_most`] — the ceiling.
//!
//! # The ceiling is measured per site, never shared
//!
//! **This module deliberately owns no constant.** A ceiling copied from
//! the row above is `memories/output-stability-as-justification.md`'s
//! shape rather than a fix, and a ceiling imposed by a shared helper
//! would be that defect at a higher altitude. Every caller passes a
//! ratio it measured on its own fixture, and says in `why` what it
//! measured.
//!
//! # `degenerate_at`: what makes a ceiling a guard
//!
//! A ceiling that sits ABOVE the scale at which the enclosure has
//! stopped being about the geometry cannot see the degradation it
//! names. The anchor is the fixture's own **whole-object box** — the
//! diagonal of the box containing the operands' control nets — because
//! an enclosure that has degenerated reports that box and nothing
//! smaller. It is derived from the geometry, not from today's output,
//! so requiring the ceiling to sit under it re-pins nothing.
//! [`Sup::within`] asserts that relation on every run, so a ceiling
//! that was never a guard is red rather than green.
//!
//! Some fixtures leave no room: where the degraded and the healthy
//! ratio are within a small factor of each other, **no honest ceiling
//! exists**, and the row says so in prose at the claim site rather than
//! asserting a number that separates nothing. That verdict is a
//! passing answer; a silently missing ceiling is not.
//!
//! # The asymmetry between the two directions
//!
//! A certified UPPER bound degenerates UPWARD, without limit, which is
//! why [`Sup`] needs the anchor. A certified LOWER bound — a meter —
//! degenerates DOWNWARD toward the value it reports when it gives up,
//! which in this tree is non-positive. Any give-away fraction strictly
//! below 1 therefore already excludes the degenerate answer, and
//! [`Meter`] takes no anchor for that reason rather than by oversight.

/// A certified UPPER bound (a sup, an envelope, a hull) against the
/// value a dense scan actually measured.
#[derive(Clone, Copy, Debug)]
pub struct Sup<'a> {
    claim: &'a str,
    bound: f64,
    truth: f64,
}

impl<'a> Sup<'a> {
    /// `claim` names the row's own obligation; it appears in every
    /// message this builder can produce.
    #[must_use]
    pub fn new(claim: &'a str, bound: f64, truth: f64) -> Self {
        Self {
            claim,
            bound,
            truth,
        }
    }

    /// The anti-vacuity floor on the SAMPLED side. `why` says what a
    /// truth below `floor` means about the fixture — which geometry
    /// stopped being exercised, not that the number moved.
    ///
    /// # Panics
    ///
    /// If the sampled truth is below `floor`.
    #[track_caller]
    pub fn truth_at_least(self, floor: f64, why: &str) -> Self {
        assert!(
            self.truth >= floor,
            "VACUOUS FIXTURE: {self} — the sampled truth is under this row's \
             floor of {floor:e}, so every comparison against it holds for \
             free; {why}"
        );
        self
    }

    /// Soundness: the certified bound is not below the sampled truth.
    ///
    /// # Panics
    ///
    /// If the bound is below the truth (or is NaN).
    #[track_caller]
    pub fn dominates(self) -> Self {
        assert!(
            self.bound >= self.truth,
            "UNSOUND: {self} — the certified bound is below a value that was \
             actually sampled"
        );
        self
    }

    /// The ceiling: `bound <= ratio * truth + extra`.
    ///
    /// `extra` is for the rows whose truth approaches zero, where a
    /// ratio measures the machinery's own floor instead of the
    /// enclosure; pass `0.0` for a pure ratio. `degenerate_at` is the
    /// whole-object box scale (module docs) — the ceiling must sit
    /// under it or it is not a guard.
    ///
    /// # Panics
    ///
    /// If the ceiling is at or above `degenerate_at`, or if the bound
    /// exceeds the ceiling.
    #[track_caller]
    pub fn within(self, ratio: f64, extra: f64, degenerate_at: f64, why: &str) {
        let ceiling = ratio.mul_add(self.truth, extra);
        assert!(
            ceiling < degenerate_at,
            "CEILING IS NOT A GUARD: {self} — the ceiling {ratio}x + {extra:e} \
             admits {ceiling:e}, at or above {degenerate_at:e}, the scale at \
             which this enclosure has degenerated to the whole object. A \
             ceiling above that passes the very degradation it names; {why}"
        );
        assert!(
            self.bound <= ceiling,
            "CEILING: {self} — over this row's ceiling of {ratio}x the sampled \
             truth plus {extra:e} (= {ceiling:e}); {why}"
        );
    }
}

impl core::fmt::Display for Sup<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{}: certified {:e} against a sampled truth of {:e}, a ratio of {:e}",
            self.claim,
            self.bound,
            self.truth,
            self.bound / self.truth
        )
    }
}

/// A certified LOWER bound — a meter — against the minimum a dense
/// scan actually measured.
#[derive(Clone, Copy, Debug)]
pub struct Meter<'a> {
    claim: &'a str,
    bound: f64,
    truth: f64,
}

impl<'a> Meter<'a> {
    /// `claim` names the row's own obligation.
    #[must_use]
    pub fn new(claim: &'a str, bound: f64, truth: f64) -> Self {
        Self {
            claim,
            bound,
            truth,
        }
    }

    /// The anti-vacuity floor on the SAMPLED side: a meter's
    /// soundness clause is satisfiable by an arbitrarily small bound
    /// once the true minimum is near zero.
    ///
    /// # Panics
    ///
    /// If the sampled minimum is at or below `floor`.
    #[track_caller]
    pub fn truth_at_least(self, floor: f64, why: &str) -> Self {
        assert!(
            self.truth > floor,
            "VACUOUS FIXTURE: {self} — the sampled minimum is at or under this \
             row's floor of {floor:e}, so a meter of any size satisfies the \
             soundness clause; {why}"
        );
        self
    }

    /// The ceiling, in the form a lower bound takes: the meter may sit
    /// at most `fraction` of the way below the sampled minimum.
    ///
    /// # Panics
    ///
    /// If `fraction` is not in `(0, 1)` — a fraction of 1 or more
    /// admits the non-positive answer a meter reports when it gives up,
    /// so it would not be a guard — or if the meter gave away more.
    #[track_caller]
    pub fn gives_away_at_most(self, fraction: f64, why: &str) {
        assert!(
            fraction > 0.0 && fraction < 1.0,
            "CEILING IS NOT A GUARD: {self} — a give-away fraction of \
             {fraction} admits the non-positive answer this meter reports when \
             it gives up; {why}"
        );
        let floor = (1.0 - fraction) * self.truth;
        assert!(
            self.bound >= floor,
            "CEILING: {self} — the meter gave away {:.3}% of the sampled \
             minimum, over this row's admitted {:.3}% (which puts the floor at \
             {floor:e}); {why}",
            100.0 * (1.0 - self.bound / self.truth),
            100.0 * fraction
        );
    }
}

impl core::fmt::Display for Meter<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{}: metered {:e} against a sampled minimum of {:e}, giving away {:.3}%",
            self.claim,
            self.bound,
            self.truth,
            100.0 * (1.0 - self.bound / self.truth)
        )
    }
}

#[cfg(test)]
#[allow(clippy::panic, clippy::expect_used)]
mod tests {
    use super::*;
    use crate::panic_capture::caught;

    /// Every guard here exists to fire, so every row below drives one
    /// across its boundary in BOTH directions.
    #[test]
    fn the_truth_floor_fires_under_it_and_passes_at_it() {
        assert!(
            caught(|| {
                let _ = Sup::new("row", 2.0, 1.0).truth_at_least(1.0, "why");
            })
            .is_none()
        );
        let msg = caught(|| {
            Sup::new("row", 2.0, 0.5).truth_at_least(1.0, "the wiggle collapsed");
        })
        .expect("a truth under the floor must fire");
        assert!(msg.contains("VACUOUS FIXTURE"), "{msg}");
        assert!(msg.contains("the wiggle collapsed"), "{msg}");
        assert!(msg.contains("1e0"), "the floor is in the message: {msg}");
    }

    #[test]
    fn domination_fires_below_the_truth_and_passes_at_it() {
        assert!(
            caught(|| {
                let _ = Sup::new("row", 1.0, 1.0).dominates();
            })
            .is_none()
        );
        let msg = caught(|| {
            Sup::new("row", 0.9, 1.0).dominates();
        })
        .expect("an undercut must fire");
        assert!(msg.contains("UNSOUND"), "{msg}");
        assert!(msg.contains("ratio"), "the ratio is in the message: {msg}");
    }

    #[test]
    fn the_ceiling_fires_above_it_and_passes_at_it() {
        Sup::new("row", 3.0, 1.0).within(3.0, 0.0, 100.0, "why");
        let msg = caught(|| Sup::new("row", 3.5, 1.0).within(3.0, 0.0, 100.0, "the cancellation"))
            .expect("a bound over the ceiling must fire");
        assert!(msg.contains("CEILING:"), "{msg}");
        assert!(msg.contains("the cancellation"), "{msg}");
        assert!(
            msg.contains("3e0"),
            "the ceiling value is in the message: {msg}"
        );
    }

    #[test]
    fn extra_carries_the_rows_whose_truth_is_zero() {
        // The a = 0 shape: a pure ratio says nothing about a truth of
        // zero, and the additive term is what the row actually claims.
        Sup::new("row", 1.0, 0.0).within(1.0, 2.0, 100.0, "why");
        let msg = caught(|| Sup::new("row", 3.0, 0.0).within(1.0, 2.0, 100.0, "the floor"))
            .expect("must fire");
        assert!(msg.contains("CEILING:"), "{msg}");
    }

    #[test]
    fn a_ceiling_at_or_above_the_degeneracy_scale_is_itself_the_failure() {
        // The guard this module exists for: not that the bound is
        // large, but that the CEILING could never have caught it.
        let msg = caught(|| Sup::new("row", 1.0, 1.0).within(3.0, 0.0, 3.0, "the box diagonal"))
            .expect("a ceiling at the degeneracy scale must fire");
        assert!(msg.contains("CEILING IS NOT A GUARD"), "{msg}");
        assert!(msg.contains("the box diagonal"), "{msg}");
        // And it fires even though the bound itself is comfortably
        // under the ceiling — the row is red for its own shape.
        assert!(msg.contains("degenerated to the whole object"), "{msg}");
        Sup::new("row", 1.0, 1.0).within(3.0, 0.0, 3.001, "why");
    }

    #[test]
    fn the_meter_floor_fires_at_a_collapsed_minimum_and_passes_above_it() {
        assert!(
            caught(|| {
                let _ = Meter::new("row", 1.0, 2.0).truth_at_least(1.0, "why");
            })
            .is_none()
        );
        let msg = caught(|| {
            Meter::new("row", 1.0, 1.0).truth_at_least(1.0, "the curve stopped");
        })
        .expect("a minimum at the floor must fire");
        assert!(msg.contains("VACUOUS FIXTURE"), "{msg}");
        assert!(msg.contains("the curve stopped"), "{msg}");
    }

    #[test]
    fn the_meter_ceiling_fires_on_a_give_away_over_the_fraction() {
        Meter::new("row", 0.95, 1.0).gives_away_at_most(0.1, "why");
        let msg = caught(|| Meter::new("row", 0.85, 1.0).gives_away_at_most(0.1, "the hull"))
            .expect("a give-away over the fraction must fire");
        assert!(msg.contains("CEILING:"), "{msg}");
        assert!(msg.contains("15.000%"), "the measured give-away: {msg}");
        assert!(msg.contains("10.000%"), "the admitted give-away: {msg}");
    }

    #[test]
    fn a_give_away_fraction_of_one_or_more_is_itself_the_failure() {
        // The meter's form of the degeneracy check: at 1.0 the floor is
        // zero, which the non-positive give-up answer already meets.
        let msg = caught(|| Meter::new("row", 1.0, 1.0).gives_away_at_most(1.0, "the give-up arm"))
            .expect("a fraction of 1 must fire");
        assert!(msg.contains("CEILING IS NOT A GUARD"), "{msg}");
        assert!(msg.contains("the give-up arm"), "{msg}");
    }
}
