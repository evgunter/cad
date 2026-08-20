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
Append-only logs and source comments both still cite deleted files by
name, and are not edited in place to match.

Deleting a document and recording it in the ledger are one commit.
