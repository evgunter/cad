---
id: the-capsule-rod-ending-on-the-joint-parts-by-member-order
kind: issue
title: The capsule unioned with a coaxial rod ending on its joint builds in one member order and escalates in the other
status: open
opened: 2026-10-10
priority: P1
cost: M
---


The one-profile capsule of
`crates/sweep/tests/strut_cover_on_cylinder_pairs.rs` (a tube
`z ∈ [0, 2]` of radius 0.5 with a half ball on it) is unioned with a
coaxial rod `z ∈ [−1, 2]` of its radius that ends on the joint circle,
the rod's wall declared a continuation of the tube's
(`the_capsules_strut_waits_at_the_crossing_layer`). The two member
orders give different results:

- **rod, capsule** builds, and its volume is the closed form to 1e-12;
- **capsule, rod** refuses `Escalated`, predicate `bool_wedge_reflex`,
  at a margin of 1.6e-16.

On main before INTENT stage 4 PR E, both orders refused
`CurvedPierceUnsupported`. Undeclared, each order now gives the
declared order's outcome.

A union decides alike in every member order (REFERENCES DM4), so the
escalation is the defect to root-cause. The joint's sector
coincidence reads a wedge as reflex at a margin of zero in one order
and not in the other. The pin admits both outcomes by name until this
issue lands.
