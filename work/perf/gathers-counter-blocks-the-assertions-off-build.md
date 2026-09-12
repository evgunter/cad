---
id: gathers-counter-blocks-the-assertions-off-build
kind: issue
title: gathers_on_this_thread is cfg(debug_assertions) and a test calls it unguarded, so no build can turn debug assertions off
status: closed
opened: 2026-09-10
closed: 2026-09-11
pr: 2328
---

## The finding

`editor_core::gathers_on_this_thread` is `#[cfg(debug_assertions)]`
(`crates/editor-core/src/product.rs:375`, re-exported at
`src/lib.rs:171`) and `crates/editor-core/tests/docm5_subject.rs` calls
it at twelve sites with no matching guard, so
`CARGO_PROFILE_TEST_DEBUG_ASSERTIONS=false` fails with 12 × E0425.
Nobody can build the suite with assertions off today — the PERF
developer lane had to un-gate the counter to take its measurement.

## What a fix is

Guard the twelve call sites the way the counter is guarded, or make the
counter compile in every profile and only *count* under
`debug_assertions`. Zero runtime saved; it restores a knob the
`d1-per-op-tier1-sweep-price` ruling and any future measurement need.
Test-only plus one `cfg`; no A/B row. DOCM territory
(`docm5_subject.rs`), announced.
