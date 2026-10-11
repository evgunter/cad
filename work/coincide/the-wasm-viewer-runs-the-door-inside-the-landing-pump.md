---
id: the-wasm-viewer-runs-the-door-inside-the-landing-pump
kind: issue
title: On wasm the viewer's inline door seam runs the coincidence door inside the same pump that lands the evaluation
status: open
opened: 2026-10-11
priority: P2
cost: M
---

Found in review of PR 4670 (`the-symbolic-door-proves-at-one-strength-at-rest`).

The viewer runs the coincidence door at rest through `viewer::evalseam::DoorService`. The native build uses `ThreadProver`, on its own worker. The wasm build (`crates/viewer/src/app.rs` `prover`) and every session built by `DocSession::new` / `DocSession::inline` use `InlineProver`, whose `poll` runs the door. `DocSession::pump` polls it right after landing the evaluation, so on wasm the frame that lands a filleted document waits for the door's replay and discharge: seconds in release on the composed dies (die_composed_tour, 27.7 s).

That contradicts `DoorService`'s promise that a landing draws at once, and Ev's "off the draw path" (PR 4644) for that build. The evaluation seam has the same posture on wasm today (`InlineEvaluator` runs inside `poll`), so the fix is the same: a Worker-backed seam, or an inline slice that spreads the door across frames.

Done when the wasm build's landing draws before its door runs.
