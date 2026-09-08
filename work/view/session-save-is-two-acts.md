---
id: session-save-is-two-acts
kind: issue
title: 'SessionOp::Save routes through the workspace save door; a save-as-new-document op spells the fork'
status: open
opened: 2026-09-08
refs: [LIB-SAVEFORK, save-a-copy-duplicate-id-bricks-store, 1117, 2016]
---

## The hand-off

Ev ruled (PR 2016, recorded at `crates/editor-core/ASSEMBLY.md`'s A4
clause — "A save is two acts …") that saving a document at a path
keeps its identity and refuses typed when the store already holds that
id under another filename, and that saving it AS A NEW DOCUMENT mints
a fresh id, an explicit fork that leaves every inbound `DocRef`
pointing at the original.

The library half landed as LIB-SAVEFORK; the viewer half is this item.
LIB's `keep_out` fences `crates/viewer`, so `SessionOp::Save` was left
exactly as it is and this is filed rather than built.

## The two store doors, by their final spellings

`crates/pncad/src/workspace.rs`, both on `Workspace`:

- `save_at(&mut self, doc: &ProfileDoc, target: impl AsRef<Path>, tol: Tol) -> Result<PathBuf, WorkspaceError>`
  — the ordinary save at a path. Identity is kept. The scan decides
  the act: the id claimed at a DIFFERENT path refuses; claimed at
  `target` it is a resave; unclaimed it is a create at that name.
  `target` is a `*.pncad` file directly in the store's root (bare
  name, or the root written out); anything else refuses, because a
  different root is a different store.
- `save_as_new_document(&mut self, doc: &ProfileDoc, tol: Tol) -> Result<(DocumentId, PathBuf), WorkspaceError>`
  — the fork. Fresh random id, `{newid}.pncad`, written through
  `create`'s validator; the original untouched. The fork's content pin
  EQUALS the original's (the pin's preimage excludes the `id` key), so
  a fork is detectably the same version of different part.

## The refusal the session must surface

`WorkspaceError::SaveWouldDuplicateId { id, existing, requested }`
(`crates/pncad/src/workspace.rs`) — raised BEFORE any write, so the
copy never exists and the store stays scannable. Its recourse is a
CHOICE, which is what a session has to put in front of a person:
resave in place, or save as a new document. It is a separate arm from
the scan's `DuplicateId` for exactly that reason; that one reports two
files that already exist and is fixed by deleting one.

`WorkspaceError::SaveTargetNotInStore { path }` is the second new arm:
a target that is not a `*.pncad` file of this store.

## What the viewer owes

1. `SessionOp::Save(PathBuf)` (`crates/viewer/src/session/op.rs:224`)
   routes through `Workspace::save_at` and surfaces
   `save_would_duplicate_id` as a refusal. Today it does not touch the
   store at all: `DocSession::save`
   (`crates/viewer/src/session.rs:1517`) calls `docio::save_path`,
   which is `persist::save` + `std::fs::write`
   (`crates/viewer/src/docio.rs:171`) — the one production write of a
   document that goes around the store, and the reason the ruled
   refusal cannot fire from the viewer today.
2. A second op spelling the fork — the save-as-new-document act —
   which is what the person picks when they wanted a copy.
3. The `SessionOp::Save` doc comment's "**Saving a COPY beside the
   original bricks the store for both** (issue #1117)" paragraph
   becomes false once (1) lands, and is deleted with it. The save-as
   MOVES-the-seam paragraph beside it (issue #1387) is a different
   hazard and stays.

Filed on VIEW rather than CHROME: both programs' `paths` cover
`crates/viewer/src/*`, and VIEW's `keep_out` records that CHROME's
slate landed 2026-09-04 and has been dormant since. Re-home if that
reading is wrong.
