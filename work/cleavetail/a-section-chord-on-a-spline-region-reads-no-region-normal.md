---
id: a-section-chord-on-a-spline-region-reads-no-region-normal
kind: issue
title: A section chord whose region is a spline face reads no region normal, so the chord ladder passes it as a limit
status: open
opened: 2026-11-27
priority: P3
cost: M
---


## What

`topo::boolean::join` `seam_germ_side` reads a section loop's side at
each chord as the region face's inward direction against the holding
face's outward normal. Since the LATHE teapot lane
(`lathe/the-teapot-spouts-union-stops-at-section-loop-undecided-where-its-chords-hold-in-a-spline-wall`),
a spline HOLDING face reads its normal. `holding_face_normal` keeps the
in-trim foot its placement found (`contain::face_placement_charted`)
and reads `face_normal::spline_outward_normal_at` there
(`join::spline_held_normal`).

A spline REGION face does not read its normal. `held_normal` on the
region arm answers `HoldingUnread::Spline`, and the chord is passed as
`Refusal::Limit`. The chord's point lies on the region's boundary, so
a placement there reads `OnEdge` and returns no foot. The chart point
would have to come from the chord's own pcurve on the region face, or
from a projection onto the region's sheet with a certified
single-foot bound.

## Reach

No measured scene hits this yet. The teapot's wall-3 loops that reach
the chord reading are on the vessel's side, where every region is
analytic (measured 2026-11-27 at ε 1e-9 and 1e-6). A loop on a spline
operand's side whose regions no witness reads would refuse
`Join(SectionLoopUndecided)` here.
