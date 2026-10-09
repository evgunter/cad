---
id: a-turned-hemisphere-keeps-the-crossing-layers-door
kind: issue
title: The sphere-capped tube with its hemisphere turned about the axis (seam rulings misaligned) refuses at the crossing layer
status: closed
opened: 2026-10-02
priority: P1
cost: M
closed: 2026-10-08
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

## Released from the D10 hold (2026-10-08)

Nothing D10 changes gates this row, so it is open: the cell is reduce::lying_on's certificates against a partner seam ruling, crossing-layer geometry the Zero path keeps (check first whether it already builds; its sibling closed by PR 4148). (INTENT's re-homing of the parked rows, `work/intent/log.md`.)

## Closed (2026-10-08, TANG)

It already builds on main. PR 4123 did it, not PR 4148 (found by
bisecting first-parent merges from 4148's merge). The refusal was the
tube's seam ruling against the far hemisphere face, in the covered line
rung: the ruling's top end touches the sphere on the rim, outside that
face. PR 4123 reads a sphere face's reach over its azimuth window, and
that reach clears the pair before the arm runs. The rim arcs are then
placed by `reduce::lying_on`'s certificate (b), or by PR 4148's
interior question where (b) declines.

`pi_seam_and_kiss_through_the_boolean.rs`,
`the_sphere_capped_tube_builds_with_its_hemisphere_turned` (slow set),
pins it. Turns 0°, 7°, 30°, 45°, 90° and 173°, and the pair spun 45° and
70°, all in both member orders: tier 3 and 3′, the census, no contact
records, the volume, and point membership against the closed form. ∖
and ∩ stop at the fallback extent at every turn, as they do aligned.

Within a hair of aligned
(`the_sphere_capped_tube_turned_within_a_hair_of_aligned`), the rim
vertices `θ·R` apart:
- inside the zero band, it builds the aligned body;
- in the sliver band, the seam cover escalates;
- from about `5e-8` to `2e-4` at ε 1e-9, the same ruling door refuses
  `CurvedPierceUnsupported`, because the reach's clearance
  `R·(1 − cos θ)` is inside the pad.

Serving that from the Seam cover would widen a declared lane where
values do not decide (undeclared, the graze refuses at every turn). It
is filed and parked on D10 as
`a-covered-line-ending-just-off-the-face-keeps-the-door`.
