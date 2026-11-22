---
id: a-stranded-name-row-offers-no-rebind-in-the-viewer
kind: issue
title: viewer: a stranded-name row is a sentence on the status line, with no door to rebind or clear the name
status: open
opened: 2026-11-22
priority: P2
cost: H
design: true
---


Found by the lane for `cascade-delete-shows-the-strand-count`, which
owed "the delete's result surfaces the report where the user can reach
`Rebind`".

**What exists.** An accepted action's maintenance rows leave the
session on `OpOutcome::maintenance`, net over the action, and
`frame::outcome_notices` (`crates/viewer/src/frame.rs`) words each row
as one notice on the status line, in the row's own sentence
(`frame::maintenance_notice`). That is pinned for a delete's payload
strand and appearance strand by
`edit_maintenance::a_delete_that_strands_a_payload_name_reaches_the_line`
and `…_an_appearance_key_reaches_the_line`
(`crates/viewer/tests/edit_maintenance.rs`). The delete button states
the count before the click (`DeleteAffordance::stranded`).

**What does not.** The sentence ends "until it is rebound", and the
viewer has no door that rebinds: no `SessionOp` commits a
`DocEdit::Rebind` or a `DocEdit::ClearAppearance` for a name the
person picks out of a report (`rg -n 'Rebind' crates/viewer/src`
finds the edit only in the arm of `session.rs` that classes edits).
The notices are text; the next act takes them off the line; nothing
lists the names a document currently holds stranded, so a reopened
document shows none of them (maintenance is derived per edit and never
persisted).

**What a door needs decided** before a lane builds it:

- where the stranded names are listed once the status line has moved
  on — per carrier in the properties pane, or one list for the
  document. No kernel query for "every name this document holds
  whose referent is gone" was found (no public function of
  `crates/editor-core/src` is named for a strand, a dangling or a gone
  name); the rows exist only at the edit;
- how the person says what the name should denote instead — a pick in
  the viewport is the natural spelling of `Rebind`'s `to`;
- what an appearance key offers: `Rebind`, or `ClearAppearance`, which
  does not need a live node.

Related: `a-refusal-offers-no-action-in-the-viewer` (a recourse shown
as prose), and CHROME's
`a-long-cascades-maintenance-crowds-the-status-line`, which points at
this list as where the per-row sentences belong.
