---
id: save-as-new-document-writes-at-a-chosen-name
kind: issue
title: Workspace::save_as_new_document takes the target file name, so the viewer's fork has one door
status: closed
opened: 2026-10-31
closed: 2026-10-31
pr: 6135
priority: P1
cost: E
refs: [session-save-is-two-acts]
---

Filed by the VSEAM orchestrator from `[ev]` PR 6123 (2026-10-31), where
both designers found this gap.

`Workspace::save_as_new_document` (`crates/pncad/src/workspace.rs`)
always writes the fork at `{newid}.pncad`. That suits a deterministic
caller such as split (D9). An interactive Save as New Document… names
its own file, and nobody wants a file name that is a hex id. The door
should take `target` the way `save_at` does, through the same
`store_path`, or stand as sugar over minting an id and calling
`save_at`.

Until it does, `work/vseam/session-save-is-two-acts` composes the
fork in the viewer: `random_document_id()`, `under_identity`, then
`save_at` at the chosen name. That is a second spelling of the fork.
Once this door takes a target, the viewer calls it instead and the
composition goes. Repointing the viewer is one call, and it rides
this row's PR or a VSEAM follow-up.

## 2026-10-31 — closed (PR 6135)

`Workspace::save_as_new_document(doc, target, tol)` mints a random id and
writes through `save_at`, so the fork reads `target` the same way
`store_path` does and refuses the same way, `SaveWouldReplaceDocument`
included. No deterministic caller used the `{newid}.pncad` spelling:
split writes through `create`. The callers were updated: pncad's
store tests, pncad-py's `Workspace.save_as_new_document(doc, target)`
(the stub, the typing fixture, `test_workspace.py`), and the viewer.
The viewer's own fork now calls this door through
`docio::save_as_new_document`, so the second spelling is gone.
VSEAM's lane made this change on LIB's ground, authorised by the
VSEAM orchestrator.
