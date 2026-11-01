//! A deliberately thin wrapper over `pncad::topo::{union, subtract,
//! intersect}` for the scenes that run through it (lily, klein and
//! snowman call the doors directly), plus the shared exact-volume oracle
//! every boolean scene runs its results through.
//!
//! Generic over [`Scalar`] (M4 PR 8b): the same wrappers serve the
//! f64 tour and the Probe K-telemetry sweep; oracles compare through
//! the exact `f()` extraction.
//!
//! Operands are finished bodies ([`AtRestBody`]), the doors' own
//! type: a scene finishes each operand it builds once, with
//! [`finished`], and passes a boolean result (already finished) straight
//! on to the next op.
//!
//! Tier-3 posture: boolean results validate as they are, via
//! `validate_pseudomanifold` with the op's own `contacts`
//! (M3 PR 6a's 3′ contract) — see `crate::run_body`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use pncad::topo::{AtRestBody, Body, BooleanBody, BooleanError, BooleanResult, BooleanResultKind};

use crate::scalar::Scalar;
use pncad::geom_core::Tol;

/// `body` finished for the boolean doors, through the scalar's at-rest
/// gate. A tour scene authors its own operands, so one the gate refuses
/// is a scene or kernel defect, and the panic names `what` and the
/// validator's findings.
pub fn finished<S: Scalar>(what: &str, body: Body<S>, tol: Tol) -> AtRestBody<S> {
    S::gate_at_rest_kept(body, tol)
        .unwrap_or_else(|e| panic!("{what} is not a finished body: {e:?}"))
}

/// A copy of `body` finished for the measurement doors
/// (`mass_properties`, `classify_shells`), which read a finished body
/// (Ev's decision on PR 4822). Each body a stop measures is finished
/// once, where props and classification both read it.
pub fn at_rest<S: Scalar>(body: &Body<S>, tol: Tol) -> AtRestBody<S> {
    finished("the measured body", body.clone(), tol)
}

/// A ∪* B — refusals surface to the caller; every result goes through
/// the scene builders' exact-volume oracle before it ships.
pub fn try_union<S: Scalar>(
    a: &AtRestBody<S>,
    b: &AtRestBody<S>,
    tol: Tol,
) -> Result<BooleanResult<S>, BooleanError> {
    pncad::topo::union(a, b, tol)
}

/// A ∖* B (same posture as [`try_union`]).
pub fn try_subtract<S: Scalar>(
    a: &AtRestBody<S>,
    b: &AtRestBody<S>,
    tol: Tol,
) -> Result<BooleanResult<S>, BooleanError> {
    pncad::topo::subtract(a, b, tol)
}

/// A ∩* B (same posture as [`try_union`]).
pub fn try_intersect<S: Scalar>(
    a: &AtRestBody<S>,
    b: &AtRestBody<S>,
    tol: Tol,
) -> Result<BooleanResult<S>, BooleanError> {
    pncad::topo::intersect(a, b, tol)
}

/// The oracle: volume of a boolean result vs the exact expectation.
/// `Good` carries the whole [`BooleanBody`] (body + kind + the
/// contact records the 3′ gate consumes); the two failure shapes
/// carry what actually happened, for narration.
// Size skew vs the slim failure variants is inherent (same posture as
// the kernel's own `BooleanResult`).
#[allow(clippy::large_enum_variant)]
pub enum Verdict<S: Scalar> {
    /// The bool records whether the volume matched the oracle
    /// BIT-EXACTLY (#91 review N1: the gate is 1e-9 because the
    /// table's non-dyadic dims carry a few-ulp integration gap;
    /// dyadic scenes are observed bit-exact and say so).
    Good(BooleanBody<S>, bool),
    /// Op "succeeded" (tier 1+2 legal) with the WRONG volume — the
    /// silent wrong-component defect class, which no kernel gate
    /// checks for.
    Wrong(f64, BooleanResultKind),
    Refused(BooleanError),
    /// The op returned the typed EMPTY success (F8: the empty set is
    /// a value) — no tour scene expects one, so it gets its own
    /// honest label instead of masquerading as a refusal.
    Empty,
}

pub fn check<S: Scalar>(
    r: Result<BooleanResult<S>, BooleanError>,
    expected: f64,
    tol: Tol,
) -> Verdict<S> {
    match r {
        Ok(BooleanResult::Body(b)) => {
            let v = pncad::topo::mass_properties(&b.body, tol, crate::printed())
                .expect("mass properties")
                .enclosure
                .volume
                .f();
            if (v - expected).abs() <= 1e-9 {
                Verdict::Good(b, v == expected)
            } else {
                Verdict::Wrong(v, b.kind)
            }
        }
        Ok(BooleanResult::Empty) => Verdict::Empty,
        Err(e) => Verdict::Refused(e),
    }
}

pub fn describe<S: Scalar>(v: &Verdict<S>, expected: f64) -> String {
    match v {
        Verdict::Good(b, bit_exact) => format!(
            "OK (kind {:?}, volume = {expected}, {})",
            b.kind,
            if *bit_exact {
                "observed bit-exact (gated 1e-9)"
            } else {
                "within 1e-9 of the oracle"
            }
        ),
        Verdict::Wrong(vol, kind) => format!(
            "SILENT WRONG RESULT (kind {kind:?}): tier 1+2 passed but volume = {vol} \
             instead of {expected} — caught by the tour's volume oracle"
        ),
        Verdict::Refused(e) => format!("typed refusal (fail-loud): {e:?}"),
        Verdict::Empty => {
            "typed EMPTY success (empty result - unexpected in this scene)".to_string()
        }
    }
}

/// Unwraps a [`Verdict`] the scene REQUIRES to be good and `Seamed`,
/// with the failure narrated in the panic.
pub fn expect_seamed<S: Scalar>(what: &str, v: Verdict<S>, expected: f64) -> BooleanBody<S> {
    match v {
        Verdict::Good(b, _) => {
            assert_eq!(
                b.kind,
                BooleanResultKind::Seamed,
                "{what}: expected a Seamed result"
            );
            b
        }
        other => panic!("{what} failed: {}", describe(&other, expected)),
    }
}
