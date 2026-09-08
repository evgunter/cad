---
id: placement-lifts-its-affine-by-hand-beside-affine3-map
kind: issue
title: Placement::linear and Placement::affine lift twelve components by hand from [[f64;3];3] + [f64;3] into Mat3<T>/Affine3<T>, one construction away from self.affine::<f64>().map(T::from_f64)
status: open
opened: 2026-09-08
refs: [2139, D368]
---

(EVAL orchestrator) Filed from EVAL-1's sweep (PR 2139), which retired
`editor-core`'s second home of the per-coordinate affine walk into
`Affine3::map` / `SketchPlane::map`. The same shape survives in
`crates/editor-core/src/placement.rs:202` (`Placement::linear`) and
`:215`–`:218` (`Placement::affine`): a hand lift of the nine matrix
components and the three translation components from the stored `f64`
arrays into `Mat3<T>` / `Affine3<T>`, where `self.affine::<f64>().map(T::from_f64)`
is one construction. `placement.rs` is in no program's `paths`
(`work/eval/program.md` keep_out lists it among the unowned,
unfinished files), so the finding waits here; the program that draws
that fence takes it. Citations accurate at `bd2fe4289`.
