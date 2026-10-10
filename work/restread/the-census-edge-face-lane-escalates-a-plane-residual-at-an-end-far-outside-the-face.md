---
id: the-census-edge-face-lane-escalates-a-plane-residual-at-an-end-far-outside-the-face
kind: issue
title: The census's edge-face and vertex-face lanes escalate a plane residual in band before reading the face's region, so an end 0.58 outside the face escalates pm_census_ef_residual
status: open
opened: 2026-10-08
priority: P0
cost: M
refs: [4335]
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
