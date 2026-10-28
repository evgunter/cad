---
id: a-fused-instances-held-frame-has-no-reset
kind: issue
title: viewer: a frame held on a loose instance that is then fused has no control to reset it
status: closed
opened: 2026-10-18
closed: 2026-10-28
priority: P3
cost: E
---

Found by the sequential review of PR 5296 (F2, demonstrated by execution).

A loose instance `a` holds a committed free-move frame. An edit then fuses
`a` with `b` into a `Union` that a world-pinned `Place` places. The prune keeps
the frame on purpose (`crates/viewer/src/display.rs`, module doc, "Hides name
instances"): `a`'s space is still loose and the frame still moves `a`'s own
copy. The properties pane, though, ends the instance's section at the hide
toggle when `display_check` refuses (`crates/viewer/src/pane/properties.rs`,
`addressable.is_err()`), and `free_move_check` runs `display_check` first, so
it refuses `FusedGeometry`. That leaves the held frame on screen with no
control to reset it. Only an edit that pins or removes the space, or an undo,
clears it.

Options: have the free-move admission skip `display_check`, since a frame
moves a space and not `drawn_targets` (the pane's early return would then have
to split), or have prune discard a frame whose root's instance becomes fused.

## Re-read at D′1 (2026-10-22)

There is no loose instance any more: a free-move frame is keyed by a
shown space's variable (`crates/viewer/src/display.rs`) and checked
against the document alone at each edit. Whether a frame held on a
space whose copy is then fused still has no reset was not re-measured.

## Closed (2026-10-28): the premise is false on main

Re-measured through the session's doors. A space is shown only while
no placement reads a body of it (`shown_spaces` in
`crates/viewer/src/world.rs`), and the placement that draws a fused
product reads a body of the space its copies are in: a union of the
copies lives in that space, and a copy placed out of it reads it too.
So the edit that fuses a space's copies into a placed product stops
the space being shown, and that edit discards the frame and says so
(`AdmissionFault::NotShown` in the edit's withdrawals). No frame is
left without a reset.

Pinned by `a_frame_whose_copies_are_fused_goes_saying_so`
(`crates/viewer/tests/assembly_display.rs`): the shelf and post_b in one
space with a frame held on it, then a stored union of the two placed by
`SessionOp::Duplicate`. Both copies then refuse `FusedGeometry`, and the
edit withdraws the frame. The module doc in `crates/viewer/src/display.rs`
said the frame stays; it now says it goes.

The one shape the argument does not cover is a fused body the recipe
holds and the evaluation refuses (members in two spaces,
`SpaceMismatch`): no session door places it, since `Duplicate` refuses a
body with no value, and no frame can be held on a copy that is already
fused.
