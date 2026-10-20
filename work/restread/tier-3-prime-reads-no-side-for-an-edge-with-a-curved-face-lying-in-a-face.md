---
id: tier-3-prime-reads-no-side-for-an-edge-with-a-curved-face-lying-in-a-face
kind: issue
title: Tier 3′ cannot read the side of an edge with a curved face lying in a face of its own solid, and refuses it undecided
status: open
opened: 2026-10-20
priority: P2
cost: M
---


Found by `census-passes-a-face-inside-an-edges-material-wedge`'s lane
(branch `restread/face-in-edge-wedge`), measured there.

## What

Tier 3′ reads a line edge lying in a planar face of its own solid for
sides (`census.rs` `edge_in_own_face`), through the touch analysis's
links. A link's sectors are planar (`Sector::of`), so where one of the
edge's own two faces is curved (a cylinder whose ruling is the edge)
the reading answers `Unreadable` and the census refuses
`FaceThroughEdgeWedgeUndecided` ("a curved face meets the point…").
That is fail-loud by ruling (D1: tier 3′ reads every touch for sides),
but it refuses bodies that passed before and are presumably valid:

- `sweep` `a_pierce_held_in_a_curved_face::a_plane_face_holding_the_pierce_over_a_cylinder_builds_in_every_op_and_order`:
  the union of the joined prisms and the lying cylinder, held operand
  first;
- `sweep` `rest_zip_admission::a_line_kiss_beside_a_tangent_site_records_its_contact`:
  `dUp (0.25, 0.25) a ∪ b′` and the rows after it, a line kiss that
  overlaps nothing, almost certainly a valid body now refused;
- `sweep` `rest_zip_admission::the_reflex_site_off_the_pins_poses_never_ships_the_overlap_twice`:
  `pure dRight (0.5, 0.5), p ∪ q`.

Each test now accepts this one refusal and cites this row.

## Fix

Read a curved edge face's side at the edge from its tangent half-plane
there, where the edge is a line on the surface (a ruling), with the
wedge's points drawn on the surface rather than on the tangent plane,
so the reading stays sound inside the ball `rest::judge` confirms a
crossing in. Then restore the three rows to tier 3′ passing.
