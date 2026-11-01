---
id: axial-door-refuses-a-one-surface-seam-corner
kind: issue
title: the axial offset door refuses a one-surface seam corner: the full-period torus, solid and hollow alike
status: closed
opened: 2026-09-08
closed: 2026-09-08
refs: [SHELL-7, SHELL-5]
---


Measured in SHELL-5 (PR 2159,
`crates/sweep/tests/verbs_shell.rs`,
`the_full_period_torus_refuses_at_the_axial_doors_seam_corner_solid_and_hollow_alike`):
`shell` of the full-period torus from `tube_along_arc` (`R = 2`,
`r = 0.5`) and of its hollow twin from `tube_along_arc_hollow` (wall
`0.125`) both refuse at the axial door's corner solve,
`ReplaceFaceError::TogetherAxialCorner { surfaces: 1, .. }` — "one
profile constraint meets here and the vertex is not on the axis, so
its station is determined but its radius is not" — at the same face
and vertex (`FaceKey(4v1)`, `VertexKey(1v1)`, on the first shell). The
tube's seam vertex is where one surface meets itself, so the meridian
solve (`crates/topo/src/offset_axial.rs`, the corner arm of
`offset_charts_together`) has one constraint for two unknowns. The
refusal is the door's, not the hollow operand's: the solid torus takes
it identically.

The closed form waiting on it, for the hollow torus at `t`: two thin
tori, `2π²R[(r² − (r−t)²) + ((r−w+t)² − (r−w)²)]`; the row above
carries it and flips to asserting it the day the door takes a
one-surface seam corner (a torus seam vertex's radius is the torus's
own minor radius offset by `d`, which the corner solve could read from
the single surface's meridian circle instead of intersecting two).

## Closed

Closed by SHELL-7 (`docs/SHELL-7-SPEC.md`, branch `shell/7-seam-corner`).
A vertex all of whose incident faces lie on one surface of revolution
is a point of that surface and moves under the surface's own offset:
`Profile::image_of` in `crates/topo/src/offset_axial.rs` gives the
concentric point on a profile circle and the perpendicular foot on a
profile line, and `solve_corner`'s single-profile branch carries a
corner through it when no plane contains the axis there (the seam
corner) as well as when one moved cap does (the rim corner, which
already carried the same arithmetic); the azimuth is carried from the
old vertex as every seam's is. `mint_carrier`'s torus seam arm tells a
LATITUDE seam (centre on the axis: the standard latitude rule) from a
MERIDIAN seam (centre on the tube-centre circle: concentric) by where
the seam's centre stands, and `restate` re-authors a revolved point's
declared rotation family from the moved corner. Measured on the full
torus from `tube_along_arc` (`R = 2`, `r = 0.5`): two faces on one
torus, two meridian half-circle seams about `(2, 0, 0)`, two equator
seams about the axis at radii `2.5` and `1.5`, two vertices each with
exactly one distinct surface. Both bodies now shell: the solid to
`2π²R[r² − (r − t)²]`, the hollow twin to the closed form this item
carried, two solids and four shells, `[Outer, Void]` each
(`crates/sweep/tests/verbs_shell.rs`,
`the_full_period_torus_shells_solid_and_hollow_alike`;
`crates/sweep/tests/shell7_seam_corner.rs` for the corner's own
closed forms and the refusal that remains).
