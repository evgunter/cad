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

`fit_minimum` no longer returns a polyline short. A midpoint that does
not settle is read by the march's transversality decision at the gap's
chord midpoint: near tangent, that refusal stands (the clearer angle);
clear of the band, the gap counts as `unsettled`. Where nothing halves
and every gap's half falls in the band, the polyline refuses sized
(`ShortBranchUncertified`); otherwise it stops with nothing to halve,
counting what each midpoint did: on the ℝ³ lane
`RefinementExhausted`/`NothingToHalve` with the slab's limit
(`TraceUnresolved`), on the plane × NURBS lane `MarchShortOfFit`. So
`Fit(TooFewPoints)` is unreachable from refinement. Rows:
`the_fits_minimum_halves_to_four_keeping_every_settled_midpoint`,
`a_midpoint_that_does_not_settle_is_read_by_the_transversality_at_its_chord`
and `a_wall_polyline_short_of_the_fit_says_what_its_midpoints_did`
(`ssi/refine.rs`).
