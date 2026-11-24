---
id: main-red-geom-breps-crossing-parameter-rows-do-not-compile
kind: issue
title: main red: geom-brep's lib tests do not compile (crossing_parameter_rows hands settle a residual of the old shape)
status: open
opened: 2026-11-24
priority: P0
---


Found by `from-f64-is-split-by-where-the-number-comes-from` on its first local `cargo check --workspace --all-targets`, on files it did not touch.

`crates/geom-brep/src/curve_face/composite/crossing_parameter_rows.rs`, `settle_and_the_splits_interiority_answer_alike_on_admitted_parameters`, calls `settle(.., &residual)` with the file's `fn residual(t: f64) -> Result<Sign, CompositeRefusal>`. `settle` (`curve_face/composite.rs`) takes `off: &impl Fn(Point3<T>) -> Margin<T>` since the solved-value door landed, so `geom-brep`'s lib test target does not compile:

```
error[E0631]: type mismatch in function arguments
   --> crates/geom-brep/src/curve_face/composite/crossing_parameter_rows.rs:130:13
    = note: expected function signature `fn(geom_core::Point3<f64>) -> _`
               found function signature `fn(f64) -> _`
```

Both files are byte-identical at `origin/main` 679f90e (PR 7651's merge) and at that PR's head c185be6, whose CI run was cancelled before the merge (runs 45425409446 cancelled, 45425577813 skipped); a push to main runs no gate. Every `geom-brep` unit test is unrunnable until the row is given the residual `settle` now takes.
