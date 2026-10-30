---
id: the-sketch-tool-authors-a-tangent-continuation
kind: unit
title: INTENT stage 6 PR E: the viewer's sketch tool picks a face, shows its trace and writes the tangent-continuation step
status: review
opened: 2026-10-29
priority: P0
cost: M
blocked_on: []
pr: 6071
---


INTENT stage 6, PR E. Spec: `docs/INTENT-STAGE6-SPEC.md` §6.

The viewer's sketch tool (`crates/viewer/src/sketch.rs`, `pane/profile.rs`) draws a picked face's trace in the sketch plane, display only, and writes C's step with its branch; the document never holds a heading typed off the drawn trace. VIEWER's ground, announced on its log. Parked on C.

Note from C's review (PR #6043): `viewer::sketch::is_committed` (`crates/viewer/src/sketch.rs:464`) compares the base against a program with `on: None`, so a profile drawn on a frame never reads as committed. E's tool, which holds drawn profiles, owns that comparison.

**Dispatched 2026-10-30** to cloud lane `session_01VyvYnLgN4x5RNGfT71WRpC`, branch `intent/s6-e-sketch-tool`. Review tier: single FULL review, because the tool's invariant (the document holds C's step, never a heading typed off the drawn trace) is a claim to falsify, not only read.

**In review 2026-10-30** on #6071: the add-profile path's trace tool (`SessionOp::AddDrawnProfile`), the frame read off the picked face's axis (the door proves the stage's boss `Structural`), and `is_committed` carrying the base's frame.
