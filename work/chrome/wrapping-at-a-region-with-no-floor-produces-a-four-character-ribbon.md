---
id: wrapping-at-a-region-with-no-floor-produces-a-four-character-ribbon
kind: issue
title: viewer: a message wrapped at a deeply indented narrow pane lays out four characters to a line, and MAX_CHARS's ratified rule says a box owes a number to meet
status: closed
opened: 2026-09-22
closed: 2026-09-22
pr: 3089
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

## Closed

Closed by PR 3089 (`chrome/message-floor`), per Ev's ruling above.
`widgets::wrapped_in_region` lays a message out at
`max(available_width, message_floor)`. Below the floor it stops
narrowing, and the pane's `ScrollArea::both()` scrolls.

- **The floor** is `widgets::widest_number` plus one space advance, in
  the font `FontSelection::Default` resolves to. At egui's default body
  font that is 172.6 points. `widest_number` is
  `readout::widest_render`: `MAX_CHARS + 1` characters at the widest of
  `readout::GLYPHS`. The extra character is the scientific arm's sign,
  which `MAX_CHARS` leaves aside. It is the widest spelling in a
  monospace font (`-1.7976931348623157e308`); in the body font the
  22-digit `1000000000000000000000` is. On top of that goes one pixel
  for epaint snapping glyph positions to the pixel grid (measured 0.09
  points over at a 1.01 scale, in monospace). The δ field in
  `pane/view.rs` (`delta_text_edit`) reads the same width in place of
  its old hand-kept 176-point constant.
- **The space** is what keeps a number whole. epaint tests a glyph for
  overflow before it records the glyph as a break candidate, so when a
  number starts a line, the overflow fires on the space after it. With
  no space candidate on that line yet, the break falls back to a `-`, a
  `.`, or any character inside the number. The claim is exactly this: a
  line never breaks inside a number `readout` renders, when a space or
  the end of the text follows it. A longer word, such as a path or a
  number glued to punctuation, is broken by epaint's own rule. A first
  version of this fix also laid out at the text's widest word. That was
  dropped in review: epaint breaks such a word at its `.` or `-`
  anyway, for the reason above, and one long token then set the width
  of every line in the paragraph.

Rows: `below_the_floor_a_message_stops_narrowing`,
`the_widest_number_holds_every_number_readout_renders` (both fonts, six
scales) and `a_line_never_breaks_inside_a_number_readout_renders` (both
fonts).

The ruling's second half was taken at the measured site as well.
`pane/features.rs`'s lines under a row now indent at `message_indent`:
the row's indent plus one step, for as long as that leaves the floor.
Past that, the indent gives way first
(`a_deep_rows_line_gives_up_its_indent_before_its_width`,
`a_line_under_a_row_in_a_wide_pane_keeps_its_whole_indent`). The
give-way does not move as the scroll area's vertical bar comes and goes.
egui 0.36.1's default `ScrollStyle` is `floating()`, whose
`allocated_width` is `floating_allocated_width: 0.0`, and the chrome
sets no scroll style. So a bar appearing takes no width from the
content, and `available_width` is the same with or without one.
