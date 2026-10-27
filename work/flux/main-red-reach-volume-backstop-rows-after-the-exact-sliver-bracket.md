---
id: main-red-reach-volume-backstop-rows-after-the-exact-sliver-bracket
kind: issue
title: Main red (slow set): two reach_volume_backstop rows fail since PR 5734's exact cylinder-sliver bracket
status: open
opened: 2026-10-26
priority: P1
cost: M
---


## What

Two slow-set rows in `crates/sweep/tests/reach_volume_backstop.rs`
fail on `origin/main` (reproduced at `2f9e6f9e`). Slow sets do not run
in hosted CI, so the PR gate missed them. Found by SPLICE's
slow-set triage lane (branch `splice/slow-set-reds`).

**Bisect** (`git bisect run --first-parent`, both rows):
- first bad: `ee4a094e`, the merge of PR 5734
  (`flux/cylinder-sliver-exact-bracket`);
- last good: `4bc935de`, its first parent.

**Failures, verbatim:**

- `a_result_equal_in_truth_but_larger_at_the_midpoint_passes`,
  `reach_volume_backstop.rs:492` (the fixture's premise):

      the fixture's premise: the two rods' midpoints differ (2.7488935718910685, 2.7488935718910685)

  Both rods now read the same midpoint, which is `3.5·π·0.25²`
  (2.748893571891069) to the last bit. The row needs two readings
  with different midpoints for its test of the enclosure arithmetic's
  direction, and it has lost that witness.
- `planted_subtraction_results_refuse_down_to_the_stated_resolution`,
  `reach_volume_backstop.rs:392`:

      assertion `left == right` failed: a fattening by δ = ε/100 is below the stated resolution
        left: Err(ResultVolumeImplausible)
       right: Ok(Validated)

  The `1e-3`, `100·ε` and `ε` plants still refuse, and the right
  answer still passes. The `ε/100` plant is now resolved and refused
  as well, so the open range `last_round_open` states (`1.2e-13·s`)
  no longer matches what the backstop does on this pair.

## What is owed

Decide, on FLUX's ground, whether the new readings are right. If they
are, re-meter `last_round_open` and its doc and give the midpoint row
a pair whose midpoints still differ. If they are not, fix the bracket.
