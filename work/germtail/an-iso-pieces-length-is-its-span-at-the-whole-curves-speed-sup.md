---
id: an-iso-pieces-length-is-its-span-at-the-whole-curves-speed-sup
kind: issue
title: A ring path's iso piece is metered at its span times the whole curve's speed sup
status: open
opened: 2026-11-16
priority: P4
cost: E
---


Filed by the `sectarms/planar-spline-winding` fix pass (PR 7028), from
its sweep for the class it fixed in the planar winding sum. Not traced
further than the site.

## What

`crates/topo/src/ring_path.rs` meters a path's `Piece::Iso` at
`Margin::metered_sup(p.span.1 - p.span.0, p.curve.speed_upper_bound())`:
its parameter span times the speed sup of the whole curve. Where the
piece covers part of the curve, a faster stretch outside the piece
lengthens it. The same shape in the planar winding sum read 85× to
3100× an edge's length and decided real regions `Zero`
(`work/spiricarm/quad-lanes-length-bound-charges-an-edge-for-its-whole-carriers-fastest-stretch.md`
has the measurements).

Not established: whether an iso piece ever covers less than its curve,
and which way the reader takes an over-long piece.

## Done when

The piece is metered over its own span
(`geom::NurbsCurve3::length_upper_bound`), or it is shown that the
piece is always the whole curve or that the reading is one-sided, and
the comment says so.
