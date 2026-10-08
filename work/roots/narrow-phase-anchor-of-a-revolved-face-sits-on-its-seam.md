---
id: narrow-phase-anchor-of-a-revolved-face-sits-on-its-seam
kind: issue
title: The boolean narrow phase anchors a face at the mean of its boundary vertices, which for a revolved face sits on its seam, so two curved faces may find no separating direction
status: open
opened: 2026-10-06
priority: P2
cost: M
---


Found building `boolean::separating` (the operand-gate pose lane,
`boolean-operand-gate-separates-only-along-world-axes`). Unmeasured:
no fixture has been built for it.

## What

`separating::apart` proposes two kinds of separating direction: the
axis between the two items' anchors, and every planar face's outward
normal on either operand. The anchor is the mean of the item's
boundary vertices. A full-turn revolved face has its vertices on its
seam meridian, so its anchor sits on the seam rather than near the
face's centre, and the axis between two such anchors can point well
off the direction that parts the two faces. Where neither operand has
a planar face that separates them (a cone wall beside a sphere, two
torus walls), the pair then keeps its world-axis verdict, and the
gate, the sweep's curved arm and the section walk refuse it at the
poses where the world boxes overlap.

## The general form

An anchor that turns with the face and sits inside its reach: the
centre of the face's reach in its carrier's own frame (the axis frame
of a cone, cylinder or torus, the centre of a sphere), or the carrier's
own axis offered as a candidate direction. Measure first: a cone wall
beside a ball, clear, swept through poses as
`crates/sweep/tests/operand_gate_pose.rs` sweeps the cone and the bar.

## Measured, 2026-10-06 (PR 4122's review)

The review of PR 4122 (`analysis/reach-review/4122`, `review.md`
NOTE-1, `probes/review4122_probes.rs`) measured the refusals this item
predicts. Each one is uniform across poses, so the verdict is the
pair's; what it lacks is a separating direction.

- A bar whose edge sits 2%·s off the frustum's cone wall (gap
  0.019·s) refuses at every pose.
- The 270° cone refuses at the cut, at every pose.
- Plates set 1e-3·s off the support of the 270° frustum, along 16
  oblique directions at three scales, refuse 30 of 120 runs.

In every case the narrow phase finds no planar normal and no anchor
axis that parts the pair.
