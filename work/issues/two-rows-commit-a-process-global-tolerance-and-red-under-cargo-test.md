---
id: two-rows-commit-a-process-global-tolerance-and-red-under-cargo-test
kind: issue
title: Two rows commit a process-global tolerance, so they red under cargo test and pass only under nextest
status: open
opened: 2026-10-07
priority: P4
cost: E
---


## What

Found by the review of PR 4240 (`REVIEW.md` NOTE-5, branch
`join/tier3-pinch-checks-review`). It is unrelated to that PR. Two
rows commit a tolerance of their own to the process-global commit.
Under plain `cargo test`, where many tests share one process, they
panic `AlreadyInitialized` once another test has committed:

- `topo` `sphere_twin_rows_interval::a_turned_half_cap…`
  (`crates/topo/tests/sphere_twin_rows_interval.rs`);
- `editor-core` `msolve8_levered_clash::c4_band_refuses_every_mate_…_{empty,overflow}`
  (`crates/editor-core/tests/msolve8_levered_clash.rs`; also noted in
  DECIDE-5's closed record).

Each passes alone, and CI's nextest forks per test, so the gate never
sees it. It bites only a local `cargo test` run.

Two owners (TOPO's tests, and TCOST/TINT's `msolve8` row), so it is filed
here rather than on either slate.

## The shape to give

Either re-exec such a row in its own process (the #448 self-re-exec
probe pattern), or mark it nextest-only by name, the way other
process-global rows are.
