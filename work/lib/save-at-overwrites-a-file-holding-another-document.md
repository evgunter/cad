---
id: save-at-overwrites-a-file-holding-another-document
kind: issue
title: Workspace::save_at writes over a target file that holds a different document, and compares paths literally
status: open
opened: 2026-10-31
priority: P0
cost: E
refs: [session-save-is-two-acts]
---

Found by the designer pair weighing `work/vseam/session-save-is-two-acts`
(`[ev]` PR 6123). The two defects are filed here because
`crates/pncad/src/workspace.rs` is LIB's ground. (VSEAM orchestrator)

## 1. A target holding a different id is overwritten

`Workspace::save_at` (`crates/pncad/src/workspace.rs`) refuses only
when the document's OWN id is claimed at another path
(`SaveWouldDuplicateId`). When `target` is a file that holds a
DIFFERENT document, `std::fs::write` replaces it. That part disappears
from the store with no refusal, so every assembly pinning it now fails
to resolve. `by_id` then maps two ids to one path, because the
overwritten document's entry is never removed. Both designers
recommend a typed refusal (named `SaveWouldReplaceDocument { id, path }`
in their reports), whose recourse is to pick another name or remove
that document first.

## 2. The resave comparison is literal

`existing != &path` compares the scanned path with `store_path`'s
join, byte for byte. On a case-insensitive filesystem, resaving
`Part.pncad` as `part.pncad` names the same file, yet it refuses
`SaveWouldDuplicateId` against itself. The comparison wants the
filesystem's notion of the same file (a canonicalized path, or file
identity).

## Why P0

(1) silently deletes a part through the library's ordinary save door. It is a live wrong answer, whoever calls the door. As of 2026-10-31 it does not block `work/vseam/session-save-is-two-acts`: the viewer's `docio::save_path` overwrites any target already, so routing through `save_at` loses nothing new. Once this lands, the viewer's Save inherits the refusal.
