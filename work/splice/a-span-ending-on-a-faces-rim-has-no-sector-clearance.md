---
id: a-span-ending-on-a-faces-rim-has-no-sector-clearance
kind: issue
title: A span ending on a face's boundary, tangent to the face there, has no sector certificate clearing it of the face near that end
status: open
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
