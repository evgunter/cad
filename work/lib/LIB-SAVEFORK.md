---
id: LIB-SAVEFORK
kind: unit
title: a save is two acts: save-at-path refuses a duplicate id; fork mints a fresh one
status: review
branch: lib/savefork
refs: [save-a-copy-duplicate-id-bricks-store]
opened: 2026-09-08
pr: 2184
---

Builds Ev's ruling (A) at PR 2016, recorded at ASSEMBLY.md A4: a save
is two acts.

`Workspace::save_at` writes a document at a caller-named file of the
store and KEEPS its identity, refusing typed
(`WorkspaceError::SaveWouldDuplicateId`) before any write when the
store already holds that id under another name — so the ordinary "save
a copy beside the original" no longer leaves a duplicate that makes
every later scan refuse for every document in the store. At the id's
own scanned path it is a resave; for an unclaimed id it is a create at
the caller's name.

`Workspace::save_as_new_document` is the other act: the same content
under a fresh random id at `{newid}.pncad`, through `create`'s
validator, answering `(DocumentId, PathBuf)`. The original is
untouched, so every inbound `DocRef` pinning the old id still resolves
to it; the fork's content pin EQUALS the original's, because the pin's
preimage excludes the `id` key.

Both doors are bound on Python's `Workspace` with the two new tags.
The viewer half — `SessionOp::Save` routing through the save door and
a save-as-new-document op — is
`work/view/session-save-is-two-acts.md`.
