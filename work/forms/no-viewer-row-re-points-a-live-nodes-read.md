---
id: no-viewer-row-re-points-a-live-nodes-read
kind: issue
title: No viewer row writes a live node's operand slot, so a split's or mirror's plane cannot be re-picked after insert
status: open
opened: 2026-11-03
priority: P2
cost: M
---



## Finding

`crates/viewer/README.md`, "A create gesture commits the node": a live
node's reads change only by the slot door or `SetMembers` (DM6,
`crates/editor-core/REFERENCES.md`). The kernel's door is there:
`DocEdit::SetParam` with `SlotValue::Read` on `SlotId::Operand(..)`,
which `combine_ops.rs`'s
`the_split_tool_cuts_along_a_picked_face_and_its_plane_re_picks`
drives to move a split's cut onto another face. No row or gesture of
the viewer reaches it for a person:

- `props::slot_rows` lists `Node::slots()`, the expression slots; no
  operand slot has a row (`crates/viewer/src/props.rs`, `slot_rows`).
- `SessionOp::SetSlot` takes `props::SlotValue`, a number, not a read.
- `DocSession::writes_nothing` (`crates/viewer/src/session.rs`) says it
  in so many words: "A read is lowered by the door ...; no panel
  offers one". The only `SlotValue::Read` writes in the viewer are the
  takeover's re-points (`Takeover::repoint`) and the re-target into a
  space (`session::into_space`).

So a split's plane (`OperandSlot::Tool`) and a mirror's
(`OperandSlot::Plane`) are fixed at insert in the GUI: to cut along
another face, the person deletes the node and inserts it again. The
same holds for every operand slot (a boolean's members aside, which
`SetMembers` writes).

The build is a re-pick row on a node's operand slots, held as the
seated tools hold picks, committing one `SetParam` with the read.
(FORMS, the split and pattern forms unit.)
