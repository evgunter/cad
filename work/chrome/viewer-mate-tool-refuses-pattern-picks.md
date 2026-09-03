---
id: viewer-mate-tool-refuses-pattern-picks
kind: issue
title: Viewer mate tool refuses pattern-placed picks — cannot author the mates the A11 member vocabulary admits
status: open
opened: 2026-08-31
github: 1412
refs: [1400]
---

## From GitHub issue 1412

Opened 2026-08-31; 0 comments.

Filed from the MATE-1 dual review (PR #1400, R2 MINOR-2, verified at the site). `crates/viewer/src/matetool.rs:417` gates mate-member picks through `is_instance` (`display.rs:193`), which matches `InstantiatePart` heads only — so the GUI refuses `NotAnInstancePick` for exactly the pattern-placed heads the member-vocabulary rider now admits and the solve now places. A user can author these mates through the recipe/Python doors but not by picking in the viewer.

GAUTH ground (viewer chrome), flagged for the GAUTH orchestrator's queue: the fix is presumably widening the pick gate to the member vocabulary (`Pattern` + `Instance(i)` over a live instance) and emitting the `Instance(i)`-headed reference, with `NotAnInstancePick` retained for everything else.

Signed: (S-MATE orchestrator)

## Home

`work/issues/` — the issue routes itself to GAUTH (viewer chrome), and both GAUTH and GUI are closed programs, so no open program owns `crates/viewer`.
