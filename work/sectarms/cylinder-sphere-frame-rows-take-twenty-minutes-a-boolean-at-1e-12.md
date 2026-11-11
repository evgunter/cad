---
id: cylinder-sphere-frame-rows-take-twenty-minutes-a-boolean-at-1e-12
kind: issue
title: The cylinder x sphere frame rows take about 20 minutes per boolean at eps 1e-12 (debug), and the nightly's 1e-12 job runs out of time
status: open
opened: 2026-11-11
priority: P1
cost: M
refs: [main-red-the-cylinder-sphere-ruling-sweep-refuses-tube-straddles-near-the-node-at-1e-6]
---


Split out of
`main-red-the-cylinder-sphere-ruling-sweep-refuses-tube-straddles-near-the-node-at-1e-6`
(closed by `sectarms/ruling-sweep-straddle-1e-6`), whose 1e-12 half this is.

## Measured

The nightly's 1e-12 job (debug, the `ci` profile, slow set included):

| row (`sweep::all cylinder_sphere_frame::…`) | nightly 42761779270 (`50cfcf499d`) | nightly 42965378060 (`b2916558d9`) |
|---|---|---|
| `a_ball_across_a_cylinder_wall_in_two_loops` | 567 s | past 960 s when the job was cancelled at 120 min |
| `…_in_two_loops_turned` | 545 s | past 960 s |
| `…_wider_than_the_wall` | 511 s | past 960 s |
| `…_turned_chart` | 268 s | past 960 s |

In the same job before, `germ_torus_components`' two germ-mint rows took
1010 s and 879 s.

A local probe of the first row's first run only (`A ∪ B`), debug, ε 1e-12,
one thread per binary, the two binaries side by side on one 4-core box:

| tree | `A ∪ B` |
|---|---|
| main `33e3d606` | 1220 s, builds at its volume |
| `sectarms/ruling-sweep-straddle-1e-6` (witness at the loop's middle) | 1187 s, builds at its volume |

This box ran the same row at ε 1e-6 about six times slower than the
nightly's runner (`in_two_loops`: 117 s here, 19.5 s in nightly 42761779270).
So one boolean costs roughly 3 minutes of runner time. Each row runs six
booleans, about 18 to 20 minutes of runner time.

## What is known

- The section is the same size on every tree measured: at 1e-12 each of
  the pose's two loops marches 6368 samples, fits 6368 control points and
  certifies at tube radius 0.04375 over 6365 boxes. That held at
  `08f0298b` (the cell-seeded march) and on this lane's branch (the
  witness-seeded march).
- Moving the march's seed changed nothing measurable (the table above).
- `gdb` samples of the row at 1e-12 land in topo's boolean pipeline, not in
  the section. They hit edge certification (`analytic_rung3`'s limb-3
  probe and `one_arc_r3`, `section_bounds::tube_bounds`), the chord join's
  path parity (`ring_path::spline_crossings` → `curve_pair::meet`), and
  pcurve certification (`ProjectedImage::sweep_box`). Each one walks the
  6365-span carrier, and the pipeline asks each of them many times.
- `08f0298b`, the merge before PR 6518, is not a baseline. There every run
  of the row stops on `ResultInvalid { VolumeUncomputable … }` (seen
  here at ε 1e-6). That is the issue's "ends in 99 s, red": the row stopped
  before doing its work.

## What closing it takes

Bisect the cost between `50cfcf499d` (567 s) and main at ε 1e-12. Then
either cut what each boolean asks of a 6365-span carrier, or move the
four rows' 1e-12 runs where the job's 120-minute limit does not reach
them, and say why in the row.
