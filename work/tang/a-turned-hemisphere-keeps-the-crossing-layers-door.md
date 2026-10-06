---
id: a-turned-hemisphere-keeps-the-crossing-layers-door
kind: issue
title: The sphere-capped tube with its hemisphere turned about the axis (seam rulings misaligned) refuses at the crossing layer
status: open
opened: 2026-10-02
---


## What

Measured by the dual review of PR 3849. The sphere-capped tube of
`crates/sweep/tests/pi_seam_and_kiss_through_the_boolean.rs`, walls
declared `Seam` and discs `Rest`, builds with the hemisphere's meridian
seams aligned with the tube's seam rulings
(`the_sphere_capped_tube_builds_with_its_walls_declared_a_seam`).
Turned 30° or 90° about `z`, so the two operands' rim vertices no longer
coincide, it refuses `CurvedPierceUnsupported` in both member orders.

## Why, as far as read

Each rim semicircle of one operand then ends inside a rim arc of the
other. That is the cell of
`a-rim-lying-on-a-wall-across-its-seam-ruling-keeps-the-door` (a rim
arc crossing the partner face's seam ruling mid-arc, a crossing neither
`reduce::lying_on` certificate places), and likely of
`a-turned-lens-keeps-the-door`. Check whether it is the same cell before
building it separately.
