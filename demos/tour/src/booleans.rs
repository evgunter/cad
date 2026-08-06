//! The tour's ONLY boolean call sites — a deliberately thin,
//! centralized wrapper over `topo::{union, subtract, intersect}` so
//! any API shift is a one-file adaptation here — plus the shared
//! exact-volume oracle every boolean scene runs its results through.
//!
//! Generic over [`Scalar`] (M4 PR 8b): the same wrappers serve the
//! f64 tour and the Probe K-telemetry sweep; oracles compare through
//! the exact `f()` extraction.
//!
//! Tier-3 posture: RETIRED as a workaround. Boolean results validate
//! as they are, via `validate_pseudomanifold` with the op's own
//! declared `contacts` (M3 PR 6a's 3′ contract) — see
//! `crate::run_body`. The `upgrade_edges_to_intersections` clone hack
//! that used to live here (a PR 3-era description-gap workaround) is
//! deleted; the kernel caught up.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use topo::{Body, BooleanBody, BooleanError, BooleanResult, BooleanResultKind};

use crate::scalar::Scalar;

/// A ∪* B — refusals surface to the caller; every result goes through
/// the scene builders' exact-volume oracle before it ships.
pub fn try_union<S: Scalar>(a: &Body<S>, b: &Body<S>) -> Result<BooleanResult<S>, BooleanError> {
    topo::union(a, b)
}

/// A ∖* B (same posture as [`try_union`]).
pub fn try_subtract<S: Scalar>(a: &Body<S>, b: &Body<S>) -> Result<BooleanResult<S>, BooleanError> {
    topo::subtract(a, b)
}

/// A ∩* B (same posture as [`try_union`]).
pub fn try_intersect<S: Scalar>(
    a: &Body<S>,
    b: &Body<S>,
) -> Result<BooleanResult<S>, BooleanError> {
    topo::intersect(a, b)
}

/// A ∪* B with the scene's flush contacts DECLARED (M4 PR 5: the
/// author's coincidence intent, stated — the kernel never infers it
/// from values).
pub fn try_union_declared<S: Scalar>(
    a: &Body<S>,
    b: &Body<S>,
) -> Result<BooleanResult<S>, BooleanError> {
    topo::union_with(a, b, &flush_declarations(a, b))
}

/// A ∩* B with the scene's flush contacts declared
/// ([`try_union_declared`]).
pub fn try_intersect_declared<S: Scalar>(
    a: &Body<S>,
    b: &Body<S>,
) -> Result<BooleanResult<S>, BooleanError> {
    topo::intersect_with(a, b, &flush_declarations(a, b))
}

/// Demo-authoring convenience (M4 PR 5, same shape as the kernel test
/// suites'): the [`topo::BooleanDeclarations`] declaring every
/// geometrically-plausible cross-operand flush-plane face pair — the
/// scene author BUILT the contact deliberately; this writes the
/// intent down. Certification still happens inside the op through
/// the verified declared rung.
pub fn flush_declarations<S: Scalar>(a: &Body<S>, b: &Body<S>) -> topo::BooleanDeclarations {
    use geom_core::k_stats::{decide, decide_flagged};
    use geom_core::{Length, Sign};
    let band = geom_core::Band::linear().unwrap();
    let planes =
        |body: &Body<S>| -> Vec<(topo::FaceKey, geom_core::Point3<S>, geom_core::Vec3<S>)> {
            body.faces()
                .filter_map(|(k, f)| match body.get_surface(f.surface) {
                    Some(&geom_surfaces::Surface::Plane { origin, normal, .. }) => {
                        Some((k, origin, normal))
                    }
                    _ => None,
                })
                .collect()
        };
    let mut decls = topo::BooleanDeclarations::none();
    for &(fa, oa, na) in &planes(a) {
        for &(fb, ob, nb) in &planes(b) {
            if matches!(
                decide_flagged(
                    "demo_flush_parallel",
                    na.cross(nb).norm(),
                    band,
                    "demo fixture: bare sine gate (the topo test-common declarer's twin)",
                ),
                Ok(Sign::Positive)
            ) {
                continue;
            }
            let sigma = match decide_flagged(
                "demo_flush_orient",
                na.dot(nb),
                band,
                "demo fixture: bare cosine gate (the topo test-common declarer's twin)",
            ) {
                Ok(Sign::Positive) => S::from_f64(1.0),
                Ok(Sign::Negative) => S::from_f64(-1.0),
                _ => continue,
            };
            let da = na.dot(oa - geom_core::Point3::origin());
            let db = nb.dot(ob - geom_core::Point3::origin());
            if matches!(
                decide("demo_flush_offset", Length::of(da - sigma * db), band),
                Ok(Sign::Zero)
            ) {
                decls.coincident_faces.push((fa, fb));
            }
        }
    }
    decls
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

pub fn check<S: Scalar>(r: Result<BooleanResult<S>, BooleanError>, expected: f64) -> Verdict<S> {
    match r {
        Ok(BooleanResult::Body(b)) => {
            let v = topo::mass_properties(&b.body)
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
