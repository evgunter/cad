---
id: the-sector-walk-reads-no-nurbs-face-at-a-crossing-vertex
kind: issue
title: The boolean's sector walk has no arm for a NURBS face: a crossing vertex between two spline walls refuses CurvedBooleanUnsupported, so a union across a spline edge cannot build
status: closed
opened: 2026-10-26
closed: 2026-10-26
priority: P0
cost: M
branch: splice/sector-walk-nurbs
---



Filed by `nurbs-edge-crossing-rung-is-the-ring-composite`'s lane, from
its end-to-end row.

## What

With the ring-composite arm in, the boolean's sweep reads a spline edge
against a plane face and splits it at every crossing. `sweep`'s
`a_face_across_a_wavy_corner` pins this on the wavy loft's corners,
three crossings each, against the face `x = 1` of a box. The union of
that loft and box (`editor-core` `emit_topo`'s
`wavy_loft::a_union_across_a_nurbs_edge_refuses_at_the_sector_walk`)
then refuses `CurvedBooleanUnsupported { operand: A, face, kind: Nurbs }`
from `boolean/sectors.rs`, where `SectorFaceError::Unsupported` is
mapped (`sectors.rs:608`). There, the sector face resolution
(`sector_face::resolve`, its `_ => SectorFaceError::Unsupported` arm)
has no arm for a NURBS face. It is reached at a crossing vertex, which
lies on a corner edge between two spline walls of the loft, so both
faces of its sectors are splines.

## Owed

The end-to-end row the crossing rung could not write: the crossings of
each corner named through the boolean, the first and third, of one
sense, tied. Replace that tripwire with it.

## The crease class

The arm reads a face's tangent or normal at the crossing vertex. It
must not read it across an interior knot of multiplicity at least the
degree (`KnotStructure::crease_runs`), and it must not read it at a
foot on another sheet. This class has been found three times: PR 5715,
PR 5764, and SELECT's plane × NURBS limbs.

## Closed (2026-10-26)

`topo::sector_face::resolve` has a spline arm
(`SectorCarrier::Spline`). It reads a `Nurbs` or `Approx` face's
outward normal at the vertex: `S_u × S_v` at the vertex's chart
parameters, with the face's sense folded once. The parameters are
the pcurve of the face's half-edge into the vertex (C4), evaluated
at the edge's parameter of the vertex. A projection foot is not used:
the pcurve is the edge's image in the face's own chart, so it is on
the face's sheet by construction.

The normal is read only where the chart has one. The arm reuses
`dihedral_side`'s two readings, now public as `geom_brep::regular_jet`
(no irregular point within the band) and `geom_brep::chart_normal_at`
(no crease run or closed seam within the band). A vertex within the
band of either refuses `SectorFaceError::Crease`. The boolean maps
that to the new `BooleanError::NormalAtSplineCrease`. A face with no
chart image of the edge refuses `CurvedBooleanUnsupported`. The
splitting lane refuses the spline carrier typed, as it does a sphere
or a torus.

Rows (`crates/topo/src/sector_face.rs`), each red on main, which
answers `Unsupported`:

- `a_spline_face_reads_its_outward_chart_normal_at_each_corner`: both
  senses, the chart and its transpose.
- `a_vertex_on_a_crease_of_the_chart_refuses_and_one_on_a_smooth_knot_reads`:
  a degree-1 knot run refuses, and the same simple knot at degree 2
  reads.
- `a_vertex_at_an_irregular_point_of_the_chart_refuses`: a bilinear
  chart whose Jacobian vanishes at one corner. A chart with a
  collapsed row (a true pole) cannot be built at rest: its loop's
  pcurves do not close (`LoopDiscontinuity` at mint).

**The end-to-end row is not written.** The union now passes the
sector walk and refuses one site on, at `vtxfac::held_reads`:
`HeldPierceUnbuilt { operand: A, placed: false }` on a spline wall of
the loft against the box's face `x = 1`. The tripwire is now
`wavy_loft::a_union_across_a_nurbs_edge_refuses_at_a_spline_walls_held_pierce`.
The owed row moves to
`a-spline-wall-cannot-say-whether-it-holds-a-pierce-point`.

