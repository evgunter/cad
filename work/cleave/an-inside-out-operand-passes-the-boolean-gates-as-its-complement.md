---
id: an-inside-out-operand-passes-the-boolean-gates-as-its-complement
kind: issue
title: An inside-out operand passes the Boolean's operand gates and is consumed as its complement
status: open
opened: 2026-10-03
priority: P1
cost: M
---


## What

A prism built over a clockwise profile (`test_support_fixtures::prism_z`
asks for counterclockwise) is a closed, tier-1 and tier-2 valid body
whose faces all point inward: tier 3's check 7 refuses it as
inside-out. `reduce::gate_operand_pairs` (`crates/topo/src/boolean/reduce.rs`)
gates Boolean operands on tiers 1 and 2 only, so the Boolean consumes
such a body as the complement of the region it bounds, and answers.

Measured on main 69700de2c, `Tol::witness()`, brick
`(0,1)×(0,1)×(0.5,1.5)` against `prism_z` over the triangle (0,0),
190°, 80° (unit radius, clockwise), z ∈ (0.5, 1):

- `subtract_with` returns volume 0.0352 and `intersect_with` 0.9648:
  the brick ∩ the wedge and the brick minus it, swapped, each a body
  that looks like a plausible answer;
- `union_with` refuses `ResultVolumeImplausible` (−0.1997), which is
  the volume backstop catching it, not a gate.

Found by `fuse/shared-vertex-crossings`: the row
`work/fuse/a-vertex-crossing-both-sides-of-a-pinch-refuses-shared-vertex-crossings.md`
gave its witness wedge in this clockwise order, so the refusal it pinned
was reached through an inside-out operand (the counterclockwise wedge
refused the same way on main, so the row itself stands).

## Owed

Decide whether an inside-out solid is a legal Boolean operand. If not,
refuse it at the gate typed (a tier-3 check-7 verdict per operand, or a
cheaper signed-volume reading), before any classification reads it; if
so, say what the Boolean means by one. Either way, pin the ∖/∩ answers
above, which today come back silently.
