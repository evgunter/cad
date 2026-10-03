---
id: an-edge-crossing-a-cone-face-has-no-root-lane
kind: issue
title: A line, circle or ellipse edge that the enclosures cannot clear of a cone face refuses at the frontier: the cone cell has no root lane, though its quadric form is a degree-2 residual along a conic
status: open
opened: 2026-10-03
priority: P1
cost: H
refs: [ellipse-edge-crossing-a-torus-has-no-root-lane]
---


Found by the sweep of `reduce::wall_crossing`'s carrier × surface table
in the lane that gave the ellipse × torus cell its root lane
(`ellipse-edge-crossing-a-torus-has-no-root-lane`).

## Measured (by reading, not on a shape)

`reduce::wall_crossing` (`crates/topo/src/boolean/reduce.rs`) routes a
`Line` to `line_wall_roots_of`, a `Circle` to the sphere, cylinder and
torus doors, and an `Ellipse` to `ellipse_roots` on the same three
kinds. Every other cell answers `SpanVerdict::Unsettled`, so an edge
whose arc enclosure cannot clear a CONE face refuses
`CurvedPierceUnsupported` at the curved pierce arm, crossing or not.
After this lane the cone column is the whole of the table's remainder
for line and conic carriers (spiric and NURBS carriers are refused by
the operand gate first). No real shape has been measured reaching it.

## Why it is tractable

The cone's quadric form `(ρ cos α)² − (s sin α)²` (`ρ` the distance
from the axis, `s` the height above the apex) is quadratic in the
point, so along a line it is a quadratic and along a circle or an
ellipse a trigonometric polynomial of degree two — the shape
`circle_roots::certified_subdivision` already answers (and the root
slack `circle_roots::RootSlack` meters). Its zero set is the DOUBLE
cone: a root on the far nappe has to be told off by
`geom_brep::cone_elevation`'s nappe reading, and the apex, where the
quadric form's gradient vanishes, has to refuse.
