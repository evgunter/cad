---
id: a-meeting-on-a-spline-boundary-edge-has-no-split-parameter
kind: issue
title: A meeting the boundary pass finds on a spline boundary edge refuses PointSplitCarrierUnsupported: the split asks param_near for a parameter the meeting already holds
status: closed
opened: 2026-10-26
priority: P0
cost: M
refs: [a-span-is-read-against-a-faces-boundary-edges-first, an-analytic-edge-against-a-nurbs-face-has-no-crossing-lane]
branch: splice/split-parameter
closed: 2026-11-03
pr: 6441
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

Moved from SPLICECARRIER to SPLICE on 2026-11-02, by `git mv`, when SPLICECARRIER merged back into SPLICE: the track budget now counts only the P0, P1 and unbanded rows a sitting dispatches (Ev, in chat, 2026-11-02), so the four tracks of SPLICE's 2026-10-28 split came in under one budget.

## Re-banded P0 (2026-11-03)

This row is teapot wall 3's current stop: `demos/tour/src/teapot.rs` `wall_probes` pins `PointSplitCarrierUnsupported { operand: B }` on the spout. A blocker inherits the band of what waits on it (`work/README.md`, Priority), and the demo joins are P0 (`work/splice/curved-demo-joins-build.md`).

## Closed (SPLICE, PR 6441, 2026-11-03)

The boundary pass's meeting carries its parameter on the boundary edge (`Boundary::Meets.on_edge`) through the pierce to the point split, as a `carrier_cross::Held` that names the edge it was read on, so the split takes it only for that edge (a mismatch refuses as an invariant). The split no longer asks `param_near` where a meeting already holds the parameter, on any carrier: on analytic edges the stored child interval may move by one ulp against the read-back, and the vertex never moves. `sheets_part` reads the tangent at the held parameter too. Teapot wall 3 now passes its split. At the default ε and 1e-12 it stops at `HeldPierceUnbuilt { pierced_kind: Sphere }` (`a-spline-wall-cannot-say-whether-it-holds-a-pierce-point`, PR 6435), and at 1e-6 at `SplineCrossingUnsettled { why: Floor }` against a bore wall (`teapot-wall-3-at-1e-6-floors-the-meridian-against-a-spout-wall`). Residue filed: `a-point-the-pre-pass-places-on-a-spline-boundary-edge-has-no-split-parameter` (P2), `sheets-part-on-a-spline-boundary-edge-has-no-row` (P3).
