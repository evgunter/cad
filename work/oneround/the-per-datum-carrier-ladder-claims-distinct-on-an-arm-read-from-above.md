---
id: the-per-datum-carrier-ladder-claims-distinct-on-an-arm-read-from-above
kind: issue
title: the per-datum carrier ladder claims Distinct on an arm read from above
status: open
opened: 2026-11-21
priority: P3
cost: M
---


Found by the review of PR 7219 (`oneround/lever-sites-brep`, its note
n7, by inspection), which that PR does not introduce and makes less so.

## What

`geom_brep::carrier_eq`'s ladder (`ladder_reading`, through `cone_data`
and `cylinder_datum` into `data_rungs`) levers each angular datum at
`at_consumed_extent`'s arm and acts on every sign: a datum decided apart
is `Distinct`, and all of them Zero goes on to the sum
(`coincident_over_extent_as`). The arm is the extent ball's reach from a
pivot (`ExtentBall::lever_from`, `lever_from_either`), a bound from
ABOVE: a ball says nothing about where the consumed region reaches.

`crates/geom-core/README.md`, "Float rounding": a verdict that claims a
definite sign reads the extent from below, and one that acts on every
sign reads both and acts only where they agree. So the `Distinct` arm is
decided on the wrong bound. An axis tilt that the true consumed region
leaves in the band can be read definite over the ball's far side, and
two carriers that are one over the region read apart.

`topo::carrier_pair_arm` reads the same arm for
`editor_core::coincide::replay`; the review did not trace that use.

## What it costs today

Not measured to cost a verdict. PR 7219 shortened the cone arm (the
reach from the farther apex, where it was the first apex's reach plus the
apexes' separation), which over-claims `Distinct` less than before.

## The fix

The ladder reads `LeverBounds`: the extent's reach from below for the
definite arm and from above for the Zero, as `edge_reach_bounds` gives
an edge's. An `ExtentBall` alone holds no bound from below, so the
ladder's callers hand the region (a cover or a `Reach`), not a ball.
