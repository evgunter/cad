---
id: a-seated-tool-previews-what-a-click-would-take
kind: issue
title: a seated tool does not preview what a click under the cursor would take
status: open
opened: 2026-11-01
priority: P3
cost: M
---


## Finding

Split from `work/chrome/after-a-duplicate-the-original-cannot-be-picked-into-a-tool.md` (option C, out of that unit's scope).

With a seated tool open (`crates/viewer/src/tools.rs`, `Tools::feed`), a user learns what a click takes only after clicking: the hover path (`PickIndex::op_under` with `PickAction::Hover`, `crates/viewer/src/pane/viewport.rs`) feeds the session's hover and nothing of the tool. On a body with a hole, or two bodies framed small, the user cannot tell before clicking whether the cursor is on material or which seat it would fill.

Sketch: while a tool is open, read the hovered selection through the same route a click takes (`Selection::seat_node` → `world::seat_read`) and show it beside the tool's picks line as a quiet preview ("click takes: Extrude …" or "click takes nothing"), with no state held beyond the frame. The line-message miss (`frame::missed_pick`) stays the answer to a click.
