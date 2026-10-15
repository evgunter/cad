---
id: a-wedge-across-a-box-tangent-to-a-wall-refuses-sector-straight-in-one-slot
kind: issue
title: A wedge tool across a box resting tangent to a cylinder wall refuses sector_straight in the tool-first slot where the line-first slot builds
status: open
opened: 2026-10-15
priority: P3
cost: M
---


## What

Found while probing the curved half of
`a-vertex-minted-on-an-edge-lying-on-a-curved-face-is-unread`.

- **Line body:** the cylinder r = 1, z ∈ [0, 2], unioned (cylinder first)
  with the box x ∈ [1, 1.5], y ∈ [0, 0.5], z ∈ [0.5, 1.5]. The box's
  face x = 1 is the wall's tangent plane along the ruling at (1, 0), and
  the box's edge x = 1, y = 0 lies on that ruling.
- **Tool:** the wedge prism with its apex edge on the ruling, sides 0.5
  long at 60° and 120° from +x, z ∈ [0.2, 1.8]. This is the fixture in
  `crates/sweep/tests/a_minted_vertex_on_a_curved_face.rs` (`wedge`).
  The mirror pose answers the same: box y ∈ [−0.5, 0], tool at
  −120°..−60°.

**Subtraction:** the line body minus the tool builds at its closed form
(volume 6.414761), tier 3 clean. The tool minus the line body refuses
`Escalated { decision: LineWedges, predicate: "sector_straight" }`,
margin 1.0, rejected `{ sign: Positive, passes: Negative }`. That is a
corner read as folding back on itself, not a reading in the band.
Intersection and union build in both slots.

On main (before the curved half), the tool-first slot refused
`JoinDesync` in all three ops; the curved half's reads clear the join,
and this is what is left.

## Owed

Find why the tool-first slot's line wedges read a folded corner where
the line-first slot's do not, then make the two slots answer alike.
