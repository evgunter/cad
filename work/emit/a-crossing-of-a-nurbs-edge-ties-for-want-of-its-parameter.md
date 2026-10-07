---
id: a-crossing-of-a-nurbs-edge-ties-for-want-of-its-parameter
kind: issue
title: Crossings of a NURBS edge tie because param_along has no parameter for a NURBS carrier, where N2 ranks them by the carrier's own parameter
status: review
branch: emit/nurbs-crossing-param
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

## After PR 4134 (2026-10-06)

Ev's ruling on PR 4134 names every crossing by its sense. Only crossings
of one edge by one face with the same sense are still ranked along the
edge (`OrderAlong`), and a vertex where two edges cross carries each
edge's sense against the other operand's closed body. Once the sense is
built (`a-second-crossing-by-one-face-renames-the-first-and-its-pieces`,
branch `emit/crossing-sense`), re-measure this row against the narrower
case that is left.

## Built (emit lane, 2026-10-07, branch `emit/nurbs-crossing-param`)

Re-measured after the sense landed: the case left is two crossings of
one NURBS piece by one face with one sense. No body the kernel accepts
reaches it. The split refuses a plane that may meet a NURBS edge
(`topo::splitting::classify::carrier_gate`, `CurvedEdgeUnsupported`),
and the boolean refuses any operand with one (`gate_operand_edges`). So
the rows are unit rows on `rank_crossings`, with real pieces: the
corner edges of a wavy loft.

`emit_topo::chord_along` reads a crossing on a NURBS piece along the
piece's chord, in meters. It does so only when every control point
shaping the piece's certified interval advances strictly along that
chord, each step decided positive through `name_frag_order_along`. The
weights are positive, so knot insertion keeps that polygon advancing,
and variation diminishing makes the piece a graph over its chord. The
readings then order as the parameters do. A difference the band
decides is the order of the feet, wherever the crossing points sit
within tolerance of the curve. The check is sufficient, not necessary.
A piece it cannot certify keeps N2's tie, and so does a pair the band
cannot part.

`rank_crossings` keeps the span order across pieces and uses the chord
reading for two crossings on one piece.
`discriminate::order_along` split into `extent_before` (one pair) and
`rank_by` (the ranks from any pairwise order), so both readings go
through one decision. Neither carries `CertifiedBounds`, which the
naming scalar lacks. `Curve3::project` was not used: its foot
parameter is an uncertified `f64`, and bounding the parameter error
from its residuals would need a speed bound and a uniqueness argument
of the same kind.

Rows (`emit_topo`'s `nurbs_crossings_rank_by_parameter`, at ε 1e-6,
1e-9 and 1e-12): the first and third of three same-sense crossings of
a plane rank by parameter, whichever order they are handed in. Two
crossings a quarter of the band apart tie, and so does one point
handed in twice.

Found on the way and filed:
`work/nurbs/a-swaying-loft-corner-refuses-as-a-vanishing-span.md`.
