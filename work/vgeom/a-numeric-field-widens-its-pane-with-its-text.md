---
id: a-numeric-field-widens-its-pane-with-its-text
kind: issue
title: A numeric field is as wide as its exact text, so three free-move boxes can widen the pane
status: open
opened: 2026-11-25
priority: P1
cost: M
---

Found by the review of PR 7775 (`vgeom/field-seed`).

Since PR 7775, an editable field's text is its value's exact spelling
(`crates/viewer/src/props.rs`, `FieldNumber::text`). A value no short
decimal names shows all its figures: δ = default/30 seeds
`0.0016666666666666669`, and a free-move box over a computed coordinate
shows up to seventeen figures and its zeros.

`egui::DragValue` lays its text out with `TextWrapMode::Extend`
(egui 0.36.1, `widgets/drag_value.rs`, the `Button` it builds when not
editing), so the field is as wide as its text. This causes two defects
in ordinary use:

- The free-move probe's three boxes
  (`crates/viewer/src/pane/properties.rs`, `free_move_probe`, through
  `widgets::vec3_row_ops`) are about 19 characters each, and as a row
  they can be wider than the properties pane. That pushes the pane out.
- The field's width changes mid-drag, as the dragged value moves
  between short and long spellings.

The δ field is a `TextEdit` and is fixed-width since PR 7775: its text
scrolls inside it (`pane/view.rs`, `delta_text_edit`). The `DragValue`
fields have no such cheap option, because the wrap mode is hard-coded
in egui. Candidate fixes:

- Draw the button state through a child `Ui` with a fixed rect and a
  clip.
- Use a fixed-width `TextEdit` beside a drag handle.
- Keep a fixed width and show a told text while not editing. This would
  re-open the ratified rule, since the field's text would no longer be
  its value's exact spelling.
