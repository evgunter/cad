---
id: a-missed-ray-reads-the-solids-volume-sign-for-its-side
kind: issue
title: A containment ray that misses the body takes its side from the solid's volume sign (margin V/A), so a definite thin body refuses as a disjoint operand
status: dispatched
opened: 2026-10-11
priority: P1
cost: M
branch: cleave/missed-ray-side
---


Found by FLUSH's thin-hollow design fork (fork log row 113). Both
designers measured it on main at ε = 1e-9.

## Measured

The slab `brick((0,1),(0,1),(0,1.5e-8))` is built at rest. Every pair
is definite, and it passes tier 3 and the census.
- `slab ∪ brick (5..6)³`, either order, and `slab ∪` a brick beside it
  refuse `ShellWitnessExhausted`: 26 of 26 witnesses read in band at
  7.5e-9.
- `point_in_solid(slab, (5.5, 5.5, 5.5))` refuses `Escalated`; the
  predicate is `bool_point_in_solid_infinity`.
- `big ∪ slab` builds, where the slab is absorbed and its sign never
  read.

## Cause

`boolean/solid_contain.rs` `at_infinity_side`, reached when a ray meets
no face, takes the far side from the solid's signed volume. Its margin
is "scaled to a mean thickness (V / surface area)", 7.5e-9 here, which
is in band. A ray that hits the slab would decide: its two crossings
are 1.5e-8 apart, and the nearer one's face orientation says which side
holds material.

## The shape to give

On a ray that misses, cast a ray at an interior point of one of the
body's faces and read the sign of the closest crossing. Keep V/A only
as a fast path where it is definite.
`a-built-sliver-is-not-a-legal-operand` (parked here) has the same
symptom on an unmeasurable sphere lens, and its hypothesis section is
superseded by this row's site.
