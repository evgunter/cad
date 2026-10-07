---
id: a-corner-is-a-slice-of-its-face-tier-3-check
kind: issue
title: Tier 3 does not check that every corner is a slice of its own face, so a crossing at a pinch passes the gates and only the mesher refuses it
status: open
opened: 2026-10-06
priority: P2
cost: M
refs: [a-pinch-no-kept-face-can-cross-refuses, a-boolean-ships-a-face-whose-loop-passes-two-vertices-on-one-point]
branch: join/tier3-pinch-checks
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
