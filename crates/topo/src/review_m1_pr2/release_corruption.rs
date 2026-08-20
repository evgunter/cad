//! Adversarial e2e review artifact for M1 PR 2 (2026-07-16). These are
//! **independent derivations** — do not "simplify" them to match shipped
//! fixtures; the independence is the regression value. Promoted per
//! Evan's request (PR #17 thread).
//!
//! Tier-1-INVALID bodies fed into the operators. The debug-vs-release
//! expectations are split with `cfg(debug_assertions)` guards, so neither
//! profile alone covers this file and BOTH have to run it: the
//! garbage-success cases exist only in --release (debug builds fire the
//! postcondition assert on them -- see the
//! debug_postcondition_fires_on_corrupt_input test, which documents that
//! tension).
//!
//! Both are gated. The debug rows ride the standard nextest matrix; the
//! release rows are the `corrupt input (release profile)` job in
//! `.github/workflows/ci.yml`, which is the only release-profile test
//! invocation the kernel workspace has. That job greps this sentence, so
//! renaming it here or there is loud.
//!
//! # What of the contract is still ratified
//!
//! "Never a panic, never a hang" is: DESIGN.md's D9 footnote, as amended
//! by the **D2 addendum** (2026-08-19), keeps exactly the bounded-traversal
//! half -- "never a hang; every traversal is bounded".
//!
//! "Or garbage bodies" is NOT. The addendum's rule is **silent discard is
//! never an answer**, and it explicitly supersedes the footnote's original
//! "typed errors where cheaply detectable, or documented garbage-out in
//! release". The ~60 silent discards in `euler{,_ring,_kill}.rs` that
//! produce those garbage bodies are now a known deviation pending **W2c**,
//! not a ratified disposition. So
//! `foreign_parent_loop_garbage_in_garbage_out_release` below documents
//! what the kernel currently DOES, not what it is entitled to do, and W2c
//! is expected to change its meaning or retire it. Its independent value
//! until then is that it is the file's only
//! `#[cfg(not(debug_assertions))]` item, so it is the one thing here that
//! a debug-only CI could not even type-check.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::{Body, EulerOpError, MefSite, MevSite};
use geom_core::Point3;
// Only the release-profile garbage-out test validates; guard the import
// so debug builds stay warning-free.
#[cfg(not(debug_assertions))]
use crate::validate;

fn p(x: f64) -> Point3<f64> {
    Point3::new(x, 0.0, 0.0)
}

fn pillow() -> (
    Body<f64>,
    crate::MvfsCreated,
    crate::MevCreated,
    crate::MefCreated,
) {
    let mut body = Body::<f64>::new();
    let seed = body.mvfs(p(0.0)).unwrap();
    let seg = body
        .mev_line(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            p(1.0),
        )
        .unwrap();
    let split = body
        .mef_chord(MefSite::Chords {
            he1: seg.he_plus,
            he2: seg.he_minus,
        })
        .unwrap();
    (body, seed, seg, split)
}

/// Broken orbit (edge<->half bijection corrupted): typed error, no hang.
#[test]
fn broken_orbit_yields_typed_error() {
    let (mut body, _, seg, split) = pillow();
    body.get_edge_mut(seg.edge).unwrap().he_plus = split.he_plus;
    body.get_edge_mut(seg.edge).unwrap().he_minus = split.he_minus;
    let err = body
        .mev_line(
            MevSite::Fan {
                he1: seg.he_plus,
                he2: split.he_plus,
            },
            p(9.0),
        )
        .unwrap_err();
    assert!(matches!(err, EulerOpError::FanOrbitBroken { .. }));
}

/// Torn cycle (next crosses loops): typed error, no hang. The walk is
/// bounded even when the tear makes a long spurious path.
#[test]
fn torn_cycle_yields_typed_error() {
    let (mut body, _, seg, split) = pillow();
    body.get_half_edge_mut(seg.he_plus).unwrap().next = split.he_plus;
    let err = body
        .mef_chord(MefSite::Chords {
            he1: seg.he_plus,
            he2: split.he_minus,
        })
        .unwrap_err();
    assert!(matches!(err, EulerOpError::LoopCycleBroken { .. }));
}

/// A LARGE torn structure: 3000 struts then a tear -- the bounded walks
/// must terminate quickly (no O(inf) hang). Mirrors PR 1's torn-link
/// attack through the operator path.
///
/// Promotion note: in debug builds the shipped per-op postcondition
/// sweep makes the construction loop quadratic (minutes at n=3000), so
/// the strut count is scaled down there; the torn-walk timing assertion
/// under attack is identical in both profiles.
#[test]
fn large_torn_body_terminates_quickly() {
    let n: i32 = if cfg!(debug_assertions) { 500 } else { 3000 };
    let mut body = Body::<f64>::new();
    let seed = body.mvfs(p(0.0)).unwrap();
    let seg = body
        .mev_line(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            p(1.0),
        )
        .unwrap();
    let mut last = seg;
    for i in 0..n {
        last = body
            .mev_line(
                MevSite::Fan {
                    he1: seg.he_plus,
                    he2: seg.he_plus,
                },
                p(2.0 + f64::from(i)),
            )
            .unwrap();
    }
    // Tear: make the loop's next-chain skip around.
    let target = last.he_minus;
    body.get_half_edge_mut(seg.he_plus).unwrap().next = seg.he_plus;
    let start = std::time::Instant::now();
    let err = body
        .mef_chord(MefSite::Chords {
            he1: seg.he_plus,
            he2: target,
        })
        .unwrap_err();
    assert!(matches!(err, EulerOpError::LoopCycleBroken { .. }));
    let walked = start.elapsed();
    // The clause this row defends is D9's surviving one: every traversal
    // is bounded. MEASURED for the attacked call itself -- 1.5 us in
    // release (n = 3000) and 7.4 us in debug at opt-0 (n = 500), on a
    // 4-core container with four other lanes live, so not CI's 2-vCPU
    // box. 10 ms is therefore ~1400x the slower measurement: a
    // hang-and-blowup detector with room for scheduler jitter on a shared
    // runner, NOT a perf budget. What that catches: an unbounded walk,
    // and any growth beyond roughly quadratic at this n. What it does
    // NOT catch: a constant-factor regression. A bound tight enough for
    // that would sit within a few multiples of runner jitter, which
    // buys a flaky gate rather than a stricter one.
    assert!(
        walked < std::time::Duration::from_millis(10),
        "bounded walk took too long: {walked:?}"
    );
}

/// Foreign-parent-loop corruption that PASSES every precondition: the op
/// completes and returns Ok, producing a garbage body. This RECORDS
/// current behaviour rather than blessing it -- the D2 addendum retired
/// the garbage-out half of the contract and W2c converts the discards
/// that cause it (see the module docs). In DEBUG builds the postcondition
/// assert fires instead -- which contradicts the module doc's claim that
/// a firing postcondition is "never an input failure". Kept release-only
/// here, which also makes it the file's only item a debug-only CI cannot
/// compile.
#[test]
#[cfg(not(debug_assertions))]
fn foreign_parent_loop_garbage_in_garbage_out_release() {
    let (mut body, seed, seg, split) = pillow();
    // Both chord halves genuinely share a cycle, but claim the OTHER
    // loop as parent (consistently). Preconditions (same parent, parent
    // resolves, parent is a cycle, walk reaches he2) all pass.
    let foreign = seed.r#loop; // the old loop; the halves are in split's
    let he_a = seg.he_plus; // actually in split.r#loop after the mef
    let he_b = split.he_minus;
    assert_eq!(body.get_half_edge(he_a).unwrap().parent_loop, split.r#loop);
    body.get_half_edge_mut(he_a).unwrap().parent_loop = foreign;
    body.get_half_edge_mut(he_b).unwrap().parent_loop = foreign;
    // No panic, no hang; Ok(garbage) is within contract.
    let result = body.mef_chord(MefSite::Chords {
        he1: he_a,
        he2: he_b,
    });
    assert!(result.is_ok(), "preconditions cannot see this corruption");
    // The body is garbage now -- validate must say so (and not panic).
    assert!(validate(&body).is_err());
}

/// The same corruption in a DEBUG build: documents that the debug
/// postcondition DOES fire on tier-1-invalid input (panic), i.e. the
/// "one legitimate panic site / never an input failure" doc sentence
/// only holds under the tier-1-valid input assumption.
#[test]
#[cfg(debug_assertions)]
fn debug_postcondition_fires_on_corrupt_input() {
    let result = std::panic::catch_unwind(|| {
        let (mut body, seed, seg, split) = pillow();
        let foreign = seed.r#loop;
        body.get_half_edge_mut(seg.he_plus).unwrap().parent_loop = foreign;
        body.get_half_edge_mut(split.he_minus).unwrap().parent_loop = foreign;
        let _ = body.mef_chord(MefSite::Chords {
            he1: seg.he_plus,
            he2: split.he_minus,
        });
    });
    assert!(
        result.is_err(),
        "expected the debug postcondition to panic on corrupt input; if \
         this stops panicking, the doc claim becomes true and this test \
         should move to the release-style garbage-out assertion"
    );
}

/// Null-key mvfs spam plus interleaved failures on an empty body: the
/// ops never panic on a fresh body either.
#[test]
fn empty_body_error_paths() {
    let mut body = Body::<f64>::new();
    assert!(
        body.mev_line(
            MevSite::Lone {
                r#loop: crate::LoopKey::default()
            },
            p(0.0)
        )
        .is_err()
    );
    assert!(
        body.mef_chord(MefSite::Lone {
            r#loop: crate::LoopKey::default()
        })
        .is_err()
    );
    assert!(
        body.mev_line(
            MevSite::Fan {
                he1: crate::HalfEdgeKey::default(),
                he2: crate::HalfEdgeKey::default(),
            },
            p(0.0),
        )
        .is_err()
    );
    assert!(
        body.mef_chord(MefSite::Chords {
            he1: crate::HalfEdgeKey::default(),
            he2: crate::HalfEdgeKey::default(),
        })
        .is_err()
    );
    assert_eq!(body.vertices().count(), 0);
}
