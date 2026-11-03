---
id: a-unit-line-reads-its-parameter-through-a-division
kind: issue
title: geom::Line::param divides by dir·dir even where dir is unit by contract, tripling an Interval parameter difference's width beside the old dot form
status: open
opened: 2026-11-03
priority: P3
cost: E
refs: [6475]
---


Found by the FULL review of PR 6475. With the normal's component boxes of width 1e-9, the width of `param(y) − param(x)` is 9.6e-8 through `/dir·dir`, against 3.2e-8 for the old `(y−p0)·d − (x−p0)·d`. No decision flipped in 668 topo rows at three ε rows. But a margin near 3e-8 decides under the old form and straddles under the new one: `shell.rs` `along`, `walls_cross` crossings, and `splitting/join.rs` sort keys. Several callers' lines are unit by contract (`TangentLocus::Line`, the `normalize()`d rulings). Decide whether a unit direction is a type (`geom::Line` over a `UnitVec3`, or a `UnitLine`) whose `param` is the dot product, or whether the division stays and the width is accepted.
