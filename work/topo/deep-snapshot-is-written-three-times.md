---
id: deep-snapshot-is-written-three-times
kind: issue
title: topo's tests write the deep body snapshot three times: fixtures::deep_snapshot, review_m1_pr2's deep_snapshot and review_m1_pr3's snapshot
status: closed
opened: 2026-09-29
priority: P3
cost: E
pr: 3557
branch: topo/one-deep-snapshot
closed: 2026-09-30
---


## What

Found by `mev-fan-plan-trusts-the-orbits-start-vertices`'s fix pass,
which folded the crate's four "fails with exactly this error and
leaves the body untouched" helpers into one,
`fixtures::assert_err_deep_unchanged`. The snapshot that helper
compares is still written three times, each walking all ten arenas
and the topology entities' provenance, differing only in line format
and return type:

- `fixtures::deep_snapshot` (`crates/topo/src/fixtures.rs`), a
  `Vec<String>`.
- `review_m1_pr2::deep_snapshot` (`crates/topo/src/review_m1_pr2/mod.rs`),
  one `String`, used across that module's files.
- `review_m1_pr3::snapshot` (`crates/topo/src/review_m1_pr3.rs`), a
  `Vec<String>`, still used by that file's direct before/after
  comparisons.

A field added to an arena's `Debug` form reaches all three, but an
arena or provenance record added to the body has to be added three
times, and a copy that misses it passes silently.

## The shape to give

One snapshot, `fixtures::deep_snapshot`; the two review modules call
it. The integration tests under `crates/topo/tests/` cannot see
`fixtures` (it is `cfg(test)` in the lib) and are out of scope.
