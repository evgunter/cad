---
id: offset-fit-on-locus-fold-drops-a-nan-sample
kind: issue
title: offset fit folds its on-locus samples with f64::max, so a NaN sample never reaches the limb-1 guard
status: open
opened: 2026-10-01
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

## Why it is latent today

A fitted NURBS with finite control points evaluates finite, and an
`offset_point` that cannot answer returns `INFINITY` on its own path. No
fixture is known to reach a NaN sample.

## Repair

`Real::max(m, ..)` and `Real::max(on_locus_max, ..)`: one line per
site, and the limb-1 guard already refuses the NaN that would then
arrive. `hull_sup`'s fold beside it reads `cell_bound`, which maps every
non-finite to `INFINITY`, so it cannot see a NaN and is not this shape.
