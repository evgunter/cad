---
id: save-at-overwrites-a-file-holding-another-document
kind: issue
title: Workspace::save_at compares the resave path literally, so a case-insensitive filesystem refuses a resave against itself
status: open
opened: 2026-10-31
priority: P3
cost: E
refs: [session-save-is-two-acts, 6135]
---

Found by the designer pair weighing `work/vseam/session-save-is-two-acts`
(`[ev]` PR 6123). Two defects were filed here, and one remains, because
`crates/pncad/src/workspace.rs` is LIB's ground. (VSEAM orchestrator)

## 2. The resave comparison is literal

`existing != &path` compares the scanned path with `store_path`'s
join, byte for byte. On a case-insensitive filesystem, resaving
`Part.pncad` as `part.pncad` names the same file, yet it refuses
`SaveWouldDuplicateId` against itself. The comparison wants the
filesystem's notion of the same file (a canonicalized path, or file
identity).

## 2026-10-31 — part (1) landed in PR 6135; re-banded P0 → P3

The overwrite this row first recorded is fixed at the source in PR 6135
(VSEAM's save lane, authorised by the VSEAM orchestrator onto LIB's
ground, since LIB has no orchestrator). `Workspace::save_at` now refuses
`WorkspaceError::SaveWouldReplaceDocument { id, path }` before any
write when `target` is the file of a different document, so `by_id`
never maps two ids to one path. What is left is (2): the literal path
comparison. It refuses rather than corrupting anything, and only on a
case-insensitive filesystem, so it is P3.
