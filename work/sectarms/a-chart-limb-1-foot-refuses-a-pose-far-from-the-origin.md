---
id: a-chart-limb-1-foot-refuses-a-pose-far-from-the-origin
kind: issue
title: limb 1's foot on a NURBS chart refuses a pose a thousand metres from the origin, its projector's acceptance an absolute 1e-13 m
status: open
priority: P2
cost: M
opened: 2026-11-03
---

Found by PR 6412's dual review (R2), on the 1×2 lane that PR built.

## What happens

The sphere-and-strip scene of `crates/geom-brep/tests/implicit_chart_section.rs`
(`a_sphere_cuts_the_strip_in_two_arcs_across_it`), translated by
`(10³, −10³, 500)`, refuses at ε = 1e-9 with
`SsiError::FootPointInconclusive { t: 0.875, last_distance: 1.14e-13 }`.
Offsets of 10, 100 and 300 certify. The same strip translated against a
plane gives two branches (`Ok`).

## Why

Limb 1 checks each sample's foot on the NURBS chart by projection
(`crates/geom-brep/src/ssi/certify.rs`, `nurbs_limbs`, about line 740:
`surface.project_from_seed(..)`, mapped to `FootPointInconclusive`). The
projector accepts a foot within `PROJECT_EPS_POINT = 1e-13` m
(`crates/geom/src/projection_policy.rs:137`), an absolute distance. One
float spacing near 10³ is 1.1e-13, so a point a thousand metres from the
origin cannot be resolved that finely, and the projection does not
converge. (This row first said the plane lane reached the same limb only
with a pcurve whose foot it already held. That stopped being true when
PR 6479 folded plane × NURBS onto the 1×2 trace: the plane now reaches
the same projector, below.)

## What closes it

The projector's acceptance read relative to the coordinates' resolution
where the point lies (or limb 1 reading the foot's distance through a
door that states its own floor), with a row at a far offset. The
`geom` file has no owning program; this row sits on SECTARMS because the
1×2 lane is what reaches it. The guard row is
`crates/geom-brep/tests/plane_nurbs_readings.rs`,
`the_plane_reads_the_kilometre_foot_frontier_on_both_sides` (the `x`
loop at 1000 m decided, at 2000 m refused).

## Plane × NURBS reaches it (2026-11-04)

PR 6479 folded plane × NURBS onto the 1×2 trace, so the plane meets
this frontier too. That is why this row is now P2.

PR 6479's dual review (R2) ran a sweep of 54 poses through
`ssi::plane_nurbs_ssi` at ε 1e-6, on main (the plane's retired ℝ⁴ trace)
and on the PR head (the 1×2 trace). The poses are three fixtures, each
moved along three directions by six offsets:
- **loop**: R2's bump (a bicubic 5×5 net over `[0, 1]²`, the middle
  nine controls 0.6 up) cut at `z = 0.15`;
- **loopHi**: the same bump cut at `z = 0.35`;
- **rim**: the inflected wall (cubic in `u`, 4×4) cut by the plane of
  normal `(0, 0.25, 1)` through `(0.5, 0, 0.4)`.

The directions are `diag` (the vector `(−1500, 900, 1100)` scaled by
0.05 … 2) and `x` and `z` (0.05 … 2 times 2000 m). On head after this
fix pass, every reading below is unchanged from R2's head column; the
re-run is from this branch's scratch probe.

| pose | \|t\| (m) | main (ℝ⁴) | head (1×2) |
|---|---|---|---|
| diag loop / loopHi / rim | 103, 207, 517 | decided | decided |
| diag loop, loopHi | 1033 | decided | decided |
| diag rim | 1033 | 1 rim | **`FootPointInconclusive`** |
| diag loop | 2066 | 1 loop | **`FootPointInconclusive`** (t 0.375, 9.15e-10) |
| diag loopHi | 2066 | `FootPointInconclusive` | 1 loop |
| diag rim | 2066 | `FootPointInconclusive` | 1 rim |
| diag loop / loopHi / rim | 4133 | `FootPointInconclusive` | `FootPointInconclusive` |
| x loop / loopHi / rim | 100, 200 | decided | decided |
| x loop, rim | 500 | decided | decided |
| x loopHi | 500 | `FootPointInconclusive` | 1 loop |
| x loop, rim | 1000 | decided | decided |
| x loopHi | 1000 | 1 loop | **`FootPointInconclusive`** |
| x loop | 2000 | 1 loop | **`FootPointInconclusive`** |
| x loopHi | 2000 | `FootPointInconclusive` | `FootPointInconclusive` |
| x rim | 2000 | 1 rim | **`FootPointInconclusive`** |
| x loop / loopHi / rim | 4000 | `FootPointInconclusive` | `FootPointInconclusive` |
| z loop / loopHi / rim | 100, 200, 500 | decided | decided |
| z loop, loopHi | 1000, 2000 | decided | decided |
| z rim | 1000, 2000, 4000 | `FootPointInconclusive` | `FootPointInconclusive` |
| z loop, loopHi | 4000 | `FootPointInconclusive` | 1 loop |

In total:
- 34 poses decide on both traces;
- 10 refuse on both;
- 5 decided on main refuse on head (bold);
- 5 refused on main decide on head.

So the frontier between about 1 km and 4 km is noisy on both traces,
rather than a systematic move. Every refusal is the foot's: limb 1's
projector misses its absolute `1e-13` m acceptance, where one float
spacing is already that size.
