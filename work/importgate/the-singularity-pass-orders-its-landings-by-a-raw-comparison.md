---
id: the-singularity-pass-orders-its-landings-by-a-raw-comparison
kind: issue
title: the singularity pass orders a meridian's landings, and a landing against the other pole, by a raw comparison of parameters
status: open
opened: 2026-11-18
priority: P3
cost: E
---


Found by the review of PR 7117 (`importgate/singularity-meet`), NOTE-6.

## What

`crates/step-import/src/normalize/singularity.rs`, `first_hit`, keeps the
nearest of the landings `geom_brep::curve_pair::meet` decides by a raw
`delta < *b` on the ray's parameter: in the `offer` closure, and again
where a singular point is weighed against the nearest boundary landing
(the singular loop, after `on_boundary` has read the two apart). `meet`
decides each edge's meetings distinct from one another (its `ringed`
rule), but nothing decides two edges' meetings apart: two edges crossing
the ray within the band of each other, with no shared vertex, are
ordered by which parameter is smaller, a choice inside the band that
`[verdicts]` says the answer may not depend on.

## Fix

Read the nearest landing's distance from each other candidate at the
pass's band before preferring it: apart, the smaller parameter wins; in
band, refuse (`SingularityUndecided`), as a landing beside a vertex does.
