---
id: the-feature-trees-shown-checkbox-is-drawn-over-a-refusal-the-op-will-give
kind: issue
title: The feature tree's 'shown' checkbox is gated on the row's kind, not on display_check, the admission SetInstanceHidden runs
status: closed
opened: 2026-09-28
priority: P3
cost: E
refs: [the-mirror-class-is-unswept-outside-the-properties-pane, the-hide-toggle-is-drawn-over-a-refusal-the-op-will-give, the-hide-admission-is-read-through-two-spellings]
closed: 2026-09-28
branch: vnews/the-shown-checkbox-reads-its-refusal
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

## Closed

`DocSession::instance_hidden_refusal` (`crates/viewer/src/session.rs`,
beside `slot_unit_refusal`) answers what `SetInstanceHidden` would
refuse an instance with: `display_check` against the committed
document, as the op's `Refusal::Display`. That is the op's whole
admission, since `SetInstanceHidden` is permitted in both drag tables
`perform` applies first. `ViewerBehavior::feature_row`
(`pane/features.rs`) still draws the checkbox on `InstantiatePart`
rows only, and now draws it with `add_enabled` on that answer, with
the refusal's sentence on its disabled hover. The fused case and the
deleted-since-landing case are both covered.

The lift of `hide_toggle` was not done: `pane/properties.rs` was held
by another lane. The two controls read one function on one document,
so they agree today. The second spelling is filed as
`the-hide-admission-is-read-through-two-spellings`.

Tests: `app.rs` `properties_pane_tests` drives the real app over a
fused pair (each row's checkbox is disabled, its hover is the op's
sentence for that instance, and a click hides nothing and sets no
status) and over an unfused pair (live, silent on hover, and a click
hides that row's instance). `session.rs` `tests` pins the
deleted-since-landing case: the tree still draws the row, and the
answer is the refusal the op gives.
