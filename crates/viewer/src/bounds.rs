//! **How far a field can move before something breaks**: the locally
//! valid range of one editable number, found by probing.
//!
//! # What the question is
//!
//! A user about to change a dimension wants to know the room they have
//! — "if I set any number between these two, nothing new goes wrong;
//! immediately outside either, something does". This module answers
//! that for one slot or one document parameter, as a value.
//!
//! **"Nothing NEW"** is the load-bearing word. A document that is
//! already failing somewhere does not have to be repaired before its
//! other fields can be reasoned about: validity here is measured
//! against a BASELINE — the set of nodes failing at the value the field
//! has right now ([`Verdict`]) — so the current value is valid by
//! construction, in a broken document exactly as in a healthy one, and
//! a bound marks where a failure appears that was not there before.
//!
//! # This is a PROBE, and says so
//!
//! The answer is found by sampling: step outward from the current value
//! until a sample goes bad, then bisect between the last good and the
//! first bad one. That is the whole method, and its limits are
//! consequences of it rather than bugs in it:
//!
//! * **Validity is not monotone.** A field can be valid, invalid, and
//!   valid again — a hole that closes and reopens, a fillet that
//!   collides and clears. What this reports is the nearest boundary the
//!   sampling could SEE. A valid island beyond a bound is not reported;
//!   an invalid sliver narrower than the sampling stride can be stepped
//!   over.
//! * **A bound is a bracket, not a number.** Each side reports the
//!   furthest value found valid and the nearest found invalid, and the
//!   true boundary is between them. Rendering the pair rather than a
//!   single number is deliberate: a single number would claim a
//!   precision the search did not establish.
//! * **The reach is finite.** A field with no failure anywhere out to
//!   the search's furthest sample reports [`Bound::Open`] carrying how
//!   far it looked — never "unbounded", which the search cannot know.
//!
//! Stating all three is the point. The alternative — reporting a tidy
//! interval and letting the reader assume it was derived — is the kind
//! of confident wrong answer this codebase's fail-loud posture exists
//! to keep out.
//!
//! # The certified answer this method stands in for
//!
//! Sampling is a guess, and this kernel can do better than guess when
//! it is given time: `editor_core::range::certified_range` replays the
//! document with ONE field widened to an interval and reports, per
//! side, whether every value in it decides what the current value
//! decides — not "these values worked" but "nothing in this box builds
//! anything else". Its three kernel-side doors are built, and each is
//! a door this module cannot open for itself:
//!
//! * **A widened binding reaches evaluation as one.**
//!   `editor_core::analysis::param_env_over` binds an axis as
//!   `nominal + [lo, hi]` in the scalar's own arithmetic, where
//!   `Doc::param_env` binds `T::from_f64` of the nominal alone and
//!   every binding is therefore degenerate.
//! * **A node SLOT has a name to widen.**
//!   `editor_core::range::RangeField::Slot` names one, and the
//!   widening stays a property of the QUERY rather than of the
//!   document: the name is minted in a clone the query derives for
//!   itself, never as an interval-valued literal in the caller's
//!   recipe.
//! * **An INDETERMINATE interval decision means subdivide.**
//!   `editor_core::drive`'s leaf classifier answers
//!   `LeafVerdict::Bisect` for every escalation but the ratified
//!   terminal sliver, which it refuses as `SliverTerminal` carrying
//!   the escalation's predicate — `<unnamed>` where the escalation
//!   carries none, so the arm is typed but the culprit is not always
//!   named. The SEPARATE question of what a certified lane owes when
//!   a value goes non-real is open and is not this door:
//!   `work/props/certified-lane-non-real-contract-audit.md`.
//!
//! **What keeps THIS module the interactive answer is cost**, and a
//! drive's cost is measured rather than argued: `editor_core::range`
//! carries the table (seconds per leaf on the corpus documents,
//! nothing certified at a budget a caller can afford) beside the
//! instruction to re-take it rather than trust it. **Nothing in
//! either tree reds if those numbers drift** — they are a measurement
//! at a tree, not an invariant — so this paragraph names where they
//! live instead of copying them, and a reader who needs the figure
//! reads it there and re-takes it.
//!
//! So the certified range is an ON-DEMAND query whose answer arrives
//! later and REPLACES this reading rather than merging with it.
//! **The two claims are not nested and neither contains the other.**
//! A certified range is a subset of the LOCALLY-VALID RANGE — the
//! thing this module is trying to report — because "does anything
//! decide differently" is strictly more than "does anything new
//! fail". It is NOT a subset of the BRACKET this module reports: the
//! probe reports the furthest value it SAMPLED and found valid, which
//! on a field with a nearby boundary sits well outside what a drive
//! can prove, and `editor_core::range`'s own docs carry the
//! counterexample (an 8 mm slot certified to `1.6e-8` against a
//! furthest valid sample of `3.9e-6`). So a panel shows both or names
//! which one it is showing, and never draws one inside the other.
//!
//! **A consumer reads four arms, not a yes or no.**
//! `editor_core::range::RangeSide` is `Certified` (the seed's edge —
//! never "unbounded"), `NewFailure` and `DecisionFlip` (a bracket
//! around a boundary, with the driver's evidence), and
//! `Indeterminate` (the driver could not decide). Rendering
//! `Indeterminate` as an edge is the specific mistake to avoid, and
//! it is sharper than "unknown": a definite failure reaches that arm
//! today priced `Budget` at the depth floor, so "raise the budget" is
//! honest advice only above the floor
//! (`work/props/coincidence-zone-priced-budget-at-the-floor.md`).
//!
//! Nothing in `crates/viewer` asks for one on a user's behalf yet —
//! the affordance is
//! `work/chrome/certify-affordance-on-the-bounds-panel.md`, and
//! `crates/viewer/tests/docm9_range_vs_probe.rs` (the `interval`
//! feature) is where the two answers are measured against each other.
//!
//! What lets that arrive without disturbing anything here:
//! [`BoundsProbe`] evaluates nothing itself, so the oracle is
//! replaceable without the panel noticing.
//!
//! # Why it is a resumable state machine
//!
//! [`BoundsProbe`] never evaluates anything. It answers "which value
//! should be tried next" ([`BoundsProbe::next`]) and takes the verdict
//! back ([`BoundsProbe::observe`]); the caller supplies the meaning of
//! valid. Two things follow. The search logic — the part that can be
//! wrong about brackets, strides and termination — is testable against
//! synthetic validity functions, with no document and no geometry in
//! sight. And the driving is the caller's: [`probe`] runs it against a
//! closure inline today, and running it a step per frame or on a worker
//! thread later changes nothing here.
//!
//! # Cost
//!
//! One document evaluation per sample, bounded by
//! [`BoundsProbe::MAX_SAMPLES`] over both directions together. The
//! session drives it against the landed evaluation as the memo, so each
//! sample re-runs the edited node's downstream cone rather than the
//! whole recipe.
//!
//! Module kind: **vocabulary** — it names no driver type and no
//! `app`-only crate (`crates/viewer/README.md`, Module boundaries).

use std::collections::BTreeSet;

use pncad::document::{Evaluation, NodeResult, RecipeNodeId};
use pncad::quantity::UnitDef;

/// The set of recipe nodes that FAILED — the comparison "is this value
/// as good as the one we started from" is made on.
///
/// Failed only, never poisoned: a poisoned node is the recorded
/// consequence of an ancestor's failure, so counting it would make one
/// failure register as many and make the verdict depend on how deep the
/// recipe happens to be below the break.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Verdict(BTreeSet<RecipeNodeId>);

impl Verdict {
    /// The failing nodes of an evaluation.
    pub fn of(eval: &Evaluation<f64>) -> Self {
        Self(
            eval.nodes
                .iter()
                .filter(|(_, result)| matches!(result, NodeResult::Failed(_)))
                .map(|(id, _)| *id)
                .collect(),
        )
    }

    /// A verdict from an explicit set — for a caller (or a test)
    /// naming the failures directly.
    pub fn from_nodes(nodes: impl IntoIterator<Item = RecipeNodeId>) -> Self {
        Self(nodes.into_iter().collect())
    }

    /// Whether this verdict introduces no failure `baseline` did not
    /// already have.
    ///
    /// A SUBSET test, not equality: a value that FIXES an existing
    /// failure is not a reason to stop searching — the question is
    /// where things get worse.
    pub fn no_worse_than(&self, baseline: &Self) -> bool {
        self.0.is_subset(&baseline.0)
    }

    /// The failing nodes.
    pub fn nodes(&self) -> &BTreeSet<RecipeNodeId> {
        &self.0
    }
}

/// One direction's answer.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Bound {
    /// No new failure appeared anywhere out to `probed`, which is as
    /// far as the search looked. Not "unbounded" — the search cannot
    /// establish that, and saying it would be a claim about values it
    /// never tried.
    Open {
        /// The furthest value sampled, all of them valid.
        probed: f64,
    },
    /// A boundary was bracketed: `valid` is the furthest value found
    /// valid, `invalid` the nearest found invalid, and the true edge
    /// lies between them.
    Edge {
        /// The furthest value found valid.
        valid: f64,
        /// The nearest value found invalid.
        invalid: f64,
    },
}

impl Bound {
    /// The value a reader should treat as the limit: the furthest one
    /// actually found valid, in either arm.
    pub fn limit(self) -> f64 {
        match self {
            Self::Open { probed } | Self::Edge { valid: probed, .. } => probed,
        }
    }

    /// Whether a boundary was found at all.
    pub fn is_edge(self) -> bool {
        matches!(self, Self::Edge { .. })
    }
}

/// The finished probe: the room a field has on each side of where it
/// is now.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Bounds {
    /// The value the search started from — valid by construction (the
    /// baseline is taken there).
    pub origin: f64,
    /// Downward.
    pub low: Bound,
    /// Upward.
    pub high: Bound,
    /// How many values were sampled, both directions together. Reported
    /// because the answer's quality is a function of it and because a
    /// probe that hit [`BoundsProbe::MAX_SAMPLES`] is saying something
    /// different from one that converged.
    pub samples: usize,
}

impl Bounds {
    /// The reading, as one line, written in `unit`.
    ///
    /// **It says what the search established and no more.** An open
    /// side reads as an inequality against how far the search looked
    /// (`≥ 2 mm`), never as an unbounded one, because the search cannot
    /// establish that. A bracketed side reads as the furthest value
    /// found VALID, which is the number a user can act on — the nearest
    /// invalid one is the same reading's other half and is what makes
    /// the `…` a bracket rather than a promise about everything inside
    /// (the module's non-monotone caveat).
    ///
    /// It lives here rather than in the panel because it is the one
    /// place the probe's limits become a sentence a user reads, and a
    /// sentence that overclaimed would undo the care the search takes
    /// not to. A headless row pins it.
    ///
    /// **Each number is rendered rather than formatted**
    /// ([`crate::readout::number`]), and the care above is the reason
    /// rather than tidiness. A fixed `{:.4}` overclaimed at both ends
    /// of this sentence: a floor at 40 nm written in millimetres read
    /// `valid from 0.0000 mm`, which promises a range down to zero that
    /// the search did not look at and the edge it found argues against,
    /// and a reach of a thousand read `1024.0000`, which is four figures
    /// a doubling probe never established. The render
    /// says what the value is to within its own stated accuracy and no
    /// more, in as many characters as that takes.
    ///
    /// **The written number is the panel's own divide**
    /// ([`crate::props::shown_in`]), not a second one: a range reading
    /// and a panel field showing the same canonical value in the same
    /// unit are the same number by construction, and a change to how a
    /// written value is derived from a canonical one reaches both.
    ///
    /// **A bound may be zero or negative, and the render is right about
    /// both.** The rule is that a text reads back as the value, not that
    /// it is non-zero: a bound that IS zero reads `0`, and a sign is not
    /// a distance ([`crate::readout::reads_back`] measures how far a
    /// text reads FROM the value). This is what stops the rule being
    /// [`crate::scene::DisplayTolerance::render_mm`] with the δ taken
    /// out — δ is strictly positive and a probed field is not.
    ///
    /// **And a bound may be one the NOTATION cannot name**, which is
    /// the one thing this sentence must not spell `inf`. Writing a
    /// canonical value in millimetres multiplies it up by a thousand,
    /// so a bound above `f64::MAX * MILLI` has no millimetre value and
    /// `inf mm` would read as a search that reached infinity — the
    /// overclaim this whole doc comment exists to refuse, at the one
    /// end where nothing the probe did is wrong. The conversion is
    /// asked rather than performed
    /// ([`crate::props::shown_text`], over [`crate::props::written`]),
    /// which is also why the divide is no longer written out here.
    /// The `map_or` this line used to open with was a third
    /// hand-written spelling of [`crate::props::shown_in`] — the one
    /// that door's own doc warns about — and the door that renders it
    /// is the same door with the render attached, so there is one home
    /// for `canonical / factor` and one for what to say when it has no
    /// answer.
    ///
    /// **This render owns no bound on the value itself, and no door
    /// upstream of it does either.** A probed bound is where the
    /// doubling search reached from the field's own value, and the
    /// chrome's `f64` fields carry no `.range()`, so the origin is
    /// whatever a user typed. There is nothing here to narrow; what
    /// there is, is a value that has no reading in the unit asked for,
    /// and saying so is the whole of the repair.
    pub fn wording(self, unit: Option<UnitDef>) -> String {
        let show = |value: f64| crate::props::shown_text(unit, value);
        match (self.low, self.high) {
            (Bound::Open { probed: low }, Bound::Open { probed: high }) => format!(
                "nothing new fails anywhere from {} to {} — as far as {} samples looked",
                show(low),
                show(high),
                self.samples
            ),
            (Bound::Open { probed }, Bound::Edge { valid, .. }) => format!(
                "valid up to {}; nothing new fails down to {} (as far as it looked)",
                show(valid),
                show(probed)
            ),
            (Bound::Edge { valid, .. }, Bound::Open { probed }) => format!(
                "valid from {}; nothing new fails up to {} (as far as it looked)",
                show(valid),
                show(probed)
            ),
            (Bound::Edge { valid: low, .. }, Bound::Edge { valid: high, .. }) => {
                format!("valid {} … {}", show(low), show(high))
            }
        }
    }
}

/// Which phase a direction's search is in.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Phase {
    /// Stepping outward, doubling the stride, looking for a failure.
    Reaching,
    /// A failure was found; narrowing the bracket.
    Refining,
    /// Nothing more to learn in this direction.
    Settled,
}

/// One direction's search state.
#[derive(Clone, Copy, Debug)]
struct Sweep {
    /// −1 downward, +1 upward.
    sign: f64,
    phase: Phase,
    /// The furthest OFFSET from the origin found valid, always ≥ 0.
    valid: f64,
    /// The nearest offset found invalid, when one has been.
    invalid: Option<f64>,
    /// Doublings spent.
    reaches: u32,
    /// Bisection steps spent.
    refines: u32,
}

impl Sweep {
    fn new(sign: f64) -> Self {
        Self {
            sign,
            phase: Phase::Reaching,
            valid: 0.0,
            invalid: None,
            reaches: 0,
            refines: 0,
        }
    }

    /// The next offset to try, or `None` when this direction is done.
    fn next_offset(&self, seed: f64, integral: bool) -> Option<f64> {
        match self.phase {
            Phase::Settled => None,
            Phase::Reaching => Some(BoundsProbe::reach_offset(seed, self.reaches)),
            Phase::Refining => {
                let invalid = self.invalid?;
                let mid = midpoint(self.valid, invalid, integral)?;
                Some(mid)
            }
        }
    }

    /// Record a sample's verdict at `offset`.
    fn observe(&mut self, offset: f64, ok: bool, seed: f64, integral: bool) {
        if ok {
            if offset > self.valid {
                self.valid = offset;
            }
        } else if self.invalid.is_none_or(|nearest| offset < nearest) {
            self.invalid = Some(offset);
        }
        match self.phase {
            Phase::Settled => {}
            Phase::Reaching => {
                if self.invalid.is_some() {
                    self.phase = Phase::Refining;
                } else {
                    self.reaches += 1;
                    if self.reaches >= BoundsProbe::MAX_REACHES {
                        self.phase = Phase::Settled;
                    }
                }
            }
            Phase::Refining => self.refines += 1,
        }
        // One settle test for both phases, so a bracket that has
        // become too tight to split stops whichever phase found it.
        if self.phase == Phase::Refining
            && (self.refines >= BoundsProbe::MAX_REFINES
                || self
                    .next_offset(seed, integral)
                    .is_none_or(|mid| mid <= self.valid || Some(mid) >= self.invalid))
        {
            self.phase = Phase::Settled;
        }
    }

    fn bound(&self, origin: f64) -> Bound {
        match self.invalid {
            Some(invalid) => Bound::Edge {
                valid: origin + self.sign * self.valid,
                invalid: origin + self.sign * invalid,
            },
            None => Bound::Open {
                probed: origin + self.sign * self.valid,
            },
        }
    }
}

/// The midpoint of a bracket, `None` when it cannot be split further.
///
/// The integral arm is not a rounding of the continuous one: a Count
/// field's answer IS an integer, so the bracket is split in whole
/// numbers and closes exactly — `valid` ends up the largest integer
/// that works, with no residual uncertainty for a reader to interpret.
fn midpoint(valid: f64, invalid: f64, integral: bool) -> Option<f64> {
    if !integral {
        let mid = valid + (invalid - valid) / 2.0;
        return (mid > valid && mid < invalid).then_some(mid);
    }
    let mid = (valid + (invalid - valid) / 2.0).floor();
    (mid > valid && mid < invalid).then_some(mid)
}

/// The search itself: a value machine that asks for samples and takes
/// verdicts back. See the module docs for what it is and is not.
#[derive(Clone, Debug)]
pub struct BoundsProbe {
    origin: f64,
    seed: f64,
    integral: bool,
    low: Sweep,
    high: Sweep,
    samples: usize,
}

impl BoundsProbe {
    /// How many doublings a direction spends looking for a failure
    /// before reporting [`Bound::Open`]. Twelve reaches `2¹¹` seeds —
    /// two thousand times the field's natural step — which is past the
    /// point where "there is no limit near here" is the useful answer.
    pub const MAX_REACHES: u32 = 12;

    /// How many bisection steps a direction spends narrowing a
    /// bracket. Ten halvings take it to about a thousandth of the
    /// width it STARTED at, which is the reach stride that caught the
    /// failure and NOT the seed: the stride doubles, so a bracket
    /// entered four seeds wide closes to about four thousandths of a
    /// seed, and one entered a thousand seeds wide closes no finer
    /// than a seed. How fine the reported pair is therefore depends
    /// on how far out the failure was, not on the step alone.
    ///
    /// **The division is exact and the reported pair is not.** Each
    /// end of a [`Bound::Edge`] is `origin ± offset`, so the width a
    /// reader measures off the pair carries the ORIGIN's rounding,
    /// which at a millimetre-scale bracket around a centimetre-scale
    /// origin is four thousand times the bracket's own. A check on
    /// this law counts halvings; it does not compare widths.
    pub const MAX_REFINES: u32 = 10;

    /// The ceiling on samples over both directions — the cost bound the
    /// module docs quote, stated where it is enforced.
    pub const MAX_SAMPLES: usize = 2 * (Self::MAX_REACHES as usize + Self::MAX_REFINES as usize);

    /// The offset the `doubling`th reach step places, stepping by
    /// `seed`: `seed · 2^doubling`. The ladder is geometric, so a
    /// failure far out is found in a logarithmic number of samples.
    ///
    /// `doubling` is a step index and is below [`Self::MAX_REACHES`]
    /// at every caller, which is what keeps the shift in range.
    fn reach_offset(seed: f64, doubling: u32) -> f64 {
        seed * f64::from(1u32 << doubling)
    }

    /// The furthest offset a reach can place, stepping by `seed`: the
    /// last of [`Self::MAX_REACHES`] doublings, `seed · 2^(N−1)`.
    ///
    /// Public for [`crate::camera::Camera::pitch_limit`]'s reason: it
    /// is a *contract* a test has to reason against — how far
    /// [`Bound::Open`] looked, for a seed — and a test that restates
    /// it as a literal is a hand-synced copy of a constant's
    /// consequence, which is the defect this door exists to remove.
    /// One home; read it.
    #[must_use]
    pub fn furthest_reach(seed: f64) -> f64 {
        Self::reach_offset(seed, Self::MAX_REACHES - 1)
    }

    /// A probe around `origin`, stepping by `seed`.
    ///
    /// `seed` is the field's natural step — one of whatever unit the
    /// panel is writing it in, and 1 for a count — and sets the scale
    /// of the whole search: the first sample is one seed out, the
    /// furthest is [`BoundsProbe::furthest_reach`] of one, and a
    /// bracket closes to about a thousandth of the stride that found
    /// it ([`BoundsProbe::MAX_REFINES`]). A non-finite or
    /// non-positive seed is replaced by its magnitude or by 1, because
    /// a probe that refused would leave the panel with nothing to say
    /// about a field whose scale it could not guess.
    ///
    /// `integral` makes the search step and settle in whole numbers —
    /// the Count fields, whose answer is an integer.
    pub fn new(origin: f64, seed: f64, integral: bool) -> Self {
        let seed = if seed.is_finite() && seed != 0.0 {
            seed.abs()
        } else {
            1.0
        };
        Self {
            origin,
            seed: if integral { seed.max(1.0) } else { seed },
            integral,
            low: Sweep::new(-1.0),
            high: Sweep::new(1.0),
            samples: 0,
        }
    }

    /// The value to evaluate next, or `None` when the search is done.
    ///
    /// The low direction is exhausted before the high one, so a caller
    /// that stops early has one complete answer rather than two
    /// half-answers.
    pub fn next(&self) -> Option<f64> {
        if self.samples >= Self::MAX_SAMPLES {
            return None;
        }
        for sweep in [&self.low, &self.high] {
            if let Some(offset) = sweep.next_offset(self.seed, self.integral) {
                return Some(self.origin + sweep.sign * offset);
            }
        }
        None
    }

    /// Record what [`BoundsProbe::next`]'s value turned out to be.
    ///
    /// `ok` is the caller's verdict: true when evaluating at that value
    /// introduced no failure the baseline did not have
    /// ([`Verdict::no_worse_than`]).
    ///
    /// A value that is not the one `next` asked for is still recorded,
    /// against whichever direction it falls on. Nothing depends on the
    /// caller having asked first, which is what lets a driver batch,
    /// cache or reorder samples.
    pub fn observe(&mut self, value: f64, ok: bool) {
        self.samples += 1;
        let offset = value - self.origin;
        let (seed, integral) = (self.seed, self.integral);
        if offset < 0.0 {
            self.low.observe(-offset, ok, seed, integral);
        } else if offset > 0.0 {
            self.high.observe(offset, ok, seed, integral);
        }
        // An offset of exactly zero is the origin, whose verdict is the
        // baseline itself — it teaches neither direction anything.
    }

    /// Whether the search has nothing left to try.
    pub fn done(&self) -> bool {
        self.next().is_none()
    }

    /// The answer so far — meaningful at any point, and final once
    /// [`BoundsProbe::done`].
    pub fn result(&self) -> Bounds {
        Bounds {
            origin: self.origin,
            low: self.low.bound(self.origin),
            high: self.high.bound(self.origin),
            samples: self.samples,
        }
    }
}

/// Run a probe to completion against a validity oracle.
///
/// The one driver, shared by the session (whose oracle applies the edit
/// and evaluates) and by the suite (whose oracles are arithmetic). A
/// caller that wants to spread the samples over time drives
/// [`BoundsProbe`] directly instead; this is the inline case.
pub fn probe(mut probe: BoundsProbe, mut valid: impl FnMut(f64) -> bool) -> Bounds {
    while let Some(value) = probe.next() {
        let ok = valid(value);
        probe.observe(value, ok);
    }
    probe.result()
}
