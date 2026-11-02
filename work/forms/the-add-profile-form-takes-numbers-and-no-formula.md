---
id: the-add-profile-form-takes-numbers-and-no-formula
kind: issue
title: The add-profile form takes only numbers: a step's argument cannot be written as a variable or a formula until the profile is committed
status: open
opened: 2026-10-31
priority: P1
---


Reported by Ev (chat, 2026-10-31): "it seems like variables can't be
entered in the profile". Filed P1 at Ev's request: a GUI defect Ev
reported, short of blocking because the edit door is a workaround.

## What happens

The add-profile form (`ViewerBehavior::add_profile_ui`,
`crates/viewer/src/pane/create.rs:707`) draws its step list through
`path_steps_ui` (`crates/viewer/src/pane/profile.rs:481`) with
`WrittenIn` as the argument doors (`create.rs:791-797`).
`WrittenIn::field` (`crates/viewer/src/widgets.rs:1328`) is a
`named_field` — a drag-value number in the form's notation — so the
form's drafts hold `f64`s (the `Step` values, e.g.
`crates/viewer/src/sketch.rs:141`, and `ProfileShape`, `sketch.rs:83`)
and `loop_program` (`sketch.rs:280`) lowers each to a written literal.
There is no field a name or an expression can be typed into, so `w`,
`w * 2` or `px - w` cannot be written at the door where a profile is
made.

The preview assumes as much: `sketch::preview` (`sketch.rs:1242`)
resolves the form's loops in a scratch document and treats a step that
reads a name as unreachable (`sketch.rs:1265`, "a form's program reads
no name").

## The workaround, and why it is not the fix

A committed profile's arguments are slots, and its edit door draws
each kept step's argument as the slot's field
(`HeldArgs`, `crates/viewer/src/pane/profile.rs:177`, into
`held_slot_field_ui`, `crates/viewer/src/pane/properties.rs:753`),
whose text door is `SessionOp::SetSlotExpression`. So a person can
commit numbers, open the profile, and retype each field as a formula —
but only while the draft holds no other edit (`properties.rs:794`,
refused with `HELD_EDITS_FIRST`, `:1338`), one field at a time, each
landing as its own edit. Every profile whose dimensions are parameters
has to be authored twice.

## What a fix has to settle

- The form's drafts hold numbers; a formula field needs a draft that
  holds text (or a `Formula`) per argument, lowered through the same
  formula door the slot fields use, with the same refusal shown at the
  field.
- The preview must resolve names against the document's variables
  rather than a scratch resolver that reads none, or say plainly that a
  formula field is previewed at its current value.
- The other creation forms in `create.rs` (17 `unit_field` /
  `point_fields` / `named_field` sites at this writing, the revolve
  tool's axis among them) have the same shape. Whether this row widens
  to them or each form gets its own is the FORMS orchestrator's call;
  the profile form is the one Ev hit.
