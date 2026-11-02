---
id: numbered-placement-forms-draw-no-preview
kind: issue
title: the transform, pattern and sketch-on-face forms place geometry by typed numbers with no viewport preview
status: open
opened: 2026-11-01
---


## Finding

Found by the sweep in PR 6240, which draws the split tool's plane while the tool is open (`splitplane::push_split_plane`, beside the revolve tool's axis, `pane/viewport.rs`). Three other forms place geometry from numbers the person types, and draw nothing of them before commit:

- the transform tool's translation and rotation (`ViewerBehavior::transform_tool_ui`, `pane/create.rs`);
- the pattern tool's linear step and circular axis (`ViewerBehavior::pattern_tool_ui`);
- the sketch-on-face form's distance and spin (`sketch_face_rows`).

Each is typed in the coordinates of a space the panel does not draw, which is the split's defect 1 (`work/chrome/the-split-tools-plane-is-typed-blind.md`). A preview for each can follow the split's shape: drawn under the copy the pick was made through (`Seats::through`, `PickIndex::drawn_map`), in the marks lane.
