---
id: w2-reads-a-one-sheet-predicate-off-the-walk
kind: unit
title: W2 reads its own predicate off the loop walk (every loop's winding the identity, no singular interior joint) instead of chart_boundary(..).is_ok(); the cone-apex clause folds in and the sphere pole gets its twin
status: review
pr: 5657
opened: 2026-10-24
priority: P1
cost: M
branch: chart/w2-one-sheet
---


## What

`boolean::ops::ChartCache::describes` reads `chart_boundary(..).is_ok()`
(or the cone's `apex_closure_describes`) as the premise of the section
certificate W2: every closed curve in the face's interior lifts with zero
winding. One `Ok` carries two promises, W2's ("lifts to one sheet") and
clearance's ("covers the region"), and the two have different refusal sets.
Any change to `chart_boundary`'s refusals is therefore silently a change to
W2. A polar description that made a cap around a pole `Ok` would let W2
clear a latitude section inside the cap: a wrong boolean.

## Owed

- W2 gets its own predicate, read off the loop walk: every loop's winding
  is the identity deck (in `u` and `v`; `Winding::is_zero` already checks
  both), and no joint is singular in the face's INTERIOR. A singularity on
  the boundary (a sphere pole vertex, a cone apex) is outside the interior,
  which is the reading `apex_closure_describes` gives the cone today.
  The sphere pole gets the same clause, and the cone clause folds in.
- A refusal for a numerical reason stays a typed refusal, never "does
  not describe". So does a torn face: that is the passenger
  `w2-describes-folds-a-corrupt-face-into-a-miss`.

## Measure first

Whether booleans refuse today on a sphere face with a pole vertex. One
designer reads W2 as lost on every such face; the other notes that W3 and
W4 can still clear the pair. A probe: a sphere octant cut by a plane along
a latitude near the pole, checked for `Refusal::Undecided` in
`section_pairs`. **If a normal boolean on a sphere octant refuses, this row
is P0** (a normal verb broken on normal geometry). Tell the orchestrator
before building.

Ground: `crates/topo/src/boolean/ops.rs` and `section_cert.rs` are CLEAVE's,
HONE's and ORBIT's, by announced seam. The walk is in `pcurves.rs`.
