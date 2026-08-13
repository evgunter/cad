//! Margin-statistics collection for the K-value experiments — a
//! recording wrapper, not telemetry infrastructure (M2-PLAN PR 2's
//! K-experiment hook; Q1's "ambiguity constant K" residue; unified
//! here from `profile::k_stats` in M2 PR 7 so every crate's decisions
//! share one recorder).
//!
//! Every decision the kernel makes goes through one funnel, [`decide`]:
//! it notes the predicate's static name in a thread-local, classifies
//! the margin through the one sanctioned door ([`Decide::sign_within`]),
//! and attaches the name to any indeterminate outcome. Each deciding
//! crate re-exports or wraps this funnel as its single greppable
//! `sign_within` call site (the `geom-brep` funnel pattern); after the
//! PR 7 unification a [`Probe`]-lane run tags every sample with its
//! real predicate name — `<unnamed>` is unreachable from shipped decide
//! paths. Production code pays one thread-local `Cell` write per
//! decision and records nothing.
//!
//! Recording happens through the [`Probe`] scalar: a transparent `f64`
//! wrapper whose `Decide` implementation logs `(predicate, margin,
//! band, outcome)` to a thread-local sink before delegating. Running
//! validation at `T = Probe` therefore yields the complete per-predicate
//! margin distribution of the run — the data PR 7's K report pulls —
//! with **zero** instrumentation in the validation code itself and
//! bit-identical decisions to `f64` (delegation is exact). The sink is
//! thread-local and explicitly installed ([`start_recording`] /
//! [`take_samples`]), so tests never race and production never records.

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
    /// The installed verdict-log sink, if any (NAMING-DESIGN N5 /
    /// M4 PR 4: evaluations record their verdict vectors so the
    /// verdict-diff engine can attribute flips).
    static VERDICTS: RefCell<Option<Vec<Verdict>>> = const { RefCell::new(None) };
}

/// The one classification funnel of the kernel: notes `name` for the
/// recorder, classifies `margin` against `band`, and names any
/// indeterminate outcome. Every deciding crate routes its predicates
/// through this function (directly or via a thin crate-local wrapper),
/// so it is the only shipped `sign_within` call site outside [`Probe`]
/// (greppable per crate).
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
    CURRENT.with(|c| c.set(name));
    let outcome = margin
        .value()
        .sign_within(band)
        .map_err(|e| e.with_predicate(name));
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

/// The classify seam's **finding lane** — [`decide`] for a shipped
/// comparand whose ledger row (`docs/predicate-dimension-audit.md`)
/// documents that **no construction door honestly fits**: the margin is
/// not (yet) a length, and wrapping it in a [`Margin`] door would
/// launder the very defect the ledger records. This function does NOT
/// construct a `Margin` — the margin stays bare `T` and never claims
/// the dimension it lacks; classification and recording are otherwise
/// [`decide`] verbatim, so the K stream is unchanged.
///
/// `ledger_row` names the audit row that argues the absence (e.g.
/// `"F2"`, `"F7"`). It is a compile-time obligation, not telemetry: a
/// new call site without a corresponding flagged ledger row is a review
/// reject — this lane is the greppable inventory of clause-(i) debt,
/// shrinking as the flagged families get their own units, never a
/// convenience door.
///
/// **Standing rule (the debt lane is tracked as issue #214): no new
/// `decide_flagged` site ships without a ledger row in
/// `docs/predicate-dimension-audit.md`.** The census count assertion
/// (`geom-core/tests/flagged_census.rs`) pins the shipped-site count to
/// the ledger's inventory, so adding a site without updating both the
/// ledger and the assertion fails the suite — the rule enforces
/// itself.
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
    let _ = ledger_row;
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
/// Classification and recording are [`decide`] verbatim (same
/// recorder, same names, same values — the K stream is unchanged). A
/// certified violation on this lane is a **kernel invariant** failure:
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
