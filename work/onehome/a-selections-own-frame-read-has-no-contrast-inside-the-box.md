---
id: a-selections-own-frame-read-has-no-contrast-inside-the-box
kind: issue
title: A selection's own-frame read has no row that a world-composed read would fail inside the session box
status: open
opened: 2026-11-18
---


`[placement.frame]` has a selection read the model in the body's own
frame, composing nothing into a space. A selection over a placed copy
decides a distance between the copy's own plane and its own corners.

The guard used to be
`lib_sel1_geoselect::a_far_turned_copys_corners_sit_on_its_own_plane`. It
stood a turned copy at 1e8 m, where composing into the world rounds a
corner off the composed plane by about 7.5e-9, inside the band. There the
world-composed read refused `InBand` and the own-frame read answered four
corners.

Since PR 7088 a placement asks the session box (1 km), so that copy
refuses at its Place and the row can't stand there. At about 9e2 m
(`crates/editor-core/tests/lib_sel1_geoselect.rs:804`, now
`a_turned_copys_caps_corners_are_selected_on_their_planes`) the
translation's rounding, an ulp of 1e3 or about 1.1e-13, is below the
band. A world-composed selection answers the same, so the row pins the
answer and not the frame.

Owed: a contrast that can arise inside the box. One candidate is a
selection whose world composition goes through a widened enclosure-lane
rotation, where composing widens the plane's enclosure past the band
while the own-frame read stays exact. Another is a lever on the rounding
that composing adds, read at the witness. Whichever is chosen, it is a
row red against a selection that composes the copy into the world.
