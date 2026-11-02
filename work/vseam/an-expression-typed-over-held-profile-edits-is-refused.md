---
id: an-expression-typed-over-held-profile-edits-is-refused
kind: issue
title: An expression typed into a profile editor field while the draft holds edits is refused, not carried into the draft
status: open
opened: 2026-10-31
priority: P3
cost: M
refs: [the-profile-editors-fields-carry-no-per-argument-affordances, 6144]
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
