---
id: edit-error-respeaks-from-a-later-version
kind: unit
title: EditError can be spoken again from a later version of its document, so a rename later in its own batch reaches the viewer's line
status: closed
pr: 3832
opened: 2026-10-02
closed: 2026-10-02
priority: P3
cost: M
parent: node-labels-are-document-data
refs: [viewer-refusals-speak-the-node]
---


Filed by `viewer-refusals-speak-the-node`. The viewer speaks a batch's refusal again at the end of the batch, from the committed document the batch leaves (`frame::batch_refusal`, `Refusal::respoken`), so a rename later in the same batch is the label the status line says. `Refusal::Edit` is the one arm that cannot: `EditError` holds its nodes as `SpokenNode`, built by the edit door at the refusal (about 87 mentions in `edit.rs`), and has no way to be spoken again. So `[an edit the door refuses, SetLabel]` in one frame shows the label from before the rename until the next acting batch clears the line.

The fix is editor-core's: an `EditError::respoken(&self, doc)` (or the same arm-by-arm walk the viewer's `Refusal::respoken` does) that rebuilds each `SpokenNode` with `doc.spoken(node.id())` and each `HeldNodes` with `held_by`, sound for a later version of the same document because an id names one node within one document's history (`Refusal::respoken`'s doc comment, pinned by `node_labels::an_undo_then_a_different_insert_mints_a_different_id`). Then `Refusal::respoken`'s `Edit` arm calls it.
