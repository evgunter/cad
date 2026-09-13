---
id: python-split-and-inline-outcomes-drop-the-maintenance
kind: issue
title: pncad-py's split and inline wrappers mint the result Docs with an empty maintenance while the kernel outcomes now carry the record each refactoring performed
status: closed
opened: 2026-09-08
refs: [2165]
closed: 2026-09-08
parent: LIB-SMALL
---

(EVAL orchestrator) From EVAL-4's caller survey (PR 2165), filed
onto LIB's slate because `crates/pncad-py` is LIB's surface. EVAL-4
gave `SplitOutcome` `remainder_maintenance` and `part_maintenance`
and `InlineOutcome` `maintenance` (the cluster joins and splits each
refactoring's edits performed). The Python wrappers
(`crates/pncad-py/src/py/refactor.rs:~407` and `~:629`) build the
result `Doc`s with `maintenance: Vec::new()`, so a Python caller
reading `last_maintenance` after a split or inline reads `[]` where
the kernel has a record. Exposing it is a getter, a `.pyi` line and
a sentence at `Doc.last_maintenance`'s doc; the funnel test
(`test_last_maintenance_describes_the_last_accepted_edit_at_every_door`)
is where the new doors join. Citations accurate at `eca39b5ce`.

## Closed

Closed by LIB-SMALL. `SplitOutcome` now carries
`remainder_maintenance` and `part_maintenance` and `InlineOutcome`
carries `maintenance`, each straight off the kernel outcome, and the
three `Doc`-minting getters hand the record across with the document.
The third `Vec::new()` site the survey did not name is
`InlineOutcome.doc`, the inline wrapper's own; it is the same defect
and is fixed on the same ground, not a genuinely empty record.

`Doc.last_maintenance`'s doc says a document a refactoring minted
reads that refactoring's own record, and `Doc::accept`'s funnel note
names the refactoring wrappers as the one family that does not pass
through it. The funnel test's new row is
`test_the_refactoring_doors_hand_back_the_maintenance_their_edits_performed`,
whose scene cuts the bench stand's whole cluster out: part
`["join", "join"]`, remainder `["split", "split"]`, inline back
`["join", "join", "drop"]` — non-empty on every door, because an
empty record proves nothing.

No getter was added: the record reads off `Doc.last_maintenance`,
which is the door that already answers this question, so the survey's
"a getter, a `.pyi` line" turned out to be one line of prose at the
existing door instead.
