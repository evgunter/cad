---
id: face-run-is-a-state-machine-kept-by-convention
kind: issue
title: topo::props' FaceRun holds its state in four fields (open_at, converged_at, taken_at, refusal) kept consistent by five spellings of 'next state'
status: closed
opened: 2026-10-21
priority: P3
cost: M
parent: the-measurement-door-refines-until-a-predicate-resolves
closed: 2026-10-23
pr: 5628
---


From PR 5568's review (S1) and its fix-pass re-review (Style Q7), 2026-10-21.
`crates/topo/src/props.rs`' `FaceRun` now carries `open_at`, `converged_at`,
`taken_at` and `refusal`, which together encode one state: open, converged,
exhausted, or refused-and-open. Five sites move it to its next state:
`advance`, `stopped()`, `continue_to_target`'s `*run = resumed`,
`PastTarget::refine`'s plain replacement, and the window constructors. They
agree only because the plain replacements never touch refused faces (filtered
by `resumes_after` and `converged_at`), an invariant held by filtering, not by
the type. `target_walk` is also a second reporting-level walk beside
`mass_properties_runs`; it exists so the backstop can hold a `PastTarget`.
Make the state one enum with one transition, and say whether the reporting
walk can hand back a `PastTarget` itself. Sequence it with the stop-rule unit
that `[ev]` PR 5574 would open: that unit rewrites the same walk.

## Closed

PR 5628 (measure door unit B, R11). `FaceRun` is now `{ face, contribution, state: FaceState }`, with `FaceState { Closed, Quad { taken, ran, stop: LaneStop } }`. `face_flux_about` mints it and `FaceRun::advance` alone moves it, keeping the narrower enclosure. Whether a face meets a resolution is read off its kept enclosure (`meets`), and the interim number refusal off its state (`interim_budget`). `open_at`, `converged_at`, `refinable`, `taken_at` and `refusal` are gone, and so are `stopped`, `resumes_after`, `Resume` and `target_walk`. There is no reporting-level walk to hand back a `PastTarget`: the backstop walks the sign-level certificates themselves (`walk_set`).
