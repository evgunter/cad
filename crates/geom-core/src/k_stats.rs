//! Margin-statistics collection for the K-value experiments — a
//! recording wrapper, not telemetry infrastructure (Q1's "ambiguity
//! constant K" residue). One recorder serves every crate's decisions;
//! there is no second one.
//!
//! Every decision the kernel makes goes through one funnel, [`decide`]:
//! it notes the predicate's static name in a thread-local, classifies
//! the margin through the one sanctioned door ([`Decide::sign_within`]),
//! and attaches the name to any indeterminate outcome. Each deciding
//! crate re-exports or wraps this funnel as its single greppable
//! `sign_within` call site (the `geom-brep` funnel pattern), so a
//! [`Probe`]-lane run tags every sample with its real predicate name —
//! `<unnamed>` is unreachable from shipped decide paths.
//!
//! All three doors — [`decide`], [`decide_flagged`], [`decide_invariant`]
//! — delegate to one private `classify`. The name write and the verdict
//! push therefore happen in exactly one place, so no door can carry one
//! channel and miss the other, and the three doors classify identically
//! because they are the same code and not because three bodies are kept
//! level.
//!
//! Cost on the production path: one thread-local `Cell` write per
//! decision, plus — **inside an open [`Bracket`]** — one `Vec` push per
//! outcome, definite or not (see `decide`'s own contract below). That
//! caveat is not hypothetical: `editor_core`'s evaluator brackets
//! **every node evaluation** (NAMING-DESIGN N5, so the verdict-diff
//! engine can attribute flips), and retains the result on the node. So
//! production *does* record, on the one path that asks to.
//!
//! **The verdict log is a bracket with a stack.** [`Bracket::open`]
//! pushes an empty frame onto this thread's frame stack; every
//! decision the funnel classifies while that frame is the innermost
//! open one lands in it — a definite sign as a [`Verdict`], an
//! indeterminate outcome as an [`Escalation`] carrying the
//! [`Indeterminate`] the predicate produced — and [`Bracket::finish`]
//! pops the frame and hands both channels back as a [`Recorded`]. The
//! shape is what makes the two hard cases correct by construction
//! rather than by comment:
//!
//! - **Nesting.** An evaluation that evaluates another document inside
//!   one of its own ops (an instantiated part) opens an inner bracket
//!   per inner node, on top of the outer node's frame. Each inner node
//!   pops its own frame into its own value; the outer frame is never
//!   overwritten and receives exactly the outer op's own decisions. The
//!   same holds for a rayon worker that steals another task while
//!   waiting on a join: the stolen task runs to completion inside the
//!   join, so its frames sit strictly above the waiting task's and are
//!   gone before it resumes. A frame is popped by the bracket that
//!   pushed it, in the reverse of the order they opened; a bracket
//!   closed out of that order is a bug, asserted in debug builds, and
//!   in release it discards the frames above its own rather than ever
//!   handing one bracket another's decisions.
//! - **Thread confinement.** A [`Bracket`] is `!Send` — it carries a
//!   `PhantomData<*const ()>` — so the value that closes a frame cannot
//!   leave the thread whose stack holds it; the compiler refuses the
//!   move. The stack itself is thread-local, and idiom-1 parallelism
//!   (whole nodes on one worker each) opens each node's bracket on the
//!   worker that runs the op.
//! - **Every path closes the frame.** [`Bracket`] pops in `Drop`, so a
//!   bracket that leaves scope without `finish` — an early `return`, a
//!   `?`, a panic unwinding through the op — still pops its frame, and
//!   the thread's stack is empty again once the guard is gone. An
//!   unbracketed state is unrepresentable: there is no call that
//!   installs a log without producing the guard that removes it.
//!
//! **Why a bracket and not a returned value.** The funnel is reached
//! from every deciding crate — 530 call sites in 82 files across seven
//! crates, in 261 distinct functions of which 104 are public — from
//! ops that carry no collector parameter. Returning verdicts as a value
//! means threading a sink through every one of those signatures and
//! every signature between an op's door and its predicates, and the
//! `Decide` trait itself; the measurement and the decline are recorded
//! in the PR that ratified this shape. The stack and the `!Send` guard
//! buy the same two guarantees a returned value would — a nested
//! evaluation cannot clobber its parent, and a frame cannot be read
//! from another thread — as types, at the cost of one thread-local.
//!
//! **The frame stack is not part of the `probe` lane and must not be
//! gated on it.** The K-telemetry sink is `SINK`, which *is*
//! feature-gated; the frame stack merely shares this funnel because
//! this is where decisions pass. Its consumer is production editor-core
//! code — `resolve::vdiff` reads `NodeValue::verdicts` to compare
//! per-predicate sign populations and emit `NodeVerdictDelta`'s flips
//! and divergences, and `drive::classify` reads a node's escalations
//! to tell a terminal sliver from a refinable indeterminacy without
//! matching on the op's error enum. Putting it behind `probe` would not
//! reduce recording; it would hand both consumers empty logs in every
//! default build and silently stop them attributing anything.
//!
//! Paths that never open a bracket (STL export, step-import, the demos)
//! still pay the `RefCell` borrow and an empty-stack check per
//! decision. Gating *that* on a `Cell<bool>` is a live optimization and
//! is orthogonal to any feature — it behaves identically in every build
//! configuration.
//!
//! Recording happens through the [`Probe`] scalar: a transparent `f64`
//! wrapper whose `Decide` implementation logs `(predicate, margin,
//! band, outcome)` to a thread-local sink before delegating. Running
//! validation at `T = Probe` therefore yields the complete per-predicate
//! margin distribution of the run — the data `docs/K-REPORT.md` pulls —
//! with **zero** instrumentation in the validation code itself and
//! bit-identical decisions to `f64` (delegation is exact). The sink is
//! thread-local and explicitly installed ([`start_recording`] /
//! [`take_samples`]), so tests never race and production never records.
//!
//! **The recorder is pinned to this crate from both directions — and it
//! is the IMPL that is pinned, not the type.** `Probe` records by
//! implementing [`Decide`], and that impl can live nowhere else.
//! *Below* geom-core: `Decide`'s supertrait
//! [`SpanLocate`](crate::spline::SpanLocate) is sealed by a
//! `pub(crate)` module whose impl list is the kernel's scalar set, so a
//! downstream type cannot decide. *Above* it: naming `Decide` at all
//! means depending on geom-core, and geom-core would have to depend
//! back — a dependency cycle, which cargo refuses outright.
//!
//! **The `Probe` type is NOT pinned**, and conflating the two is easy
//! enough to be worth a sentence. Nothing stops the newtype being
//! defined in a crate above this one and re-exported here; its five
//! `core::ops` impls are the only things that would travel with it, and
//! every kernel-trait impl — `Real`, `Bounds`, `CertifiedEnclosure`,
//! `SpanLocate`, `Decide` — stays, because a local trait on a foreign
//! type is legal and the reverse is not. That move relocates a newtype
//! and leaves the recorder exactly where it was.
//!
//! What keeps the scalar out of shipped builds is the `probe` feature,
//! not the crate boundary; geom-core's manifest carries the
//! monomorphization measurement that gate was cut on.

use core::cell::{Cell, RefCell};
use core::marker::PhantomData;
use core::mem::ManuallyDrop;

use crate::predicate::{Band, Decide, Indeterminate, Margin, Sign};
// Only `Probe`'s impls name these.
#[cfg(feature = "probe")]
use crate::real::{Bounds, Real};

thread_local! {
    /// The name of the predicate currently being decided (set by the
    /// funnel just before classification; read by [`Probe`]).
    ///
    /// **Deliberately NOT behind the `probe` feature**, even though only
    /// `Probe` reads it. `decide` writes it unconditionally, and the
    /// whole point of the gate is that the funnel's code path is
    /// byte-identical with the feature on and off — a `cfg` here would
    /// make the production decision path differ between build
    /// configurations, which D9 does not permit. The cost is the one
    /// `Cell` write this module has always documented.
    static CURRENT: Cell<&'static str> = const { Cell::new("<unnamed>") };
    /// The installed sample sink, if any.
    #[cfg(feature = "probe")]
    static SINK: RefCell<Option<Vec<MarginSample>>> = const { RefCell::new(None) };
    /// The frame stack: one [`Recorded`] per open [`Bracket`] on this
    /// thread, innermost last (module docs; NAMING-DESIGN N5:
    /// evaluations record their verdict vectors so the verdict-diff
    /// engine can attribute flips). Empty whenever no bracket is open.
    static FRAMES: RefCell<Vec<Recorded>> = const { RefCell::new(Vec::new()) };
}

/// Classifies `margin` against `band`, noting `name` for the recorder
/// and attaching it to any indeterminate outcome — the shared body of
/// every public door below.
///
/// This is the only place either recording channel is written, which
/// is why the doors delegate rather than each carrying a copy: the
/// `CURRENT` write is the recorder's name channel (read by `Probe`),
/// and the frame push is the evaluation-artifact channel (the frame is
/// opened and closed by a [`Bracket`], and read by the verdict-diff
/// engine and the subdivision driver). A door cannot acquire one and
/// miss the other, and an outcome cannot reach one channel of the
/// frame and miss the other: a definite sign is a [`Verdict`], an
/// indeterminate one an [`Escalation`], in one decision order.
fn classify<T: Decide>(name: &'static str, margin: T, band: Band) -> Result<Sign, Indeterminate> {
    CURRENT.with(|c| c.set(name));
    let outcome = margin.sign_within(band).map_err(|e| e.with_predicate(name));
    FRAMES.with(|f| {
        if let Some(top) = f.borrow_mut().last_mut() {
            match outcome {
                Ok(sign) => top.verdicts.push(Verdict {
                    predicate: name,
                    sign,
                }),
                Err(source) => top.escalations.push(Escalation {
                    predicate: name,
                    source,
                }),
            }
        }
    });
    outcome
}

/// The one classification funnel of the kernel: notes `name` for the
/// recorder, classifies `margin` against `band`, and names any
/// indeterminate outcome. Every deciding crate routes its predicates
/// through this function (directly or via a thin crate-local wrapper).
/// The shipped `sign_within` call outside [`Probe`] is the private
/// `classify` these doors delegate to — **exactly one**, which is what
/// makes the greppability claim true rather than approximately true;
/// each door carrying its own copy is what made it false before.
///
/// The margin is a [`Margin<T>`] **by signature** (D4's margin
/// dimensional convention, clause (i)): the caller states its
/// dimensional argument by choosing a construction door at the site,
/// and every recorded margin is therefore a length in the kernel's
/// internal metres. The newtype is `#[repr(transparent)]` and the doors
/// perform exactly the operation they name, so classification and the
/// recorded stream are bit-identical to the pre-convention bare-`T`
/// seam.
///
/// Cost on the production path: exactly one thread-local `Cell` write
/// per decision (plus, inside an open [`Bracket`], one `Vec` push per
/// outcome); the decision itself is `sign_within` verbatim, so
/// outcomes are bit-identical to an unfunneled classification.
///
/// # Errors
///
/// The [`Indeterminate`] from [`Decide::sign_within`], with `name`
/// attached.
pub fn decide<T: Decide>(
    name: &'static str,
    margin: Margin<T>,
    band: Band,
) -> Result<Sign, Indeterminate> {
    classify(name, margin.value(), band)
}

/// The classify seam's **finding lane** — [`decide`] for a shipped
/// comparand whose ledger row (`docs/predicate-dimension-audit.md`)
/// documents that **no construction door honestly fits**: the margin is
/// not (yet) a length, and wrapping it in a [`Margin`] door would
/// launder the very defect the ledger records. This function does NOT
/// construct a `Margin` — the margin stays bare `T` and never claims
/// the dimension it lacks.
///
/// Classification and recording are otherwise [`decide`]'s, so the K
/// stream is unchanged; the only difference reaching `classify` is that
/// [`decide`] unwraps a `Margin` and this door has none to unwrap.
///
/// `ledger_row` names the audit row that argues the absence (e.g.
/// `"F2"`, `"F13"`). It is an obligation, not telemetry: the value
/// reaches no recorder and no column, and `classify` never sees it.
/// What reads it is `geom-core/tests/flagged_census.rs`, which scans
/// the shipped trees. This lane is the greppable inventory of
/// clause-(i) debt, shrinking as the flagged families get their own
/// units, never a convenience door.
///
/// **Standing rule (the debt lane is tracked as issue #214): no new
/// `decide_flagged` site ships without a ledger row in
/// `docs/predicate-dimension-audit.md`.** Two assertions in
/// `flagged_census.rs` carry it over `crates/*/src`. Fixtures
/// and demos are outside the scan and cite a prose reason rather than a
/// row.
///
/// # Errors
///
/// As [`decide`].
pub fn decide_flagged<T: Decide>(
    name: &'static str,
    margin: T,
    band: Band,
    ledger_row: &'static str,
) -> Result<Sign, Indeterminate> {
    // Nothing computes with the row at runtime, by design: it is read
    // from the source text by `geom-core/tests/flagged_census.rs`, which
    // is where a citation can be checked against the document it cites.
    let _ = ledger_row;
    classify(name, margin, band)
}

/// The classify seam's **invariant lane** — [`decide`] for the
/// kernel's **consistency backstops**: inequalities between integral
/// RESULTS (the `volume_backstop` family — wrong-component detectors),
/// which are **outside the length seam by design — not a door, not
/// debt** (Ev's #213 layering ruling). A consistency backstop is
/// never an accuracy gate: pointwise-ε accuracy is owned upstream, and
/// a body whose geometry is ε-right everywhere is never refused for
/// its integral differing at tiny-wiggle scale. Mean displacement is
/// only the honest UNIT of the check's near-zero (indeterminate) zone,
/// so the margin stays **bare `T`** — no [`Margin`] is minted, keeping
/// the lane visibly distinct from every geometric decision.
///
/// Classification and recording are [`decide`]'s: same recorder, same
/// names, same values, the K stream unchanged.
///
/// A certified violation on this lane is a **kernel invariant** failure:
/// callers surface it as their Corrupt-class typed error ("this is a
/// bug", with a report affordance), never as a validity refusal and
/// never as a panic (the `clippy::panic` denial; the Corrupt
/// precedent).
///
/// # Errors
///
/// As [`decide`].
pub fn decide_invariant<T: Decide>(
    name: &'static str,
    margin: T,
    band: Band,
) -> Result<Sign, Indeterminate> {
    classify(name, margin, band)
}

/// One recorded predicate decision: the funnel's static name and the
/// definite sign it classified to. Scalar-independent by construction
/// (the N4 invariant's currency: same verdicts ⇒ same names at f64 AND
/// Interval); float-free, so verdict vectors compare exactly.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Verdict {
    /// The predicate that decided (the funnel's static name).
    pub predicate: &'static str,
    /// The definite sign it returned.
    pub sign: Sign,
}

/// One recorded indeterminate outcome: the funnel's static name and
/// the [`Indeterminate`] the predicate produced, in the frame beside
/// the verdicts and in the same decision order. What a consumer reads
/// to answer "did any predicate here escalate, and on what margin"
/// without matching on whichever op error enum carried the escalation
/// out of the op.
///
/// `predicate` duplicates `source.predicate` deliberately: the funnel
/// attached the name it was called with, and this field states it as
/// a plain `&'static str` beside [`Verdict::predicate`], so the two
/// channels are keyed the same way.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Escalation {
    /// The predicate that escalated (the funnel's static name).
    pub predicate: &'static str,
    /// The indeterminate outcome, with the name attached.
    pub source: Indeterminate,
}

/// Everything one [`Bracket`] recorded: both channels of its frame, in
/// decision order. Empty by default, which is also what an empty frame
/// is.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Recorded {
    /// The definite decisions, in decision order.
    pub verdicts: Vec<Verdict>,
    /// The indeterminate outcomes, in decision order.
    pub escalations: Vec<Escalation>,
}

/// The verdict-log bracket: a guard whose lifetime IS one frame on
/// this thread's stack (module docs). [`Bracket::open`] pushes the
/// frame; [`Bracket::finish`] pops it and returns what it recorded;
/// dropping the guard pops it too, by any path including a panic
/// unwinding through the bracketed code. Frames nest — a bracket
/// opened inside another records into its own frame and leaves the
/// outer one untouched.
///
/// `!Send` by construction (the `*const ()` phantom): a bracket closes
/// the frame on the thread that opened it, and the compiler refuses to
/// move it anywhere else.
///
/// ```compile_fail
/// let bracket = geom_core::k_stats::Bracket::open();
/// std::thread::spawn(move || drop(bracket));
/// ```
#[must_use = "a bracket records only while it is held; bind it, then `finish` it"]
#[derive(Debug)]
pub struct Bracket {
    /// The index of this bracket's frame on the stack — the invariant
    /// the close checks, so a bracket only ever pops its own frame.
    depth: usize,
    _confined: PhantomData<*const ()>,
}

impl Bracket {
    /// Opens a fresh, empty frame on this thread's stack. Every
    /// decision classified through the funnel from now until the
    /// bracket is finished or dropped (and outside any inner bracket)
    /// lands in it.
    pub fn open() -> Self {
        let depth = FRAMES.with(|f| {
            let mut frames = f.borrow_mut();
            frames.push(Recorded::default());
            frames.len() - 1
        });
        Self {
            depth,
            _confined: PhantomData,
        }
    }

    /// Closes the frame and returns everything it recorded. Consumes
    /// the bracket, so a frame is popped exactly once.
    pub fn finish(self) -> Recorded {
        // `Drop` would pop the frame a second time; skipping it is the
        // whole reason for the wrapper.
        let this = ManuallyDrop::new(self);
        pop_frame(this.depth)
    }
}

impl Drop for Bracket {
    fn drop(&mut self) {
        pop_frame(self.depth);
    }
}

/// Pops the frame a bracket opened at `depth`.
///
/// Brackets close in the reverse of the order they opened, so the
/// frame is the innermost one; a close out of that order is a bug in
/// the caller. It is asserted in debug builds. In release, frames above
/// `depth` (inner brackets still open) are discarded with this one, and
/// a stack already shorter than `depth + 1` (this frame is gone
/// already) pops nothing — so a bracket never returns another
/// bracket's decisions, whichever way the nesting was broken.
fn pop_frame(depth: usize) -> Recorded {
    FRAMES.with(|f| {
        let mut frames = f.borrow_mut();
        debug_assert_eq!(
            frames.len(),
            depth + 1,
            "brackets close in the reverse of the order they opened"
        );
        if frames.len() <= depth {
            return Recorded::default();
        }
        frames.truncate(depth + 1);
        frames.pop().unwrap_or_default()
    })
}

/// How many brackets are open on this thread — the tests' witness that
/// every path closes its frame.
#[cfg(test)]
fn open_frames() -> usize {
    FRAMES.with(|f| f.borrow().len())
}

/// How a recorded classification came out.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg(feature = "probe")]
pub enum SampleOutcome {
    /// A definite sign.
    Definite(Sign),
    /// The margin landed in the ambiguity band.
    Indeterminate,
    /// The margin was poisoned (NaN).
    Invalid,
    /// **The symbolic tier answered** (`crate::sym`, ERROR-DESIGN E12):
    /// the margin's expression is identically zero in the document's
    /// parameters, so `Zero` was a theorem and no enclosure was
    /// consulted.
    ///
    /// A separate outcome rather than a `Definite(Sign::Zero)` row, and
    /// the distinction is the whole E12 evidence: this sample's margin
    /// was never CLASSIFIED against the band, so it is never a rule-1
    /// in-band landing and never evidence about K. What it is evidence
    /// about is the ratio of symbolic to numeric decisions, which is
    /// what the tier exists to move.
    SymbolicZero,
}

#[cfg(feature = "probe")]
impl SampleOutcome {
    /// **Every outcome, once** — the roster a consumer enumerates
    /// instead of writing its own `match`.
    ///
    /// `tests::all_lists_every_variant` matches on each variant to
    /// prove the list is complete, so adding one without listing it
    /// here reds a test rather than leaving a silent hole in whatever
    /// derives from it.
    pub const ALL: [Self; 6] = [
        Self::Definite(Sign::Negative),
        Self::Definite(Sign::Zero),
        Self::Definite(Sign::Positive),
        Self::Indeterminate,
        Self::Invalid,
        Self::SymbolicZero,
    ];

    /// **The one spelling of this outcome**, and the K sweep's CSV
    /// vocabulary.
    ///
    /// It exists because there were five hand-kept copies of this
    /// `match` — four in Rust test harnesses that write the CSV, one in
    /// `tools/k-lint` that reads it — and when `SymbolicZero` arrived
    /// the writers learned it and the reader did not. The reader was
    /// right to refuse a token it did not know; nobody noticed, because
    /// the CI row that would have said so could not fail. A vocabulary
    /// that must agree across a tool boundary gets ONE definition and a
    /// test that pins the boundary
    /// (`k-lint`'s `tests/outcome_vocabulary.rs`).
    #[must_use]
    pub fn token(self) -> &'static str {
        match self {
            Self::Definite(Sign::Negative) => "negative",
            Self::Definite(Sign::Zero) => "zero",
            Self::Definite(Sign::Positive) => "positive",
            Self::Indeterminate => "indeterminate",
            Self::Invalid => "invalid",
            Self::SymbolicZero => "symbolic_zero",
        }
    }
}

/// One recorded classification: the raw material of a margin
/// distribution.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg(feature = "probe")]
pub struct MarginSample {
    /// The predicate that classified the margin (the funnel's name).
    pub predicate: &'static str,
    /// The classified margin, in the predicate's units (meters for
    /// the geometric predicates; radians·arm-metered margins where a
    /// predicate documents an angular band — see each crate's
    /// predicate inventory).
    pub margin: f64,
    /// The band's coincidence threshold.
    pub band_zero: f64,
    /// The band's escalation threshold (= K·zero).
    pub band_escalate: f64,
    /// The classification outcome.
    pub outcome: SampleOutcome,
}

/// Installs a fresh, empty sample sink for the current thread (dropping
/// any samples already recorded).
#[cfg(feature = "probe")]
pub fn start_recording() {
    SINK.with(|s| *s.borrow_mut() = Some(Vec::new()));
}

/// Removes the sink and returns everything recorded since
/// [`start_recording`]. Returns an empty vector if recording was never
/// started on this thread.
#[cfg(feature = "probe")]
pub fn take_samples() -> Vec<MarginSample> {
    SINK.with(|s| s.borrow_mut().take()).unwrap_or_default()
}

/// Records one sample if a sink is installed (called by [`Probe`]).
#[cfg(feature = "probe")]
fn record(margin: f64, band: Band, outcome: SampleOutcome) {
    SINK.with(|s| {
        if let Some(sink) = s.borrow_mut().as_mut() {
            sink.push(MarginSample {
                predicate: CURRENT.with(Cell::get),
                margin,
                band_zero: band.zero(),
                band_escalate: band.escalate(),
                outcome,
            });
        }
    });
}

/// **Re-tags the sample just recorded as [`SampleOutcome::SymbolicZero`]**
/// — the symbolic tier's door into the SAME funnel population
/// (`crate::sym`'s `Decide` impl calls it where the tier overrides the
/// numeric answer).
///
/// A re-tag rather than a second `record`, and that is the whole design:
/// `Sym<T>` asks its base scalar first (its domain refusal is clause 1
/// of the theorem), so at `Probe` the base scalar has ALREADY pushed the
/// sample with the margin it classified. Re-tagging keeps that margin —
/// a real number a reader can compare against the band — and keeps the
/// count exact, where recording a second row would double-count one
/// decision and inventing a margin would fabricate one.
///
/// No sink, or nothing recorded (every scalar but `Probe`): a no-op.
///
/// **The sample is named by INDEX, taken before the base scalar ran**,
/// never by position afterwards. `sink.last_mut()` was the first
/// spelling and it was wrong: at `Sym<Interval>` the base scalar records
/// nothing, so "the last sample" was whatever some earlier `Probe`
/// decision had left there — an unrelated row, and re-labelling an
/// `Indeterminate` one would have erased a rule-1 landing from the
/// population that exists to count them. [`sink_mark`] answers where
/// this decision's own sample WOULD go; re-tagging that index and only
/// when the base scalar actually filled it ties the two together by
/// construction.
#[cfg(feature = "probe")]
pub(crate) fn retag_symbolic_zero_at(mark: Option<usize>) {
    let Some(at) = mark else { return };
    SINK.with(|s| {
        if let Some(sink) = s.borrow_mut().as_mut()
            // Exactly one sample since the mark, and it is at `at`: the
            // base scalar recorded this decision and nothing else did.
            // Any other length means the base recorded nothing (a
            // non-`Probe` base, so there is no row of ours to re-tag)
            // or more than one, which no single `sign_within` can do —
            // and in either case the honest move is to leave the
            // population alone.
            && sink.len() == at + 1
            && let Some(mine) = sink.get_mut(at)
        {
            mine.outcome = SampleOutcome::SymbolicZero;
        }
    });
}

/// **Where the next recorded sample will land**, or `None` when no sink
/// is installed — the index [`retag_symbolic_zero_at`] re-tags.
///
/// Read BEFORE the base scalar decides, so the index names this
/// decision's own row rather than whatever happens to be last later.
#[cfg(feature = "probe")]
pub(crate) fn sink_mark() -> Option<usize> {
    SINK.with(|s| s.borrow().as_ref().map(Vec::len))
}

/// A transparent `f64` wrapper that records every sign classification —
/// the K-experiment recording scalar (module docs).
///
/// `Real` delegates every operation to the `f64` implementation
/// (through `<f64 as Real>`, so libm routing and all f64 semantics are
/// inherited verbatim); `Decide` delegates and records. Decisions are
/// therefore bit-identical to a plain `f64` run by construction.
#[derive(Clone, Copy, Debug)]
#[cfg(feature = "probe")]
pub struct Probe(pub f64);

#[cfg(feature = "probe")]
impl core::ops::Add for Probe {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0)
    }
}

#[cfg(feature = "probe")]
impl core::ops::Sub for Probe {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self(self.0 - rhs.0)
    }
}

#[cfg(feature = "probe")]
impl core::ops::Mul for Probe {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self(self.0 * rhs.0)
    }
}

#[cfg(feature = "probe")]
impl core::ops::Div for Probe {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        Self(self.0 / rhs.0)
    }
}

#[cfg(feature = "probe")]
impl core::ops::Neg for Probe {
    type Output = Self;

    fn neg(self) -> Self {
        Self(-self.0)
    }
}

#[cfg(feature = "probe")]
impl Real for Probe {
    fn from_f64(x: f64) -> Self {
        Self(x)
    }

    fn zero() -> Self {
        Self(<f64 as Real>::zero())
    }

    fn one() -> Self {
        Self(<f64 as Real>::one())
    }

    fn pi() -> Self {
        Self(<f64 as Real>::pi())
    }

    fn tau() -> Self {
        Self(<f64 as Real>::tau())
    }

    fn sqrt(self) -> Self {
        Self(Real::sqrt(self.0))
    }

    fn abs(self) -> Self {
        Self(Real::abs(self.0))
    }

    fn is_poison(self) -> bool {
        Real::is_poison(self.0)
    }

    fn powi(self, n: i32) -> Self {
        Self(Real::powi(self.0, n))
    }

    fn sin_cos(self) -> (Self, Self) {
        let (s, c) = Real::sin_cos(self.0);
        (Self(s), Self(c))
    }

    fn tan(self) -> Self {
        Self(Real::tan(self.0))
    }

    fn asin(self) -> Self {
        Self(Real::asin(self.0))
    }

    fn acos(self) -> Self {
        Self(Real::acos(self.0))
    }

    fn atan(self) -> Self {
        Self(Real::atan(self.0))
    }

    fn atan2(self, x: Self) -> Self {
        Self(Real::atan2(self.0, x.0))
    }

    fn min(self, other: Self) -> Self {
        Self(Real::min(self.0, other.0))
    }

    fn max(self, other: Self) -> Self {
        Self(Real::max(self.0, other.0))
    }

    fn floor(self) -> Self {
        Self(Real::floor(self.0))
    }

    fn copysign(self, sign: Self) -> Self {
        Self(Real::copysign(self.0, sign.0))
    }
}

/// `Probe` brackets itself exactly, like `f64` (it IS an f64 with a
/// recorder attached; delegation is exact, so the bracket is the
/// value). Needed so `Bounds`-bounded construction sugar (e.g. the
/// profile fillet constructor) runs at the recording scalar.
#[cfg(feature = "probe")]
impl Bounds for Probe {
    fn lo(self) -> f64 {
        self.0
    }

    fn hi(self) -> f64 {
        self.0
    }
}

/// `Probe` certifies exactly as `f64` does, and for the same reason: it
/// IS an `f64` with a recorder attached, so its value is its whole
/// domain-violation channel and it refuses on NaN and only on NaN.
/// Delegating rather than restating the test is what keeps the two in
/// step: a `--features probe` build that refused where the `f64` build
/// certifies, or certified where it refuses, is precisely the divergence
/// D9 forbids of this scalar.
#[cfg(feature = "probe")]
impl crate::real::CertifiedEnclosure for Probe {
    fn certified_bracket(self) -> Option<(f64, f64)> {
        crate::real::CertifiedEnclosure::certified_bracket(self.0)
    }
}

/// `Probe` locates spans through its `f64` (module docs of
/// [`crate::spline::locate`]): it IS an `f64` with a recorder, and span
/// selection is structure selection, not a recorded decision — no
/// margin sample is emitted (span choice never drives topology).
#[cfg(feature = "probe")]
impl crate::spline::SpanLocate for Probe {
    fn locate_spans(self, knots: &crate::spline::KnotVector) -> crate::spline::SpanSet {
        crate::spline::SpanLocate::locate_spans(self.0, knots)
    }

    fn enclosure_hull(self, _other: Self) -> Self {
        // Unreachable through the evaluators (single-span locator, like
        // f64); total anyway — poison, never a fabricated value.
        Self(f64::NAN)
    }
}

#[cfg(feature = "probe")]
impl Decide for Probe {
    fn sign_within(self, band: Band) -> Result<Sign, Indeterminate> {
        let outcome = self.0.sign_within(band);
        let sample = match &outcome {
            Ok(sign) => SampleOutcome::Definite(*sign),
            Err(e) => match e.margin {
                crate::predicate::MarginDiag::Invalid => SampleOutcome::Invalid,
                _ => SampleOutcome::Indeterminate,
            },
        };
        record(self.0, band, sample);
        outcome
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::panic)]

    use super::*;
    use crate::tolerance::Tol;

    fn band() -> Band {
        Band::linear(Tol::witness()).unwrap()
    }

    /// [`SampleOutcome::ALL`] lists every variant, proven by matching on
    /// each one: a variant added without a roster entry stops compiling
    /// here rather than leaving whatever derives from `ALL` silently
    /// short — which is exactly how `tools/k-lint`'s accepted-token list
    /// came to be missing `symbolic_zero`.
    #[cfg(feature = "probe")]
    #[test]
    fn all_lists_every_variant_and_every_token_is_distinct() {
        for o in SampleOutcome::ALL {
            // Exhaustive by construction: a new variant reds here.
            let seen = match o {
                SampleOutcome::Definite(Sign::Negative)
                | SampleOutcome::Definite(Sign::Zero)
                | SampleOutcome::Definite(Sign::Positive)
                | SampleOutcome::Indeterminate
                | SampleOutcome::Invalid
                | SampleOutcome::SymbolicZero => true,
            };
            assert!(seen, "{o:?} is listed in ALL");
        }
        let mut tokens: Vec<&str> = SampleOutcome::ALL.iter().map(|o| o.token()).collect();
        let before = tokens.len();
        tokens.sort_unstable();
        tokens.dedup();
        assert_eq!(
            tokens.len(),
            before,
            "two outcomes share a token, so a CSV reader cannot tell them apart: {tokens:?}"
        );
    }

    #[test]
    fn verdict_log_records_definite_signs_in_decision_order() {
        let b = band();
        let bracket = Bracket::open();
        assert_eq!(decide("vlog_a", Margin::of(1.0f64), b), Ok(Sign::Positive));
        assert_eq!(decide("vlog_b", Margin::of(-1.0f64), b), Ok(Sign::Negative));
        assert_eq!(decide("vlog_c", Margin::of(0.0f64), b), Ok(Sign::Zero));
        let log = bracket.finish();
        assert_eq!(
            log.verdicts,
            vec![
                Verdict {
                    predicate: "vlog_a",
                    sign: Sign::Positive
                },
                Verdict {
                    predicate: "vlog_b",
                    sign: Sign::Negative
                },
                Verdict {
                    predicate: "vlog_c",
                    sign: Sign::Zero
                },
            ]
        );
        assert!(log.escalations.is_empty());
    }

    /// An indeterminate outcome is not a verdict; it is an escalation,
    /// recorded in the same frame with the `Indeterminate` the
    /// predicate produced, so a consumer reads it without matching on
    /// whatever error the op wrapped it in. With no bracket open,
    /// nothing records anywhere.
    #[test]
    fn indeterminate_outcomes_are_escalations_and_nothing_records_outside_a_bracket() {
        let b = band();
        assert_eq!(open_frames(), 0);
        // No bracket: decisions record nothing, and the stack stays empty.
        assert_eq!(decide("vlog_d", Margin::of(2.0f64), b), Ok(Sign::Positive));
        assert_eq!(open_frames(), 0);
        let bracket = Bracket::open();
        let mid = f64::midpoint(b.zero(), b.escalate());
        let escalated = decide("vlog_e", Margin::of(mid), b).unwrap_err();
        assert_eq!(decide("vlog_f", Margin::of(-1.0f64), b), Ok(Sign::Negative));
        let log = bracket.finish();
        assert_eq!(log.verdicts.len(), 1);
        assert_eq!(log.verdicts[0].predicate, "vlog_f");
        assert_eq!(
            log.escalations,
            vec![Escalation {
                predicate: "vlog_e",
                source: escalated,
            }]
        );
        assert_eq!(log.escalations[0].source.predicate, Some("vlog_e"));
        assert_eq!(open_frames(), 0);
    }

    /// **The nesting row**: an inner bracket records into its own
    /// frame and leaves the outer one exactly as it was, and the outer
    /// frame receives only the decisions made outside the inner one.
    #[test]
    fn a_nested_bracket_records_its_own_frame_and_leaves_the_outer_untouched() {
        let b = band();
        let outer = Bracket::open();
        decide("vlog_g", Margin::of(1.0f64), b).unwrap();
        let inner = Bracket::open();
        assert_eq!(open_frames(), 2);
        decide("vlog_h", Margin::of(1.0f64), b).unwrap();
        let inner_log = inner.finish();
        decide("vlog_i", Margin::of(-1.0f64), b).unwrap();
        let outer_log = outer.finish();
        let names = |r: &Recorded| r.verdicts.iter().map(|v| v.predicate).collect::<Vec<_>>();
        assert_eq!(names(&inner_log), ["vlog_h"]);
        assert_eq!(names(&outer_log), ["vlog_g", "vlog_i"]);
        assert_eq!(open_frames(), 0);
    }

    /// A bracket dropped without `finish` still pops its frame: what it
    /// recorded is gone, and the frame beneath it is the innermost
    /// again.
    #[test]
    fn a_dropped_bracket_pops_its_frame() {
        let b = band();
        let outer = Bracket::open();
        {
            let _inner = Bracket::open();
            decide("vlog_j", Margin::of(1.0f64), b).unwrap();
            assert_eq!(open_frames(), 2);
        }
        assert_eq!(open_frames(), 1);
        decide("vlog_k", Margin::of(1.0f64), b).unwrap();
        let log = outer.finish();
        assert_eq!(log.verdicts.len(), 1);
        assert_eq!(log.verdicts[0].predicate, "vlog_k");
        assert_eq!(open_frames(), 0);
    }

    /// A panic unwinding through a bracketed region pops the frame on
    /// the way out (the guard's `Drop` runs during unwinding), so the
    /// thread's stack is empty again once the panic is caught and the
    /// next bracket starts from a clean stack.
    #[test]
    fn a_panic_unwinding_through_a_bracket_pops_its_frame() {
        let b = band();
        let unwound = std::panic::catch_unwind(|| {
            let _bracket = Bracket::open();
            decide("vlog_l", Margin::of(1.0f64), b).unwrap();
            assert_eq!(open_frames(), 1);
            panic!("unwinding through the bracket");
        });
        assert!(unwound.is_err());
        assert_eq!(open_frames(), 0);
        let after = Bracket::open();
        decide("vlog_m", Margin::of(1.0f64), b).unwrap();
        let log = after.finish();
        assert_eq!(log.verdicts.len(), 1);
        assert_eq!(log.verdicts[0].predicate, "vlog_m");
    }

    /// A second bracket opened beside the first is a NESTED one, not a
    /// replacement: the first keeps everything it recorded.
    #[test]
    fn opening_a_second_bracket_does_not_drop_the_first_frame() {
        let b = band();
        let first = Bracket::open();
        decide("vlog_n", Margin::of(1.0f64), b).unwrap();
        let second = Bracket::open();
        decide("vlog_o", Margin::of(1.0f64), b).unwrap();
        assert_eq!(second.finish().verdicts.len(), 1);
        let log = first.finish();
        assert_eq!(log.verdicts.len(), 1);
        assert_eq!(log.verdicts[0].predicate, "vlog_n");
    }
}
