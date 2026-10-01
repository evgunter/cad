---
name: docs-ledger
description: process docs (logs, specs, plans) are pruned when a program closes — anything worth referencing later gets a short note in docs/doc-ledger/ with a commit hash, and permanent docs do not cite process docs
metadata:
  type: convention
---

prune process docs (logs, specs, plans...) when the program closes. if
anything would plausibly want to reference them later, add a file to
docs/doc-ledger with a SHORT note about what was deleted and a commit
hash from before the deletion.

everything a program writes under work/ is a process doc, and older
programs put process docs straight in docs/ too, so location isn't the
test: process docs are about doing the work, permanent docs are about
how the code is. anything in a process doc worth keeping (why the code
took this shape, anything user- or dev-relevant) moves out before the
delete, into a present-tense README next to the code.

permanent docs ideally shouldn't reference process docs in the first
place, but if they do, when deleting, those references should be (a)
deleted or (b) if truly necessary, the info should be moved out of the
process doc to a permanent doc (e.g. the code's new README). don't
bother updating references on process docs.
