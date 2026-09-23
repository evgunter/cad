---
id: the-toolbars-canceled-line-is-a-sentence-the-chrome-wrote-and-did-not-convert
kind: issue
title: viewer: the toolbar's 'canceled' line is a sentence in the same wrapping row as the converted status line, unconverted
status: closed
opened: 2026-09-22
closed: 2026-09-22
pr: 3089
priority: P0
cost: E
refs: [error-and-check-text-overflows-its-region]
---


`crates/viewer/src/app.rs`'s `toolbar_ui` draws
`ui.label("canceled — showing an older result")` in the SAME
`ui.horizontal_wrapped` row as the status line the layout half of
`error-and-check-text-overflows-its-region` converted to
`crate::widgets::message`.

## Why it is a site

The first version of `message`'s doc put the class at *"a sentence
somebody else wrote"* — a claim about provenance, where egui's layout
cares only about geometry. That wording has been corrected to "a whole
sentence", which is the geometric test
`messages-in-the-creation-and-properties-panes-still-draw-past-their-row`
states, and under it this literal is a sentence: it is two clauses, and
a 400-point window is measurably too narrow for the toolbar row
(`app::tests::the_toolbar_asks_for_more_width_than_a_narrow_window_gives`).

So it is in the same left-edge branch the status line was in:
`egui::Label::layout_in_ui` in a wrapping row places the whole galley
at `ui.max_rect().left()` and indents only the first line to the
cursor. Measured for the status line in that row at a 400-point
window, before the fix: first line at x = 279, second at x = 8 — 271
points of drift. Nothing about this label differs.

## What a fix owes

One of two things, and not silence:

- convert it, which is a one-line change in a file the converting PR
  already touched; or
- decide that the chrome's OWN fixed literals are out of the class on
  purpose — in which case the reason belongs in `message`'s doc, at the
  category, because a reader applying the stated test will otherwise
  keep finding this site.

The second is the weaker answer here: the literal is fixed, but the
window is not, and this row is drawn beside a spinner and a button
whose widths move with it.

## Closed

Closed by PR 3089 (`chrome/message-floor`): converted. `toolbar_ui`'s
`Progress::Canceled` arm draws `crate::widgets::message(ui,
CANCELED_LINE)`. The literal is named in `app.rs` so that the row can
find it. `app::tests::the_toolbars_canceled_line_begins_every_line_in_the_same_place`
drives the real toolbar over an inline session left `Outstanding::Canceled`
(submit, cancel, pump) across seventeen window widths from 200 to 600
points. At every width it asserts that each line of the canceled
sentence begins under the first. Reverted to `ui.label`, it goes red at
a 200-point window with 95 points of drift (first line at x = 103,
second at x = 8), which is the defect this row described.
