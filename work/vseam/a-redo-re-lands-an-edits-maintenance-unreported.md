---
id: a-redo-re-lands-an-edits-maintenance-unreported
kind: issue
title: A redo re-lands an edit's DM7 maintenance without reporting it, where the commit reported it
status: open
opened: 2026-09-24
priority: P3
cost: D
---


`crates/viewer/src/session.rs` `DocSession::step` (the undo/redo door)
moves the history pointer and reports only the display prune
(`OpOutcome::from_prune`), so `OpOutcome::maintenance` is empty on every
redo. The commit that first landed the action carried its net DM7 rows
(`record_action`, through `editor_core::MaintenanceNet`) and the chrome worded them
(`frame::outcome_notices`); redoing the same action after an undo
re-lands the same stranded or rewritten names and says nothing.

The withdrawals are not asymmetric this way: a redo's prune reports
them again, because the prune is recomputed against the redone
document. Maintenance is not recomputable from the pointer move — it is
a function of each edit applied to its predecessor, which the history
holds as logged entries (`History::entry(..).edits()`) — so reporting
it on redo means either keeping each action's net rows beside its
history entry, or re-applying the entry's edits to the state before it.

Undo is the other half and is a question rather than a defect: undoing
a strand restores the name's referent, which is arguably news of its
own ("node 7's name denotes again"), and nothing reports it.

Found while building the maintenance report
(`the-viewer-drops-every-dm7-rename-report`).
