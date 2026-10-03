---
id: a-notched-full-turn-wall-has-no-ray-trim
kind: issue
title: point_in_solid refuses at bool_wall_trim_period on a full-turn cylinder wall notched by a cut: the ray trim's cosine window needs a sub-period face
status: open
opened: 2026-10-03
priority: P2
cost: M
---


Found by REACH's `reach/arc-from-pairing` lane, whose change lets the
pose below build.

## Measured

`crates/sweep/tests/germ_coplanar_conic.rs`, the fixture "tube strut
arc in the box top": the tube `0.5 ≤ ρ ≤ 1`, `y ∈ [−1, 1]` about `y`,
its outer wall two full-turn faces meeting in the circle `ρ = 1`,
`y = 0`, against the box `(−0.3, 0.3) × (−2, 0) × (0.8, 1.3)`. ∪ and
∖ build to the closed form (`1.5π + 0.6 − overlap`, `1.5π − overlap`),
all three tiers clean. `topo::point_in_solid` on the result, at the
fixture's witnesses `(0, ±0.5, 0.9)`, refuses

```
Escalated { face: FaceKey(8v5), diag: { margin: Invalid,
  predicate: "bool_wall_trim_period" } }
```

`solid_contain`'s wall outline (`narrower_than_period`) needs the
face's azimuth window narrower than a period for its cosine
construction, and face `8v5` is the lower outer wall, a full turn with
the box's notch cut from its top rim: its window is a full period
although the face is not the whole band.

## What a fix owes

A ray trim for a full-turn wall face that is not a plain band (the
notch's own outline read against the height window), with the
fixture's witnesses agreeing on set membership.
