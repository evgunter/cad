---
id: a-name-door-admits-a-piece-role-its-steps-verb-never-draws
kind: issue
title: The name-writing doors admit a profile piece whose role its step's verb never draws
status: open
opened: 2026-09-29
priority: P3
cost: M
design: true
---


## Finding

- **Where**: `crates/editor-core/src/edit.rs` — `check_name_steps`
  (~:2472), called by the `InsertNode`, `Rebind`, `SetAppearance` and
  `SetAppearanceMeta` arms of `apply`; and the load door's twin in
  `crates/editor-core/src/persist/check.rs` (the `NameStepNotMinted`
  walk, ~:1360).
- **What**: both doors check that every step a name spells is in the
  mint log (`StableName::piece_steps`), and nothing else about the
  locator. So a name spelling `{ step: <a line_to's id>, role: Arc }`,
  or `Piece(7)` on a `circle`, is admitted and resolves `Vanished`
  forever, although no value of the program can ever draw it. The
  piece door (`ProfileProgram::piece`, `crates/editor-core/src/step_handle.rs`)
  now refuses such a role against the one per-verb list
  (`profile::RoleList`), so the authored path cannot spell one; a name
  spelled any other way (hand-built, carried in from another document,
  loaded) still can. The Python binding has one such way on purpose:
  `Piece(StepId, Role)` (`crates/pncad-py/src/py/step.rs`, `Piece::new`)
  builds a piece from any id and role with no check, for a caller that
  holds a `StepId` across a `set_program` and no handle; it is a further
  door this row's check would have to cover.
- **Why it is not a one-line close**: the check needs the step's verb,
  which only a live profile holding the step knows. A dropped step's id
  stays in the mint log with no verb behind it, so its names cannot be
  checked and must stay admitted (they resolve `Vanished`, N1). And
  `SetProgram` keeps an id by the NEW program's address, so it can keep
  an old `line_to`'s id on a new `fillet`; the names already held on
  that id then spell a role the new verb never draws. A role check at
  the name doors alone would hold only until the next such reshape, and
  one at the load door would refuse a document `SetProgram` produced.
  Whether a keep across verbs whose role lists differ should refuse,
  strand the names on roles the new verb lacks, or stay as it is, is
  the question to settle first.
- **Importance**: low — the names denote nothing rather than the wrong
  thing, and are reported only as they fail to resolve.
