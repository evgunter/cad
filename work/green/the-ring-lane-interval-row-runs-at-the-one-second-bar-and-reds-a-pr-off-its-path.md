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

## Two more rows, on the next run

PR 6935's second head (`fd97599c`, run 43618343337) passed the ring-lane
row and went red on two other existing sweep rows instead, again with
nothing failing:

```
SLOW [default eps] sweep::all pis_arc_capped_poses::the_cut_cylinders_ellipse_face_takes_nothing_away 1.017 s (existing test: 1.0 s)
SLOW [default eps] sweep::all ring_carry_through_by_piece::a_three_arc_ring_is_metered_arc_by_arc_in_every_subtraction_order 1.009 s (existing test: 1.0 s)
```

Both subtract, so the diff is on their path. Timed three times each on
one box with the branch's `revert.rs`, `boolean/ops.rs` and
`transform.rs` and again with main's: 1.42 to 1.57 s against 1.45 to
1.88 s for the first row, 1.65 to 1.83 s against 1.72 to 1.75 s for the
second. The change does not move them. Which row tips over differs from
run to run, so the finding is the set of rows within a few percent of
the bar, not any one of them.

## What closing it takes

Make the row faster, or name it in the slow set in
`.config/nextest.toml`, as the step's own message says. Not done on PR
6935: neither the test nor the slow set is that lane's.
