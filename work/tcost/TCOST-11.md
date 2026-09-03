---
id: TCOST-11
kind: unit
title: The one-declaration guard: one home in test_utils, and pncad decided
status: dispatched
opened: 2026-09-03
branch: tcost/11-aggregation-guard-home
---


Candidate `one-declaration-guard-one-home-in-test-utils` (raised in
TCOST-B2's style review), taken up as a test-infrastructure unit: the
two assertions of `every_suite_file_is_aggregated`, byte-identical in
fourteen `crates/*/tests/all.rs`, get one home in `test_utils::source`
with each `all.rs` keeping one call; proved by mutation (an
un-aggregated suite file; a `mod foo;` in a suite file) that nothing
the guard catches stops being caught; `crates/pncad/tests/all.rs`
decided explicitly (the walk and the guard, or one sentence saying
why a single-file crate needs neither). Touches
`crates/test-utils/src/` only among `src/` trees. Brief:
`/home/user/tcost-work/briefs/tcost-11-brief.md`. Closes the
candidate at merge.
