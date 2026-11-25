---
id: a-loop-seam-meeting-the-neck-cylinder-at-its-edge-reads-in-band
kind: issue
title: A spline wall's seam ending exactly on another face's edge reads in band at about 1.5–3 ε (CurvedPierceEscalated, Crossing(OnEdge)); klein wall 3 and the bent-tube rim are its witnesses
status: open
opened: 2026-11-23
priority: P0
cost: M
---


Found by LATHE's `lathe/skin-sub-arc-walls`, which cuts a swept wall at each
sub-arc joint (`work/lathe/the-skin-cuts-an-arc-wall-at-each-sub-arc-joint.md`).

## The class

A seam between two spline walls that ends exactly on another face's edge
(the meeting exact in ℝ) is read by the curve-pair reader against that
edge (`Crossing(OnEdge)`) with an enclosure that does not shrink to zero:
it stays a few ε wide, inside the band, so the meeting is decided neither
on the edge nor off it and the boolean escalates.

## Witnesses

- **klein wall 3** (`demos/tour/src/klein.rs` `wall3_pinned`,
  `work/splice/curved-demo-joins-build.md`; wall 4 the same). At ε 1e-9
  `union(bulb, loop)` and `subtract(bulb, loop)` refuse alike:

      CurvedPierceEscalated { operand: B, face: FaceKey(4v1), edge: EdgeKey(10v1),
        decision: Crossing(OnEdge), diag: Indeterminate { margin: Enclosure
        [3.1838e-9, 3.1838e-9], band { zero 1e-9, escalate 1e-8 },
        predicate: "spline_composite_past_the_band" } }

  4v1 is the bulb's neck cylinder; 10v1 a spline seam of the swept loop.
- **the bent tube on a cylinder's rim**
  (`sweep`'s `a_rim_arc_on_a_cylinder_beside_a_spline_wall::a_rim_arc_beside_a_bent_spline_wall_is_taken_on_the_cylinder`,
  which pins it): the tube's seam 6v1 meets the cylinder's wall 3v1 on the
  wall's top rim, read at 1.521e-6, 1.538e-9 and 1.556e-12 at ε 1e-6,
  1e-9 and 1e-12, about 1.5 ε at every row.

## Mechanism

The meeting is exact in ℝ: in both witnesses the seam lies in a plane that
holds the other face's edge. The reading's enclosure of it is what stays
wide, a few ε at every ε row, so it is the reader's own enclosure, not the
geometry. Station-frame rounding (`crates/sweep/README.md`,
`[sweep.frame]`, double reflection) would put the seam off the plane by
rounding, about 1e-16 relative, some seven orders smaller than the margin
read, so it is not the cause.

The seam in `y = 0` on klein's loop is not new: it is the seam at a
profile vertex, where the loop's circles' two half turns meet, which main
already had. What the sub-arc cut changed is which pair the boolean meets
first: the walls are now quarter turns with joint seams too, and the walk
reaches the vertex seam's meeting with the neck before the stops main
read (`work/splice/curved-demo-joins-build.md` has the table).

## Owed

Read a seam ending on a face's edge as meeting it there: an enclosure of
the meeting that shrinks with ε (the reader carrying the seam's end and
the edge's plane exactly), or a decision by identity that the seam's end
lies on the edge's carrier. Re-measure both witnesses after.
