---
id: a-steep-plane-section-of-a-one-face-wall-meshes-non-manifold
kind: issue
title: A one-face cylinder wall cut by a plane at 85 or 88 degrees, the wrap edge at the ellipse's major-axis end, meshes non-manifold while tessellate returns Ok
status: open
opened: 2026-10-08
priority: P2
cost: M
---

Found by the PR 4345 dual review (r2 NOTE 3; probes `r2_f1s_steep_planes_wide_box`
and `r2_d_mesh_split_control` on `join/wrap-edge-section-loop-review-r2`).
Reproduced on base, where the boolean refused this shape. The boolean
now reaches it (`join/wrap-edge-section-loop`).

## Witness

- The wall: the one-segment circle `r = 1` about the origin, its vertex
  at azimuth 0, extruded `z ∈ [0, h]`, `h = 2·tan θ + 2`.
- The cut: the plane through `(0, 0, h/2)` with normal `(sin θ, 0, cos θ)`,
  so it meets the wrap edge at `90° − θ`, with the seam at the
  ellipse's major-axis end.
- The split lane: `topo::split` of that wall by that plane, on base.

| θ | piece | `tessellate` | `check_mesh` at chordal 2e-2 and 5e-3 |
|---|---|---|---|
| 85° | below | `Ok` | `NonManifoldEdge { count: 4 }` |
| 85° | above | `Ok` | ok |
| 88° | below | `Ok` | `NonManifoldEdge { count: 4 }` |
| 88° | above | `Ok` | ok |

- The boolean: ∩ of that wall with a 120-wide box below the plane gives
  the same mesh defect in both operand orders, and ∖ at azimuth π
  does too. Every body passes tiers 2 and 3′ and the certificate, at
  the closed-form volume.
- The operands, and the two-arc wall's result, mesh clean.

## What is wrong

`tessellate` returns a mesh that `check_mesh` rejects. Either the curved
lane should refuse the piece typed, or it should mesh it.
