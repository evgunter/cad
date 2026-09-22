---
id: error-and-check-text-overflows-its-region
kind: issue
title: viewer: error and check text runs off the page or wraps past its region, and the messages are wordy (Ev's request)
status: dispatched
opened: 2026-09-17
priority: P0
cost: E
branch: chrome/wrap-in-region
---

**Ev reported this** (in chat, 2026-09-17). There are two halves.

1. **Layout.** Error messages and check results either run off the
   page or wrap strangely. They should wrap inside the region they are
   drawn in, and not return all the way to the window's left edge.
   Sibling of `the-toolbar-row-does-not-wrap.md`. It may be the same
   egui cause: text laid out against the full available width rather
   than the width of its own container.
2. **Concision.** Error messages should be edited to be shorter. Much
   of the text the viewer shows comes from the kernel's typed refusals
   (`Display` impls across `profile`, `editor-core` and others), not
   from `viewer`. So this half reaches past VIEW's territory, and
   whoever takes it should split off per-crate rows as needed rather
   than editing kernel prose from a viewer lane.

Not investigated when filed.

## A worked example (Ev, 2026-09-17)

The Boolean refusal Ev hit unioning two dumbbell halves is about 280
words. It covers the MAY-vs-DOES box semantics, which pairs are live,
the dispatch table's wiring, and why the refusal is structural.
Nearly all of that is for kernel developers, not the person holding
the mouse. It also opens by naming undeclared coincidence as "the
common case" before saying that this case is a torus×plane pair,
which points the reader at the wrong recourse. The full text is in
this session's chat; a fixture that fails the same way (a torus face
against a plane face in a union) reproduces it.

## The layout half, measured and answered (2026-09-22)

**Both symptoms are one cause with two faces**, and neither is a
per-label flag: egui decides a label's wrap mode from the LAYOUT it is
in (`egui::Ui::wrap_mode`), and a sentence is not what either answer is
for.

- *Runs off the page.* In a non-wrapping `ui.horizontal` the mode is
  `TextWrapMode::Extend`, which lays the galley out at infinite width.
  Measured: the 90-character fixture in a 220-point region paints one
  row 538 points wide — 318 points past the right-hand edge.
- *Returns to the window's left edge.* In `ui.horizontal_wrapped` the
  mode is `Wrap`, and `egui::Label::layout_in_ui` takes its branch that
  places the whole galley at `ui.max_rect().left()` and indents only
  the first row to the cursor. Measured: rows at x = 46, 0, 0. The
  toolbar is this crate's only wrapping row, and the status line — where
  every refusal goes (`frame::apply`) — is drawn in it.

The repair is `crates/viewer/src/widgets.rs`'s `message` /
`message_link`: lay the sentence out into a galley at
`egui::Ui::available_width` and hand it over already laid out, which is
the one path `layout_in_ui` neither extends nor re-places. Converted:
the checks window's findings and the toolbar's status line
(`app.rs`), the feature tree's failure line and standing note
(`pane/features.rs`), the profile editor's preview verdicts
(`pane/profile.rs`), the view pane's status line (`pane/view.rs`).
`widgets::message_tests` holds the two measurements; both go red if
`message` degrades to `ui.label`.

Not converted, and scheduled by
`messages-in-the-creation-and-properties-panes-still-draw-past-their-row`:
eleven sites in `pane/create.rs` and `pane/properties.rs`, which were
live in other lanes' open PRs that wave.

**The concision half is untouched** and the worked example above
stands.
