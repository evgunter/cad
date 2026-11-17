---
id: the-save-format-drops-its-edit-log
kind: issue
title: The save format drops its edits field: a save file is the document's state (Ev, PR 6123)
status: closed
opened: 2026-10-31
priority: P1
cost: M
refs: [session-save-is-two-acts]
closed: 2026-11-17
branch: doctail/the-save-format-drops-its-edit-log
pr: 7094
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

## 2026-10-31 — one more caller with a non-empty log (VSEAM sweep)

Found by the sweep of `work/vseam/session-save-is-two-acts`. The viewer's
`docio::save_path` is gone and the viewer now writes through
`Workspace::save_at`, which passes `&[]`. One caller still writes a
non-empty log, and this row's caller list above misses it:
`demos/tour/src/diefillet.rs`, `corpus_text` (the `die-corpus` mode in
`demos/tour/src/lib.rs`), which saves the EMPTY document plus the whole
model as an edit log, so the editor-core Band 4 corpus file can load at
every ε row. Dropping `edits` takes that design with it. The corpus
needs another way to be ε-portable (regenerate per ε row, or a
snapshot that re-stamps ε), decided in this row.

## Closed

A save file holds the document's state and nothing else. The body is
`{ "snapshot": <Doc> }`; `persist::save(doc, tol)` takes no edits and
`persist::load` answers the `ProfileDoc` itself. `Loaded` (and with it
`Loaded.edits`, `Loaded.records`), `PersistError::EditReplay`,
`NonFiniteSite::Edit`, the save door's replay check and the validator's
edit-log walk are gone, and the module doc's "Format" and "What
persists" sections and `canon.rs`'s exclusion sentence say so. Every
caller moved (pncad, pncad-py's `save` and `load`, which now answers a
`Doc`, the viewer, the tour, test fixtures, the save/load/replay-identity
row). Committed files carrying `"edits"` were re-blessed: `golden.cad`
(its trailing edit folded into the document), `die_tool.pncad`,
`plate_param.pncad`, `gallery_ring.pncad`, and the tour die corpus.

The tour's die corpus is now a saved state that the corpus reads at
each ε row by re-stamping its one ε line, as `die_tool`'s committed
document already was. The old log replayed at 1e-6, 1e-9 and 1e-12
gives a document that is bit-identical to the new file loaded at that row, and
saves to the same bytes.

`persist::nesting::ENVELOPE` now names the deepest place a save writes
an expression, an index variable's count (6). The parked pose-chain row
stays open in a new form: the file half is gone, but the edit door's
stack is not bounded (see that row).
