//! **Instrumentation only** — a thread-local stage timer for the PERF
//! exploration lane (`perf/explore-kernel`). Not part of any kernel
//! contract, never merged to main, never read by a decision: it only
//! accumulates wall-clock nanoseconds per named stage so an
//! exploration harness can attribute a document's evaluation time.
//!
//! Disabled by default (`enable(true)` to arm), so an armed run is the
//! only one that pays anything beyond one relaxed load per span.

use std::cell::{Cell, RefCell};
use std::time::Instant;

thread_local! {
    static ACC: RefCell<Vec<(&'static str, u128, u64)>> = const { RefCell::new(Vec::new()) };
    static ON: Cell<bool> = const { Cell::new(false) };
    static TRACE: RefCell<Vec<(&'static str, u128)>> = const { RefCell::new(Vec::new()) };
    static TRACING: Cell<bool> = const { Cell::new(false) };
}

/// Arms or disarms the per-call trace (every span, in call order).
pub fn enable_trace(on: bool) {
    TRACING.with(|f| f.set(on));
}

/// Drains the per-call trace.
#[must_use]
pub fn take_trace() -> Vec<(&'static str, u128)> {
    TRACE.with(|t| std::mem::take(&mut *t.borrow_mut()))
}

/// Arms or disarms the probe on the current thread.
pub fn enable(on: bool) {
    ON.with(|f| f.set(on));
}

/// Whether the probe is armed on the current thread.
#[must_use]
pub fn armed() -> bool {
    ON.with(Cell::get)
}

/// Adds `nanos` to the named stage's accumulator (count += 1).
pub fn add(name: &'static str, nanos: u128) {
    if !armed() {
        return;
    }
    if TRACING.with(Cell::get) {
        TRACE.with(|t| t.borrow_mut().push((name, nanos)));
    }
    ACC.with(|acc| {
        let mut acc = acc.borrow_mut();
        if let Some(row) = acc.iter_mut().find(|row| row.0 == name) {
            row.1 += nanos;
            row.2 += 1;
        } else {
            acc.push((name, nanos, 1));
        }
    });
}

/// Drains the accumulator, returning `(stage, total_nanos, calls)`.
#[must_use]
pub fn take() -> Vec<(&'static str, u128, u64)> {
    ACC.with(|acc| std::mem::take(&mut *acc.borrow_mut()))
}

/// Clears the accumulator without reporting.
pub fn reset() {
    ACC.with(|acc| acc.borrow_mut().clear());
}

/// An open span; the elapsed time lands on `name` when it drops.
pub struct Span {
    name: &'static str,
    t0: Option<Instant>,
}

impl Span {
    /// Opens a span for `name` (a no-op while disarmed).
    #[must_use]
    pub fn start(name: &'static str) -> Self {
        Self {
            name,
            t0: armed().then(Instant::now),
        }
    }
}

impl Drop for Span {
    fn drop(&mut self) {
        if let Some(t0) = self.t0 {
            add(self.name, t0.elapsed().as_nanos());
        }
    }
}
