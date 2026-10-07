---
id: a-ring-on-a-cone-or-torus-face-has-no-island-winding
kind: issue
title: A ring on a torus face has no island winding: the plane×torus germ pair refuses before the ring lane
status: parked
opened: 2026-10-07
priority: P2
cost: M
refs: [a-ring-on-a-sphere-face-has-no-island-winding, boolean-sector-algebra-has-no-cone-arm]
blocked_on: [c5-plane-torus-cone-cylinder-arms]
---


Found by the sweep of `a-ring-on-a-sphere-face-has-no-island-winding`
(TANG). The item first named a cone face too; that half closed on
2026-10-07 (below), and what is left is the torus.

## What is left

No op reaches a ring on a torus face. A box edge through the outer
wall of the `R = 2, r = 1/2` donut (the box `x > 2.3, y > 0.2`, a slab
of it, or a corner inside it) refuses every op as
`GermFrameUnsupported { Torus, Plane }`, in both member orders, at the
join's germ-pair frame dispatch (`boolean::join::pair_section_frame`).
C5 has no plane×torus section arm (GERM,
`c5-plane-torus-cone-cylinder-arms`), and that is what the dispatch
refuses. Behind it, `chord_join::wall_section` refuses a torus as "a
section through a face kind no section arm reads". It used to say "the
gate refuses", which was wrong: the operand gate admits a torus face
(`reduce::boolean_arm_exists`), and it is the section arms that have no
torus.

## What a fix owes

The sphere and cone ring lanes share one chart-free reading,
`crate::ring_path`: a path parity from an outer-loop point to the
closing chord's midpoint, built from pieces whose crossings with a
plane-section arc are closed form. A torus could ride it:
- its parallels (axis-normal circles) and meridians (circles in planes
  through the axis) are both circle pieces;
- a plane section is planar, so a piece meets it where it crosses the
  section's plane, as on a cone.

Two things are new:
- the in-span reading of a `Curve3::Spiric` arc, which needs the
  spiric's parameter at a point;
- the parity's path-independence. A ring lies in a face's interior and
  a face is cut by its seams, so the curve bounds a disc there. That has
  to be checked against the torus faces the lanes mint.

## Closed: the cone (2026-10-07, TANG `tang/cone-ring-island-winding`)

A cone face's ring lane winds its island and re-homes its rings by the
shared reading (`chord_join::path_island_winding`,
`chord_join::path_ring_side`). The cone's path runs along a ruling and
round a parallel, clear of the apex. Its rows are on a cone sheet,
`topo`'s `chord_join::cone_ring_rows` and `ring_path::tests`. No op
reaches a cone ring yet, because GERM's cone gate comes first
(`boolean-sector-algebra-has-no-cone-arm`, where the doors are
measured).
