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
//! 2. [`Sup::dominates`] / [`Meter::dominates`] — soundness, the half
//!    that was already there.
//! 3. [`Sup::within`] / [`Meter::gives_away_at_most`] — the ceiling.
//!
//! # What makes a ceiling a guard
//!
//! **A measured degraded reading that it sits below.** Break the
//! mechanism the row names — in the way it would actually break, not by
//! multiplying the answer — measure the ratio the fixture then reports,
//! and put the ceiling under it. A ceiling with no such number beside
//! it is a formality: it has never been shown to separate anything.
//!
//! Nothing in this module can check that for you, because the degraded
//! reading is a second run of a modified tree. What it can do is make
//! the ceiling's *existence* structural — a chain that never reaches a
//! ceiling does not compile as a statement — and refuse a ceiling that
//! is obviously vacuous. That is [`Anchor`], and it is much weaker than
//! it sounds.
//!
//! # `Anchor` is a NECESSARY condition, and only that
//!
//! The whole-object box — the diagonal of the box containing the
//! operands' control nets — is a scale no useful enclosure reaches: an
//! enclosure that reports it has stopped being about the geometry. So a
//! ceiling at or above it admits everything and is worth refusing.
//!
//! It does **not** follow that a ceiling below it is a guard, and in
//! this tree it usually is not. Measured, the degraded readings sit
//! *well under* their boxes — 20.8× against a box admitting 45.7×,
//! 6.7× against 16.0×, 5.1× against 7.7× — so a ceiling can pass the
//! anchor comfortably and still be blind to the degradation it names.
//! One did: a `6.0` ceiling on a fixture whose box admitted 7.7× and
//! whose degraded reading was 5.1×.
//!
//! Some sites have no box scale that bounds them at all; they say so
//! with [`Anchor::Unbounded`] rather than passing a number they will
//! then document as irrelevant.
//!
//! # The ceiling is measured per site, never shared
//!
//! **This module deliberately owns no constant, in either direction.**
//! A ceiling copied from the row above is
//! `memories/output-stability-as-justification.md`'s shape rather than
//! a fix, and a ceiling imposed by a shared helper would be that defect
//! at a higher altitude. That includes the degeneracy anchors: a leaf
//! crate cannot see any caller's mechanism, so
//! [`Meter::gives_away_at_most`] takes the give-away at which the meter
//! has degenerated as an argument rather than assuming the one this
//! tree's meters happen to use.

/// What a ceiling is required to sit under — a necessary condition on
/// the ceiling, never evidence that it guards anything (module docs).
#[derive(Clone, Copy, Debug)]
pub enum Anchor<'a> {
    /// The diagonal of the box containing the operands' control nets,
    /// from [`control_net_box_diagonal`]. An enclosure that reports
    /// this has degenerated to the whole object.
    ObjectBox(f64),
    /// No box scale bounds this site's ceiling, and why not. The
    /// ceiling then rests entirely on its measured degraded reading,
    /// which is where the evidence always was.
    Unbounded(&'a str),
}

/// Diagonal of the axis-aligned box containing every control point of
/// the given nets, each net being one `Vec` of coordinates per channel.
///
/// # Panics
///
/// If any net has fewer than three channels. An empty or short net
/// would otherwise leave an infinite diagonal, and an infinite
/// [`Anchor::ObjectBox`] passes every ceiling silently — the vacuity
/// this module exists to make loud.
#[must_use]
pub fn control_net_box_diagonal(nets: &[&[Vec<f64>]]) -> f64 {
    let mut lo = [f64::INFINITY; 3];
    let mut hi = [f64::NEG_INFINITY; 3];
    for net in nets {
        assert!(
            net.len() >= 3,
            "a control net needs three coordinate channels to have a box; got {}. \
             An unbounded box admits every ceiling without saying so",
            net.len()
        );
        for (d, ch) in net.iter().enumerate().take(3) {
            assert!(
                !ch.is_empty(),
                "a control net channel with no points has no box"
            );
            for v in ch {
                lo[d] = lo[d].min(*v);
                hi[d] = hi[d].max(*v);
            }
        }
    }
    (0..3).map(|d| (hi[d] - lo[d]).powi(2)).sum::<f64>().sqrt()
}

/// A certified UPPER bound (a sup, an envelope, a hull) against the
/// value a dense scan actually measured.
///
/// The chain must reach [`Self::within`]: a `Sup` dropped as a
/// statement is a row that asserted the clauses it happened to reach
/// and then stopped, which is the shape this module exists to close.
#[derive(Clone, Copy, Debug)]
#[must_use = "a bound-domination chain that never reaches `within` states no ceiling, \
              which is the defect this module exists to close"]
pub struct Sup<'a> {
    claim: &'a str,
    bound: f64,
    truth: f64,
}

impl<'a> Sup<'a> {
    /// `claim` names the row's own obligation; it appears in every
    /// message this builder can produce.
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
    /// enclosure; pass `0.0` for a pure ratio. `why` carries the
    /// measured degraded reading this ceiling sits below — the thing
    /// that makes it a guard, which nothing here can check.
    ///
    /// # Panics
    ///
    /// If the ceiling is at or above an [`Anchor::ObjectBox`], or if
    /// the bound exceeds the ceiling.
    #[track_caller]
    pub fn within(self, ratio: f64, extra: f64, anchor: Anchor<'_>, why: &str) {
        let ceiling = ratio.mul_add(self.truth, extra);
        if let Anchor::ObjectBox(box_diagonal) = anchor {
            assert!(
                ceiling < box_diagonal,
                "CEILING IS NOT A GUARD: {self} — the ceiling {ratio}x + {extra:e} \
                 admits {ceiling:e}, at or above the whole-object box \
                 {box_diagonal:e}, which no useful enclosure reaches. Passing this \
                 check is necessary and not sufficient: the ceiling still has to \
                 sit under a measured degraded reading; {why}"
            );
        }
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
            "{}: certified {:e} against a sampled truth of {:e}",
            self.claim, self.bound, self.truth
        )?;
        // A ratio against a truth of zero is `inf`, which is the one
        // rung the additive `extra` exists to serve — report the excess
        // it actually claims about instead.
        if self.truth > 0.0 {
            write!(f, ", a ratio of {:e}", self.bound / self.truth)
        } else {
            write!(f, ", exceeding it by {:e}", self.bound - self.truth)
        }
    }
}

/// A certified LOWER bound — a meter — against the minimum a dense
/// scan actually measured.
///
/// The chain must reach [`Self::gives_away_at_most`], for the reason
/// [`Sup`] gives.
#[derive(Clone, Copy, Debug)]
#[must_use = "a meter chain that never reaches `gives_away_at_most` states no ceiling, \
              and a lower bound's loose direction is the one soundness cannot see"]
pub struct Meter<'a> {
    claim: &'a str,
    bound: f64,
    truth: f64,
}

impl<'a> Meter<'a> {
    /// `claim` names the row's own obligation.
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

    /// Soundness: the meter is not above the sampled minimum, up to
    /// `slack`. `why` states what `slack` covers — it is a difference
    /// between two evaluation paths for the same quantity, so it is a
    /// rounding budget and the caller is the only one who knows the
    /// magnitudes it is a budget for.
    ///
    /// # Panics
    ///
    /// If the meter exceeds the sampled minimum by more than `slack`.
    #[track_caller]
    pub fn dominates(self, slack: f64, why: &str) -> Self {
        assert!(
            self.bound - self.truth <= slack,
            "UNSOUND: {self} — the meter is above a speed that was actually \
             sampled, by {:e}, past this row's rounding budget of {slack:e}; {why}",
            self.bound - self.truth
        );
        self
    }

    /// The ceiling, in the form a lower bound takes: the meter may sit
    /// at most `fraction` of the way below the sampled minimum.
    ///
    /// `degenerate_fraction` is the give-away at which THIS meter has
    /// stopped being about the curve — the value it reports when it
    /// gives up, expressed as a fraction. A leaf crate cannot know it,
    /// so it is an argument rather than a baked-in `1.0`.
    ///
    /// # Panics
    ///
    /// If `fraction` is not positive, or is at or above
    /// `degenerate_fraction`, or if the meter gave away more.
    #[track_caller]
    pub fn gives_away_at_most(self, fraction: f64, degenerate_fraction: f64, why: &str) {
        assert!(
            fraction > 0.0 && fraction < degenerate_fraction,
            "CEILING IS NOT A GUARD: {self} — a give-away fraction of {fraction} \
             is not inside (0, {degenerate_fraction}), the range in which this \
             meter is still saying something about the curve; {why}"
        );
        let floor = (1.0 - fraction) * self.truth;
        assert!(
            self.bound >= floor,
            "CEILING: {self} — over this row's admitted {:.3}% (which puts the \
             floor at {floor:e}); {why}",
            100.0 * fraction
        );
    }
}

impl core::fmt::Display for Meter<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{}: metered {:e} against a sampled minimum of {:e}",
            self.claim, self.bound, self.truth
        )?;
        if self.truth > 0.0 {
            write!(
                f,
                ", giving away {:.3}%",
                100.0 * (1.0 - self.bound / self.truth)
            )
        } else {
            write!(f, " (a minimum of zero admits any meter)")
        }
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
            let _ = Sup::new("row", 2.0, 0.5).truth_at_least(1.0, "the wiggle collapsed");
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
            let _ = Sup::new("row", 0.9, 1.0).dominates();
        })
        .expect("an undercut must fire");
        assert!(msg.contains("UNSOUND"), "{msg}");
        assert!(msg.contains("ratio"), "the ratio is in the message: {msg}");
    }

    #[test]
    fn the_ceiling_fires_above_it_and_passes_at_it() {
        Sup::new("row", 3.0, 1.0).within(3.0, 0.0, Anchor::ObjectBox(100.0), "why");
        let msg = caught(|| {
            Sup::new("row", 3.5, 1.0).within(
                3.0,
                0.0,
                Anchor::ObjectBox(100.0),
                "the cancellation",
            );
        })
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
        Sup::new("row", 1.0, 0.0).within(1.0, 2.0, Anchor::ObjectBox(100.0), "why");
        let msg = caught(|| {
            Sup::new("row", 3.0, 0.0).within(1.0, 2.0, Anchor::ObjectBox(100.0), "the floor");
        })
        .expect("must fire");
        assert!(msg.contains("CEILING:"), "{msg}");
        // And the message does not report a ratio of `inf` for the one
        // rung the additive form exists to serve.
        assert!(!msg.contains("inf"), "{msg}");
        assert!(msg.contains("exceeding it by"), "{msg}");
    }

    #[test]
    fn a_ceiling_at_or_above_the_object_box_is_itself_the_failure() {
        // The necessary condition: not that the bound is large, but
        // that the CEILING admits a reading no enclosure survives.
        let msg = caught(|| {
            Sup::new("row", 1.0, 1.0).within(3.0, 0.0, Anchor::ObjectBox(3.0), "the box diagonal");
        })
        .expect("a ceiling at the box scale must fire");
        assert!(msg.contains("CEILING IS NOT A GUARD"), "{msg}");
        assert!(msg.contains("the box diagonal"), "{msg}");
        // It fires though the bound is comfortably under the ceiling —
        // the row is red for its own shape. And it says out loud that
        // passing is not evidence of anything.
        assert!(msg.contains("necessary and not sufficient"), "{msg}");
        Sup::new("row", 1.0, 1.0).within(3.0, 0.0, Anchor::ObjectBox(3.001), "why");
    }

    #[test]
    fn an_unbounded_anchor_skips_the_box_check_and_nothing_else() {
        // A site with no box scale still gets its ceiling enforced.
        Sup::new("row", 1.0, 1.0).within(3.0, 0.0, Anchor::Unbounded("no box here"), "why");
        let msg = caught(|| {
            Sup::new("row", 9.0, 1.0).within(
                3.0,
                0.0,
                Anchor::Unbounded("no box here"),
                "the envelope",
            );
        })
        .expect("the ceiling still applies");
        assert!(msg.contains("CEILING:"), "{msg}");
        assert!(!msg.contains("NOT A GUARD"), "{msg}");
    }

    #[test]
    fn the_box_diagonal_refuses_a_net_that_has_no_box() {
        let net: Vec<Vec<f64>> = vec![vec![0.0, 1.0], vec![0.0, 2.0], vec![0.0, 2.0]];
        let d = control_net_box_diagonal(&[&net]);
        assert!((d - 3.0).abs() < 1e-12, "{d}");
        let short: Vec<Vec<f64>> = vec![vec![0.0, 1.0], vec![0.0, 1.0]];
        let msg = caught(move || {
            let _ = control_net_box_diagonal(&[&short]);
        })
        .expect("a two-channel net must fire rather than return infinity");
        assert!(msg.contains("three coordinate channels"), "{msg}");
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
            let _ = Meter::new("row", 1.0, 1.0).truth_at_least(1.0, "the curve stopped");
        })
        .expect("a minimum at the floor must fire");
        assert!(msg.contains("VACUOUS FIXTURE"), "{msg}");
        assert!(msg.contains("the curve stopped"), "{msg}");
    }

    #[test]
    fn meter_soundness_fires_past_the_slack_and_passes_inside_it() {
        assert!(
            caught(|| {
                let _ = Meter::new("row", 1.0 + 1e-13, 1.0).dominates(1e-12, "rounding");
            })
            .is_none()
        );
        let msg = caught(|| {
            let _ = Meter::new("row", 1.1, 1.0).dominates(1e-12, "rounding");
        })
        .expect("a meter above the sampled minimum must fire");
        assert!(msg.contains("UNSOUND"), "{msg}");
        assert!(msg.contains("rounding budget"), "{msg}");
    }

    #[test]
    fn the_meter_ceiling_fires_on_a_give_away_over_the_fraction() {
        Meter::new("row", 0.95, 1.0).gives_away_at_most(0.1, 1.0, "why");
        let msg = caught(|| {
            Meter::new("row", 0.85, 1.0).gives_away_at_most(0.1, 1.0, "the hull");
        })
        .expect("a give-away over the fraction must fire");
        assert!(msg.contains("CEILING:"), "{msg}");
        assert!(msg.contains("15.000%"), "the measured give-away: {msg}");
        assert!(msg.contains("10.000%"), "the admitted give-away: {msg}");
    }

    #[test]
    fn a_give_away_fraction_at_the_callers_degenerate_value_is_itself_the_failure() {
        // The meter's form of the necessary condition, and the caller
        // supplies the degenerate value rather than this crate.
        let msg = caught(|| {
            Meter::new("row", 1.0, 1.0).gives_away_at_most(0.5, 0.5, "the give-up arm");
        })
        .expect("a fraction at the degenerate value must fire");
        assert!(msg.contains("CEILING IS NOT A GUARD"), "{msg}");
        assert!(msg.contains("the give-up arm"), "{msg}");
        // And the same fraction is fine against a meter that degenerates
        // further out — which is why it is an argument.
        Meter::new("row", 1.0, 1.0).gives_away_at_most(0.5, 1.0, "why");
    }
}
