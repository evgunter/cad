---
id: parameter-row-field-cites-a-pre-split-app-rs
kind: issue
title: parameter-row-field-has-no-text-door cites four app.rs bands past the end of the file, all four for code the split moved to pane/properties.rs
status: open
opened: 2026-09-12
---



Found by VIEW's `view/drag-field-precision` lane while reading
`work/chrome/parameter-row-field-has-no-text-door` for a duplicate
before filing its own residue. The subject is CHROME's slate, so this
is a report rather than a change, and it follows the precedent
`drag-tick-row-cites-app-rs-for-a-finding-that-lives-in-forms-rs` set.

**The finding in that row is intact. Four of its addresses are not**,
and nothing here says it should close — its no-op-guard half is still
open, and the unit that prompted this reading narrowed it without
taking it (below).

## The four, and the cheapest check that catches all of them

`crates/viewer/src/app.rs` is **2018 lines**. Every one of these names
a line past the end of it, so no shift map applies: a diff cannot move
a line that is not in the file. They are pre-split addresses, from the
5,696-line `app.rs` that `viewer-session-god-module-split` (#1830)
took apart.

| in the row | what it names | where that is now |
|---|---|---|
| `app.rs:4549-4626` | `slot_value_ui`, the slot's text door | `crates/viewer/src/pane/properties.rs`, `ViewerBehavior::slot_value_ui` |
| `app.rs:4583-4596` | its `custom_parser` through `props::field_edit` | same function, the `.custom_parser(…)` arm |
| `app.rs:4611-4626` | the *"text that says what the slot already says is not an edit"* guard | same function, the `match typed.into_inner()` arm under that comment |
| `app.rs:2927-2975` | the `Selection::Param` arm the row contrasts against | `crates/viewer/src/pane/properties.rs`, the `Selection::Param(name)` arm of `ViewerBehavior::properties_ui` |

The new homes are named by SUBJECT rather than by number on purpose:
the lane that filed this was editing `pane/properties.rs` in the same
PR, so any line number it wrote would have been wrong before this file
reached `main`.

## Not stale, checked and left alone

Two of the row's other citations were read and are correct about their
subjects, so this is a report on four addresses and not on the row:

- `crates/viewer/src/props.rs:91-116` — the *"One field for numbers and
  expressions"* section is at `:91`, and it now carries the
  slot-only disclaimer the row says it gained.
- `crates/editor-core/src/doc.rs:39-80` — `DocParam` opens at `:39`
  in a 605-line file.

The row's `## Home` section also says *"The field is
`crates/viewer/src/app.rs`"*, which is the same fact in prose.

## What VIEW's unit changed underneath it, and what it did not

`work/view/a-drag-field-renders-a-length-at-a-precision-its-drag-speed-sets`
gave every numeric field in the crate — the parameter row's included —
a text that reads back as the value it holds. That narrows this row's
**no-op guard** half: the guard exists because a `DragValue` commits
its own rendered text on losing focus, so a render that misread the
value used to make that click destructive (a parameter holding 40 nm
became a parameter holding zero). It is no longer destructive.

It is still an EDIT: the render is accepted within
`crate::readout::REL_TOLERANCE`, so a click-in and a click-away can
still move a parameter by up to 5·10⁻⁴ of itself and cost an undo step
for a click nobody meant as one. The guard this row asks for is
therefore still owed, with a smaller stake.
VIEW's `a-fields-text-commits-within-the-renders-own-tolerance` is the
same residue read from the other side.
