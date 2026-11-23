---
id: rate-reads-an-axis-length-the-unit-axis-type-retired
kind: issue
title: Psi::rate divides by an axis's length and square_to takes any length, though since PR 7349 a carrier's axis is a UnitVec3
status: open
opened: 2026-11-23
priority: P2
cost: E
---

Found by EMBED's one-plane lane merging main `9f2ed1df15` into the first
landing (`embed/wave3-a-main2`, PR 7491).

## What

PR 7349 (`onehome/carrier-directions`) made `Implicit::{Cylinder, Cone}`'s
`axis` a `UnitVec3<Interval>`. At `Interval` no door of `UnitVec3` admits
a vector off unit: `UnitVec3::checked` keeps a box only where its bracket of
`v·v − 1` holds zero and refutes it otherwise, and `squared` and `new`
renormalise. So no non-unit axis reaches `Psi::rate` (`crates/topo/src/injective.rs`).

`Psi::rate` still reads as if one could: the cone's axial term divides by
the axis's length (`axis.get().dot(v) / axis.get().norm()`), and `square_to`
divides by `|axis|²` so that it holds for any length. Both are sound for a
unit axis and were left as written in the merge. Nothing can now exercise
the division, so it is a guard with no pin.

The merge retired the four rows that pinned it (all iso-end's, PR 7327,
commits `3fe86d6a91`, `b8c1103e34` and `3b119f8c3a`), each with a comment
where it stood:
- `a_carriers_rate_along_its_ruling_reads_tight_on_a_tilted_axis_near_and_far`:
  its sub-case about an axis twice as long;
- `a_carriers_rate_encloses_every_members_rate_where_the_axis_is_a_bracket`:
  its sub-case about an axis enclosure holding zero;
- `a_cylinders_rate_about_a_long_axis_is_its_values_derivative`: the row
  (scales 2, 1.001 and 0.5);
- `a_cones_rate_about_an_axis_off_unit_by_the_convention_holds_its_truth`:
  the row (`|a| = 1 + 5e-11`).

## Owed

Either remove the division by the length and `square_to`'s any-length
form, with a mutant run first to show nothing reads them, or pin them with
the widest axis the type admits: a bracket that holds a unit vector and
is as wide as `checked` keeps.
