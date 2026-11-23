---
id: vacuous-reads-a-cell-at-a-corner-local-at-every-depth
kind: issue
title: injective's vacuous reads a cell that touches a face's corner from outside Local at every depth, because the touching line's edge is read without running on past the corner
status: dispatched
opened: 2026-11-01
priority: P0
pr: 7322
branch: embed/collinear-cover
---


## Finding

POTDOC's review found this (PR #6172), and it predates that PR.

`crates/topo/src/injective.rs`, `vacuous`, reads a cell outside the face
as holding none of it only where a line the cell touches has an edge
covering the cell's whole side. It takes that edge's range from plain
`edge_ranges` and `covered`, with no run-on past a corner.

A cell beside a face's corner, outside the face, straddles the corner
along the touching line at every depth. For example, at the corner
`(2, 1)` of a rectangle `u ∈ [1, 2]`, `v ∈ [0, 1]`, take the cell
`u ∈ (2, 2 + h)`, `v ∈ (1 − h, 1 + h)`.

The `u = 2` edge stops at `v = 1` inside the cell's side, so the cell is
never covered. It is not read `Vacuous`, and `present_chart` returns
`Local` for it at every depth. A pair whose touching cells keep one such
cell splits to the depth limit and refuses, though the cell holds no
point of the face. That is an over-refusal.

`present_chart`'s clip sides now run on past a corner (`run_on`, read
by `sides_are_the_faces`). Whether `vacuous` wants the same reading or a
corner-aware one is this row's question: the cell there is past the
corner on both lines' non-face sides.

## Instances in the per-solid census (EMBED trace lane, 2026-11-21)

Traced on main `c2a0c41f40` with the at-rest gate
(`embed/gate-measure`) and prints of `vacuous`'s inputs and of
`sides_are_the_faces`' cover (`embed/trace-cone-torus-measure`). Two
shapes, both a torus cell lying outside its face across a line it
touches. Valid bodies, P0 at the gate's landing.

**Two collinear edges on one line, neither covering the cell's side
(13 tests, cylinder/torus).** A fillet band on a disc's or a bore's rim
splits the torus's foot parallel into arcs at the rim's joints.
In `blend_value_decided_rows::a_disc_rim_records_its_joints_and_its_supports`
the last pair is cylinder `3v1` against torus `9v1`; the torus cell at
`u ∈ (4.18777, 4.19084)`, `v ∈ (6.27091, 2π)` lies below its face's
parallel `v = 2π` (`YLo`, face side above), so it is outside every
line, and `vacuous` is asked. The parallel holds two edges:
`ei 2` over `u ∈ [4.18879, 2π]` and `ei 4` over `u ∈ [2.0944, 4.18879]`,
meeting at the joint `u = 4π/3` inside the cell. `vacuous` asks
`covered` of each edge's range alone, and neither covers the cell's
side; their union does. So `present_chart` returns `Local`
(its "outside every near line and not vacuous" arm) at every depth.
`blend_bore_two_rims::each_wall_seam_is_split_once_per_band_and_named_after_its_source`
is the same at `v = π`. Checked against the printed ranges; the fix
(reading the union of the line's edges) is unprobed. All 13 of the
bucket's rows are rims of this kind (`blend_bore_two_rims`,
`closed_chain_junctions`, `rim_of_structural_review_probes`,
`tier_gate`'s import corpus), assigned by family; two traced.
Cheapest: `sweep::all blend_value_decided_rows::a_disc_rim_records_its_joints_and_its_supports` (0.76 s).

**Outside one line and inside the other, at a corner (11 tests,
plane/torus and torus/plane, Boolean results).** In
`contact11_torus_chart_l::a_notched_half_donut_answers_its_volume` the
torus cell at `u ∈ (0, 7.7e-4)`, `v ∈ (0, 6.1e-3)` sits at the chart
corner where the meridian `u = 0` (face side `u > 0`, over
`v ∈ [−π/2, 0]`) meets the parallel `v = 0`, the face below it. The
cell is inside the meridian's half and outside the parallel's, so
`vacuous` is not asked (`inside` holds). The outside line is dropped,
the meridian is the cell's `XLo` clip, and `sides_are_the_faces` finds
its edge does not cover the cell's `v` range (`want = (0, 6.1e-3)`,
`ranges = [−π/2, 0]`): `Local` at every depth. The cell holds none of
the face's interior. The 7 plane/torus Boolean rows and the 4
torus/plane ones are assigned by shape, one traced; unchecked for the
other ten. Cheapest: `sweep::all contact11_torus_chart_l::a_notched_half_donut_answers_its_volume` (1.1 s).

Neither instance is closed by PR 7115's topo diff (checked for the
Boolean one).
