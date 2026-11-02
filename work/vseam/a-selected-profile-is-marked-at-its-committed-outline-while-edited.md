---
id: a-selected-profile-is-marked-at-its-committed-outline-while-edited
kind: issue
title: a profile edited in place is marked as selected at its committed outline, which the pick no longer offers
status: closed
opened: 2026-11-02
closed: 2026-11-02
branch: vseam/an-edited-profile-is-not-picked-by-its-old-outline
pr: 6352
priority: P3
cost: E
---


Found by the lane that kept the pick off an outline the edit door
previews in place (`an-edited-profile-is-not-picked-by-its-old-outline`).

Selecting a profile opens its edit door, so a selected profile is
always previewed in place (`drafts::Drafts::edited_in_place`). The
committed-profile lane and the pick both leave its committed outline
out (`display::DisplayView::previewed_profile`). The selection mark
does not: `marks::edge_overlay` fills the selected lane from
`PickIndex::profile_outline(node)`, which is the committed outline
regardless of the view. Once the draft moves a side, the old side is
still drawn in the selected tint, where nothing is offered to a click,
while the moved side is drawn only in the preview tint.

`app::properties_pane_tests::a_click_on_a_profiles_outline_selects_it_and_beside_it_nothing`
asserts the selected mark's eight legs while the edit door is open,
so the selection mark is the only drawing of the committed outline at
that moment. The question is what a selected profile under edit is
marked by: its committed outline (today), the preview's loops, or
nothing beyond the preview tint. `edge_overlay` already takes the
`DisplayView`, so whichever answer is chosen reads the same exclusion.

## Closed

Closed 2026-11-02 by PR 6352. `marks::edge_overlay` now marks a
profile by `PickIndex::drawn_outline(node, display)`. That is the edit
preview's outline while `DisplayView::previewed_profile` names the node,
and the committed outline otherwise, so a selected profile under edit is
marked where its preview draws it. Row:
`marks::tests::a_profile_previewed_in_place_is_marked_by_its_preview`.
