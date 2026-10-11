---
id: the-convex-boehm-step-is-looser-than-lerp-on-a-varying-column
kind: issue
title: The certified Boehm step's convex form, met with its sources' hull, still reads the plane x NURBS envelope 2.4x looser than the lerp form did at a = 1e-12
status: closed
opened: 2026-10-01
priority: P3
cost: M
design: true
refs: [3524, 3737]
closed: 2026-10-10
branch: nurbs/monotone-step-and-blossom
pr: 4551
---


## What

#3524 changed the certified Boehm step (`algebra::convex_step`, shared
by `CurvePlan::apply_certified` and `compose`'s `insert_once_ring`)
from the lerp form `x + (y − x)·α` to the convex form `β·x + α·y`. #3737
meets that step with the hull of its two sources, which keeps a
constant column exact. On columns that are NOT constant, the convex
form is still looser than lerp on the plane × NURBS envelope.

`geom-brep` `r1_pxn_probes::the_certified_sup_bounds_the_dense_sampled_true_sup`,
certified `hull_sup` (the sampled truth is 1.000088900582341e-12 at
a = 1e-12 and 1.0014211682118912e-13 at a = 1e-13):

| tree | a = 0 | a = 1e-13 | a = 1e-12 (ratio) |
|---|---|---|---|
| `fe2447a440`, before #3524 (lerp) | 1.732e-14 | 1.2233e-13 | 1.000681 |
| `ba06ed4bf3`, #3524's merge (convex) | 7.55e-15 | 1.5410e-13 | 1.00676 |
| #3737 (convex met with the hull) | 3.66e-15 | 1.1621e-13 | 1.00166 |
| #3737 also met with the lerp form (not shipped) | 3.55e-15 | 1.066e-13 | 1.00058 |

At a = 1e-12 the excess over the truth is 6.8e-16 m before #3524 and
1.66e-15 m on #3737: **2.4× looser**. At a = 0 and a = 1e-13, #3737 is
tighter than lerp was. The last row shows the remainder is the form on
varying columns: meeting the convex step with the lerp form as well
recovers 1.00058, better than either form alone.

## What is owed

A decision on the form, which is #3524's design call: keep the convex
form, intersect it with the lerp form (two sound enclosures of one
value; the meet door exists, `Certification::meet`), or pick per
fixture. Width per step and fold inflation are the convex form's
argument (`compose`'s `the_convex_form_does_not_inflate_the_fold`);
this row is lerp's. Whatever lands, re-measure the r1 row's ceilings
(1.003 ratio, 2.5e-14 floor), which are set from #3737's readings.

## Re-homed from FLUX to NURBS (2026-10-10)

(FLUX orchestrator) FLUX measured 125.5 budget points against 30 and was cut on its priority seam: FLUX kept the curved closed-form arms. NURBS's charter is the spline doors (knot insertion and the certified Boehm step), and its `certified-blossom-primitive-in-geom-core-spline` waits on the first of these two rows; the second is the same combine in `CurvePlan::apply_points`. The id and the body above are unchanged; the move may have set `priority`, `cost` or `status` in the header.

## Closed

The form is decided (fork 3, `[ev]` PR 4539's two designer reports):
the certified two-point step is `Certification::between`, the monotone
form — the lower ends combined together and the upper ends together,
from the nearer source, met with the sources' hull.
Each corner is also met with its convex form `x·(1 − Λ) + y·Λ` (the
orchestrator's ruling on PR 4551, refining fork row 107's form): both
enclose the same corner range and either can be the narrower. At an
exact dyadic ratio the convex products are exact and it rounds once
where the lerp rounds twice, which `ssi::section`'s sign bisection at
`½` needs; at a thin knot-quotient ratio the convex corner is the
narrower on about a fifth of drawn steps, by up to 62.5%.
`algebra::convex_step` is gone; `CurvePlan::apply_certified` steps
through the door, as does every certified two-point step in the spline
layer (`CoeffWindow::blossom`).

The row's premise had lapsed before the decision: the 2.4× at
a = 1e-12 was the sequential extra-break insertion PR 4489 removed, and
on the tree this closes against every form reads the plane × NURBS
envelope the same to seven digits. Re-measured on head
(`r1_pxn_probes::the_certified_sup_bounds_the_dense_sampled_true_sup`):
1.443e-15 at a = 0, 1.00426e-13 against a 1.00142e-13 truth at
a = 1e-13, and 1.0000704e-12 against 1.0000889e-12 at a = 1e-12. The
row's ceilings are re-set from those readings (floor 3e-15, ratio
1.0001), with what each guards in the test's docs.

Against an exact-rational reference the monotone step is no wider than
the convex one on every row of the designers' battery
(`compose`'s `the_monotone_step_is_no_wider_than_the_convex_step_on_every_battery_row`);
PR 3737's weight guarantee is `a_refined_weight_stays_between_its_sources`.
