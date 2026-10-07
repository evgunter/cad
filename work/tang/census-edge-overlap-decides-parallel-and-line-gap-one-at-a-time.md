---
id: census-edge-overlap-decides-parallel-and-line-gap-one-at-a-time
kind: issue
title: the census's collinear edge overlap decides the edges' tilt and line gap one at a time, not their sum
status: open
opened: 2026-10-07
priority: P3
cost: M
---

## What

`topo::census` decides `pm_census_ee_parallel` (the edges' sine
levered at the shorter edge, `crates/topo/src/census.rs:2002`) and then
`collinear_overlap`'s `pm_census_ee_line_gap` (near 2361); both Zero
serves the collinear-overlap verdict (`UndeclaredContact`
EdgeEdgeOverlap). A detector, so the error over-reports rather than
building wrong geometry.

Found by the sweep of the TANG unit that closed
`cylinder-axis-rows-decide-tilt-and-gap-one-at-a-time`.

## The shape of a fix

Decide the served verdict on one margin carrying both terms, as the
section classifiers' `decide_across` (`crates/geom-brep/src/intersect.rs`)
and `carrier_cyl_reach` (`crates/topo/src/boolean/carrier_eq.rs`) do:
the zero side on `|datum| + tilt·lever`, a definite side on the datum
shrunk toward zero by the tilt, the tilt row kept only to route.
