---
id: the-drafts-keep-held-slot-ids-across-a-document-replacement
kind: issue
title: Drafts::document_replaced drops the held faces but keeps the refused-expression draft and the naming field, both keyed by ids of the outgoing document
status: open
opened: 2026-10-31
priority: P3
cost: E
refs: [viewerapp-document-derived-state-has-no-boundary]
---


Found by the sweep in `viewerapp-document-derived-state-has-no-boundary`
(VSEAM), which put the viewer's document-derived state behind one reset
at `ViewerApp::document_replaced` — the one door every operation
`SessionOp::replaces_the_document` names goes through. That door calls
`Drafts::document_replaced` (`crates/viewer/src/drafts.rs`), which drops
the four held-face fields (`trace_plane`, `trace_face`, `sketch_face`,
`sketch_face_said`) and nothing else.

## What survives the replacement

`Drafts` holds four more values keyed by a `RecipeNodeId` of the
document they were made in:

- **`expr_target` / `expr_text`** — the refused expression text a
  parse refusal sent back (`frame::retype_draft`). Nothing reconciles
  it: it is released only by the next `SetSlotExpression` batch
  (`ViewerApp::perform_batch`). `ViewerBehavior::slot_value_ui`
  (`pane/properties.rs`) shows it whenever `(node, slot)` matches, so
  reopening the same file (same ids) and selecting that node shows text
  refused against the outgoing document.
- **`name_draft`** (`NameDraft { node, slot, var, .. }`) — checked at
  draw time only (`name_field_ui`: the variable is unnamed and the slot
  reads it), so a document with the same ids revives the field.
- **`label_text`** — dropped by `Drafts::rename_shown_for` when the
  shown node differs, but only on a frame `standing_ui` runs; with no
  selection after the replacement, check whether that call is reached.
- **`profile_edit`** — dropped by `Drafts::sync_profile_edit` when the
  selection leaves its node, and reloaded when the program moves; the
  replacement empties the selection, so this one is probably answered
  already. Confirm, then say so in `Drafts::document_replaced`'s doc.

## Fix

Drop the first two (and the other two unless they are confirmed
answered) in `Drafts::document_replaced`, which is already the drafts'
half of the one replacement door; a row in `app.rs` beside
`a_replaced_document_drops_the_held_face` can set each and assert it
gone after an `Open` of the same file. The deeper version is the
`DocumentDerived` move inside `Drafts`: its id-holding fields as one
value rebuilt at the door, so the next one cannot be missed.

## 2026-10-31 — the fix is the move, not more lines in the walk

PR 6125's review (S1): `Drafts::document_replaced` is a hand-written,
field-by-field reset, which is the shape `DocumentDerived` closed one
layer up in `ViewerApp`, and before it `Derived` closed in `DocSession`.
Adding the missing fields to the walk would leave the next id-holding
draft just as able to land outside it. The fix is the same move:
collect `Drafts`' document-derived fields (the held faces, the
expression and naming drafts, the rename text, the profile edit) into
one value that the replacement door rebuilds from nothing, so that a
field is reset by being declared there.
