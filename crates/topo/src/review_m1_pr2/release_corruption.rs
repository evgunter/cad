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
//! release". W2c executed that rule across `euler{,_ring,_kill}.rs`: no
//! mutation phase there discards a failed lookup any more, and neither
//! does the one write helper they share — `link_half_edges` announces,
//! on a precondition its callers each discharge by minting the key or
//! proving it live in the same call.
//!
//! **That did not retire the row below, and the reason is the row's whole
//! point.** `foreign_parent_loop_garbage_in_garbage_out_release` corrupts
//! `parent_loop` to a key that is *live but wrong*, so no lookup fails —
//! every write lands, on the wrong topology. The garbage it observes is
//! wrong data, never a swallowed failure, and it is the residue the
//! addendum leaves standing: corruption a plan phase cannot observe still
//! yields `Ok` plus a body the validator refuses. The row therefore still
//! documents what the kernel DOES rather than what it is entitled to do.
//! Its other value is that it is the file's only
//! `#[cfg(not(debug_assertions))]` item, so it is the one thing here that
//! a debug-only CI could not even type-check.
//!
//! # What this file does NOT cover, stated rather than left to be found
//!
//! The row-4 `unreachable!`s the Euler modules and `link_half_edges` now
//! carry fire on a key that fails to RESOLVE. Every corruption planted below is either
//! live-but-wrong (`parent_loop`, the edge<->half bijection) or a torn
//! `next` -- by construction none of them makes a lookup fail, which is
//! why the whole file still passes unchanged after the conversion. So
//! **no row here plants a dangling key into a mutation phase**, and in
//! release, where the postcondition is compiled out, those
//! `unreachable!`s are the only guard left.
//!
//! That gap is recorded rather than closed. Reaching a mutation phase
//! with a dangling key means defeating the plan phase that exists to
//! refuse exactly that: on today's operators every such key is either
//! minted in-phase or proven live, so a fixture would have to corrupt
//! the body BETWEEN the plan and the mutation -- which no in-crate
//! surface offers, the two phases being straight-line code inside one
//! `&mut self` call. A fixture that instead corrupts before the call
//! gets a typed `StaleKey`, which is the row above, not this one. The
//! honest statement is that these arms are unreachable by construction
//! and therefore also untestable by construction; the thing that WOULD
//! go red on a bad conversion is
//! `debug_postcondition_fires_on_corrupt_input`, which is why it now
//! asserts its panic's source instead of only that one occurred.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::{Body, EulerOpError, MefSite, MevSite};
use geom_core::Point3;
// Only the release-profile garbage-out test validates; guard the import
// so debug builds stay warning-free.
#[cfg(not(debug_assertions))]
use crate::validate;
use geom_core::Tol;

fn p(x: f64) -> Point3<f64> {
    Point3::new(x, 0.0, 0.0)
}

fn pillow(
    tol: Tol,
) -> (
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
            tol,
        )
        .unwrap();
    let split = body
        .mef_chord(
            MefSite::Chords {
                he1: seg.he_plus,
                he2: seg.he_minus,
            },
            tol,
        )
        .unwrap();
    (body, seed, seg, split)
}

/// Broken orbit (edge<->half bijection corrupted): typed error, no hang.
#[test]
fn broken_orbit_yields_typed_error() {
    let tol = Tol::witness();
    let (mut body, _, seg, split) = pillow(tol);
    body.get_edge_mut(seg.edge).unwrap().he_plus = split.he_plus;
    body.get_edge_mut(seg.edge).unwrap().he_minus = split.he_minus;
    let err = body
        .mev_line(
            MevSite::Fan {
                he1: seg.he_plus,
                he2: split.he_plus,
            },
            p(9.0),
            tol,
        )
        .unwrap_err();
    assert!(matches!(err, EulerOpError::FanOrbitBroken { .. }));
}

/// Torn cycle (next crosses loops): typed error, no hang. The walk is
/// bounded even when the tear makes a long spurious path.
#[test]
fn torn_cycle_yields_typed_error() {
    let tol = Tol::witness();
    let (mut body, _, seg, split) = pillow(tol);
    body.get_half_edge_mut(seg.he_plus).unwrap().next = split.he_plus;
    let err = body
        .mef_chord(
            MefSite::Chords {
                he1: seg.he_plus,
                he2: split.he_minus,
            },
            tol,
        )
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
    let tol = Tol::witness();
    let n: i32 = if cfg!(debug_assertions) { 500 } else { 3000 };
    let mut body = Body::<f64>::new();
    let seed = body.mvfs(p(0.0)).unwrap();
    let seg = body
        .mev_line(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            p(1.0),
            tol,
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
                tol,
            )
            .unwrap();
    }
    // Tear: make the loop's next-chain skip around.
    let target = last.he_minus;
    body.get_half_edge_mut(seg.he_plus).unwrap().next = seg.he_plus;
    let start = std::time::Instant::now();
    let err = body
        .mef_chord(
            MefSite::Chords {
                he1: seg.he_plus,
                he2: target,
            },
            tol,
        )
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
/// the garbage-out half of the contract. **No discard is involved**: the
/// planted `parent_loop` is live but wrong, so every lookup succeeds and
/// writes the wrong topology (see the module docs). In DEBUG builds the postcondition
/// assert fires instead -- which contradicts the module doc's claim that
/// a firing postcondition is "never an input failure". Kept release-only
/// here, which also makes it the file's only item a debug-only CI cannot
/// compile.
#[test]
#[cfg(not(debug_assertions))]
fn foreign_parent_loop_garbage_in_garbage_out_release() {
    let tol = Tol::witness();
    let (mut body, seed, seg, split) = pillow(tol);
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
    let result = body.mef_chord(
        MefSite::Chords {
            he1: he_a,
            he2: he_b,
        },
        tol,
    );
    assert!(result.is_ok(), "preconditions cannot see this corruption");
    // The body is garbage now -- validate must say so (and not panic).
    assert!(validate(&body).is_err());
}

/// The same corruption in a DEBUG build: documents that the debug
/// postcondition DOES fire on tier-1-invalid input (panic), i.e. the
/// "one legitimate panic site / never an input failure" doc sentence
/// only holds under the tier-1-valid input assumption.
///
/// **Why the panic's SOURCE is asserted and not just that one
/// occurred.** Since the D2 addendum landed in the Euler modules there
/// is a second panic source in this call path -- the row-4
/// `unreachable!`s the mutation phases now carry. A bare `is_err()`
/// passes identically whether the panic came from the postcondition
/// this row is named for or from a mis-converted `unreachable!`, and
/// the postcondition is the only debug-side signal that separates
/// them. Both `assert_euler_postcondition` messages carry the literal
/// asserted below; no `unreachable!` message does.
///
/// The message is captured through a panic HOOK rather than by
/// downcasting `catch_unwind`'s payload: `Any` downcasts are a second
/// bit channel and the `bit-identity punning` gate bans them outside
/// `geom_core::bit_identity`, test code included. The hook is
/// process-global, so it is restored immediately and the capture is
/// treated as advisory -- a concurrently panicking test in the same
/// process would show up as an empty or foreign message, which fails
/// loudly here rather than passing quietly.
#[test]
#[cfg(debug_assertions)]
fn debug_postcondition_fires_on_corrupt_input() {
    let tol = Tol::witness();
    let captured = std::sync::Arc::new(std::sync::Mutex::new(String::new()));
    let sink = std::sync::Arc::clone(&captured);
    let previous = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        if let Ok(mut slot) = sink.lock() {
            *slot = info.to_string();
        }
    }));
    let result = std::panic::catch_unwind(|| {
        let (mut body, seed, seg, split) = pillow(tol);
        let foreign = seed.r#loop;
        body.get_half_edge_mut(seg.he_plus).unwrap().parent_loop = foreign;
        body.get_half_edge_mut(split.he_minus).unwrap().parent_loop = foreign;
        let _ = body.mef_chord(
            MefSite::Chords {
                he1: seg.he_plus,
                he2: split.he_minus,
            },
            tol,
        );
    });
    std::panic::set_hook(previous);
    assert!(
        result.is_err(),
        "expected the debug postcondition to panic on corrupt input; if \
         this stops panicking, the doc claim becomes true and this test \
         should move to the release-style garbage-out assertion"
    );
    let message = captured.lock().map(|slot| slot.clone()).unwrap_or_default();
    assert!(
        message.contains("postcondition"),
        "panicked, but not from the postcondition -- a row-4 \
         `unreachable!` fired instead, which means a conversion's \
         not-input-reachable claim is false: {message}"
    );
}

/// Null-key mvfs spam plus interleaved failures on an empty body: the
/// ops never panic on a fresh body either.
#[test]
fn empty_body_error_paths() {
    let tol = Tol::witness();
    let mut body = Body::<f64>::new();
    assert!(
        body.mev_line(
            MevSite::Lone {
                r#loop: crate::LoopKey::default()
            },
            p(0.0),
            tol,
        )
        .is_err()
    );
    assert!(
        body.mef_chord(
            MefSite::Lone {
                r#loop: crate::LoopKey::default()
            },
            tol
        )
        .is_err()
    );
    assert!(
        body.mev_line(
            MevSite::Fan {
                he1: crate::HalfEdgeKey::default(),
                he2: crate::HalfEdgeKey::default(),
            },
            p(0.0),
            tol,
        )
        .is_err()
    );
    assert!(
        body.mef_chord(
            MefSite::Chords {
                he1: crate::HalfEdgeKey::default(),
                he2: crate::HalfEdgeKey::default(),
            },
            tol
        )
        .is_err()
    );
    assert_eq!(body.vertices().count(), 0);
}
