---
id: sweep-revolve-decision-endings-re-spell-ending-kind-end
kind: issue
priority: P3
title: sweep's ClearanceDecision::ending and TubeDecision::ending re-spell geom-core EndingKind::end
status: open
opened: 2026-10-13
cost: C
---


(Filed by ENCL, from PR 5003's delta review.)

## What

PR 5003 added `geom_core::EndingKind { Sized, Lever(&'static str) }`. `EndingKind::end` and `Indeterminate::ending_by` are now the one by-kind dispatch for an escalation's ending; profile and editor-core both call them. Two copies remain, on carve's ground:
- `crates/sweep/src/revolve/mod.rs` `ClearanceDecision::ending`;
- `crates/sweep/src/revolve/tube.rs` `TubeDecision::ending`.

Both are byte-for-byte `EndingKind::end` with lever `COINCIDENCE_RECOURSE`.

## Repair shape

Both call `EndingKind::end`. No rendered text should move.

Related, not in this row: topo `chord_join::order_ending` (chord_join.rs, about :452) puts both arms under `JOIN_RECOURSE`. `EndingKind::Sized` hard-codes `COINCIDENCE_RECOURSE`, so that site cannot use the shared dispatch yet. That asymmetry is ENCL's to weigh.
