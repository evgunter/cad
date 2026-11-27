---
id: a-closed-ball-the-no-crossings-path-re-charts-comes-back-poled-off-its-axis
kind: issue
title: The no-crossings path's re-chart moves a whole one-band ball's poles and seam off the input's axis
status: open
opened: 2026-11-23
priority: P2
refs: [closed-sphere-escape-is-re-charted-by-rotation-beside-the-meridian-cut, a-section-loop-inside-a-one-band-curved-face-refuses-the-interior-loop-guard]
cost: M
---


Found by the LATHE interior-loop lane (2026-11-23, PR 7440), on the
LATHE orchestrator's routing of the dual review.

## What

`boolean::ops::sphere_extent_scan` answers a CLOSED sphere group's
escape by re-charting it: `apply_recuts` turns the group about its
centre so its seam crosses every escape plane. On a one-band ball that
moves the chart's poles and its wrap edge off the axis the construction
put them on, and the result carries the turned chart, against
`[topology]`'s seam clause (a native construction's seam is its wrap
edge, kept to its tips at the chart singularities it wrote).

Measured on the PR's branch (head `e6fb8792e2`): the unit ball poled on
`y` (`sweep::test_support::ball_poled_y`, its sphere's frame axis
`(0, 1, 0)`, `u_ref` `(1, 0, 0)`) ∩ the slab `|z| < 0.1` (no crossing
anywhere) comes back on a sphere framed on axis `(0, 0, 1)`: the result
is poled on `z`. Pre-existing on main with two π-bands (the same scan).

The crossings path's cut-in (PR 7440) cuts every R-loop in along the
face's own chart and leaves the chart where it was, so the two paths now
answer one escape two ways, which ORBIT's
`closed-sphere-escape-is-re-charted-by-rotation-beside-the-meridian-cut`
already records for the rotation against the trimmed cut-in.

## Open

Whether the no-crossings path cuts a closed group's escape in as it cuts
a trimmed one's (the ORBIT row's measured move), now that the output
merge glues a sphere's cut away; the re-chart's `EscapeParallel` and
`RecutAlign` questions would retire with it.
