---
id: the-seated-values-that-already-have-a-home
kind: unit
title: Blend, mirror, clearance and the pattern count: the seated values with an agreed home leave their forms
status: dispatched
opened: 2026-11-03
priority: P1
cost: M
parent: creating-a-thing-and-editing-it-are-two-implementations
branch: forms/seated-homed
refs: [a-seated-tools-values-have-no-home-after-insert, a-mirror-of-an-unplaced-body-labels-its-union-mirror]
---


## The unit

These are the parts of fork row 247 that all three designers agreed on
and that do not wait on Ev's answers on `[ev]` PR 6406 or on PLACING's
build of 6369:

- **Blend.** The size field goes. The insert writes a default `Length`
  slot (`SlotId::Radius` / `ChamferDistance`), which is already a
  Properties row.
- **Mirror.** The label box goes. The insert stores "Kind N" on the
  node the gesture's noun names, which closes
  `a-mirror-of-an-unplaced-body-labels-its-union-mirror`.
- **Clearance.** The bounds become slots (`SlotId::ClearanceBound`, in
  Low and High arms, on the precedent of VR4's "an assertion's value
  and its bound"). `Node::slots()` lists them, so they are Properties
  rows. The form's bound fields go and the bounds default to 0
  (contact). The bound's shape stays a create button until Ev answers
  6406's question 2.
- **Pattern count.** The index's count is reachable as a row: on the
  index variable's own row (`Selection::Variable` draws an Index) and
  on the pattern's rows, through the existing `props::index_count_edit`.

Not in this unit: the split plane, the pattern's direction, axis,
pitch and step fields, the pose-scalar slot address and the
fused-union discovery. Those wait on 6406 and on PLACING.
