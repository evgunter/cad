---
id: pick-face-orders-targets-on-rays-each-carried-by-its-own-body-frame
kind: issue
title: pick_face orders targets with different body frames by spans each certified on the ray carried into its own frame
status: open
opened: 2026-11-25
priority: P3
cost: M
---


## Finding

`crates/editor-core/src/resolve/pick.rs`, `pick_face`: each target's
ray is read into the coordinates its mesh is in (`target.map`,
`back = map.linear().transpose()`, origin `back * (ray.origin −
map.origin())`), and every candidate's `TSpan` is certified on that
carried ray as rounded. The survivors are then taken over all targets
together (`lowest_hi`, `TSpan::survivors`). The doc states "a rigid map
keeps the ray's parameter, so every target's `t` is one parameter along
the one ray" — true of the parameter, not of the certificate: two
targets under different frames are ordered by spans certified on two
different rays, each the caller's only to its carry's rounding, which
is of the order of the frame's translation's ulp and can exceed the
width a crossing near its local origin certifies (the transpose is not
the inverse either, to the frame's own rounding).

The viewer's half of the same class was measured and repaired in
`vgeom/moved-copy-witness` (`crates/viewer/tests/moved_copy_pick.rs`):
a copy posed ~600 lengths out and drawn back near the origin had a
quarter of its edge hovers read as hidden by its own face, and a
coincident twin pair was ordered where it should tie. Inside one
`pick_face` call the case needs two targets whose drawn faces coincide
to within that rounding under different frames — two bodies whose
local geometry differs by the offset their frames undo — which is rarer
than the display case, hence P3.

## Next step

A witness: two targets over meshes offset by `s` in local coordinates
and framed by translations that undo it, a ray onto the coincident
face; the call should refuse with both. If it flips, the viewer's
repair shape applies here (`viewer::pickindex::carry_bound` /
`carried_reach`: grow each carried span by the carry's rounding over
the hit face's triangles, capped at each triangle's extent along the
ray), or certify each candidate on the caller's ray against corners
placed through the frame.
