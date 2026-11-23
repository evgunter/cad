---
id: the-line-wall-arm-reads-its-offset-about-a-wall-origin-far-along-the-axis
kind: issue
title: The line × wall arm reads the line's offset from the axis about the wall's stored origin, which rounds at that origin's ulp when it lies far along the axis, and the line × quadric quadratics charge their shift's rounding
status: open
opened: 2026-11-22
---


Filed by GERM's `germ/plane-pose-far-frame` lane (the plane pose's
far-frame re-pose), from its sweep for offsets formed about a stored
origin far from the geometry in the curve × face pose and doors.

## Finding

A cylinder's (and the line × quadric door's) axis origin is free along
the axis: nothing ties it to the face's stretch of the wall. The line ×
wall arm reads the line's offset from the axis about that origin:

- `curve_face::pose::line_wall_read` (`crates/geom-brep/src/curve_face/pose.rs`)
  forms `at_mid = wall.distance(line.origin + d·mid) − radius`, and
  `geom::Line::distance` / `geom::Line::param_and_offset`
  (`crates/geom/src/curves.rs`) take `p − origin` first. With the wall's
  origin a megametre along the axis from the line, each component of
  `p − origin` rounds at `ulp(1e6) ≈ 1e-10`, past the zero band at ε
  1e-12: the same far-frame cancellation the plane pose had, across the
  axis instead of across a plane.
- `curve_face::line_quadric::wall_quadratic` forms `w0p = wall.offset(q)`
  the same way, and `Centred::charged` / `Centred::disc_bound`
  (`line_quadric.rs`) charge the re-centring shift's rounding
  (`rounding_charge(far·near)`), which `[simple.conditioned]` says to
  re-pose rather than charge.

The plane's case was re-posed by `Real::difference_dot` (compensated
`(q − o)·n`, `geom::plane_offset`). The perpendicular offset
`(p − o) − a((p − o)·a)` wants the same: the error-free differences
`p − o` carried as pairs into the projection.

## Settles when

The line × wall arm's offset and the wall quadratic's `w0p` are formed
well conditioned where the arm decides, measured by a far-origin row
(a wall stored `far` along its axis from the line, ε 1e-12, read
against an exact oracle), and `Centred`'s charge is gone.
