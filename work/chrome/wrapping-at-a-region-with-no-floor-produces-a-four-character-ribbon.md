---
id: wrapping-at-a-region-with-no-floor-produces-a-four-character-ribbon
kind: issue
title: viewer: a message wrapped at a deeply indented narrow pane lays out four characters to a line, and MAX_CHARS's ratified rule says a box owes a number to meet
status: open
opened: 2026-09-22
priority: P1
cost: D
refs: [messages-wrapped-at-a-region-and-numbers-bounded-by-characters-are-two-answers]
---


`crates/viewer/src/widgets.rs`'s `wrapped_in_region` lays a sentence
out at `egui::Ui::available_width` and takes whatever that is. It has
no floor, and in one place the chrome already builds, what it is is
almost nothing.

## Measured

`pane/features.rs` indents a row by `indent(row.depth) + INDENT_STEP`,
which at `INDENT_MAX_DEPTH` is `8 * 12 + 12 = 108` points. In a
150-point pane that leaves 42. Driven headlessly through
`pane::headless::landed`, at that indent and that width:

- an 89-character message lays out as **15 rows, widest 39.97 points**
  — four to six characters a line;
- a 62-character message carrying a rendered number
  (`the offset is 0.30000000000000004 mm, which the solver refused`)
  lays out as **14 rows, widest 37.97 points**, and at that width the
  break falls INSIDE the number: epaint's word-boundary search falls
  back to `.any` when no boundary fits the line.

## Why it is filed rather than accepted

**It crosses a ratified rule.** `crate::readout::MAX_CHARS`'s doc
already settles what a box owes a text it cannot hold:

> A box narrower than this clips, and a clipped render reads as a
> different value — that is the box's number to meet, not this one's to
> lower.

`wrapped_in_region` is a box with no number to meet. It answers a
too-narrow region by degrading the text instead, which is the
direction that doc rules out.

**And the fix made the symptom quieter, not smaller.** Before this
change the sentence ran off the edge inside the
`egui::ScrollArea::both()` that `app::ViewerBehavior::pane` wraps every
chrome pane in, so a reader could scroll to it. Now it fits, so no
horizontal scrollbar is offered — and the ribbon is what there is.

## What a fix has to decide

A floor is one answer: below some width, stop wrapping and let the
scroll area do its job, or clip with an ellipsis and put the text on
hover. Which one is the same question
`messages-wrapped-at-a-region-and-numbers-bounded-by-characters-are-two-answers`
asks, and this row should be taken with it.

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
