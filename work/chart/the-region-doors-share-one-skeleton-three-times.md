---
id: the-region-doors-share-one-skeleton-three-times
kind: issue
title: The sphere, torus and ruled region doors each carry their own copy of one skeleton: the boundary gather, the path roster and the leg helpers
<<<<<<< HEAD
status: dispatched
=======
status: review
branch: chart/region-skeleton
pr: 5671
>>>>>>> origin/main
priority: P1
cost: M
refs: [curved-face-containment-answers-none-on-a-ringed-face]
opened: 2026-10-24
branch: chart/region-skeleton
---


## What

`topo/src/boolean/region.rs` shares only the leaves of the three region
doors: `Hit`, `span`, `in_band`, `TARGET_SHARES`, `circle_edge_hit`,
`side_at_closest`, `turn` and `found`. The skeleton above the leaves is
written three times, nearly line for line:

- **The boundary gather.** Each door walks the outer loop and the rings,
  skips null scaffolding, refuses an uncertified edge and leaves out an
  edge the loop runs both ways (a seam):
  - `sphere_region.rs:194-248` (`sphere_face_region`; seam filter
    `:238-240`);
  - `torus_region.rs:364-466` (`torus_face_region`; seam filter
    `:447-455`);
  - `ruled_region.rs:500-597` (`ruled_face_region`; seam filter
    `:574-582`).
- **The path roster and its walk.** The single legs, the aimed paths
  over `TARGET_SHARES` targets and `ray_walk::walk` over them:
  `torus_region.rs:329-362` and `:477-516` against
  `ruled_region.rs:453-475` and `:621-685`.
- **The legs.** `Leg`, its `angle` and `heading`, and the door's
  `parallel` and `polar`: `torus_region.rs:298-328` and `:564-606`
  against `ruled_region.rs:348-452` and `:804-857`.
- **The boundary pre-pass and the crossing offer.** `on_boundary` and
  `leg_crossings`: `torus_region.rs:517-563` and `:677-723` against
  `ruled_region.rs:779-803` and `:975-1011`.

The line numbers are at PR 5664's head.

Both of PR 5664's reviewers found this independently. One noted that
`sphere_region` belongs in the same pass.

## Why it matters

CHART's next unit reads these doors. A fix to one copy has to be found
and made in the others by hand: the seam filter, for one, went
unexercised in `ruled_region` until PR 5664's review added a row.

## Owed

One skeleton in `region.rs` (the gather, the walk over a path roster, the
crossing offer), with each door supplying only its surface's legs and
landings.

## Closed

Closed 2026-10-24 by PR 5671 (FULL review: an independent differential of about 2.27M records was byte-identical against base). The remaining duplication is `the-sphere-ray-and-the-circle-leg-stand-outside-the-region-skeleton`. (CHART orchestrator)
