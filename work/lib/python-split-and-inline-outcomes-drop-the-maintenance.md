---
id: python-split-and-inline-outcomes-drop-the-maintenance
kind: issue
title: pncad-py's split and inline wrappers mint the result Docs with an empty maintenance while the kernel outcomes now carry the record each refactoring performed
status: open
opened: 2026-09-08
refs: [2165]
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
