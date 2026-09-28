---
id: a-corrupt-id-on-a-hidden-roots-patch-reads-as-a-named-face
kind: issue
title: An id-buffer word that lands on a hidden root's patch id reads as a named face, not as an id this picture does not draw
status: closed
opened: 2026-09-25
priority: P3
cost: M
branch: vnews/every-id-is-said
closed: 2026-09-28
---

Filed by `vnews/an-unnamed-id-is-not-nothing` (PR #3249), which made
`idpass::IdAnswer::Unassigned` say *"id N, which no patch of this
picture draws"*.

## The finding

`IdAnswer::of` reads the id against the whole index. The index assigns
ids to every part, hidden or not. `PickIndex::scene_for` skips a part
whose root is in `DisplayView::hidden_roots`, so the picture draws
none of its ids, but `PickIndex::name_of` still names them. So a
corrupt id-buffer word that happens to equal a hidden root's patch id
reads as `IdAnswer::Named` and prints as an ordinary face disagreement:
*"id buffer <hidden face>, ray <visible face or nothing>"*. That
sentence claims the id buffer saw a face that is not on screen.

"An id this picture does not draw" is therefore wider than "an id the
index never assigned". `Unassigned` catches only the part of it past
the index's range.

## What a fix would be

The id side reads through the display view as well as the index: a
`Named` answer whose part is hidden is an id this picture does not
draw. `cursor_news` has the `DisplayView` in hand (`RayQuestion`).

Kept apart from `an-unassigned-id-under-a-refused-ray-is-unsaid`
because that row is about a refused ray. This one is about which ids
count as drawn.

## Fence

`crates/viewer/src/idpass.rs` (CHROME, VGEOM) and `cursor_news` in
`crates/viewer/src/pane/viewport.rs`.

## Closed

Fixed on `vnews/every-id-is-said`. `IdAnswer::of` takes the
`DisplayView` the picture was drawn under, and the arm that was
`Unassigned` is now `IdAnswer::Undrawn`: an id the index never
assigned, or one it assigned to a part whose root the view hides
(read through `IdMap::key_of`'s node, the same test `scene_focused`
drops a part by). Whether an id is drawn is asked before what it is
named, so a hidden part's id is `Undrawn` even when the naming layer
refused its name. `idpass::compare` and `cursor_news` pass the view
the ray is asked under, which is the same view.
`a_hidden_roots_patch_id_is_said_as_an_id_this_picture_does_not_draw`
hides the plate's root and asserts the sentence through `cursor_news`,
with the unhidden reading (the named face) as its control.
