---
id: arc-fillet-corner-circle-pair-cancels-at-unequal-radii
kind: issue
title: An arc-leg fillet corner between two circular carriers is placed by differences of squares, so at radii orders of magnitude apart it sits off the smaller carrier or is lost to a negative radicand
status: open
opened: 2026-10-14
priority: P2
cost: E
---



Found by the sweep of `paths/circle-pair-shared-home` (the unit that
moved the circle-pair algebra to `geom_core::Circles`). #5032's sweep
missed it: its spelling is `d_sq + r1_sq - r2_sq` over squared radii.

## The shape

`crates/profile/src/path/arc_fillet.rs` `circle_circle`, the corner of
two circular fillet sides (`derive`, `SideCarrier::Circle` ×
`SideCarrier::Circle`), takes the squared radii and places the corner
at `k = (d² + r₁² − r₂²)/2d²` along `c₁ → c₂` and `h = √(r₁²/d² − k²)`
across it: the textbook radical form, with the same cancellation as
the rows `arc-arc-secant-arm-misses-crossings-at-unequal-radii` fixed.
Its three margins (`d`, `r₁ + r₂ − d`, `d − |r₁ − r₂|`) are a third
spelling of the circle-pair margins (noted in
`check-9-arm-5-is-a-second-copy-of-seg-planar-edge-pair-intersector`).

## Measured

The same IEEE arithmetic in Python, the corners' distance from both
circles against an 80-digit residual, in ulps of the largest radius or
centre coordinate:

| pair | larger first | smaller first |
|---|---|---|
| r = 80.44 × 0.0317 (the profile issue's second pair) | 133 ulps (2.4e-12) | 0.44 |
| r = 1e3 × 1e-3, d = 1e3 + 1e-4 | 2.2e5 ulps (4.9e-8, 49ε at 1e-9) | 0.03 |
| r = 62.76 × 1.28e-7 (the first pair) | NaN: `r₁²/d² − k²` rounds negative | 0.25 |

Whether a fillet whose two sides are arcs this unequal reaches the
corner (rather than refusing earlier) was not measured; the NaN row
poisons the gates after it, so it refuses rather than misplaces.

## Where to look

`geom_core::Circles2` (`crates/geom-core/src/arc/pair.rs`): build it
from the two centres and the radii, decide the three margins from its
`identity`/`external`/`internal` (the concentric `d` gate is its own),
and take the corners from `crossings()`. The squared radii the caller
passes are `(anchor − c)·(anchor − c)`; the radii are their roots,
which this function already takes for the margins.
