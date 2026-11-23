---
id: a-profile-edited-in-place-is-picked-by-its-undrawn-outline
kind: issue
title: a profile the edit door previews in place is still picked by its committed outline, which is not drawn
status: closed
opened: 2026-11-02
closed: 2026-11-02
branch: vseam/an-edited-profile-is-not-picked-by-its-old-outline
pr: 6352
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

## Closed

Closed 2026-11-02 by PR 6352. `display::DisplayView::previewed_profile`
carries the node the edit door previews in place together with the
world outline of that preview, which the frame builds from the preview
it already took (`display::PreviewedProfile::of`). The committed-profile
lane passes the node to `sketch::committed`'s `except`.
`PickIndex::drawn_outline` gives the preview's outline for that node and
the committed one for every other node. `PickIndex::near` and
`marks::edge_overlay` both read it. So the old outline is neither drawn
nor picked, and the preview is picked and marked as its profile. A
click on the preview keeps the selection and the draft. Rows:
`app::properties_pane_tests::a_profile_edited_in_place_is_picked_by_its_preview_not_its_old_outline`
and `marks::tests::a_profile_previewed_in_place_is_marked_by_its_preview`.

The selected-mark finding filed with the first version of this PR,
`a-selected-profile-is-marked-at-its-committed-outline-while-edited`,
was the same fix and is closed by the same PR.
