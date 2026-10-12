---
id: a-wide-loft-section-escalates-chord-side-at-1e-12-on-main
kind: issue
title: A wide loft section escalates chord_side at the 1e-12 row on main
status: closed
pr: 4785
opened: 2026-10-12
priority: P0
cost: E
branch: encl/wide-loft-section-red
closed: 2026-10-12
---


## What

`sweep::loft_profiles_door::at_interval_a_wide_section_refuses_as_section_not_a_point`
fails at `CAD_TOLERANCE_EPS=1e-12` on main (`d7eb3abe8b`): validating the
section escalates `chord_side` with band zero 1e-12 instead of refusing
the section. The test arrived with PR 4741
(`intent/nurbs-weights-knots-in-t`). Main's per-PR CI does not run
sweep's eps rows unless a diff touches sweep, so main reads green; the
first run to take the row was PR 4740's (CI run 38184535720, eps
1e-6/1e-12 job), and the failure reproduces identically on `origin/main`
in a clean worktree. The nightly will go red on it.

Filed by CONTACTHOLD, which met it on PR 4740 and did not fix it.

## CI's run of it varies (NURBS, 2026-10-12)

PR 4551's CI met it in run 38198669660 at `f50ab134` (eps step, red). It did **not** red in run 38205248872 at `0d5b56685e`, whose eps step ran 13 352 rows with 1 failure, and that failure was editor-core's.

NURBS build A reproduces it locally on main `8cd31ca1` and on PR 4551's branch, each on its own target dir, at opt-level 1 and ε 1e-12, with identical panics:

```
crates/sweep/tests/loft_profiles_door.rs:39:10: the section validates: Escalated { site: SegmentPair(seg 0, seg 1), source: Indeterminate { margin: Enclosure [-2.000177801164682e-12, 2.000177801164682e-12], band: Band { zero: 1e-12, escalate: 1e-11 }, predicate: Some("chord_side") } }
```

So the row reds intermittently on CI. That is either nondeterminism in the chord-side margin or an eps row the CI filter does not always select; the cause is not checked. — (NURBS orchestrator)
