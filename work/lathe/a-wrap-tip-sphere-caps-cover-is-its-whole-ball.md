---
id: a-wrap-tip-sphere-caps-cover-is-its-whole-ball
kind: issue
title: A one-band sphere cap reached by its wrap tip reads no chart window, so its cover is the whole ball and the box cull keeps pairs it used to drop
status: closed
opened: 2026-11-23
priority: P1
cost: M
refs: [a-full-revolves-curved-wall-carries-two-seam-meridians-nobody-wrote]
closed: 2026-11-26
pr: 7487
---


Found by the LATHE one-site lane (2026-11-23), measured on its branch
`lathe/one-site-wrap-edges` (stacked on `lathe/full-revolve-one-band`).

## What

The snowman lens (`intersect(ball(1, 0), ball(0.8, 1.4))`, `sweep`'s
`pierce_tangent_off_face::lens`) is now two one-band caps, each bounded
by the rim circle and its wrap edge walked out to the pole tip and back.
`topo::cover::face_cover` of each cap is its WHOLE ball: the unit cap's
`aabb` spans `[-1, 1]³`, the 0.8 cap's `y ∈ [0.6, 2.2]` (measured by a
throwaway probe). `cover.rs` `sphere_window` reads no window for them:
its doc says "a wrap-tip cap holds the pole at its tip, outside that
class, and its rim alone encodes no side", so the material-sign gate
refuses and the face keeps the ball. With two π-bands each band's
rectangle was read and the cap's box was floored near the rim.

So `pierce_tangent_off_face::a_touch_outside_the_lens_face_within_reach_of_its_rim_along_it_builds`,
whose argument is that the cap's box (floored at `RIM − pad`) culls the
brick's edge, now reaches the pierce and refuses
`CurvedPierceUnsupported { operand: B, .. }` under union.

## Ground

`crates/topo/src/cover.rs` (`sphere_window`, `face_cover`); the trim
`solid_contain::sphere_chart_trim` it reads. A cap whose loop reaches a
pole by a wrap tip lies in the latitude zone from its rim to that pole,
which is the window it should read.

Possibly the same cause as
`split-refuses-a-plane-clear-of-a-one-band-sphere-cap` (a plane half a
unit clear of a one-band cap reaches it); whether its reach reads this
cover is not measured.
