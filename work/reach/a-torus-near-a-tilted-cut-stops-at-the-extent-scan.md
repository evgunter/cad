---
id: a-torus-near-a-tilted-cut-stops-at-the-extent-scan
kind: issue
title: A torus near or across a tilted cut's ellipse rim stops at the extent scan or the join's germ frame: its oblique plane and wall pairs have no section arm
status: open
opened: 2026-10-03
priority: P1
cost: H
refs: [ellipse-edge-crossing-a-torus-has-no-root-lane, c5-plane-torus-cone-cylinder-arms]
---


The next door after `ellipse-edge-crossing-a-torus-has-no-root-lane`.

## Measured (branch `reach/ellipse-torus-roots`)

The exact tilted cut of the drum (`crates/sweep/tests/ellipse_torus.rs`:
radius 0.5, height 1, cut through `(0, 0, 0.5)` with normal
`(sin 0.3, 0, cos 0.3)`), its lower part, against a ring torus near
its `Ellipse` rim. The rim's crossing layer now decides the pair by its
certified roots (on main every pose below refused
`CurvedPierceUnsupported` on the rim), and every op stops one door on:

| pose | rim pairs | ∪, ∩, A ∖ B, B ∖ A |
| --- | --- | --- |
| tube 1e-4 to 100ε clear of the rim, outside the drum's corner (`corner_torus`) | examined, none accepted (`Miss`) | `FallbackExtentUnsupported` (the torus face's box over the cut face and the wall, no crossing event, no section classification) |
| tube 1 mm through the corner | accepted (2 certified roots) | `GermFrameUnsupported` (plane × torus, torus × cylinder) |
| ring R 0.45, r 0.1 about the drum axis at the cut's centre | accepted (4) | `GermFrameUnsupported` (cylinder × torus, torus × plane) |

Pinned by `ellipse_torus::a_torus_just_clear_of_the_rim_is_decided_by_its_roots`
and `a_torus_crossing_the_rim_reaches_the_join`.

## Why no pose builds

A torus that comes near the rim comes near the cut plane and the drum
wall, and meets both obliquely: the plane is not square to the torus's
axis (a spiric section, a quartic) and the wall is not coaxial with it.
Neither pair has a section arm (`work/germ/c5-plane-torus-cone-cylinder-arms.md`
built only the axis-containing plane), so the extent scan cannot prove
a clear pose clear and the join has no frame for a crossing one. Making
the plane square to the torus's axis forces the wall off it, and the
reverse; the same holds for a cone, so every body whose ellipse rim
reaches a torus face meets one of these two doors.

## What a fix needs

A clearance certificate for an oblique torus × plane and torus × wall
pair in the extent scan (the clear poses), and the general plane ×
torus and cylinder × torus sections for the join (the crossing ones).
