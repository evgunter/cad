---
id: the-census-edge-face-lane-escalates-a-plane-residual-at-an-end-far-outside-the-face
kind: issue
title: The census's edge-face and vertex-face lanes escalate a plane residual in band before reading the face's region, so an end 0.58 outside the face escalates pm_census_ef_residual
status: closed
opened: 2026-10-08
priority: P0
cost: M
refs: [4335]
closed: 2026-10-17
---

## What

Found by JOIN's near-tangent census measurement (`near-tangent-boolean-results-ship-with-an-escalated-tier-3-census`, its `## Measured`), on main `047d10d5`, release.

`crates/topo/src/census.rs` `pair_edge_face` decides
`pm_census_ef_residual` at both ends of the edge against the face's PLANE
and pushes any escalation before anything else is read. `pair_vertex_face`
does the same with `pm_census_vf_residual`. In near-tangent results, an end
lies within the band of a face's plane but far outside the face:

- 177 escalations at ε = 1e-9, all at d = ±1e-8. Every end is 0.576 from
  the face's region, its projection outside.
- 195 at ε = 1e-12 (d = ±1e-11) and 197 at ε = 1e-6 (d = ±1e-5, ±1e-7),
  the same shape.
- Witness: `vee300 nt e0 a0 d1e-8 pc U`, `EdgeKey(44v1)` against
  `FaceKey(20v3)`. The residual reads 5.79e-9 (exact the same). The end
  is 0.576 from the face.

The margin is exact; the plane is the proxy. Where the end really lies in
band of the face's region (`w345 nt e2 a6 d±3e-8`, 9.55e-9, projecting
inside), the pair is a real sliver:
`a-near-tangent-vertex-lands-within-the-band-of-a-face-and-ships-unrecorded`.

Repro: `NT_DUMP=1 cargo run -p sweep --release --example near_tangent_census_probe | python3 scripts/oracles/near_tangent_census_classify.py`, with `NT_ONLY`/`NT_POSE`/`NT_D` to pick the pose and `CAD_TOLERANCE_EPS` the row. The classifier reproduces each census margin bit for bit in f64 and recomputes it at 60 digits on the same coordinates.

## The shape to give

Before escalating an end's residual, read whether the end can lie on the
face at all: its distance to the face's region, or the face's
containment of its projection at K·ε. An end definitely outside the face
by more than K·ε cannot make a pierce or an overlap there.

## Closed

`census::pair_edge_face` and `pair_vertex_face` now read a residual
left undecided by the band through `census::read_undecided`. It runs the
lane under each reading the residual can take, and escalates only where
one of them finds something. An undecided edge end reads `Zero` or the
side its residual lies on (`undecided_signs`, read at the least band),
and all three only where that side cannot be read. Each such reading is
run through the lane: the pierce test, the overlap lane, or nothing.
For a vertex, the on-plane branch is run (containment, then the
declared rungs). The off reading finds nothing, so "some reading finds
something" is exactly "the readings differ". The refusal carries every
branch's refusals and defects, but no contact found under a
hypothetical reading.

Under opposite signs, the residuals' difference is decided first
(`pm_census_ef_pierce_lever`). An end on a strict side lies past the
band's zero, so a difference read `Zero` rules the signs out, and one
read definite places the crossing. Decided residuals take the path they
did before.

Measured on the probe's default grid, `pm_census_ef_residual` falls
from 266 to 3 at ε = 1e-9 and from 562 to 4 at 1e-6. The few that
remain are pairs whose overlap branch itself escalates, for example
`asym nt e2 a4 d-1e-9 pc I`, whose branch meets a `bool_contact_vertex`
escalation. Most of the 266 had stayed only because the overlap
branch's crossing reading escalated `curve_pair_pose_offset` at a
straight boundary edge sharing the edge's end. That is fixed in the same
PR (see the edge-face-cut row's `## Closed`). Pinned by
`census::tests::a_plane_residual_in_band_escalates_only_where_the_face_can_be_touched`
(a diamond 5e-9 above a cube's top, clear of it and over its corner)
and `sweep`'s `near_tangent_census_edge_face_lane`.
