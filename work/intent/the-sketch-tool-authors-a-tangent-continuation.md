---
id: the-sketch-tool-authors-a-tangent-continuation
kind: unit
title: INTENT stage 6 PR E: the viewer's sketch tool picks a face, shows its trace and writes the tangent-continuation step
status: parked
opened: 2026-10-29
priority: P0
cost: M
blocked_on: [a-sketch-continues-tangent-to-a-surfaces-trace]
---


INTENT stage 6, PR E. Spec: `docs/INTENT-STAGE6-SPEC.md` §6.

The viewer's sketch tool (`crates/viewer/src/sketch.rs`, `pane/profile.rs`) draws a picked face's trace in the sketch plane, display only, and writes C's step with its branch; the document never holds a heading typed off the drawn trace. VIEWER's ground, announced on its log. Parked on C.
