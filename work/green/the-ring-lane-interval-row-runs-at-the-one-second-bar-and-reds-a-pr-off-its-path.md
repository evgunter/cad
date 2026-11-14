---
id: the-ring-lane-interval-row-runs-at-the-one-second-bar-and-reds-a-pr-off-its-path
kind: issue
title: sweep's verbs_germarms_interval ring-lane row runs at the 1 s fast-set bar and reds a PR that does not reach it
status: open
opened: 2026-11-14
---


Filed by the onedoor revert-door lane on PR 6935.

## What

`sweep::all verbs_germarms_interval::the_ring_lane_builds_at_the_certified_scalar`
(`crates/sweep/tests/verbs_germarms_interval.rs`) ran 1.024 s in the
default-eps step of `test (kernel)` on PR 6935 (run 43555352157, head
`7e88903d`), against the 1.0 s bar an existing fast-set test is held
to. It passed; the step reds on the time alone, and `gate ok` with it:

```
SLOW [default eps] sweep::all verbs_germarms_interval::the_ring_lane_builds_at_the_certified_scalar 1.024 s (existing test: 1.0 s)
```

PR 6935's diff is `crates/topo/src/{revert,transform}.rs`,
`crates/topo/src/boolean/ops.rs` (`OneRecordTokens::of_result`) and one
new sweep test file. The row's file names no Boolean (`grep -n
"Subtract\|subtract\|boolean"` over it is empty), so the diff is not on
its path: the row sits at the bar on main and a runner 2% slow tips it.

## What closing it takes

Make the row faster, or name it in the slow set in
`.config/nextest.toml`, as the step's own message says. Not done on PR
6935: neither the test nor the slow set is that lane's.
