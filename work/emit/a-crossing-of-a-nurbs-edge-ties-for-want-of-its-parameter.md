---
id: a-crossing-of-a-nurbs-edge-ties-for-want-of-its-parameter
kind: issue
title: Crossings of a NURBS edge tie because param_along has no parameter for a NURBS carrier, where N2 ranks them by the carrier's own parameter
status: open
opened: 2026-10-01
priority: P2
cost: M
refs: [edge-pieces-are-named-by-their-ends]
---

## What

N2 ranks the crossings of one edge by one face along the crossed edge
"by its carrier's own parameter". `emit_topo::param_along`
(`crates/editor-core/src/names/emit_topo.rs:1933`) reads that parameter
through `Curve3::param_near` (line 1963), which answers `None` for a
NURBS carrier, and `emit_topo::rank_crossings` (line 1974) then mints
the crossings as N2's tie. The rule has an answer there; the code does
not compute it.

## Fix shape

Project the crossing points onto the NURBS carrier (`Curve3::project`
needs `CertifiedBounds`, which the naming scalar may not carry), seeded
at the edge's certified interval, and decide the order through
`name_frag_order_along` as for the other carriers. No fixture reaches a
NURBS edge crossed twice by one face today.
