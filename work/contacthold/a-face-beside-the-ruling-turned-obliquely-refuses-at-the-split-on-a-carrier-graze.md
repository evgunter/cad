---
id: a-face-beside-the-ruling-turned-obliquely-refuses-at-the-split-on-a-carrier-graze
kind: issue
title: A brick beside a tube's ruling, turned off the world's axes, passes the door and refuses split_conic_belly_graze on the carriers
status: open
opened: 2026-10-13
priority: P2
---



## What

A brick slid beside a tube's tangent ruling, its top `5.5·ε` below the
wall's carrier: aligned with the world's axes every op builds (PR 4918's
face-clear test settles the glue door's in-band reading). Turned 0.7 rad
about `(1, 2, 3)`, with `dx = 0.3` and `0.05` (faces about 0.044 m and
1.2·10⁻³ m apart), the door still answers no tangency, and every op in
both orders then refuses `Escalated(Coincidence(EdgeOnPlane))` on
`split_conic_belly_graze` at the split stage. The same happens on PR 4918's
base.

This is the same class as PR 4918's first review MAJOR: a carrier reading
taken before anything asks whether the face or edge reaches the locus.
Here it is the split's conic-belly graze, which reads the brick's top
carrier against the wall's on boxes that meet once the pose is turned. A
`pair_extent(` grep cannot see it.

Pose: `crates/sweep/tests/review2_glue_door_probes.rs`
`r2_brick_beside_the_ruling_turned_oblique` on branch
`contacthold/glue-door-escalates-review-2` (af2af2227d).
