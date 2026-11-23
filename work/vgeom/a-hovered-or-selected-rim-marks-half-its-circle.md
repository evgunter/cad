---
id: a-hovered-or-selected-rim-marks-half-its-circle
kind: issue
title: a hovered or selected arc of a seam-split rim marks half the circle the picture draws
status: open
opened: 2026-10-31
priority: P3
cost: E
---


Found by VSEAM's sweep for `a-picked-half-rim-blends-half-and-refuses`
(branch `vseam/a-picked-rim-is-the-whole-rim`).

On a full revolve of a profile touching the axis, every latitude
circle is two edges, `BandRim` and `BandRimPi`. That PR gives the pick
index the fact that they are one drawn circle
(`PickIndex::whole_edge`, `crates/viewer/src/pickindex.rs`), and the
blend tool now holds a click on the circle as both arcs, so its held
lane (`marks::HeldEdges::mark`) draws the whole circle.

The hover and selected lanes do not ask. `marks::edge_overlay` marks
each through `marks::edge_segments`, which reads
`PickIndex::edges_of_target(edge).first()`: one arc. Hovering the
circle lights half of it; clicking it with no tool open lights half of
it; and with the blend tool open the hover shows half while the click
holds all of it. The picture contradicts itself about what the cursor
means.

Fix shape: the hover and selected lanes mark `whole_edge`'s names
(each through `edges_of_target`), so the lit circle is the one a
click means. The selection value stays one name (GQ7's single-select);
only what the picture lights changes. A row driving a hover over the
`axis_cylinder` fixture in `crates/viewer/tests/blend_authoring.rs`
and counting the hovered lane's segments against the held lane's
would go red today.
