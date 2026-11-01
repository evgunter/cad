---
id: the-wedge-reach-reads-each-faces-vertices-once-per-edge
kind: issue
title: The wedge reach reads every vertex of both faces once per edge: O(deg²) per face over a pass, and each door builds a fresh reader
status: open
opened: 2026-10-16
priority: P3
cost: M
---

`topo::WedgeReach` (`crates/topo/src/wedge_reach.rs`) collects each face's boundary vertices once, but `reach` still measures every one of them from each edge's middle: a face of degree `n` costs `O(n)` per edge it bounds and `O(n²)` over a tier-3 pass, and every door that certifies or re-asks an edge outside tier 3 builds a fresh reader (`topo::wedge_reach`), walking both faces again. The scenarios: an n-gon cap (one face, `n` edges, `n` vertices: `n²` distance forms per pass, each a symbolic form in the interval lanes), and a plate with many holes (every hole's edges read every hole's vertices).

A per-face maximum cannot be cached as such: the reach is measured from each edge's own middle. What would make it `O(n)` per face: a per-face bound independent of the edge (the boundary's diameter, or its two farthest vertices chosen once, which bound the reach below within a factor of two), or threading one `WedgeReach` through a door's whole pass. Either changes the lever's value or the doors' signatures, so it was not done in U-lever (PR 5204, second review F5). Measure first: `sym_9_retry_interval` and the m10 drives are where the symbolic cost shows (main 297 s against U-lever's 328 s for `sym_9`, debug, 1e-9).

