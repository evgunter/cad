//! The three seams the picture is built across — where a document
//! becomes a result DAG, where the display budget prices that result
//! to choose the δ it is drawn at, and where the DAG becomes the pick
//! index the viewport draws and picks against. **The one place in this
//! crate that OWNS a thread**: `app` spawns all three workers because
//! it decides which implementation this build runs, and every join
//! handle, channel and hand-off between them is here.
//!
//! # Why a seam at all
//!
//! Evaluation is the only unbounded computation the interaction layer
//! provokes, and the layer above it must never assume how it runs. The
//! GUI plan carries that as a standing constraint for the web lane:
//! natively a background thread, on wasm a Worker or an inline slice,
//! with no source change above this boundary. So the vocabulary here
//! is submit / poll over a [`Generation`] — plus cancel, for the one
//! of the three whose work can be stopped — and
//! [`InlineEvaluator`] — which runs the whole evaluation inside
//! `poll` — satisfies it exactly as well as [`ThreadEvaluator`] does.
//! Rows in this crate's suite drive both, and the application drives
//! the threaded one; nothing else changes.
//!
//! # The policy for an edit during an evaluation: CANCEL AND RESTART
//!
//! A submit while a run is in flight cancels that run and starts the
//! new document. The alternative — queue and let the old run finish —
//! spends work on a document nobody is looking at any more, and its
//! result would be discarded by [`Generation`] anyway. Multiple
//! submits while busy COALESCE: only the newest document is held, so a
//! slider drag that outruns the evaluator produces at most one
//! evaluation per completed run rather than a backlog of them, and a
//! superseded run's result is dropped HERE rather than travelling up to
//! be discarded by generation.
//!
//! **Every implementation of every seam does this, and there are two
//! mechanisms rather than six**: at most one request is ever
//! outstanding, and a submit while one is outstanding REPLACES the
//! waiting request rather than adding to it. An INLINE seam is a
//! single `Option` and nothing else — `Option::replace` is
//! latest-wins, `busy` is the slot, and there is no worker to be
//! ahead of — so the rule holds there by the type rather than by a
//! machine anyone maintains. A THREADED seam is that same `Option`
//! in the handle plus a `running` flag, so the channel to the worker
//! never holds more than one job; that half IS a machine, and it is
//! written ONCE — the `Coalescing` handle in the native module below,
//! which every threaded seam is built from and which is where the
//! whole invariant, `busy` included, is stated. Making the worker
//! drain a queue would have produced the same observable answer, but
//! it would have left the handle's own accounting (what `busy`
//! reports) describing a queue the caller cannot see; keeping the
//! queue in the handle is why the implementations are the same shape
//! rather than merely the same outcome. The row that pins it drives
//! [`InlineEvaluator`] and [`ThreadEvaluator`] both.
//!
//! The cancelation is the shipped `CancelToken` and nothing else: it
//! is checked between nodes, so a canceled run returns its completed
//! prefix typed as `EvalOutcome::Canceled`. That prefix answers a
//! document nobody asked to see half of, so it never becomes anyone's
//! picture: the memo is primed only from a completed run
//! ([`run_once`]), and the session refuses to land a result that is not
//! [`EvalDone::completed`] (`DocSession::land`). A cancel therefore
//! leaves the last good evaluation on screen and the session still
//! owing an answer — which `DocSession::running` distinguishes from
//! "an answer is coming", so the chrome can say which.
//!
//! # Staleness is decided here, by generation
//!
//! Results carry the generation of the request that produced them.
//! [`EvalService::poll`] can hand back a result for a document two
//! edits old; the session that owns the seam compares generations and
//! drops it. That rule is a pure function of two integers, which is
//! why it is testable without a thread in sight.
//!
//! # The index seam, and the two ways it is WEAKER than this one
//!
//! [`IndexService`] carries the second half of the same journey: a
//! landed evaluation plus a δ becomes a [`crate::pickindex::PickIndex`],
//! which is the tessellation the viewport draws AND the structure
//! every pick is answered from. It is the same vocabulary —
//! submit / poll over a [`Generation`] — and its two implementations
//! stand in the same relation, [`InlineIndexer`] for the browser and
//! the tests and [`ThreadIndexer`] for the application.
//!
//! **It has no `cancel`, and that is a promise this seam makes and
//! that one does not.** A cancelation here could only be the shipped
//! `CancelToken`, which is checked BETWEEN NODES; the step an index
//! build is made of — `mesh::tessellate` and the triangle BVH — has no
//! nodes to be checked between and takes no token at all. So the
//! policy is **restart without cancel**: a submit while a build is in
//! flight lets that build run to completion and drops its answer
//! inside the seam. The cost is one wasted build, and on a document
//! whose index takes 13 s a δ change made during it costs about 27 s
//! before the picture is right. The trait therefore offers no cancel
//! door rather than one that would quietly do nothing.
//!
//! **It is a SECOND worker, not a second payload on the first one.**
//! One worker would put an uninterruptible multi-second index build in
//! front of the next evaluation, so an edit made during it would wait
//! for it — which is exactly the cancel-and-restart promise the
//! evaluation seam makes above. A seam cannot keep that promise behind
//! a queue it does not control, so the two runs are concurrent and the
//! superseded one is discarded by its key rather than stopped.
//!
//! **Its key is a PAIR**, `(generation, δ)`, because δ is an input to
//! the tessellation and not to the evaluation: an index answered for
//! the generation on screen at a δ nobody asked for any more is as
//! wrong as one answered for the wrong document, and only the pair
//! separates them.
//!
//! Module kind: **vocabulary** — it names no driver type and no
//! `app`-only crate (`crates/viewer/README.md`, Module boundaries).

use std::sync::Arc;

use pncad::document::{
    CancelToken, Doc, EvalOptions, EvalOutcome, Evaluation, PartResolver, ProfileProgram, evaluate,
};
use pncad::geom_core::Tol;
use pncad::select::PickMemo;
use pncad::topo::Body;

use crate::generation::Generation;
use crate::pickindex::{PickIndex, PickIndexError, PictureKey};
use crate::scene::{DisplayTolerance, FittedDelta, SceneError, fit_delta, product_of_evaluation};

/// What the seam was asked to evaluate.
#[derive(Clone, Debug)]
pub struct EvalRequest {
    /// The document version this request is for.
    pub generation: Generation,
    /// The document itself — a value, so the worker owns its copy and
    /// the interaction layer keeps editing.
    pub doc: Doc<ProfileProgram>,
    /// The ε the run decides at.
    pub tol: Tol,
    /// The document seam this run resolves `InstantiatePart` nodes
    /// through — the session's workspace over the opened file's own
    /// directory, or `None` for a document with no backing file, in
    /// which case every instantiate node refuses typed (the shipped
    /// no-resolver semantics, rendered as the tree's badges). Shared
    /// by `Arc` so the worker holds a handle, not a copy of the store.
    pub resolver: Option<Arc<dyn PartResolver>>,
}

/// A finished run.
#[derive(Clone, Debug)]
pub struct EvalDone {
    /// The generation the request carried.
    pub generation: Generation,
    /// The result DAG, shared rather than cloned: the panels, the
    /// scene and the memo all read the same value.
    pub evaluation: Arc<Evaluation<f64>>,
}

impl EvalDone {
    /// Whether this run reached every scheduled node.
    pub fn completed(&self) -> bool {
        self.evaluation.outcome == EvalOutcome::Completed
    }
}

/// The seam's vocabulary.
///
/// Implementations own the memo: `prior` is never handed across this
/// boundary, because a caller holding the previous evaluation to pass
/// back in is a caller that has assumed the run is synchronous.
pub trait EvalService {
    /// Ask for `request`. Cancels an in-flight run for an older
    /// generation and replaces any queued one.
    fn submit(&mut self, request: EvalRequest);

    /// Cancel whatever is in flight. A cancelation is not a failure:
    /// the run returns its completed prefix and the session shows the
    /// last landed result.
    fn cancel(&mut self);

    /// Take a finished run, if one is ready. Never blocks.
    fn poll(&mut self) -> Option<EvalDone>;

    /// Whether a run is in flight or queued.
    fn busy(&self) -> bool;
}

/// The previous completed run, together with the resolver that ran it
/// — the memo's priming source, and the identity that bounds it.
#[derive(Debug)]
struct PriorRun {
    /// The resolver the run resolved through. `None` for a run with
    /// none.
    resolver: Option<Arc<dyn PartResolver>>,
    /// The completed evaluation.
    evaluation: Arc<Evaluation<f64>>,
}

/// Whether two requests' resolvers are the same SEAM, by `Arc`
/// identity.
///
/// Pointer identity is the honest key here: a resolver value is
/// immutably bound to its directory, and the session replaces the
/// `Arc` exactly when that binding changes (open, save-as into a new
/// directory). Comparing by directory instead would treat a rebind to
/// the same path as a change (harmless) and, worse, would need the
/// trait to expose an identity it does not have.
fn same_resolver(a: &Option<Arc<dyn PartResolver>>, b: &Option<Arc<dyn PartResolver>>) -> bool {
    match (a, b) {
        (None, None) => true,
        (Some(a), Some(b)) => Arc::ptr_eq(a, b),
        _ => false,
    }
}

/// Run the evaluation, priming the memo from `prior` and updating it
/// when the run completes.
///
/// The seam's one call into `evaluate`, shared by both implementations
/// so the memo discipline the SEAM owns has a single home. Two of the
/// three rules are that discipline and live here:
///
/// - **prime from the previous COMPLETED run only**;
/// - **prime only under the SAME RESOLVER** ([`same_resolver`]). A
///   memoized instantiate-node result is an answer the old resolver
///   gave; priming a run whose resolver moved would let the memo
///   answer for a directory nobody consulted — the silent-divergence
///   class the directory rule exists to prevent, and exactly what
///   made the save-as rebind inert before this gate existed. A
///   resolver replacement therefore costs one full re-evaluation, by
///   design: the next run re-resolves every reference against the new
///   directory.
///
/// The third rule is **prime only from a run of the SAME DOCUMENT**,
/// and its home is the KERNEL, not here (DI3): `evaluate` drops a
/// prior whose document id is not the one being evaluated, before it
/// builds the schedule, and reports the drop as
/// `Evaluation::prior_refused`. This function does not re-check it —
/// there would be no point, the kernel's check is the authority — but
/// it does READ the report, because a `PriorRun` that got refused is
/// one the seam should not keep offering: the session's document was
/// replaced under it (a file opened into the same session), so the
/// held run is about a document nobody is looking at any more and
/// every later run would pay the same refusal. Dropping it makes the
/// NEXT run's `prime` honestly `None` instead of a value the kernel
/// throws away.
fn run_once(
    request: &EvalRequest,
    prior: &mut Option<PriorRun>,
    cancel: &CancelToken,
) -> Arc<Evaluation<f64>> {
    let prime = prior
        .as_ref()
        .filter(|p| same_resolver(&p.resolver, &request.resolver))
        .map(|p| p.evaluation.as_ref());
    let evaluation = Arc::new(evaluate::<f64>(
        &request.doc,
        prime,
        cancel,
        &EvalOptions {
            resolver: request.resolver.clone(),
            ..EvalOptions::default()
        },
        request.tol,
    ));
    // The kernel refused what we primed with: the held run is of
    // another document, so drop it rather than offer it again. Read
    // before the store below, which overwrites it on a completed run
    // anyway — the drop is what a CANCELED run needs, since that path
    // leaves `prior` untouched.
    if evaluation.prior_refused.is_some() {
        *prior = None;
    }
    if evaluation.outcome == EvalOutcome::Completed {
        *prior = Some(PriorRun {
            resolver: request.resolver.clone(),
            evaluation: Arc::clone(&evaluation),
        });
    }
    evaluation
}

/// The seam with no thread behind it: `submit` records the request and
/// `poll` runs it.
///
/// This is the wasm shape and the test shape, and it is a complete
/// implementation rather than a stub — which is the point of the
/// boundary. What it cannot do is keep the frame responsive during a
/// long evaluation; what it proves is that nothing above the seam
/// depends on being able to.
#[derive(Debug, Default)]
pub struct InlineEvaluator {
    pending: Option<EvalRequest>,
    prior: Option<PriorRun>,
    cancel: CancelToken,
}

impl InlineEvaluator {
    /// A seam that has evaluated nothing.
    pub fn new() -> Self {
        Self::default()
    }
}

impl EvalService for InlineEvaluator {
    fn submit(&mut self, request: EvalRequest) {
        // The whole machine is the `Option` (module docs, the
        // coalescing section): nothing has started, so the newer
        // request replaces the older one. The token is fresh because
        // the run this cancel would have stopped is the one being
        // replaced.
        self.cancel = CancelToken::new();
        self.pending = Some(request);
    }

    fn cancel(&mut self) {
        self.cancel.cancel();
    }

    fn poll(&mut self) -> Option<EvalDone> {
        let request = self.pending.take()?;
        let evaluation = run_once(&request, &mut self.prior, &self.cancel);
        Some(EvalDone {
            generation: request.generation,
            evaluation,
        })
    }

    fn busy(&self) -> bool {
        self.pending.is_some()
    }
}

// --- the index seam -------------------------------------------------

/// What the index seam was asked to build.
///
/// Everything [`crate::pickindex::PickIndex::build`] reads, as owned values,
/// so the worker holds its own copy of the document and a handle on the
/// evaluation while the interaction layer keeps editing.
#[derive(Clone, Debug)]
pub struct IndexRequest {
    /// The picture to build — the generation of the evaluation this
    /// index describes and the chordal tolerance its roots are
    /// tessellated at, as one value ([`PictureKey`]). The δ half is
    /// the half the evaluation knows nothing about.
    pub key: PictureKey,
    /// The document whose roots are walked.
    pub doc: Doc<ProfileProgram>,
    /// The run those roots' payloads are read from. Shared rather than
    /// copied: the panels, the scene and this build read one value.
    pub evaluation: Arc<Evaluation<f64>>,
    /// The ε the tessellation decides at.
    pub tol: Tol,
}

/// A finished index build, carrying **the whole key it was built for**.
///
/// The refusal arm carries no index to read a generation off, and the
/// success arm's δ is one a caller would have to reach through the
/// index to see. A result that cannot state its own key can only be
/// matched against the request by trusting the order it arrived in,
/// which is the assumption a coalescing seam exists to break.
#[derive(Debug)]
pub struct IndexDone {
    /// The picture the request asked for.
    pub key: PictureKey,
    /// What the seam's memo did for this answer.
    pub memo: MemoReport,
    /// The index, or the refusal that stopped it — a failed or
    /// poisoned root is an ordinary editing state and its refusal is
    /// the answer, not an absence.
    pub index: Result<PickIndex, PickIndexError>,
}

/// What the seam's memo did for one answer, and what it holds after
/// it: the counts of the picture just closed. Carried on
/// [`IndexDone`] so a consumer of either implementation reads the
/// reuse the same way; the threaded seam's memo is otherwise
/// unreachable from the thread that asked.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct MemoReport {
    /// (node, body) picks held after the build.
    pub nodes: usize,
    /// (node, body) picks answered without a build.
    pub node_hits: usize,
    /// (node, body) picks built (through the patch memo).
    pub node_misses: usize,
    /// Face patches held after the build.
    pub faces: usize,
    /// Faces answered from the memo.
    pub face_hits: usize,
    /// Faces run through their lane.
    pub face_misses: usize,
    /// The patch memo's heap footprint, approximately.
    pub bytes: usize,
    /// Per-patch pick tables held after the build.
    pub tables: usize,
    /// Patches whose whole pick table was served from the memo.
    pub table_hits: usize,
    /// Patches whose pick table was built.
    pub table_misses: usize,
}

impl MemoReport {
    /// The memo's counts, read after a picture closed.
    pub fn of(memo: &PickMemo) -> Self {
        let patches = memo.patches();
        Self {
            nodes: memo.len(),
            node_hits: memo.node_hits(),
            node_misses: memo.node_misses(),
            faces: patches.len(),
            face_hits: patches.hits(),
            face_misses: patches.misses(),
            bytes: patches.bytes(),
            tables: memo.table_len(),
            table_hits: memo.table_hits(),
            table_misses: memo.table_misses(),
        }
    }
}

/// The index seam's vocabulary — [`EvalService`]'s shape, minus the
/// door it cannot honestly offer.
///
/// **There is no `cancel`.** The module docs carry the argument: the
/// step behind this seam is uninterruptible, so a cancel could only
/// set a token nothing reads. The policy is restart without cancel,
/// and a submit while a build is in flight is how it is spelled.
pub trait IndexService {
    /// Ask for `request`. A build already in flight runs to completion
    /// and its answer is dropped; a request only WAITING is replaced.
    fn submit(&mut self, request: IndexRequest);

    /// Take a finished build, if one is ready. Never blocks.
    fn poll(&mut self) -> Option<IndexDone>;

    /// Whether a build is in flight or waiting.
    fn busy(&self) -> bool;
}

/// Run one index build over the seam's memo, stamping the answer with
/// the request's own key.
///
/// The seam's one call into [`crate::pickindex::PickIndex::build_with`],
/// shared by both implementations, so the generation the index is
/// built under and the generation the answer is filed under are read
/// from one place and cannot disagree — and so both implementations
/// reuse across pictures by the same rule.
///
/// **The memo is the seam's.** An index is still discarded whole and
/// rebuilt whole above this seam (`crate::pickindex`'s staleness
/// rule); what survives between builds is HERE, where the previous
/// picture already lived: the previous generation's `NodePick`s by
/// (node, body) under the evaluation's content keys, and the per-face
/// patch memo under them (`PickMemo`'s docs). A build answers a reused
/// root's pick without touching it and a recomputed root's unchanged
/// faces without meshing them; the answer is byte-identical to a build
/// with no memo, and the rows in `tests/index_memo.rs` are the proof.
fn build_index(request: &IndexRequest, memo: &mut PickMemo) -> IndexDone {
    let index = PickIndex::build_with(
        &request.doc,
        &request.evaluation,
        request.key,
        request.tol,
        memo,
    );
    IndexDone {
        key: request.key,
        memo: MemoReport::of(memo),
        index,
    }
}

/// The index seam with no thread behind it: `submit` records the
/// request and `poll` builds it.
///
/// The wasm shape and the test shape, and a complete implementation
/// rather than a stub — the same standing [`InlineEvaluator`] has, for
/// the same reason. What it cannot do is keep the frame responsive
/// while a document is indexed; what it proves is that nothing above
/// the seam depends on being able to.
#[derive(Debug, Default)]
pub struct InlineIndexer {
    pending: Option<IndexRequest>,
    /// What the previous picture left behind ([`build_index`]).
    memo: PickMemo,
}

impl InlineIndexer {
    /// A seam that has built nothing.
    pub fn new() -> Self {
        Self::default()
    }

    /// The memo the next build primes from — what the previous
    /// picture left behind, and the counts of the build that left it.
    pub fn memo(&self) -> &PickMemo {
        &self.memo
    }
}

impl IndexService for InlineIndexer {
    fn submit(&mut self, request: IndexRequest) {
        self.pending = Some(request);
    }

    fn poll(&mut self) -> Option<IndexDone> {
        let request = self.pending.take()?;
        Some(build_index(&request, &mut self.memo))
    }

    fn busy(&self) -> bool {
        self.pending.is_some()
    }
}

// --- the display budget's fit -----------------------------------------

/// What the display budget was asked to price: a landed body, and the
/// δ someone wants to see it at.
///
/// **Keyed by generation alone**, where [`IndexRequest`]'s key is a
/// pair. δ is this request's ANSWER and not an input to it, so two
/// outstanding fits of one landing could not be two different wants.
/// The requested δ rides along so an answer can be recognised as being
/// about a number the View pane has since moved off — see
/// [`FitDone::requested`].
#[derive(Clone, Debug)]
pub struct FitRequest {
    /// The landing this fit prices.
    pub generation: Generation,
    /// The δ that was asked for — what the ladder prices, and may
    /// coarsen.
    pub requested: DisplayTolerance,
    /// What to probe.
    pub subject: FitSubject,
    /// The ε the probe tessellations run at.
    pub tol: Tol,
}

/// The body a fit probes, in the two shapes a landing leaves it.
///
/// Read off the landing by the SUBMITTER, where that landing is
/// (`crate::app::ViewerApp::sync_scene`), so this seam names no
/// session.
#[derive(Clone, Debug)]
pub enum FitSubject {
    /// The landing's own body, shared rather than copied: the gather
    /// that produced it was paid once, where the result became the
    /// session's (`crate::session::DocSession::land`).
    Landed(Arc<Body<f64>>),
    /// A landing whose A5 gate consumed the product it judged. There
    /// is no body to share, so the fit gathers one — **here, on the
    /// worker**, which is half of what this seam buys: a gather is
    /// unbounded per-document work, and the frame is where this one
    /// used to run.
    Ungathered {
        /// The document, shared: this is the landing's own copy, which
        /// nothing edits.
        doc: Arc<Doc<ProfileProgram>>,
        /// The run it gathers, shared.
        evaluation: Arc<Evaluation<f64>>,
    },
}

/// A finished fit.
///
/// Not `Clone`, and neither is [`IndexDone`]: an answer carries what a
/// request does not — here a [`SceneError`] on the refusing arm, there
/// a built index — and one answer has exactly one reader.
#[derive(Debug)]
pub struct FitDone {
    /// The generation the request carried.
    pub generation: Generation,
    /// The δ the request asked for. The submitter compares it against
    /// the δ in force now: a user who typed a δ while this ran has
    /// asked for that number verbatim, and the budget's authority is
    /// over the δ a document OPENS at and nothing else
    /// (`crate::pickcache::PickCache::sync`).
    pub requested: DisplayTolerance,
    /// The budget's answer, or why it had none.
    pub fit: Result<FittedDelta, SceneError>,
}

/// The fit seam's vocabulary — [`IndexService`]'s shape, and for the
/// same reason it has that shape rather than [`EvalService`]'s: the
/// step behind it is a ladder of `pncad::mesh::tessellate` calls, none
/// of which reads a token, so the policy is restart without cancel.
pub trait FitService {
    /// Ask for `request`. A fit already in flight runs to completion
    /// and its answer is dropped; a request only WAITING is replaced.
    fn submit(&mut self, request: FitRequest);

    /// Take a finished fit, if one is ready. Never blocks.
    fn poll(&mut self) -> Option<FitDone>;

    /// Whether a fit is in flight or waiting.
    fn busy(&self) -> bool;
}

/// Run one fit, stamping the answer with the request's own key.
///
/// The seam's one call into [`crate::scene::fit_delta`], shared by
/// both implementations for [`build_index`]'s reason: the key the
/// answer is filed under is read from the request that produced it,
/// in one place, so the two cannot disagree.
fn run_fit(request: &FitRequest) -> FitDone {
    let fit = match &request.subject {
        FitSubject::Landed(body) => fit_delta(body, request.requested, request.tol),
        FitSubject::Ungathered { doc, evaluation } => {
            product_of_evaluation(doc, evaluation, request.tol)
                .and_then(|body| fit_delta(&body, request.requested, request.tol))
        }
    };
    FitDone {
        generation: request.generation,
        requested: request.requested,
        fit,
    }
}

/// The fit seam with no thread behind it: `submit` records the request
/// and `poll` runs it — [`InlineIndexer`]'s standing, for
/// [`InlineEvaluator`]'s reason.
#[derive(Debug, Default)]
pub struct InlineFitter {
    pending: Option<FitRequest>,
}

impl InlineFitter {
    /// A seam that has fitted nothing.
    pub fn new() -> Self {
        Self::default()
    }
}

impl FitService for InlineFitter {
    fn submit(&mut self, request: FitRequest) {
        self.pending = Some(request);
    }

    fn poll(&mut self) -> Option<FitDone> {
        let request = self.pending.take()?;
        Some(run_fit(&request))
    }

    fn busy(&self) -> bool {
        self.pending.is_some()
    }
}

/// **The seam's traffic is `Send`, checked here rather than assumed.**
///
/// The threaded implementation would fail to compile without it, but
/// it is absent from the wasm build entirely — so on the target where
/// a Worker-backed sibling is the whole point, nothing would catch an
/// `Rc` growing into a mesh, a BVH or a name table. This costs one
/// monomorphisation and holds on every target.
const _: fn() = || {
    fn assert_send<T: Send>() {}
    assert_send::<EvalRequest>();
    assert_send::<EvalDone>();
    assert_send::<IndexRequest>();
    assert_send::<IndexDone>();
    assert_send::<FitRequest>();
    assert_send::<FitDone>();
    // The memo moves onto the worker thread with the loop that owns it.
    assert_send::<PickMemo>();
};

#[cfg(not(target_family = "wasm"))]
pub use threaded::{SpawnError, ThreadEvaluator, ThreadFitter, ThreadIndexer, Worker};

/// The native seam: one worker thread, a request channel, a result
/// channel.
///
/// Behind `cfg(not(target_family = "wasm"))` because `thread::spawn`
/// is what it is built on and the browser has no such thing — the
/// module is absent there and [`InlineEvaluator`] is what the wasm
/// build uses until a Worker-backed sibling lands beside this one.
#[cfg(not(target_family = "wasm"))]
mod threaded {
    use std::sync::mpsc::{Receiver, Sender, TryRecvError, channel};
    use std::thread::JoinHandle;

    use pncad::document::CancelToken;

    use super::{
        EvalDone, EvalRequest, EvalService, FitDone, FitRequest, FitService, IndexDone,
        IndexRequest, IndexService, PickMemo, PriorRun, build_index, run_fit, run_once,
    };

    /// Which of this module's three workers a refusal is about (D4 ¶3:
    /// a closed enum, because the set is this file's own and a reader
    /// asking which values occur should be able to see them).
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Worker {
        /// The evaluation worker ([`ThreadEvaluator`]).
        Evaluation,
        /// The index worker ([`ThreadIndexer`]).
        Index,
        /// The display budget's fit worker ([`ThreadFitter`]).
        Fit,
    }

    impl core::fmt::Display for Worker {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.write_str(match self {
                Self::Evaluation => "evaluation",
                Self::Index => "index",
                Self::Fit => "display fit",
            })
        }
    }

    /// Why a worker could not be started.
    ///
    /// The worker is NAMED, because this crate spawns one per seam and
    /// a startup refusal that did not say which would send its reader
    /// to the wrong third of this module.
    #[derive(Debug)]
    pub enum SpawnError {
        /// The OS refused the thread.
        Thread {
            /// Which worker.
            worker: Worker,
            /// What the OS said.
            error: std::io::Error,
        },
    }

    impl core::fmt::Display for SpawnError {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            match self {
                Self::Thread { worker, error } => {
                    write!(
                        f,
                        "the {worker} worker could not be started: the OS refused \
                         the thread: {error}"
                    )
                }
            }
        }
    }

    impl core::error::Error for SpawnError {}

    /// What a worker carries, what it answers with, and **the one rule
    /// that differs between the three seams**.
    ///
    /// Everything else about carrying a job is [`Coalescing`]'s and is
    /// written once there; this trait is where a seam says the part
    /// that is its own.
    trait Job: Send + 'static {
        /// What the worker sends back for one job.
        type Done: Send + 'static;

        /// Whether `self`, which was WAITING when `done` arrived, asks
        /// for something `done` does not already answer.
        ///
        /// `false` drops the waiting job and keeps the answer in hand.
        /// **This is the only place a seam's notion of "the same
        /// picture" is written**, so a seam that keys on a pair and a
        /// seam that keys on a generation differ here and nowhere
        /// else.
        fn supersedes(&self, done: &Self::Done) -> bool;
    }

    /// **The coalescing machine, written once.** Every threaded seam in
    /// this module is one of these plus whatever else that seam owns.
    ///
    /// The invariant, in one place rather than once per seam, because
    /// four hand-maintained copies of it are four things that can
    /// silently disagree:
    ///
    /// - **at most one outstanding**: the worker holds at most one job,
    ///   which is why `running` is a flag and not a count, and at most
    ///   one more job is `waiting`;
    /// - **latest wins**: a submit while the worker is busy REPLACES
    ///   `waiting` rather than queueing behind it;
    /// - **a worker ends in exactly one of two ways, and they are not
    ///   the same event**: [`Coalescing::close`] took its request
    ///   channel, which is an orderly shutdown and forgets the work
    ///   quietly, clearing `running` and `waiting` so `busy` stops
    ///   claiming an answer that is not coming
    ///   ([`Coalescing::forget_worker`]); or the worker
    ///   CRASHED under a job, which is a bug in this process and ends
    ///   it ([`Coalescing::crashed`]). Both are noticed at the same
    ///   two places — a `send` that fails and a `Disconnected` receive
    ///   — and telling them apart is a read of `to_worker`, which
    ///   `close` is the only thing that takes;
    /// - **`busy()` is `running || waiting.is_some()`** — the two
    ///   fields are what the caller's one boolean is computed from, and
    ///   nothing else may compute it.
    ///
    /// What is NOT here is what each seam does differently: whether a
    /// waiting job supersedes the answer in hand
    /// ([`Job::supersedes`]), whether there is a token to cancel with,
    /// and whether `Drop` may wait for the worker
    /// ([`Coalescing::close_and_join`] against [`Coalescing::close`]).
    #[derive(Debug)]
    struct Coalescing<J: Job> {
        /// Which seam this is. Carried so a crash names the worker it
        /// was, rather than being labelled by whichever consumer
        /// happened to be the one that noticed.
        worker: Worker,
        to_worker: Option<Sender<J>>,
        from_worker: Receiver<J::Done>,
        /// Whether the worker holds a job. A flag rather than a count,
        /// because the channel never holds more than one.
        running: bool,
        /// The newest job, held back until the worker is free.
        /// Replaced, never appended to: that is latest-wins.
        waiting: Option<J>,
        handle: Option<JoinHandle<()>>,
    }

    impl<J: Job> Coalescing<J> {
        /// Spawn the worker and return the handle on it.
        ///
        /// `answer` is the seam's own work, run on the worker thread
        /// over `state` — the previous completed run an evaluation
        /// worker primes its memo from, the [`PickMemo`] an index
        /// worker keeps between builds, `()` for a seam with nothing to
        /// carry. **That state lives on the worker thread for the
        /// thread's life, which is the seam's**, so nothing above the
        /// seam holds it in order to hand it back.
        ///
        /// # Errors
        ///
        /// [`SpawnError::Thread`] if the OS refuses the thread. Loud
        /// rather than degraded on purpose: a seam whose worker never
        /// started accepts every submit and answers none, so the
        /// application would sit behind an indicator forever with no
        /// failure anywhere to read.
        fn spawn<S: Send + 'static>(
            worker: Worker,
            thread_name: &str,
            mut state: S,
            mut answer: impl FnMut(&mut S, J) -> J::Done + Send + 'static,
        ) -> Result<Self, SpawnError> {
            let (to_worker, requests) = channel::<J>();
            let (results, from_worker) = channel::<J::Done>();
            let handle = std::thread::Builder::new()
                .name(thread_name.to_owned())
                .spawn(move || {
                    // The worker loop, once for all three seams:
                    // answer one job at a time — which is the other
                    // half of at-most-one-outstanding, the half that
                    // lives on this side of the channel — and stop
                    // when either end closes. A failed `send` means
                    // the handle is gone, so there is nobody left to
                    // answer.
                    while let Ok(job) = requests.recv() {
                        if results.send(answer(&mut state, job)).is_err() {
                            return;
                        }
                    }
                })
                .map_err(|error| SpawnError::Thread { worker, error })?;
            Ok(Self {
                worker,
                to_worker: Some(to_worker),
                from_worker,
                running: false,
                waiting: None,
                handle: Some(handle),
            })
        }

        /// Take `job`: hand it to the worker if it is free, otherwise
        /// hold it as the one waiting job.
        fn submit(&mut self, job: J) {
            if self.running {
                // Latest wins: the previous waiting job is dropped, not
                // queued behind this one.
                self.waiting = Some(job);
            } else {
                self.dispatch(job);
            }
        }

        /// Hand `job` to the worker — or find out that there is none.
        ///
        /// **The two ways there is none are different events**, and
        /// this is one of the two places the difference is read. A
        /// sender still in hand whose `send` failed means the receiving
        /// end went down with the thread holding it, which is a crash;
        /// a sender that is gone means [`Coalescing::close`] took it,
        /// which is shutdown.
        fn dispatch(&mut self, job: J) {
            match self.to_worker.as_ref() {
                Some(to_worker) if to_worker.send(job).is_ok() => self.running = true,
                Some(_) => self.crashed(),
                None => self.forget_worker(),
            }
        }

        /// **A worker thread crashed, so this process ends here** (Ev,
        /// in-chat, 2026-09-17: *"isn't a worker dying an infra thing
        /// that should show up as a panic?"*, then *"panic on crash is
        /// good"*).
        ///
        /// # Why a panic and not a typed fact
        ///
        /// A crash is not a state the application can be IN. Nothing
        /// respawns a worker, so a seam that loses one accepts every
        /// later submit and answers none, for the life of the window —
        /// and every consumer above the seam reads that as an idle
        /// seam, because the reset that stops the indicator lying
        /// clears the very fields a consumer would have to read to
        /// tell the two apart. Describing that state in the chrome
        /// means carrying a vocabulary for a condition no document and
        /// no gesture can cause; ending the process says the same
        /// thing once, at the moment it becomes true, and cannot be
        /// missed.
        ///
        /// **This does not touch D9.** The workspace's no-panic family
        /// is scoped to INPUT — *"the kernel never panics on any
        /// INPUT — every input-reachable failure is a typed error"*
        /// (the root `Cargo.toml`'s `[workspace.lints.clippy]`) — and a
        /// worker thread dying is reachable from no input at all: no
        /// document, no gesture and no parameter can stop one. It is
        /// the same class that clause hands to `unreachable!`, a bug
        /// the code can observe in a branch, and it takes a `panic!`
        /// rather than `unreachable!` because it IS reachable and
        /// saying otherwise would be false.
        ///
        /// # The message is the whole of what a user sees
        ///
        /// So it names the seam, says whose fault it is, and offers no
        /// recourse: the process is already going down, and advice a
        /// reader cannot act on before the window closes would be
        /// decoration on a crash.
        #[expect(
            clippy::panic,
            reason = "a crashed worker is not input-reachable: D9's family is scoped \
                      to input, and this is the bug-observed-in-a-branch class"
        )]
        fn crashed(&self) -> ! {
            panic!(
                "the {} worker thread crashed. This is a bug in the viewer: no \
                 document and nothing a reader can do stops a worker, and nothing \
                 starts another one, so the seam is dead and every answer it owes \
                 is lost. Stopping here rather than going on with a picture that \
                 silently never changes again.",
                self.worker
            );
        }

        /// The worker ended in the ORDERLY way: [`Coalescing::close`]
        /// took the request channel and the thread's `recv` returned.
        /// Nothing more will ever be answered, so nothing may go on
        /// reporting busy.
        ///
        /// **Only shutdown reaches here now.** This used to answer
        /// both endings in the same three lines, which is what made a
        /// crashed seam read as a quiet one everywhere above it; the
        /// crash goes to [`Coalescing::crashed`] instead. `close` is
        /// called from `Drop` and from nowhere else, so on a running
        /// application there is no path to this function at all — the
        /// rows that reach it close the channel by hand.
        fn forget_worker(&mut self) {
            self.running = false;
            self.waiting = None;
        }

        /// Take a finished answer, if one is ready. Never blocks.
        ///
        /// An answer a waiting job supersedes dies HERE rather than
        /// travelling up to be discarded by key.
        fn poll(&mut self) -> Option<J::Done> {
            loop {
                match self.from_worker.try_recv() {
                    Ok(done) => {
                        self.running = false;
                        match self.waiting.take() {
                            Some(next) if next.supersedes(&done) => self.dispatch(next),
                            _ => return Some(done),
                        }
                    }
                    Err(TryRecvError::Empty) => return None,
                    // The worker is gone. Which way it went is the
                    // same read `dispatch` makes, for the same reason:
                    // a request channel still in hand means the thread
                    // that held the other end died under a job.
                    Err(TryRecvError::Disconnected) => {
                        if self.to_worker.is_some() {
                            self.crashed();
                        }
                        self.forget_worker();
                        return None;
                    }
                }
            }
        }

        /// Whether a job is in flight or waiting.
        fn busy(&self) -> bool {
            self.running || self.waiting.is_some()
        }

        /// Close the request channel so the worker's `recv` returns and
        /// the thread ends after whatever it is running, and drop the
        /// job that will now never be sent. **The worker is not waited
        /// for**: the handle is dropped with `self`, so the thread
        /// finishes and dies on its own.
        fn close(&mut self) {
            self.to_worker = None;
            self.waiting = None;
        }

        /// [`Coalescing::close`], and then WAIT for the worker.
        ///
        /// Only a seam whose job can be stopped may do this, because
        /// nothing else bounds the join.
        fn close_and_join(&mut self) {
            self.close();
            if let Some(handle) = self.handle.take() {
                let _ = handle.join();
            }
        }
    }

    /// A request plus the token that stops it.
    ///
    /// **The token travels with the job, and is minted by the
    /// submitter.** The alternative — one long-lived token the worker
    /// clears between runs — loses a cancel raised while the queue is
    /// draining: the clear would wipe a cancelation aimed at the job
    /// it was clearing for. A per-job token has no such window,
    /// because the only thing a cancel can name is a job that already
    /// exists.
    #[derive(Debug)]
    struct EvalJob {
        request: EvalRequest,
        cancel: CancelToken,
    }

    impl Job for EvalJob {
        type Done = EvalDone;

        /// **Always**, where the two seams below compare keys — and it
        /// is cancel-and-restart that makes it so. A job only ever
        /// waits because a submit put it there, and that same submit
        /// canceled the run `done` answers, so what is in hand is at
        /// best a canceled prefix of a document the caller has already
        /// moved past.
        fn supersedes(&self, _done: &EvalDone) -> bool {
            true
        }
    }

    impl Job for IndexRequest {
        type Done = IndexDone;

        /// **Superseded is decided by KEY, not by position.** A waiting
        /// request for the picture `done` already IS is what a δ moved
        /// away and back produces, and rebuilding it would cost a
        /// second full build of an answer in hand — the one wasted
        /// build this seam accepts, paid twice for nothing.
        ///
        /// The key is a [`crate::pickindex::PictureKey`] and the
        /// comparison is over the whole of it, so this impl and
        /// [`FitRequest`]'s below cannot be read as two spellings of
        /// one rule: they compare different values, and only one of
        /// them is a picture.
        fn supersedes(&self, done: &IndexDone) -> bool {
            self.key != done.key
        }
    }

    impl Job for FitRequest {
        type Done = FitDone;

        /// By key, for [`IndexRequest`]'s reason: a waiting request for
        /// the answer already in hand would cost a second ladder for a
        /// number nobody's view of the world has moved off.
        ///
        /// **The pair is not a [`crate::pickindex::PictureKey`] and
        /// must not become one.** It is spelled identically and sits
        /// one impl from one that is, but `requested` is the δ somebody
        /// ASKED for — this seam's question, not its answer — where a
        /// picture's δ is what an index was built at. A fit for a δ the
        /// ladder will coarsen and a picture at that δ are different
        /// things.
        fn supersedes(&self, done: &FitDone) -> bool {
            (self.generation, self.requested) != (done.generation, done.requested)
        }
    }

    /// A background-thread evaluation seam.
    ///
    /// [`Coalescing`]'s machine with a token added: at most one job is
    /// ever with the worker, a submit while the worker holds one
    /// replaces the waiting job rather than queueing, and
    /// [`EvalService::poll`] drops a result a waiting job has already
    /// superseded — which is the coalescing the module docs promise, in
    /// the same shape [`super::InlineEvaluator`] has it.
    #[derive(Debug)]
    pub struct ThreadEvaluator {
        inner: Coalescing<EvalJob>,
        /// The token of the most recently submitted job — what
        /// `cancel` names, and what the next `submit` cancels.
        cancel: CancelToken,
    }

    impl ThreadEvaluator {
        /// Spawn the worker.
        ///
        /// The memo it primes from is the worker's own state
        /// ([`Coalescing::spawn`]), so nothing above the seam holds the
        /// previous evaluation in order to hand it back.
        ///
        /// # Errors
        ///
        /// [`SpawnError::Thread`] if the OS refuses the thread; the
        /// application would otherwise sit at "evaluating…" forever
        /// with no failure anywhere to read.
        pub fn spawn() -> Result<Self, SpawnError> {
            let inner = Coalescing::spawn(
                Worker::Evaluation,
                "viewer-eval",
                None::<PriorRun>,
                |prior, job: EvalJob| {
                    let evaluation = run_once(&job.request, prior, &job.cancel);
                    EvalDone {
                        generation: job.request.generation,
                        evaluation,
                    }
                },
            )?;
            Ok(Self {
                inner,
                cancel: CancelToken::new(),
            })
        }
    }

    impl EvalService for ThreadEvaluator {
        fn submit(&mut self, request: EvalRequest) {
            // Cancel-and-restart: whatever the worker holds is for an
            // older request, so it is stopped at its next node
            // boundary. The same call also cancels a job that is only
            // WAITING — correct, because that job is superseded too and
            // the token it carries is about to be dropped with it.
            self.cancel.cancel();
            let cancel = CancelToken::new();
            self.cancel = cancel.clone();
            self.inner.submit(EvalJob { request, cancel });
        }

        fn cancel(&mut self) {
            self.cancel.cancel();
        }

        fn poll(&mut self) -> Option<EvalDone> {
            self.inner.poll()
        }

        fn busy(&self) -> bool {
            self.inner.busy()
        }
    }

    impl Drop for ThreadEvaluator {
        fn drop(&mut self) {
            // Cancel, then close the request channel so the worker's
            // `recv` returns, then wait for it: a detached thread
            // holding a `Doc` past the session's life is exactly the
            // shape that makes shutdown nondeterministic. This seam is
            // the one that may wait, because the evaluation checks the
            // token between nodes and the join is therefore bounded by
            // one node.
            self.cancel.cancel();
            self.inner.close_and_join();
        }
    }

    /// A background-thread index seam.
    ///
    /// [`Coalescing`]'s machine with no token at all: at most one build
    /// is ever with the worker, a submit while it holds one replaces
    /// the waiting request rather than queueing, and
    /// [`IndexService::poll`] drops an answer a waiting request has
    /// already superseded by key ([`Job::supersedes`]).
    ///
    /// **Its own thread, not the evaluator's.** Sharing one worker
    /// would put an uninterruptible index build in front of the next
    /// evaluation, and the evaluation seam promises an edit made
    /// during a run cancels that run and starts the new document at
    /// once. It cannot keep that promise from behind a queue, so the
    /// two runs are concurrent: an index for a generation the session
    /// has moved past finishes into a `poll` that discards it.
    #[derive(Debug)]
    pub struct ThreadIndexer {
        inner: Coalescing<IndexRequest>,
    }

    impl ThreadIndexer {
        /// Spawn the worker.
        ///
        /// The memo lives on the worker thread for the thread's life,
        /// which is the seam's: it is what the previous picture left
        /// behind, and nothing above the seam sees it
        /// ([`build_index`]).
        ///
        /// # Errors
        ///
        /// [`SpawnError::Thread`] if the OS refuses the thread; the
        /// picture would otherwise sit behind an indexing indicator
        /// forever with no failure anywhere to read.
        pub fn spawn() -> Result<Self, SpawnError> {
            let inner = Coalescing::spawn(
                Worker::Index,
                "viewer-index",
                PickMemo::new(),
                |memo, request| build_index(&request, memo),
            )?;
            Ok(Self { inner })
        }
    }

    impl IndexService for ThreadIndexer {
        fn submit(&mut self, request: IndexRequest) {
            // Restart WITHOUT cancel: the build the worker holds has no
            // token to stop it, so it runs to completion and `poll`
            // throws its answer away. Latest wins for the one that has
            // not started — that one costs nothing to drop.
            self.inner.submit(request);
        }

        fn poll(&mut self) -> Option<IndexDone> {
            self.inner.poll()
        }

        fn busy(&self) -> bool {
            self.inner.busy()
        }
    }

    impl Drop for ThreadIndexer {
        /// Close the request channel so the worker's `recv` returns
        /// and the thread ends after whatever it is building — and
        /// **do not wait for it** ([`Coalescing::close`]).
        ///
        /// [`ThreadEvaluator`] joins, and can: it cancels first, and
        /// the evaluation checks the token between nodes, so the join
        /// is bounded by one node. Nothing bounds a join here. A
        /// window whose close button did nothing for the thirteen
        /// seconds an index build takes would be paying for shutdown
        /// determinism with the one thing this whole seam exists to
        /// buy, so the worker is left to finish and die on its own.
        /// What it holds while it does is a document copy and a handle
        /// on a run nobody is looking at any more.
        fn drop(&mut self) {
            self.inner.close();
        }
    }

    /// A background-thread fit seam.
    ///
    /// [`ThreadIndexer`]'s shape exactly, over a shorter job: the
    /// ladder of probe tessellations the display budget prices a
    /// document with. [`Coalescing`]'s machine, no token, superseded by
    /// key.
    ///
    /// **Its own thread, not the indexer's.** The two are sequential
    /// for one document — the fit picks the δ the index is built at —
    /// but they are not sequential across documents: a δ typed into
    /// the View pane submits an index build for the picture on screen,
    /// and a fit for a document that arrived meanwhile must not sit
    /// behind it. That is the argument the index worker's own docs
    /// make about the evaluator, one step further along: an
    /// uninterruptible job in front of another job's queue weakens
    /// whatever promise was made above it.
    #[derive(Debug)]
    pub struct ThreadFitter {
        inner: Coalescing<FitRequest>,
    }

    impl ThreadFitter {
        /// Spawn the worker.
        ///
        /// Nothing survives between jobs here, so the worker's state is
        /// `()`. The index worker keeps a memo because a picture's
        /// faces recur; a fit reads a body it is handed and answers a
        /// number, so there is no state a second fit could reuse.
        ///
        /// # Errors
        ///
        /// [`SpawnError::Thread`] if the OS refuses the thread. The
        /// index build waits on this seam's answer, so every document
        /// would otherwise open to a picture that never arrives, with
        /// no failure anywhere to read.
        pub fn spawn() -> Result<Self, SpawnError> {
            let inner = Coalescing::spawn(Worker::Fit, "viewer-fit", (), |_, request| {
                run_fit(&request)
            })?;
            Ok(Self { inner })
        }
    }

    impl FitService for ThreadFitter {
        fn submit(&mut self, request: FitRequest) {
            // Restart WITHOUT cancel: the ladder the worker is on has
            // no token to stop it, so it runs to completion and `poll`
            // throws its answer away.
            self.inner.submit(request);
        }

        fn poll(&mut self) -> Option<FitDone> {
            self.inner.poll()
        }

        fn busy(&self) -> bool {
            self.inner.busy()
        }
    }

    impl Drop for ThreadFitter {
        /// [`ThreadIndexer`]'s `Drop`, for its reason: the job has no
        /// token, so nothing bounds a join and a close button that did
        /// nothing until the ladder finished would be paying for
        /// shutdown determinism with what the seam exists to buy.
        fn drop(&mut self) {
            self.inner.close();
        }
    }

    /// **The crash announcement, on the machine that makes it.**
    ///
    /// # Why these rows are here and not against a public door
    ///
    /// The three shipped handles cannot be crashed from outside, by
    /// construction: a worker only dies by panicking inside its own
    /// job, the job is `build_index`, `run_fit` or `run_once`, and the
    /// closure that calls it is private to this module with no door to
    /// inject a failure through. So a row driven through
    /// [`ThreadIndexer`] would have to make the KERNEL panic on an
    /// input, which is the thing D9 says cannot happen.
    ///
    /// What these drive instead is [`Coalescing`] itself — not a
    /// hand-written mirror of it, but the very type all three handles
    /// delegate every `submit`, `poll` and `busy` to, carrying both of
    /// the arms under test. **What that does not prove** is the last
    /// inch: that a panic inside one of those three private closures is
    /// the only way to arrive, which is argued from the worker loop's
    /// three exits rather than executed.
    ///
    /// Each row's worker prints one `thread '…' panicked` line to
    /// stderr before the row's own panic. That line is the subject, not
    /// a failure.
    #[cfg(test)]
    mod tests {
        // Panicking is a test's failure mechanism (workspace lint
        // note), and here it is also the subject twice over: the worker
        // dies by panicking and the machine answers by panicking.
        #![allow(clippy::expect_used)]
        #![allow(clippy::panic)]

        use super::{Coalescing, Job, Worker};

        /// A job with nothing in it: these rows are about the machine's
        /// bookkeeping, and a payload would be scenery.
        struct Nothing;

        impl Job for Nothing {
            type Done = ();

            /// Never superseded — the arms under test are the ones
            /// where no answer comes at all.
            fn supersedes(&self, (): &()) -> bool {
                false
            }
        }

        /// A seam whose worker dies under the first job it is handed.
        fn dying(worker: Worker) -> Coalescing<Nothing> {
            Coalescing::spawn(worker, "viewer-test-crash", (), |(), Nothing| {
                panic!("the worker dies under the job it was handed")
            })
            .expect("the worker starts")
        }

        /// Wait for the crash to have actually happened.
        ///
        /// Taking the handle is a WAIT and not a construction: it does
        /// not put the seam in the state under test, it only removes
        /// the race from observing it. The state itself — a dead
        /// worker, a request channel still in hand, and a `poll` on the
        /// next frame — is what the application reaches on its own.
        fn await_crash(seam: &mut Coalescing<Nothing>) {
            let handle = seam.handle.take().expect("the worker was spawned");
            assert!(
                handle.join().is_err(),
                "the row needs the worker to have actually panicked"
            );
        }

        /// **The reachable arm**: the frame after a worker died under
        /// its job polls for an answer and finds the channel gone.
        ///
        /// This is the sequence an application really takes — submit on
        /// one frame, poll on a later one — and the panic names the
        /// seam it was.
        #[test]
        #[should_panic(expected = "the index worker thread crashed")]
        fn a_worker_that_crashed_takes_the_process_down_at_the_next_poll() {
            let mut seam = dying(Worker::Index);
            seam.submit(Nothing);
            assert!(seam.busy(), "the job is with the worker");
            await_crash(&mut seam);
            let _ = seam.poll();
        }

        /// **The other arm, which is NOT sequence-reachable**, and the
        /// row says so rather than implying otherwise.
        ///
        /// A `send` that fails with the sender still in hand means the
        /// receiving end went down with its thread, so it is a crash
        /// wherever it comes from — but this machine cannot get here.
        /// `dispatch` runs only when `running` is false, and after a
        /// crash `running` stays true until a `poll` clears it, which
        /// is the row above. The alternative entry, `poll`'s
        /// redispatch of a superseding job, needs a buffered answer AND
        /// a dead worker at once: the loop sends an answer only when
        /// `answer` RETURNED, and a worker that returned is a worker
        /// that went back to `recv` and can only die on a job it was
        /// then handed — for which no answer is ever sent. So the two
        /// cannot hold together.
        ///
        /// The arm stays because the condition means what it means, and
        /// answering it with [`Coalescing::forget_worker`] would put
        /// back the conflation this change removes. This row clears
        /// `running` by hand to execute it, and therefore asserts the
        /// ARM's behaviour and nothing at all about reachability.
        #[test]
        #[should_panic(expected = "the display fit worker thread crashed")]
        fn a_send_that_fails_with_the_channel_still_ours_is_a_crash_too() {
            let mut seam = dying(Worker::Fit);
            seam.submit(Nothing);
            await_crash(&mut seam);
            seam.running = false;
            seam.submit(Nothing);
        }

        /// **Shutdown is not a crash**, which is the whole point of
        /// discriminating: `close` takes the request channel, and a
        /// machine that has been closed forgets its work quietly.
        #[test]
        fn a_closed_channel_is_forgotten_rather_than_announced() {
            let mut seam = Coalescing::<Nothing>::spawn(
                Worker::Evaluation,
                "viewer-test-close",
                (),
                |(), Nothing| {},
            )
            .expect("the worker starts");
            seam.close();
            seam.submit(Nothing);
            assert!(
                !seam.busy(),
                "an orderly shutdown drops the job that will never be sent"
            );
        }
    }
}
