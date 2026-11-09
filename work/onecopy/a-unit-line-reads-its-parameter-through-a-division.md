---
id: a-unit-line-reads-its-parameter-through-a-division
kind: issue
title: geom::Line::param divides by dir·dir even where dir is unit by contract, tripling an Interval parameter difference's width beside the old dot form
status: closed
opened: 2026-11-03
priority: P3
cost: E
refs: [6475]
closed: 2026-11-09
pr: 6594
---


Found by the FULL review of PR 6475. With the normal's component boxes of width 1e-9, the width of `param(y) − param(x)` is 9.6e-8 through `/dir·dir`, against 3.2e-8 for the old `(y−p0)·d − (x−p0)·d`. No decision flipped in 668 topo rows at three ε rows. But a margin near 3e-8 decides under the old form and straddles under the new one: `shell.rs` `along`, `walls_cross` crossings, and `splitting/join.rs` sort keys. Several callers' lines are unit by contract (`TangentLocus::Line`, the `normalize()`d rulings). Decide whether a unit direction is a type (`geom::Line` over a `UnitVec3`, or a `UnitLine`) whose `param` is the dot product, or whether the division stays and the width is accepted.

## Closed (`onecopy/planar-foot`)

Re-read under PR 6568's ruling A′. **`Line::param` keeps its division by `|dir|²`.** It is the parameterization, which must be correct for any nonzero `dir`: chords `b − a`, and `Curve3::Line` carriers minted unnormalized. A unit read, `param` as the bare dot, would be wrong by about `δ·|w|` on a direction off unit by `δ`. A frame axis is unit only to the convention's `ε/arm`, and a line `dir` is not checked at all. The width is the price of that correctness. The typed unit axis (`an-axis-reads-its-direction-as-unit-by-convention-not-type`) is where a unit read could return.

**The row's sites read the parameter as a length**, so they now read `Line::height` (`w·dir / |dir|`), the metric reading:
- `shell.rs` `MovedWall::walls_cross`'s `along`;
- the moved-wall ball read in `cut_counting`, now `height_and_distance`;
- `splitting/join.rs` `pair_along_line`'s sort keys.

`sort_along_line`'s keys are metres, and it decides the order of two keys against the band, so the keys' values matter, not only their order. On one line `height` and `param` order alike, and `height` is the one in metres for any `dir`.

Measured width of the key difference `k(y) − k(x)`: 2000 random tilted directions normalized at `Interval` with component boxes of half-width `r`, origins to ±100 m, points within 1 m (a scratch probe, not committed).

| `r` | `param` (before) | `height` (after) | bare dot (the old form) |
| --- | --- | --- | --- |
| 0 | 5.5e-15 | 4.5e-15 | 2.6e-15 |
| 1e-12 | 1.19e-11 | 8.9e-12 | 6.0e-12 |
| 1e-9 | 1.19e-8 | 8.9e-9 | 6.0e-9 |

So `height` recovers about a quarter of the width `param` cost against the bare dot. The rest is the division that keeps it correct off unit.
