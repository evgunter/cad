---
id: a-point-touchs-kappa-is-levered-by-the-carriers-radius-not-the-cells-reach
kind: issue
title: The point-touch confirm levers κ_rel by the curved carrier's radius, so a large sphere on a small face reads a definite point touch where the carriers stay within ε over the whole cell
status: open
opened: 2026-10-13
priority: P2
design: true
refs: [point-touch-reading]
---


`census::point` (`crates/topo/src/census/point.rs`, `separates`) decides
`κ_rel` with `Margin::sagitta(κ_rel, arm)`, the arm the curved carrier's
radius (`r` for a sphere against a line or a plane and for a circle
against a plane, the smaller radius for two spheres). The carriers'
separation over the cells is `κ_rel·L²/2` with `L` how far the cell
reaches from the touch point, and `L` can be far shorter than the
radius: a sphere of radius 50 resting on a 1 mm face, or a nearly flat
circle on a short arc. There the carriers stay within ε of each other
over the whole cell, yet the lane reads a definite point touch.

The kernel's other second-order readings lever by reach:
`boolean::sectors::tangent_relative_side` reads each bound over its own
length (`BoolSector::reached_bounds`), and `geom_brep::conic_bend_side`
reads the lift at `min(max(extent, arc), lifts_to)`. Adopting that here
is not only a lever change, which is why it is a design question:

- In band at the reach lever, the lane today answers `Apart` (stale)
  for every lane but a circle in the plane through its centre. A cell
  lying within ε of the other carrier over its whole extent is not
  apart. It is a patch- or curve-shaped contact. Whether the
  point-touch record then reads confirmed (the interiors do meet at
  the point), refuses typed (a contact of the wrong dimension), or
  re-routes to the patch or curve lanes is the record's meaning under
  C3, which `[ev]` PR 4951 is settling.
- The reach must be a lower bound on how far the cell runs from the
  point (vertices and edge midpoints serve) so that a definite reading
  stays sound.

Case: `crates/sweep/tests/point_touch_records.rs`'s slab with a ball of
radius 50 at `(2, 2, 51)` on a 1 mm square face instead of the slab's
top. The reach-limited sagitta there is `0.001²/(2·50)` = 1e-8 m (10ε
at ε = 1e-9, so undecided for a band wider than that), against the
radius lever's 25 m.
