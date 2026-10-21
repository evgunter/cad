---
id: the-census-circle-plane-touch-point-direction-carries-u-over-sin
kind: issue
title: census::point circle_plane places its touch point along d/sin, whose rounding is u/sin relative: r·u/sin off the circle, unmeasured
status: closed
opened: 2026-10-18
priority: P3
cost: E
closed: 2026-10-20
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

## Closed

Measured: over random unit pairs `3e-11` to `2e-10` rad off parallel,
the direction `d/sin` from `(a × n) × a` was off by up to `0.48·u/sin`,
so the touch point moved up to `2.7e-6·r` along the circle at the
separation's threshold (`sin = 2Kε/r`, ε = 1e-12). One stored pose, the
plane's normal and the circle's axis off every coordinate axis and
`3e-11` rad apart, put it 1.8e-6 off. The point is read by
`curved::on_curved_interior` (the arc's span) and `face_holds` (the
face's region), so an arc end or a face boundary within that distance
read on the wrong side. With both vectors along coordinate axes the
products are exact, which is why the existing rows never showed it.

`plane_across` now forms `a × (n − a)` (or `a × (n + a)` with the
circle facing away, chosen by `select_le_zero` on `a·n`), equal to
`a × n` in reals. `n ∓ a` is formed to `u` of its own size, so nothing
cancels and the direction is off by at most 3.3u anywhere (20 000 pairs
from parallel to right). Pinned by
`census::point::tests::a_circle_nearly_parallel_to_a_plane_is_touched_where_it_touches`
(the 1.8e-6 pose read to within 1e-15 of its exact touch direction).
