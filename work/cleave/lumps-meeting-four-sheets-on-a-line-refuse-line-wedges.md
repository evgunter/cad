---
id: lumps-meeting-four-sheets-on-a-line-refuse-line-wedges
kind: issue
title: Lumps whose faces meet four sheets on a line, less a flush partner, refuse LineWedges Overlap
status: open
opened: 2026-10-27
priority: P2
cost: M
refs: [a-box-straddling-the-ringed-top-less-the-plate-refuses-line-wedges, two-lumps-touching-along-an-edge-refuse-pinch-rays-against-a-flush-partner]
---

Found by PR 5812's claims review (`cleave/flush-partners-at-a-slit-build`),
at ε default, 1e-6 and 1e-12, at rest, turned, and flipped and turned.

## What

- `x` is the cube `[0, 1]³` ∪ the S `[1, 2] × [0, 1] × [1, 2]` ∪ the S2
  `[1, 2] × [1, 2] × [0, 1]`, in either union order: four pieces of
  material meet at `(1, 1, 1)`. `y` is `[−0.5, 2.5] × [−0.5, 1.5] ×
  [0.5, 1]`. `y − x` refuses `LineWedges { fault: Overlap }` on the line
  `(1, 1, z)`, `z ∈ [0.5, 1]`. Main refuses it `ClassificationInvariant`
  ("a fate cut's end lies inside no one piece of its face");
  PR 5812's fate placement moves it on to the result gate.
- The same class on the ringed plate (the plate less the pyramid
  lying under its top): `y` less two boxes flush under the top that
  touch along the edge `x = tip.x, y = tip.y` at the ring's tip,
  `[1, tip.x] × [0.5, tip.y] × [0.3, 1]` ∪ `[tip.x, 2.2] × [tip.y, 1.8] ×
  [0.3, 1]`. `y − x` refuses `LineWedges` `Overlap` on that edge, on
  main and on PR 5812 alike.

`LineWedges` is the typed refusal of a re-pairing the kernel has not
built (four sheets meet on the line and the material decides which
pair into edges), so these are construction gaps.

## Owed

Build them as the material decides, or refuse them before the gate.


## A brick's edge along a prism's reflex edge (2026-11-01, `cleave/orbit-readings-read-the-beside-tie`)

The same refusal where one operand's convex edge lies along the
other's reflex edge. `x` is the pinch of
`union_flush_onto_edge_contact` (bricks `[0, 1]² × [0.5, 1.5]` and
`[−1, 0]² × [0.5, 1.5]`, touching along the z-axis); `y` is the
notched prism of
`two_dangling_null_edges_meeting_on_the_pinch_line_build_in_every_op`
stood in the plane `x = 0` and extruded along `+x` or `−x`, so the
reflex edge of its notch runs along the bottom edge `y = 0, z = 0.5` of
the brick it holds. `y − x` refuses `LineWedges { fault: Overlap }` on
that line, from `(0, 0, 0.5)` to `(±1, 0, 0.5)`, at four notches
(−40° to 200°, −20° to 220°, −60° to 240°, −10° to 190°), at ε default,
1e-6 and 1e-12, on main and on the branch alike: the prism's 240°
there less the brick's 90° is two wedges, 40° and 110° at the
−40°–200° notch, which do not overlap. The other five ops build at
their volumes. Pinned as the refusal it is by
`union_flush_onto_edge_contact::two_dangling_null_edges_meeting_beside_the_pinch_line_build_in_every_op`.
