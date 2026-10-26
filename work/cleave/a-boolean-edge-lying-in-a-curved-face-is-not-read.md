---
id: a-boolean-edge-lying-in-a-curved-face-is-not-read
kind: issue
title: The Boolean's parting reads edges lying in plane faces only; an edge lying in a curved face keeps the rider form
status: open
opened: 2026-10-19
priority: P1
cost: H
---


Found by the Boolean slit-form lane (`cleave/boolean-builds-the-slit-form`,
the parent row `a-boolean-builds-a-reflex-edge-lying-inside-a-face`).

## What

`topo::boolean::slit::riders` reads an edge as lying in a face only
where the face is a plane (`lies_in`): a line edge whose midpoint the
plane face holds, or a conic in the face's plane. A keel resting along
a ruling of a cylinder floor, or an arc resting in a sphere or cylinder
face, is not read, so the Boolean keeps the rider form there: one edge
claiming the face's empty side, which tiers 3 and 3′ pass. Unmeasured:
no fixture of the class is built yet.

## Fix

Read a line edge along a ruling of a cylinder or cone face, and a conic
on a quadric face, as lying in it (the reduction's `LyingIn::Curved`
already decides the second at the operand), then part it as the plane
case does: the wall-side reading is first order on the face's normal at
the edge's midpoint, so it carries over.

## 2026-10-26 — read on cylinders, cones and quadrics (CLEAVE, `cleave/riders-read-every-edge-in-a-face`)

- `slit::on_carrier` (`crates/topo/src/boolean/slit.rs:804`) reads a
  line on a cylinder or cone face at its ends and midpoint, and a
  conic on a sphere, cylinder or cone face at its ends, midpoint and
  quarter points, each point's offset decided on the band
  (`bool_rider_on_curved`); `slit::holding` (`slit.rs:715`) then reads
  it as on a plane face. `walls_inside` (`slit.rs:892`) reads the
  face's outward normal at the edge's midpoint
  (`face_outward_normal_at`), first order as before. A wall tangent to
  a curved face along the line refuses `Tangent` (`slit.rs:962`):
  which side its sheet lies on is second order in both faces' bends.
- Witness, `sweep/tests/edges_lying_in_a_face_part_wherever_they_lie.rs`
  `a_keel_along_a_cylinder_floors_ruling_parts_in_every_op`: a
  tunnel's apex resting along the bottom ruling of a rod's wall, the
  rod through the block (both ends on the wall's boundary) and ending
  inside it (the apex crossing the wall's two arcs). Every op, both
  orders, at 1e-6, 1e-9 and 1e-12 builds at its closed-form volume with
  tiers 3 and 3′, its material at sample points, and two coincident
  edges over the ruling in `A ∖ B`. Against main's reading `A ∖ B`
  holds one edge there and tiers 3 and 3′ pass it: the census reads
  an edge lying in a face of its own solid on planes only, filed as
  `work/restread/tier-3-prime-passes-an-edge-lying-along-a-curved-face-of-its-own-solid.md`.
- An arc resting in a curved face, a revolved V's tip arc on a bore's
  wall or a spherical floor (`an_arc_resting_in_a_cylinder_or_sphere_face_refuses_typed`),
  refuses `CurvedPierceUnsupported` in every op at every ε, at the
  reduction, on main and here; past it the parting refuses `Curved`.
  Filed as `work/cleave/an-arc-lying-in-a-curved-faces-interior-refuses-at-the-reduction.md`.
- Not read: an edge lying in a torus or spline face.
