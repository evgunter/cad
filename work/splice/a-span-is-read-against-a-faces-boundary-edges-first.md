---
id: a-span-is-read-against-a-faces-boundary-edges-first
kind: issue
title: An operand edge is read against the other face's carrier, never first against that face's boundary edges, so a rim it shares with a NURBS wall cannot glue
status: closed
opened: 2026-10-26
closed: 2026-10-26
priority: P0
cost: H
refs: [an-analytic-edge-against-a-nurbs-face-has-no-crossing-lane, a-unions-flush-reading-cannot-follow-a-nurbs-carrier, a-flush-reading-finds-no-point-on-a-nurbs-member-edge]
branch: splice/boundary-first
---


Filed by the SPLICE orchestrator from the design Ev took on PR 5727
(C5's curve × face clause, fork log row 198).

## What

The sweep reads an operand edge against a face through the face's
carrier (a residual sign), and reads a meeting on the face's boundary
through that same carrier. C5's curve × face clause instead reads a
span's ends first, then the span against each of the face's boundary
edges through the curve-pair table (`geom-brep` `curve_pair`). Its
rules:
- A shared run along a boundary edge is a coincidence decided Zero at
  the coincidence door (DESIGN `[coincidence]`), and the join glues
  the two edges.
- A point meeting there is a meeting on the face's boundary.
- Only what the boundary leaves undecided goes to the interior arm
  (`an-analytic-edge-against-a-nurbs-face-has-no-crossing-lane`).

The only reading of a span against a face's boundary today is
`carrier_cross::boundary_crossing`, and it presupposes that the span
lies on the face's carrier.

## Witnesses

- **lily wall 8.** The blade's base rim circle IS the sheath wall's
  boundary rim. This row, the two flush rows on this slate and the
  dihedral side reading
  (`the-side-of-a-spline-face-is-read-by-no-dihedral`) are all lily
  needs. No interior arm is involved.
- **klein wall 3's end.** The neck's seam line ends on the wall's rim
  circle, a line × circle meeting decided in closed form. The wall is
  tangent to the line there, so a clearance certificate is owed around
  that point: the sector one, comparing the line's departure with the
  face's inward departure across its boundary. `ConicDeparture` reads an
  edge's departure, not a face's, so that reading is part of this row.
  Klein also waits on the cap tilt
  (`carvetail/a-half-turn-spine-sweeps-only-off-its-exact-tangents`).

Serves every face kind, analytic ones included.

## Closed (2026-10-26)

Built on `splice/boundary-first`.
- **The table**: `geom_brep::curve_face::route`, one exhaustive
  `(CurveKind, SurfaceKind)` match with no wildcard. The analytic arms
  name the sweep's existing root doors; every arm with a spline side,
  and every spiric arm, is unbuilt and names what it lacks
  (`curve_face::rows::every_kind_pair_routes_to_its_door`).
- **The pass**: `geom_brep::curve_face::boundary`, each boundary
  vertex on the span, then each boundary edge through
  `curve_pair::meet`: `Meets` (a meeting strictly inside the span),
  `Along` (a shared run), or `Clear`. Rows in `curve_face::rows`:
  shared run in both senses, the rest of the rim's circle (no run), a
  run shorter than the span, a crossing, an end on the rim, a far
  span, a corner in the band's gap (escalates), a vertex in the band
  of a thin ellipse.
- **The sweep**: `topo` `reduce::spline_face` reads a spline face
  through the pass ahead of any carrier reading. `Meets` is a pierce
  on the boundary; `Along` records the edge's ends on the boundary
  (`MintedRule::Along` places a later split's vertex the same way);
  `Clear` refuses as before, unless the span is an edge of the
  face's body (`reduce::twin_edge`), whose incidence with a face it
  does not bound is its ends: the boolean's no-self-intersection
  precondition, not a check. The sweep reaches `spline_face` through
  `route`; the analytic doors are not yet dispatched through it
  (`the-analytic-crossing-doors-are-not-dispatched-through-the-curve-face-table`,
  filed here).
- `carrier_cross::boundary_crossing` reads through the pass and keeps
  only its placement.

Witnesses: `sweep` `a_sheath_grafted_on_its_blade` (both operand
orders), lily wall 8, teapot wall 3.

**What the witnesses reached.** Lily wall 8 needed more than the
shared run: the blade's rim is also read against the sheath's other
wall, which it meets only at the lens's tips, and that is settled by
the twin-edge certificate above. It now refuses
`EdgeCarrierUnsupported { operand: A, site: PlanarCrossing }` on a
NURBS tip seam of the blade against the sheath's base cap: the
converse arm, `nurbs-edge-crossing-rung-is-the-ring-composite`.
Teapot wall 3 moved too: the belly meridian pierces the spout's wall
on the wall's seam, so the pass finds a boundary meeting and the split
of the spline seam refuses
(`a-meeting-on-a-spline-boundary-edge-has-no-split-parameter`, filed
here). Klein wall 3 does not move: the seam line's end is not on the
rim at the demos' ε (the cap tilt,
`carvetail/a-half-turn-spine-sweeps-only-off-its-exact-tangents`), so
the pass reads it `Clear`; the sector certificate at that end has no
subject until the tilt is fixed
(`a-span-ending-on-a-faces-rim-has-no-sector-clearance`, filed here).
