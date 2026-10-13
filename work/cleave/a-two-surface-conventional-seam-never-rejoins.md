---
id: a-two-surface-conventional-seam-never-rejoins
kind: issue
title: edge_join never re-joins a conventional seam between two distinct surfaces, so a cut under-determined seam stays two edges at rest
status: open
opened: 2026-10-12
priority: P2
cost: E
---


## Finding

Filed by the TANG orchestrator from the five-conventions designer pair.

`topo::boolean::edge_join`'s `locus_of` (in `crates/topo/src/boolean/edge_join.rs`) admits a `Chart` edge only when
`sf == sg && chart.surface == sf`. A conventional edge between two
distinct surfaces is therefore never joinable. That is a smooth seam the
must-carry rule read as under-determined: a G2 profile join, or a
tangent piece too short for its second order. So the two halves of such
a seam cut at a vertex stay two edges at rest, against D1's maximal
edges.

`work/fuse/curved-joinable-vertices-are-left-unjoined` (closed, PR
4233) covered intersections, tangencies and one-surface `Chart` seams,
not this case.

If Ev takes decision 1 of PR 4907 (such a seam stores
`OnPair { pair }`), the fix is a same-pair `Locus` arm beside
`Intersection`'s and `TangentIntersection`'s. If not, the arm reads both
pieces' `Chart` as one locus on the pair.

