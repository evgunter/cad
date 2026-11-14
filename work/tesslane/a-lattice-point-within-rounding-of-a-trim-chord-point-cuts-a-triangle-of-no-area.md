---
id: a-lattice-point-within-rounding-of-a-trim-chord-point-cuts-a-triangle-of-no-area
kind: issue
title: a trimmed face's lattice point within rounding of a boundary chord point cuts triangles of no area, on faces that mesh on main
status: open
opened: 2026-11-14
priority: P1
cost: M
design: true
---


Found by TESSLANE unit 3 (`a-curved-faces-domain-is-read-from-every-loop`,
branch `tesslane/domain-loops`) while meshing a ringed torus wall.

## What

The trimmed lane lays its lattice (`mesh::trimmed::uniform_candidates`,
`per_cell_candidates`) without regard to the trim polygon, and drops a
candidate only where it lands EXACTLY on a polygon point
(`TrimmedFace::insert_points`, which merges it into the boundary
vertex) or exactly on a side (`TrimmedFace::constrain_boundary`). A
candidate a few ulps from a boundary chord point is neither: the CDT
keeps both, and the triangles between them have no area. `check_mesh`
passes such a mesh, and no deviation certificate sees it; an STL
export of it would refuse `DegenerateTriangle`.

## Measured

- **On main, an outer loop.** `crates/sweep/tests/tilted_sphere_pair.rs`,
  `a_plane_tilted_against_the_balls_chart_builds_under_every_boolean`:
  the box over `x ∈ (0.5, 3)`, `z ∈ (0, 2)` (and over `z ∈ (−2, 2)`)
  against the `y`-poled unit ball. At `δ = 0.04`, A ∪ B's sphere face
  emits a triangle with `2·area/L² = 1.1e-15` and B ∖ A's one with
  `5.9e-15`. The candidate `(−1.0471975511965979, 0)` sits `3e-16` in
  `u` and `1e-16` in `v` from the loop vertex
  `(−1.0471975511965976, −1.06e-16)`: the section circle at `x = 0.5`
  meets the equator at azimuth `π/3`, a lattice column at that `δ`.
- **A ring.** The donut `(2, 0.5)` and the bar B2 of
  `crates/mesh/tests/reach_probes.rs`
  (`a_torus_wall_holding_a_spiric_ring_meshes_on_the_torus_row`): at
  `δ = 10⁻³` the ringed torus faces of A ∪ B, B ∪ A and A ∖ B each
  emit two triangles with `2·area/L² = 1.67e-15` about the spiric's
  extreme chord point `(0, 0.25, 2.433)`. The bar's face `y = 0.25`
  is the tube's latitude `π/6`, a lattice row there, and the point's
  azimuth a lattice column. The row pins it (`Got::NoArea`).

## A fix, and why it is not taken

Measured on that branch and withdrawn: before the first attempt, drop
every candidate within `2⁻⁴⁰` of the trim box's extent, per axis, of a
side of any loop (the band `trimmed::CREASE_MERGE` reads a crease by),
found by a `partition_point` on the `(v, u)`-sorted candidates per side.
It clears the torus ring at every δ, fires on no face of `mesh`'s suite
but by exact coincidence (where the CDT already merges the candidate,
so no byte moves), and fires on the two tilted-pair faces above, whose
bytes it moves. A change to the mesh of a face that meshes on main is
not the unit's to make; it is this row's, with the bytes that move named
(the tilted pair's two faces at `δ = 0.04`, and whatever the D9
goldens, `docs/tess-budget-data` and the tour pins show).

Whether a near-coincident candidate should be dropped, merged onto the
boundary point, or kept off by laying the lattice against the polygon is
this row's decision. A candidate within rounding of a side but away from
its ends is the same class, unmeasured.
