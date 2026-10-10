---
id: census-containment-gate-reads-instances-along-world-axes-only
kind: issue
title: The census containment gate clears an instance pair only on a world-axis margin, so whether it probes depends on how the pair is turned
status: open
opened: 2026-10-10
priority: P3
cost: M
---



Split from `census-backstop-separates-curved-pairs-only-along-world-axes`
(arm 1's half landed in its PR, on branch `contacthold/census-backstop`).
Unmeasured: read off the code; no fixture has been built for it.

## What

`census::sweep_cross_solid_backstop`'s arm 2 (instance containment)
reads a solid's claimable extent as the WORLD box hull of its faces'
`census::face_reach` boxes (`solid_reach`), and its gate clears an
ordering for free only when a shell's vertex hull has a definitely
negative `census_backstop_containment` margin on world `x`, `y` or `z`.
Its pre-filter (`extent_boxes`) is the same world box, padded. A turned
container reads wider, so a pair the gate clears upright is sent to the
material test when turned.

Over-width there is not a wrong clear: the material test probes every
vertex through `boolean::point_in_solid_faces` and is the real
decision. But the probe can refuse (an escalation, a kind the door does
not serve, every vertex `OnBoundary`), so a pair's verdict can still
move with its pose.

## The general form

As arm 1 now does (`census::backstop_apart`, over
`boolean::separating::apart`): read the container's extent and the
contained shell's vertex hull along directions that turn with the pair
(`census::face_reach_in` in `boxes::BoxFrame::aimed` frames; the axis
between the two instances' anchors, the planar normals of the
container), keeping the world axes as first candidates. A clear stays
sound only where the contained side is a SUBSET (vertices) and the
container a SUPERSET (face reaches) along the same direction.
