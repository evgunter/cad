---
id: the-lily-lanterns-can-take-their-three-tepal-seams
kind: issue
title: the lily's lanterns can take their three tepal seams: the carve builds and now draws
status: open
opened: 2026-11-14
priority: P3
cost: E
---


Found by TESSLANE unit 3 (`a-curved-faces-domain-is-read-from-every-loop`).

The tour's lily wall 7 asked to carve the lantern's three tepal seams
(`demo_tour::lily::seamed_lantern`, the wall-7 ball turned a third and
two thirds of a turn about the lantern's axis) and draw it. The carve
built; the drawing refused `RingOnCurvedFace`, because the seam a third
of a turn round lies inside one half-band of the zone sphere, a ring of
that face. Since unit 3 the trimmed lane meshes a ringed sphere face,
and the seamed lantern draws at `δ = 2e-3`: 4363 mesh points,
`check_mesh` clean, its volume within the chordal deficit of the
body's (`demos/tour/tests/lily.rs`,
`wall7_the_cone_pair_is_box_looseness_the_ball_meets_the_zone`).

Wall 7 is retired in `demo_tour::lily::wall_probes`, which now prints
that the seamed lantern draws. What the wall's retirement asked for —
giving the scene's lanterns their three seams — is a scene change that
moves the tour's rendered frames, so it is left to this row: author the
seams into the plant (`lily::plant`), render with a `[render]` commit,
and look at the frames before re-baselining them.
