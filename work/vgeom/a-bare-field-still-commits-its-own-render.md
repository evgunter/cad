---
id: a-bare-field-still-commits-its-own-render
kind: issue
title: A bare egui::DragValue takes the render from the context and the echo guard from nothing, so it still commits its own text
status: closed
opened: 2026-09-21
priority: P2
cost: D
branch: vgeom/field-product
pr: 3067
closed: 2026-09-22
---

Filed by `vgeom/p0-fields` while generalising the echo guard of
`a-fields-text-commits-within-the-renders-own-tolerance` from the two
panel value fields to every field the chrome builds through
`crate::widgets::number_field`.

## The finding

`widgets::install_number_formatter` is the FLOOR under
`widgets::number_field`: it writes `crate::widgets::number_text` onto
`egui::Style::number_formatter`, so a bare `egui::DragValue::new` —
the twelfth site, a helper that wraps the widget, an `egui::Slider` —
renders by the crate's rule without knowing the door exists. Its own
doc argues at length for why that is a DEFAULT rather than a
detection.

**The commit half cannot travel the same way.** *Text the field itself
produced is not an edit* is enforced in a `custom_parser`, and
`egui::Style` has a `number_formatter` and no counterpart for parsing
(`egui-0.36.1/src/style.rs`; `egui-0.36.1/src/widgets/drag_value.rs`'s
`parse` falls to a private `default_parser` when no custom one is
set). So a bare field shows the right text and still writes that text
back when a click leaves it — and the render names its value only to
`crate::readout::REL_TOLERANCE`, so the value MOVES.

Pinned, not argued:
`widgets::field_tests::a_bare_field_commits_a_render_the_door_would_refuse`
asserts a bare field holding `1000.001` commits `1000.0` and that the
door's field holding the same value commits nothing.

## Reachability: none in the tree today, by construction of the census

Every numeric field in `crates/viewer/src` is built through
`number_field` — the census is the `number_field(` call sites, and the
one bare `egui::DragValue::new` in the crate is the test harness's own
`Built::Bare`. So this is a door hardening against the arrival the
floor exists for, exactly as `install_number_formatter` is.

## The fork

- **Nothing**, and say so at `install_number_formatter`: the floor
  carries the render, the door carries the commit, and a site that
  wants both uses the door. That is what this branch did, as a
  statement rather than a repair.
- **A gate**, the shape `scripts/gates/viewer-module-kinds.sh` already
  has: refuse a bare `egui::DragValue::new` under `crates/viewer/src`
  outside the door and its own harness. Makes the census above a
  standing fact rather than a measurement taken once.
- **Upstream**, an `egui::Style::number_parser` beside the formatter.
  Out of this repo.

