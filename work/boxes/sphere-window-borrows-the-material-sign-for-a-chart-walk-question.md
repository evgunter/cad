---
id: sphere-window-borrows-the-material-sign-for-a-chart-walk-question
kind: issue
title: boxes::sphere_window borrows boundary_material_sign == Encoded(sense) to mean 'the face lies in its boundary's rectangle', a chart-walk question the material sign does not answer
status: open
opened: 2026-10-10
priority: P4
cost: E
---


Found off-question by the sphere-arm designer pair (FLUX fork log row
108), 2026-10-10.

`topo::boolean::boxes::sphere_window` takes
`geom_brep::props::boundary_material_sign(...) == Encoded(sense)` as
"the face lies within its boundary's chart rectangle", and falls back to
the ball box when it does not get that answer. That fallback is looser
but never wrong.

The question the window actually needs answered is whether, given σ,
the face holds no pole. FLUX's sphere-arm build changes what the
material sign answers on one-loop faces. It will answer `Encoded` more
often, so the window keeps working. The borrow is still the wrong
question, and the window should ask its own. (FLUX orchestrator)
