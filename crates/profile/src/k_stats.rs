//! Margin-statistics collection for the K-value experiments — a
//! recording wrapper, not telemetry infrastructure (M2-PLAN PR 2's
//! K-experiment hook; Q1's "ambiguity constant K" residue).
//!
//! Every decision this crate makes goes through one funnel,
//! [`decide`](crate::k_stats) (crate-internal): it notes the predicate's
//! static name in a thread-local, classifies the margin through the one
//! sanctioned door ([`Decide::sign_within`]), and attaches the name to
//! any indeterminate outcome. Production code pays one thread-local
//! `Cell` write per decision and records nothing.
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

use geom_core::{Band, Decide, Indeterminate, Real, Sign};

thread_local! {
    /// The name of the predicate currently being decided (set by the
    /// funnel just before classification; read by [`Probe`]).
    static CURRENT: Cell<&'static str> = const { Cell::new("<unnamed>") };
    /// The installed sample sink, if any.
    static SINK: RefCell<Option<Vec<MarginSample>>> = const { RefCell::new(None) };
}

/// The one classification funnel for this crate: notes `name` for the
/// recorder, classifies `margin` against `band`, and names any
/// indeterminate outcome. Every predicate in this crate calls this —
/// it is the crate's only `sign_within` call site outside [`Probe`]
/// (greppable).
pub(crate) fn decide<T: Decide>(
    name: &'static str,
    margin: T,
    band: Band,
) -> Result<Sign, Indeterminate> {
    CURRENT.with(|c| c.set(name));
    margin.sign_within(band).map_err(|e| e.with_predicate(name))
}

/// How a recorded classification came out.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
pub struct MarginSample {
    /// The predicate that classified the margin (the funnel's name).
    pub predicate: &'static str,
    /// The classified margin, in the predicate's units (meters for
    /// every geometric predicate in this crate — see the predicate
    /// inventory in `crate::validate`).
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
pub fn start_recording() {
    SINK.with(|s| *s.borrow_mut() = Some(Vec::new()));
}

/// Removes the sink and returns everything recorded since
/// [`start_recording`]. Returns an empty vector if recording was never
/// started on this thread.
pub fn take_samples() -> Vec<MarginSample> {
    SINK.with(|s| s.borrow_mut().take()).unwrap_or_default()
}

/// Records one sample if a sink is installed (called by [`Probe`]).
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
pub struct Probe(pub f64);

impl core::ops::Add for Probe {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0)
    }
}

impl core::ops::Sub for Probe {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self(self.0 - rhs.0)
    }
}

impl core::ops::Mul for Probe {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self(self.0 * rhs.0)
    }
}

impl core::ops::Div for Probe {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        Self(self.0 / rhs.0)
    }
}

impl core::ops::Neg for Probe {
    type Output = Self;

    fn neg(self) -> Self {
        Self(-self.0)
    }
}

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

impl Decide for Probe {
    fn sign_within(self, band: Band) -> Result<Sign, Indeterminate> {
        let outcome = self.0.sign_within(band);
        let sample = match &outcome {
            Ok(sign) => SampleOutcome::Definite(*sign),
            Err(e) => match e.margin {
                geom_core::MarginDiag::Invalid => SampleOutcome::Invalid,
                _ => SampleOutcome::Indeterminate,
            },
        };
        record(self.0, band, sample);
        outcome
    }
}
