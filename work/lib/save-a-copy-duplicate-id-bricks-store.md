---
id: save-a-copy-duplicate-id-bricks-store
kind: issue
title: Save-a-copy beside the original bricks the workspace store (DuplicateId for the whole directory)
status: closed
opened: 2026-08-28
closed: 2026-09-08
github: 1117
refs: [1113]
---

## From GitHub issue 1117

Opened 2026-08-28; 0 comments.

Found by GUI-4's R2 review (a failing probe), banked from the fix pass of PR #1113.

**The act**: with a document open from `dir/a.pncad`, `SessionOp::Save(dir/b.pncad)` — "save a copy beside the original", an ordinary user act.

**The consequence**: the copy carries the same document id (`id:` header), so the directory now holds two files claiming one identity. The workspace scan refuses `DuplicateId` naming both paths, which means **every subsequent resolution through that directory refuses — for every document in the store, in every session**. Typed, honest, recoverable (delete either file), but the blast radius of the ordinary act is the whole store.

**Why no cheap fix shipped**: identity is the document's, not the file's (A4 — the id answers "which part"; pins answer "which version"), so a save cannot silently mint a fresh id without FORKING the document — every inbound `DocRef` pinning the old id would then miss the copy, which is correct for a fork and wrong for a backup. The right shape is a design question: an explicit "save as new document (fork identity)" act distinct from "save this document at a path", possibly with a save-door warning when the target directory already holds the id under a different filename.

**Where it is documented today** (the fix pass): `SessionOp::Save`'s doc carries the hazard and this issue's number; the store's own `DuplicateId` refusal already names both files.

Refs: PR #1113 (GUI-4), review R2 MINOR-4; `crates/pncad/src/workspace.rs` (`WorkspaceError::DuplicateId`), `crates/viewer/src/session.rs` (`SessionOp::Save`).

## Home

The workspace store is `crates/pncad/src/workspace.rs`, in LIB's `paths:` territory, and document identity is the library contract's ground.

**Re-homed to LIB (CHROME orchestrator, 2026-09-04).** DOCM's 2026-09-04
hand-off routed this to `work/chrome/`, whose `paths` do not cover
`crates/pncad` at all — so the program holding it could not have taken
it. The Home section above, written before that hand-off, already named
LIB, and the item's own account of the fix is a design question about
identity (an explicit "save as new document (fork identity)" act,
distinct from "save this document at a path"), which is the library
contract's to answer.

The viewer half is real but secondary: `SessionOp::Save`
(`crates/viewer/src/session.rs`) is where the act is spelled, and LIB's
`keep_out` fences the viewer. So this item's landing needs a viewer
rider once the identity question is ruled — which is a hand-off in the
other direction, after the decision, not before it.

## Question for Ev (2026-09-06, LIB orchestrator; `[ev]` PR)

The fix is a design fork about what a save IS, so it is asked rather
than dispatched. Three shapes, with the orchestrator's recommendation
first:

- **(A) Two acts.** `Save(path)` keeps the document's identity and,
  when the target directory already holds this id under a DIFFERENT
  filename, refuses typed at the save door (pre-empting the store's
  own `DuplicateId` rather than letting the store discover it later
  for every document). A second act, "save as new document", writes
  the same content under a FRESH id at the new path — an explicit
  fork, so every inbound `DocRef` pinning the old id keeps pointing
  at the original, which is what a fork means.
- **(B) Silent re-mint.** A save to a new path mints a fresh id.
  Rejected by the issue's own argument: it forks the document without
  saying so, and a backup and a fork are different acts.
- **(C) Leave it.** Typed, recoverable, documented at `SessionOp::Save`.
  The blast radius (the whole store refuses) is the cost.

Recommendation: (A). The library half (the save-door refusal and the
fork act) is LIB's in `crates/pncad/src/workspace.rs`; the viewer half
(spelling the second act in `SessionOp`) is a rider handed to the GUI
programs after the ruling, in that order.

## Ruled (Ev, PR 2016, 2026-09-06): **(A) — two acts**

`Save(path)` keeps the document's identity and refuses typed at the
save door when the target directory already holds this id under a
different filename; a separate "save as new document" act writes the
same content under a fresh id, an explicit fork. The library half
(the save-door refusal and the fork act, `crates/pncad/src/workspace.rs`)
is LIB's unit; the viewer spelling of the second act (`SessionOp`) is a
rider handed to the GUI programs after it lands. Dispatchable as a LIB
unit; the ruling is recorded here and at ASSEMBLY-DESIGN A4's clause.

## Closed (LIB-SAVEFORK, 2026-09-08)

Ruling (A) is built. Two doors on `Workspace`
(`crates/pncad/src/workspace.rs`), bound on Python's `Workspace` under
the same names:

- **`save_at(doc, target, tol) -> PathBuf`** — the ordinary save at a
  path, identity KEPT. Before any write it reads the scan: the id
  claimed at a DIFFERENT path refuses typed; claimed at `target` it is
  a resave; unclaimed it is a create at the caller's name (unlike
  `create`, which forces `{id}.pncad`). `target` is a `*.pncad` file
  directly in the store root — a different root is a different store,
  which this door does not copy between.
- **`save_as_new_document(doc, tol) -> (DocumentId, PathBuf)`** — the
  fork. Fresh random id at `{newid}.pncad`, written through `create`'s
  validator; the original untouched, so every inbound `DocRef` pinning
  the old id still resolves to it. The fork's content pin EQUALS the
  original's: the pin's preimage is the serde form with the `id` key
  removed, so a copy under a fresh identity is detectably the same
  content. The two save FILES differ, in the `id:` header and the
  snapshot's own id.

**The arm decision: a NEW arm**, `WorkspaceError::SaveWouldDuplicateId
{ id, existing, requested }`, not `DuplicateId` reused — the recourse
differs. The scan's `DuplicateId` reports two files that already exist
and is fixed by deleting one; the save door's reports a write that has
NOT happened and is fixed by choosing an act (resave in place, or
fork). A second new arm, `SaveTargetNotInStore { path }`, refuses a
target that is not a save file of this store. Both are matched
exhaustively at `resolve_fault`, `workspace_err` and the tag map.

**Does NOT cover**: the viewer half — `SessionOp::Save` is unchanged
(LIB's `keep_out` fences `crates/viewer`) and the hand-off is
`work/view/session-save-is-two-acts.md`; any change to how ids are
minted or to `DocRef`/pins; a "save with warning" shape (the ruling is
refuse typed); copying a document ACROSS stores.
