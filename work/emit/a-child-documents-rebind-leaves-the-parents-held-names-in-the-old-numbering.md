---
id: a-child-documents-rebind-leaves-the-parents-held-names-in-the-old-numbering
kind: issue
title: A child document's rebind does not reach the parent assembly's names spelled in its numbering; UpdateReference moves the pin and rewrites no name
status: open
opened: 2026-09-24
priority: P0
cost: H
needs_ev: true
---


**Argued from the code, not executed.** Measuring it needs a two-version
part store and a mate on a child wall, which is not cheap here. The row
states what the code does.

- An assembly holds names spelled in a child document's numbering, for
  example a mate on a child wall.
- When the child document is reshaped, the child's own names are rewritten:
  - `SetProgram` does this;
  - since PR #3180, so does a value edit that moves its canonical
    numbering.
- `reshape_report` walks only the child document's carriers
  (`Doc::rewrite_names` over the one `Doc`).
- `DocEdit::UpdateReference` (`crates/editor-core/src/edit.rs`, the
  `UpdateReference` arm) moves `doc_ref.pin` and rewrites no name.
- So after the parent updates its pin, a parent-held name keeps the old
  spelling. That spelling denotes whatever the new version draws at the
  old coordinate. When the child rebound the name, that is a different
  wall.
- That is a **silent rebind**, which is the P0 banding. Where the child
  retired the coordinate, the parent's name dangles instead (`Vanished`).

A fix has to carry the child's maintenance (its `Rebound` and `Strand`
rows) across the pin move. Either:
- `UpdateReference` reads the version diff's maintenance and applies it to
  the parent's held names, reporting as DM7 does; or
- the child's rows are recorded with the version so the parent can
  replay them.

## Folded into the stable-name question (2026-09-24)

This row, the other `needs_ev` EMIT row on profile numbering, and EDIT's
`a-slot-edit-through-a-zero-fit-renumbers-a-loops-live-names` share
one cause. A profile name spells a position that is recomputed from
current state, so any edit that moves the positions re-denotes it. All
three are put to Ev as one question: name a profile piece
by an id minted when its step is authored (`names/README.md`, "N1, the
profile pieces"), or keep positions and persist a rename ledger. Under
the id rule this row's fork does not arise, because no numbering is
left to carry.
