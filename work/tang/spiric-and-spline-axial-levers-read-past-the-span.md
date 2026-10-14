---
id: spiric-and-spline-axial-levers-read-past-the-span
kind: issue
title: a spiric edge's axial lever is its torus's support and a spline's its whole control net, past the span the edge holds
status: open
opened: 2026-10-07
priority: P4
cost: M
---


Found by PR 4255's first full review (M2), which made the conic arcs'
axial lever exact over their span.

## What

`geom_brep::Reach::range_along` (`crates/geom-brep/src/extent.rs`)
reads a `Reach::Span` of a conic exactly over `[t0, t1]`
(`conic_arc_range`), but:

- a **spiric** span is read at its torus's support along the axis,
  `|(c − pivot)·a| + R·|a × k| + r`, whatever span of the oval the edge
  holds and whichever oval it is;
- a **NURBS** span is read at the whole carrier's control net, not the
  span's.

Both are sound (never short of the span), but an edge holding a short
piece of either is levered past the face it bounds, and on a two-sided
served row (`pc_axis_plane_parallel`, through
`splitting::rules::face_axial_range`) an over-long lever escalates a
tilt that is Zero at the face's real extent, and serves a conic for one
that is in the band there. No
fixture builds a cylinder wall bounded by such an edge today.

## The shape of a fix

- NURBS: the control net of the span alone, split at `t0` and `t1`
  (`NurbsCurve3::split_at`), with the split read outward (an enclosure
  of the parameter, never a rounded-in one).
- Spiric: the oval's own support over the span, if a closed form or a
  cheap sound bound exists (the axial coordinate is
  `(u·a)·offset + (m·a)·√((R + r cos v)² − offset²) + (k·a)·r·sin v`).
