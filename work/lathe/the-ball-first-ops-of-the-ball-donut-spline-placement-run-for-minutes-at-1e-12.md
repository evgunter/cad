---
id: the-ball-first-ops-of-the-ball-donut-spline-placement-run-for-minutes-at-1e-12
kind: issue
title: The ball-first ops of the ball-donut spline placement run past 300 s each at ε 1e-12, on main and on lathe/strut-ring-homing
status: open
opened: 2026-11-28
priority: P2
cost: M
refs: [a-section-loop-inside-two-one-band-curved-faces-refuses-at-the-section-certificate]
---


Found by the LATHE strut-ring lane (`lathe/strut-ring-homing`,
2026-11-28) while it wrote
`sweep` `germ_sphere_torus_placements::the_spline_edge_placement_under_every_op`.

## What

The ball turned 1.1 rad about `(0.3, 1, 0.2)` and moved to
`(0.42, −0.15, 0.3)` (`moved`), against the donut at the origin
(`donut0`), with the ball first: `topo::union(moved, donut0)`,
`intersect` and `subtract`.

| ε | release |
|---|---|
| 1e-9, 1e-6 | each builds in well under a minute (the whole row takes 58 s and 6 s) |
| 1e-12 | each runs past a 300 s cap, and the row past 50 minutes |

Main at `1460fc7dbc1`, with `chord_join.rs` as main has it, runs
`union(moved, donut0)` past the same 300 s cap at 1e-12. So this is not
the strut-ring change.

Not bisected. It may be the class of
`inside/main-slow-since-7801-the-face-door-spline-arm-runs-slow-rows-and-the-mesh-budget-job-for-hours`;
nobody has checked.

## Pin

At ε 1e-12, `the_spline_edge_placement_under_every_op` holds only the
donut-first three. The ball-first three are a row of their own,
`the_spline_edge_placement_ball_first_ops`, `#[ignore]`d with a reason
citing this item, so a run lists it as ignored. At the other ε the
every-op row reads all six.
