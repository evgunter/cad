---
id: svd-sigma-folds-re-spell-real-min-max
kind: issue
title: Svd::sigma_min and sigma_max hand-roll the NaN-propagating fold Real::min and Real::max already are
status: open
opened: 2026-10-01
---


Found by the SSI lever-arm lane's shape sweep (a `min`/`max` fold whose
result feeds a guard with a NaN or indeterminate arm), PR "SSI: the
lever-arm fold propagates poison, from one home". Not a defect: a
duplicated spelling.

`crates/geom-core/src/linalg/svd.rs`, `Svd::sigma_min` and
`Svd::sigma_max`, fold the singular values with an explicit
`if s.is_nan() || acc.is_nan() { NaN } else { acc.min(*s) }` (and the
`max` twin). That is exactly `geom_core::Real::min` / `Real::max` at
`f64`, whose contract is "either input NaN ⇒ NaN"; the hand-rolled
guard is needed only because the inherent `f64::min` shadows the trait
method in concrete code. Spelling the fold `Real::min(acc, *s)` (as
`topo::boolean::boxes` and SSI's lever-arm folds now do) keeps one home
for the NaN-propagating fold. `geom-brep/src/ssi.rs`'s
`nan_propagating_max` (the ℝ⁴ seeding guard's chart-speed fold) is a
third copy of the same thing; that PR left it in place because the
chart-speed guards are under a separate design.
