---
id: the-transform-tool-and-the-placement-editor-are-one-editor-under-two-names
kind: issue
title: viewer: the transform tool and editing a placement are the same editor under two tool names, and nothing says so
status: open
opened: 2026-11-22
priority: P1
cost: M
---

**Ev reported this** (in chat, 2026-11-22): "unclear what the relation between the 'transform tool' and editing a placement is".

They are the same editor. `ToolKind::Transform` (`crates/viewer/src/tools.rs:67-69`, labelled "transform" at `:107`) hands a pick to `MateTool::moving` / `moving_picked` (`tools.rs:948-960`). That replaces the open tool with `OpenTool::Mate`, the placement editor (`crates/viewer/src/matetool.rs:1829-1846`, `door_on(..., Purpose::Moving)` at `:1348`, `:1375`). `TransformTool` itself (`crates/viewer/src/combine.rs:1115-1125`) holds only the which-half-of-a-split wait. The mate tool opens the same editor (`crates/viewer/src/pane/create.rs:642-645`, "pick a body to place it, or a copy to edit its placement"), as do a tree row and a constraint row (`matetool.rs:1-15`). The transform panel says only "pick the body or the copy to move" (`create.rs:1474-1490`).

So there are three or four doors into one editor, under two tool names, and the person is never told they lead to the same place. The fork is whether to keep two tools (a transform door that opens the placement editor and says so in its panel) or to fold them into one named tool. That is why `design: true` is set. Related: `work/doors/a-pick-opening-the-edit-door-starts-a-mate-row.md` (open).
