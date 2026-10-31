---
id: the-save-format-drops-its-edit-log
kind: issue
title: The save format drops its edits field: a save file is the document's state (Ev, PR 6123)
status: open
opened: 2026-10-31
priority: P1
cost: M
refs: [session-save-is-two-acts]
---

Filed by the VSEAM orchestrator from Ev's ruling on `[ev]` PR 6123
(2026-10-31). GQ3 (`crates/viewer/GUI-DESIGN.md`) now reads: a save file
holds the document's state, which is its id and current snapshot.
Undo is session state and ends at a reopen, and history that outlives a
session belongs to a separate history store (GUI-6). On the residual
question Ev answered "drop it": the `edits` field leaves the format,
rather than staying and going unfilled.

## What this row owes

`crates/editor-core/src/persist`: remove the edit log from the
serialised body (`SerBody`), from `Loaded`, from `save`'s replay check
and from the module doc's "Format" and "What persists" sections.
`persist::save` loses its `edits` argument. Every caller (the store
writers in `crates/pncad/src/workspace.rs`, pncad-py's `save`, the
tour, test fixtures and the save/load/replay-identity CI row) writes
`&[]` today, except the viewer's `docio::save_path`. That one is
deleted by `work/vseam/session-save-is-two-acts`, which writes an empty
log in the meantime, so this row does not block it.

Check what else reads `Loaded.edits` / `Loaded.records` on load before
removing them. Corpus files that carry non-empty logs are regenerable
and need re-blessing. `canon.rs`'s module doc says the pin excludes the
log; that sentence goes along with the log.
