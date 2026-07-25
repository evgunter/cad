//! The tour's ONLY boolean call sites — a deliberately thin,
//! centralized wrapper over `topo::{union, subtract, intersect}` so
//! any API shift is a one-file adaptation here — plus the shared
//! exact-volume oracle every boolean scene runs its results through.
//!
//! Tier-3 posture: RETIRED as a workaround. Boolean results validate
//! as they are, via `validate_pseudomanifold` with the op's own
//! declared `contacts` (M3 PR 6a's 3′ contract) — see
//! `crate::run_body`. The `upgrade_edges_to_intersections` clone hack
//! that used to live here (a PR 3-era description-gap workaround) is
//! deleted; the kernel caught up.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use topo::{Body, BooleanBody, BooleanError, BooleanResult, BooleanResultKind};

/// A ∪* B — refusals surface to the caller; every result goes through
/// the scene builders' exact-volume oracle before it ships.
pub fn try_union(a: &Body<f64>, b: &Body<f64>) -> Result<BooleanResult<f64>, BooleanError> {
    topo::union(a, b)
}

/// A ∖* B (same posture as [`try_union`]).
pub fn try_subtract(a: &Body<f64>, b: &Body<f64>) -> Result<BooleanResult<f64>, BooleanError> {
    topo::subtract(a, b)
}

/// A ∩* B (same posture as [`try_union`]).
pub fn try_intersect(a: &Body<f64>, b: &Body<f64>) -> Result<BooleanResult<f64>, BooleanError> {
    topo::intersect(a, b)
}

/// The oracle: volume of a boolean result vs the exact expectation.
/// `Good` carries the whole [`BooleanBody`] (body + kind + the
/// declared contacts the 3′ gate consumes); the two failure shapes
/// carry what actually happened, for narration.
// Size skew vs the slim failure variants is inherent (same posture as
// the kernel's own `BooleanResult`).
#[allow(clippy::large_enum_variant)]
pub enum Verdict {
    Good(BooleanBody<f64>),
    /// Op "succeeded" (tier 1+2 legal) with the WRONG volume — the
    /// silent wrong-component defect class (extinct since the PR 5
    /// fix pass; the oracle stays armed anyway).
    Wrong(f64, BooleanResultKind),
    Refused(BooleanError),
}

pub fn check(r: Result<BooleanResult<f64>, BooleanError>, expected: f64) -> Verdict {
    match r {
        Ok(BooleanResult::Body(b)) => {
            let v = topo::mass_properties(&b.body)
                .expect("mass properties")
                .volume;
            if (v - expected).abs() <= 1e-9 {
                Verdict::Good(b)
            } else {
                Verdict::Wrong(v, b.kind)
            }
        }
        Ok(BooleanResult::Empty) => Verdict::Refused(BooleanError::UnrepresentableResult),
        Err(e) => Verdict::Refused(e),
    }
}

pub fn describe(v: &Verdict, expected: f64) -> String {
    match v {
        Verdict::Good(b) => format!("OK (kind {:?}, volume exact {expected})", b.kind),
        Verdict::Wrong(vol, kind) => format!(
            "SILENT WRONG RESULT (kind {kind:?}): tier 1+2 passed but volume = {vol} \
             instead of {expected} — caught by the tour's volume oracle"
        ),
        Verdict::Refused(e) => format!("typed refusal (fail-loud): {e:?}"),
    }
}

/// Unwraps a [`Verdict`] the scene REQUIRES to be good and `Seamed`,
/// with the failure narrated in the panic.
pub fn expect_seamed(what: &str, v: Verdict, expected: f64) -> BooleanBody<f64> {
    match v {
        Verdict::Good(b) => {
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
