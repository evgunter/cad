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
oracle is the first plate's volume, 24 − (4 − π)/4.

After `origin/main` was merged into PR 3657 with CLEAVE's #3716, the
subtract and intersect of these poses no longer refuse at the join.
Five of the six build at the oracle, half the thick plate's volume,
valid at tier 3 and 3′. The flush-top intersect refuses
`ResultVolumeImplausible` on a two-ulp rounding tie
(`work/reach/volume-backstop-refuses-a-closed-form-rounding-tie.md`).
The union still refuses `FallbackExtentUnsupported` in all three poses,
and so does the stacked pose's subtract and intersect (the repro above,
re-measured on the same merge). Pinned by
`declared_rounded_continuations_inside_a_wall_build_subtract_and_intersect`.

## Operand order (`reach/door-backstop`)

`declared_rounded_continuations_inside_a_wall_build_subtract_and_intersect`
now runs both operand orders, with the declarations keyed for each
order (`findings(&b, &a)` for (B, A)). For the three in-wall poses
(sunk, flush top, flush bottom), at ε = 1e-9, 1e-6 and 1e-12:

- `A ∪ B` refuses `FallbackExtentUnsupported` (no crossing event,
  near-tangent carriers).
- `B ∪ A` BUILDS the thick plate at its oracle, `24 − (4 − π)/4 =
  23.785398163397448`. The measured volumes are 23.785398163397442,
  …463 and …456; tier 3 and 3′ are clean. The result keeps its walls
  split where the thin plate's lay: 18 faces sunk, 14 flush, against
  the plate's 10.
- `B ∖ A` (empty in truth) refuses `FallbackExtentUnsupported` as
  `A ∪ B` does.

So the extent pass refuses in one operand order and not the other.
Whatever closes this item should build `A ∪ B` as `B ∪ A` does. The
split walls of `B ∪ A` are a separate question: a result that is not
maximal.
