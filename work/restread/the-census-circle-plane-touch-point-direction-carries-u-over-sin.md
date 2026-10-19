---
id: the-census-circle-plane-touch-point-direction-carries-u-over-sin
kind: issue
title: census::point circle_plane places its touch point along d/sin, whose rounding is u/sin relative: r·u/sin off the circle, unmeasured
status: open
opened: 2026-10-18
priority: P3
cost: E
---


Found by the sweep of `restread/no-shared-key-crossing` (closed forms
whose rounding is divided by an angle). Suspected, not measured.

## What

`crates/topo/src/census/point.rs` `circle_plane` takes `d = a × n × a`,
`sin = |d|`, and places the touch point at `c ∓ (d / sin)·r`. The
cancellation in `d` leaves it about `u` off in any direction, so the
direction is `u/sin` off and the point `r·u/sin`. `separates(sin/r, r)`
holds only `sin ≳ Kε/r`, so the point can be `r²·u/(Kε)` off the
circle: about 1e-5·r² at ε = 1e-12, far past ε for r ≳ 3e-4.

## Owed

Find what reads that point (`coincident`, then the touch analysis),
and whether its error can turn a verdict. If it can, charge it or
place the point from a better-conditioned form.
