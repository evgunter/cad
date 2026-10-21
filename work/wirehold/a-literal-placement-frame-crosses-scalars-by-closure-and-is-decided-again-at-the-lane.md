---
id: a-literal-placement-frame-crosses-scalars-by-closure-and-is-decided-again-at-the-lane
kind: issue
title: A literal placement frame crosses to the lane scalar through Affine3::map and is decided rigid again there; the frame witness has an exact embed door now
status: open
opened: 2026-10-14
priority: P3
cost: M
---


Filed by PATHS' `the-sketch-plane-is-its-frame` unit (the off-question
its unit file names), which retired `SketchPlane::map`/`try_map` and
gave the frame witness its exact crossing, `RigidMotion::embed`
(`f64` to any scalar through `Real::from_f64`; `embed_computed` for a
computed frame).

## What

`editor-core/src/placement.rs`: `Frame::affine<T>` reads a stored
frame at the backend scalar as `self.affine_f64().map(T::from_f64)` —
the closure lift the sketch plane just gave up. Its rigid readers then
decide the lifted map rigid AGAIN at `T`:

- `Frame::rigid_motion` (a literal `Placement` step):
  `RigidMotion::check_rigid(&self.affine(), band)`;
- `eval/wire.rs`, the transform node's listed frames:
  `RigidMotion::check_rigid(&f.affine::<T>(), band)`.

`Frame::admission_fault` already decided the same frame rigid at `f64`
(`check_rigid(&self.affine_f64(), band)`) when the document admitted
it. So a literal frame is decided twice, once per scalar, and the lane
decision runs on a map that carries the `f64` rounding as exact (at
`Interval`, point intervals of the `f64` components: the seven F10
residuals are the `f64` residuals, enclosed).

## Why it did not fall out of the unit

`Frame` is a general map — it may be improper, and `Frame::compose`
and `Motion::Map` keep it general — so `Frame::affine` cannot become an
`embed`; only the rigid readers could. The shape a fix would take:
decide once at `f64` and carry the witness,
`RigidMotion::check_rigid(&self.affine_f64(), band)?.embed::<T>()`, the
way the sketch plane's `Pinned` lift carries its `f64` frame. That
moves which scalar the literal step's K samples are recorded at (the
F10 rows at `T` go, the `f64` ones stay), and whether a lane should
re-decide a literal frame is the question this row asks, not a
re-spelling: the guided lane re-decides everything at its own scalar
by design (`ProfileLift`'s doc), and a literal placement has no
`Pinned`/`Guided` distinction of its own today.

## The shape above is no longer available (2026-10-14, Ev's ruling A on #5068)

`RigidMotion::embed` is gone: a frame crosses no scalar upward, and
`ProfileLift` (with its `Pinned` lift) retired, so every lane decides
at its own scalar. A fix here therefore cannot carry the `f64` witness
up. What remains is whether `Frame::affine`'s lift by
`affine_f64().map(T::from_f64)` and the rigid readers' re-decision at
`T` is the right reading of a literal placement, which by the ruling's
rule is a frame minted at `T` from its stored constants.

## Opened (2026-10-21)

`transform-retires-into-a-placement` closed and nothing else gates
this row. The lift stands after it: `Frame::affine` in
`editor-core/src/placement.rs` still reads `affine_f64().map(T::from_f64)`,
and `eval/wire.rs` still decides a listed frame rigid at `T` with
`RigidMotion::check_rigid(&f.affine::<T>(), band)`.
