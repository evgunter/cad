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
- `crates/viewer/src/drafts.rs`, five `sketch::program_edits` sites
  (287, 332, 340, 969, 979, 1014 — the doc references and the three
  calls): a draft's held loops are measured against the base program
  through the per-slot diff, and a reshaped draft is what that diff
  refuses; with `SetProgram` a draft is one edit and the diff has no
  question left to ask.
- `crates/viewer/src/pane/profile.rs`, four `ShapeEdits::Locked`
  sites (40, 86, 211, 474): the profile pane draws its step lists
  locked on a committed profile, the second place the lock is
  spelled.
- `crates/viewer/tests/profile_edit_order.rs`:
  `accepted_order_refuses_only_when_no_order_lands` (104) and the
  `sketch::program_edits` call at 160 — the order search's own rows,
  which retire with it.
- `crates/viewer/tests/panel_edits.rs`, `ProfileRestructure` (458): a
  panel row pinning the lock's refusal, which becomes the one-edit
  landing.

## What the door reports, and the editor has to surface

`Applied.maintenance` after a `SetProgram` carries every
`Maintenance::Rebound { from, to }` (a name on a kept step, rewritten
in place) and every `Maintenance::Strand` / `StrandedAppearance` (a
name on a dropped or changed step, retired to a coordinate at or above
`editor_core::RETIRED_FLOOR` that no program draws, resolving
`Vanished` until rebound). The chrome's cascade affordance
shows a delete's strand count already; a reshaping's rows want the same
surface, since a strand here is the fillet the author is about to lose.

## Sweep

The pattern that found the sites above is
`rg -il 're-author|ProfileRestructure|ShapeEdits::Locked|accepted_order|program_edits' crates/viewer`,
which names eleven files: `src/session.rs`, `src/sketch.rs`,
`src/forms.rs`, `src/session/refuse.rs`, `src/session/op.rs`,
`src/drafts.rs`, `src/pane/profile.rs`, `tests/profile_edit.rs`,
`tests/profile_edit_order.rs`, `tests/panel_edits.rs` — every one a
site listed above — and `tests/valid_range.rs`, whose one hit is the
word "millimetre-authored" (the pattern matches inside it) and is not
a site. `pane/create.rs`'s `path_steps_ui` reads `ShapeEdits` by the
enum and matches no word of the pattern; it was found by reading. The
pattern cannot match a control that is disabled by a different word
than `Locked` — none was found by reading `path_steps_ui`, which takes
the enum.
