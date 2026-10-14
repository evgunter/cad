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
109), 2026-10-10.

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

**What the build changed under the borrow (FLUX `flux/sphere-arm`,
2026-10-10).** `boundary_material_sign` on a sphere is now
`sphere_loop_side`. A one-loop face answers `Encoded` when a pole off
its loop says which side holds none, which is the window's own question
once the sense agrees. A face whose loop doubles back on a strut answers
`Err` (`props_sphere_loop_cusp`), so the window falls back to the ball.
`boxes::sphere_rect_rows`' zone-arm rows were built on such strut caps,
so they now stand on a seamed zone (`strut_cap(.., zone: true, ..)`),
and `a_caps_strut_does_not_bound_its_latitude_window` pins the
strut cap's fallback to the ball.
