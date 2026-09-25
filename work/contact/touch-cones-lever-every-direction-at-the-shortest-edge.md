---
id: touch-cones-lever-every-direction-at-the-shortest-edge
kind: issue
title: The census's touch analysis levers every direction at the shortest edge of either cone, so a sliver corner's long nearly-in-plane edge reads in band where its far endpoint is definite
status: open
opened: 2026-09-25
priority: P3
cost: E
---


Filed by CONTACT-1, as a narrowing its analysis makes by name. The
census's touch analysis (`crates/topo/src/census.rs`, `touch_verdict`,
`Cone::within`) decides each direction's side of a candidate plane as
a dot of unit vectors levered at ONE metering arm: the shortest edge
of either cone at the touch (the `sector_shape` convention, where the
local cone stops describing the solid). A corner with a 3 cm edge
beside a 0.8 m edge that leaves the plane at `1e-7` m reads that long
edge at `3.75e-9` m, in band, where its far endpoint is definite; the
vertex-on-face analysis it replaced read the far endpoint and cleared.

Witness: `contact1_touch_cones::a_sliver_corner_whose_side_is_in_band_refuses_typed`.
The refusal is typed (`TouchInBand`) and conservative; the question is
whether each boundary ray should be levered at its own edge's length
(a chord ray) and only synthetic directions (sector bisectors, a
wedge's in-face directions, cross-product candidates) at the arm.
Difficulty S.
