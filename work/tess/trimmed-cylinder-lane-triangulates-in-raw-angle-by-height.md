---
id: trimmed-cylinder-lane-triangulates-in-raw-angle-by-height
kind: issue
title: The trimmed cylinder lane sizes cells hu (radians) by r·hu (metres) and triangulates in raw (θ, v), so at r much larger than 1 m an off-row trim point may fan across many columns
status: open
opened: 2026-10-20
priority: P3
cost: E
---


## What

Unverified, from a designer on `lofted-rational-walls-refuse-at-some-deltas` (2026-10-20): the cylinder lane's parameter aspect is 1:r, so the raw-UV Delaunay sliver that makes lofted walls refuse may also hit trimmed cylinders of large radius.

## To do

Probe first: a trimmed cylinder with r = 20 m and an off-row trim point, at several δ. If it refuses or fans wide, the lattice-metric CDT ruled for the lofted walls covers it (the cylinder's isometric development), and this rides that unit. If not, close it with the measurement.
