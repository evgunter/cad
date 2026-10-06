---
id: the-convex-boehm-step-is-looser-than-lerp-on-a-varying-column
kind: issue
title: The certified Boehm step's convex form, met with its sources' hull, still reads the plane x NURBS envelope 2.4x looser than the lerp form did at a = 1e-12
status: open
opened: 2026-10-01
priority: P3
cost: M
design: true
refs: [3524, 3737, pxn-ratio-ceiling-reds-main-at-default-and-1e-6]
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
