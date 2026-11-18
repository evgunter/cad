---
id: the-clearance-assertion-does-not-ask-the-depth-gap
kind: issue
title: A metric clearance bound is not asked the depth gap the injectivity question reads, so coaxial walls 0.5 apart refine at 0.2 and refuse at 0.499 where the root would answer
status: open
opened: 2026-11-18
priority: P1
cost: M
---


Found by EMBED's lane `embed/nested-tori` (2026-11-18, PR 7119).

## Evidence

`topo::clearance`'s walk asks the census's depth gap of a face pair's
root (`apart_in_depth`: one face's greatest distance from an axis, a
centre or a core circle below the other's least), and discharges the
root where the gap clears the bound. It is asked by the injectivity
question alone (`Bound::pair`, `at_root`).

The first head of PR 7119 (`bc0ebc7b1`) asked it under every bound.
`apart_in_depth` takes the bound `c` and is pinned against one, so
that is a one-line change. On that head five rows of
`editor-core`'s `m10_5_clearance_interval` went red, each because the
tube's coaxial walls, 0.5 apart, answered at their root:

- `a_starved_cell_budget_refuses_typed` and
  `a_starved_cell_depth_refuses_on_its_own_axis`: `≥ 0.45` read
  `Holds` in one cell, where the rows want a budget and a depth
  refusal (hosted CI run 44271216982);
- `the_answer_is_deterministic_across_repeats`: its pinned 978 cells
  became one;
- `the_cost_curve_is_measured_at_both_ends`: `≥ 0.2` held in one cell,
  no dearer than a witnessed violation;
- `a_band_costs_its_costlier_bound`: `≥ 0.1` and `≤ 0.4` both answered
  with no split, so neither is the costlier.

Not measured there: `≥ 0.499`, which the cost-curve row pins as the
frontier no budget settles. The depth gap less its pads is above it,
so the root would read `Holds`.

## The question

Those rows describe the assertion's cost on a pair "neither the tree
nor a support can separate at its root". With the depth gap asked,
that pair is separated at its root, the three regimes of the cost
curve are no longer shown by this fixture, and a bound the engine
refused as its honest limit holds. Whether the metric bounds ask the
depth gap is CLEAR's: it moves verdicts of the assertion door
(refusals to `Holds`), and the five rows need a pair with no depth
site (two spline walls, or analytic faces whose depth ranges overlap)
to keep saying what they say.

`the-clearance-determinism-row-runs-under-a-thousand-cells-since-6902`
wants a deeper fixture for the same reason.
