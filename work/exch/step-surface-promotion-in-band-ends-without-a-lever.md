---
id: step-surface-promotion-in-band-ends-without-a-lever
kind: issue
title: STEP surface promotion reads raw f64 against eps_in, and its ill-conditioned arm refuses RecognitionAmbiguous with no lever where the definite arm refuses Topology
status: open
opened: 2026-10-20
priority: P3
cost: M
---

Filed by PCERT unit E1 (`pcert/lane-choices`), from its sweep of lane
choices outside `pcurve_cache`.

Each site below picks between two forms of the same cells, a lane
choice under D3 (`docs/DESIGN.md`, `[verdicts.consumers]`).

## What

`crates/step-import/src/recognize.rs:146` (surface promotion, plane or
cylinder vs NURBS), consumed at `crates/step-import/src/entities.rs:1343`.
- The tests are raw `<= eps_in` on f64. They are not `decide`.
- At the multi-bound curved-face gate, the ill-conditioned arm refuses
  `RecognitionAmbiguous`, which names no lever, while the definite arm
  refuses `Topology`. So the in-band and definite arms of one lane
  choice end differently.
- Curve recognition (`recognize_curve.rs`, `entities.rs:1178`) is
  already a lane choice: StaysNurbs on every arm but Zero.

## Fix

Read the promotion through `decide`, and end in band as definite does:
StaysNurbs, or the same refusal as definite. Add a row and a mutant.
