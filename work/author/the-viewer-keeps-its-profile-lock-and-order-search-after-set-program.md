---
id: the-viewer-keeps-its-profile-lock-and-order-search-after-set-program
kind: issue
title: The viewer's profile editor still locks the shape and searches a write order: SetProgram exists and the lock, program_edits, accepted_order and three refusals are droppable
status: closed
opened: 2026-09-20
priority: P0
cost: D
closed: 2026-09-30
pr: 3446
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
does not cross into `crates/viewer` — so this row is AUTHOR's follow-up (filed on VIEW's slate, re-homed when VIEW left the tracker).

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

## Triaged P0 and dispatched (2026-09-29)

Filed with no priority. Triaged **P0**: a committed sketch's shape controls are drawn disabled under a sentence (`SHAPE_LOCKED`) that has been false since `DocEdit::SetProgram` landed, so reshaping a sketch after committing it is a door the GUI cannot author, which is AUTHOR's charter. Dispatched as **AUTH-6** (`docs/AUTH-6-SPEC.md`, branch `author/profile-reshape`) in parallel with AUTH-5, which works in `sketch::preview`.

## AUTH-6 landed the unit (2026-09-29, `author/profile-reshape`)

The edit door commits one `DocEdit::SetProgram`. The lock,
`program_edits`/`Restructure`, `accepted_order`/`ORDER_SEARCH_CAP` and
the three refusals are gone, along with the rows that pinned them. The
three design calls and the mutation table are in the PR body.

**Two premises above were stale, and so was the spec.** Since #3193
(2026-09-25), `SetProgram` carries `ids: Vec<Vec<Option<StepId>>>`, not
a provenance. `LoopProvenance::identity` no longer exists, and neither
does `Maintenance::Rebound`: a kept step's names keep their spelling and
nothing is rewritten. The editor's provenance is therefore which
committed step each held step IS (`ProfileEdit::ids`). The stale
sentence in `SlotId::Profile`'s doc is filed as
`work/recipe/slot-id-profile-doc-says-set-program-rebinds-names`. Also,
"the delete cascade's strand-count affordance" counts dependent
features, not strands, so the strand count on the edit door's Apply is
new, modelled on it.

**The sweep's blind spot, checked.** Grepping the TYPE `ShapeEdits`
rather than the variant `Locked` found a twelfth file the row's pattern
missed: `widgets.rs`. Its `path_step_fields`, `arc_fields` and
`target_fields` took `shape: ShapeEdits` and disabled the arc-mode,
side, winding, target-form and split-count controls with
`shape.free()`. No word of the row's pattern appears there. All are
live now. A second pass, `rg 'add_enabled|add_enabled_ui'` over
`pane/profile.rs`, `pane/create.rs` and `widgets.rs`, found no other
shape control disabled by a word other than `Locked`.

## Closed 2026-09-30 — PR 3446 merged (`75d074e3`)

**A committed sketch's shape is editable.** The edit door commits the
editor's whole program as one `DocEdit::SetProgram` — one edit, one
undo, whatever changed: numbers, steps inserted, removed or reordered,
verbs, arc modes, targets, a split circle's `n`. The lock, the per-slot
diff, the write-order search and its cap, and the three refusals about
states nobody writes any more are retired with the rows that pinned
them. Ev confirmed the P0 triage (2026-09-29).

**The tree had moved under the spec.** Since Ev's step-id ruling on
#3193, `SetProgram` carries per-step `StepId`s, not a provenance, and
rewrites no names. So the design call became *which committed step each
held step is*: the editor tracks it through every row edit rather than
guessing from values, and a held step keeps its id only while it draws
the committed step's pieces (the kernel's own `Step::pieces`). A verb
change, a new split count or a re-made step gets a fresh id, so its
names strand and are reported — before Apply on the button's hover, and
after it on the status line, in the kernel's own sentence.

**Review.** The correctness review could not falsify the id table
running edits through the real door; the before-Apply report matched
the commit in every case run. The style review found the no-op rule
and the strand sentence each with two homes; both now have one. The fix
pass rewrote the id rule onto the kernel's `Step::pieces` and proved it
identical to the old rule on all 1369 pairs of 37 steps.

Residue filed: `work/recipe/slot-id-profile-doc-says-set-program-rebinds-names`
(six stale kernel sentences), `work/recipe/set-program-ids-keep-all-is-hand-spelled-outside-the-kernel`,
and `work/wire/a-split-circles-phase-edit-re-aims-its-piece-names-silently`
— a name re-aimed with no report when a split circle's phase turns by
2π/n, pre-existing and on the naming layer's ground.
