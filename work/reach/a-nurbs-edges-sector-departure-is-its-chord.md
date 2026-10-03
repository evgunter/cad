---
id: a-nurbs-edges-sector-departure-is-its-chord
kind: issue
title: the split's sector walk takes a NURBS edge's chord as its departure direction at an ON vertex
status: open
opened: 2026-10-03
priority: P3
cost: E
refs: [planar-crossing-lane-reads-a-curved-carrier-as-a-line]
---


Filed from the sweep of `planar-crossing-lane-reads-a-curved-carrier-as-a-line`
(the "curved carrier read as a line" class).

## Finding

Both vertex-neighbourhood walks take a NURBS edge's departure direction
at a vertex as its CHORD `p_end − p_base`, as for a line, rather than
its tangent:

- `crates/topo/src/splitting/neighborhood.rs` `chord`: the arm
  `Curve3::Line { .. } | Curve3::Nurbs(_)` returns the chord and no jet;
- `crates/topo/src/boolean/sectors.rs` `build_sectors`'s `chord`
  closure: the `Curve3::Nurbs(_)` arm returns the chord with
  `Reach::Extent(|chord|)`.

A spiric already reads `walk_tangents`, and so could a NURBS. For a
curved spline the chord is not the departure direction, so the sector
classification at an ON vertex can be wrong silently.

**Latent, not live.** Both run only at ON vertices: the boolean gate
(`gate_operand_edges`) refuses every NURBS edge first, and the split
gate (`splitting::classify::gate_operand`) admits a NURBS edge only
when `edge_clears`, which a NURBS edge with an end ON the plane cannot
pass (its only box is its faces', which hold that end). The one way in
is a direct caller of the `pub` `splitting::classify_neighborhood`,
which does not run the gate. Becomes live the day either gate narrows.

## Fix

Give the NURBS arm the spiric's reading (`walk_tangents`,
`walk_departure_deriv2`, `geom_brep::edge_extent`) at both sites, with
a row that red on the chord.

## The boolean half is typed (2026-10-03)

`boolean/sectors.rs`'s NURBS arm now refuses
`EdgeCarrierUnsupported { site: VertexSector }`
(`planar-crossing-lane-reads-a-curved-carrier-as-a-line`, which deleted
the boolean gate). The split's `neighborhood.rs` `chord` is what
remains, behind the split gate.
