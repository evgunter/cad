---
id: sphere-operand-box-is-the-whole-ball
kind: issue
title: A sphere face's operand box is the whole ball - the same per-kind box class the cone, cylinder and torus arms have left
status: open
opened: 2026-09-06
refs: [torus-operand-boxes-span-whole-ring, 1907]
priority: P1
cost: H
---


## What

`FaceBoxRule::WholeBall` boxes every sphere face as the whole ball
(`boxes.rs`), reading nothing from the boundary — the class the
VERBS-GATE cone clip, the cylinder's `clip_to_boundary` and the torus
window (PR #1907) each retired for their kind. Named-and-deferred in
#1907's sweep with no file; this is the file. A sphere face's chart
window from its stored `Harmonic` images is the torus construction
with one channel (the sphere has no interior critical point in `v`
beyond the poles, which the walk's pole joint already handles).

## Home

CURVED — the operand boxes are the operand-reach lane's.

## Evidence from the split gate (REACH, 2026-10-02)

The split's operand gate (`splitting/classify.rs` `gate_operand`) is
reach-scoped since `reach/split-gate-refuses-a-whole-body-for-one-unarmed-face`,
and the whole ball made it useless for the very fixture that filed it:
a cylinder under a spherical cap (sphere radius 5/4 about `(0, 1/4)`,
cap `y ≥ 1`) cut at `y = 1/2` refused, because the ball spans
`y ∈ [−1, 1.5]`. The gate therefore boxes a sphere face itself
(`classify::gate_face_reach`): when `solid_contain::sphere_chart_trim`
pins a latitude window, the face lies in the zone between its two
extreme latitudes, and the box is `slab_extent` over that axial window
met with the ball. That is a split-local copy of the tightening this
row asks for, now with a side guard (a rectangle's boundary also
bounds its complement). Folding it into `FaceBoxRule` is its own unit,
`split-gate-sphere-zone-folds-into-face-box-rule`.
