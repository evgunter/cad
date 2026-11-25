---
id: sweep-reach-respells-a-loops-vertices
kind: issue
title: sweep's blend reach respells a loop's vertices, lone vertex included, by hand (Body::loop_vertices is crate-private)
status: open
opened: 2026-11-22
priority: P3
cost: E
---


Filed by `three-answers-to-is-this-loop-inside-that-one`'s lane, from
its sweep for "a loop's vertices, a lone vertex's one included".

That unit made `Body::loop_vertices` (`crates/topo/src/body.rs`) the one
home of the walk inside `topo`. `crates/sweep/src/blend/reach.rs`
(the support-vacancy read beside `point_of`, matching
`topo::LoopBoundary::Cycle` / `Empty { vertex } => vec![vertex]`)
spells the same walk through `topo`'s public API, because the method is
`pub(crate)`.

What follows: either publish the method (it reads only links) and route
the sweep site through it, or record why sweep keeps its own spelling.
The sweep could not see a spelling that walks the cycle through a
helper returning half-edges and maps `.start` across a function
boundary.
