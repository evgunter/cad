---
id: curve-field-walks-have-no-one-home
kind: issue
title: the curve side has the same hand-written field walks the surface side lost in PR 3429 (poisoned_curve_datums, the mesh memo's curve3)
status: open
opened: 2026-09-29
priority: P3
cost: M
refs: [surface-field-walks-and-source-theorem-checks-have-no-one-home]
---


From PR 3429's sweep (the surface-side walk unit). The surface side
now has one walk (`Surface::analytic_data` / `paired_with` in
`crates/geom/src/surfaces.rs`) that every reader folds; the curve side
still walks `Curve3` / `CurveDatum` by hand at
`validate::poisoned_curve_datums` and `crates/mesh/src/memo.rs`'s
`curve3`. Same fix: one walk beside `CurveDatum`, each site a fold, a
row that goes red when a reader skips a scalar.
