---
id: a-value-edit-never-re-asks-the-door-checks-that-read-values
kind: issue
title: A value edit never re-asks the edit-door checks that read the current values
status: open
opened: 2026-10-28
priority: P1
cost: M
design: true
---

Raised by the STYLE review of PR 5926 (F2).

Some of the edit door's checks read a node's slots under the document's
current values rather than its data alone. They run when a door writes
the node (`check_written_node` in `crates/editor-core/src/edit.rs`, and
`set_slot` after a slot edit). `DocEdit::SetVarValue` (`write_free`)
writes the variable and no reader of it, so none of these checks is
asked again when a value they read moves:

- `check_loft_shape`'s `LoftFault::DegreeOutOfRange`: the v degree read
  off a count variable;
- `check_loft_shape`'s `LoftFault::StationInFirstPlane`: a station's
  composed plane, via `station_keeps_the_first_plane`;
- `check_written_node`'s profile program replay (`EditError::ProfileProgramRefused`,
  `p.check(&new.var_env::<f64>(), tol)`), which is the same shape.

So a value edit admits a document that the door would refuse if the
same node were written again. Evaluation refuses it instead (a loft's
`LoftError::DegenerateStacking`, `need_count`'s range). The next
unrelated edit that writes the node then refuses at the door, naming a
fault that edit did not cause. `a_value_edit_to_a_zero_slide_is_refused_at_evaluation`
(`crates/editor-core/tests/intent_loft_set_sections.rs`) pins the loft
station case as it stands.

**Done when** the class has one answer, which is a design call:
- either a value edit re-asks the value-reading checks of the variable's
  readers;
- or these checks are said to be authoring-time conveniences, and
  evaluation is the judge.

If the second, the later unrelated edit should not refuse for a fault it
did not cause.
