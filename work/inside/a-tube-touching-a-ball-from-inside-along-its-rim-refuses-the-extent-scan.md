---
id: a-tube-touching-a-ball-from-inside-along-its-rim-refuses-the-extent-scan
kind: issue
title: A tube inside a ball whose rim lies on the sphere crosses nothing, and the extent scan refuses it
status: open
priority: P1
cost: M
design: true
opened: 2026-10-09
refs: [4399]
---

Found by JOIN's tube-on-a-ball lane (branch `join/tube-ending-on-a-ball`),
probing the tube that ends on the ball from inside.

## What

A `z`-axis tube of radius `a` inside the ball of radius `√2` about the
origin (`sweep::test_support::ball_poled_z`), with one or both rims on
the sphere: `z ∈ [−z0, z0]` or `z ∈ [0, z0]`, `z0 = √(2 − a²)`, for
`a ∈ {1, 0.7}`, upright or tilted. Every op in both member orders
refuses `FallbackExtentUnsupported { "the sphere's section circle runs
near the plane face's boundary …" }` at every ε row.

The tube touches the sphere along its rim and crosses nothing, so the
reduction mints no null pair and the no-crossings path takes the pose.
Its sphere extent scan (`boolean::ops::sphere_extent_scan`, the
`Surface::Plane` arm) cuts the sphere by the tube's end disc's plane,
which gives the disc's own boundary circle, and `face_boundary_meets`
reads it as running near the boundary, as it is.

The tube standing on the ball from outside (`z ∈ [z0, z0 + len]`)
reaches the crossing layer as an ON event and builds every op
(`crates/sweep/tests/a_tube_ending_on_a_ball.rs`).

## Why it is not a one-line fix

`tube ∪ ball` is the ball and `tube ∩ ball` the tube, but `ball ∖ tube`
pinches along the rim: the material on either side of the tube's wall
meets the sphere at the rim circle only. So a fix decides what the
boolean does with a touch whose difference is a pinch along a curve,
which is contact semantics, and the D10 hold
(`work/intent/d10-one-way-to-say-intent-is-unbuilt.md`) may gate it.

## Witness

`crates/sweep/tests/a_tube_ending_on_a_ball.rs`'s probe family
(`the_probe_family_ships_no_wrong_body`) carries these poses as
`inside, both rims`, `inside, one rim` and `inside, one rim tilted`, at
radii 1 and 0.7. It asserts only that they build sound or refuse typed.
A tube that ends on a half ball from inside, its wall leaving through
the flat face, crosses something and builds (the same family).
