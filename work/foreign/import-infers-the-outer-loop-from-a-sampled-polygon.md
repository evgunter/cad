---
id: import-infers-the-outer-loop-from-a-sampled-polygon
kind: issue
title: STEP import picks a face's outer loop from a 16-per-edge sampled chart polygon and a sampled metric floor, so a wrong pick is silent at the selection
status: open
opened: 2026-10-30
priority: P3
cost: M
refs: [import-splits-a-foreign-face-or-edge-at-each-c0-knot]
---


(QUAD 6013 fix lane, the class sweep on PR 6013: a sampled selection that decides topology fails silently in the direction the rule forbids.)

`crates/step-import/src/entities.rs` `outer_bound_index` reads `ring_samples` (16 points per edge) into `chart::infer_outer` (`crates/step-import/src/chart.rs`), which unwraps the samples into a chart polygon, takes `metric_floor` over the polygon's own vertices as the factor that carries `ε_in` into chart units, and picks the ring whose polygon contains the others. All three are sample readings, not bounds:

- a trim that bulges between two samples changes the polygon's containment without the inference seeing it;
- `metric_floor` is the least metric at the samples, not a floor over the face, so `tol = ε_in / floor` can be too wide where the metric dips between samples.

What it decides is topology (which loop is the face's outer loop) where the file states none or states several. A wrong pick is not refused at the selection; whether the at-rest gate catches every wrong pick has not been checked. Owed: either certify the containment (an enclosure of each ring's chart image, as `knot_split::clear_of` does for the knot line) or show that the gate refuses each wrong pick, and pin it.
