//! The pick index's LIFECYCLE: asking the index seam for one, deciding
//! what to do with the answer, and refusing a pick while there is
//! none.
//!
//! # What this module decides, and what it does not
//!
//! **It decides nothing about picking.** What is under the cursor,
//! what a click means, and whether an edge beats the face behind it
//! are [`crate::pickindex`]'s — [`PickIndex::op_for`],
//! [`PickIndex::hovered_for`] and the priority rule live there, in the
//! one door hovering and clicking both read. This module never asks
//! the index a question; it decides WHEN one exists:
//!
//! - [`IndexInputs`] — what a build is handed, minted by the session
//!   as one landing's worth so a generation cannot travel without the
//!   run it answers;
//! - [`PickCache`] — the rebuild-on-stale loop over the
//!   [`crate::evalseam::IndexService`] seam, at most one attempt per
//!   (generation, δ), reporting [`CacheStep`] for what a sync did and
//!   [`IndexLanding`] for what an answer did;
//! - [`NotIndexed`] and [`unindexed`] — the typed refusal a pick
//!   stream earns while no index describes the picture on screen.
//!
//! # The boundary is stateful against pure
//!
//! Everything here owns mutable state and drives the seam; everything
//! in [`crate::pickindex`] is a value or a pure function over one.
//! That is the line the split was cut on. Each of the two names the
//! type it is built around — [`PickCache`] here, `PickIndex` there —
//! so a reader asking how a pick WORKS opens `pickindex` and one
//! asking when an index EXISTS opens this file. That is a description
//! of these two modules and not a rule that decides a name:
//! `work/view/a-module-named-for-its-spine-type-is-unfalsifiable`
//! holds why it cannot be one.
//!
//! # Why the index is not here
//!
//! The two are layers with the index seam between them: the seam is
//! built ABOVE the structure and BELOW the cache that drives it. Held
//! in one module, an import of either half was an import of both, and
//! `evalseam` — which names [`PickIndex`] as its payload — imported the
//! module that names `evalseam`'s own seam. The seam modules now run
//! one way, `generation ← pickindex ← evalseam ← pickcache`; the
//! crate around them is not acyclic, and `crates/viewer/README.md`'s *The
//! seam modules are a chain; the crate is not acyclic* says which ring
//! survives and why.
//!
//! Module kind: **vocabulary** (`crates/viewer/README.md`, Module
//! boundaries). It names no driver type and no `app`-only crate: what
//! the pick cache needs from a session arrives as [`IndexInputs`],
//! which the session mints.

use std::sync::Arc;

use pncad::document::{Doc, Evaluation, ProfileProgram};
use pncad::geom_core::Tol;

use crate::evalseam::{IndexDone, IndexRequest, IndexService, InlineIndexer};
use crate::generation::Generation;
use crate::input::PickAction;
use crate::pickindex::{PickIndex, PickIndexError};
use crate::scene::DisplayTolerance;

/// **What a pick index is built from**: a landed run, its generation
/// and the ε to tessellate at.
///
/// **Three of the four move together, and that is the point of the
/// value.** The generation, the document and the evaluation are
/// written by one landing and describe one run, so a caller cannot
/// pick up a generation without the pair it answers — the property
/// [`PickCache::sync`] used to spell out by hand across three
/// accessors. `tol` is not one of them: it is the session's ε, fixed
/// at construction and never rewritten by a landing. It rides here
/// because the build needs it and this is what the build is handed.
///
/// The session mints it ([`crate::session::DocSession::index_inputs`])
/// so that this module names no driver; the argument for hoisting
/// rather than widening the rule has one home, in
/// `crates/viewer/README.md`'s *What a vocabulary reads, it is
/// handed*.
pub struct IndexInputs<'a> {
    generation: Generation,
    doc: &'a Doc<ProfileProgram>,
    evaluation: &'a Arc<Evaluation<f64>>,
    tol: Tol,
}

impl<'a> IndexInputs<'a> {
    /// One landing's inputs, minted together.
    ///
    /// The fields are private and this is the only door, so the
    /// pairing the doc above claims is held BY THE TYPE and not by
    /// every caller remembering: nothing in this crate can write a
    /// generation beside another run's document.
    #[must_use]
    pub fn of(
        generation: Generation,
        doc: &'a Doc<ProfileProgram>,
        evaluation: &'a Arc<Evaluation<f64>>,
        tol: Tol,
    ) -> Self {
        Self {
            generation,
            doc,
            evaluation,
            tol,
        }
    }
}

/// A [`PickIndex`] kept current with a session — **the rebuild-on-stale
/// loop, owned once**.
///
/// Two things made this a type rather than a habit. Ergonomics: a
/// consumer that only wanted to pick had to notice `current_for` said
/// stale, then rebuild with four arguments (document, evaluation,
/// generation, δ) it had to keep in step by hand, three of which are
/// one landing's and arrive together as [`IndexInputs`]. And
/// correctness: the application's own
/// rebuild loop retried on **every repainted frame** whenever a build
/// refused — a failed or poisoned root is an ordinary editing state,
/// and each frame then re-tessellated every healthy root before
/// reaching the failing one, behind a picture that was already stale.
///
/// So the retry policy is stated once, here: **at most one attempt per
/// (landed generation, δ)**, success or failure. A failure is kept and
/// readable ([`PickCache::error`]) rather than retried into a stall.
/// [`PickCache::attempted`] is written when the attempt is SUBMITTED
/// rather than when it is answered, so the policy costs the same one
/// comparison whether the answer is in this frame or several seconds
/// away. It is cleared in exactly one place, and never as part of the
/// retry rule: [`PickCache::forget`] drops it when the picture it
/// names stops existing at all.
///
/// # Current or absent, never behind
///
/// The build happens on the [`IndexService`] seam, so between the
/// submit and the answer there is no index at all: [`PickCache::sync`]
/// drops the held one the moment it submits. That is the whole of the
/// staleness rule and it is deliberate — an index that is never READ
/// while stale is not derived data that can be wrong, so nothing here
/// has to reason about how far behind it is. What the window costs is
/// carried elsewhere: the viewport keeps drawing the mesh it last got
/// (an older picture), the chrome says a build is under way
/// (`crate::frame::progress`), and a pick made meanwhile is refused
/// typed ([`NotIndexed`]) rather than answered from something older.
pub struct PickCache {
    index: Option<PickIndex>,
    /// What the last attempt was for. `Some` after any attempt is
    /// SUBMITTED, answered or not — which is what stops the retry loop.
    attempted: Option<(Generation, DisplayTolerance)>,
    /// The attempt that has been submitted and not yet answered — what
    /// [`PickCache::indexing`] reports.
    ///
    /// Distinct from `attempted`, which outlives the answer: together
    /// they separate "asked, still waiting" from "asked, and the answer
    /// was a refusal we are not retrying".
    outstanding: Option<(Generation, DisplayTolerance)>,
    error: Option<PickIndexError>,
    seam: Box<dyn IndexService>,
}

/// Exhaustive by destructuring; the shared rule is
/// `crates/viewer/README.md`'s.
///
/// The one `_` arm is `seam`, a `dyn` service implementing no `Debug`.
/// `index` is carried as the generation it describes rather than as
/// the index itself, which is the fact a dump is asked for.
impl core::fmt::Debug for PickCache {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let Self {
            index,
            attempted,
            outstanding,
            error,
            seam: _,
        } = self;
        f.debug_struct("PickCache")
            .field("index", &index.as_ref().map(PickIndex::generation))
            .field("attempted", attempted)
            .field("outstanding", outstanding)
            .field("error", error)
            .finish_non_exhaustive()
    }
}

/// What one [`PickCache::sync`] did.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CacheStep {
    /// The held index already describes the run on screen.
    Current,
    /// A build for a new generation or δ was submitted to the seam.
    /// The held index is gone from this moment, not from the moment
    /// the answer arrives.
    Submitted,
    /// A build for exactly this (generation, δ) is already with the
    /// seam — nothing was done and nothing was resubmitted.
    Indexing,
    /// This attempt was already made and refused — nothing was done.
    Held,
    /// No evaluation has landed, so there is nothing to index.
    Nothing,
}

/// What one answer from the seam did to the cache.
///
/// [`crate::session::Landing`]'s counterpart for the second seam, and
/// the same two filters read the same way: a build for a key the cache
/// is no longer asking about is discarded here rather than installed
/// and compared later.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IndexLanding {
    /// The index landed and is what picks are answered from.
    Built,
    /// The build refused; the error is on the cache and will NOT be
    /// retried until the generation or δ moves.
    Refused,
    /// The answer was for a (generation, δ) the cache has moved past,
    /// so it was dropped. Restart-without-cancel produces exactly
    /// this: the superseded build was allowed to finish.
    Stale,
}

impl PickCache {
    /// A cache over `seam`.
    pub fn new(seam: Box<dyn IndexService>) -> Self {
        Self {
            index: None,
            attempted: None,
            outstanding: None,
            error: None,
            seam,
        }
    }

    /// A cache that builds its index inside [`PickCache::pump`] —
    /// the browser's shape and the tests' ([`InlineIndexer`]).
    pub fn inline() -> Self {
        Self::new(Box::new(InlineIndexer::new()))
    }

    /// Ask the seam for an index of a landed evaluation at `delta`, at
    /// most one attempt per (generation, δ).
    ///
    /// **δ is built at, verbatim.** `scene::TRIANGLE_BUDGET` chooses
    /// the δ a document OPENS at (`app`'s `fit_delta_on_scene`), and
    /// that is the whole of the budget's authority: once a δ is in
    /// force it is the value someone asked for, and a cache that
    /// quietly built a different picture would make the View pane's δ
    /// field a control that does nothing.
    ///
    /// The document is CLONED into the request and the evaluation is
    /// shared, so the worker owns everything it reads and the session
    /// goes on being edited. Both arrive on [`IndexInputs`], already
    /// paired: this cache is HANDED a landing and never reads one.
    pub fn sync(&mut self, landed: Option<IndexInputs<'_>>, delta: DisplayTolerance) -> CacheStep {
        // **The one way out on "nothing landed", and it FORGETS.**
        let Some(IndexInputs {
            generation,
            doc,
            evaluation,
            tol,
        }) = landed
        else {
            self.forget();
            return CacheStep::Nothing;
        };
        if self
            .index
            .as_ref()
            .is_some_and(|index| index.current_for(Some(generation), delta))
        {
            return CacheStep::Current;
        }
        let wanted = (generation, delta);
        if self.outstanding == Some(wanted) {
            // Asked, and the answer is not here yet. Asking again
            // would be the per-frame rebuild loop with a thread in it.
            return CacheStep::Indexing;
        }
        if self.attempted == Some(wanted) {
            // Attempted and refused for this exact picture. Retrying
            // is the per-frame rebuild loop; the error is already
            // recorded and the caller has already seen it.
            return CacheStep::Held;
        }
        self.attempted = Some(wanted);
        self.outstanding = Some(wanted);
        // **Dropped before the answer, not after it.** What is held
        // from here describes a run nobody is looking at any more, and
        // the one thing this cache must never do is answer a pick from
        // it.
        self.index = None;
        // The refusal on the cache is a statement about the attempt
        // that produced it, and this is a different attempt.
        //
        // **No row reds if this line goes**, and the reason is stated
        // rather than left to be rediscovered: what it buys is
        // narrower than the two lines above it. A refusal is only ever
        // read alongside the `Held` step that keeps it, and that step
        // is unreachable for a key still being built — so a stale
        // refusal surviving this window is readable through
        // [`PickCache::error`] and shown by nothing. It is cleared
        // because a cache whose error outlives its subject is a
        // question a later reader would have to answer, not because a
        // caller can tell.
        self.error = None;
        self.seam.submit(IndexRequest {
            generation,
            delta,
            doc: doc.clone(),
            evaluation: Arc::clone(evaluation),
            tol,
        });
        CacheStep::Submitted
    }

    /// Drop everything that describes a picture: the held index, the
    /// attempt that produced it or is producing it, and its refusal.
    ///
    /// **This is where "current or absent, never behind" is
    /// enforced**, and the one place it can be. Every other transition
    /// replaces one picture's key with another's, so a late answer is
    /// compared against a key and discarded. Here there is no next
    /// key: the session has no landed run at all, because a document
    /// was opened or a new one authored under a build that is still
    /// with the seam. Leaving `attempted` set would leave that build a
    /// key to match on arrival, and it would install — an index of a
    /// document nobody is looking at, over a scene of a third one,
    /// with nothing running and nothing said. Clearing `attempted` is
    /// what turns that answer into [`IndexLanding::Stale`]; the other
    /// three fields go with it because all four describe the same
    /// vanished picture.
    ///
    /// **Exhaustive by destructuring, like the walk above.** A field
    /// added to [`PickCache`] is an unbound-pattern error here, so a
    /// fifth thing describing the picture cannot outlive the picture
    /// by being forgotten at the declaration and not here. `seam` is
    /// the one `_` arm and must be: it is the service, not the
    /// picture.
    fn forget(&mut self) {
        let Self {
            index,
            attempted,
            outstanding,
            error,
            seam: _,
        } = self;
        *index = None;
        *attempted = None;
        *outstanding = None;
        *error = None;
    }

    /// Take whatever the seam has finished, discarding answers for
    /// pictures the cache has moved past.
    ///
    /// Returns one entry per answer handled, so a caller can assert on
    /// what was discarded rather than infer it.
    pub fn pump(&mut self) -> Vec<IndexLanding> {
        let mut landings = Vec::new();
        while let Some(done) = self.seam.poll() {
            landings.push(self.land(done));
        }
        landings
    }

    /// Decide one answer's fate. Public so the staleness rule is
    /// testable without a scheduler.
    ///
    /// **The key is the PAIR.** A build carrying the generation on
    /// screen at a δ the user has since moved off is a picture nobody
    /// asked for, and it would install without complaint if only the
    /// generation were compared — the sharper half of the same failure
    /// a wrong generation is, because the document is right and only
    /// the tessellation is not.
    pub fn land(&mut self, done: IndexDone) -> IndexLanding {
        if self.attempted != Some((done.generation, done.delta)) {
            return IndexLanding::Stale;
        }
        self.outstanding = None;
        match done.index {
            Ok(index) => {
                self.index = Some(index);
                self.error = None;
                IndexLanding::Built
            }
            Err(error) => {
                self.error = Some(error);
                IndexLanding::Refused
            }
        }
    }

    /// The held index, if the last attempt produced one — `None` for
    /// every frame between a submit and its answer.
    pub fn index(&self) -> Option<&PickIndex> {
        self.index.as_ref()
    }

    /// Whether a build is outstanding: the indexing state the chrome
    /// reads, as a value (`crate::frame::progress`).
    pub fn indexing(&self) -> bool {
        self.outstanding.is_some()
    }

    /// Why the last attempt refused, if it did.
    pub fn error(&self) -> Option<&PickIndexError> {
        self.error.as_ref()
    }
}

/// **A pick attempted while no index describes the document on
/// screen** — the typed *not indexed yet*.
///
/// Distinct from a miss, and that distinction is the whole of it. A
/// miss is an answer: the index was asked and there is nothing under
/// the cursor, so clearing the selection is right. This is the absence
/// of anybody to ask, and doing nothing quietly is what made the
/// window between two indexes look like a viewport that had decided
/// the user was pointing at empty space.
///
/// **Two arms, because waiting and not waiting are different advice.**
/// They are named for what is observably true rather than for a cause,
/// so neither can be shown over a state it does not describe: a
/// refused build and a document that has never been evaluated are both
/// "no index and nobody building one", and a sentence promising an
/// answer shortly would be false in both.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NotIndexed {
    /// A build is under way ([`PickCache::indexing`]): the answer is
    /// coming, and the toolbar is already saying so.
    Building,
    /// No index, and no build under way — the last attempt refused
    /// (its reason is [`PickCache::error`]), or nothing has been
    /// evaluated yet. Waiting will not help; the retry policy holds
    /// until the generation or δ moves.
    Absent,
}

impl core::fmt::Display for NotIndexed {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Building => write!(
                f,
                "not picked: the picture is still being indexed, and a pick \
                 is answered from the index or not at all"
            ),
            Self::Absent => write!(
                f,
                "not picked: the picture on screen has no pick index and none \
                 is being built — the last index build refused, or nothing has \
                 been evaluated yet"
            ),
        }
    }
}

impl core::error::Error for NotIndexed {}

/// The refusal a pick stream earns when there is no index to answer it
/// — `Some` for an ACT, `None` for an observation.
///
/// **A hover is not news.** It is pushed on every frame the pointer is
/// inside the pane, so a refusal raised for one would rewrite the
/// status line sixty times a second and erase every other writer's
/// sentence with it (`crate::frame`, the chrome's two channels);
/// the indexing indicator is what tells a reader why the model is
/// inert while they move over it. A click is an act — the user asked
/// for something and did not get it — and that is exactly what the
/// line carries.
///
/// `indexing` is [`PickCache::indexing`] — which of the two sentences
/// is true, asked of the one value that knows.
pub fn unindexed<'a>(
    actions: impl IntoIterator<Item = &'a PickAction>,
    indexing: bool,
) -> Option<NotIndexed> {
    actions
        .into_iter()
        .any(|action| match action {
            // An ACT: the user asked for something and did not get it.
            PickAction::Select(_) => true,
            // Observations. Exhaustive on purpose, the way
            // `ToolKind::pick_kinds` is: a fifth action added to the
            // stream must be classified here rather than falling into
            // "not news" because a wildcard put it there.
            PickAction::Hover(_) | PickAction::ClearHover => false,
        })
        .then_some(if indexing {
            NotIndexed::Building
        } else {
            NotIndexed::Absent
        })
}
