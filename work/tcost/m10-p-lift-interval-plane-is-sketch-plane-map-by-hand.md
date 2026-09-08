---
id: m10-p-lift-interval-plane-is-sketch-plane-map-by-hand
kind: issue
title: editor-core/tests/m10_p_lift.rs interval_plane rebuilds a SketchPlane from twelve from_f64 calls, which is plane.map(Interval::from_f64)
status: open
opened: 2026-09-08
refs: [2139, D385]
---

(EVAL orchestrator) From EVAL-1's style review (PR 2139, S2), filed
onto S-TCOST's slate because the file is in its fence.
`crates/editor-core/tests/m10_p_lift.rs:207`–`:224` `interval_plane`
is `SketchPlane::new(Affine3::from_parts(Mat3::from_cols(v(c0), v(c1),
v(c2)), v(translation)))` with `Interval::from_f64` per component —
`plane.map(Interval::from_f64)` by another name, the walk EVAL-1
retired from `editor-core/src`. Not on `D385`'s list, which names
`m10_p_fence.rs:397` and `cert3r1_dump.rs:97` in this crate; one line
to fix and a `D385` sibling. Citations accurate at `bd2fe4289`.
