---
id: the-describing-door-refuses-a-wall-re-charted-reversed-in-u
kind: issue
title: set_face_surfaces_describing refuses a loft wall re-charted onto its own surface reversed in u: the carried seam restatement certifies Unimplemented
status: open
opened: 2026-10-23
priority: P3
cost: M
---

Found by `iso/arc-rim-sub-arc` (PR 5630), writing a row whose wall runs
its arc rims against the chart's `u`.

## What happens

`sweep::loft_body` of a square profile with one semicircular edge
(bulge 1). Its arc wall's net reverses exactly (`NurbsSurface::reversed_u`,
the knots mirror bit for bit). Re-charting that wall onto the reversed
net through the describing door, with the edges the move strands
restated by `Body::carried_redescriptions`, refuses:

```text
RechartFalsifies { door: SetFaceSurfacesDescribing, edge: EdgeKey(6v1), error: Unimplemented }
```

`6v1` is a wall–wall seam, a straight degree-1 spline carrier at
`x = 2`, and its carried spec is `Chart { surface: SurfaceKey(4v1),
image: None, wrap: false }`. So the door offers a re-description and
then cannot certify its own carried spelling of a seam it would keep.
The re-chart is the same surface (the same points, `u` reversed), so
every edge on the wall lies where it did.

Rows that want such a wall fall back to the failure-injection door,
`set_face_surface_unvouched_for_tests`, which leaves the descriptions
naming the old chart, and so they cannot read tier 3. `sweep`'s
`a_rim_covering_part_of_its_row_re_mints_the_rows_arc::a_rim_against_the_charts_u_re_mints_its_pieces`
and `an_arc_rim_picks_the_row_it_lies_on::a_rim_against_the_charts_u_takes_its_reversed_row`
both do this.

## Fix

Find which certification lane returns `Unimplemented` for the carried
`Chart` seam on the reversed pair (the seam's other side is a spline
wall too, so a spline × spline line seam), and either certify it or
have `carried_redescriptions` carry a spelling the door certifies. Then
move the two rows above onto the describing door and read tier 3 there.

