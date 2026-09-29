---
id: the-toolbars-controls-run-past-the-panel-below-a-phone-wide-window
kind: issue
title: viewer: below a 400-point window the toolbar's theme picker, and below the message floor its canceled line, are drawn past the panel
status: open
opened: 2026-09-29
priority: P3
cost: M
design: true
refs: [the-toolbars-status-line-runs-past-the-panel-below-a-floor-wide-window]
---


Found while moving the status line into a row of its own
(`chrome/status-line`), by sweeping every text run the toolbar paints
across 120-to-640-point windows, headless, through `app::tests`'
`toolbar_drawn`. The status line is no longer in the controls' row; what
is left there has two overruns.

## The theme picker

`crate::app`'s `toolbar_ui` draws the palette `egui::ComboBox` in the
`horizontal_wrapped` row. egui's `combo_box_dyn` allocates the box
through `button_frame` with no check against what is left of the line,
so, unlike a button, it does not move to the next line when it does not
fit. Measured (the startup fixture, which is evaluating):

- 180 to 220-point windows: the picker's text ends at x = 232 and the
  row at 249, against a panel that ends at 172 to 212.
- 360-point window: the row ends at 370.6, against a panel ending
  at 352.
- 400 (`app::tests`' `NARROW`) and wider: inside.

It also widens the panel's `Ui`: its `max_rect` grows to take in the
overrun, so anything laid out after the row sees the wider width.
`toolbar_ui` now reads the panel's width before the row, so the
status line's row is not affected. Nothing else placed after the row
reads that width.

## The canceled line

`CANCELED_LINE` is a `crate::widgets::message` in the same row. Below
a window of about 190 points (the floor, 172.6, plus the panel's
margins) it is laid out at the floor, and the panel has nothing to
scroll. Measured: 8 points past the panel at 120 to 180-point windows,
inside from 200. The status line's answer (a row of its own that
scrolls) is not obviously right here, because the Re-evaluate button
belongs beside the sentence.

## Not measured

The Checks window (`crate::app`'s `checks_window`, `default_width(420)`)
and the part chooser (`crate::pane::create`'s `part_window`) are
`egui::Window`s. egui caps a window at the screen (`egui::Window`'s
constrain step), and neither window scrolls, so below a floor-wide
screen a message in either is clipped rather than scrolled. The
same narrow regime, not driven.

Whether windows this narrow are in scope is the question to answer
first: `NARROW` holds the chrome at 400 points, and every overrun here
is below it.
