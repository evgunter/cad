---
id: a-cylinder-split-just-off-its-rim-vertex-exhausts-the-volume-quadrature-at-rest
kind: issue
title: A cylinder split by a plane tilted 1e-2 just off its seam rim vertex leaves a sliver whose volume quadrature exhausts its budget, so the split refuses at rest
status: open
opened: 2026-10-14
priority: P2
cost: M
---


Found by CLEAVE's PR 4810 (`sweep` `section_end_poses`, the square
poses' lever check), after main `05e88d1294` began gating each split
side at rest (`ResultNotAtRest`).

## What

The unit cylinder (`z ∈ [0, 1]`, extruded) split by the plane through
`(1, 0, d)` with normal `(−1e-2, 0, 1)`: the plane passes `d` above the
seam's rim vertex and leaves a sliver wedge below it. The split refuses
`Finish(ResultNotAtRest { side: Below, errors: [VolumeUncomputable {
Face { QuadratureBudget { rounds: 13 } } }] })`, about 30 s per run:

| ε | d | width_len | target_len |
|---|---|---|---|
| 1e-9 | 5e-8 | 1.0e-4 | 1.024e-6 |
| 1e-12 | 5e-11 | 1.05e-4 | 1.024e-9 |
| 1e-12 | 1e-9 | 5.2e-6 | 1.024e-9 |

The same plane at `+1e-2`, and square, builds. Before the at-rest gate
the split returned these sides; whether they validated at rest then is
not measured here. PR 4810's earlier census met the same budget on a
slab turned `1e-2` along a cylinder's ruling at the Boolean, which no
longer reproduces there.

## Owed

Say whether this sliver face's quadrature should converge (the wedge
between the rim disc and a plane `1e-2` off it, meeting at a chord near
the seam vertex), and if it should not, give the refusal its one
ending. PR 4810's pin accepts exactly this refusal on that move and
names this row.
