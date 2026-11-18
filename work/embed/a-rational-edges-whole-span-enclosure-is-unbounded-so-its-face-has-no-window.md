---
id: a-rational-edges-whole-span-enclosure-is-unbounded-so-its-face-has-no-window
kind: issue
title: A rational edge's one-interval enclosure over its whole span is unbounded, so its face's window is not a finite rectangle and two valid f64 cone cavities refuse before any cell is read
status: open
opened: 2026-11-18
priority: P0
cost: M
---

Traced by lane `embed/cavity-trace` (2026-11-18) for
`the-f64-cavity-gate-refuses-thirty-two-valid-shells`. Main `da49c63f`,
f64, the cavity asked through a local, uncommitted `CavityGate` switch,
with `topo::clearance` instrumented (uncommitted).

## Evidence

Two valid f64 cavity rows refuse `Unsupported { carrier: "a face whose
carrier window is not a finite rectangle", face: 1v1 }` before any cell
pair is read (an empty receipt):
- `sf2b_r2_probes::r2_a_conical_wedge_hollows_with_hyperbolic_meridian_edges`;
- `shell7_seam_corner::a_three_quarter_turn_cone_frustum_hollows`.

Both are cheap (0.2 s). They have refused this way since 2026-11-13.

## Trace

Face `1v1` is in each case a plane cap (`x = 0.0078125` and
`x = −0.05`) with an ordinary frame. Its window is
`(−∞, ∞) × (−∞, ∞)`, because `window_of` reads it off `boundary_of`'s
box, and that box is all of space.
- **The edge that makes it so** is `8v1`, the cap's hyperbola: the cut
  of the cone by a plane off its axis. It is stored as a rational
  quadratic NURBS, weights `[1, 1.0722, 1]` and `[1, 1.0638, 1]`.
- **`boundary_of` encloses each edge by one interval evaluation** of its
  carrier over the whole span (`carrier.eval([t0, t1])`). For a rational
  curve that evaluation comes out unbounded. The likely cause, not
  traced into the evaluator, is that the weighted basis sum's interval
  reaches zero over `[0, 1]`.
- **Every other point of the face is finite**: its vertices and its
  other edges.

**The missing reading.** A bounded enclosure of a rational edge over
its span. With positive weights, the curve lies in its control points'
convex hull, which is finite. Subdividing the span until the
denominator is decided positive would also serve. Either keeps a curved
boundary bounding its own face's window, which is what the whole-span
evaluation is for.

**Not checked:** whether this is the shape behind the at-rest gate
lane's census of 59 workspace tests refusing the same text. Any face
bounded by a rational edge whose whole-span denominator interval reaches
zero would refuse so. The row named here is the cheapest evidence.
