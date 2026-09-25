//! **How much this run actually exercised**, in the tree's two shapes.
//! [`Exposure`] is the anti-vacuity FLOOR: a tally a run can fall
//! through, so it reds. The STAND-DOWN is a run saying which mode it
//! could not enter, which asserts nothing and cannot red — spelled
//! [`stood_down`] from inside a row that ran, and
//! [`crate::loud_skip_marker!`] for a whole binary whose rows did not
//! compile. Only the floor is a guard. The stand-down is documented
//! here at the same length so that nobody reads it as one.
//!
//! # The floor
//!
//! A guard that samples — a fuzz sweep, a corpus drive, a hammer pass —
//! has two failure modes, and only one of them is loud. It can observe
//! the thing it forbids, which reds; or it can observe **nothing at
//! all**, which greens. The second is the dangerous one, because a
//! sweep whose every call refused at the door and a sweep that proved
//! the property are the same colour on the board.
//!
//! So a sampling guard states its exposure and asserts it:
//!
//! 1. [`Exposure::note`] / [`Exposure::add`] count what the run reached,
//!    in the vocabulary of the property under attack — not "iterations",
//!    which a `continue` inflates for free, but the events that put the
//!    code under test in scope.
//! 2. [`Exposure::report`] prints the whole tally **unconditionally**,
//!    before the floors, so a local run leaves the numbers where a reader
//!    of the claim can find them. **On a green CI run it reaches
//!    nobody** (see *What a passing row prints reaches nobody on the
//!    gate*, below), so the tally is *also* carried in every floor's
//!    panic message, which is the path CI shows. The numbers that matter
//!    on a red run are on the red run; the ones on a green run are for
//!    whoever goes looking.
//! 3. [`Exposure::require`], [`Exposure::require_each`] and
//!    [`Exposure::require_nonzero_among`] put a floor under it. Below the
//!    floor the run is red, and the message carries the whole tally plus
//!    the caller's own sentence, so the failure names the coverage that
//!    went missing rather than the arithmetic.
//!
//! **Choose the floor by what can carry the claim alone.** A total is the
//! wrong floor wherever one arm of a mechanism can satisfy it by itself —
//! `review_d18`'s hammer drives seven operators, and on a destination
//! whose every arena field is nulled one of them still returns `Ok`, so
//! *"at least one mutation phase"* is a floor almost nothing can break.
//! [`Exposure::require_nonzero_among`] is for that shape: it counts how
//! much of the surface was reached, not how often.
//!
//! The floor is a **vacuity** floor, not a coverage target: it is set
//! where a run that reaches it has genuinely entered the mode, and it
//! moves only when what the guard exercises changes. [`crate::fuzz::scaled`]
//! scales sample counts, so a floor stated as a literal must survive
//! `CAD_FUZZ_EFFORT=1` — state it against the floor of the dial, never
//! against a full-effort run, and put the measured value beside it.
//!
//! # The other half: a run that legitimately cannot enter its mode
//!
//! Some rows have an ε, a feature or a fixture at which the mode they
//! attack is genuinely unreachable. That run must not assert a floor it
//! cannot meet — and it must not return green in silence either, which
//! is the same vacuity one level up. It calls [`stood_down`], which
//! prints the `SKIPPED (…)` line naming the coverage this run did not
//! deliver.
//!
//! **On the gate that line reaches nobody** — the section below this one
//! states why, once, for everything in this module that prints.
//!
//! The channel is the smaller half. `stood_down` has **no failing
//! path**: a row that stood down where it could have asserted is the
//! same green as a row that asserted, and would be even if every word
//! reached the log. Turning that red is not a louder print — it is a
//! floor the ROW states about its own condition, which is that row's
//! posture to argue and not this door's to impose. It cannot be a
//! suite-wide count either, because nextest runs each test in its own
//! process and no tally survives the row that built it.
//!
//! `stood_down` is for a mode that is *unreachable in this
//! configuration*, never for one that merely did not happen to come up.
//! The second case is what the floor is for. And a stand-down states why
//! it is entitled to one — see `m5_pr7_ssi`'s fit-budget arms, which
//! assert the budget is D9's, genuinely overrun, at a finer-than-default
//! ε before they announce.
//!
//! **The hand-rolled in-row `println!`s a sweep found were converted;
//! that every in-row stand-down in `crates/` goes through this door is
//! not a claim this module can make.** Nothing guards it, and the
//! sweep's blind spot has a known occupant: in
//! `crates/editor-core/tests/m10_5_r1_probes_interval.rs`,
//! `a_partial_revolve_band_reports_its_phantom_turn` asserts nothing in
//! three of its four arms and announces each through a bare `println!`
//! the pattern cannot match. What the sweep matched is `SKIPPED (` /
//! `SKIPPED:` over `crates/*/{src,tests}`, so it sees no stand-down
//! announced without the word — an `eprintln!("standing down …")`, a
//! `dbg!`, or a comment where a print should be — and it does not reach
//! `demos/`, `tools/` or `interval-transcendentals/`.
//!
//! # The third spelling: a whole binary that did not compile
//!
//! [`crate::loud_skip_marker!`] is the other stand-down, and it is a different
//! idiom from [`stood_down`] rather than a caller of it. `stood_down`
//! announces from INSIDE a row that ran and could not enter its mode;
//! the marker exists only because its siblings did not compile, so its
//! condition is a `#[cfg]` and there is no running row to announce
//! from. It has the working half this door lacks: its NAME reaches the
//! PASS list, which is the payload a gating run actually carries.
//! `memories/test-suite-cost.md` points at the viewer's `app` spelling of
//! that name, and no other.
//!
//! # What a passing row prints reaches nobody on the gate
//!
//! Stated once, because three things here print — [`Exposure::report`],
//! [`stood_down`] and [`crate::loud_skip_marker!`]'s row — and the fact
//! is the same for all three. libtest and nextest both capture a passing
//! test's stdout. nextest then discards it unless `--success-output`
//! says otherwise: its default is `never`, every gating
//! `cargo nextest run` passes no such flag, and no `nextest.toml` in
//! this tree sets a profile default. Exactly one job passes it —
//! `viewer --features app`, for its smoke row's adapter — and libtest's
//! `--nocapture` is passed by no gating job either. **So a `println!`
//! from a row that PASSES is read on a local run, on a run someone asked
//! for one of those flags on, and nowhere else.** A `println!` from a
//! row that FAILS is shown, which is why every floor in this module
//! carries its tally in the panic message rather than trusting the
//! print.
//!
//! That is a channel fact about the gating jobs, not about this crate:
//! if it ever changes, this section is the one place that has to.

use std::collections::BTreeMap;

/// What a sampling guard reached, by category.
///
/// Categories are the property's own vocabulary; the tally is sorted so
/// the printed evidence line is stable across runs.
#[derive(Clone, Debug)]
pub struct Exposure {
    label: String,
    counts: BTreeMap<String, usize>,
}

impl Exposure {
    /// An exposure labelled with the row it belongs to. The label
    /// appears in the evidence line and in every failure message.
    #[must_use]
    pub fn new(label: &str) -> Self {
        Self {
            label: label.to_string(),
            counts: BTreeMap::new(),
        }
    }

    /// Record one observation in `category`.
    pub fn note(&mut self, category: &str) {
        self.add(category, 1);
    }

    /// Record `n` observations in `category`. `n = 0` still creates the
    /// category, so a regime that was looked for and never reached
    /// prints as `0` rather than vanishing from the evidence line.
    pub fn add(&mut self, category: &str, n: usize) {
        *self.counts.entry(category.to_string()).or_default() += n;
    }

    /// Fold another exposure in, category by category.
    pub fn merge(&mut self, other: &Exposure) {
        for (k, n) in &other.counts {
            self.add(k, *n);
        }
    }

    /// What `category` reached; `0` if it was never recorded.
    #[must_use]
    pub fn count(&self, category: &str) -> usize {
        self.counts.get(category).copied().unwrap_or(0)
    }

    /// Print the whole tally as an `[evidence]` line. Call it
    /// **unconditionally**, before the floors: the numbers are what a
    /// reader of the claim wants, and a number printed only on failure
    /// is not evidence about the green runs.
    pub fn report(&self) {
        println!("[evidence] {self}");
    }

    /// Floor under one category. `why` is the caller's sentence about
    /// what falling through it means — what coverage the run lost, and
    /// how to reproduce it ([`crate::fuzz::replay`]).
    ///
    /// # Panics
    ///
    /// If `category` reached fewer than `min` observations.
    #[track_caller]
    pub fn require(&self, category: &str, min: usize, why: &str) {
        assert!(
            self.count(category) >= min,
            "VACUOUS: {self} — {category:?} reached {} of the {min} this row needs; {why}",
            self.count(category)
        );
    }

    /// The same floor under each of several categories — the shape for
    /// *every regime is present*, where a builder change can silently
    /// empty one while the others keep the run looking healthy.
    ///
    /// # Panics
    ///
    /// If any of `categories` reached fewer than `min` observations.
    #[track_caller]
    pub fn require_each(&self, categories: &[&str], min: usize, why: &str) {
        for c in categories {
            self.require(c, min, why);
        }
    }

    /// **At least `min` of `categories` were reached at all.** The floor
    /// for *how much of the mechanism the run touched*, where a raw count
    /// can be carried by one arm: a sweep that drives seven operators and
    /// enters the mutation phase of only one is vacuous about the other
    /// six, and no total says so.
    ///
    /// # Panics
    ///
    /// If fewer than `min` of `categories` have a nonzero count.
    #[track_caller]
    pub fn require_nonzero_among(&self, categories: &[&str], min: usize, why: &str) {
        let reached = categories.iter().filter(|c| self.count(c) > 0).count();
        assert!(
            reached >= min,
            "VACUOUS: {self} — {reached} of these {} reached anything, against a floor \
             of {min}: {categories:?}; {why}",
            categories.len()
        );
    }
}

impl core::fmt::Display for Exposure {
    /// The label and the tally. **No total**: categories in one exposure
    /// are not always summable — `review_d18` holds a call count beside
    /// per-operator completions of those same calls — so a printed sum
    /// would be a number with no referent.
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{} {:?}", self.label, self.counts)
    }
}

/// The loud stand-down: this run **cannot** enter the mode `label`
/// names, so it asserts nothing about it and says so by name.
///
/// `what_is_not_asserted` states the claim this run did not make, in the
/// row's own words. A stand-down that says only "skipped" leaves the
/// reader to work out what the green meant.
///
/// **What this does not do.** It is a print, and it is the whole
/// mechanism: it cannot fail, it counts nothing, and no floor reads it.
/// It does not make the row it stands in visible as a skip — the name
/// that reaches a gating run's PASS list is the row's own, which says
/// the row passed. And on every gating job the line itself is discarded
/// (see this module's docs), so the reader it addresses is a local one.
/// A caller that needs a stand-down something can go RED on has to state
/// a floor over its own condition; this door will not supply one.
pub fn stood_down(label: &str, what_is_not_asserted: &str) {
    println!("SKIPPED ({label}): {what_is_not_asserted}");
}

/// **The whole-binary loud skip, in one spelling.** Emits the marker row
/// a test binary carries when the feature its rows need is not built:
///
/// ```ignore
/// test_utils::loud_skip_marker!(
///     feature = "app",
///     row = app_lane_skipped_no_chrome_coverage_here,
///     absent = "coverage of the chrome's labels",
/// );
/// ```
///
/// `feature` is the feature the file's rows need, `row` is the `fn` name
/// the PASS list will carry, and `absent` names the coverage this build
/// does not have — a SUBJECT, in the file's own vocabulary, never a list
/// of rows.
///
/// # Why the row exists at all
///
/// A binary whose every row is gated away reports "0 passed", which
/// reads like coverage in a battery summary; a file that merely loses
/// some of its rows reports a smaller number and nothing else. Either
/// way a reader of a default-feature run cannot tell a lane that was
/// never built from a lane that was deleted. The marker is the sentence
/// that says which.
///
/// # Why this is a macro and not a copy per file
///
/// The marker's payload is split: the NAME reaches the PASS list and is
/// read; the `println!` body is discarded on every gating run (see
/// [`crate::vacuity`]'s *What a passing row prints reaches nobody on the
/// gate*). A
/// body nobody can read is the worst possible home for anything kept by
/// hand, so nothing here is. The feature reaches both the `#[cfg]` and
/// the printed text from the SAME token, so the two cannot disagree; the
/// file name is `file!()` at the invocation site, as rustc spells it, so
/// a `git mv` re-spells it; and the sentence about what the row is and
/// is not lives here, once, instead of in every file that has one.
///
/// Which JOB builds the feature is deliberately not named: that is
/// `.github/workflows/ci.yml`'s to say, it is not knowable from here,
/// and a job name written down in every test file is the hand-kept
/// enumeration this macro exists to end.
///
/// # What it claims about the file, and what it does not
///
/// It says only that the rows THIS FILE gates behind `feature` are not
/// compiled in this build. It does **not** say the binary is otherwise
/// empty — several of these files carry rows that run at default
/// features — and it does not enumerate, count or name any gated row.
/// Adding a row to the gated block, removing one, or renaming one leaves
/// every word of this true.
///
/// # What it does not enforce
///
/// Nothing. It cannot fail, it asserts nothing, and no gating job reads
/// its body. Its whole delivered payload is `row` appearing in the PASS
/// list, so a reader of a default-feature run meets the absence instead
/// of inferring it from a test count. Keep gating to the rows
/// themselves.
#[macro_export]
macro_rules! loud_skip_marker {
    (feature = $feature:literal, row = $row:ident, absent = $absent:literal $(,)?) => {
        #[cfg(not(feature = $feature))]
        #[test]
        fn $row() {
            println!(
                "SKIPPED (no --features {feature}): {file} contributes NO {absent} \
                 in this run — the rows it gates behind the `{feature}` feature are \
                 not compiled in this build, and run wherever that feature is.",
                feature = $feature,
                file = ::core::file!(),
                absent = $absent,
            );
        }
    };
}

#[cfg(test)]
#[allow(clippy::panic, clippy::expect_used)]
mod tests {
    use super::*;
    use crate::panic_capture::caught;

    fn three() -> Exposure {
        let mut e = Exposure::new("row");
        e.add("a", 3);
        e.add("b", 0);
        e
    }

    #[test]
    fn add_zero_creates_the_category_so_an_unreached_regime_still_prints() {
        // `review_d18::hammer` depends on this: it pre-adds every
        // operator at 0 so the evidence line shows the ones that never
        // completed.
        let e = three();
        assert_eq!(e.count("b"), 0);
        assert!(format!("{e}").contains("\"b\": 0"), "{e}");
        assert!(!format!("{}", Exposure::new("row")).contains('b'));
    }

    #[test]
    fn count_of_an_unrecorded_category_is_zero_not_a_panic() {
        assert_eq!(three().count("never recorded"), 0);
    }

    #[test]
    fn require_fires_below_the_floor_and_passes_at_it() {
        let e = three();
        assert!(caught(|| three().require("a", 3, "why")).is_none());
        let msg = caught(|| three().require("a", 4, "the sweep lost its arm"))
            .expect("a floor above the count must fire");
        // The message carries the tally, the shortfall and the caller's
        // sentence — the three things that make a red actionable.
        assert!(msg.contains("VACUOUS"), "{msg}");
        assert!(msg.contains("reached 3 of the 4"), "{msg}");
        assert!(msg.contains("the sweep lost its arm"), "{msg}");
        assert!(msg.contains(&format!("{e}")), "{msg}");
    }

    #[test]
    fn require_fires_on_a_category_that_was_never_recorded() {
        let msg = caught(|| three().require("absent", 1, "why")).expect("must fire");
        assert!(msg.contains("reached 0 of the 1"), "{msg}");
    }

    #[test]
    fn require_each_fires_on_the_one_empty_regime_and_names_it() {
        assert!(caught(|| three().require_each(&["a"], 1, "why")).is_none());
        let msg = caught(|| three().require_each(&["a", "b"], 1, "a regime went silent"))
            .expect("the empty regime must fire");
        assert!(msg.contains("\"b\""), "{msg}");
    }

    #[test]
    fn require_nonzero_among_counts_reached_categories_not_observations() {
        // One arm carrying the whole count is exactly the shape this
        // floor exists for: `a` alone is 3 observations and 1 category.
        let e = three();
        assert!(caught(move || e.require_nonzero_among(&["a", "b"], 1, "why")).is_none());
        let msg = caught(|| three().require_nonzero_among(&["a", "b"], 2, "one arm carried it"))
            .expect("two of two must fire when only one is nonzero");
        assert!(msg.contains("1 of these 2 reached anything"), "{msg}");
        assert!(msg.contains("one arm carried it"), "{msg}");
    }

    #[test]
    fn merge_folds_category_by_category() {
        let (mut a, mut b) = (Exposure::new("a"), Exposure::new("b"));
        a.add("x", 2);
        b.add("x", 5);
        b.add("y", 1);
        a.merge(&b);
        assert_eq!((a.count("x"), a.count("y")), (7, 1));
    }
}
