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
