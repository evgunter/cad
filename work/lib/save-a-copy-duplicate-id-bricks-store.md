---
id: save-a-copy-duplicate-id-bricks-store
kind: issue
title: Save-a-copy beside the original bricks the workspace store (DuplicateId for the whole directory)
status: open
opened: 2026-08-28
github: 1117
refs: [1113]
---

## From GitHub issue 1117

opened 2026-08-28, 0 comments.

Found by GUI-4's R2 review (a failing probe), banked from the fix pass of PR #1113.

**The act**: with a document open from `dir/a.pncad`, `SessionOp::Save(dir/b.pncad)` — "save a copy beside the original", an ordinary user act.

**The consequence**: the copy carries the same document id (`id:` header), so the directory now holds two files claiming one identity. The workspace scan refuses `DuplicateId` naming both paths, which means **every subsequent resolution through that directory refuses — for every document in the store, in every session**. Typed, honest, recoverable (delete either file), but the blast radius of the ordinary act is the whole store.

**Why no cheap fix shipped**: identity is the document's, not the file's (A4 — the id answers "which part"; pins answer "which version"), so a save cannot silently mint a fresh id without FORKING the document — every inbound `DocRef` pinning the old id would then miss the copy, which is correct for a fork and wrong for a backup. The right shape is a design question: an explicit "save as new document (fork identity)" act distinct from "save this document at a path", possibly with a save-door warning when the target directory already holds the id under a different filename.

**Where it is documented today** (the fix pass): `SessionOp::Save`'s doc carries the hazard and this issue's number; the store's own `DuplicateId` refusal already names both files.

Refs: PR #1113 (GUI-4), review R2 MINOR-4; `crates/pncad/src/workspace.rs` (`WorkspaceError::DuplicateId`), `crates/viewer/src/session.rs` (`SessionOp::Save`).

## Home

The workspace store is `crates/pncad/src/workspace.rs`, in LIB's `paths:` territory, and document identity is the library contract's ground.
