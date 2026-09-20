---
id: the-viewer-keeps-its-profile-lock-and-order-search-after-set-program
kind: issue
title: The viewer's profile editor still locks the shape and searches a write order: SetProgram exists and the lock, program_edits, accepted_order and three refusals are droppable
status: open
opened: 2026-09-20
refs: [a-committed-profile-program-has-no-whole-program-edit]
---

## The finding

`DocEdit::SetProgram` (EDIT-PROGRAM, ruled by Ev on `[ev]` #2904)
replaces a live profile's program whole — its loops, verbs, order and
count, arc modes, targets, a split circle's `n` — validated once, under
a stated provenance, and rebinds or reports every name its reshaping
touches (`crates/editor-core/src/edit.rs`, `DocEdit::SetProgram`;
`crates/profile/README.md` V2; `crates/editor-core/REFERENCES.md`
DM7). The viewer's profile editor was built before that door existed
and still pays the cost the missing door imposed. Every site below was
named by the unit's survey and is untouched by it — the kernel unit
does not cross into `crates/viewer` — so this row is VIEW's follow-up.

## The sites

- `crates/viewer/src/session.rs`, `DocSession::edit_profile`
  (~1935/1962–2004): the whole program is checked once through
  `ProfileProgram::check` and then written as a list of `SetParam`s
  fed to `accepted_order`. The natural spelling now is ONE
  `DocEdit::SetProgram { node, loops, provenance }` committed through
  `commit`, with the provenance the editor already knows (it inserted
  the leg; a moved number is the identity provenance —
  `LoopProvenance::identity(&loops)`).
- `crates/viewer/src/sketch.rs`, `program_edits` (~538–612) and
  `Restructure`: the per-slot diff whose only remaining job was to
  refuse a reshaped program. Its refusal is the door behind the lock
  and goes with the lock.
- `crates/viewer/src/forms.rs`, `ShapeEdits::Locked` and
  `SHAPE_LOCKED` (~235–253): the shape controls drawn disabled on a
  committed profile, and the sentence over them saying the document
  has no edit that rewrites a committed profile's program. Both are
  false now.
- `crates/viewer/src/session/refuse.rs`, `Refusal::ProfileRestructure`,
  `Refusal::ProfileEditOrder`, `Refusal::ProfileEditOrderCapped`
  (~266–348): three refusals about states nobody wrote — the reshaped
  program the lock refused, and the intermediate programs the order
  search could not thread. A whole-program write has no intermediate
  state, so the second and third have nothing left to say.
- `crates/viewer/src/session/op.rs`, `SessionOp::EditProfile`'s doc
  (~448–465): carries the lock and the order search as its rules.
- `crates/viewer/src/session.rs`, `accepted_order` and
  `ORDER_SEARCH_CAP` (~2330–2480): the depth-first search over write
  orders, exact up to twelve writes, and its cap.
- `crates/viewer/tests/profile_edit.rs`:
  `a_reshaped_program_refuses_restructure`,
  `a_move_whose_first_write_alone_crosses_still_lands`,
  `numbers_no_order_reaches_refuse_edit_order`,
  `a_move_past_the_search_cap_says_it_was_capped` — the rows that pin
  the lock and the order search; the first becomes "a reshaped program
  lands as one edit and its names follow", the other three retire with
  the search (a set of numbers valid together is one edit now, whatever
  order the slots would have needed).

## What the door reports, and the editor has to surface

`Applied.maintenance` after a `SetProgram` carries every
`Maintenance::Rebound { from, to }` (a name on a kept step, rewritten
in place) and every `Maintenance::Strand` / `StrandedAppearance` (a
name on a dropped or changed step, retired past the loop's end,
resolving `Vanished` until rebound). The chrome's cascade affordance
shows a delete's strand count already; a reshaping's rows want the same
surface, since a strand here is the fillet the author is about to lose.

## Sweep

`re-authoring`, `Locked`, `program_edits`, `accepted_order`,
`one argument`, `ProfileRestructure` over `crates/viewer`: every hit is
one of the sites above (`session.rs`, `sketch.rs`, `forms.rs`,
`session/refuse.rs`, `session/op.rs`, `tests/profile_edit.rs`, and
`pane/create.rs`'s `path_steps_ui`, which reads `ShapeEdits`). The
pattern cannot match a control that is disabled by a different word
than `Locked` — none was found by reading `path_steps_ui`, which takes
the enum.
