---
id: a-number-fields-focus-round-trip-writes-its-rounded-text
kind: issue
title: Focusing and leaving a form's number field writes back the rounded value its text shows
status: open
opened: 2026-09-19
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
