//! Adversarial e2e review artifact for M1 PR 2 (2026-07-16).
//!
//! Tier-1-INVALID bodies fed into the operators. The debug-vs-release
//! expectations are split with `cfg(debug_assertions)` guards, so neither
//! profile alone covers this file and BOTH have to run it: the
//! garbage-success cases exist only in --release (debug builds fire the
//! postcondition assert on them -- see the
//! debug_postcondition_fires_on_corrupt_input test, which documents that
//! tension).
//!
//! Both are gated, on every code-tier run. The debug rows ride the standard
//! nextest matrix; the release rows are the
//! `corrupt input (release profile)` job in `.github/workflows/ci.yml`,
//! which is the only release-profile test invocation the kernel workspace
//! has.
//!
//! **That job is the only lane in the tree that compiles the
//! `cfg(not(debug_assertions))` rows.** It pins
//! `CARGO_PROFILE_RELEASE_DEBUG_ASSERTIONS: "false"`; any other
//! `--release` run, against the root `[profile.release]`'s
//! `debug-assertions = true`, compiles the DEBUG arms of this file.
//!
//! # The contract under attack
//!
//! Every traversal is bounded (D9): a torn body never hangs an operator.
//! A torn link the plan phase reads -- a walk that does not close, a
//! record naming a key that does not resolve -- is a kernel bug (D2 row
//! 4), so the operator panics naming the record and the premise, before
//! writing anything. Only a key the caller passed is a typed refusal
//! (`EulerOpError::Argument`).
//!
//! **Corruption a plan phase cannot observe still yields `Ok`.**
//! `foreign_parent_loop_garbage_in_garbage_out_release` corrupts
//! `parent_loop` to a key that is *live but wrong*, so no lookup fails
//! and no walk breaks: every write lands, on the wrong topology, and the
//! validator refuses the result. The row documents what the kernel DOES
//! rather than what it is entitled to do. It is also the file's only
//! `#[cfg(not(debug_assertions))]` item, the one thing here a debug-only
//! CI could not even type-check.
//!
//! # What this file does NOT cover
//!
//! No row plants a dangling key into a MUTATION phase. Every key a
//! mutation phase writes through is minted there or proven live by the
//! plan, so a fixture would have to corrupt the body BETWEEN the two
//! phases -- straight-line code inside one `&mut self` call, which no
//! in-crate surface interrupts. A fixture that corrupts before the call
//! meets the plan phase's panic (the torn rows here). Those mutation-phase
//! arms are unreachable and untestable by construction; what goes red on
//! a mis-stated premise is `debug_postcondition_fires_on_corrupt_input`,
//! which asserts its panic's source.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use super::panic_message;
use crate::fixtures::deep_snapshot;
use crate::{BadArgument, Body, EntityId, EulerOpError, MefSite, MevSite};
use geom_core::Point3;
use std::panic::AssertUnwindSafe;
// Only the release-profile garbage-out test validates; guard the import
// so debug builds stay warning-free.
#[cfg(not(debug_assertions))]
use crate::validate;
use geom_core::Tol;

/// The premise a walk that does not close names.
const WALK: &str = crate::body::WALKS_CLOSE;

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
    let seed = body.mvfs(p(0.0), true).unwrap();
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

/// Broken orbit (edge<->half bijection corrupted): the orbit walk
/// panics naming the hop, no hang.
#[test]
fn broken_orbit_panics_naming_the_walk() {
    let tol = Tol::witness();
    let (mut body, _, seg, split) = pillow(tol);
    body.get_edge_mut(seg.edge).unwrap().he_plus = split.he_plus;
    body.get_edge_mut(seg.edge).unwrap().he_minus = split.he_minus;
    let message = panic_message(AssertUnwindSafe(|| {
        let _ = body.mev_line(
            MevSite::Fan {
                he1: seg.he_plus,
                he2: split.he_plus,
            },
            p(9.0),
            tol,
        );
    }));
    assert!(
        message.contains("the orbit walk from") && message.contains(WALK),
        "{message}"
    );
}

/// Torn cycle (next crosses loops): the loop walk panics naming the
/// hop, no hang. The walk is bounded even when the tear makes a long
/// spurious path.
#[test]
fn torn_cycle_panics_naming_the_walk() {
    let tol = Tol::witness();
    let (mut body, _, seg, split) = pillow(tol);
    body.get_half_edge_mut(seg.he_plus).unwrap().next = split.he_plus;
    let message = panic_message(AssertUnwindSafe(|| {
        let _ = body.mef_chord(
            MefSite::Chords {
                he1: seg.he_plus,
                he2: split.he_minus,
            },
            tol,
        );
    }));
    assert!(
        message.contains("the loop walk from") && message.contains(WALK),
        "{message}"
    );
}

/// A LARGE torn structure: 3000 struts then a tear -- the bounded walks
/// must terminate quickly (no O(inf) hang). Mirrors PR 1's torn-link
/// attack through the operator path.
///
/// Promotion note: this loop calls `mev` DIRECTLY, so every call is
/// its own door and sweeps the whole body (`topo::surgery`) — which
/// makes the construction quadratic in debug builds (minutes at
/// n=3000), so the strut count is scaled down there. A composing door
/// would pay one sweep for the whole loop; a consumer's own loop
/// cannot, because nothing has undertaken to check the body later.
/// The torn-walk timing assertion under attack is identical in both
/// profiles.
#[test]
fn large_torn_body_terminates_quickly() {
    let tol = Tol::witness();
    let n: i32 = if cfg!(debug_assertions) { 500 } else { 3000 };
    let mut body = Body::<f64>::new();
    let seed = body.mvfs(p(0.0), true).unwrap();
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
    // Tear: seg.he_plus's next closes on itself, cutting the rest of
    // the loop off its walk.
    let target = last.he_minus;
    body.get_half_edge_mut(seg.he_plus).unwrap().next = seg.he_plus;
    // Started inside the capture, so waiting on the panic-hook lock is
    // not timed.
    let start = std::cell::Cell::new(None);
    let message = panic_message(AssertUnwindSafe(|| {
        start.set(Some(std::time::Instant::now()));
        let _ = body.mef_chord(
            MefSite::Chords {
                he1: seg.he_plus,
                he2: target,
            },
            tol,
        );
    }));
    let walked = start.get().expect("the capture ran the call").elapsed();
    assert!(
        message.contains("the cycle walk from")
            && message.contains("never reaches it: on a tier-1-valid body a loop's next cycle"),
        "{message}"
    );
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
/// The panic's SOURCE is asserted, not only that one occurred: the
/// plan phases' row-4 `unreachable!`s are a second panic source on this
/// path, and a bare `is_err()` passes identically whether the
/// postcondition fired or a premise the planted corruption does not
/// break was mis-stated. Both `assert_euler_postcondition` messages
/// carry the literal asserted below; no row-4 message does.
#[test]
#[cfg(debug_assertions)]
fn debug_postcondition_fires_on_corrupt_input() {
    let tol = Tol::witness();
    let message = panic_message(|| {
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
    assert!(
        message.contains("postcondition"),
        "panicked, but not from the postcondition -- a row-4 \
         `unreachable!` fired instead, which means a conversion's \
         not-input-reachable claim is false: {message}"
    );
}

/// Null-key calls on an empty body: each refuses with the caller's
/// stale argument, naming its role, and leaves the body as it was, key
/// slots included.
#[test]
fn empty_body_error_paths() {
    let tol = Tol::witness();
    let mut body = Body::<f64>::new();
    type Refusal = Box<dyn Fn(&mut Body<f64>) -> Option<EulerOpError>>;
    let refusals: [(&str, EulerOpError, Refusal); 4] = [
        (
            "mev lone",
            stale("loop", EntityId::Loop(crate::LoopKey::default())),
            Box::new(move |b| {
                b.mev_line(
                    MevSite::Lone {
                        r#loop: crate::LoopKey::default(),
                    },
                    p(0.0),
                    tol,
                )
                .err()
            }),
        ),
        (
            "mef lone",
            stale("loop", EntityId::Loop(crate::LoopKey::default())),
            Box::new(move |b| {
                b.mef_chord(
                    MefSite::Lone {
                        r#loop: crate::LoopKey::default(),
                    },
                    tol,
                )
                .err()
            }),
        ),
        (
            "mev fan",
            stale("he1", EntityId::HalfEdge(crate::HalfEdgeKey::default())),
            Box::new(move |b| {
                b.mev_line(
                    MevSite::Fan {
                        he1: crate::HalfEdgeKey::default(),
                        he2: crate::HalfEdgeKey::default(),
                    },
                    p(0.0),
                    tol,
                )
                .err()
            }),
        ),
        (
            "mef chords",
            stale("he1", EntityId::HalfEdge(crate::HalfEdgeKey::default())),
            Box::new(move |b| {
                b.mef_chord(
                    MefSite::Chords {
                        he1: crate::HalfEdgeKey::default(),
                        he2: crate::HalfEdgeKey::default(),
                    },
                    tol,
                )
                .err()
            }),
        ),
    ];
    for (call, expected, refusal) in &refusals {
        let before = deep_snapshot(&body);
        assert_eq!(refusal(&mut body).as_ref(), Some(expected), "{call}");
        assert_eq!(deep_snapshot(&body), before, "{call}: body changed on Err");
    }
}

fn stale(role: &'static str, key: EntityId) -> EulerOpError {
    EulerOpError::Argument(BadArgument::Stale { role, key })
}
