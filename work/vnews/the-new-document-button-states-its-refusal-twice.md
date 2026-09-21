---
id: the-new-document-button-states-its-refusal-twice
kind: issue
title: The New document button's disabled reason is a literal beside a comment claiming Refusal::EmptyName backs it, and the two sentences differ
status: review
opened: 2026-09-11
refs: [a-disabled-control-says-why-in-four-shapes]
priority: P3
cost: E
---


Reported by the review of the cancel-door unit (2026-09-11, #2320),
eighteen lines above that unit's own doors and in the same
`ui.horizontal` block. Filed rather than taken there: the repair is a
choice between two answers and neither is a comment edit.

## What happens

*(The three line numbers in this section are the filing's, from
2026-09-11, and no longer resolve. They are left as written and
re-derived under "Decided" below, per the register's re-derive-never-
shift rule.)*

`crates/viewer/src/app.rs:1156-1159` says the `Create` button's disabled
state is backed by a typed refusal:

> the op is emitted only by Create, and only for a non-blank name (the
> typed refusal backing the disabled button is `Refusal::EmptyName`).

The button's `on_disabled_hover_text` (`app.rs:1175`) is a literal:

> the document id is derived from the name

`Refusal::EmptyName` renders (`crates/viewer/src/session/refuse.rs:426-430`):

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

## Decided: answer 1, read the refusal (2026-09-19)

The census this row waits on —
`a-disabled-control-says-why-in-four-shapes`, run at merge base
`2654cc111417da806d9786c40136106469096fec` — lands on the rule *a
control a reader cannot use owes the sentence a click would have been
answered with — when there is such a sentence*, and this button is a
genuine hit of it. The condition IS `NewDocument`'s refusal condition,
so the pre-click sentence and the refusal are one sentence and get one
composition.

**Answer 2 is refused.** *"The tooltip is a draft-gate message like
'pick a dimension first'"* is false here and the census says why: a
draft gate is a control no operation can be formed behind. This one can
— `SessionOp::NewDocument { name }` is formed from the same string — and
`Refusal::EmptyName` is what the door answers. The comment is not
describing a coupling that should be deleted; it is describing the
coupling the code owes and does not have.

**The stated cost of answer 1 is already paid for in `refuse.rs`.**
"The refusal's sentence is written for a status line rather than for a
button" is the objection `Refusal::exists_wording` exists to answer:
*"the add-parameter form shows the same sentence BEFORE the click — one
composition, so the pre-click notice and the refusal cannot drift
apart."* If the button needs shorter words, the words move in
`refuse.rs` and both surfaces move with them.

**Re-derived citations.** The comment is `crates/viewer/src/app.rs:
1422-1427`, and the sentence the filing quotes is its last four lines
(was `:1156-1159`); the button and its
`on_disabled_hover_text` are `:1442-1443` (was `:1175`);
`Refusal::EmptyName`'s `Display` is `crates/viewer/src/session/
refuse.rs:488-493` (was `:426-430`).

**One thing to know before writing the fix.** The button is the only
producer of `SessionOp::NewDocument` (`app.rs:1446`), and it is
disabled on exactly the blank-name case — so `Refusal::EmptyName` is
plausibly unreachable through the chrome. That does not weaken the
answer; it means the button is the *only* reader that condition will
ever have, which is the strongest reason for its words to be the
refusal's.

## Done (2026-09-20) — answer 1, as ruled

Taken with `undo-and-redo-are-disabled-in-silence-over-a-refusal-that-
has-words` as one unit. Branch `vnews/app-controls-read-their-refusals`.

The census's ruling was applied unchanged and the tree had not moved
under it: the Create button was gated on `!name.trim().is_empty()` and
`DocSession::new_document` refused `Refusal::EmptyName` on
`name.trim().is_empty()` — one condition, two spellings, two
sentences.

**What landed, and the one place it goes past the ruling.** The button
shows `Refusal::EmptyName`'s own words, so the comment's claim that the
disabled state is *backed by* a typed refusal is now true. It also
reads the CONDITION from the same place: `Refusal::empty_name(name)`
is `Some` exactly when the name carries no non-whitespace text, and
both the door and the button call it. The trim was the latent half of
the drift the row names — a button that stopped trimming would offer a
click the door refuses — and it costs one function to put it beside the
words rather than leave it copied.

The comment was rewritten to state the invariant rather than the
coupling it wished for. The literal *"the document id is derived from
the name"* is gone from the button; the fact it carried survives in the
comment above the form, where it is the reason the field exists, and in
`Refusal::EmptyName`'s own sentence, which says it.

Asserted by `crates/viewer/tests/chrome_labels.rs`'s
`a_disabled_toolbar_control_says_what_its_own_operation_refuses`, whose
Create half goes red if the trim leaves `empty_name`.
