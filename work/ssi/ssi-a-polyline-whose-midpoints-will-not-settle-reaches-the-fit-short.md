---
id: ssi-a-polyline-whose-midpoints-will-not-settle-reaches-the-fit-short
kind: issue
title: ssi/refine: fit_minimum returns a polyline short of the cubic's samples where no midpoint settles inside the domain, so the fit refuses TooFewPoints with the defect ending
status: closed
opened: 2026-10-04
priority: P3
cost: E
closed: 2026-10-05
pr: 4034
branch: ssi/neighbour-cap
---



(SSI implementer on `ssi/neighbour-cap`, PR 4034, from its §5 sweep.)

## What

`fit_minimum` (`ssi/refine.rs`) gives a polyline the cubic's four
samples by halving its longest gap whose midpoint settles onto the
locus inside the domain. Where half of a gap falls in the band it
refuses, sized (`ShortBranchUncertified`). Where every midpoint fails
to settle inside the domain instead, it returns the polyline short, and
`fit_branch` refuses `Fit(TooFewPoints)`, whose ending is the kernel
defect's. Before PR 4034 the ℝ³ lane's still-short trace refused
`TraceUnresolved` (the slab's limit) and no trace reached the fit
short. A branch lying along a slab face within the settling residual
is the reading that could reach it: its midpoints settle just outside
the slab. Not met by a fixture.

## Done when

A polyline whose midpoints will not settle inside the domain refuses
as its lane's limit (on the ℝ³ lane `TraceUnresolved`, the slab's), or
a row shows the case cannot arise.

## Closed (2026-10-05, PR 4034)

Refinement's one-arc stop replaces the short exit. `refine.rs::halve`
keeps a midpoint only where it settles within half its gap and the
settling residual of the gap's chord midpoint, the most an arc turning
by at most π lies from it. A midpoint that settles farther, or does
not settle, refuses the polyline as not one arc
(`SsiError::PolylineNotOneArc`, the march's limit, one recourse); in
`fit_minimum`, so does a polyline none of whose midpoints settles
inside the domain and none of whose half gaps falls in the band. Every
other exit of `fit_minimum` is the sized refusal, so `Fit(TooFewPoints)`
is unreachable from refinement. Rows:
`the_fits_minimum_halves_to_four_and_stops_at_a_gap_of_more_than_one_arc`
and `refinement_stops_at_a_gap_that_does_not_hold_one_arc`
(`ssi/refine.rs`).
