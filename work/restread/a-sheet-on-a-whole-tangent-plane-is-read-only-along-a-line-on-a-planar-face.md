---
id: a-sheet-on-a-whole-tangent-plane-is-read-only-along-a-line-on-a-planar-face
kind: issue
title: A sheet on a whole tangent plane is read only along a line on a planar face
status: open
opened: 2026-11-12
priority: P3
cost: M
---



Found by the valid-bodies lane (branch
`restread/self-touch-valid-bodies`), which reads a ridge's end on a wall
tangent to its floor. Priority P3: no measured body reaches any of
these, and each refuses `Curved(Unreadable)`, which is sound.

## What

`second_order::along_a_line` (`crates/topo/src/link/second_order.rs`)
reads a curved sheet's one wedge lying on a whole plane only where the
plane is a planar face's and exactly one end of the wedge is a line
edge. A tangent sheet on a face holding the point inside it still
refuses, as two sectors lying on each other
(`rest::sectors_material`'s `notes.coincident`,
`crates/topo/src/link.rs`), where:

- **the face is curved**: the sheet lies on a cylinder's, cone's or
  sphere's whole tangent plane (`Tangent::plane_exact` unset);
- **neither end of the wedge is a line**: a tangent sheet's corner
  resting on the face at the point alone, both its edges curves leaving
  the plane (two mouths);
- **the sheet's corner is wider than one wedge** (over half a turn):
  `second_order::pair` takes a sheet of one wedge only.

A sheet facing the same way as the face under it (their outward normals
one) is an overlap or a doubled sheet, and refuses too; that one should
stay refused or become a crossing.

## The fix

For two mouths the form is `b₀(1 − t)² + 2b₁t(1 − t) + b₂t²` with both
ends decided, which `second_order::kind` already reads for a lune with
no pinch; what is missing is the candidate. A curved face under the
sheet needs first order's tie along the line read as `on_carrier` reads
a ruling. A wide sheet needs the form read over each of its wedges.
