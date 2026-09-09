---
id: cross-list-payload-rungs-under-document-only-carriers
kind: issue
title: two select-list payloads ride document-only carriers, which the four-list sweep can now see
status: open
opened: 2026-09-09
refs: [LIB-SWEEP, payload-rung-sweep-is-prose-and-a-third-run-disagrees]
---


`scripts/payload-rung-sweep.py` reads all four curated façade lists, so
for the first time it can tell "uncurated" from "curated on a list I do
not read". That split has a second half nobody has looked at: a payload
that IS curated, on a list that does not carry its CARRIER. The
uncurated column at LIB-SWEEP's merge base has no new row; this column
has four, over two names.

| payload | on | carrier | carrier on | carrier at |
| --- | --- | --- | --- | --- |
| `EntityKind` (`crates/editor-core/src/names/role.rs:42`) | prelude, select | `NodeErrorKind` | document | `crates/editor-core/src/eval/mod.rs:675` |
| `EntityKind` | prelude, select | `RefusedRef` | document | `crates/editor-core/src/assembly.rs:278` |
| `SplitHalf` (`crates/editor-core/src/names/role.rs:189`) | prelude, select | `NodeErrorKind` | document | `crates/editor-core/src/eval/mod.rs:675` |
| `SplitHalf` | prelude, select | `PartSelect` | document | `crates/editor-core/src/node.rs:919` |

Reproduce with `python3 scripts/payload-rung-sweep.py`, the NARROWED
CROSS-LIST table.

## Why it is a question and not obviously a defect

The document list states a payload rule — a payload rides with its
carrier so a consumer can match the variant AND name what it caught,
which is why `VerbKind`/`Arity` are on it beside `NodeErrorKind`
(`crates/pncad/src/document.rs:163`). By that rule these four rows are
gaps: a consumer of `pncad::document` alone matches the arm and cannot
name what it holds.

The counter-argument is in the tree too, one file over.
`crates/pncad/src/select.rs:57` carries `NamingError` "by the payload
rule `crate::document` states" — the payload of
`NodeErrorKind::Naming`, placed on the list of the module that RAISES
it rather than the list of its carrier. `EntityKind` and `SplitHalf`
are that same naming vocabulary, and both are already on `select`
beside `RoleSeg` and `Denotation`.

So the two readings are:

1. The `NamingError` precedent generalises — the naming vocabulary
   lives on `select`, a document-layer consumer reaches it there, and
   these four rows are the rule working. Then what is missing is one
   sentence saying so, next to the precedent, so the next sweep reads
   a disposition instead of a finding.
2. The precedent is about one name, and the document list's own rule
   is the binding one. Then `EntityKind` and `SplitHalf` belong on
   `crate::document` too, the way `VerbKind` is.

## What a unit closing it would decide

Which reading holds, and it is a curation act either way — a list
moves, or an argument is written where the two lists meet. Deciding it
also settles the general question, because the cross-list column is
new and this is the whole of it: whether a payload owes its carrier's
list, or the list of the module that owns the vocabulary.

Until it is decided the two names are pinned in
`CROSS_LIST_DISPOSITIONS` in the script, pointed at this file, so a
THIRD cross-list name reds CI rather than joining them silently.
