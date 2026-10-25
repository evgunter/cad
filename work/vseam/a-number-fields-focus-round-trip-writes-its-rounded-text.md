---
id: a-number-fields-focus-round-trip-writes-its-rounded-text
kind: issue
title: Focusing and leaving a form's number field writes back the rounded value its text shows
status: open
opened: 2026-09-19
priority: P4
cost: E
---


## What happens

The path editor's number fields (and every form field built on
`widgets::number_field`: `named_field`, `unit_field`, `point_fields`,
`named_scalar`) are `egui::DragValue`s with a `custom_formatter`
(`widgets::number_text`) that shows a fixed number of decimals. When a
field takes keyboard focus, egui fills its edit buffer with the
FORMATTED text; when focus leaves, the buffer is parsed and, if it
parses, written back — so a value with more digits than the field shows
(0.00123 m shown as `0.0012`) comes back rounded, although nobody typed
anything. `named_field` writes back on `response.changed()`, which a
focus round trip reports.

Found by the review of PR #2862 (the profile editor), where it now
reaches COMMITTED profiles: tabbing through the edit door's fields
turns a program's untouched numbers into rounded ones, marks the draft
moved, and Apply would write them. It predates that PR — the create
form's fields and the Properties pane's slot fields share the widget —
so it was filed rather than fixed there.

## What would close it

A field over a value must write back only a value the person changed:
compare the parsed buffer with the formatted text of the held value
(not with the value), or keep full precision in the edit buffer. A row
driving focus in and out of an untouched field through
`egui::Context::run` and asserting the held value's bits is the test.

## 2026-10-21 — mostly closed by the readout rule (P0 → P4)

The case above no longer happens. `widgets::number_text`
(`crates/viewer/src/widgets.rs:307`) now falls back to
`readout::number` whenever the fixed-decimal spelling does not
`readout::reads_back` to the value, so a field over 0.00123 m shows a
spelling that reads back to it, and a focus round trip writes that.

What remains: `reads_back` accepts a text within
`readout::tolerance` (the finer of 0.1·ε and 5·10⁻⁴ of the value), so a
round trip can still change a value's bits by at most that much, and a
draft compared by value may still read as moved. That is far below the
modeling tolerance, so the shape is invisible. The test this row asks
for (focus in and out of an untouched field, assert the bits) is still
the way to settle whether anything is left to fix or the row closes.
