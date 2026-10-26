---
id: a-spline-wall-cannot-say-whether-it-holds-a-pierce-point
kind: issue
title: A union across a spline edge refuses HeldPierceUnbuilt: faces_holding_pierce asks a spline wall whether it holds a pierce point, and its trim cannot say
status: open
opened: 2026-10-26
priority: P0
cost: M
refs: [a-point-has-no-place-in-a-nurbs-faces-trim, the-sector-walk-reads-no-nurbs-face-at-a-crossing-vertex]
---

Filed by `the-sector-walk-reads-no-nurbs-face-at-a-crossing-vertex`'s
lane, from its end-to-end row.

## What

With the sector walk's spline arm in (`topo::sector_face`'s
`spline_normal`), the wavy loft united with a box across its corners
gets past the sector walk and refuses one site on.
`editor-core` `emit_topo`'s
`wavy_loft::a_union_across_a_nurbs_edge_refuses_at_a_spline_walls_held_pierce`
pins it: `HeldPierceUnbuilt { operand: A, kind: Nurbs, pierced_kind:
Plane, placed: false }`. The face is a spline wall of the loft. The
pierced face is the box's face `x = 1`.

The site is `boolean::vtxfac::held_reads` (inside
`faces_holding_pierce`). For a pierce point of the face `x = 1`, it
asks each face of the loft whose box holds the point whether the face
holds it. A spline wall's placement is
`contain::curved_face_placement_decided`, which answers
`CurvedPlacement::Trim(None)` for a spline face
(`a-point-has-no-place-in-a-nurbs-faces-trim`), and `held_reads`
refuses on it.

Two things stand behind this site:

- **The placement.** The point-in-face row gives a spline face its
  trim reading. With it, a wall that does not hold the point reads
  `Out` and is passed over.
- **A wall that does hold the point.** `held_reads` refuses
  `placed: true` for a spline face read `In` ("a spline face's trim
  never places a point In"): it has no residual to decide the point
  onto the carrier and reads no normal there.

The second may not be reached by the wavy loft. Measure it once the
first lands.

## Owed

The end-to-end row the sector-walk row could not write: the union
builds, and the crossings of each corner are named through the
boolean, the first and third, of one sense, tied. It replaces the
tripwire above. If the union then refuses at a later site, pin that
site and file it.
