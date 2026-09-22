---
id: an-auto-sized-window-makes-available-width-last-frames-content
kind: issue
title: viewer: in a window with no default_width, available_width is last frame's content, so a wrap at it never fires
status: closed
opened: 2026-09-22
closed: 2026-09-22
pr: 3089
priority: P1
cost: E
refs: [messages-in-the-creation-and-properties-panes-still-draw-past-their-row]
---


`crates/viewer/src/widgets.rs`'s `message` doc enumerates TWO cases for
what `egui::Ui::available_width` names — from the cursor to the
right-hand edge in an ordinary row, and the whole row's width in a
wrapping one. There is a third, and the chrome builds it.

## The case

`egui::Resize::begin` ratchets its `desired_size` to last frame's
content size. In a window with no `default_width`, the available width
an inner `Ui` reports is therefore **last frame's content width** — so
a sentence laid out at `available_width` asks for exactly the width it
already had, and the wrap never fires. The window grows to the
sentence instead.

## Where it bites

- **Safe**: the Checks window (`crates/viewer/src/app.rs`,
  `checks_window`) sets `.default_width(420.0)`, so its content has a
  width to wrap against from the first frame. Both of this crate's
  converted `message` sites in a window are in it.
- **Not safe**: the "Add part" window (`crates/viewer/src/pane/create.rs`,
  `add_part_ui`) is `.collapsible(false).resizable(false)` with **no
  default width**. Its contents are on
  `messages-in-the-creation-and-properties-panes-still-draw-past-their-row`'s
  list for later conversion — three of them, including the
  empty-directory sentence and the store's own refusal — and nothing at
  either end warns that converting them there will not do what it does
  elsewhere.

## What a fix owes

Either a `default_width` on the "Add part" window, or a third bullet in
`message`'s doc saying what `available_width` means in an auto-sized
container and that a caller in one has to give it a width. A converted
site in that window with neither is a change that looks like the others
and behaves differently.

## Closed

Closed by PR 3089 (`chrome/message-floor`) with the doc half.
`widgets::message`'s doc now lists three cases for what
`available_width` names. The third is an auto-sized container:
`egui::Resize::begin` ratchets to last frame's content, so the wrap
never fires, and a caller in one gives the container a width, as
`checks_window`'s `default_width` does. The "Add part" window's own
`default_width` belongs to `pane/create.rs`, which was live in three
other PRs this wave. It is carried as a bullet on
`messages-in-the-creation-and-properties-panes-still-draw-past-their-row`,
where the `add_part_ui` conversion is scheduled, so the conversion and
the width land together.
