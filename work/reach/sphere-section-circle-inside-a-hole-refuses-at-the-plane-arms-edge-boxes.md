---
id: sphere-section-circle-inside-a-hole-refuses-at-the-plane-arms-edge-boxes
kind: issue
title: A sphere whose section circle lies inside a plane face's hole refuses FallbackExtentUnsupported at the extent scan's edge-box test, the circle nowhere near the face
status: open
opened: 2026-10-03
priority: P2
cost: M
---


Found by PR 3980's dual review (`analysis/reach-dual/3980-r2`, NOTE-5);
re-measured on `reach/rest-mate-intersect-diff`. Older than that PR.

## Measured

The arc-split collar `mate2_common::collar_of(0.5, 1.5, 0, 1, 1)`
(its lower rim plane `z = 1`, an annulus `ρ ∈ [0.5, 1.5]`) against
`peg_of(0.5, 0, 0.5, 2)` with a ball of radius 0.2 centred at
`(0, 0, 1)` taken out of it (`topo::subtract`), bore × shaft walls
declared `Rest`, ε 1e-9:

- `∪` builds, two shells, 7.820471312336193 (`2π + π/2 − 4π·0.2³/3`);
- `∩`, `collar ∖ B` and `B ∖ collar` refuse
  `FallbackExtentUnsupported { what: "the sphere's section circle runs near the plane face's boundary — whole-circle membership cannot be certified from the enclosures, and no crossing layer saw an event" }`.

## Cause

`ops.rs` `sphere_extent_scan`, the plane arm: the cavity sphere crosses
the annulus's carrier in the circle `ρ = 0.2`, `z = 1`, which lies in
the annulus's hole and nowhere near the face. The arm marks a circle
"near the boundary" when its padded box overlaps any boundary edge's
box, and the box of the hole's rim circle (`ρ = 0.5`) contains the
whole section circle's box. The test is conservative by boxes where
the question is a distance: the circle and the rim are 0.3 apart.

`work/tang/a-declared-seam-subtract-and-intersect-stop-at-the-fallback-extent.md`
refuses with the same sentence where the circle does lie on the
boundary.
