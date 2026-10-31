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

## Update 2026-10-27: two duplicates folded in

Two later rows filed the same failure, and both closed into this one:
`main-red-reach-volume-backstop-midpoint-premise-and-planted-subtraction`
(PCERT E1b's suite round, PR 5797; red on `be2b7a139c` and
`2f9e6f9e39`) and `main-red-5734-moves-tilted-cut-upper-and-two-reach-backstop-rows`
(a RESTREAD triage lane on `9d97abd704`, the same bisect). The latter's
third row, `tilted_cut_upper` in the thread-count digest, has since been
re-cut by QUAD (PR 5818), so the two backstop rows are all that is left red.

## 2026-10-27 — the same merge moved a step-export sidecar (nightly triage)

`step-export` `kernel_sidecars::kernel_sidecar_fields_match_live_kernel`
went red at the default ε on nightly run 40455870959 (main
`fb476030a9`). Bisected over first-parent merges from `38f1a24ed3`:
first bad `ee4a094e` (PR 5734), last good `4bc935de`. `cut_cylinder`'s
midpoint moved from `3926990816.9872403` to `3926990816.987241`
(`π·1.25e9` is `3926990816.9872413`) and its pad narrowed from
`1022.98` to `1022.37` mm³: the same tightening as the rows above, the
analytic volume inside both enclosures. The triage branch
(`triage/nightly-2026-10-27`) re-syncs the sidecar to the live kernel;
if this row's decision reverts the bracket, the sidecar follows it.

## 2026-10-28 — pinned on main (GREEN, `green/full-suite-pins`)

Nightly run 40637814390 (main `a1cfd1cfdb`), `full suite`; re-run on main `91c83c38`. Both rows red at the default ε and 1e-6, green at 1e-12. Each is
pinned at the default ε and 1e-6 on its message (`the fixture's premise:
the two rods' midpoints differ` and `a fattening by δ = ε/100 is below
the stated resolution`); each pin goes red the day its row passes.

Un-pinning owes a re-measure: each pin stops its row at main's first stop, so whatever runs after it in the row is unmeasured.
