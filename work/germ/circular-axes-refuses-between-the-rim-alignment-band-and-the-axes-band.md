---
id: circular-axes-refuses-between-the-rim-alignment-band-and-the-axes-band
kind: issue
title: Tilts between eps/r and sqrt(2 eps/r) refuse CircularAxes with no lever: pc_rim_alignment and ellipse_axes_distinct read two different quantities
status: open
opened: 2026-10-12
priority: P3
cost: E
---

Found by the CLEAVE section-end design fork (PR 4810).

## What

`pc_rim_alignment` decides sin θ·r against the band, where θ is the
plane's tilt off square to a cylinder's axis. Past it, the table builds
an ellipse through `Curve3::ellipse`, whose `ellipse_axes_distinct`
decides (a − b) ≈ r·θ²/2. Tilts between about ε/r and √(2ε/r)
therefore pass the first check and refuse `CircularAxes` at the
second, with no lever. PR 4810 measured this: "1e-4 is not clear
enough off square" at ε = 1e-9 or so.

## Fix

One reading of the same input decides both: either the table stands
the circle in until the axes are distinct, or the refusal is typed
with the tilt lever. These are two readings of one input, so one of
them is the decision.
