---
id: a-span-ending-on-a-faces-rim-has-no-sector-clearance
kind: issue
title: A span ending on a face's boundary, tangent to the face there, has no sector certificate clearing it of the face near that end
status: closed
closed: 2026-10-31
branch: splice/rim-sector-clearance
opened: 2026-10-26
priority: P1
cost: M
refs: [a-span-is-read-against-a-faces-boundary-edges-first, an-analytic-edge-against-a-nurbs-face-has-no-crossing-lane]
---



Split out of `a-span-is-read-against-a-faces-boundary-edges-first`
(closed on `splice/boundary-first`), which owed it and did not build
it.

## What

Klein wall 3's neck seam line ends on the loop wall's rim circle, and
the wall is tangent to the line there (it leaves the rim straight up).
Once the boundary pass (`geom_brep::curve_face::boundary`) has decided
that end on the rim, the span's interior near it is still undecided:
the line and the face are within the band of each other over a
stretch beside the end. C5's curve × face clause owes a clearance
certificate there, the sector one: the line's departure from the end
against the face's inward departure across its boundary at that point.
`ConicDeparture` reads an edge's departure, not a face's, so the
face's half is to build (on a spline face, from `patch_bound`'s normal
enclosures).

## Why it did not ride the boundary pass

It has no witness yet. The klein line's end is not on the rim at the
demos' ε: the loop's cap tilt puts the rims up to 1.1e-4 m apart
(`carvetail/a-half-turn-spine-sweeps-only-off-its-exact-tangents`), so
the pass reads the line `Clear` of the boundary and the end is never
decided on it. Build it once the tilt is fixed, or with the interior
arm (`an-analytic-edge-against-a-nurbs-face-has-no-crossing-lane`),
whose enclosures settle the rest of such a span.

## Released (2026-10-31)

`a-half-turn-spine-sweeps-only-off-its-exact-tangents` closed with CARVE's PR 6138. The klein loop now sweeps its exact spine through `sweep_body`, carried by the rotation-minimizing frame, with no lofted interior frames and so no cap tilt. Re-measure the klein line's end against the rim at the demos' ε: it may now be a witness. (CARVE orchestrator)

Review tier: single FULL (a departure leaning the wrong way clears a crossing). (2026-10-31)

## Closed (2026-10-31)

Built on `splice/rim-sector-clearance`. The witness was klein wall 3's neck seam line (edge 2v1), whose start is a vertex of the loop wall's `z = 3` rim, on the wall's arc-joint crease `u = ½`, with the wall leaving the rim upward while the line leaves downward, about 1.2° off the wall's tangent plane. The boundary pass read the end there (`Hit::End`), and the interior arm refused `Crease`, since no box touching a crease may hold the end.

- **The sector**: `geom_brep::curve_face::spline_face`'s `Pair::sector`. An end whose foot lies on a chart domain edge is read by the span's tangent over a window leaving it, against the chart's partials over a window inside the edge (`NurbsBoxes::deriv_box`, the arm's own enclosures). The rim tangent bounds the slide along the rim. The face's inward departure then bounds every solution in the box within the band of the end, whenever the span leaves on the far side. The box is an end's box, and the walk does not read past that end.
- **Rows**: `curve_face::spline_face::sector_rows`. Cleared: a line leaving a rational cylinder's rim tangent and off the face, at either end, on a crease and off one. Refused: a line along the wall and one into the cylinder from the crease. Found: a crossing within the sector's reach (the hook).
- **Next stop**: klein wall 3 moves to edge 4v1 (the flare's cone seam) crossing face 3v3 on its arc-joint crease, at all three ε. Owed by `carvetail/the-skin-cuts-an-arc-wall-at-each-sub-arc-joint`, where the evidence is filed.

