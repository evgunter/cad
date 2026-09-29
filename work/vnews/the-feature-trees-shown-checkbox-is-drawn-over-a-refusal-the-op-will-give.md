---
id: the-feature-trees-shown-checkbox-is-drawn-over-a-refusal-the-op-will-give
kind: issue
title: The feature tree's 'shown' checkbox is gated on the row's kind, not on display_check, the admission SetInstanceHidden runs
status: open
opened: 2026-09-28
priority: P3
cost: E
refs: [the-mirror-class-is-unswept-outside-the-properties-pane, the-hide-toggle-is-drawn-over-a-refusal-the-op-will-give]
---


Found by the census in `the-mirror-class-is-unswept-outside-the-properties-pane`,
at merge base `f4e9aa68b`. It is the sibling of the closed
`the-hide-toggle-is-drawn-over-a-refusal-the-op-will-give`, missed
because that sweep read `pane/properties.rs` only.

## What happens

`ViewerBehavior::feature_row` (`crates/viewer/src/pane/features.rs`
~:154) draws `ui.checkbox(&mut shown, "shown")` on every row whose
`row.kind == "InstantiatePart"` and pushes
`SessionOp::SetInstanceHidden`. The door, `DisplayState::set_hidden`
(`crates/viewer/src/display.rs`), runs `display_check` against
`history.doc()` and refuses:

- `Refusal::Display` carrying `AdmissionFault::FusedGeometry` for an instance
  whose geometry shares a drawn root with another's (a cross-instance
  boolean). The properties pane's `hide_toggle` is drawn disabled here,
  with the fault's sentence; the tree's checkbox is live.
- `AdmissionFault::NoSuchNode`, and `FusedGeometry` for a boolean
  committed since the last landing. `DocSession::tree_rows` reads the
  LANDED document, while the door reads the committed one, so until
  the next landing (indefinitely after a cancelled evaluation) a row
  for a deleted instance draws a live checkbox.

## Fix

The one the properties pane already has: compute
`display::display_check(session.committed_doc(), row.id)` for the row,
draw the checkbox with `add_enabled(addressable.is_ok(), ..)`, and
carry the fault's own words. `hide_toggle` in `pane/properties.rs` is
private, and the natural move is to lift it so both panes draw the
same control, rather than spell the gate twice.
