---
id: split-gate-reads-a-world-axis-box
kind: issue
title: The split gate reads a world-axis-aligned box, so a cut clear of a sphere face refuses or splits by the body's pose
status: open
opened: 2026-10-02
priority: P1
cost: M
---


Filed by the split-gate lane (PR 3843), from its dual review.

`splitting/classify.rs` `gate_operand` clears an unarmed face behind
its padded reach box (`gate_face_reach`, then `box_clears`). That box is
axis-aligned in WORLD coordinates, so how far a cut must stand clear of a
curved face depends on how the body is turned, not on the geometry.

Measured on PR 3843's capped cylinder (`sweep/tests/reach_split_gate_per_face.rs`
`capped_cylinder`: the unit cylinder `y ∈ [0, 1]` under a sphere cap of
radius 5/4 about `(0, 1/4)`). The cuts are perpendicular to the body's
own axis, `clear` below the rim:

| pose | clear 0.05 … 0.7 | clear 0.8, 0.9 |
|---|---|---|
| upright | splits | splits |
| turned 0.3 rad about `z` (`topo::transform_rigid`) | refuses `CurvedBooleanUnsupported { kind: Sphere }` | splits |

The latitude zone's slab box (`slab_extent` over the cap's axial window)
is exact along the axis when the axis is a world axis. Turned, its
perpendicular widening of `R·√(1 − aᵢ²)` spends the cap's 5/4 radius
on the `y` extent too.

**The general fix** is a box in the carrier's own frame: decide the
plane against the zone in the sphere's axial coordinate (the cap's
support along `n` is a closed form in `n·â` and the window), and likewise
the torus window and the cylinder or cone slab, rather than against a
world AABB. That is the face-box rule's business
(`boxes/split-gate-sphere-zone-folds-into-face-box-rule`), and the gate's
refusal text names the bounding box so that it stays true until then.
