---
id: rounded-stack-subtract-and-intersect-refuse-fallback-extent
kind: issue
title: The rounded two-plate stack's subtract and intersect refuse FallbackExtentUnsupported with every finding declared
status: open
opened: 2026-10-01
priority: P3
cost: M
---

Found by the review of PR 3657, measured on `d2d5b09076`.

## Repro

The rounded two-plate stack (`crates/sweep/tests/reach_continuation.rs`:
6 × 4 plates, corner fillets r = 0.5, z 0 to 1 and 1 to 2), every
finding declared (the mating plane `Rest`, eight continuations):

- union: builds (14 faces, 4 recorded curved skips);
- subtract P − Q and intersect P ∩ Q: refuse
  `FallbackExtentUnsupported` (`crates/topo/src/boolean/ops.rs:1124`
  and its siblings at `:2323` and `:2382`).

The sharp stack's subtract and intersect build with the same
declarations. Whether this is the fallback-extent lane meeting the
coaxial fillet pair (two cylinders on one carrier, touching along the
mating arc) has not been checked.

## The union of a rounded plate whose walls lie inside the other's

Measured on `3aff2e6a07` and after its fix pass: the rounded
6 × 4 × 1 plate and a rounded plate of the same outline 0.5 thick,
sunk inside it (z 0.25..0.75) or flush with its top (z 0.5..1) or
bottom (z 0..0.5), every finding declared (the walls as
continuations). The union refuses `FallbackExtentUnsupported` at
`boolean/ops.rs` `section_extent_pass` (the no-crossings path: the
second plate's edges all lie on the first's walls, so no crossing layer
event exists, and that pass exempts no declared pair by design). The
oracle is the first plate's volume, 24 − (4 − π)/4. Subtract and
intersect reach the join instead and refuse `SectionLoopMixed` at the
chord-midpoint anchor
(`work/zip/role-resolution-interior-tiers-certify-only-planar-region-faces.md`).
Pinned by `declared_rounded_continuations_inside_a_wall_refuse_typed`.
