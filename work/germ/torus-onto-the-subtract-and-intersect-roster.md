---
id: torus-onto-the-subtract-and-intersect-roster
kind: issue
title: Torus onto revert_arm_exists: subtract and intersect still refuse a torus operand at the front door
status: open
opened: 2026-09-25
refs: [torus-operand-gate-admission]
priority: P2
cost: D
---

## What

`reduce.rs`'s `boolean_arm_exists` has a torus now: the union's
crossing layer has its torus arms (circle rung with carrier identity,
line×torus quartic, chart containment, sector and pierce normals).
`revert_arm_exists`, the roster ∖ and ∩ read at the front door
(`ops`), still lists `Plane | Cylinder | Sphere`, so a torus operand
under `Subtract` or `Intersect` refuses before any of those arms runs.
`crates/sweep/tests/mate7a_torus_rest.rs` pins it ("∖ and ∩ keep their
roster verbatim").

What admitting it needs is unmeasured. The seam lane ∖ and ∩ run
(the revert of the other operand and its seam) has not been walked
with a torus operand. Measure first, as the union's doors were.

## Home

GERM, beside `torus-operand-gate-admission`.
