---
id: step-surface-promotion-in-band-ends-without-a-lever
kind: issue
title: STEP surface promotion reads raw f64 against eps_in, and its ill-conditioned arm refuses RecognitionAmbiguous with no lever where the definite arm refuses Topology
status: open
opened: 2026-10-20
---

Filed by PCERT unit E1 (`pcert/lane-choices`), from its sweep of lane
choices outside `pcurve_cache`.

D3 (`[verdicts.consumers]`) treats a decision that picks between two forms
of the SAME cells (a closed form at Zero, a general one otherwise) as a
lane choice. In band it takes the general lane, as definite does, or it
refuses lever-only "not yet" where no general lane is written. It never
escalates with a tolerance offer and never ends as a defect.

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
