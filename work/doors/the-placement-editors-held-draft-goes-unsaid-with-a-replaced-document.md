---
id: the-placement-editors-held-draft-goes-unsaid-with-a-replaced-document
kind: issue
title: The placement editor's held draft is let go of unsaid when a document replaces the one it edits
status: open
opened: 2026-11-22
priority: P3
cost: E
---


Found by FORMWORDS' sweep for state held about a replaced document
(`a-creation-forms-held-pick-survives-a-document-swap`), read rather
than driven.

The two live editors let a pending draft go alike and say so
(`drafts::Dropped`, `drafts::Editor`), except at a document
replacement. `Drafts::document_replaced` lets the profile editor's
draft go with `DropWhy::DocumentMoved`, which the status line says
("discarded — the document changed under them"). `Tools::document_replaced`
(`crates/viewer/src/tools.rs`) rebuilds the open tool from its kind, so
a placement editor holding changes (`MateTool::holds_changes`) becomes
`MateTool::new()` and the changes go with nothing said — where an undo
over the same draft says it (`Tools::undo_pending`,
`ViewerApp::undo_placement_draft`, `Drafts::placement_undone`).

## Fix

Have `Tools::document_replaced` answer the placement a held draft
edited, as `Tools::undo_pending` does, and say it on the same channel
with `DropWhy::DocumentMoved` and `Editor::Placement`. The placement
must be spoken from the OUTGOING document, which is gone by the time
`ViewerApp::document_replaced` runs (it is called after the session
performed the replacement), so the said text has to be taken before
the op is performed or carried by the editor.

`Drafts::mate_value_edits` (the placement editor's typed text) goes silently too: `Drafts::document_replaced` drops it with nothing said.
