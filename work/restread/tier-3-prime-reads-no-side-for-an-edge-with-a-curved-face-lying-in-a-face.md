---
id: tier-3-prime-reads-no-side-for-an-edge-with-a-curved-face-lying-in-a-face
kind: issue
title: Tier 3′ cannot read the side of an edge with a curved face lying in a face of its own solid, and refuses it undecided
status: closed
opened: 2026-10-20
priority: P1
cost: M
closed: 2026-10-27
---


Found by `census-passes-a-face-inside-an-edges-material-wedge`'s lane
(branch `restread/face-in-edge-wedge`), measured there.

## What

Tier 3′ reads a line edge lying in a planar face of its own solid for
sides (`census.rs` `edge_in_own_face`), through the touch analysis's
links. A link's sectors are planar (`Sector::of`), so where one of the
edge's own two faces is curved (a cylinder whose ruling is the edge)
the reading answers `TouchUnread::Unreadable` and the census refuses
`FaceThroughEdgeWedgeUndecided` with that reason. That is fail-loud by
ruling (D1: tier 3′ reads every touch for sides), and it refused 42
results that passed tier 3′ before, the same at ε = 1e-9, 1e-6 and
1e-12. Review 1's probe (branch `restread/face-in-edge-wedge-review-1`)
read the material on a circle about each edge's midpoint:

- **38 are outside touches**, the wedge one arc of material on one side
  of the face, so presumably valid bodies now refused:
  - 16 in `sweep` `a_pierce_held_in_a_curved_face::a_plane_face_holding_the_pierce_over_a_cylinder_builds_in_every_op_and_order`:
    the union of the joined prisms (`sector ∪ triangle` and
    `triangle ∪ sector`) and the lying cylinder, either operand first,
    over both spans (8), and every member order of the fold that builds
    (8);
  - 22 in `sweep` `rest_zip_admission`: every run of
    `a_line_kiss_beside_a_tangent_site_records_its_contact` (20, a line
    kiss that overlaps nothing), and
    `the_reflex_site_off_the_pins_poses_never_ships_the_overlap_twice`'s
    `pure dRight (0.5, 0.5)` in both orders (2).
- **4 were rider-shaped**, the material two arcs about the edge, so
  invalid: the subtraction of `(sector ∪ triangle)` and of
  `(triangle ∪ sector)` with the lying cylinder, the held operand last,
  over both spans, the curved shape of CLEAVE's keel-in-face row
  (`work/cleave/a-boolean-builds-a-reflex-edge-lying-inside-a-face.md`).
  Since the Boolean parts an edge lying in a face
  (`topo::boolean::slit`) they build without this refusal, measured on
  the merge of main into `restread/face-in-edge-wedge`.

Each test pins its runs: `a_pierce_held_in_a_curved_face`'s `curved`
argument and `rest_zip_admission`'s `CURVED` list name the runs that
refuse, and every other run passes.

## Fix

Read a curved edge face's side at the edge from its tangent half-plane
there, where the edge is a line on the surface (a ruling), with the
wedge's points drawn on the surface rather than on the tangent plane,
so the reading stays sound inside the ball `rest::judge` confirms a
crossing in. Then the 38 outside touches pass tier 3′; re-pin both
tests to that.

## Closed

2026-10-27, branch `restread/curved-edge-in-face`. Measured on main at
`39134695b9`, the refusals were 42: the 20 `a_pierce_held_in_a_curved_face`
pins (the union, 8, and every member order of the fold, 12) and the 22
`rest_zip_admission` pins. In every one, all three faces were planar. The
curve was a face's far boundary: an ellipse where the lying cylinder trims
the prisms' side faces, and the post's rounded footprint (circles) ringing
`a`'s top. `Sector::of` refused any face with a non-line edge. Such a face,
held inside it or inside one of its line edges, now reads as the disc about
the point that its other edges clear (`link::Sector::cleared`,
`link::clearance`). All 42 read `Rest`, and both tests pin them as passing
tier 3′ whole. The fix this row proposed, for an edge face that is itself
curved, reached no measured body; it is filed as
`tier-3-prime-reads-no-side-for-an-edge-on-a-ruling-of-a-curved-face-lying-in-a-face`.

