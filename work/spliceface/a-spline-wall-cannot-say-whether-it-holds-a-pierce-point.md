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

## Measured against point-in-face (2026-10-27)

The point-in-face lane (`splice/point-in-face`, PR 5805) measured this
union on a scratch merge of 5805's head with 5802's. The union still
refuses `HeldPierceUnbuilt { placed: false }`, and the spline arm is
reached. `vtxfac::faces_holding_pierce` asks
`curved_face_placement_decided` about the loft wall `FaceKey(4v1)` at
`(1, −1, 3)`. The arm answers `Trim(None)` because it cannot build the
wall's trim mid-boolean: `loop_lift` returns
`LiftGap { half_edge: 53v1 }`. That half-edge has no stored pcurve row,
most likely because it is a piece of a corner edge the sweep split.

So this row also waits on the boolean keeping, or re-deriving, the
rows of a spline face's split edges before it asks the trim. There are
two doors:
- `pcurves::split_cache`, which restricts a stored row to a split
  piece;
- `pcurve_of`, which derives a row on demand but needs an
  `AtRestPolicy` that the `Decide`-generic containment door does not
  have.

Decide which while building this row. Re-measure once PR 5805 lands.

## Measured with point-in-face (SPLICE, PR 5805, 2026-10-27)

Measured with the spline arm of `contain::curved_face_placement_decided`
(`topo::boolean::spline_region`) on a merge of PR 5805 with PR 5802.
The union still refuses `HeldPierceUnbuilt { placed: false }`, now for a
different reason. The arm is reached for the loft wall at the pierce
point `(1, −1, 3)`, but it cannot build the wall's trim:
`Body::loop_lift` returns `LiftGap` for a half-edge of the wall's loop
with no stored pcurve row mid-boolean, most likely a piece of a corner
edge the sweep split. So the trim answers `Trim(None)`.

What this row needs now is the boolean keeping, or re-deriving, the
rows of a spline face's split edges before it asks the face's trim.
There are two doors for that:
- `pcurves::split_cache` restricts a stored row to a split piece;
- `pcurves::pcurve_of` derives a row on demand, but needs `AtRestPolicy`,
  which the `Decide`-generic containment door does not have.

Moved from SPLICE to SPLICEFACE on 2026-10-28, by `git mv` in SPLICE's split along its seams (Track size; Ev, in chat, 2026-10-28).

## The normal door its second half needs (2026-10-31)

`splice/a-vertex-piercing-a-spline-face-reads-no-outward-normal` leaves
this row's site as it stands. `held_reads` still reads its spline wall
through `contain::curved_face_placement_decided` and still refuses
`placed: true` for a wall read `In`. What that lane added is the door
the second half needs:
- `contain::curved_face_placement_charted` returns the in-trim foot an
  `In` placement was decided at;
- `face_normal::spline_outward_normal_at` reads the wall's outward
  normal at that chart point.

The wall's bend there is `T::spline_chart_arm`, as the pierce now reads
its pierced spline face's (`PierceLever::Within`). The placement's
uniqueness on the wall is
`the-spline-placement-read-certifies-no-second-in-trim-foot`.
