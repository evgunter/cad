---
id: an-expression-typed-over-held-profile-edits-is-refused
kind: issue
title: An expression typed into a profile editor field while the draft holds edits is refused, not carried into the draft
status: closed
closed: 2026-11-02
opened: 2026-10-31
priority: P1
cost: M
refs: [the-profile-editors-fields-carry-no-per-argument-affordances, 6144]
parent: the-profile-create-form-is-a-second-profile-editor
branch: forms/held-expression
---


## The finding

Filed by PR 6144, which gave the profile editor's fields their slots'
expression door (`pane::properties` `ViewerBehavior::held_slot_field_ui`).

An expression typed into a field lands in the document at once
(`SessionOp::SetSlotExpression`). That moves the program the edit draft was
loaded from, and `Drafts::profile_edit` reloads the draft. So while the
draft holds unapplied edits, the field refuses the text with
`HELD_EDITS_FIRST` ("apply or revert them first") rather than dropping
those edits.

## The better shape

The draft holds the typed expression beside the number, as it already holds
a driven argument's expression (`ProfileEdit::driven`, written back by
`ProfileEdit::programs_noted`), and Apply commits it inside the one
`SetProgram`. The cost is parsing the text against the document's variables
in the draft rather than in the session's `SetSlotExpression` door, and
saying a refused parse at the field before Apply.

## 2026-11-02 — the draft now holds `Formula`s (PR 6302)

`ProfileEdit::driven` and `ProfileEdit::programs_noted`, named above,
are gone: the draft holds the authored program, one `sketch::HeldStep`
(a one-step `LoopProgram<Formula>`) per step, and a driven argument is
simply its expression there, written back with the rest of the step
(`HeldStep::written`) and committed in the one `SetProgram`. So the
better shape is cheaper than it reads: an expression typed over held
edits is a formula written into the held step's argument, and only the
parse against the document's variables, and saying a refused parse at
the field, remain to build.

## Closed — 2026-11-02, branch `forms/held-expression`

Every argument of the profile editor, of a committed step or of one
the draft made, is drawn over the argument as the draft holds it
(`pane::profile`'s `held_field`, over `ProfileEdit::held_row`). Text
typed into it is parsed by the slot door's own parse
(`session::parse_text`) and written into the held step
(`ProfileEdit::write_text`, `HeldStep::with_arg`), committed in the one
`SetProgram`; a refused parse is held in its field with the refusal's
sentence. A driven argument previews at what its formula reads under
the document's variables. `HELD_EDITS_FIRST` and the editor's route
through `SetSlotExpression` are gone. Residue:
`the-profile-editors-naming-and-offer-doors-wait-on-held-edits`
(this slate) and
`work/placing/the-placement-editors-typed-value-says-a-parse-refusal-in-its-own-words.md`.
