---
id: main-red-reach-volume-backstop-rows-lose-their-premise
kind: issue
title: main red: two reach_volume_backstop rows fail on main, one on its own fixture premise and one refusing a fattening below the stated resolution
status: open
opened: 2026-10-27
priority: P1
cost: E
---

Filed by the SPLICE interior-arm lane (`splice/interior-arm`), which
split its slow-set reds against `origin/main` at `6117d399` in a
separate worktree. Both are red on main with the branch's messages.

## What

`crates/sweep/tests/reach_volume_backstop.rs`:

- `a_result_equal_in_truth_but_larger_at_the_midpoint_passes` panics on
  its own premise: "the fixture's premise: the two rods' midpoints
  differ (2.7488935718910685, 2.7488935718910685)". The two midpoints
  now read equal, so the row tests nothing it states.
- `planted_subtraction_results_refuse_down_to_the_stated_resolution`:
  "a fattening by δ = ε/100 is below the stated resolution", left
  `Err(ResultVolumeImplausible)`, right `Ok(Validated)`.
