---
id: cylinder-sphere-frame-rows-take-twenty-minutes-a-boolean-at-1e-12
kind: issue
title: The cylinder x sphere frame rows take about 20 minutes per boolean at eps 1e-12 (debug), and the nightly's 1e-12 job runs out of time
status: closed
opened: 2026-11-11
closed: 2026-11-11
priority: P1
cost: M
branch: sectarms/cylsph-1e-12-cost
refs: [main-red-the-cylinder-sphere-ruling-sweep-refuses-tube-straddles-near-the-node-at-1e-6, a-curve-pair-walk-samples-a-whole-spline-carrier-at-every-foot]
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

## Closed

2026-11-11, by `sectarms/cylsph-1e-12-cost`.

**The bisect: no merge made a boolean dearer.** One run of the
`in_two_loops` row (`A ∪ B`), ε 1e-12, timed around `topo::union` alone
and around the run's whole check (`at_its_volume`), on one 4-core box:

| tree | profile | the boolean | the run with its checks | ends |
|---|---|---|---|---|
| `50cfcf499d` | debug | 993 s | 993 s | `RingOnCurvedFace` on the sphere face, a pinned door |
| main `19eb4541` | debug | 885 s | 1325 s | builds at its volume |
| `343823b756` (PR 6509's parent) | release | 87 s | 87 s | `RingOnCurvedFace` |
| `e2435632d8` (PR 6509) | release | 98 s | 154 s | builds at its volume |
| `50cfcf499d` | release | 100 s | 100 s | `RingOnCurvedFace` |
| `19eb4541` | release | 61 s | 88 s | builds at its volume |

(`release` is the workspace's release profile, debug assertions on; the
bisect used it where a debug run costs a quarter of an hour.) The
boolean costs the same or less on every tree. What grew is the row:
PR 6509 (`spiricarm/rotational-lane-sphere`) gave the chorded sphere
face its volume lane, so every run that stopped at the result gate's
volume (`Door::Sphere`) now builds, and the row then checks the body it
built (`at_its_volume`: tiers 2 and 3′, the at-rest certificate, a
union with a far brick, the volume). Those checks cost half a boolean
again, about 1.8× the run, which is the nightly's 567 s going past
960 s. That is the row doing the work it was written to do, not a
regression.

**What each boolean asked of the 6365-span carrier.** `gdb` samples of
the debug run on main put 19 of 30 in pcurve certification's period
gate (`ProjectedImage::sweep_box` → `frame_box_on`), and the leaf was
`span_stretches` scanning every knot of the net to find the few a part
crosses. `sweep_box` boxes sixteen parts a span, so the run asked 28
sweeps, 1.43 million parts and 1.94 million `span_stretches`, each a
scan of the ~6400-knot list: quadratic in the spans. `spans_of`
(the span hulls' memo fill) found each part's holding span by the
same kind of scan. Both now bisect the sorted list, answer for answer
(`pcurve_cache::bisected_reads_rows` holds each to the scan it replaced
over ranges at and between repeated knots).

| one run, ε 1e-12, debug | the boolean | the run with its checks |
|---|---|---|
| main `19eb4541` | 885 s | 1325 s |
| this branch | 455 s | 638 s |

**What is left is linear per call and filed.** After the fix the samples
spread over the chord join's path parity (`ring_path::spline_crossings`
→ `curve_pair::meet`, 14 of 40), the span hulls' composite (8), and
limb 3's tube (`analytic_rung3` → `tube_bounds`, 9). The first is the
curve-pair walk's foot search, which samples the whole 6365-span carrier
at each of 2435 feet a run asks: filed on CPAIR as
`a-curve-pair-walk-samples-a-whole-spline-carrier-at-every-foot`. The
other two read each span of the edge once per call.

**Verified** (local, debug, this box about nine times the nightly
runner's time): every `cylinder_sphere_frame` row passes at the default
ε (wall 1141 s, four at a time) and at 1e-6 (274 s), and the four rows
above pass at 1e-12, four at a time: `in_two_loops_turned` 3303 s,
`wider_than_the_wall` 3546 s, `in_two_loops` 3734 s, `turned_chart`
3869 s. The nightly's 1e-12 job is the check of record: at this
branch's per-run cost the four rows sit at or under their `50cfcf499d`
times.
