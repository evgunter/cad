---
id: the-seams-chart-arm-builds-every-cell-of-the-chart-per-read
kind: issue
title: The seam's chart arm builds every cell of a spline chart, uncached, for each spline face of each seam it reads: about 0.85 s a read in debug on lily's walls
status: open
opened: 2026-10-27
priority: P3
cost: M
---


Filed by the SPLICE lane that gave the result's seam a spline reading
(`the-results-seam-between-two-spline-walls-is-read-by-no-dihedral`), on
its review (PR 5868, both reviewers).

## What

`offset_meters::chart_arm` (through `NurbsLane::chart_arm`, which
`geom_brep::classify_seam` calls once per spline face of a seam) builds
every cell of the chart at `RATIONAL_CERT_SPLITS` per span
(`patch_cells_refined` or `patch_cells_across_creases`), then keeps the
cells within the reach. Nothing is cached: `describe_edges` reads it again
for every face of every seam, and a chart shared by several seams is
re-celled each time.

Measured (debug, the sweep row
`a_sheath_grafted_on_its_blade::a_sheath_on_its_blade_stops_where_its_walls_touch_on_the_rim`):
four reads on the row's rational loft walls (15 control points each) took
0.80 to 0.92 s apiece, about 3.4 s of the row's 100 s.

## The likely shape

Cell only the spans whose control hull comes within the reach of the
point (a span's surface lies in its active control points' hull), or
cache a chart's cells beside its key for the length of one describe pass.

Moved from SPLICE to SPLICESEAM on 2026-10-28, by `git mv` in SPLICE's split along its seams (Track size; Ev, in chat, 2026-10-28).
