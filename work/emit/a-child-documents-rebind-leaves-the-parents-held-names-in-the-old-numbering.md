---
id: a-child-documents-rebind-leaves-the-parents-held-names-in-the-old-numbering
kind: issue
title: A child document's rebind does not reach the parent assembly's names spelled in its numbering; UpdateReference moves the pin and rewrites no name
status: closed
opened: 2026-09-24
priority: P0
cost: H
closed: 2026-09-25
pr: 3223
---


**Executed** (below); first argued from the code as follows.

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

## Measured

`crates/editor-core/tests/asm_parent_held_names.rs`,
`a_parents_held_name_follows_the_childs_rebind_across_a_pin_update`:
- The part is a 2 × 2 square, extruded. It inserts a leg before its wall 1
  (`(2,0)→(2,2)`) by `SetProgram`. The part's own door reports
  `Rebound {wall 1 → wall 2}` for its own paint.
- The parent instantiates version 1 and paints `InPart { part wall 1 }`,
  which denotes `(2,0)→(2,2)`. It then applies `UpdateReference` to
  version 2.
- `maintenance: []`. The held spelling now denotes the leg `(2,0)→(3,1)`.
- This is a silent rebind, so the row stays **P0**. The row pins the
  measured behaviour and is the one that turns when this is fixed.

## Why the fix is blocked: the translation does not survive to `UpdateReference`

1. **The child's report is not stored.**
   - `Applied.maintenance`'s rename rows are not logged. `LoggedEdit`
     keeps only cluster rows, because rename rows are re-derived at
     replay.
   - The store writes every save as a snapshot with an empty log
     (`crates/pncad/src/workspace.rs` module docs: "Every writer here
     writes the CURRENT state as a snapshot with an empty log (history is
     not state)").
   - It holds one file per id: the CURRENT version, not the version the
     parent pins.
   - So at `UpdateReference` time nothing in the store connects the old
     pin to the new one.
2. **The child's report would not be enough even if it were stored.**
   - A child edit reports rows only for names the CHILD's own carriers
     hold.
   - A name a parent holds in the child's numbering is usually held
     nowhere in the child. The measurement only has a child-side row
     because the part painted its own wall 1.
   - The translation needed is the child's segment map, a function over
     every locator of every swept profile. The report is not that.
3. **It cannot be recomputed from the two versions.** Two snapshots do
   not say whether a leg was inserted before wall 1 or wall 1 was
   replaced. Only the edit that made the change knows; that is what
   `SetProgram`'s `provenance` exists to state. Even the old version's
   content is gone from a store that keeps only the current file.
4. `UpdateReference`'s apply is storeless by design
   (`crates/editor-core/src/edit.rs`, `DocEdit::UpdateReference` doc: a
   door reading the store "would make the edit's meaning depend on which
   store was mounted"). So a translation would have to arrive as recorded
   data.

## What would have to persist (the fork, for Ev)

Per document version, a **rename ledger**: for each version saved, the
composite map from the previous saved pin, per swept node, from canonical
locator to canonical locator or retired. It is composed across every
`SetProgram` and every numbering-moving value edit between the two saves.
- The store keeps it beside the current file and chains it across pins.
- A parent's `UpdateReference` then carries the composed old-pin →
  new-pin map as edit data, the way `SetProgram` carries `provenance`,
  computed by the store-aware caller (`pncad::workspace::update_to_store`).
- Apply rewrites every parent-held `InPart` name at the instance through
  it and reports `Rebound` or `Strand` at the parent (DM7).
- This is new persisted state, on the store and on the edit's wire.

It is the same question as
`a-value-edits-last-published-numbering-is-not-recipe-state`: names are
spelled in a numbering whose history the document does not keep. A
ruling for option A there, recording the last published numbering, is
the per-document half of this ledger, so the two should be decided
together.

**Interim that needs no ledger: strand every held name at a pin move.**
- `UpdateReference` reports every parent-held name into the instance as
  stranded, since the old version's numbering cannot be read at the door.
- This is `SetProgram`'s rule for an unreadable old program. It is loud,
  but it breaks every mate on every update, even one that renumbered
  nothing.
- I recommend against landing it without Ev: it trades a silent P0 for a
  loud regression on every assembly update. The ledger is the fix.

## Ruled (2026-09-25)

Ev ruled that profile pieces are named by minted step ids (N1, "the
profile pieces"). Under that rule nothing renumbers, so this row is
fixed by building it: parked on `profile-pieces-are-named-by-minted-step-ids`.
