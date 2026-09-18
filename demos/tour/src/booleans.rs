//! The tour's ONLY boolean call sites — a deliberately thin,
//! centralized wrapper over `pncad::topo::{union, subtract, intersect}` so
//! any API shift is a one-file adaptation here — plus the shared
//! exact-volume oracle every boolean scene runs its results through.
//!
//! Generic over [`Scalar`] (M4 PR 8b): the same wrappers serve the
//! f64 tour and the Probe K-telemetry sweep; oracles compare through
//! the exact `f()` extraction.
//!
//! Tier-3 posture: boolean results validate as they are, via
//! `validate_pseudomanifold` with the op's own declared `contacts`
//! (M3 PR 6a's 3′ contract) — see `crate::run_body`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use pncad::topo::{Body, BooleanBody, BooleanError, BooleanResult, BooleanResultKind};

use crate::scalar::Scalar;
use pncad::geom_core::Tol;

/// A ∪* B — refusals surface to the caller; every result goes through
/// the scene builders' exact-volume oracle before it ships.
pub fn try_union<S: Scalar>(
    a: &Body<S>,
    b: &Body<S>,
    tol: Tol,
) -> Result<BooleanResult<S>, BooleanError> {
    pncad::topo::union(a, b, tol)
}

/// A ∖* B (same posture as [`try_union`]).
pub fn try_subtract<S: Scalar>(
    a: &Body<S>,
    b: &Body<S>,
    tol: Tol,
) -> Result<BooleanResult<S>, BooleanError> {
    pncad::topo::subtract(a, b, tol)
}

/// A ∩* B (same posture as [`try_union`]).
pub fn try_intersect<S: Scalar>(
    a: &Body<S>,
    b: &Body<S>,
    tol: Tol,
) -> Result<BooleanResult<S>, BooleanError> {
    pncad::topo::intersect(a, b, tol)
}

/// A ∪* B with the scene's flush contacts DECLARED (M4 PR 5: the
/// author's coincidence intent, stated — the kernel never infers it
/// from values).
pub fn try_union_declared<S: Scalar>(
    a: &Body<S>,
    b: &Body<S>,
    tol: Tol,
) -> Result<BooleanResult<S>, BooleanError> {
    pncad::topo::union_with(a, b, &flush_declarations(a, b, tol), tol)
}

/// A ∩* B with the scene's flush contacts declared
/// ([`try_union_declared`]).
pub fn try_intersect_declared<S: Scalar>(
    a: &Body<S>,
    b: &Body<S>,
    tol: Tol,
) -> Result<BooleanResult<S>, BooleanError> {
    pncad::topo::intersect_with(a, b, &flush_declarations(a, b, tol), tol)
}

/// The scene's flush contacts, DETECTED and then DECLARED — the two
/// library doors a user would reach for, spelled the way a user would
/// spell them: [`pncad::topo::flush::find_flush_candidates`] reports
/// the cross-body `Rest` pairs the boolean's own verifier would accept
/// — every carrier that verifier has a rung for, plane through torus
/// — and [`pncad::topo::flush::declare_all`] turns the findings the
/// caller has seen into the declarations the op takes. The scene
/// author BUILT the contact deliberately; this writes the intent down.
/// Certification still happens inside the op through the verified
/// declared rung.
///
/// The findings pass through this function's hands as VALUES, which is
/// the no-fusion boundary the library keeps: there is no door that
/// detects and declares in one call, and this helper is the tour's
/// one place that does both in sequence.
///
/// It declares EVERY finding, which is what makes it a helper rather
/// than a scene's own voice: a scene that means some of its contacts
/// and not others picks out of the report itself (`twopeg` does).
/// Scenes whose contacts must keep REFUSING call this too — the
/// lily's stem glue and its socket — and they still do: a declaration
/// unlocks the declared rung, not the lanes past it.
pub fn flush_declarations<S: Scalar>(
    a: &Body<S>,
    b: &Body<S>,
    tol: Tol,
) -> pncad::topo::BooleanDeclarations {
    // The `expect` is the demo doctrine's shape, not an oversight, and
    // it asserts MORE since the detector's reach became the `Rest`
    // ladder's: a curved pair whose margins land in the band refuses
    // the whole query now, where before it was no candidate at all. A
    // tour scene builds its own contacts, so one that cannot decide
    // them is a scene to fix or a kernel to fix — never a refusal to
    // route around, and never a partial declaration set assembled
    // from the pairs that happened to decide.
    let found = pncad::topo::flush::find_flush_candidates(a, b, tol).expect(
        "the tour's contacts are authored exactly, so every cross-body pair — planar or \
         curved — decides definitely",
    );
    #[cfg(test)]
    census::record(a, b, &found);
    pncad::topo::flush::declare_all(&found)
}

/// The oracle: volume of a boolean result vs the exact expectation.
/// `Good` carries the whole [`BooleanBody`] (body + kind + the
/// declared contacts the 3′ gate consumes); the two failure shapes
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
    /// silent wrong-component defect class (extinct since the PR 5
    /// fix pass; the oracle stays armed anyway).
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
            let v = pncad::topo::mass_properties(&b.body, tol)
                .expect("mass properties")
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

/// **What this helper has declared, by CARRIER KIND** — a test-only
/// census, so the claim "every scene that declares through this helper
/// declares planar contacts, except the plant's socket" is MEASURED
/// rather than read off the renders, which can only say that nothing
/// moved and never what was declared.
///
/// It lives here because this is the tour's one detection site: a
/// per-scene assertion would rebuild each scene's operands and would
/// still miss the next consumer added.
#[cfg(test)]
pub(crate) mod census {
    use super::{Body, Scalar};
    use core::cell::RefCell;
    use pncad::geom_brep::SurfaceKind;
    use pncad::prelude::query;

    type KindPair = (Option<SurfaceKind>, Option<SurfaceKind>);

    thread_local! {
        static DECLARED: RefCell<Vec<KindPair>> = const { RefCell::new(Vec::new()) };
    }

    /// Records the carrier-kind pair of every finding the helper is
    /// about to declare.
    pub(crate) fn record<S: Scalar>(
        a: &Body<S>,
        b: &Body<S>,
        found: &[pncad::topo::flush::FacePairFinding],
    ) {
        DECLARED.with(|d| {
            d.borrow_mut().extend(found.iter().map(|f| {
                (
                    query::face_surface_kind(a, f.pair.0),
                    query::face_surface_kind(b, f.pair.1),
                )
            }));
        });
    }

    /// Takes and clears what this thread has recorded.
    pub(crate) fn drain() -> Vec<KindPair> {
        DECLARED.with(|d| core::mem::take(&mut *d.borrow_mut()))
    }
}

#[cfg(test)]
mod consumer_census {
    use super::*;
    use pncad::geom_brep::SurfaceKind;

    /// **Every consumer of [`flush_declarations`] declares PLANAR
    /// contacts, except the plant's socket** — the claim the flush
    /// detector's widening rests on, measured at the helper rather
    /// than inferred from renders being byte-identical.
    ///
    /// The scenes run for their effect on the census: the cross-lap's
    /// declared mate, the table's four corner-aligned legs and the
    /// letterforms' declared intersect. The plant's own four pairs —
    /// the stem glue, the leaf sheath, the socket and the flower weld
    /// — are measured in `lily`'s probe module instead, beside the
    /// walls they belong to, because running the whole plant here
    /// would rebuild it a second time in one suite for nothing.
    #[test]
    fn every_planar_consumer_of_the_helper_declares_planes_only() {
        let tol = Tol::witness();
        let _ = census::drain();
        let _ = crate::crosslap::build::<f64>(tol);
        let _ = crate::bool_bodies::table::<f64>(tol);
        let _ = crate::letterforms::build::<f64>(tol);
        let declared = census::drain();
        assert!(
            declared.len() >= 3,
            "three scenes declare through this helper; census {declared:?}"
        );
        for pair in &declared {
            assert_eq!(
                *pair,
                (Some(SurfaceKind::Plane), Some(SurfaceKind::Plane)),
                "these scenes' contacts are planar; a curved one here would be a scene \
                 change, not a detector change: {declared:?}"
            );
        }
    }
}
