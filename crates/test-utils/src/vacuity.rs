//! **The anti-vacuity floor** — the tree's spelling of *how much this run
//! actually exercised*, printed every run and asserted.
//!
//! # The idiom
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
//!    before the floors, so a `--nocapture` or local run leaves the
//!    numbers where a reader of the claim can find them. **On a green CI
//!    run it reaches nobody** — libtest and nextest both capture a
//!    passing test's stdout and no job here passes `--nocapture` — so
//!    the tally is *also* carried in every floor's panic message, which
//!    is the path CI shows. The numbers that matter on a red run are on
//!    the red run; the ones on a green run are for whoever goes looking.
//!    Getting them onto the board is a `ci.yml` question and `ci.yml`
//!    belongs to another lane.
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
//! `stood_down` is for a mode that is *unreachable in this
//! configuration*, never for one that merely did not happen to come up.
//! The second case is what the floor is for. And a stand-down states why
//! it is entitled to one — see `m5_pr7_ssi`'s fit-budget arms, which
//! assert the budget is D9's, genuinely overrun, at a finer-than-default
//! ε before they announce.
//!
//! **This is not yet the tree's only spelling**, and saying so is the
//! point of the sentence. `rg 'SKIPPED \('` over `crates/*/{src,tests}`
//! finds hand-rolled stand-down `println!`s in `review_m5_pr7b_ssi.rs`,
//! `topo/tests/{m6_2_fitted_at_rest,review_m6_2_probes}.rs` and
//! `mesh/tests/fitted_refusals.rs` that predate this module and are not
//! converted. Two of those files are under another lane's hand. A module
//! that claimed to be the one spelling while nine sites said otherwise
//! would be making this file's own mistake.

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
pub fn stood_down(label: &str, what_is_not_asserted: &str) {
    println!("SKIPPED ({label}): {what_is_not_asserted}");
}

#[cfg(test)]
#[allow(clippy::panic, clippy::expect_used)]
mod tests {
    use super::*;

    thread_local! {
        /// The last panic this module's hook saw, on THIS thread. The
        /// hook runs on the panicking thread, so a thread-local keeps
        /// two concurrent rows' messages apart without a lock.
        static LAST_PANIC: core::cell::RefCell<Option<String>> =
            const { core::cell::RefCell::new(None) };
    }

    /// The panic message a floor produces, or `None` if it passed. The
    /// module's whole content is assertions that must fire, so every row
    /// below drives one across its boundary in BOTH directions.
    ///
    /// The message is taken from a **panic hook**, not by downcasting
    /// the unwind payload: `downcast_ref` is a second bit channel and
    /// `scripts/gates/bit-identity-punning.sh` forbids it outside
    /// `geom-core/src/bit_identity.rs` — correctly, and it caught the
    /// first draft of this file.
    fn caught(f: impl FnOnce() + std::panic::UnwindSafe) -> Option<String> {
        let hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(|info| {
            let seen = info.to_string();
            LAST_PANIC.with(|c| *c.borrow_mut() = Some(seen));
        }));
        let out = std::panic::catch_unwind(f);
        std::panic::set_hook(hook);
        match out {
            Ok(()) => None,
            Err(_) => Some(
                LAST_PANIC
                    .with(|c| c.borrow_mut().take())
                    .unwrap_or_default(),
            ),
        }
    }

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
        // completed, which is how `kemr` became visible at all.
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
