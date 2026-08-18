//! **The per-triangle certificate falsifier's accumulator** (issue
//! #558): the review probe that resamples every emitted NURBS triangle
//! and checks the measured deviation `|S - Pi|` against that triangle's
//! own certificate, in place, at 12 barycentric samples per edge.
//!
//! It is a FALSIFIER, not a meter. `crate::budget` asks how much of the
//! deviation budget a face spent; this asks whether the certificate was
//! ever wrong. The two are separate instruments with separate consumers
//! (`tools/tess-lint` reads the meter's CSV; this one's only consumer
//! is `tests/probe_review.rs`), which is why they have separate
//! features — see the manifest.
//!
//! # The gate, and why it is here
//!
//! Gated at the MODULE boundary, deliberately not at the call sites:
//! `crate::trimmed` calls `armed` and `record` unconditionally, and
//! with the feature off those resolve to the `inert` stubs — `armed()`
//! is a `const fn` returning `false`, so the whole per-triangle
//! resampling block folds away and NOTHING of this module reaches a
//! shipped build. Zero `#[cfg]` in the tessellation lane means there is
//! no second version of that lane to keep in step.
//!
//! `arm` and `take` exist ONLY in the live half, on purpose: a
//! caller able to arm a falsifier that records nothing is a fail-quiet,
//! so with the feature off the driving suite does not compile rather
//! than reporting zero samples. `crate::budget`'s posture, same reason.
//!
//! # What the gate closes (the measured exposure)
//!
//! `armed` used to also answer true for a `NURBS_PROBE` environment
//! variable — the reviewer's original manual drive, kept as a
//! convenience after M8-5's MIN-1 made the suite self-arming. That was
//! a BACK CHANNEL INTO SHIPPED CODE and #562 removed it:
//!
//! * it was ambient and process-wide, so a variable in a deployment
//!   environment silently switched on a 91-sample resampling of every
//!   emitted triangle — measured at 7.9 s → 19.8 s on the demo tour's
//!   release binary, same binary, same arguments, ~71M extra surface
//!   evaluations;
//! * and the sampling block asserts, so it also converted
//!   [`crate::tessellate()`]'s typed [`TessellateError`] contract into
//!   a PANIC — chosen by the environment rather than by the caller.
//!
//! #562 closed the AMBIENT half. It left the COMPILED-IN half: the
//! module was still unconditionally compiled and still `pub` (reachable
//! as `pncad::mesh::probe_stats`), so the sampling block, its `assert!`,
//! and the public arming API all shipped, reachable by any caller
//! willing to call `arm(true)`. This module boundary is that half's
//! close. The standing rule both halves come from: instrumentation is
//! feature-gated at its MODULE boundary and armed by an explicit call,
//! never by the environment (the `discipline` job's "no ambient
//! environment in the kernel" grep enforces the second half).
//!
//! # Thread-local, and why
//!
//! R1 of the M8-5 PR (MINOR-2): tessellation runs entirely on the
//! calling thread, so arming and stats scoped to the arming thread make
//! the armed evidence rows reproducible under a parallel test runner —
//! a concurrent test's tessellations can no longer contaminate the
//! driving test's `take`. Soundness never depended on this (every
//! recorded sample is also assert-checked in place in `crate::trimmed`);
//! this is about the EVIDENCE being attributable.
//!
//! [`TessellateError`]: crate::types::TessellateError

/// **The armed half — compiled only under the `probe-stats` feature.**
///
/// Holds the thread-local accumulator and the arming switch. Nothing
/// here runs in a normal tessellation even when it IS compiled: every
/// recording site in `crate::trimmed` sits behind an [`armed`] check
/// that is hoisted once per face.
///
/// [`armed`]: live::armed
#[cfg(feature = "probe-stats")]
mod live {
    use std::cell::Cell;

    thread_local! {
        /// (worst measured deviation, its cert, max ratio d/cert,
        /// count) — this thread's accumulator.
        static STATS: Cell<(f64, f64, f64, u64)> = const { Cell::new((0.0, 0.0, 0.0, 0)) };
        /// Is the falsifier armed on this thread?
        static ARMED: Cell<bool> = const { Cell::new(false) };
    }

    /// Arm/disarm the falsifier for tessellations on THIS thread.
    ///
    /// INVARIANT: explicit call only. There is no ambient channel into
    /// this switch and there must never be one again (#562, and the
    /// `discipline` job's "no ambient environment in the kernel" grep).
    pub fn arm(on: bool) {
        ARMED.with(|a| a.set(on));
    }

    /// Is the falsifier armed? **Explicit [`arm`] only.**
    pub fn armed() -> bool {
        ARMED.with(Cell::get)
    }

    /// Record one sample: measured deviation `d` against its triangle's
    /// certificate `cert`.
    pub fn record(d: f64, cert: f64) {
        STATS.with(|c| {
            let mut s = c.get();
            if d > s.0 {
                s.0 = d;
                s.1 = cert;
            }
            if cert > 0.0 {
                let r = d / cert;
                if r > s.2 {
                    s.2 = r;
                }
            }
            s.3 += 1;
            c.set(s);
        });
    }

    /// Take-and-reset this thread's accumulated stats.
    pub fn take() -> (f64, f64, f64, u64) {
        STATS.with(|c| c.replace((0.0, 0.0, 0.0, 0)))
    }
}

/// **The disarmed half — compiled when `probe-stats` is off**, which is
/// every shipped build and every default `cargo test`.
///
/// [`armed`][inert::armed] is a `const fn` answering `false`, so `crate::trimmed`'s
/// per-triangle resampling block — and the `assert!` inside it — are
/// deleted from the build outright: the falsifier costs nothing
/// STRUCTURALLY rather than by argument. [`record`][inert::record] keeps its signature
/// so the call site is shared with the live half; `arm` and `take` are
/// absent so nothing can pretend to drive an instrument that is not
/// here.
#[cfg(not(feature = "probe-stats"))]
mod inert {
    /// Always false: the falsifier is not in this build.
    ///
    /// INVARIANT: `const fn`, so the constant is visible to the
    /// compiler at every call site rather than left to inlining — the
    /// static assertion below is that invariant checked by the build
    /// itself, and it is what removes the trimmed lane's per-triangle
    /// resampling block and its `assert!` from a shipped build.
    pub const fn armed() -> bool {
        false
    }

    /// The gate, proved at COMPILE TIME: this item fails to build if
    /// the inert half's [`armed`] ever stops folding to `false`.
    const _: () = assert!(!armed());

    /// No-op. Unreachable in this build (its one call site is behind
    /// [`armed`]); it exists so the lane has one shared call site
    /// instead of a `#[cfg]`.
    pub fn record(_d: f64, _cert: f64) {}
}

#[cfg(not(feature = "probe-stats"))]
pub use inert::*;
#[cfg(feature = "probe-stats")]
pub use live::*;
