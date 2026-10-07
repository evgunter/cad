---
id: a-corner-is-a-slice-of-its-face-tier-3-check
kind: issue
title: Tier 3 does not check that every corner is a slice of its own face, so a crossing at a pinch passes the gates and only the mesher refuses it
status: open
opened: 2026-10-06
priority: P2
cost: M
refs: [a-pinch-no-kept-face-can-cross-refuses, a-boolean-ships-a-face-whose-loop-passes-two-vertices-on-one-point]
---


## What

Under the ruling (Ev, PR 4057), a face may pass several vertices on one
point, each a cone of the solid. The ruling also asks for a tier-3 check
that every corner is a slice of its own face: a corner's wedge contains
none of that face's other edges at that point. The pinch unit (branch
`join/pinch-one-vertex-per-cone-build`) did not build it.

Nothing in tier 3 sees a crossing at a pinch today. The mesher does:
its planar and trimmed lanes refuse `TessellateError::PinchWedge` where
a sector at a pinch handle is bounded by two passes' sides (PR 4074).
Measured on the unit's batteries with `tessellate` added to `outcome`
(pierce, pinch, corner pairs, both reflex, all 84 `rc_wide` shards, r1
and r2 pinch probes, near-tangent, cylinder, holed, island): 0
`PinchWedge` refusals. No body ships with a crossing there, and the
check would refuse nothing those batteries build.

Where a crossing could still come from: the pre-zip weld
(`the-pre-zip-pinch-weld-retires-once-coincident-pierces-split-per-cone`),
or a split that groups cones wrong.

## The shape to give

Per planar face, per point key its loops pass at two or more vertices:
each corner's wedge, from its leaving edge's tangent round to its
arriving edge's reversed tangent about the face normal, contains no other
of the face's edges at that point. Refuse a crossing typed. Pin it on a
hand-built crossed face (the mesher's `crossed_corners_at_a_pinch_refuse_typed`
chart, made a body).

## Built (branch `join/tier3-pinch-checks`)

Check 9 gained a corner arm (`validate.rs`, `pinch_corner_errors`).
Where one loop of a face passes a point key at two or more corners,
each corner's wedge must hold none of that loop's other edges there.
The wedge runs counterclockwise about the face's outward normal from
the leaving side to the arriving side. A crossing refuses
`ValidationError::PinchCornerCrossed { face, vertex, edge }`, once per
point. An in-band sign refuses `PinchCornerEscalated`.

- Each side is a departure from the point, levered at the edge's
  extent: a line's chord, an arc's walk tangent. Each sign is
  `n·(x × y)/|x|`, a length. A side along a bound is not inside it.
- Corners of two different loops at one point are left to the contact
  arms. Arm 1 already reports that as a `Vertex` contact
  (`RingMeetsOuter`, `RingMeetsRing`).
- **A curved face is read the same way**, in the tangent plane at the
  point, about the outward normal `face_outward_normal_at` gives
  (cylinder, sphere, torus). A corner of a smooth face is, to first
  order, a corner of its tangent plane. The surface's parametrisation
  is not needed; the chart's normal at the point is. Silent where that
  door gives no normal (cone, NURBS, `Approx`), on a NURBS side, and on
  first-order ties. Filed:
  `restfront/the-corner-slice-arm-is-silent-on-cones-nurbs-faces-and-first-order-ties`.

Pinned by `validate::tests::check_9_refuses_a_corner_crossing_its_face_at_a_pinch`:
the mesher's `crossed_corners_at_a_pinch_refuse_typed` chart extruded,
its two passes of the origin put on one point key per cap. The crossed
prism refuses once per cap; its uncrossed twin passes check 9.

**Measured** (release, main `fe26bdfe` vs head): every line of
`pierce_runs_battery` (4 536), `pinch_runs_battery` (3 024),
`corner_pairs_battery` (16 380), both reflex batteries (1 152 each)
and all 84 `rc_wide` shards (40 320) is byte-identical. That is 0 new
refusals, as predicted.
