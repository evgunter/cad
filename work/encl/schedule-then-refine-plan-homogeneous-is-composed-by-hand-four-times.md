---
id: schedule-then-refine-plan-homogeneous-is-composed-by-hand-four-times
kind: issue
title: "Equal-split schedule, then refine_plan_homogeneous" is composed by hand in four places
status: open
opened: 2026-09-26
priority: P4
cost: D
---


Filed by the fix pass on PR 3292 (`encl/equal-split-points-home`). The
same class as the equal-split schedule that PR homed, one level up.

## What

"Take `equal_split_points(kv, splits)`, then hand it to
`refine_plan_homogeneous`" is written out by hand four times:

* `geom_brep::patch_bound::refine_chain` (which also takes the last
  plan's knots as the refined vector);
* `geom_core::spline::net`'s test module, `chain()`;
* `geom_core::spline::algebra`'s test module, twice
  (`the_ring_applier_stays_in_step_and_near_the_described_hull` and
  `the_convex_form_bulges_by_the_ratios_own_rounding`).

A `geom_core::spline::algebra` helper returning the plan chain for an
equal-split schedule (and perhaps the refined vector) would give the
composition one home, the way `equal_split_points` gave the schedule
one.

## Also look at

`patch_bound::RATIONAL_CERT_SPLITS = 16` describes itself as the
`RATIONAL_METER_SPLITS = 16` precedent of `geom::curves::nurbs`'
rational speed meter, "mirrored". Whether the two are one decision (one
constant, one home) or two independent choices that happen to agree is
not stated anywhere; the fix should settle it and say which.
