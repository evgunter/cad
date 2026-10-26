---
id: a-span-is-read-against-a-faces-boundary-edges-first
kind: issue
title: An operand edge is read against the other face's carrier, never first against that face's boundary edges, so a rim it shares with a NURBS wall cannot glue
status: dispatched
opened: 2026-10-26
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
