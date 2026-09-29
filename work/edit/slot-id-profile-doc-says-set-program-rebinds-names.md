---
id: slot-id-profile-doc-says-set-program-rebinds-names
kind: issue
title: SlotId::Profile's doc says SetProgram rebinds every name it moves; since #3193 it rewrites none
status: open
opened: 2026-09-29
---

## The finding

`SlotId::Profile`'s doc (`crates/editor-core/src/node.rs`, the
`Profile` arm of `SlotId`, near line 419) says program structure changes
only by `DocEdit::SetProgram`, "which reports every name its reshaping
strands and rebinds every name it moves". Since the step-id ruling
(`crates/profile/README.md` V2, "ruled by Ev ... on #3193,
2026-09-25"), `SetProgram` rewrites no name. A kept step's names keep
their spelling, and a dropped step's names are reported
(`DocEdit::SetProgram`'s doc, "The names": "nothing is rewritten").
There is no `Maintenance::Rebound` arm left for the sentence to mean.

The same stale premise was in AUTHOR's
`the-viewer-keeps-its-profile-lock-and-order-search-after-set-program`
row and in `docs/AUTH-6-SPEC.md`, which asked for a
`LoopProvenance::identity` and a `Rebound { from, to }` that the tree
no longer has. AUTH-6 found it there.

## The fix

A one-clause doc edit: "which keeps every kept step's names and reports
every name on a step it drops". This is a re-wording that follows an
approved change, not a design change.
