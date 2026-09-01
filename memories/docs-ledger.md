---
name: docs-ledger
description: docs/ is pruned rather than archived — deleted documents are recorded in docs/DOC-LEDGER.md, so a pointer into docs/ that finds nothing means look it up, not that the pointer is wrong
metadata:
  type: convention
---

`docs/` is pruned, not archived. A document whose subject is finished
is DELETED and its filename recorded in **`docs/DOC-LEDGER.md`**, with
why it went and the SHA it is recoverable from. The repo is merge-only,
so git is the archive; the ledger is the index git cannot give you,
since a deleted path is invisible to anyone who does not know its name.

**A pointer into `docs/` that finds nothing is not a wrong pointer.**
Look the filename up in the ledger, then `git show <sweep-sha>:<path>`.
Append-only logs still cite deleted files by name and are not edited in
place to match. Note that these worktrees are SHALLOW clones, so
`git log -G` and `git show` find nothing until `git fetch --deepen` or
`--unshallow` has run — "it is in git" is not reachable by default.

**Source comments are the exception: code may not cite a document that
gets pruned** (Evan, 2026-09-01), by filename or by the document's own
finding/row numbers. A pruned doc is a pointer with an expiry, and the
comment outlives it — so if the comment needs the context, the context
goes IN the comment. Citing a ratified contract (`DESIGN.md` and the
other `*-DESIGN.md`) or a committed data file is different: those are
not pruned when work closes.

Deleting a document and recording it in the ledger are one commit.
