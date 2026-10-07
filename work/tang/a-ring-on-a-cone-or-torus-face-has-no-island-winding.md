---
id: a-ring-on-a-cone-or-torus-face-has-no-island-winding
kind: issue
title: A ring on a cone or torus face has no island winding or re-homing reading: the ring lane reads a cylinder's chart and a sphere's sections only
status: open
opened: 2026-10-07
priority: P2
cost: M
refs: [a-ring-on-a-sphere-face-has-no-island-winding]
---


Found by the sweep of `a-ring-on-a-sphere-face-has-no-island-winding`
(TANG), which gave a sphere face's ring lane a chart-free winding and
re-homing reading.

## What is left

`chord_join::chart_island_winding` (the island winding
`boolean::join::choose_roles` asks for on `RingClosure::Wall`) and
`chord_join::chart_ring_side` (ring re-homing on a curved face) read a
cylinder wall's chart; a sphere face goes to
`chord_join::sphere_island_winding` and `chord_join::sphere_ring_side`.
Every other curved kind refuses
`Join(RingOffCylinderChart { kind })` through `chord_join::no_wall_chart`.
The chord lane mints a section on a cone face
(`chord_join::wall_section` admits `Cone`), so a cone face's
ring lane, if a pose reaches it, stops there.

## Measured

Nothing yet: no row reaches a ring on a cone or torus face. The first
step is a pose (a box edge through a cone's wall, clear of its rims
and apex) under every op.

## What a fix owes

The sphere arm's statement leans on the section plane cutting the
sphere in a circle that bounds two caps; a cone or torus section has no
such pair, so it does not carry over. Candidates: the cylinder arm's
chart reading on a cone's (apex-free) chart, or the sphere arm's
great-circle parity carried to a path on the face whose crossings with
the loop's carriers are closed-form.
