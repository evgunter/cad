---
id: replay-and-load-keep-the-document-without-its-maintenance
kind: issue
title: Doc::replay and persist load keep the document without the maintenance its edits performed, and Loaded has no column for it
status: open
opened: 2026-09-08
refs: [2165]
---

(EVAL orchestrator) From EVAL-4's sweep (PR 2165), filed onto DOCM's
slate because `edit.rs` and `persist/*` are DOCM's. After EVAL-4
every `editor-core` door in EVAL's fence returns an accepted edit
whole (`Applied`: document, record, maintenance). Two DOCM doors do
not: `edit.rs:~2109` `Doc::replay` keeps `applied.doc` alone across
the replayed edits, and `persist/mod.rs:~456` `load` returns the
document and the record with no maintenance, `Loaded` carrying no
column for it (`persist/mod.rs:~403` discards by design, its comment
says). Whether a replay or a load should surface the maintenance its
edits performed — the documented load boundary says the loaded
document IS the state, so possibly not — is DOCM's call; this row
records that the asymmetry now exists at the layer boundary rather
than inside EVAL's doors. Citations accurate at `eca39b5ce`.

## Re-homed (2026-09-13)

Moved from `work/docm/` to `work/edit/` at DOCM's exit sweep (`docs/DOC-LEDGER.md`,
sweep 14): EDIT is DOCM's successor on the document-model ground (persist, the edit vocabulary, the node and resolver doors). Id, body and header are unchanged; the directory is the
claim (`work/README.md`). Any `## Home` section above is superseded by
this line and is kept as the record of why the file was where it was.
