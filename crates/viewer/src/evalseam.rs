//! The two seams the picture is built across — where a document
//! becomes a result DAG, and where that DAG becomes the pick index the
//! viewport draws and picks against. The one place in this crate that
//! may know about threads.
//!
//! # Why a seam at all
//!
//! Evaluation is the only unbounded computation the interaction layer
//! provokes, and the layer above it must never assume how it runs. The
//! GUI plan carries that as a standing constraint for the web lane:
//! natively a background thread, on wasm a Worker or an inline slice,
//! with no source change above this boundary. So the vocabulary here
//! is submit / poll / cancel over a [`Generation`], and
//! [`InlineEvaluator`] — which runs the whole evaluation inside
//! `poll` — satisfies it exactly as well as [`ThreadEvaluator`] does.
//! Every test in this crate drives the inline one; the application
//! drives the threaded one; nothing else changes.
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
//! **Both implementations do this, by the same mechanism**: at most one
//! request is ever outstanding, and a submit while one is outstanding
//! REPLACES the waiting request rather than adding to it. For
//! [`InlineEvaluator`] that is a single `Option`; for
//! [`ThreadEvaluator`] it is a single `Option` in the handle plus a
//! `running` flag, so the channel to the worker never holds more than
//! one job. Making the worker drain a queue would have produced the
//! same observable answer, but it would have left the handle's own
//! accounting (what `busy` reports) describing a queue the caller
//! cannot see; keeping the queue in the handle is why the two
//! implementations are the same shape rather than merely the same
//! outcome. The row that pins it drives BOTH.
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
//! landed evaluation plus a δ becomes a [`crate::pick::PickIndex`],
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

use crate::pick::{PickIndex, PickIndexError};
use crate::scene::DisplayTolerance;

/// A request's identity: the seam's own monotone counter, minted by
/// the session on every submit.
///
/// Distinct from the shipped evaluation `Epoch`, which identifies the
/// RUN. This identifies the REQUEST, and the session mints a fresh one
/// for every submit — including a re-submit of an unchanged document
/// (`SessionOp::Reevaluate`). That is deliberately stricter than
/// "identifies the document version": a result may land only against
/// the request that asked for it, so a run canceled and then re-asked
/// can never have its abandoned answer accepted for the new ask.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Generation(u64);

impl Generation {
    /// The first generation.
    pub const FIRST: Self = Self(0);

    /// The next generation after this one.
    ///
    /// Saturating, not wrapping. A wrap would make a stale result
    /// compare equal to the current request — the one thing this type
    /// exists to prevent — and it is unreachable at `u64` anyway, so
    /// the arithmetic that cannot produce the failure is the one to
    /// write. At the ceiling every request shares a generation and the
    /// staleness filter degrades to accepting everything, which is the
    /// pre-existing behaviour of a counter that never advances; no run
    /// of this application gets within astronomical distance of it.
    pub fn next(self) -> Self {
        Self(self.0.saturating_add(1))
    }

    /// The raw counter, for a caller displaying it.
    pub fn get(self) -> u64 {
        self.0
    }
}

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
        // Cancel-and-restart, degenerately: nothing has started, so
        // the newer request simply replaces the older one. The token
        // is fresh because the run this cancel would have stopped is
        // the one being replaced.
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
/// Everything [`crate::pick::PickIndex::build`] reads, as owned values,
/// so the worker holds its own copy of the document and a handle on the
/// evaluation while the interaction layer keeps editing.
#[derive(Clone, Debug)]
pub struct IndexRequest {
    /// The generation of the evaluation this index describes.
    pub generation: Generation,
    /// The chordal tolerance the roots are tessellated at — half the
    /// key, and the half the evaluation knows nothing about.
    pub delta: DisplayTolerance,
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
    /// The generation the request carried.
    pub generation: Generation,
    /// The δ the request carried.
    pub delta: DisplayTolerance,
    /// The index, or the refusal that stopped it — a failed or
    /// poisoned root is an ordinary editing state and its refusal is
    /// the answer, not an absence.
    pub index: Result<PickIndex, PickIndexError>,
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

/// Run one index build, stamping the answer with the request's own key.
///
/// The seam's one call into [`crate::pick::PickIndex::build`], shared
/// by both implementations, so the generation the index is built under
/// and the generation the answer is filed under are read from one
/// place and cannot disagree.
fn build_index(request: &IndexRequest) -> IndexDone {
    IndexDone {
        generation: request.generation,
        delta: request.delta,
        index: PickIndex::build(
            &request.doc,
            &request.evaluation,
            request.generation,
            request.delta,
            request.tol,
        ),
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
}

impl InlineIndexer {
    /// A seam that has built nothing.
    pub fn new() -> Self {
        Self::default()
    }
}

impl IndexService for InlineIndexer {
    fn submit(&mut self, request: IndexRequest) {
        // Restart, degenerately: nothing has started, so the newer
        // request replaces the older one and the older one costs
        // nothing at all.
        self.pending = Some(request);
    }

    fn poll(&mut self) -> Option<IndexDone> {
        let request = self.pending.take()?;
        Some(build_index(&request))
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
};

#[cfg(not(target_family = "wasm"))]
pub use threaded::ThreadIndexer;

#[cfg(not(target_family = "wasm"))]
pub use threaded::{SpawnError, ThreadEvaluator};

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
        EvalDone, EvalRequest, EvalService, IndexDone, IndexRequest, IndexService, PriorRun,
        build_index, run_once,
    };

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
    struct Job {
        request: EvalRequest,
        cancel: CancelToken,
    }

    /// Why a worker could not be started.
    ///
    /// The worker is NAMED, because this crate spawns one per seam and
    /// a startup refusal that did not say which would send its reader
    /// to the wrong half of this module.
    #[derive(Debug)]
    pub enum SpawnError {
        /// The OS refused the thread.
        Thread {
            /// Which worker: `"evaluation"` or `"index"`.
            worker: &'static str,
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

    /// A background-thread evaluation seam.
    ///
    /// **At most one job is ever with the worker.** A submit while the
    /// worker holds one replaces [`ThreadEvaluator::waiting`] rather
    /// than queueing, and [`EvalService::poll`] drops a result that a
    /// waiting job has already superseded — which is the coalescing the
    /// module docs promise, in the same shape [`super::InlineEvaluator`]
    /// has it.
    #[derive(Debug)]
    pub struct ThreadEvaluator {
        to_worker: Option<Sender<Job>>,
        from_worker: Receiver<EvalDone>,
        /// The token of the most recently submitted job — what
        /// `cancel` names, and what the next `submit` cancels.
        cancel: CancelToken,
        /// The job the worker is evaluating, if any. A flag rather
        /// than a count, because the channel never holds more than one.
        running: bool,
        /// The newest request, held back until the worker is free.
        /// Replaced, never appended to: that is latest-wins.
        waiting: Option<Job>,
        worker: Option<JoinHandle<()>>,
    }

    impl ThreadEvaluator {
        /// Spawn the worker.
        ///
        /// # Errors
        ///
        /// [`SpawnError::Thread`] if the OS refuses the thread. Loud
        /// rather than degraded on purpose: a seam whose worker never
        /// started accepts every submit and answers none, so the
        /// application would sit at "evaluating…" forever with no
        /// failure anywhere to read.
        pub fn spawn() -> Result<Self, SpawnError> {
            let (to_worker, requests) = channel::<Job>();
            let (results, from_worker) = channel::<EvalDone>();
            let worker = std::thread::Builder::new()
                .name("viewer-eval".to_owned())
                .spawn(move || work(&requests, &results))
                .map_err(|error| SpawnError::Thread {
                    worker: "evaluation",
                    error,
                })?;
            Ok(Self {
                to_worker: Some(to_worker),
                from_worker,
                cancel: CancelToken::new(),
                running: false,
                waiting: None,
                worker: Some(worker),
            })
        }

        /// Hand `job` to the worker, or record that the worker is gone.
        fn dispatch(&mut self, job: Job) {
            match self.to_worker.as_ref() {
                Some(to_worker) if to_worker.send(job).is_ok() => self.running = true,
                // The worker ended (only reachable after `Drop` has
                // closed the channel, or if it panicked). Nothing more
                // will ever be answered, and the indicator must not
                // stay lit for an answer that is not coming.
                _ => {
                    self.running = false;
                    self.waiting = None;
                }
            }
        }
    }

    /// The worker loop: evaluate each job, answer with its generation,
    /// keep the memo.
    ///
    /// The memo lives HERE, on the thread that owns it, so nothing
    /// above the seam holds the previous evaluation in order to hand
    /// it back.
    fn work(requests: &Receiver<Job>, results: &Sender<EvalDone>) {
        let mut prior: Option<PriorRun> = None;
        while let Ok(job) = requests.recv() {
            let evaluation = run_once(&job.request, &mut prior, &job.cancel);
            if results
                .send(EvalDone {
                    generation: job.request.generation,
                    evaluation,
                })
                .is_err()
            {
                return;
            }
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
            let job = Job { request, cancel };
            if self.running {
                // Latest wins: the previous waiting job is dropped, not
                // queued behind this one.
                self.waiting = Some(job);
            } else {
                self.dispatch(job);
            }
        }

        fn cancel(&mut self) {
            self.cancel.cancel();
        }

        fn poll(&mut self) -> Option<EvalDone> {
            loop {
                match self.from_worker.try_recv() {
                    Ok(done) => {
                        self.running = false;
                        match self.waiting.take() {
                            // `done` answers a request the caller has
                            // already superseded. Coalescing means it
                            // dies HERE rather than travelling up to be
                            // discarded by generation.
                            Some(next) => self.dispatch(next),
                            None => return Some(done),
                        }
                    }
                    Err(TryRecvError::Empty) => return None,
                    // The worker is gone. Nothing further will ever
                    // land, so the indicator must not stay lit forever.
                    Err(TryRecvError::Disconnected) => {
                        self.running = false;
                        self.waiting = None;
                        return None;
                    }
                }
            }
        }

        fn busy(&self) -> bool {
            self.running || self.waiting.is_some()
        }
    }

    impl Drop for ThreadEvaluator {
        fn drop(&mut self) {
            // Close the request channel so the worker's `recv` returns
            // and the thread ends, then wait for it: a detached thread
            // holding a `Doc` past the session's life is exactly the
            // shape that makes shutdown nondeterministic.
            self.cancel.cancel();
            self.to_worker = None;
            if let Some(worker) = self.worker.take() {
                let _ = worker.join();
            }
        }
    }
    /// A background-thread index seam.
    ///
    /// [`ThreadEvaluator`]'s shape with the token taken out: at most
    /// one build is ever with the worker, a submit while it holds one
    /// replaces [`ThreadIndexer::waiting`] rather than queueing, and
    /// [`IndexService::poll`] drops an answer a waiting request has
    /// already superseded.
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
        to_worker: Option<Sender<IndexRequest>>,
        from_worker: Receiver<IndexDone>,
        /// Whether the worker holds a request. A flag rather than a
        /// count, because the channel never holds more than one.
        running: bool,
        /// The newest request, held back until the worker is free.
        /// Replaced, never appended to: that is latest-wins.
        waiting: Option<IndexRequest>,
    }

    impl ThreadIndexer {
        /// Spawn the worker.
        ///
        /// # Errors
        ///
        /// [`SpawnError::Thread`] if the OS refuses the thread. Loud
        /// rather than degraded, for [`ThreadEvaluator::spawn`]'s
        /// reason: a seam whose worker never started accepts every
        /// submit and answers none, so the picture would sit behind an
        /// indexing indicator forever with no failure anywhere to read.
        pub fn spawn() -> Result<Self, SpawnError> {
            let (to_worker, requests) = channel::<IndexRequest>();
            let (results, from_worker) = channel::<IndexDone>();
            std::thread::Builder::new()
                .name("viewer-index".to_owned())
                .spawn(move || index_work(&requests, &results))
                .map_err(|error| SpawnError::Thread {
                    worker: "index",
                    error,
                })?;
            Ok(Self {
                to_worker: Some(to_worker),
                from_worker,
                running: false,
                waiting: None,
            })
        }

        /// Hand `request` to the worker, or record that the worker is
        /// gone.
        fn dispatch(&mut self, request: IndexRequest) {
            match self.to_worker.as_ref() {
                Some(to_worker) if to_worker.send(request).is_ok() => self.running = true,
                // The worker ended (only reachable after `Drop` has
                // closed the channel, or if it panicked). Nothing more
                // will ever be answered, and the indicator must not
                // stay lit for an answer that is not coming.
                _ => {
                    self.running = false;
                    self.waiting = None;
                }
            }
        }
    }

    /// The worker loop: build each index and answer with its key.
    ///
    /// No memo and nothing kept between runs: an index is discarded
    /// whole and rebuilt whole (`crate::pick`'s staleness rule), so
    /// there is nothing here for a later build to prime from.
    fn index_work(requests: &Receiver<IndexRequest>, results: &Sender<IndexDone>) {
        while let Ok(request) = requests.recv() {
            if results.send(build_index(&request)).is_err() {
                return;
            }
        }
    }

    impl IndexService for ThreadIndexer {
        fn submit(&mut self, request: IndexRequest) {
            if self.running {
                // Restart WITHOUT cancel: the build the worker holds
                // has no token to stop it, so it runs to completion
                // and `poll` throws its answer away. Latest wins for
                // the one that has not started — that one costs
                // nothing to drop.
                self.waiting = Some(request);
            } else {
                self.dispatch(request);
            }
        }

        fn poll(&mut self) -> Option<IndexDone> {
            loop {
                match self.from_worker.try_recv() {
                    Ok(done) => {
                        self.running = false;
                        match self.waiting.take() {
                            // **Superseded is decided by KEY, not by
                            // position.** A waiting request for the
                            // picture `done` already IS is what a δ
                            // moved away and back produces, and
                            // rebuilding it would cost a second full
                            // build of an answer in hand — the one
                            // wasted build this seam accepts, paid
                            // twice for nothing.
                            Some(next)
                                if (next.generation, next.delta)
                                    != (done.generation, done.delta) =>
                            {
                                // Genuinely superseded: it dies HERE
                                // rather than travelling up to be
                                // discarded by key.
                                self.dispatch(next);
                            }
                            _ => return Some(done),
                        }
                    }
                    Err(TryRecvError::Empty) => return None,
                    // The worker is gone. Nothing further will ever
                    // land, so the indicator must not stay lit forever.
                    Err(TryRecvError::Disconnected) => {
                        self.running = false;
                        self.waiting = None;
                        return None;
                    }
                }
            }
        }

        fn busy(&self) -> bool {
            self.running || self.waiting.is_some()
        }
    }

    impl Drop for ThreadIndexer {
        /// Close the request channel so the worker's `recv` returns
        /// and the thread ends after whatever it is building — and
        /// **do not wait for it**.
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
            self.to_worker = None;
            self.waiting = None;
        }
    }
}
