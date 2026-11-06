---
id: a-knot-vectors-breakpoints-are-collected-four-times-with-no-door
kind: issue
title: A knot vector's distinct breakpoints are collected by hand at four sites, with no Knots::breakpoints door
status: open
opened: 2026-11-04
priority: P3
cost: E
---

Filed by SPLICE's `splice/rim-arc-on-cylinder` (PR 6481), from its review.

`Knots::runs()` yields `(breakpoint, multiplicity)` pairs, and four sites
drop the multiplicity by hand to get the distinct breakpoints, ascending,
clamps included:

- `crates/topo/src/boolean/reduce.rs:4218` (`spline_off_carrier`'s Bézier
  pieces);
- `crates/geom-core/src/spline/knot_values.rs:469` (`Knots::crease_values`);
- `crates/geom-brep/src/edge_nurbs.rs:2087`;
- `crates/geom-brep/src/pair_section/winds.rs:483`.

A `Knots::breakpoints()` beside `runs()` and `crease_values()` would be
the one statement of where a spline's Bézier pieces begin and end, and
the four would read it. No site is wrong today. The risk is a fifth site
that splits at `values()` (every knot, repeated) or at uniform cells, and
so misses or doubles a piece boundary: that is the hole
`spline_off_carrier`'s piece split had to be pinned against.
