---
id: a-driven-slots-field-draws-its-expression-source-at-any-width
kind: issue
title: viewer: a driven slot's value field shows the expression's source as its text, which nothing bounds, inside a non-wrapping row
status: open
opened: 2026-09-23
priority: P2
cost: D
refs: [messages-in-the-creation-and-properties-panes-still-draw-past-their-row]
---


Found by the second sweep pass of `chrome/properties-messages`, the
one aimed at the blind spot
`messages-in-the-creation-and-properties-panes-still-draw-past-their-row`
names: a `*_ui` body called from inside a caller's `ui.horizontal`.

## The site

`crates/viewer/src/pane/properties.rs`, `slot_group_ui`, draws each
slot's field through `slot_value_ui` inside a `ui.horizontal` — one
field for a scalar, three for a vector. `slot_value_ui` hands
`crate::widgets::value_field_ops` a fixed text for a driven slot or a
slot that did not evaluate: `crate::props::field_text(row)`, the
slot's own expression SOURCE. `value_field_ops` renders it through the
`egui::DragValue`'s `custom_formatter`, and a `DragValue` draws its
text as a button in `TextWrapMode::Extend` (`crate::widgets`'s
`number_text` doc says so for the 311-character case).

So a slot driven by `outer_enclosure_wall_thickness * 2 +
gasket_compression_allowance` draws a field as wide as that source,
and a vector with three such components draws three, in a row that
does not wrap. The row runs past the pane's right-hand edge.

## Why it is not a message

The field's text is a VALUE the field holds and commits
(`number_text`'s *a field's text is a commit path*), not a sentence,
so wrapping it at the region is not the answer `crate::widgets::message`
gives. `crate::readout::MAX_CHARS` bounds a NUMBER's text by its
characters; nothing bounds an expression's, and a parameter name is
authored by the user.

## What a fix has to decide

Whether a field showing source is given a width (and clips, with the
source on hover), or whether a driven slot's source moves out of the
row — the ruling's second half (Ev, 2026-09-22, carried on
`wrapping-at-a-region-with-no-floor-produces-a-four-character-ribbon`):
a sentence-length text in a row is a finding about the layout. The
affordance under the row (`slot_notes`) already quotes the parameters
the expression reads.
