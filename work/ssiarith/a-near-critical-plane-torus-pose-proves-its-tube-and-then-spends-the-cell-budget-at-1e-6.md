---
id: a-near-critical-plane-torus-pose-proves-its-tube-and-then-spends-the-cell-budget-at-1e-6
kind: issue
title: a near-critical plane x torus pose now proves its tube at eps 1e-6 and then spends the whole cell budget before it refuses
status: open
opened: 2026-11-25
priority: P3
cost: M
---



Filed by SECTARMS's `sectarms/limb3-halves-cover` lane (the build of Ev's
decision on PR 7056, PR 7776).

## Measured

The torus `(2, 0.5)` of
`pair_section::the_count_is_the_morse_count_at_every_pose_near_every_critical_height`
(`crates/geom-brep/tests/pair_section.rs`) against the plane tilted
from its axis by the sine 0.2 at height 0.89, a hundredth under the
critical height `sR + r`, carried through `march_over_the_ball`:

| ε | main | the branch |
|---|---|---|
| 1e-6 | refuses `TubeStraddles` in 0.27 s | refuses `CellBudget { budget: 200000 }` in 5.6 s |
| 1e-9 | refuses `CellBudget { budget: 200000 }` in 9.4 s | the same, 11.2 s |

At 1e-6 on main no rung of the as-marched carrier is a graph, and
limb 3 refuses before the accounting runs. On the branch limb 3 halves
its cover and proves a tube, as it does at 1e-9 on main, and the
exhaustiveness accounting then spends its whole cell budget and
refuses, as it does at 1e-9 on main. So the pose refuses at both bands
on both, and the 1e-6 refusal moved to the later stop and costs twenty
times as much.

The row carries a transverse pose at each critical value offset by
`±1e-2` over eleven tilts, and allows a typed refusal. At ε = 1e-6 it
runs 226 s on main and 754 s on the branch, alone. (It is red at
ε = 1e-6 on both, on its count of poses, 418 against the 500 it asks:
that count is taken before any carry and is reported to the
orchestrator, not filed here.)

## What would answer it

Read why the accounting does not close on these poses at either band:
the tube the pose proves, against the cells it is asked to account.
`work/sect/main-red-cylinder-sphere-section-runs-out-of-cells-near-the-node-at-1e-6.md`
is the cylinder × sphere shape of the same stop.
