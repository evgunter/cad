---
id: a-parallel-leg-near-cos-v-minus-r-over-r-reads-touch-against-a-villarceau-edge
kind: issue
title: A torus parallel leg within about 1e-5 rad of the latitude cos v = -r/R reads Touch against a Villarceau edge it crosses transversally: the fixed Parallel x Villarceau row's both-planes-touch premise fails there
status: open
opened: 2026-11-23
priority: P2
---

Filed by GERM's `germ/co-surface-table` lane (PR 7431), from that PR's
review.

## Finding

The co-surface arm's direction table
(`geom_brep::curve_pair::on_surface::direction`) fixes a torus parallel
against a circle edge to read the parallel across the edge's plane. C5
says a pair whose curves' planes both touch the surface somewhere along
their curves has its row fixed all the same, and escalates in band
there; a Villarceau circle's plane is tangent to the torus at two
points.

The review measured a parallel leg within about 1e-5 rad of the latitude
`cos v = −r/R` reading `Met::Touch` against a Villarceau edge
(`OnSurface::meet`), though the two cross transversally there (the
oracle's crossing at the edge's parameter `t ≈ 1.875`). At 1e-3 rad off
that latitude the reading is correct. So the premise that both planes
touch, on which the fixed `Parallel × Villarceau` row rests, does not
describe that latitude: there the parallel's circle grazes the
Villarceau circle's plane while the two curves cross.

Today it is conservative: the region reads `Touch` as a graze and sets
the path aside (`region::landings`).

## For Ev

Whether C5's sentence on the fixed rows wants that latitude named, or
the table a reading that answers there. The README is not edited here.
