---
id: a-sphere-cut-in-refuses-a-sphere-face-with-a-boundary-edge-that-is-not-a-circle
kind: issue
title: ops::apply_cut_ins refuses a sphere face whose boundary holds an edge that is not a circle, so a sphere cut by a cylinder or cone refuses its cut-in
status: open
opened: 2026-10-28
priority: P1
cost: M
---


Found on review of PR 5940 (`splitdoor/tilted-plug-rosters`), a
lines-and-circles roster its sweep missed: the sweep grepped `match`
arms on `Curve3::Line`/`Circle` and one-line `let … else` on them, and
this site is a `let geom::Curve3::Circle { .. } = … else` spread over
several lines in `crates/topo/src/boolean/ops.rs`.

## What

`ops::apply_cut_ins` cuts a sphere face along a meridian half-plane of
its chart, reading every boundary arc's crossings with the meridian's
plane through `circle_roots::first_harmonic_roots`
(`FirstHarmonic::circle_off_plane`). A boundary edge whose carrier is
not a circle refuses `FallbackExtentUnsupported { what: "a boundary
edge of the sphere face is not a circle" }`. A plane meets a sphere in
a circle, but a cylinder, cone or torus does not: a sphere face trimmed
by a bore (a ball with a hole drilled through it) holds a quartic edge,
and any cut-in on that face refuses. Not driven to a body here.

## Direction

The cut needs each boundary arc's crossings with one plane. For a
conic, that is the conic–plane meeting `splitting::plane_crossing_lane`
already reads (`ConicPlaneMeet`); for a spline or spiric, the crossing
layer's own plane rung. Build the fixture first (a sphere face bounded
by a cylinder section, taking a cut-in), then route the roots by kind.
