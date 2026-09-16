---
id: parameter-row-field-has-no-text-door
kind: issue
title: A parameter row's value field is a bare DragValue — no parser, no unit authoring, no no-op guard
status: parked
opened: 2026-09-04
refs: [1776]
blocked_on: [doc-param-unit-edit-has-no-door]
---


## Finding

**A slot's value field is a text door; a document parameter's is a
bare `DragValue`.** `props`' module docs describe the text door as
"One field for numbers and expressions"
(`crates/viewer/src/props.rs`, the module docs' own section heading)
with no disclaimer that it is a slot's field only — the section was
written when parameters had no panel story at all, and CHROME unit 8
(PR 1776) brought the parameter row up to the units half without
bringing it up to this one. The disclaimer is now in the docs; the
affordance is not.

What the slot field has that the parameter field does not
(`ViewerBehavior::slot_value_ui` against the `Selection::Param(name)`
arm of `ViewerBehavior::properties_ui` — both in
`crates/viewer/src/pane/properties.rs`):

1. **A `custom_parser` routing typed text through `props::field_edit`**
   — the `.custom_parser(…)` arm of `slot_value_ui`. It is what makes
   `50 mm` an authoring of a unit and `base_r * 2` an authoring of an
   expression. The parameter field has none, so egui's own number parse
   is the whole door: text is a number or it is nothing.
2. **The "text that says what the slot already says is not an edit"
   guard** — the `match typed.into_inner()` arm of `slot_value_ui`,
   under the comment that names it. The field commits on leaving it, so
   clicking into one and clicking away again must not cost an undo
   step. The parameter field has no such guard.

## What each half is worth, and what each half needs

- **The expression half does not apply.** A `DocParam::Continuous`
  holds an `f64`, not an `Expr` (`DocParam` in
  `crates/editor-core/src/doc.rs`),
  so there is no expression a parameter can be set to and no
  `SetDocParamExpression` to reach. Typing `base_r * 2` into a
  parameter field is a refusal, not a door — and saying so is itself
  an affordance the field currently does not offer.
- **The unit half is `50 mm`.** Setting the value AND its notation
  from one piece of text needs the kernel door that does not exist
  (`work/edit/doc-param-unit-edit-has-no-door.md`) — unless the
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

CHROME. The field is `crates/viewer/src/pane/properties.rs`
(`ViewerBehavior::properties_ui`'s `Selection::Param` arm, with
`ViewerBehavior::slot_value_ui` beside it as the shape to copy); the
parse door is `crates/viewer/src/props.rs` (`props::field_edit`). The
unit half is blocked on the `editor-core` issue above; the guard and
the refusal wording are not.

## The fired entry pruned, still parked (2026-09-04)

`viewer-session-god-module-split` closed on 2026-09-04. This row also
waits on `doc-param-unit-edit-has-no-door`, which is open, so `parked`
is still TRUE of it and only the fired entry leaves `blocked_on`.
Pruned from VIEW's PR #1857 for the reason the eight sibling rows carry:
a `blocked_on` entry that has fired is stale data whether or not the
status it supports is still true. It is a lint WARNING rather than an
error precisely because this row is genuinely still blocked.

## Re-pointed by subject, still parked (2026-09-15, `chrome/citation-repoint`)

Four `app.rs` bands in this row named lines past the end of that file —
they were addresses in the 5,696-line `app.rs` that
`viewer-session-god-module-split` (#1830) took apart. All four subjects
now live in `crates/viewer/src/pane/properties.rs` and are re-derived
above **by name**, with no line number written, per
`docs/prompts/implementer-discipline.md` §7. The report that found
them, `parameter-row-field-cites-a-pre-split-app-rs`, is closed by this
pass.

Two further corrections the same pass made, neither of them a number:

- `work/issues/doc-param-unit-edit-has-no-door.md` is now
  `work/edit/doc-param-unit-edit-has-no-door.md` — the item was
  claimed by EDIT, not deleted. It is **open**, which is why this row
  is still `parked` and its `blocked_on` still resolves.
- `## Home` said the field is `crates/viewer/src/app.rs`. It is not,
  and had not been since #1830.

**What VIEW narrowed underneath this row, carried here so it does not
die with the report that disclosed it.**
`work/view/a-drag-field-renders-a-length-at-a-precision-its-drag-speed-sets`
gave every numeric field in the crate — the parameter row's included —
a text that reads back as the value it holds, so a click-in/click-away
is no longer DESTRUCTIVE (a parameter holding 40 nm used to become a
parameter holding zero). It is still an EDIT: the render is accepted
within `crate::readout::REL_TOLERANCE`, so the round trip can still
move a parameter by up to 5·10⁻⁴ of itself and cost an undo step for a
click nobody meant as one. The no-op guard this row asks for is
therefore still owed, with a smaller stake.
`work/view/a-fields-text-commits-within-the-renders-own-tolerance` is
the same residue read from the other side and is open.
