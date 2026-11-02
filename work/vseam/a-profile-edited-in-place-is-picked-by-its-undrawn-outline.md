---
id: a-profile-edited-in-place-is-picked-by-its-undrawn-outline
kind: issue
title: a profile the edit door previews in place is still picked by its committed outline, which is not drawn
status: open
opened: 2026-11-02
priority: P3
cost: E
---


Found by the lane that made a profile pickable in the viewer
(`a-profile-cannot-be-picked-in-the-viewer`).

The pick index holds every committed profile's outline
(`crates/viewer/src/pickindex.rs`, `Outline`, built in
`PickIndex::assemble` from `sketch::committed(doc, eval, δ, None)`).
The viewport's profile lane leaves out the node the edit door is
previewing in place (`pane::viewport`, `committed_lane`, passing
`profile_edited` as `sketch::committed`'s `except`). The index is built
on the worker from the landing alone and does not know that node, so
while the edit door is open a click where the profile USED to be drawn
still hovers and selects it, though nothing is drawn there.

Small: the selected node is the one being edited, and the preview
usually sits near the old outline. The fix is to hand the exclusion to
the pick (`PickIndex::op_under` / `hovered_for`, or `DisplayView`) so
the pick skips exactly what the lane skips, rather than rebuilding the
index per edit.
