---
id: focus-per-segment-is-no-longer-blocked-on-a-guess
kind: issue
title: marks::focus says per-segment marking waits on a step-to-segment correspondence that ProfileEdgeRef's StepId keying now supplies
status: open
opened: 2026-09-30
priority: P3
cost: E
---


Found by a designer on AUTHOR's face-naming fork (#3571).

`crates/viewer/src/marks.rs`, `focus`'s doc, "**What it does NOT do yet (issue 1182)**". It says per-segment marking wants "the profile-step ↔ `RoleSeg::Lateral(ProfileEdgeRef)` correspondence established rather than guessed", because "a slot's `step` is an index in the AUTHORING chain and the name's `segment` an index in the LOWERED one".

That premise is stale. `ProfileEdgeRef` is now keyed by `StepId`, and the program's `step_ids` supply the step ↔ wall correspondence the paragraph says is missing. So selecting a profile step could light the walls of that step alone without guessing.

This row holds two things: the doc is false today, and the feature it defers may now be cheap. Check the `StepId` keying before relying on it; the designer read it and did not run it.
