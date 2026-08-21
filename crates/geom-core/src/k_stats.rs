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
//! decision, plus — **whenever a verdict log is installed** — one
//! `Vec` push per definite outcome (see [`start_verdict_log`] and
//! `decide`'s own contract below, which has always said this). That
//! caveat is not hypothetical: `editor_core`'s evaluator brackets
//! **every node evaluation** in a verdict log (NAMING-DESIGN N5, so
//! the verdict-diff engine can attribute flips), and retains the
//! result on the node. So production *does* record, on the one path
//! that asks to.
//!
//! **`VERDICTS` is not part of the `probe` lane and must not be gated
//! on it.** The K-telemetry sink is `SINK`, which *is* feature-gated;
//! `VERDICTS` merely shares this funnel because this is where decisions
//! pass. Its consumer is production editor-core code —
//! `resolve::vdiff` reads `NodeValue::verdicts` to compare per-predicate
//! sign populations and emit `NodeVerdictDelta`'s flips and
//! divergences. Putting it behind `probe` would not reduce recording;
//! it would hand the verdict-diff engine empty logs in every default
//! build and silently stop it attributing flips. The name is the
//! confusing part, not the design.
//!
//! Paths that never install a log (STL export, step-import, the demos)
//! still pay the `RefCell` borrow check per decision. Gating *that* on
//! a `Cell<bool>` is a live optimization and is orthogonal to any
//! feature — it behaves identically in every build configuration.
//!
//! **OPEN OBLIGATION — this mechanism is on notice; see
//! `docs/PERF-SCAN-2026-08.md` §2.** Delivering a production value by
//! thread-local side effect makes the per-node bracket's correctness a
//! comment rather than a type, and it has already failed once:
//! [`start_verdict_log`] overwrites an installed log unconditionally,
//! so a nested evaluation destroys its parent's — measured, an
//! `InstantiatePart` node records **0** verdicts where the same
//! geometry evaluated directly records 722. The obligation is that this
//! is redone so verdicts are a returned value, or that the alternative
//! is proven unaffordable in writing AND this mechanism is made
//! structurally safe (RAII bracket, re-entry refused loudly, thread
//! confinement enforced rather than asserted). **Which of those two is
//! UNRESOLVED — left open deliberately at merge (Evan, 2026-08-16), not
//! overlooked.** The nesting bug itself is not blocked on that choice
//! and can be fixed directly. Do not add call sites that deepen the
//! dependency on the current shape. **The `classify` consolidation was
//! measured against this fence and clears it in the permitted
//! direction**: the per-decision `VERDICTS` push went from three sites
//! to one. That is the funnel's share and not the remedy's blast
//! radius — [`start_verdict_log`] and [`take_verdict_log`] also touch
//! the thread-local, and either named remedy (verdicts as a returned
//! value, or an RAII bracket) has to change those two and their callers
//! as well. The consolidation makes that work smaller by one door pair;
//! it does not make it small.
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
    /// The installed verdict-log sink, if any (NAMING-DESIGN N5:
    /// evaluations record their verdict vectors so the verdict-diff
    /// engine can attribute flips).
    static VERDICTS: RefCell<Option<Vec<Verdict>>> = const { RefCell::new(None) };
}

/// Classifies `margin` against `band`, noting `name` for the recorder
/// and attaching it to any indeterminate outcome — the shared body of
/// every public door below.
///
/// This is the only place either recording channel is written, which
/// is why the doors delegate rather than each carrying a copy: the
/// `CURRENT` write is the recorder's name channel (read by `Probe`),
/// and the verdict push is the evaluation-artifact channel (the log
/// itself is installed and removed by [`start_verdict_log`] /
/// [`take_verdict_log`], and read by the verdict-diff engine). A door
/// cannot acquire one and miss the other.
fn classify<T: Decide>(name: &'static str, margin: T, band: Band) -> Result<Sign, Indeterminate> {
    CURRENT.with(|c| c.set(name));
    let outcome = margin.sign_within(band).map_err(|e| e.with_predicate(name));
    if let Ok(sign) = outcome {
        VERDICTS.with(|v| {
            if let Some(log) = v.borrow_mut().as_mut() {
                log.push(Verdict {
                    predicate: name,
                    sign,
                });
            }
        });
    }
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
/// per decision (plus, when a verdict log is installed —
/// [`start_verdict_log`] — one `Vec` push per definite outcome); the
/// decision itself is `sign_within` verbatim, so outcomes are
/// bit-identical to an unfunneled classification.
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
/// `flagged_census.rs` carry it over `crates/*/src`, and **only one of
/// them computes anything**:
///
/// - **Self-enforcing:** every site's `ledger_row` must name a row the
///   audit actually has. The rows are read out of the document at run
///   time, so a citation to a row that was renumbered or never existed
///   fails without anyone remembering to look.
/// - **Hand-synced, and it is the residue:** the site COUNT is compared
///   against a literal in the test, which is kept level with the audit's
///   prose by hand. Nothing derives it. That is the *magic count* §S13
///   names, unfixed; it is stated at the constant rather than covered
///   over here.
///
/// The scan's own pattern says what it cannot see (a renamed import, a
/// macro-generated call, a block-commented site), because a census whose
/// blind spot is unstated reads as coverage it does not have. Fixtures
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
/// debt** (Evan's #213 layering ruling). A consistency backstop is
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

/// Installs a fresh, empty verdict log for the current thread
/// (dropping any verdicts already recorded). Unlike [`start_recording`]
/// (the K-experiment margin sink, [`Probe`]-only), the verdict log
/// records through [`decide`] itself at ANY scalar — it is the
/// evaluation-artifact channel NAMING-DESIGN N5's diagnosis diffing
/// relies on ("both evaluations' verdict logs exist"). Indeterminate
/// outcomes are not verdicts (they escalate typed instead of
/// deciding) and are not logged.
pub fn start_verdict_log() {
    VERDICTS.with(|v| *v.borrow_mut() = Some(Vec::new()));
}

/// Removes the verdict log and returns everything recorded since
/// [`start_verdict_log`]. Returns an empty vector if no log was
/// installed on this thread.
pub fn take_verdict_log() -> Vec<Verdict> {
    VERDICTS.with(|v| v.borrow_mut().take()).unwrap_or_default()
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

    fn band() -> Band {
        Band::linear().unwrap()
    }

    #[test]
    fn verdict_log_records_definite_signs_in_decision_order() {
        let b = band();
        start_verdict_log();
        assert_eq!(decide("vlog_a", Margin::of(1.0f64), b), Ok(Sign::Positive));
        assert_eq!(decide("vlog_b", Margin::of(-1.0f64), b), Ok(Sign::Negative));
        assert_eq!(decide("vlog_c", Margin::of(0.0f64), b), Ok(Sign::Zero));
        let log = take_verdict_log();
        assert_eq!(
            log,
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
    }

    #[test]
    fn verdict_log_skips_indeterminate_outcomes_and_is_absent_by_default() {
        let b = band();
        // No log installed: decisions record nothing, take is empty.
        assert_eq!(decide("vlog_d", Margin::of(2.0f64), b), Ok(Sign::Positive));
        assert!(take_verdict_log().is_empty());
        // In-band margin escalates and is NOT a verdict.
        start_verdict_log();
        let mid = f64::midpoint(b.zero(), b.escalate());
        assert!(decide("vlog_e", Margin::of(mid), b).is_err());
        assert_eq!(decide("vlog_f", Margin::of(-1.0f64), b), Ok(Sign::Negative));
        let log = take_verdict_log();
        assert_eq!(log.len(), 1);
        assert_eq!(log[0].predicate, "vlog_f");
    }

    #[test]
    fn verdict_log_reinstall_drops_prior_entries() {
        let b = band();
        start_verdict_log();
        decide("vlog_g", Margin::of(1.0f64), b).unwrap();
        start_verdict_log();
        decide("vlog_h", Margin::of(1.0f64), b).unwrap();
        let log = take_verdict_log();
        assert_eq!(log.len(), 1);
        assert_eq!(log[0].predicate, "vlog_h");
    }
}
