//! **The anti-vacuity census** — the tree's one spelling of *how much
//! this run actually exercised*, printed every run and asserted with a
//! floor.
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
//! 1. [`Census::note`] / [`Census::add`] count what the run reached, in
//!    the vocabulary of the property under attack — not "iterations",
//!    which a `continue` inflates for free, but the events that put the
//!    code under test in scope.
//! 2. [`Census::report`] prints the whole tally **unconditionally**, so
//!    a green run leaves the number in the battery log where a reader
//!    of the claim can find it.
//! 3. [`Census::require`] / [`Census::require_total`] /
//!    [`Census::require_each`] / [`Census::require_nonzero_among`] put a
//!    floor under it. Below the floor the
//!    run is red, and its message carries the whole census plus the
//!    caller's own sentence, so the failure names the coverage that went
//!    missing rather than the arithmetic.
//!
//! The floor is a **vacuity** floor, not a coverage target: it is set
//! where a run that reached it has genuinely entered the mode, and it
//! moves only when what the guard exercises changes.
//! [`crate::fuzz::scaled`] scales the sample count, so a floor stated as
//! a literal must survive `CAD_FUZZ_EFFORT=1` — state it against the
//! floor of the dial, never against a full-effort run.
//!
//! # The other half: a run that legitimately cannot enter its mode
//!
//! Some rows have an ε, a feature or a fixture at which the mode they
//! attack is genuinely unreachable. That run must not assert a floor it
//! cannot meet — and it must not return green in silence either, which
//! is the same vacuity one level up. It calls [`stood_down`], which
//! prints the tree's `SKIPPED (…)` line naming the coverage this run did
//! not deliver. The named-loud-skip test
//! (`interval_lane_skipped_no_certified_coverage_here`) is the same
//! idiom at whole-binary scale.
//!
//! `stood_down` is for a mode that is *unreachable in this
//! configuration*, never for one that merely did not happen to come up.
//! The second case is what the floor is for.

use std::collections::BTreeMap;

/// A tally of what a sampling guard reached, by category.
///
/// Categories are the property's own vocabulary; the census sorts them
/// so the printed evidence line is stable across runs.
#[derive(Clone, Debug)]
pub struct Census {
    label: String,
    counts: BTreeMap<String, usize>,
}

impl Census {
    /// A census labelled with the row it belongs to. The label appears
    /// in the evidence line and in every failure message.
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

    /// What `category` reached; `0` if it was never recorded.
    #[must_use]
    pub fn count(&self, category: &str) -> usize {
        self.counts.get(category).copied().unwrap_or(0)
    }

    /// Every observation, across categories.
    #[must_use]
    pub fn total(&self) -> usize {
        self.counts.values().sum()
    }

    /// Print the whole census as an `[evidence]` line. Call it
    /// **unconditionally**, before the floors: the number is what a
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

    /// Floor under the whole census.
    ///
    /// # Panics
    ///
    /// If fewer than `min` observations were recorded in total.
    #[track_caller]
    pub fn require_total(&self, min: usize, why: &str) {
        assert!(
            self.total() >= min,
            "VACUOUS: {self} — {} observations against a floor of {min}; {why}",
            self.total()
        );
    }

    /// **At least `min` of `categories` were reached at all.** The
    /// floor for *how much of the mechanism the run touched*, where a
    /// raw total can be carried by one arm: a sweep that drives seven
    /// operators and enters the mutation phase of only one is vacuous
    /// about the other six, and its total does not say so.
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

    /// Fold another census in, category by category.
    pub fn merge(&mut self, other: &Census) {
        for (k, n) in &other.counts {
            self.add(k, *n);
        }
    }

    /// The same floor under each of several categories — the shape for
    /// *every regime is present*, where a builder change can silently
    /// empty one and the total stays healthy.
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
}

impl core::fmt::Display for Census {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{}: {} total {:?}",
            self.label,
            self.total(),
            self.counts
        )
    }
}

/// The loud stand-down: this run **cannot** enter the mode `label`
/// names, so it asserts nothing about it and says so by name.
///
/// `what_is_not_asserted` states the claim this run did not make, in
/// the row's own words. A stand-down that says only "skipped" leaves the
/// reader to work out what the green meant.
pub fn stood_down(label: &str, what_is_not_asserted: &str) {
    println!("SKIPPED ({label}): {what_is_not_asserted}");
}
