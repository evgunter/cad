---
id: the-trimmed-lanes-zero-area-gate-reads-a-float-shoelace-not-the-boundary-parity
kind: issue
title: mesh: the trimmed lane refuses a loop of no area on an exact-zero shoelace sum, a float area, where the planar lane now refuses on the boundary's parity
status: open
opened: 2026-11-20
priority: P3
cost: E
---

Found by `tesslie/empty-patch`'s measurement of
`trimmed-and-planar-lanes-answer-ok-on-an-empty-patch`, 2026-11-20.

## Finding

`mesh::trimmed::tessellate_trimmed` refuses a trim polygon of no area in
`trim_orientation`: `PolygonOrientation::of_signed_area(shoelace2(..))` is
`None` on an EXACTLY zero sum, raised as `MissingEntity { what: "degenerate
trimmed boundary" }`. That is the gate a zero-area loop on a spline chart meets
today. The row's measurement shows it: a lamina of two coincident arcs on a flat
NURBS patch refuses there at every δ tried
(`crates/mesh/tests/faces_that_bound_no_region.rs`,
`a_spline_lamina_refuses_at_the_zero_area_gate`).

Two things are wrong with it, by TESS-1's rule that a refusal is not decided on
a consequence read from floats:

- **It decides on an area.** An exact zero is what a bitwise retrace yields. A
  loop whose two traversals differ by rounding sums to noise, and goes past the
  gate. Measured: an arc pair whose second carrier is the same circle re-framed
  went on to refuse `PinchWedge`, `Triangulation` or `UnresolvedWinding`,
  depending on δ. Those are typed refusals, but each names another cause.
- **It names the wrong variant.** `MissingEntity` renders as a key that failed to
  resolve (`tessellate-missing-entity-renders-every-detail-as-a-key-that-failed-to-resolve`).

The planar lane now refuses the same state on the structural fact
(`TessellateError::BoundaryBoundsNoRegion`, `planar::triangulate_chart`): no
realised boundary side is traversed an odd number of times. The trimmed lane
builds the same `Crossings` map in `constrain_boundary`, so the same check is
available to it after the constraint pass.

## What closing it takes

Decide whether the trimmed lane refuses on the parity check after
`constrain_boundary`, as the planar lane does, and what then remains for the
shoelace zero arm. That arm also serves orientation, so it cannot simply go.
No body has been found that reaches `Ok(empty)` on this lane: the measurement
behind the parent row tried a spline rim-only cap, which refuses `DomainUnread`
(`OuterOpen`), and spline laminae, which refuse as above. This row is about the
gate's terms, not a hole.
