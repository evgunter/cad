---
id: a-loop-seam-meeting-the-neck-cylinder-at-its-edge-reads-in-band
kind: issue
title: klein wall 3: a seam of the swept loop meets the bulb's neck cylinder 3.2e-9 from the cylinder's own edge, and the curve-pair reading stays in band (CurvedPierceEscalated, Crossing(OnEdge))
status: open
opened: 2026-11-23
priority: P0
cost: M
---


Found by LATHE's `lathe/skin-sub-arc-walls`, which cuts a swept wall at each
sub-arc joint (`work/lathe/the-skin-cuts-an-arc-wall-at-each-sub-arc-joint.md`).
It is klein wall 3's and wall 4's stop
(`demos/tour/src/klein.rs` `wall3_pinned`, `work/splice/curved-demo-joins-build.md`).

## Measured (ε 1e-9)

`union(bulb, loop)` and `subtract(bulb, loop)` refuse with the same error:

    CurvedPierceEscalated { operand: B, face: FaceKey(4v1), edge: EdgeKey(10v1),
      decision: Crossing(OnEdge), diag: Indeterminate { margin: Enclosure
      [3.1838e-9, 3.1838e-9], band { zero 1e-9, escalate 1e-8 },
      predicate: "spline_composite_past_the_band" } }

The face, 4v1, is the bulb's neck cylinder. The edge, 10v1, is a spline
seam of the loop between two of its walls. The loop's two circles are
each two half turns, and the sweep now cuts each half turn at its
sub-arc joint, so the loop's seams lie in the seam plane `y = 0`, where
the neck cylinder's own edge lies. The seam meets the cylinder face
3.2e-9 from that edge: in band, so the meeting is decided neither on
the edge nor off it. The loop's station frames are carried by double
reflection, which is exact only to rounding (`crates/sweep/README.md`,
`[sweep.frame]`), so the seam may truly be 3e-9 off the plane, and the
reading is a near coincidence rather than a misreading. Not measured at
1e-6 or 1e-12.

## Owed

Decide whether the boolean should read a curve that ends within band of
a curved face's edge as meeting it there, or whether the scene should
place the loop's seams off the neck's edge, and build that.
