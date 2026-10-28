---
id: a-meeting-on-a-spline-boundary-edge-has-no-split-parameter
kind: issue
title: A meeting the boundary pass finds on a spline boundary edge refuses PointSplitCarrierUnsupported: the split asks param_near for a parameter the meeting already holds
status: open
opened: 2026-10-26
priority: P1
cost: M
refs: [a-span-is-read-against-a-faces-boundary-edges-first, an-analytic-edge-against-a-nurbs-face-has-no-crossing-lane]
---



Filed by the boundary-first lane (`splice/boundary-first`), from
teapot wall 3's measurement.

## What

The sweep now reads an edge against a spline face's boundary first
(`geom_brep::curve_face::boundary`, consumed by `topo`
`reduce::spline_face`). Teapot wall 3's belly meridian (edge 9v1)
meets the spout's NURBS wall on the wall's seam: the meridian and the
spout's seams lie in the pot's one plane of symmetry, so the pierce
lands on a boundary edge. A meeting on a boundary edge splits both
edges (`reduce::split_other_at_point`), and that split asks
`Curve3::param_near` for the seam's parameter, which a spline has
none of. So the union refuses
`PointSplitCarrierUnsupported { operand: B, edge: 10v1 }`.

The meeting already holds the parameter: `curve_pair::meet` reads the
point on both spans, and its `Meeting::on_b` is `Hit::On { t, .. }` on
the boundary edge. The fix is to carry that parameter from
`curve_face::Boundary::Meets` through `CurvedEvent::Pierce` to the
`FaceContainment::OnEdge` split, splitting the seam where the reading
put the point rather than asking the carrier again.

## Witness

Teapot wall 3 (`demos/tour/src/teapot.rs`, `wall_probes`), pinned on
this refusal. Past it, the meridian's fragments are read again, and
whatever the boundary leaves undecided is the interior arm's
(`an-analytic-edge-against-a-nurbs-face-has-no-crossing-lane`).

Moved from SPLICE to SPLICECARRIER on 2026-10-28, by `git mv` in SPLICE's split along its seams (Track size; Ev, in chat, 2026-10-28).
