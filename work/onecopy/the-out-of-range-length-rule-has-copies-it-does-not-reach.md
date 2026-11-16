---
id: the-out-of-range-length-rule-has-copies-it-does-not-reach
kind: issue
title: geom_core::length_out_of_format is the one out-of-range rule, but sector_shape, injective and four plain-norm guards still spell it by hand, and each ported guard takes the norm twice
status: closed
opened: 2026-11-09
priority: P3
cost: E
closed: 2026-11-09
pr: 6628
---


Filed from PR 6575's DELTA review.

## Copies the rule does not reach

- `crates/geom-brep/src/sector_shape.rs:362–373` `sector_arm`: a full hand copy of the two checks (`is_finite_length`, then `is_underflowed_length` against `norm_witness`), mapped to `SectorFault::NonFiniteChord` / `UnderflowedChord`.
- `crates/topo/src/injective.rs:677` `unit`: guards on the PRESCALED length, so it still admits directions past 2^511 and below about 1e-162 that main refused. f64-only, low stakes, but it contradicts "every door that refuses a direction reads it".
- Same guard on a plain norm, so the behaviour is equivalent, but these are copies: `step-import adopt.rs:1347` (the plane normal), `viewer combine.rs:1757` `width_along`, and likely `ssi/march.rs:1532` and `curves/compose.rs:503`.

## Also

- **The norm is taken twice.** Each ported guard reads the norm twice: once in `length_out_of_format`, once in `direction_and_length`. A `direction_and_length` that returns the refusal (or a `checked` form) would read it once.
- **The 2-ulp test's comment is false in general.** `direction_and_length_are_within_two_ulps_at_every_binade` claims the pair is within 2 ulps of the scaled in-range pair. 62 of 7.8M Vec3 draws are at 3 ulps; one counterexample's bits are x=10379168357661013866, y=10296199369991661344, z=10343357683394100397. Against the true values, the length error is ≤ 1.51 ulps, which is the doc's claim, and holds. Fix the test's comment, or bound it against the truth.
- **`camera.rs` overflow message.** `CameraError::NotFinite { value: len }` now carries the finite prescaled length.
- **`prescaled`'s `scale` closure is never varied** by its three callers.
