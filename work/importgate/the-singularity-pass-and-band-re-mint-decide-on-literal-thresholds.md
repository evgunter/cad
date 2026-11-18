---
id: the-singularity-pass-and-band-re-mint-decide-on-literal-thresholds
kind: issue
title: step-import: the singularity pass and the band re-mint decide on literal thresholds
status: open
opened: 2026-11-17
parent: adoption-rebuilds-caches-at-eps-from-eps-in-interpretation
priority: P3
cost: E
---

Found by the dual review of `importgate/interp-layer` (PR 7091).

## What

Decisions on the interpretation path taken on literal thresholds, no
named reading and no band:

- `crates/step-import/src/normalize/singularity.rs`, `crossings` (about
  :838 and :893): `den.abs() <= 1e-12 * g.norm()` decides a ray parallel
  to a rim circle's plane, a dimensionless literal on an unlevered dot.
- `crates/step-import/src/normalize.rs`, the chart-direction reading
  (about :226): `(step.abs() > 0.1).then_some(step > 0.0)` decides which
  way a carrier runs in the chart on a literal tenth of a chart unit.
- `crates/step-import/src/normalize.rs`, the wall-inversion reading (about
  :774): `n_c.dot(axis).abs() <= 0.5` decides whether a rim circle is
  meridional on a literal half.

## Fix

Each through a named reading at the file's band, levered at its arm
(the ray's reach, the carrier's chart speed, the rim's radius), or
re-posed so the choice does not arise.
