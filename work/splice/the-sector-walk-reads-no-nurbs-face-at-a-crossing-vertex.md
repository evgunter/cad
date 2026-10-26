---
id: the-sector-walk-reads-no-nurbs-face-at-a-crossing-vertex
kind: issue
title: The boolean's sector walk has no arm for a NURBS face: a crossing vertex between two spline walls refuses CurvedBooleanUnsupported, so a union across a spline edge cannot build
status: open
opened: 2026-10-26
priority: P0
cost: M
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
from `boolean/sectors.rs`. There, the sector face resolution
(`sector_face::resolve`, its `_ => SectorFaceError::Unsupported` arm)
has no arm for a NURBS face. It is reached at a crossing vertex, which
lies on a corner edge between two spline walls of the loft, so both
faces of its sectors are splines.

## Owed

The end-to-end row the crossing rung could not write: the crossings of
each corner named through the boolean, the first and third, of one
sense, tied. Replace that tripwire with it.
