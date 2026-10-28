---
id: a-thin-torus-lattice-is-past-the-off-lattice-aspect-bound
kind: issue
title: a torus with R/r above 7 meshes a UV lattice past the 2√2 aspect the off-lattice lemma needs, so a split rim or meridian could put a triangle across two cells
status: open
priority: P3
cost: M
opened: 2026-10-26
---

Found in PR 5717's fix pass (PRED), while sizing the sphere. The
off-lattice lemma at `mesh::sizing::sphere_grid_steps` concerns a
constrained Delaunay triangle on two adjacent lattice points and one
boundary point between columns. That triangle stays within one cell
only while the lattice's UV aspect is under `2√2`. The sphere now holds
its aspect to √2, which leaves half of that bound to the `ceil`.

The torus does not hold its aspect. `torus_grid_steps` sets
`h_v/h_u = √((R + r)/r)`, which passes 2√2 once `R/r > 7`; the tour's
`tube_along_arc` and `hollowring` scenes sit between 4 and 7. So far
the torus has needed no such bound, because its rims and meridians are
chorded exactly on the grid steps and their own sagitta counts never
exceed the grid's. A boundary chord point therefore lands on a lattice
line unless the side is split across several edges, each chorded on
its own span.

**Not measured.** No row builds a thin torus with a split rim. **What
would settle it:** such a row (R/r = 20, rim split at an irrational
angle) across a δ sweep. If it exceeds δ, apply the sphere's aspect
bound to `torus_grid_steps`, or tighten split sides onto the face's
own columns.

## Moved to TESSSIZE (the curved and trimmed lanes' sizing: which density each lattice emits, and the theorem behind it) (2026-10-28)

Cut out of TESS by its semantic-seam split (`work/README.md`, Track size) when TESS measured 146 budget points against 30. Moved by `git mv`; the id and body are unchanged.
