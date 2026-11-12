---
id: two-cones-apex-lobes-on-two-carriers-refuse-a-valid-body
kind: issue
title: Two cones' apex lobes on two carriers in one self-touch link refuse a valid body
status: open
opened: 2026-11-12
priority: P1
cost: H
---


Found by the remainder-refuses lane (PR 6854), which makes every
self-touch point the reading cannot decide refuse.

## What

`rest::at_apex` (`crates/topo/src/link.rs:4746`) reads a link holding a
cone's apex only where its lobes make one nappe whole (`whole_nappe`,
`crates/topo/src/link.rs:4839`). The nappe must be one spur corner, or
two corners of one carrier. Lobes of two cones' apexes in one link (the
"second apex" class) are no nappe, and refuse `Curved(NoTangentPlane)`.
Before PR 6854 the sweep skipped them; now they refuse
(`sweep_self_touch_points`, `crates/topo/src/census.rs:8888`).

A valid body reaches this. In
`crates/sweep/tests/a_cone_apex_meets_a_vertex.rs`,
`two_cones_through_one_apex_build_across_the_pose_families`, both forms
(tipped and revolved), at default ε and at 1e-12:

- 20/50 near-coaxial, `b ∖ a`;
- 45/30 near-coaxial, `a ∖ b`.

Each refuses `BoundaryCrossesItselfUndecided` at the shared apex
`(0, 1, 0)`, `why: Curved(NoTangentPlane)`. The row now pins that
refusal, citing this file.

## The fix

Read two nappes at one apex: two convex cones `C₁`, `C₂` about one point.
Each is the face itself at every scale its corner reaches, so the link
is decided by how the two cones and the remaining sectors sit, not by a
tangent plane. One possible approach is to hold both nappes off every
plane and off each other, and to read the windings as `at_apex` reads
one. When it lands, the pinned row goes back to asserting pass.
