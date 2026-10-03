---
id: three-loops-apply-several-edits-and-net-their-maintenance
kind: issue
title: Applying several edits as one action and netting their maintenance is written three times: the viewer's stage_run, refactor's Recording, and regauge_then_mate
status: review
opened: 2026-10-03
priority: P1
cost: M
branch: recipe/one-action-recorder
---


## The finding

PR 3925's second review (Q1, likely): the loop that applies edits in order, threads the document, pushes each `Applied` into `MaintenanceNet`, collects minted ids and finishes exists three times —
- `crates/viewer/src/session.rs` `stage_run`;
- `crates/editor-core/src/refactor.rs` `Recording::apply`/`finish`;
- `crates/editor-core/src/edit.rs` `regauge_then_mate` (added by PR 3925, with "`Recording` is concrete over `ProfileDoc`" as the reason for the copy, while the door is generic over `P`).

PR 3925's first review had already counted four composers of several edits' `Applied`; the fix pass replaced its own with a third copy of this loop. The fix-mints-a-copy shape.

## What would close it

One generic recorder in editor-core (`Recording` made generic over the payload, or a new home both use) that the viewer's `stage_run`, refactor and `regauge_then_mate` all call; the door's `unreachable!` on the minted id goes with it (the reviewer: apply the re-gauges in the loop and the insert after it).

Filed by the RECIPE orchestrator from PR 3925's second review.
