---
id: a-vector-unit-pick-is-one-undo-per-component
kind: issue
title: a vector unit pick commits one history action per component, so undoing one pick takes several Undos
status: open
opened: 2026-09-28
priority: P3
cost: E
---

Found by review of `vnews/the-unit-picker-reads-its-refusal`. The
problem predates that branch.

## What happens

`ViewerBehavior::slot_unit_ui` (crates/viewer/src/pane/properties.rs)
answers one pick on a vector's picker by pushing one
`SessionOp::SetSlotUnit` per writable component. `ViewerApp::perform_batch`
(app.rs ~:1129) performs each one separately, and
`DocSession::set_slot_unit` (session.rs) commits each through
`Self::commit`, so there is one history action per component. One pick
of `cm` over a point's x, y and z takes three Undos to take back.

## The shape

`DocSession::commit_action` (session.rs ~:2634) already takes a
`Vec<DocEdit>` and is all-or-nothing, one action. A vector pick wants
one op that carries the components (or a batch-level action) and
commits their `props::slot_unit_edit`s through it. Mind the "some
components are computed" case: the writable components are the ones
in the action, and a refusal among them refuses the pick whole.
