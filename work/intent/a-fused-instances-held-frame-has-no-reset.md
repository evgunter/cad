---
id: a-fused-instances-held-frame-has-no-reset
kind: issue
title: viewer: a frame held on a loose instance that is then fused has no control to reset it
status: open
opened: 2026-10-18
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
