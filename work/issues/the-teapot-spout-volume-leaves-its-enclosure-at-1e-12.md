---
id: the-teapot-spout-volume-leaves-its-enclosure-at-1e-12
kind: issue
title: demo-tour's tour_runs_green_at_eps_1e_12 is red on main at the teapot: the spout's measured volume lies 1.16e-10 m^3 off the straightened frustum's, outside its 8.1e-12 half-width
status: closed
opened: 2026-10-23
refs: [demo-tour-wall-8-no-longer-refuses-at-1e-12, 5637]
priority: P1
cost: E
closed: 2026-10-24
---


Found by stage 4 I (`mates-declare-no-contact`, PR 5637), whose
`demos (tour + wild)` row runs the tour suite. Reproduced on clean
`origin/main` at `debb2a60`:

    cd demos/tour && CAD_TOLERANCE_EPS=1e-12 \
      cargo nextest run --release --profile ci tour_runs_green_at_eps_1e_12

`demos/tour/src/teapot.rs:2181` panics: "the canal's V =
0.000055052648797962526 vs the straightened frustum's
0.000055052533158981754: they differ by 1.156389807722329e-10 m^3,
OUTSIDE the certified enclosure's own half-width 8.094490616656182e-12".
The branch reads the same numbers bit for bit, and touches nothing the
spout's mass properties read.

`work/tess/demo-tour-wall-8-no-longer-refuses-at-1e-12.md` is the
same test red on main for another cause (wall 8); on this main the walk
reaches the teapot. Main's push CI does not run the demos, so nothing
caught it.

## What closing it takes

Decide which half moved: the spout's certified enclosure (its
`volume_pad` at 1e-12) or the measured volume itself, and either fix
the measure or restate the scene's comparison.

## Closed

2026-10-24: the same finding as
`work/isotail/the-teapot-spouts-frustum-oracle-misses-its-measured-volume-at-1e-12.md`,
filed in parallel and closed by ISO's PR 5642 (the spout's oracle).
