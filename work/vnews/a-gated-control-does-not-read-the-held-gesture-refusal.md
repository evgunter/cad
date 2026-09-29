---
id: a-gated-control-does-not-read-the-held-gesture-refusal
kind: issue
title: gated controls read their op's own refusal but not the GestureInFlight gate perform applies first
status: open
opened: 2026-09-28
priority: P3
cost: M
design: true
---

Found by review of `vnews/the-unit-picker-reads-its-refusal`, which
gated the slot unit picker on `SetSlotUnit`'s own slot admission.

## What happens

`DocSession::perform` (crates/viewer/src/session.rs, the first lines
of `perform`, ~:1173) refuses every operation that
`SessionOp::permitted_during_value_gesture` (session/op.rs, ~:1207)
answers `false` for with `Refusal::GestureInFlight` while a value
gesture is held. It does this before the operation's own door runs.

The controls that gate on "the refusal the op would give" read only
that op's own door, so while a gesture is held each one is drawn live
and a click is refused one layer up:

- `ViewerBehavior::slot_unit_ui` (pane/properties.rs), gated on
  `DocSession::slot_unit_refusal`. `SetSlotUnit` is `false` in the
  table.
- `ViewerBehavior::range_button` / `probe_refusal` (pane/properties.rs,
  ~:941/:973), gated on the driven refusal. `ProbeBounds` is `false`.
- The toolbar's Undo and Redo (app.rs, `toolbar_ui`, the
  `Refusal::nothing_to_step` loop, ~:1551), gated on the history.
  `Undo` and `Redo` are `false`.

The free-move table (`permitted_during_free_move`, op.rs ~:1324) is
the same layer for the probe, and each control above has to be checked
against it too.

## Stated so it is not overstated

A value gesture is a drag on a field, so the pointer is usually busy
while one is held, and reaching another control mid-drag may be rare
or impossible by mouse. Whether it is reachable (keyboard, a second
pointer, a gesture left held across frames) is part of the work. If it
is not reachable, the answer is to say so where these gates are
documented, not to gate.

## The shape

Fold the session-wide gate into ONE question each control asks (a
session method answering "what would `perform` say to this op before
its door", composed with the door's own refusal), rather than having
each control spell the gesture check. The slot unit picker's doc names
this gap.
