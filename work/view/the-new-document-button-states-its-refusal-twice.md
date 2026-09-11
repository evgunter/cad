---
id: the-new-document-button-states-its-refusal-twice
kind: issue
title: The New document button's disabled reason is a literal beside a comment claiming Refusal::EmptyName backs it, and the two sentences differ
status: open
opened: 2026-09-11
refs: [a-disabled-control-says-why-in-four-shapes]
---


Reported by the review of the cancel-door unit (2026-09-11, #2320),
eighteen lines above that unit's own doors and in the same
`ui.horizontal` block. Filed rather than taken there: the repair is a
choice between two answers and neither is a comment edit.

## What happens

`crates/viewer/src/app.rs:1157-1160` says the `Create` button's disabled
state is backed by a typed refusal:

> the op is emitted only by Create, and only for a non-blank name (the
> typed refusal backing the disabled button is `Refusal::EmptyName`).

The button's `on_disabled_hover_text` (`app.rs:1176`) is a literal:

> the document id is derived from the name

`Refusal::EmptyName` renders (`crates/viewer/src/session/refuse.rs:405-409`):

> a new document needs a name; its identity is derived from it

Two sentences for one condition, with a comment asserting they are one
thing. Nothing is broken — the button IS gated on a non-blank name and
`NewDocument` DOES refuse `EmptyName` — but *"the typed refusal backing
the disabled button"* describes a coupling that does not exist in the
code: nothing reads `Refusal::EmptyName` to draw that tooltip, and the
two can drift without anything noticing. One of them already has: they
do not say the same thing today.

## The two answers

- **Read the refusal.** `Refusal::EmptyName.to_string()` as the disabled
  text, which makes the comment true and deletes the second sentence.
  Costs the tooltip its shorter wording, and the refusal's sentence is
  written for a status line rather than for a button.
- **Keep the literal and delete the claim.** The tooltip is a draft-gate
  message like *"pick a dimension first"*, not a door's refusal, and the
  comment should say the button is gated on the same CONDITION the op
  refuses rather than on the same VALUE.

Which is right is the question `a-disabled-control-says-why-in-four-
shapes` asks generally: whether a control whose condition is an
operation's refusal must read it from the operation.

## Home

VIEW's: `crates/viewer/src/app.rs`.
