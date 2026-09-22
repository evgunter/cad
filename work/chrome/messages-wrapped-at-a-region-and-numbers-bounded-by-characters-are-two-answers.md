---
id: messages-wrapped-at-a-region-and-numbers-bounded-by-characters-are-two-answers
kind: issue
title: viewer: widgets.rs answers 'text too wide for its box' twice, forty lines apart, and neither answer references the other
status: open
opened: 2026-09-22
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
