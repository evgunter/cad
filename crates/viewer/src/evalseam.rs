//! The evaluation seam: where a document becomes a result DAG, and
//! the one place in this crate that may know about threads.
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
//! submits while busy COALESCE: only the newest document is queued, so
//! a slider drag that outruns the evaluator produces at most one
//! evaluation per completed run rather than a backlog of them.
//!
//! The cancelation is the shipped `CancelToken` and nothing else: it
//! is checked between nodes, so a canceled run returns its completed
//! prefix typed as `EvalOutcome::Canceled`. This crate discards that
//! prefix (it is not the document the user is on) but never mistakes
//! it for a completed one — the memo is only ever primed from a run
//! that completed.
//!
//! # Staleness is decided here, by generation
//!
//! Results carry the generation of the request that produced them.
//! [`EvalService::poll`] can hand back a result for a document two
//! edits old; the session that owns the seam compares generations and
//! drops it. That rule is a pure function of two integers, which is
//! why it is testable without a thread in sight.

use std::sync::Arc;

use pncad::document::{
    CancelToken, Doc, EvalOptions, EvalOutcome, Evaluation, ProfileProgram, evaluate,
};
use pncad::geom_core::Tol;

/// A request's identity: the seam's own monotone counter, minted by
/// the session on every submit.
///
/// Distinct from the shipped evaluation `Epoch`, which identifies the
/// RUN. This identifies the DOCUMENT VERSION the run was asked for,
/// which is what staleness is about: two runs of the same document
/// (a cancel and its restart) share a generation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Generation(u64);

impl Generation {
    /// The first generation.
    pub const FIRST: Self = Self(0);

    /// The next generation after this one.
    pub fn next(self) -> Self {
        Self(self.0.wrapping_add(1))
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

/// Run the evaluation, priming the memo from `prior` and updating it
/// when the run completes.
///
/// The one place `evaluate` is called in this crate, shared by both
/// implementations so the memo discipline — prime from the previous
/// COMPLETED run only — has a single home.
fn run_once(
    request: &EvalRequest,
    prior: &mut Option<Arc<Evaluation<f64>>>,
    cancel: &CancelToken,
) -> Arc<Evaluation<f64>> {
    let evaluation = Arc::new(evaluate::<f64>(
        &request.doc,
        prior.as_deref(),
        cancel,
        &EvalOptions::default(),
        request.tol,
    ));
    if evaluation.outcome == EvalOutcome::Completed {
        *prior = Some(Arc::clone(&evaluation));
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
    prior: Option<Arc<Evaluation<f64>>>,
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

#[cfg(not(target_family = "wasm"))]
pub use threaded::ThreadEvaluator;

/// The native seam: one worker thread, a request channel, a result
/// channel.
///
/// Behind `cfg(not(target_family = "wasm"))` because `thread::spawn`
/// is what it is built on and the browser has no such thing — the
/// module is absent there and [`InlineEvaluator`] is what the wasm
/// build uses until a Worker-backed sibling lands beside this one.
#[cfg(not(target_family = "wasm"))]
mod threaded {
    use std::sync::Arc;
    use std::sync::mpsc::{Receiver, Sender, TryRecvError, channel};
    use std::thread::JoinHandle;

    use pncad::document::{CancelToken, Evaluation};

    use super::{EvalDone, EvalRequest, EvalService, run_once};

    /// A request plus the token that stops it.
    ///
    /// **The token travels with the job, and is minted by the
    /// submitter.** The alternative — one long-lived token the worker
    /// clears between runs — loses a cancel raised while the queue is
    /// draining: the clear would wipe a cancelation aimed at the job
    /// it was clearing for. A per-job token has no such window,
    /// because the only thing a cancel can name is a job that already
    /// exists.
    struct Job {
        request: EvalRequest,
        cancel: CancelToken,
    }

    /// A background-thread evaluation seam.
    #[derive(Debug)]
    pub struct ThreadEvaluator {
        to_worker: Option<Sender<Job>>,
        from_worker: Receiver<EvalDone>,
        /// The token of the most recently submitted job — what
        /// `cancel` names, and what the next `submit` cancels.
        cancel: CancelToken,
        /// Requests submitted and not yet answered. A count rather
        /// than a flag: the busy indicator must go dark when the LAST
        /// answer lands, not when the first does.
        in_flight: usize,
        worker: Option<JoinHandle<()>>,
    }

    impl ThreadEvaluator {
        /// Spawn the worker.
        pub fn spawn() -> Self {
            let (to_worker, requests) = channel::<Job>();
            let (results, from_worker) = channel::<EvalDone>();
            let worker = std::thread::Builder::new()
                .name("viewer-eval".to_owned())
                .spawn(move || work(&requests, &results))
                .ok();
            Self {
                to_worker: Some(to_worker),
                from_worker,
                cancel: CancelToken::new(),
                in_flight: 0,
                worker,
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
        let mut prior: Option<Arc<Evaluation<f64>>> = None;
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
            // Cancel-and-restart: the in-flight run is for an older
            // document, so it is stopped at its next node boundary and
            // the worker picks this job up immediately after.
            self.cancel.cancel();
            let cancel = CancelToken::new();
            self.cancel = cancel.clone();
            if let Some(to_worker) = self.to_worker.as_ref()
                && to_worker.send(Job { request, cancel }).is_ok()
            {
                self.in_flight += 1;
            }
        }

        fn cancel(&mut self) {
            self.cancel.cancel();
        }

        fn poll(&mut self) -> Option<EvalDone> {
            match self.from_worker.try_recv() {
                Ok(done) => {
                    self.in_flight = self.in_flight.saturating_sub(1);
                    Some(done)
                }
                Err(TryRecvError::Empty) => None,
                // The worker is gone. Nothing further will ever land,
                // so the indicator must not stay lit forever.
                Err(TryRecvError::Disconnected) => {
                    self.in_flight = 0;
                    None
                }
            }
        }

        fn busy(&self) -> bool {
            self.in_flight > 0
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
}
