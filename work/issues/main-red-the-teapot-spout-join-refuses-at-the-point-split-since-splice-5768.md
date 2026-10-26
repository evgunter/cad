---
id: main-red-the-teapot-spout-join-refuses-at-the-point-split-since-splice-5768
kind: issue
title: Main is red: the teapot's spout join refuses at the spline point split, not the crossing layer, since SPLICE's PR 5768 merged
status: open
opened: 2026-10-26
priority: P0
cost: E
refs: [5768]
---


Filed by RESTREAD (PR 5704's CI), outside its fence.

## What

CI's `python suite (wheel + guide + north-star)` fails
`test_north_star.TestTeapot.test_both_joins_refuse_through_the_document`
at `crates/pncad-py/tests/test_north_star.py:3320`. The spout join is
pinned to refuse with "cannot yet work out where one solid's spline
(NURBS) face meets" (`crates/topo/src/boolean/mod.rs:4357`). It now
refuses earlier, with `BooleanError::PointSplitCarrierUnsupported`
(`crates/topo/src/boolean/mod.rs:4422-4428`, raised at
`crates/topo/src/boolean/reduce.rs:4391`):

    a spline (NURBS) edge of the second operand has to be split where
    the solids meet, and the Boolean cannot find a point's parameter on
    a spline.

Seen on PR 5704's push ff9b7af (job 121705582425).

## Why it is main's

The single test, run locally on a wheel built at each commit:

| Commit | Result |
|---|---|
| main a0c924c | fails, the same text |
| 19a92d4 (main before #5764) | fails |
| ec676d0 (merge of #5768, splice/boundary-first) | fails |
| 1ea8d73 (its first parent) | passes |
| PR 5704's head ff9b7af (main e9680c5 merged) | passes |

Main's own push CI did not run the python suite, so main shows green.

## Fix

Either the spout's meeting reaches the crossing layer again, or the
row re-pins to the point-split refusal, saying what moved: the
refusal now names the spout's spline edge rather than its face.
