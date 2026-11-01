---
id: view-pane-cannot-hide-a-shown-space
kind: issue
title: the View pane cannot hide an unplaced body or the world
status: open
opened: 2026-10-31
priority: P1
cost: M
design: true
refs: [6209]
---


## Finding

Ev (in chat, 2026-10-31): the View pane should let each separate
unplaced body, and the world, have its visibility toggled.

Today hiding is per instance, in the Properties pane
(`hide_toggle`, `crates/viewer/src/pane/properties.rs`, behind
`display::instance_check`), and the View pane has no visibility
controls (it shows the display δ, counts, the camera readout and the
history). The picture already draws a set of shown spaces
(`world::shown_spaces`; viewer README, *The picture shows a set of
spaces*), so the natural unit for the toggle is the shown space: the
world, and every space a body lands in unplaced
(`work/spaces/a-new-body-lands-placed-in-the-world.md`).

Hiding stays display state, never persisted (G3).
