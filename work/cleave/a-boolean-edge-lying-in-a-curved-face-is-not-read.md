---
id: a-boolean-edge-lying-in-a-curved-face-is-not-read
kind: issue
title: The Boolean's parting reads edges lying in plane faces only; an edge lying in a curved face keeps the rider form
status: review
opened: 2026-10-19
priority: P1
cost: H
branch: cleave/riders-read-every-edge-in-a-face
pr: 5793
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

- `slit::carrier_on_surface` (`crates/topo/src/boolean/slit.rs:1022`) reads a
  line on a cylinder or cone face at its ends and midpoint, and a
  conic on a sphere, cylinder or cone face at its ends, midpoint and
  quarter points, each point's offset decided on the band
  (`bool_rider_on_curved`); `slit::holding` (`slit.rs:891`) then reads
  it as on a plane face. `walls_inside` (`slit.rs:1124`) reads the
  face's outward normal at the edge's midpoint
  (`face_outward_normal_at`), first order as before. A wall tangent to
  a curved face along a line is read second order (`slit.rs:1206`):
  its sheet lies in the face's material where its normal section bends
  off the tangent plane further toward the material than the face's
  does (`bend_across`, `slit.rs:1311`, under `bool_rider_wall_bend_across`,
  the gap `Δκ·s²/2` at the wall's reach as `sectors.rs`'s crescent reads it);
  bent alike, it is read as a plane wall in a plane face. A rod seated
  in a bore along a ruling, its seam on the touching line
  (`pi_seam_and_kiss_through_the_boolean.rs`
  `a_rod_in_a_bore_refuses_at_the_crossing_layer`), reads its wall
  bending away from the bore's material and keeps one edge.
- Where neither end of the edge lies on the curved face's boundary the
  parting would be a ring of no area in it, which the curved face's
  measure does not read; it refuses `EdgeInFace` (`CurvedRing`,
  `slit.rs:445`). The box with its rulings on a pipe's wall
  (`verbs_ga_r2_probes.rs`
  `r2_a_box_with_on_carrier_rulings_builds_every_op_undeclared`, `p ∖ b`)
  moves there from main's one edge per ruling. Filed as
  `work/cleave/a-ring-of-no-area-in-a-curved-face-is-not-measured.md`.
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
- Moved from main's one edge (the rider form, which tiers 3 and 3′
  pass on a curved face) to a typed refusal, each re-pinned: the box
  corner edge resting in a pipe's wall (`a_ruling_through_a_box_corner_edge.rs`,
  both tests, `CurvedRing`); the prism edge inside one tube wall face
  (`a_ruling_lying_on_a_wall.rs`, `CurvedRing`); the sunk dome's rim
  arcs on the tube's wall and a rod's top rim on the dome's sphere
  (`pi_seam_and_kiss_through_the_boolean.rs`, three tests, `Curved`).
  Moved from the rider form to the slit form: the prism edge running
  down the wall from the rim, and the ruling across the wall's
  ellipse (`a_ruling_lying_on_a_wall.rs`), one edge and one vertex
  more, tiers 3 and 3′ at the closed form.
- Where a line lies partly in several faces across their boundaries,
  it is divided against all of them at once and each piece parted from
  the face holding it (`slit::part_riders`). An arc lying in several
  faces (a rim arc crossing a seam ruling of a two-face tube) refuses
  `Curved` before it is divided.
- An in-band reading of whether the edge's carrier lies on the face is
  raised only where both its faces go into the face's material
  (`walls_inside`, whose own errors `slit::holding` raises), and a face
  only a crossing record names is not asked where its reading is in
  band.
- Not read: an edge lying in a torus or spline face
  (`work/cleave/the-parting-reads-no-edge-lying-in-a-torus-or-spline-face.md`),
  and a closed edge
  (`work/cleave/a-closed-edge-lying-in-a-face-is-never-read-by-the-parting.md`).
