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
by hand — `station(i)` computes `v = axis × u`, takes `theta.sin()` and
`theta.cos()` as two separate calls, and returns
`(center + (u·c + v·s)·radius, (v·c − u·s)·radius)`: the circle's point
and tangent at one azimuth, which is what `geom::Curve3::ders1`
answers on the `Circle` arm (one azimuthal frame from one `sin_cos`,
both fields; `crates/geom/src/curves.rs`). The rim is not carried as a
`Curve3`, so the closure is a second spelling of the circle arm's
formulas rather than a call to them.

**A fold moves bits.** The two spellings are not the same program: the
arm's tangent is `(u·(−s) + v·c)·radius` from `sin_cos`, the closure's
is `(v·c − u·s)·radius` from separate `sin` and `cos`, and the point
halves associate the frame sum the same way but from different trig
results. Folding the stations onto `ders1` therefore RE-BASELINES
every rim station the classifier samples — the same points and
tangents up to rounding, on different last bits — and any digest or
stored bit that reads a rim verdict through a station moves with it.
That is a PIN re-baseline to take deliberately, not a free tidy: say
what moved and why, as `docs/prompts/implementer-discipline.md` §3
asks.

Filed by CURVE3-JET (SCALAR), which minted the door and folded the
`eval`/`deriv` pair sites onto it: this one is not a pair site (no
`Curve3` receiver), so it was not folded. A possible consumer: build
the `Curve3::Circle` from the `Rim` fields once and ask `ders1(theta)`
per station, which puts the rim's stations on the bits the curve arm
produces. Whether a rim should carry a `Curve3` is PIN's call, not the
door's.
