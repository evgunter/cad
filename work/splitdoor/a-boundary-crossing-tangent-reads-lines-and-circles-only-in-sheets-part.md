---
id: a-boundary-crossing-tangent-reads-lines-and-circles-only-in-sheets-part
kind: issue
title: reduce::sheets_part reads a boundary edge's tangent for lines and circles only, so a touch on an ellipse boundary reads as not parted
status: dispatched
opened: 2026-10-28
priority: P1
cost: M
---


Found by `splitdoor/tilted-plug-rosters`'s sweep for lines-and-circles
rosters in `boolean/`, `splitting/` and `chord_join.rs`.

## What

`crates/topo/src/boolean/reduce.rs` `sheets_part` decides whether two
tangent sheets meeting at a crossing on `face`'s boundary edge part
there (`bool_touch_sheets_part`). It reads the boundary edge's tangent
at the crossing `p` as `dir` for a line and `axis × (p − center)` for a
circle, and returns `false` (with nothing read) for any other carrier:
an ellipse boundary reads as "not parted", the same as a boundary edge
running on through both interiors. Its doc names the gap ("a boundary
carrier other than a line or a circle").

## Direction

An ellipse's tangent at an on-curve point is the same closed form:
with `(x, y)` its in-plane coordinates along `u_ref` and
`axis × u_ref`, `axis × (u_ref·x/a² + v_ref·y/b²)` (a circle's is this
with `a = b`). Or read the tangent off the carrier's derivative at the
crossing's parameter, as the x side already does
(`curve.carrier().deriv(t)`), once the crossing's parameter on the
boundary edge is in hand. No fixture reaches it yet: build one where an
edge of one solid lies tangent along an ellipse-bounded face of the
other (a tilted rod's rim touching a plate's face), then fix.

A second fixture idea (from `splitdoor/two-rims-coplanar`'s sweep, which found the same roster): two rods of different radii, tangent inside one another and both cut by one slanted plane, put an elliptical rim tangent to the other's elliptical rim on their tangent walls.
