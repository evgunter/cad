---
id: messages-wrapped-at-a-region-and-numbers-bounded-by-characters-are-two-answers
kind: issue
title: viewer: widgets.rs answers 'text too wide for its box' twice, forty lines apart, and neither answer references the other
status: closed
opened: 2026-09-22
closed: 2026-09-22
pr: 3089
priority: P1
cost: D
---


`crates/viewer/src/widgets.rs` now holds two answers to *"this text is
too wide for the box it is in"*, forty lines apart, and neither
references the other.

- **`message` bounds the WIDTH.** It lays the sentence out at
  `egui::Ui::available_width` and hands over a galley, so the text
  survives whole and the box decides how many lines it takes.
- **`number_text` bounds the CHARACTERS.** It renders through
  `crate::readout::number_text`, whose `MAX_CHARS` is a hard limit on
  the spelling, and whose doc narrates this exact defect: *"A box
  narrower than this clips, and a clipped render reads as a different
  value — that is the box's number to meet, not this one's to lower."*

They are the two arms of one fork, and the fork has never been named.

## Why this matters now rather than as taste

The unit that added `message`
(`error-and-check-text-overflows-its-region`) has TWO halves, and its
second — **concision**, editing the messages to be shorter — is
explicitly left undone. Concision is the character-bounding arm. So
this crate has already implemented, for numbers, the answer the
unfinished half of the same row is about to want for sentences, and
nothing points from one to the other.

The immediate cost is that
`messages-in-the-creation-and-properties-panes-still-draw-past-their-row`
schedules **fifty** sites for the width answer. If some of them want
the character answer instead — a refusal shortened at its source, or
clipped with the full text on hover — converting them first is fifty
sites to revisit.
`wrapping-at-a-region-with-no-floor-produces-a-four-character-ribbon`
is the measurement that says at least some of them do: at a deeply
indented narrow pane the width answer produces four characters a line.

## What a fix owes

A sentence in one of the two docs saying which answer applies when, and
a pointer from each to the other. If the answer is "a value gets the
character bound and prose gets the width bound", that is a rule with a
name and it belongs at one of them.

## Ev's ruling on a too-narrow region (in chat, 2026-09-22)

Asked what a message owes a region too narrow to hold it — keep
wrapping, wrap to a floor and then let the pane scroll, or wrap to a
floor and then clip with the full text on hover — Ev answered:

> floor then scroll i guess, but instances of this probably point to
> places where the gui should be changed to make the issue not happen

So: **wrap down to a floor, and below it stop narrowing and let the
enclosing scroll area scroll.** And a site that reaches the floor is a
finding about the layout that put a sentence there, not only a case for
the floor to absorb.

## Closed

Closed by PR 3089 (`chrome/message-floor`). The rule has a name and
one home. `crate::widgets::message`'s doc, *Characters or width: the
rule*: **a VALUE is bounded by characters, at its source** (a number
broken or clipped reads as another number: `readout::MAX_CHARS`,
`widgets::number_text`), and **a SENTENCE is bounded by its region**
(broken between words it still reads). What decides which answer a
text gets is what a line break does to it. The two meet at the floor
Ev ruled for: a region owes a sentence at least the width of the
widest value it could quote, and `widgets::message_floor` is
`MAX_CHARS` in points. Each side points at the other: the
`MAX_CHARS` doc has a paragraph pointing at `message`'s rule and at
`message_floor`, and `number_text`'s doc names itself as the first arm
and `message` as the second.

For the fifty-site census this settles that the sites take the width
answer. The concision half of `error-and-check-text-overflows-its-region`
(shortening a sentence at its source) is not the character bound
applied to prose. A sentence has no value that reads as another value
when it is clipped, so it keeps the width answer and the floor.
