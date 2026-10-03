---
id: a-convex-graze-of-a-cone-refuses-at-some-azimuths
kind: issue
title: a convex graze of a cone or filleted corner refuses at some azimuths before rule (b) reads it
status: open
opened: 2026-10-02
priority: P3
cost: M
---


## What

A plane grazing a convex curved wall from outside lands the body
whole on its material's side (`splitting/rules.rs`, `wall_graze`).
Some poses never reach that rule: they refuse earlier, with the same
payload on main and on PR 3892's head.

Each pose below was measured with both normals:

- Narrowing frustum (radii 1 → 1/2 over height 1, revolved about y),
  azimuth θ = 0.3: `Reduce(SliverSector)` on `split_conic_departure`.
- Widening frustum (1/2 → 1), θ = 2.9: `Reduce(SliverSector)` on
  `split_conic_departure`.
- Widening frustum, θ = 1.1: `Join(UnpairedLooseEnds { count: 2 })`.
- 6 × 4 slab with r = 0.5 fillet-door corners, NE corner wall grazed
  at φ = 1.2: `Reduce(SliverSector)` on `split_conic_departure`.
- Full cone (base r = 1, apex at y = 1), grazed along a ruling through
  the apex at θ ∈ {0, 0.7, 2}: `Reduce(SliverSector)` on
  `sector_straight` at the apex vertex.

At `CAD_TOLERANCE_EPS=1e-6`, narrowing θ = 0.3 and the slab at
φ = 1.2 ANSWER with the true volumes. The refusal is a band artifact,
not geometry.

The frustum and slab poses are allowed to refuse, and nothing else
is, in `crates/sweep/tests/split_tangent_edge_curved.rs`:
`CONE_GRAZES_REFUSED` for the frusta, and the φ = 1.2 arm of
`a_convex_graze_of_a_filleted_corner_lands_the_slab_whole` for the
slab. The allowance is not an exact pin, because it varies with ε.
Fixing this row means removing the allowance. The full cone is not
pinned.

## Where to look

`split_conic_departure` is the first-order departure trilean of a rim
arc at the inserted graze vertex (`splitting/neighborhood.rs`,
`classify_neighborhood`). At those azimuths the departure margin lands
in the band rather than at exactly zero, which would hand it to the
second-order descent. That is likely rounding in the inserted root's
position and the arc tangent there; this is a hypothesis, not traced.
`sector_straight` at a cone's apex vertex is a separate door.

## Found by

CLEAVE DR-51's review of PR 3892, `review-tests/dr51` (74e151b6).
