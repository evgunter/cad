---
id: rim-wedge-hand-rolls-the-circle-jet-beside-the-curve3-door
kind: issue
title: rim_wedge's station closure hand-rolls the circle's point and tangent that Curve3::ders1 now answers
status: open
opened: 2026-09-15
---


## What

`crates/topo/src/boolean/rim_wedge.rs`'s `classify_shared_rim` builds
its sample stations from the `Rim`'s `center`/`axis`/`radius`/`u_ref`
by hand — `station(i)` computes `v = axis × u`, one `sin_cos`, and
returns `(center + (u·c + v·s)·radius, (v·c − u·s)·radius)`: the
circle's point and tangent at one azimuth, which is exactly the pair
`geom::Curve3::ders1` answers on the `Circle` arm (one azimuthal
frame, both fields; `crates/geom/src/curves.rs`). The rim is not
carried as a `Curve3`, so the closure is a second spelling of the
circle arm's formulas rather than a call to them.

Filed by CURVE3-JET (SCALAR), which minted the door and folded the
thirteen `eval`/`deriv` pair sites onto it: this one is not a pair
site (no `Curve3` receiver), so it was not folded. A possible
consumer: build the `Curve3::Circle` from the `Rim` fields once and
ask `ders1(theta)` per station, which puts the rim's stations on the
same bits the curve arm produces. Whether a rim should carry a
`Curve3` is BOOL's call, not the door's.
