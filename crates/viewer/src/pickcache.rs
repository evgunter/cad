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
//!   picture ([`crate::pickindex::PictureKey`]), reporting
//!   [`CacheStep`] for what a sync did and
//!   [`IndexLanding`] for what an answer did;
//! - [`NotIndexed`] and [`unindexed`] — the typed refusal a pick
//!   stream earns while no index describes the picture on screen,
//!   whether because there is none at all or because the one in hand
//!   describes a picture that has not been drawn.
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
use crate::pickindex::{PickIndex, PickIndexError, PictureKey};
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
/// picture** ([`crate::pickindex::PictureKey`]), success or failure. A
/// failure is kept and readable ([`PickCache::error`]) rather than
/// retried into a stall. The attempt is recorded when it is SUBMITTED
/// rather than when it is answered, so the policy costs the same one
/// comparison whether the answer is in this frame or several seconds
/// away. It is dropped in exactly one place, and never as part of the
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
    /// The one attempt this cache is holding, and how far it has got.
    /// `Some` after any attempt is SUBMITTED, answered or not — which
    /// is what stops the retry loop — and dropped only by
    /// [`PickCache::forget`].
    attempt: Option<Attempt>,
    error: Option<PickIndexError>,
    seam: Box<dyn IndexService>,
}

/// **One attempt, at one picture, in one of its two states.**
///
/// The two facts a retry policy needs are *which picture was asked
/// about* and *has the answer arrived*, and they are one value because
/// the second is only a question about the first. Held as two
/// `Option`s they were a key and a boolean wearing the key's clothes:
/// the second was always either `None` or a copy of the first, `land`
/// read one and cleared the other, and nothing said they moved
/// together. A state on the key says it, and makes the diverged pair —
/// waiting on a picture other than the one attempted — unrepresentable
/// rather than merely absent.
#[derive(Clone, Copy, Debug, PartialEq)]
enum Attempt {
    /// Submitted, and no answer yet.
    ///
    /// **Only an answer moves it on, and an answer always comes.** A
    /// seam that loses the worker it submitted to ends the process
    /// where it finds out (`crate::evalseam`) rather than going quiet,
    /// so this state never stands over a seam nobody is behind. The one
    /// way it is dropped without an answer is [`PickCache::forget`],
    /// when the picture it names stops existing at all.
    Asked(PictureKey),
    /// Submitted and answered — installed as the held index, or refused
    /// into [`PickCache::error`]. Either way this picture is not
    /// attempted again: the retry policy is one attempt per picture.
    Answered(PictureKey),
}

impl Attempt {
    /// The picture attempted, in either state.
    fn key(self) -> PictureKey {
        match self {
            Self::Asked(key) | Self::Answered(key) => key,
        }
    }
}

/// Exhaustive by destructuring; the shared rule is
/// `crates/viewer/README.md`'s.
///
/// The one `_` arm is `seam`, a `dyn` service implementing no `Debug`.
/// `index` is carried as the generation it describes rather than as
/// the index itself, which is the fact a dump is asked for. It renders
/// as an ELISION around that generation — `Some(<PickIndex for
/// Generation(4)>)` — so the summary cannot be read as an
/// `Option<Generation>` field; `finish_non_exhaustive` speaks for the
/// `_` arm and not for this one.
impl core::fmt::Debug for PickCache {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let Self {
            index,
            attempt,
            error,
            seam: _,
        } = self;
        let mut out = f.debug_struct("PickCache");
        match index {
            Some(index) => out.field(
                "index",
                &format_args!("Some(<PickIndex for {:?}>)", index.generation()),
            ),
            None => out.field("index", &Option::<()>::None),
        };
        out.field("attempt", attempt)
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
    /// A build for exactly this picture has already been
    /// submitted and not answered — nothing was done and nothing was
    /// resubmitted.
    ///
    /// **A statement about the picture THIS sync asked about**, where
    /// [`PickCache::indexing`] is the standing fact the chrome reads.
    /// The two are not the same question: a frame that submits a new
    /// picture answers [`CacheStep::Submitted`] and is indexing all
    /// the same.
    Indexing,
    /// This attempt was already made and refused — nothing was done.
    Held,
    /// There is nothing to index: no evaluation has landed, or the
    /// one that has has no δ settled for it yet — the window between a
    /// document's arrival and the display budget's answer for it
    /// (`crate::evalseam::FitService`). Either way the cache forgets,
    /// so nothing describes a picture nobody can be shown.
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
    /// The answer was for a picture the cache has moved past,
    /// so it was dropped. Restart-without-cancel produces exactly
    /// this: the superseded build was allowed to finish.
    Stale,
}

impl PickCache {
    /// A cache over `seam`.
    pub fn new(seam: Box<dyn IndexService>) -> Self {
        Self {
            index: None,
            attempt: None,
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
    /// most one attempt per picture.
    ///
    /// **δ is built at, verbatim.** [`crate::scene::TRIANGLE_BUDGET`] chooses
    /// the δ a document OPENS at
    /// ([`crate::app::ViewerApp::fit_delta_on_scene`]), and that is the
    /// whole of the budget's authority: once a δ is in force it is the
    /// value someone asked for, and a cache that quietly built a
    /// different picture would make the View pane's δ field a control
    /// that does nothing.
    ///
    /// **`delta` is an `Option` because the budget answers off the
    /// frame.** `None` is *a run has landed and no δ is settled for
    /// it*, which is every frame between the fit's submit and its
    /// answer (`crate::evalseam::FitService`). It takes the same way
    /// out the nothing-landed arm does, and it is an `Option` rather
    /// than a caller's `if` so that the one un-budgeted build the
    /// budget exists to avoid cannot be submitted by forgetting to
    /// write one.
    ///
    /// The document is CLONED into the request and the evaluation is
    /// shared, so the worker owns everything it reads and the session
    /// goes on being edited. Both arrive on [`IndexInputs`], already
    /// paired: this cache is HANDED a landing and never reads one.
    pub fn sync(
        &mut self,
        landed: Option<IndexInputs<'_>>,
        delta: Option<DisplayTolerance>,
    ) -> CacheStep {
        // **The one way out on "nothing to index", and it FORGETS.**
        let (
            Some(IndexInputs {
                generation,
                doc,
                evaluation,
                tol,
            }),
            Some(delta),
        ) = (landed, delta)
        else {
            self.forget();
            return CacheStep::Nothing;
        };
        let wanted = PictureKey::of(generation, delta);
        if self
            .index
            .as_ref()
            .is_some_and(|index| index.current_for(Some(wanted)))
        {
            return CacheStep::Current;
        }
        match self.attempt {
            // Asked, and the answer is not here yet. Asking again
            // would be the per-frame rebuild loop with a thread in it.
            Some(Attempt::Asked(key)) if key == wanted => return CacheStep::Indexing,
            // Attempted and refused for this exact picture. Retrying
            // is the per-frame rebuild loop; the error is already
            // recorded and the caller has already seen it.
            Some(Attempt::Answered(key)) if key == wanted => return CacheStep::Held,
            _ => {}
        }
        self.attempt = Some(Attempt::Asked(wanted));
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
            key: wanted,
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
    /// key — for either of the two reasons [`CacheStep::Nothing`]
    /// names: a document was opened or a new one authored under a
    /// build that is still with the seam, or a document has landed and
    /// the δ to draw it at is still being fitted, so half the key does
    /// not exist yet. Leaving the attempt set would leave that build a
    /// key to match on arrival, and it would install — an index of a
    /// document nobody is looking at, over a scene of a third one,
    /// with nothing running and nothing said. Dropping the attempt is
    /// what turns that answer into [`IndexLanding::Stale`]; the other
    /// two fields go with it because all three describe the same
    /// vanished picture.
    ///
    /// **Exhaustive by destructuring, like the walk above.** A field
    /// added to [`PickCache`] is an unbound-pattern error here, so a
    /// fourth thing describing the picture cannot outlive the picture
    /// by being forgotten at the declaration and not here. `seam` is
    /// the one `_` arm and must be: it is the service, not the
    /// picture.
    fn forget(&mut self) {
        let Self {
            index,
            attempt,
            error,
            seam: _,
        } = self;
        *index = None;
        *attempt = None;
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
        if self.attempt.map(Attempt::key) != Some(done.key) {
            return IndexLanding::Stale;
        }
        self.attempt = Some(Attempt::Answered(done.key));
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
    ///
    /// **The cache's own record, and it is the only thing that can
    /// answer.** It says which PICTURE was asked for, where the seam
    /// knows only that it is busy, so a build whose answer is already
    /// destined for [`IndexLanding::Stale`] does not light the
    /// indicator: [`PickCache::forget`] drops the attempt when the
    /// picture stops existing, while the seam goes on building the
    /// orphan and goes on reporting itself busy for it.
    ///
    /// **And the seam cannot disagree in the other direction**, which
    /// is why it is not consulted as well. An attempt is recorded in
    /// the same step it is submitted, and from there the seam holds it
    /// — running or waiting — until it hands back the answer
    /// [`PickCache::pump`] takes straight to [`PickCache::land`],
    /// which is what moves the attempt off `Asked`. The one seam that
    /// could be idle under a standing `Attempt::Asked` is one whose
    /// worker has gone, and that is not a state an implementation may
    /// be in: a seam that loses its worker ends the process at the
    /// point of detection (`crate::evalseam`) rather than going quiet.
    pub fn indexing(&self) -> bool {
        matches!(self.attempt, Some(Attempt::Asked(_)))
    }

    /// Why the last attempt refused, if it did.
    pub fn error(&self) -> Option<&PickIndexError> {
        self.error.as_ref()
    }
}

/// **A pick attempted while no index describes the picture on
/// screen** — the typed *not indexed yet*.
///
/// Distinct from a miss, and that distinction is the whole of it. A
/// miss is an answer: the index was asked and there is nothing under
/// the cursor, so clearing the selection is right. This is having
/// nobody to ask about what is on screen, and doing nothing quietly is
/// what made the window between two indexes look like a viewport that
/// had decided the user was pointing at empty space.
///
/// **The arms are named for what is observably true rather than for a
/// cause**, so none can be shown over a state it does not describe: a
/// refused build and a document that has never been evaluated are both
/// "no index and nobody building one", and a sentence promising an
/// answer shortly would be false in both. Waiting and not waiting are
/// different advice, and so is an index that exists for a picture
/// nobody is looking at.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NotIndexed {
    /// A build is under way ([`PickCache::indexing`]): the answer is
    /// coming, and the toolbar is already saying so.
    Building,
    /// No index, and no build under way — the last attempt refused
    /// (its reason is [`PickCache::error`]), or nothing has been
    /// evaluated yet. Waiting will not help; the retry policy holds
    /// until the generation or δ moves.
    ///
    /// **The observable is the arm and the causes are a list, not a
    /// definition**, which is what puts two of them under one name:
    /// either way there is no index and nobody building one, and the
    /// sentence [`NotIndexed::Building`] carries would be a promise
    /// nobody can keep.
    Absent,
    /// An index is in hand and it did not mint the picture's corners:
    /// it describes a rebuild that has not been drawn.
    ///
    /// **The one arm that is not about an absence**, and it is here
    /// rather than in a vocabulary of its own because this type's
    /// subject is *no index describes the picture on screen* and this
    /// is the third way that sentence is true. `ViewerApp::sync_scene`
    /// marks the scene's `(generation, δ)` pair current only on a
    /// successful rebuild — a refused one must not consume the pair,
    /// or the stale picture stays marked as current and is never
    /// retried — so a landing over a refused rebuild leaves a newer
    /// index beside an older picture, and nothing retries while the
    /// display revision and the focus set hold still.
    ///
    /// **Refusing is a ruling, not a repair** (Ev, 2026-09-15). The
    /// index would happily answer, and the answer would name geometry
    /// the screen is not showing; a selection the user cannot see is a
    /// worse outcome than a click that says why it did nothing.
    ///
    /// **What retires it is a scene rebuild**, not an index build —
    /// the other two arms' event. Both seams sit under
    /// [`crate::frame::Subject::Display`], which is the coarser
    /// question of what retires a fact, so the subject
    /// [`crate::frame::unindexed_refusal`] reads off this type is
    /// right for this arm too.
    AnotherPicture,
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
            Self::AnotherPicture => write!(
                f,
                "not picked: the picture on screen is older than the document \
                 under the cursor — the index that would answer this click \
                 describes a rebuild that has not been drawn, and a pick is \
                 answered about what is on screen or not at all"
            ),
        }
    }
}

impl core::error::Error for NotIndexed {}

/// The refusal a pick stream earns when no index describes the picture
/// on screen — `Some` for an ACT, `None` for an observation.
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
/// **Which sentence is true is read from what the pane holds**, not
/// chosen at the call site. `held` is the index in hand
/// ([`PickCache::index`]) and `indexing` is [`PickCache::indexing`].
///
/// **The two cannot both be set**, so they are not a pair of flags a
/// caller could swap: [`PickCache::sync`] drops the held index in the
/// same step that marks a build outstanding, which is the whole of
/// *current or absent, never behind* above.
///
/// **This door is asked where no index describes the picture on
/// screen** — the `else` of the pane's one currency read, which is
/// `pane::viewport`'s `drawn_index`. A `Some` in `held` there is
/// therefore an index for a DIFFERENT picture, which is
/// [`NotIndexed::AnotherPicture`]. The precondition is the caller's
/// because the scene's `(generation, δ)` is the pane's and not this
/// module's; nothing here can re-derive it, and a second derivation
/// of it is exactly what the one currency read exists to prevent.
pub fn unindexed<'a>(
    actions: impl IntoIterator<Item = &'a PickAction>,
    held: Option<&PickIndex>,
    indexing: bool,
) -> Option<NotIndexed> {
    actions
        .into_iter()
        .any(|action| match action {
            // An ACT: the user asked for something and did not get it.
            PickAction::Select(_) => true,
            // Observations.
            PickAction::Hover(_) | PickAction::ClearHover => false,
        })
        .then_some(match (held, indexing) {
            (Some(_), _) => NotIndexed::AnotherPicture,
            (None, true) => NotIndexed::Building,
            (None, false) => NotIndexed::Absent,
        })
}
