---
id: a-tube-ending-on-a-ball-refuses-section-loop-mixed
kind: issue
title: A tube whose end rim lies on a ball, unioned with it, passes the crossing layer and refuses Join(SectionLoopMixed)
status: closed
opened: 2026-10-02
priority: P1
cost: H
refs: [a-declared-rest-mate-does-not-license-its-rim-against-the-partner-wall]
branch: join/tube-ending-on-a-ball
pr: 4399
closed: 2026-10-10
---


Found by TANG's abutting-rim lane (branch `tang/abutting-rim`), in its
sweep for transverse rim abutments.

## What

A z-axis tube of radius 1 over `z ∈ [1, 3]` (the sweep's `extrude` of a
circle) unioned, undeclared, with a ball of radius `√2` about the origin
(`sweep::test_support::ball_poled_z`). The tube's bottom rim circle lies
on the sphere, and the tube's wall meets the sphere there at 45°. The
tube's bottom disc lies inside the ball.

Since the abutting-rim lane, the rim is an ON event at the crossing
layer (`topo::boolean::reduce::lying_on`): its two ends land on the
ball's seam meridians and split them, and its interior meets the face's
boundary nowhere else. The union then refuses in the join,
`Join(SectionLoopMixed { face })`, in both member orders. A shorter tube
over `z ∈ [1, 1.5]` refuses the same way.

The section loop is the rim circle, lying inside the ball's two faces
between their seam meridians: an in-face section curve made of operand
edges, the class JOIN-1 is about.

## What the taker owes

Measure the fixture on JOIN-1's germ loci. Its closed form is the tube's
volume above the sphere plus the ball's.

## Released from the D10 hold (2026-10-08)

Nothing D10 changes gates this row, so it is open: an undeclared union; Join(SectionLoopMixed) is the join's role resolution on an in-face section loop, which the Zero-glue path keeps. (INTENT's re-homing of the parked rows, `work/intent/log.md`.)

## Built

Branch `join/tube-ending-on-a-ball`, 2026-10-09.

- **The refusal had moved.** On main at `ba5b54d87` the witness no
  longer reaches role resolution: every op refuses
  `GermFrameUnsupported { Cylinder, Sphere }` in `partners`, where
  `germ_section_frame` read an edge's own curve only for a germ along an
  edge of BOTH solids, and the rim's germs run along the tube's edge
  inside a ball face.
- **The fix.** A germ along an edge of either solid takes that edge's
  curve as its frame; and a segment along a conic edge of one solid,
  inside a face of the other, whose kind pair has no join arm, takes
  `GermLane::EdgePlane`: the edge's solid copies its edge
  (`JoinLane::AlongEdge`), the other solid's face is cut by the edge's
  plane (`JoinLane::Split`), which meets its carrier in that conic.
- **Built:** every op in both orders, sound at its closed form (the cap
  of height `√2 − z0` is the shared volume), at radii 1, 0.3, 0.7,
  0.99, short and long, the tube's seam on and off the ball's seam
  meridians, tilted, at every ε row, and at the `Interval` scalar.
- **Refuses typed:** a rim inside one ball face (∪ and `ball ∖ tube`,
  `RingOnCurvedFace`); the rim through or near the chart's pole
  (`CurvedPierceUnsupported`, `ArcNearPole`, in-band escalations); a
  tube ending on the ball from inside
  (`a-tube-touching-a-ball-from-inside-along-its-rim-refuses-the-extent-scan`).

## Closed

Built as above by PR 4399 (merged `ee2c7879b1`, 2026-10-10). The
inside-tangent residue is its own row,
`a-tube-touching-a-ball-from-inside-along-its-rim-refuses-the-extent-scan`.
