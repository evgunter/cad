---
id: adopt-describes-a-line-by-its-end-points-difference
kind: issue
title: adopt's mapped_self_description stores a line edge's ExtrudedPoint vector as its end points' difference where the carrier held its direction
status: open
opened: 2026-11-14
priority: P3
cost: E
---


Found by BOXFINISH's sweep for `boxfinish/sweep-frames` (the shape "a stored direction re-derived from points the construction placed"), not measured.

`crates/step-import/src/adopt.rs` `mapped_self_description`, the `Curve3::Line` arm: the description it stores is `MappedSource::ExtrudedPoint { vec: p_end − p_start, .. }`, the edge's two end points' difference, where the carrier already holds its `Line { dir }` (the `nurbs_rim` branch beside it reads its carrier's frame through `line_frame`). At `f64` this is the same line to rounding; it is a datum re-derived rather than read, and the description and the carrier can then part by that rounding.

**Repair shape.** Build the vector from the carrier's direction and the edge's span.
