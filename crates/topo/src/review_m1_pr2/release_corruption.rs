//! Adversarial e2e review artifact for M1 PR 2 (2026-07-16). These are
//! **independent derivations** — do not "simplify" them to match shipped
//! fixtures; the independence is the regression value. Promoted per
//! Evan's request (PR #17 thread).
//!
//! Tier-1-INVALID bodies fed into the operators. Contract: typed errors
//! or garbage bodies -- never a panic, never a hang. The debug-vs-release
//! expectations are split with `cfg(debug_assertions)` guards, so neither
//! profile alone covers this file and BOTH have to run it: the
//! garbage-success cases are meaningful only in --release (debug builds
//! fire the postcondition assert on them -- see the
//! debug_postcondition_fires_on_corrupt_input test, which documents that
//! tension).
//!
//! Both are gated. The debug rows ride the standard nextest matrix; the
//! release rows are the `corrupt input (release profile)` job in
//! `.github/workflows/ci.yml`, which is the only release-profile test
//! invocation the kernel workspace has and exists for this file.

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
    assert!(
        start.elapsed().as_secs() < 5,
        "bounded walk took too long: {:?}",
        start.elapsed()
    );
}

/// Foreign-parent-loop corruption that PASSES every precondition: the op
/// completes and returns Ok, producing a garbage body. Allowed by the
/// documented contract in RELEASE (no validity promise on corrupt
/// input); in DEBUG builds the postcondition assert fires instead --
/// which contradicts the module doc's claim that a firing postcondition
/// is "never an input failure". Kept release-only here.
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
