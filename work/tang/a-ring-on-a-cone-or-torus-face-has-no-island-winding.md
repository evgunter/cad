---
id: a-ring-on-a-cone-or-torus-face-has-no-island-winding
kind: issue
title: A ring on a cone face has no island winding or re-homing reading (a torus face's section is refused before the ring lane)
status: open
opened: 2026-10-07
priority: P2
cost: M
refs: [a-ring-on-a-sphere-face-has-no-island-winding]
---


Found by the sweep of `a-ring-on-a-sphere-face-has-no-island-winding`
(TANG), which gave a sphere face's ring lane a chart-free winding and
re-homing reading.

## What is left

The island winding `boolean::join::choose_roles` asks for on
`RingClosure::Wall` reads a cylinder wall's chart
(`chord_join::chart_island_winding`) or a sphere's sections
(`chord_join::sphere_island_winding`). A cone face's ring lane reaches
`chart_island_winding` and refuses `Join(RingIslandUnread { kind:
Cone })` there, through `chord_join::ring_island_unread`.

Ring re-homing on a cone face never reaches `chord_join::chart_ring_side`:
`ChordJoiner::rehome_rings` reads a chart only for a cylinder or a
sphere, and sends every other kind to `face_plane_normal`, which
refuses `SectionInvariant` ("ring re-homing reads the divided face's
plane; this face's carrier is not a plane (arm not wired)"). So
`chart_ring_side`'s own `ring_island_unread` arm is unreachable.

A torus face is refused earlier, by `chord_join::wall_section`
("a section through a face kind the gate refuses"): its ring lane is
not reached at all.

## Measured

Nothing yet: no row reaches a ring on a cone or torus face. The first
step is a pose (a box edge through a cone's wall, clear of its rims
and apex) under every op.

## What a fix owes

The sphere arm's statement leans on the section plane cutting the
sphere in a circle that bounds two caps; a cone or torus section has no
such pair, so it does not carry over. Candidates: the cylinder arm's
chart reading on a cone's (apex-free) chart, or the sphere arm's
great-circle parity carried to a path on the face whose crossings with
the loop's carriers are closed-form.
