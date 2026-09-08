---
id: partial-cone-frustum-three-quarter-turn-refuses-edge-disagreement
kind: issue
title: a three-quarter-turn partial revolve of a cone frustum refuses TogetherEdgeDisagreement at its wall/cap generator
status: open
opened: 2026-09-08
---


Measured by the SHELL-7 R1 review lane (`crates/sweep/tests/shell7_r1_diff.rs`,
`wedge 3/4 turn frustum`) and pinned in
`crates/sweep/tests/shell7_seam_corner.rs`,
`a_three_quarter_turn_cone_frustum_refuses_edge_disagreement`: the
cone frustum `(0,0) → (1,0) → (0.5,2) → (0,2)` revolved
`Partial(3π/2)` and shelled at `t = 0.05` refuses
`ReplaceFaceError::TogetherEdgeDisagreement` at the axial door
(`crates/topo/src/offset_axial.rs`, `param_on`'s endpoint meter) with a
gap of about `1.34e-3` m, on the merge base and on SHELL-7's head
alike. `sf2b_axial` only turns its frustum a quarter, so no shipped row
saw this. The shape: a partial revolve of a cone with two meridian
caps, whose wall/cap generator is a LINE between a cone and a moved
cap; the line carrier is moved perpendicular to itself from its start
corner, and the two corner solves (each `[cone, cap, station plane]`
against the moved surfaces) do not agree about the edge between them
by a millimetre. Whether the corner solve or the line carrier's mint
is the one that is wrong is not measured here; the refusal is loud,
no wrong body passes, and the row keeps it visible.
