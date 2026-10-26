---
id: a-nurbs-edges-sector-departure-is-its-chord
kind: issue
title: both sector walks take a NURBS edge's chord as its departure direction at an ON vertex
status: closed
opened: 2026-10-03
priority: P0
cost: E
refs: [3984]
closed: 2026-10-25
branch: splice/gate
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

## Moved to SPLICE (2026-10-25)

From GAUGE. Moved by `git mv` in the chat-approved refold that opened SPLICE (`work/splice/program.md`), the track for a body built by loft or sweep entering every boolean. Re-banded P3 → P0: the sector walk is one of the sites the P0 gate deletion waits on.

## Closed

Landed on `splice/gate`. `geom_brep::ConicBend::of` reads a spline as it
reads a spiric (`BendBounds::Unbounded`: no certified curvature bound,
so a tangent tie reads no side). With that, a NURBS edge takes the
curved arm of `ConicDeparture::of` (`walk_tangents`,
`walk_departure_deriv2`, `geom_brep::edge_extent`) in three walks:

- `boolean/sectors.rs` `orbit_corners`;
- `splitting/neighborhood.rs` `chord`;
- `boolean/fate.rs` `leaving`, a third hit of the same shape that the
  sweep found.

Witnesses, each red on the chord (measured by mutation):

- `boolean::sectors::spline_departure_rows::the_sector_walk_reads_a_splines_tangent`;
- `splitting::neighborhood::spline_departure_rows::the_split_walk_reads_a_splines_tangent`;
- `boolean::fate::spline_departure_rows::a_fate_cut_reads_a_splines_tangent`.

Each row reads the Bézier sheet's arc leaving `(0, 0, 0)` along
`(1, 2, 0)`, where its chord runs along `x̂`.
