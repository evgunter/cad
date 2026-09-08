---
id: axial-door-refuses-a-one-surface-seam-corner
kind: issue
title: the axial offset door refuses a one-surface seam corner: the full-period torus, solid and hollow alike
status: open
opened: 2026-09-08
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
