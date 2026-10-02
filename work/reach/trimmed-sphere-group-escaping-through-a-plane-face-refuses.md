---
id: trimmed-sphere-group-escaping-through-a-plane-face-refuses
kind: issue
title: A trimmed sphere face group that a plane face cuts with no edge crossing refuses: the escape re-chart serves only closed groups
status: open
opened: 2026-10-02
priority: P1
cost: H
refs: [ball-inside-a-two-sphere-body-refuses-at-the-extent-scan, tilted-sphere-pair-section-refuses-at-the-polar-gate]
---


Found by the review of PR 3801 (the extent scan's face-scoped reading).

## Measured (branch `reach/extent-scan-faces`)

The lens `ball(1, y=0) ∩ ball(0.8, y=1.4)` (`snowman.rs` constructors)
against a 6 × 1 × 6 slab whose near face lies 0.985 from the origin
along the y axis tilted 20° about x (`snowman.rs` `tilted_slab`): the
plane cuts a cap of height 0.015 off the lens's top face, its circle
wholly inside that face and inside the plane face, at azimuth π/2 or
3π/2 of the lens's chart, so no edge of either body crosses a face.
Every op in both orders refuses `FallbackExtentUnsupported`, "a
TRIMMED sphere face group escapes through a plane face"
(`boolean::ops::sphere_extent_scan`'s plane arm). The oracle is
closed form: lens ∩ slab is the cap, `πh²(3 − h)/3` at `h = 0.015`,
and the rest follows from the lens's two caps and the slab's 36.
Pinned as the refusal by
`a_tilted_slab_against_the_lens_builds_or_refuses_the_trimmed_escape`.

Tilted about z instead (the circle across the lens's seam meridians),
the pose reaches the crossing layer and refuses
`Join(SectionNotPolar)`, the tilted-section door.

## The shape

The scan's escape conclusion feeds a re-chart that rotates a CLOSED
sphere group about its own centre so that its seams cross the escape
plane and the crossing layer sees the section circle. A trimmed group
(any boolean result carrying part of a sphere) cannot be rotated
alone, so its real escape has no repair and refuses. A fix supplies
an event for the circle on a trimmed face: re-charting the face's own
seam, or cutting the circle in directly.
