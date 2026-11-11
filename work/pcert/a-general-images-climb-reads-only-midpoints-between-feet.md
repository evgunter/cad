---
id: a-general-images-climb-reads-only-midpoints-between-feet
kind: issue
title: A General image's climb reads its miss only midway between feet, so a path that wanders between a foot and a midpoint keeps a degree-1 image its certificate refuses
status: open
opened: 2026-11-11
priority: P3
cost: M
---


Filed by `pcert/general-image-climbs-on-its-miss` (PR 6819), from its
review's NN4.

## What

`edge_nurbs::general_chart_image` decides whether to climb past the
degree-1 image, and which rung to stop at, from the miss read at one
station per node window: the midpoint, net of the standoff the two nodes
carry. A carrier whose path leaves the image between a node and the next
midpoint, and comes back by that midpoint, reads straight there. It
keeps the degree-1 image (or a coarse rung), and that image's
certificate, which reads between the stations, refuses it. The ending is
the lane's gap, "not yet" (`LaneGap::Image`), so this is a capability
gap, not a wrong ending: a forced climb of the same carrier certifies.

Probe: `p4_threshold_*`, the "tangential wiggle" half, in
`crates/geom-brep/src/pcurve_cache/review_6819_probes.rs` on
`review/6819-full` @ `045d389e`. It uses a uniform cubic spline on 256
spans along a flat bilinear wall, with one control point pushed across
its direction in the wall's plane by `k·ε`, `k ∈ {0.25, 1, 10}`. Its
support, `[(j−3)/256, (j+1)/256]` at `j = 131`, lies between node `1/2`
and midpoint `33/64` of the 33-foot schedule. The producer keeps degree
1 and the mint ends "not yet"; the climbed image certifies.

## Direction

Read more than one station per window, at a cost that stays below the
certificate's (the trigger runs on every minted image, and its cost is
evaluations, not projections). Two candidates:

- quarter points as well as the midpoint;
- the carrier's own knot spans where it is a spline: a bump narrower
  than a schedule step is a spline's, and its knots say where it is.

The certificate reads the whole span, so no reading here is unsound.
Each blind spot only keeps a coarse image the certificate then refuses.
