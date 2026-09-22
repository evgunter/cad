---
id: error-and-check-text-overflows-its-region
kind: issue
title: viewer: the CONCISION half — error messages should be shorter, and most of the text is the kernel's typed refusals (Ev's request; the layout half landed in 3058)
status: open
opened: 2026-09-17
priority: P0
cost: E
refs: [3058]
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
per-label flag. `egui::Ui::wrap_mode` answers in three steps — the
`Ui`'s own `egui::Style::wrap_mode` if something set one, else `Extend`
inside a grid, else the layout's — and nothing in this chrome sets a
style wrap mode, so what decides a label's wrap today is the layout it
happens to be in. A sentence is not what either of the layout's answers
is for.

- *Runs off the page.* In a non-wrapping `ui.horizontal` the mode is
  `TextWrapMode::Extend`, which lays the galley out at infinite width.
  Measured: the 89-character fixture in a 220-point region paints one
  row 491.7 points wide, its right edge at x = 538.2 — 318 points past
  the region's own right-hand edge.
- *Returns to the window's left edge.* In `ui.horizontal_wrapped` the
  mode is `Wrap`, and `egui::Label::layout_in_ui` takes its branch that
  places the whole galley at `ui.max_rect().left()` and indents only
  the first row to the cursor. Measured in the REAL toolbar, at a
  400-point window: the first line starts at x = 279 and the second at
  x = 8, the window's own left edge — 271 points of drift. The toolbar
  is this crate's only wrapping row, and the status line — where every
  refusal goes (`frame::apply`) — is drawn in it.

The repair is `crates/viewer/src/widgets.rs`'s `message`,
`message_link` and `message_toned`: lay the sentence out into a galley
at `egui::Ui::available_width` and hand it over already laid out, which
is the one path `layout_in_ui` neither extends nor re-places. The wrap
is asked for explicitly, so a future context-wide `Style::wrap_mode`
would move every other label in the chrome and leave a message where it
is. `message_toned` routes the voice through `app::toned`, so what
`Advisory` looks like stays decided in one place rather than
hand-spelled at each call site.

Converted: the checks window's findings and its skipped-checks line and
the toolbar's status line (`app.rs`), the feature tree's failure line
and standing note (`pane/features.rs`), the profile editor's preview
verdicts (`pane/profile.rs`), the view pane's status line
(`pane/view.rs`). The roster is re-derived from the crate's own source
by `widgets::roster_tests::the_message_roster_is_what_the_crate_actually_calls`.

`widgets::message_tests` holds the measurements. The load-bearing one
is `a_message_fills_the_region_it_is_given_rather_than_a_fixed_width`:
the other rows are all satisfied by a sentence laid out NARROWER than
its region, so they cannot tell the fix from a hardcoded constant — this
one measures the widest line across three region widths and requires it
to grow. `app`'s
`the_toolbars_status_line_wraps_under_itself_rather_than_at_the_windows_edge`
measures the second symptom in the real toolbar, which is where it was
reported. Proven red both ways: with `wrapped_in_region` hardcoded to
150.0 (the fill row goes red at 40.4 points past a 150-point region;
the toolbar row goes red because an unconstrained window then also
wraps) and with `message` degraded to `ui.label` (all four widget rows
and the toolbar row go red).

Not converted, and scheduled by
`messages-in-the-creation-and-properties-panes-still-draw-past-their-row`:
fifty sites in `pane/create.rs` and `pane/properties.rs`, which were
live in other lanes' open PRs that wave. That row's census was re-taken
here — its first count of eleven was both internally inconsistent and a
large undercount.

**The concision half is untouched** and the worked example above
stands.

## Ev's ruling on the concision half (in chat, 2026-09-22)

Asked whether the viewer should summarise (a one-line summary with the
full text behind a toggle), the kernel prose should be rewritten, or
both, Ev answered:

> probably rewrite kernel prose. if it is IMPOSSIBLE to include all
> IMPORTANT information within a reasonable amout of space in the gui,
> then you could consider adding a summary, but i'd really like to
> avoid that added complexity

So the concision half is fixed **at the source**: the refusals the
viewer shows are rewritten in the crates that raise them, keeping what
the person holding the mouse needs and dropping what is written for
kernel developers. A viewer-side summary is the fallback for a refusal
whose important content genuinely cannot fit, and needs that case
shown, not asserted.
