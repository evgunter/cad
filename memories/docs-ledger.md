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

deleted design docs which contain user- or dev-relevant info should be
replaced with a present-tense README next to the code.

permanent docs ideally shouldn't reference process docs in the first
place, but if they do, at the sweep, those references should be (a)
deleted or (b) if truly necessary, the info should be moved out of the
process doc to a permanent doc (e.g. the code's new README). don't
bother updating references on process docs.
