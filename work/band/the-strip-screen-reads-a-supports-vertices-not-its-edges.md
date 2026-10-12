---
id: the-strip-screen-reads-a-supports-vertices-not-its-edges
kind: issue
title: blend: reach::screened reads a support's vertices alone, so an edge across the strip with no vertex in it goes unseen
status: open
opened: 2026-10-12
priority: P3
cost: M
---


## Finding

`reach::screened` (`blend/reach.rs`) is the metered premise of
`Reach::replaces`: the strip a straight link's band replaces holds no
boundary feature of the support but the request's own. It reads the
support's VERTICES against the region. A boundary edge can cross the
region with neither end inside it: a straight edge parallel to the
requested edge's line, across it within the band's chord, whose ends
lie past both window ends (a far-side plateau along the line), or one
cutting a corner of the region between a window end and the tube.

Two readers stand on that premise:
- `screened`'s own refusal, on the trimline's side. A near-side edge
  of that shape is read elsewhere: predicate 2's sampled screen, the
  surgery's strip meter for a plane–plane link, the cell meter for
  the edge's own walls.
- `band_reach`'s cross-chain excuse, which reads the strip on BOTH
  sides of the line where `screened` finds no vertex across it (that
  is what lets an oblique cell straddling the line be excused). A
  far-side edge of that shape keeps the excuse two-sided, so a cell of
  the support's far part could be excused though no band replaces it.

No witness: a wrong excuse there needs a second chain's band to reach
the support's far part, within the first band's chord, without
reaching the walls that bound it. The merge base read the excuse
two-sided always, so this is no wider than it was.

## What would close it

Read each boundary edge of the support against the region, certified
(a line edge's distance to the edge's line is convex along it, so its
least value over the window is a closed form), rather than its ends; or
write the witness and show it refuses.

