---
id: nothing-holds-a-new-numeric-field-to-the-fields-door
kind: issue
title: nothing holds a new numeric field to the door that gives it a text, so an eleventh DragValue arrives spelling its own precision
status: open
opened: 2026-09-12
---



(VIEW) Residue of
`a-drag-field-renders-a-length-at-a-precision-its-drag-speed-sets`,
disclosed at the moment it was created rather than left in that row's
prose.

## The finding

That unit routed all eleven `egui::DragValue::new` sites in this crate
through one constructor, `crate::widgets::number_field`, which is the
only place `crate::widgets::number_text` is attached. **Nothing holds
the twelfth site to it.** A lane that writes `egui::DragValue::new(v)`
directly gets egui's own precision rule back — the one that renders
40 nm as `0.000` and commits that number when the user clicks into the
field and away again — and nothing in the build says so.

The receipt for today's eleven is a grep
(`grep -rn 'DragValue::new' crates/viewer/src`), and the register's own
rule about that is the reason this row exists: a grep is a claim about
a moment, and the property is *a numeric field in this chrome*, which
a new helper wrapping the widget would not match either.

## Three shapes, none free

- **`clippy::disallowed_methods`** on `egui::DragValue::new`, with the
  door carrying the one `#[allow]`. Cheapest, and the lint already runs
  on every code tier. The cost is that `clippy.toml` is at the
  WORKSPACE root and no program owns it: every other crate's clippy run
  would then carry a rule about a `pub(crate)` function in `viewer`,
  and clippy does not read a per-crate config without shadowing the
  root one (which carries `ignore-interior-mutability`, and `viewer`
  depends on the naming layer that needs it).
- **A source-text guard**, which this repo already has a home for —
  but a new one owes a line in `crates/test-utils/tests/reader_census.rs`
  and must read through `test_utils::source` rather than roll its own
  reader. That is the census working as designed; it is still another
  crate's ledger for a viewer-local rule.
- **`egui::Style::number_formatter`**, set once on the context
  (`egui-0.36.1/src/style.rs:1434` is the default it would replace).
  This is the only shape that also catches an `egui::Slider`, and the
  only one a new site cannot miss. Two costs: it must go through
  `all_styles_mut` rather than `style_mut` or a theme switch drops it
  (`crate::app`'s `apply_polarity` sets a theme PREFERENCE for exactly
  the reason that snapshotting style is wrong), and the crate's widget
  tests build a bare `egui::Context`, so the rule would become
  invisible to every test that does not go through app startup — which
  is all of them today.

The unit took none of them: it took the door, whose whole argument is
that it is visible at the call site and testable without app startup.
Whoever takes this row is choosing between that visibility and a
guarantee, and should say which.
