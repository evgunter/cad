---
id: a-free-moved-outline-is-drawn-untinted-and-marked-at-its-first-drawing
kind: issue
title: viewer: a free-moved outline is drawn untinted, marked at its first drawing only, and its placement carry re-derives display_map
status: open
opened: 2026-11-25
priority: P3
cost: E
---

Found by the delta review of PR 7771 (`vgeom/profile-display`), which puts committed outlines through the display state. Three loose ends of that change, none wrong today in a way a test sees:

1. **The placement carry is composed twice.** `display::DisplayView::outline` rebuilds the composition `DisplayView::display_map` gives a copy's body (the draft's carry under the frame of the space the draft stands the copy in) instead of calling it. The two differ on purpose where no copy is previewed: `display_map(copy)` reads `moved_nodes[copy]`, which holds only a copy the picture draws, while `outline` reads the drawn space's frame, so an outline shown through a copy an operation reads still moves with its space. Only a code comment keeps the two in step. The test `pane::viewport::tests::outline_display::an_outline_through_a_copy_rides_the_placement_preview` holds no free-move frame, so the frame half of the composition could drift and stay green.
2. **Marks light one drawing.** `PickIndex::drawn_outline` answers the first drawing of a node (`PickIndex::drawings_of` answers all of them), and `marks::edge_overlay` lights a selected or hovered profile through it. A profile drawn through two copies is marked at one copy only, though both are picked.
3. **A moved outline is not tinted.** A free-moved body is drawn with the probe tint (GUI-DESIGN G3: what is displaced from where the document puts it says so). The outline lane draws a profile or datum in a moved space in its plain colour, and `marks::edge_overlay` never sets a probed flag for an outline, because the flag means "composited over a probe-tinted body base", and an outline has no body base.

What it should do: (1) one home for "the map a drawing carried by a copy is drawn under", read by both, with a row holding a free-move frame and a draft together; (2) mark every drawing (`drawings_of`); (3) decide how a displaced outline says so, then tint it in the lane and the marks alike.

