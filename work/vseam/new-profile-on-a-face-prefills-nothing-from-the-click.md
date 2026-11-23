---
id: new-profile-on-a-face-prefills-nothing-from-the-click
kind: issue
title: New profile on a face writes the face's chart at its zero slides and spin, and reads nothing off the click
status: open
opened: 2026-11-22
priority: P3
cost: M
design: true
---

Split out of `on-the-picked-face-draws-the-profile-there` (2026-11-22,
VSEAM orchestrator) so that the retirement of the fused act does not
wait on a GUI choice.

The **New profile on a face** form (`ViewerBehavior::profile_on_face_ui`,
`crates/viewer/src/pane/create.rs`) inserts the profile on the latched
face's chart at its nearest orientation, with the chart's slides and
spin at the constant 0. Ev's ruling on `[ev]` PR 7073 lets the GUI
prefill the slides and the spin from a clicked corner ("may"), and this
row is that unbuilt prefill. It needs:

- a face pick that carries the hit point to the form;
- a rule for which corner the click names;
- what the form says it read.

The row also records a second, open question: whether the plain **New
profile** button should itself draw on a picked face, so there is no
second door. Today the two are separate buttons, and
`crates/viewer/README.md` ("A create gesture commits the node")
describes both. Both are GUI choices, so the designers weigh them
before a lane builds.
