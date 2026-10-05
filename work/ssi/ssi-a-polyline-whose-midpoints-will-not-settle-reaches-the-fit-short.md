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
clear of the band, the gap counts as `unsettled`. Where nothing halves,
a polyline half of one of whose gaps falls in the band refuses
sized (`ShortBranchUncertified`), and any other stops with nothing to
halve (`RefinementExhausted`/`NothingToHalve`), the lane's own limit as
its refusal: on the ℝ³ lane the slab's (`TraceUnresolved`). So
`Fit(TooFewPoints)` is unreachable from refinement. Rows:
`the_fits_minimum_halves_to_four_keeping_every_settled_midpoint` and
`a_midpoint_that_does_not_settle_is_read_by_the_transversality_at_its_chord`
(`ssi/refine.rs`).
