---
id: the-seated-values-that-already-have-a-home
kind: unit
title: Blend, mirror, clearance and the pattern count: the seated values with an agreed home leave their forms
status: closed
opened: 2026-11-03
closed: 2026-11-03
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

## Closed

2026-11-03, on `forms/seated-homed`.

- **Blend.** The size field is gone. The insert writes
  `viewer::blend::default_size`: a millimetre in the working unit at
  one significant figure (`1 mm`, `0.1 cm`, `0.04 in`). It is not
  measured off the picked edges, because the faces beside an edge
  bound a blend's size and its length does not. The size is the
  fillet's `Radius` or the chamfer's `ChamferDistance` row.
- **Mirror.** The label box is gone (`ToolKind::offers_label`). The
  insert answers the `Mirror` node as what it made, for a placed body
  and an unplaced one alike, so it carries `Mirror N` and is selected
  (`SessionOp::selects_what_it_makes`). The union takes no label.
- **Clearance.** `SlotId::ClearanceBound(BoundEnd::{Low, High})` in
  editor-core; `Node::slots()` lists them for a clearance claim, so
  they are Properties rows and `SetParam` writes them. A clearance's
  bounds are no longer payload (`payload_exprs`, `payload_reads`).
  The evaluator still reads them with the measured values bound, and
  `eval_slots` skips the assertion. The form keeps only the shape
  choice, and the bounds are inserted at `0`. Python's slot words are
  `clearance_low` and `clearance_high`.
- **Pattern count.** One widget (`index_count_ui`) is drawn on the
  index variable's row and on the rows of every node that reads the
  index (`props::indices_read`). It writes through
  `props::index_count_edit`, and so does a drag. The pattern form's
  count field stays.
