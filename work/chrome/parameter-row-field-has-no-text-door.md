---
id: parameter-row-field-has-no-text-door
kind: issue
title: A parameter row's value field is a bare DragValue — no parser, no unit authoring, no no-op guard
status: open
opened: 2026-09-04
refs: [1776]
---


## Finding

**A slot's value field is a text door; a document parameter's is a
bare `DragValue`.** `props`' module docs describe the text door as
"One field for numbers and expressions"
(`crates/viewer/src/props.rs:91-116`) with no disclaimer that it is a
slot's field only — the section was written when parameters had no
panel story at all, and CHROME unit 8 (PR 1776) brought the parameter
row up to the units half without bringing it up to this one. The
disclaimer is now in the docs; the affordance is not.

What the slot field has that the parameter field does not
(`slot_value_ui`, `crates/viewer/src/app.rs:4549-4626`, against the
`Selection::Param` arm at `:2927-2975`):

1. **A `custom_parser` routing typed text through `props::field_edit`**
   (`app.rs:4583-4596`). It is what makes `50 mm` an authoring of a
   unit and `base_r * 2` an authoring of an expression. The parameter
   field has none, so egui's own number parse is the whole door: text
   is a number or it is nothing.
2. **The "text that says what the slot already says is not an edit"
   guard** (`app.rs:4611-4626`). The field commits on leaving it, so
   clicking into one and clicking away again must not cost an undo
   step. The parameter field has no such guard.

## What each half is worth, and what each half needs

- **The expression half does not apply.** A `DocParam::Continuous`
  holds an `f64`, not an `Expr` (`crates/editor-core/src/doc.rs:39-80`),
  so there is no expression a parameter can be set to and no
  `SetDocParamExpression` to reach. Typing `base_r * 2` into a
  parameter field is a refusal, not a door — and saying so is itself
  an affordance the field currently does not offer.
- **The unit half is `50 mm`.** Setting the value AND its notation
  from one piece of text needs the kernel door that does not exist
  (`work/issues/doc-param-unit-edit-has-no-door.md`) — unless the
  typed unit is the one the parameter is already declared in, which is
  a `SetDocParamValue` and works today.
- **The no-op guard is free** and belongs to this crate alone.

## Its relation to the sibling item

`work/chrome/add-parameter-form-authors-canonical-only.md` is the
CREATE door's version of the same question. They are two items
deliberately: that one mints a declaration (and can be done today
through `written_length`/`written_angle`), this one edits a standing
one (and its unit half cannot). Whoever takes either should read the
other — the design call about how notation crosses `props` is shared.

## Home

CHROME. The field is `crates/viewer/src/app.rs`; the parse door is
`crates/viewer/src/props.rs`. The unit half is blocked on the
`editor-core` issue above; the guard and the refusal wording are not.
