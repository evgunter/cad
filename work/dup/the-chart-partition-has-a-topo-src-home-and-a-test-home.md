---
id: the-chart-partition-has-a-topo-src-home-and-a-test-home
kind: issue
title: a body's faces grouped by surface key are spelled in topo's shell.rs, in topo's offset_together tests, and in sweep's tests/common
status: open
opened: 2026-09-26
priority: P4
cost: D
---



## Finding

- **Where**: `crates/topo/src/shell.rs` (`chart_groups` over
  `group_by_chart`, private), `crates/topo/src/offset_together.rs`'s
  in-crate `scope_walks::moves_of`, and
  `crates/sweep/tests/common/charts.rs` (`charts`, `charts_of`).
- **Confidence**: sure; all three read at the merge base of the batch-6
  lane that homed the third.
- **Raised by**: the batch-6 lane that closed
  `a-solids-charts-as-a-move-set-is-spelled-per-sweep-test-file`, which
  answered that row's first question — *is either of `shell.rs`'s two
  move-set builders a door the suites could call?* — with **no**: both
  build over the private `chart_groups` / `group_by_chart`, and no
  public door answers "this body's faces, grouped by the surface they
  wear".

So the partition every caller of `offset_charts_together` and
`offset_planes_together` has to build before it can call either door is
spelled three times: once in production (`shell.rs`), once in `topo`'s
own tests (`moves_of`, which cannot reach `sweep`'s test tree), and
once in `sweep/tests/common` (which now serves every `sweep` suite).
Each spells the same order — groups by first appearance in the face
arena, each group in arena order.

## What a taker owes

A decision first: whether "a body's charts" is a public `topo` door —
the partition the offset doors' `ChartMove` contract is stated over, so
a user driving those doors needs it as much as the suites do — or stays
the verb's private step. If it is a door, `shell.rs`,
`offset_together`'s tests and `sweep/tests/common/charts.rs` all fold
onto it (the last keeping only the `ChartMove` builders). If it is not,
this row closes with that ruling and the two test spellings stay.

**Why P4 and cost D**: the two test-side spellings are P4 on their own
and the production one has no second production twin; the door
question is a public-API decision on `topo`, which is what makes it D.
