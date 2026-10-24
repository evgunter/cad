---
id: the-sphere-ray-and-the-circle-leg-stand-outside-the-region-skeleton
kind: issue
title: The sphere ray and the circle leg stand outside the region skeleton
status: open
priority: P1
cost: M
refs: [the-region-doors-share-one-skeleton-three-times, sphere-region-refuses-null-scaffolding-the-torus-and-ruled-doors-step-over]
opened: 2026-10-24
---

## What

PR 5671 moved the sphere, torus and ruled region doors onto one skeleton
in `crates/topo/src/boolean/region.rs`. Its FULL review (CHART
orchestrator, 2026-10-24) found five pieces still outside it:

- **(a) The sphere's own ray loop.** `SphereFaceRegion::ray` reads each
  root's span and places that root along the ray before it reads the
  next root. `region::offer` reads all of an edge's landings
  (`Paths::landings`) before it places any. Moving the sphere onto
  `offer` can change a REFUSAL, though never an In/Out verdict: the
  first kept fault can become `Escalated(span)` where it was
  `RayExhausted`. So folding it in is a re-baseline, not a refactor.
- **(b) `Paths` fuses the roster with the leg reading.** The sphere
  casts rays over a direction schedule, not a path roster, so it cannot
  use `region::read_leg`. A `Legs` trait under `Paths` (the landings,
  place, heading, outward and lever) would let it.
- **(c) "Place a crossing on a circle leg" is written three times:**
  - the torus door's `place` (angle, `bool_*_at` apart, `turn`, the
    corner check);
  - the ruled `Leg::place`'s `Parallel` arm;
  - the sphere ray's place.

  `Round` doesn't carry it.
- **(d) The two `along`s, and the leg's end under three names.** There
  is `along` in `torus_region.rs` and another in `ruled_region.rs`. The
  leg's end is spelled `corner` on a torus leg and `end` on a ruled leg,
  and `Paths::aimed`'s returned offset is its place along the path.
- **(e) Three spellings of a restricted `Curve3`:** the torus `Carrier`
  enum, the ruled `Reading` enum, and the sphere's hand-written
  `Circle::at`.

## Owed

Fold (a) to (e) into the skeleton. For (a), state what moves and
re-baseline it: the refusal a sphere path is set aside with, and the
sphere's K-stats decision order. PR 5671's differential recorder (each
`contains` wrapped in `k_stats::detached` and logged with its full
decision sequence) measures exactly that.

