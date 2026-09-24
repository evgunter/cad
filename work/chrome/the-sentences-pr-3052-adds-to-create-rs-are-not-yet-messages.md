---
id: the-sentences-pr-3052-adds-to-create-rs-are-not-yet-messages
kind: issue
title: viewer: the sentences AUTHOR's #3052 adds to pane/create.rs are drawn by the layout's rule, not widgets::message
status: open
opened: 2026-09-24
priority: P2
cost: E
refs: [messages-in-the-creation-and-properties-panes-still-draw-past-their-row]
---


`chrome/create-messages` (PR 3139) routed every sentence in
`crates/viewer/src/pane/create.rs` through `crate::widgets::message` or
`message_toned`. AUTHOR's PR 3052 (`author/part-and-duplicate`, head
`6507b87fd` when this was filed) was open at the time, and it adds
sentences of its own to that file. Those were not on PR 3139's branch,
so they were not converted:

- `part_selector_rows`: `"the tool plane's normal side is above"`,
  `"instances are numbered from zero, in placement order"` and
  `PROJECTION_HIDES_THE_REST`. All three are `ui.weak`, top-down, after
  the rows they are about.
- `projection_tool_ui`: the `ToolKind::Part.says(…)` prompt (`ui.label`)
  and its `seat_line(…)` (`ui.weak`).
- `duplicate_tool_ui`: the `ToolKind::Duplicate.says(…)` prompt, its
  `seat_line(…)` and `duplicate_note()`.

Each one is a SENTENCE by the NAME-vs-SENTENCE test on
`messages-in-the-creation-and-properties-panes-still-draw-past-their-row`.
None is in a row, so none owes a layout change. What each owes is the
spelling the rest of the file now uses:
`crate::widgets::message(ui, …)` for a `ui.label`, and
`crate::widgets::message_toned(ui, …, &self.theme, Tone::Advisory)` for
a `ui.weak`. `part_selector_rows` is a free function and takes no theme
today, so it needs a `&Theme` parameter, as `frame_picker` gained one.

Whichever lands second does the conversion: PR 3052 itself if it
rebases onto PR 3139, or a CHROME pass once 3052 is on `main`.
