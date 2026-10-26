---
id: a-boolean-edge-running-past-the-face-it-lies-in-is-not-parted
kind: issue
title: An edge of a Boolean result lying in a face along part of its length refuses EdgeInFace Partly instead of parting there
status: closed
opened: 2026-10-19
priority: P1
cost: H
branch: cleave/boolean-parts-an-edge-running-past-its-face
closed: 2026-10-26
---


Found by the Boolean slit-form lane (`cleave/boolean-builds-the-slit-form`,
the parent row `a-boolean-builds-a-reflex-edge-lying-inside-a-face`).

## What

An edge of a Boolean result whose two faces both bend into a face's
material, lying in that face along part of its length only, crossing
the face's boundary, is a doubled wedge D1 (`crates/topo/README.md`,
tier 3) parts into two coincident edges. `topo::boolean::slit` read it
and refused `BooleanError::EdgeInFace { gap: Partly }`: the parting
needs the edge and the face's boundary divided at their crossings first.
Main had built the rider form, the edge whole across the face. The
witnesses:

- `crates/sweep/tests/review_4831_probes.rs`,
  `p12_a_plane_tangent_to_the_middle_of_a_hole`: the holed plate less
  a cutter whose face `x = 1` holds the hole's ruling along its middle
  half;
- `crates/topo/tests/a_vertex_read_again_classes_every_edge.rs`: the
  half-turn partners along the face (8 cells), and a pyramid on the
  void's face, in the arch or round its island, `y − x`, every pose;
  `crates/topo/tests/a_vertex_read_by_two_sector_passes.rs`, the same
  pyramids. Their edge lies wholly in the face and ends on its corner
  by a vertex-vertex record, which the reading took as running past;
- `crates/topo/tests/three_solids_on_one_line.rs` (`UNPARTED`): the line
  body less the bridge moved 1e-2 along x, and less the 0° prism
  against the bisector, every order.

## Where it stands

`slit::divide` (`crates/topo/src/boolean/slit.rs:1034`) divides the
edge where it crosses the face's loops at a crossing a record names,
read where the two lines meet (`geom_brep::curve_pair::meet`): at an
edge of the loops (an edge-edge record; both edges split onto the
meeting's point), or at a vertex of them (a vertex-on-edge record; the
edge alone split, onto the vertex's point). Each crossing's record
becomes a vertex-vertex record of its two vertices, and a
vertex-on-edge record of a divided edge moves to the piece holding the
vertex, ordered along the line by `BooleanDecision::PartingOrder`.
`riders` (`slit.rs:599`) reads an end tied by a vertex-vertex record
to a vertex of the face's loops as on the face, and `part`
(`slit.rs:849`) joins each such end to that vertex with a zero-length
joint (`zip::fuse_by_joint`, through `zip::Joint::between`): one vertex
per crossing, the material there one cone. The fusions ride on
`BooleanNaming::vertex_merges`.

Built, sound against the closed forms at tiers 3 and 3′:

- the bridge, every order (`three_solids_on_one_line.rs`, `every_op`, and
  `the_line_less_the_moved_bridge_passes_three_prime_with_its_records`);
- the half-turn partners and the void's-face pyramids, every pose, every
  germ row read (`PARTNERS_SLIT_GAPS`; `a_vertex_read_by_two_sector_passes.rs`
  `REFUSING` no longer names them);
- `review_4831_probes.rs`,
  `p12_a_plane_through_the_middle_of_a_diamond_holes_edge`: `p12` with a
  diamond hole, `A ∖ B` = 28, the wall divided in two;
- `crates/sweep/tests/an_edge_running_past_a_face_reads_its_crossings.rs`:
  a crossing at a shallow angle builds or refuses typed at every ε, and
  a boundary edge on the edge's own line refuses `Overlap`.

A shallow crossing's lever is honest but long: a boundary line at angle
θ to the edge stays within the band of it for about `2·band/θ` either
side of the crossing, so moving the crossing along the edge walks
through escalations before it builds. At θ = 1e-4, ε = 1e-6, the
crossing at `z = 0.6` refuses `Containment`, at 0.64 vertex-on-face, at
0.7 `Containment` again, and builds at 0.749 and 0.75. Moving it out of
the face instead (0.3) ships the rider form at 1e-9 and 1e-12
(`an-edge-lying-partly-in-a-face-ships-the-rider-form-where-its-midpoint-is-off-the-face`).

Carried elsewhere:

- `p12` refuses `Tangent`: the hole's wall is tangent to the pocket's
  wall along the ruling. Its `A ∖ B` target, `30 − π`, is
  `the-boolean-parting-refuses-its-rarer-configurations`'.
- The line body less the 0° prism against the bisector refuses
  `Unplaced`: the divided edge overlaps the line body's own coincident
  edges (`an-edge-running-past-a-face-along-coincident-edges-is-not-divided`).
- An edge running past a face whose midpoint the face does not hold is
  not read at all
  (`an-edge-lying-partly-in-a-face-ships-the-rider-form-where-its-midpoint-is-off-the-face`).
