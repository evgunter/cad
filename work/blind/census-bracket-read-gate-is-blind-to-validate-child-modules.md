---
id: census-bracket-read-gate-is-blind-to-validate-child-modules
kind: issue
title: The bracket-read census and real.rs's allowlist prose count validate.rs alone; a production bracket read in a validate/ child file would pass both
status: open
opened: 2026-10-20
priority: P3
cost: E
---


## What

`crates/topo/tests/r1_lane1_bracket_read_census.rs` counts bracket reads (`.lo()` / `.hi()`) in `crates/topo/src/validate.rs` against the `Bounds` scope rule's disclosure. The `real.rs` allowlist prose ("ONE `lo` call appears in `validate.rs`") is scoped to that one file too.

PR 5492 created the first child module, `crates/topo/src/validate/gap_rows.rs`, a `#[cfg(test)]` oracle. That oracle is honestly outside the rule's intent. But a future production bracket read in any `validate/*.rs` child would pass both gates silently. (Found by the FULL reviewer of PR 5492.)

## Fix

Count the `validate` module tree, with `#[cfg(test)]` children excepted, or have the census refuse to run when `validate/` holds a non-test child it doesn't count. Update the `real.rs` prose to match.

## Moved to BLIND (2026-10-26, GUARD's split)

GUARD measured 43 points against 30 and split three ways. This row is a
spelling a kernel-discipline gate's matcher misses (or refuses wrongly),
which is BLIND's charter (`work/blind/plan.md`). Id and body unchanged.
