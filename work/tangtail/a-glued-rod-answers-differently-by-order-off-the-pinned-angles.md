---
id: a-glued-rod-answers-differently-by-order-off-the-pinned-angles
kind: issue
title: A rod glued in its bore still answers differently by operand order one step off the pinned angles, and coaxial nested equal rods do too
status: open
opened: 2026-10-15
priority: P3
cost: M
---

Found by PR 5188's review (off-target, pre-existing: identical at its merge-base).

`crates/sweep/tests/bisector_nodes.rs::a_rod_glued_in_its_bore_answers_alike_in_either_order` pins the orders alike at three angles of a family where it does not hold in general. The fixture is that row's: a unit rod against the brick `[−2, 2]³` bored to its radius, ∩ and ∪ in both orders.

## Measured (the reviewer's probe)

- **Default ε, 1e-7°:** both orders escalate on one margin, `1.3593e-9`, but at two decisions: `Containment { Loop(Boundary) }` rod-first and `Coincidence(VertexOnCurvedFace)` bore-first. The row holds this pose to the margin alone and cites this item.
- **Default ε, 3e-8°:** both orders escalate `WallRoots(AxisParallel)`, on different margins: `1.0472e-9` rod-first against `2.0944e-9` bore-first.
- **ε 1.2e-10:**
  - at 2e-9° and 3e-9°: `CurvedPierceUnsupported` rod-first, a `WallRoots` escalation bore-first;
  - at 5e-9°: `VertexOnCurvedFace` on `−1.36e-10` against `WallRoots` on `3.49e-10`.
- **ε 1e-12, 1e-10°:** `Containment` against `VertexOnCurvedFace`.
- **Coaxial nested equal-radius rods, 0 … 1e-8°:** ∩ builds A-first and refuses `FallbackExtentUnsupported` B-first; ∪ the reverse.

## The recourse

A rod glued in a bore of its own radius is the coincidence a shared construction decides. D4(i)'s "make the two one construction (D10)" fits it better than the escalation's "tighten the tolerance below m/K". Following that advice at ε 1.2e-10 just escalates elsewhere (`Containment` on `1.36e-10`), and at 1e-12 refuses `CurvedPierceUnsupported`.

## 2026-10-15 — the 1e-8° poses build alike (FUSE, PR 5154)

The curve-pair reader (PR 5154) replaced `carrier_cross`'s candidate search with `geom_brep::curve_pair::meet`, and with it the end cluster (`bool_carrier_cross_cluster_spread`). Its line×line pose then levered the tilt by the longer span: one order read the rod's whole eight-metre ruling and escalated (`curve_pair_pose_tilt`, `1.4e-9`), and the other read its split pieces and decided parallel. The verdict depended on how the sweep split the edge.

The pose now levers the tilt by the overlap of the two spans' projections, which is where they can meet, and reads the offset inside it. Inside the four-metre block a `1e-8°` tilt moves the rod's ruling `7e-10` from the bore's, within the zero band. Every `1e-8°` pose of `sweep bisector_nodes::a_rod_glued_in_its_bore_answers_alike_in_either_order` (along `y`; about `z` with the seam at 0, π and π/2) now builds alike in both orders, as `1e-9°` does. The row's `builds` flips to true at those three poses that escalated. Its `1e-7°` pose is unchanged: alike outcome, two decisions.

## At ε = 1e-12 (2026-10-19, PR 5156)

With `ellipse_axes_distinct` retired, the rod along `y` at `1e-7°`
no longer escalates alike at ε = 1e-12: ∩ refuses at a pcurve
certificate of the long bisector ellipse in both orders, in the join
(`Pcurves`) in one and in the merge (`Merge`) in the other. The row
admits that pair for this pose; the cause is filed as
`a-sliver-crossings-long-bisector-ellipse-is-served-past-representability`.
