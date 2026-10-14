---
id: offset-fit-on-locus-fold-drops-a-nan-sample
kind: issue
title: offset fit folds its on-locus samples with f64::max, so a NaN sample never reaches the limb-1 guard
status: closed
closed: 2026-10-09
pr: 4367
branch: encl/offset-fit-nan-fold
opened: 2026-10-01
priority: P2
---


Found by the SSI lever-arm lane's shape sweep (a `min`/`max` fold whose
result feeds a guard with a NaN or indeterminate arm), PR "SSI: the
lever-arm fold propagates poison, from one home". On `f64` the inherent
`f64::max` shadows `geom_core::Real::max`, and only the inherent one
drops a NaN operand; generic `T: Real` code is not affected.

## The site

`crates/geom-brep/src/offset_fit.rs`, `on_locus_cell`
(`m = m.max((fit.eval(u, v) - target).norm())`), and the per-cell fold
that calls it in `measure`
(`on_locus_max = on_locus_max.max(on_locus_cell(..))`). The consumer is
the limb-1 guard in the certifying door,
`!(report.on_locus_max <= tolerance)`, whose negated comparison refuses
on NaN — but a NaN sample norm is dropped by both folds before it gets
there, and the reported bound is the max over the samples that answered.

## Reachable through the certifying door

A NaN fit sample is reachable: evaluators are total by design
(`NetState::Poisoned` in `crates/geom/src/surfaces/nurbs.rs`;
`crates/geom/src/lib.rs`, "Totality and poison"), and `NurbsSurface::new` validates weights, not control
points, so a fit with one NaN control point reaches `certify_offset_at`
and samples NaN. With the folds dropping it, limb 1 passed and limb 2
refused (`cell_bound` maps the non-finite cell to `INFINITY`): loud,
but on the wrong limb with the wrong bound. The certification door is
the designated catch point, not construction.

## Repair

`max_bound(m, ..)` and `max_bound(on_locus_max, ..)`
(`geom_core::interval::max_bound`, `Real::max` at `f64` for the
certification files that may not name `Real`): one line per site, and
the limb-1 guard then refuses the NaN that arrives. `hull_sup`'s fold
beside it reads `cell_bound`, which maps every non-finite to
`INFINITY`, so it cannot see a NaN; it takes `max_bound` too, for one
policy.

## Closed

2026-10-09. PR 4367 merged at `53c0a00cc4` after a review (APPROVE-WITH-FIXES) and a fix pass; hosted CI green.
- The limb-1 folds and `hull_sup` use `max_bound`.
- `patch_regularity`'s sup fold was a live sibling, also fixed.
- `cell_bound` and the measurement row share `Composite::cell_terms`.
- `fit_offset_at` refuses a NaN `on_locus_max` before minting.
- The pin `a_nan_sample_refuses_at_the_on_locus_limb` reaches the bug through `certify_offset_at` with a NaN control point.
