---
id: sentences-in-the-view-and-profile-panes-wrap-with-no-floor
kind: issue
title: viewer: four sentences in the View and Profile panes are drawn by the layout's wrap, which has no floor and breaks inside a word
status: open
opened: 2026-09-22
priority: P2
cost: E
refs: [messages-in-the-creation-and-properties-panes-still-draw-past-their-row]
---


Found by the sweep of `chrome/message-floor`, over the two files
`messages-in-the-creation-and-properties-panes-still-draw-past-their-row`
says it did not sweep (`pane/view.rs`; `pane/viewport.rs` has no text
call) plus `pane/profile.rs`'s unconverted remainder.

## The sites

Each is a sentence under that row's census test (prose a narrow pane
cannot be assumed to hold, or text with an unbounded part), drawn by a
plain label in a top-down layout:

- `pane/view.rs`, `view_ui`: `format!("file: {}", path.display())` — a
  filesystem path, which nothing bounds.
- `pane/view.rs`, `view_ui`: *"chosen for the triangle budget; δ is
  yours from here"* — two clauses, a fixed literal.
- `pane/profile.rs`, `edit_profile_ui`'s refusal arm:
  `format!("the profile editor cannot hold this profile: {refusal}")`
  through `ui.colored_label` — a refusal's own `Display`.
- `pane/profile.rs`, `edit_profile_ui`: `ui.weak(SHAPE_LOCKED)`
  (`crate::forms::SHAPE_LOCKED`) — a fixed literal; include it or argue
  it out by its length.

## Why it matters now

A top-down layout wraps a plain label at the region, which is why these
read correctly at ordinary widths. What it does not do is what
`crate::widgets::message` now does: it has no floor
(`crate::widgets::message_floor`), so in a narrow pane it lays these out
a few characters a line, and it breaks inside a word once no space fits
— a path or a number included. The same pane's status line, one
`ui.separator()` below the file line in `view_ui`, goes through
`message` and does neither. Two sentences in one column now wrap by two
rules.

## What a fix owes

Route each through `crate::widgets::message` / `message_toned`, or say
at the site why it is a name. The roster row
(`widgets::roster_tests::the_message_roster_is_what_the_crate_actually_calls`)
already lists both files, so it does not move.
