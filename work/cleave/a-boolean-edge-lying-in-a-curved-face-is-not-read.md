---
id: a-boolean-edge-lying-in-a-curved-face-is-not-read
kind: issue
title: The Boolean's parting reads edges lying in plane faces only; an edge lying in a curved face keeps the rider form
status: closed
opened: 2026-10-19
priority: P1
cost: H
branch: cleave/riders-read-every-edge-in-a-face
pr: 5793
closed: 2026-10-31
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

- `slit::carrier_on_surface` (`crates/topo/src/boolean/slit.rs:1127`)
  reads a line on a cylinder or cone face through
  `census::line_on_quadric_carrier` (`crates/topo/src/census.rs:4133`,
  `bool_line_face_carrier_offset`, the one reading of a line on a
  quadric), and a conic on a sphere, cylinder or cone face at its ends,
  midpoint and quarter points (`bool_rider_on_curved`). Both read every
  sample before answering (`census::points_on_surface`, with unit rows
  `points_on_surface_rows`): one definitely off decides "not on" beside
  others in band. `slit::holding` (`slit.rs:985`) then reads the edge
  as on a plane face, by its records and crossings.
- Where the edge lies in a curved face with both its faces going into
  the face's material, the parting refuses `EdgeInFace` (`CurvedFace`,
  `slit::part_riders`, `slit.rs:418`): a slit or a zero-area ring in a
  curved face is neither meshed nor measured, the gate main's keel lane
  set (`work/splitdoor/the-boolean-builds-the-slit-on-a-curved-face-once-it-is-meshed-and-measured.md`,
  which lists the rows this branch moves onto it and what each owes
  back). An arc refuses `Curved` before that, as on a plane face.
- `walls_inside` (`slit.rs:1231`) reads the face's side at its
  carrier's foot of the edge's midpoint
  (`face_normal::face_outward_normal_at_foot`, `face_normal.rs:424`:
  on a cylinder, cone or sphere the implicit gradient's direction, the
  carrier's normal at the foot exactly, which asks nothing of where the
  midpoint stands; within the band of the carrier's axis, centre or a
  cone's apex plane it refuses, `NoNormal` here). A wall tangent to the
  face along a line is read at second order (`slit.rs:1338`,
  `bend_across` at `slit.rs:1428`, under `bool_rider_wall_bend_across`,
  the gap `Δκ·s²/2` at the wall's reach as `sectors.rs`'s crescent reads
  it), one reading for a plane face and a curved one: bending away from
  the face's material it leaves the face alone; on a curved face, going
  in or bent as the face is, it refuses `CurvedFace`. A rod seated in a
  bore along a ruling (`pi_seam_and_kiss_through_the_boolean.rs`
  `a_rod_in_a_bore_refuses_at_the_crossing_layer`) reads its wall
  bending away from the bore's material and builds as on main. The
  census reads the same tangent wall as undecided
  (`work/cleave/the-census-reads-a-wall-tangent-to-a-curved-face-as-undecided-where-the-parting-reads-its-bend.md`).
- Moved from main's one edge (the rider form, which tiers 3 and 3′
  pass on a curved face, or 3′ refuses) to a typed refusal, each
  re-pinned: `CurvedFace` for the box corner edge resting in a pipe's
  wall (`a_ruling_through_a_box_corner_edge.rs`, both tests), the
  prism edge on a tube's wall inside one face and down from the rim and
  the ruling across the wall's ellipse (`a_ruling_lying_on_a_wall.rs`,
  `t ∖ b`), the plate or slab less the rod and square
  (`a_pierce_held_in_a_curved_face.rs`, two tests), and the keel on a
  rod's ruling (`edges_lying_in_a_face_part_wherever_they_lie.rs`
  `a_keel_along_a_cylinder_floors_ruling_refuses_only_its_slit`);
  `Curved` for the sunk dome's rim arcs on the tube's wall, the tube
  through the dome's base, and a rod's top rim on the dome's sphere
  (`pi_seam_and_kiss_through_the_boolean.rs`, four tests), and the
  revolved V's tip arc on a bore's wall or a spherical floor
  (`an_arc_resting_in_a_cylinder_or_sphere_face_refuses_only_its_slit`,
  `A ∖ B`; every other op builds).
- Where a line lies partly in several faces across their boundaries,
  it is divided against all of them at once and each piece parted from
  the face holding it (`slit::part_riders`). An arc lying in several
  faces (a rim arc crossing a seam ruling of a two-face tube) refuses
  `Curved` before it is divided.
- An in-band reading of whether the edge's carrier lies on the face is
  raised only where both its faces go into the face's material
  (`walls_inside`, whose own errors `slit::holding` raises, but
  `Crossed`: faces leaving on opposite sides say the edge does not lie
  there), and a face only a crossing record names is not asked where
  its reading is in band. The gate is what builds the bisector caps
  (`bisector_nodes` `the_near_node_poses_at_the_witness_band` and
  `a_single_cap_just_past_the_band_builds_as_the_parting_leaves_it`: a
  cap's rim arc `2δ` long about `δ²/2` off the other rod's wall, whose
  faces leave it on opposite sides, at δ = 1e-4 and
  `δ = √(2·c·K·ε)`, c 1.1 to 1.3, building at their closed forms,
  where main `aed0b727c8` escalates the δ = 1e-4 pose).
- Not read: an edge lying in a torus or spline face
  (`work/cleave/the-parting-reads-no-edge-lying-in-a-torus-or-spline-face.md`),
  and a closed edge
  (`work/cleave/a-closed-edge-lying-in-a-face-is-never-read-by-the-parting.md`).

## Closed (2026-10-31)

PR 5793. The parting reads an edge lying in a curved face: a line on a
cylinder or cone (`census::line_on_quadric_carrier`) and a conic on a
sphere, cylinder or cone, every sample read first
(`census::points_on_surface`). Its side is read at the carrier's foot
(`face_normal::face_outward_normal_at_foot`), and a tangent wall's at
second order, one reading for a plane face and a curved one. Where the
material parts the edge in a curved face, the Boolean refuses typed
(`EdgeInFace`: `CurvedFace` for a line, as main's keel lane set,
`Curved` for an arc) rather than ship the one-edge form. The parted form
on a curved face is owed on
`work/splitdoor/the-boolean-builds-the-slit-on-a-curved-face-once-it-is-meshed-and-measured.md`
and `work/heldrecords/a-curved-slit-fails-tier-3-prime-on-its-two-coincident-arcs.md`,
which list the re-pinned rows. A torus or spline face, and the census's
first-order reading of a tangent wall, are filed rows.
