---
id: the-census-edge-face-pierce-point-is-placed-from-residuals-read-off-the-face-origin
kind: issue
title: The census's edge-face pierce point r0/(r0 − r1) is placed from residuals read off the face's stored origin, uncharged: at a grazing edge its rounding is about |p − o|·u·len/|r0 − r1|
status: open
opened: 2026-10-18
priority: P2
cost: M
---


Found by the sweep of `restread/no-shared-key-crossing` (closed forms
read from an arbitrary origin).

## What

`crates/topo/src/census.rs` `pair_edge_face` reads each end's residual
as `(p − f.origin)·f.normal` and `EfPair::on_signs` places the pierce
point at `e.p0 + (p1 − e.p0)·(r0 / (r0 − r1))`, then reads `contain`
there. Each residual's `f64` rounding is about `|p − f.origin|·u`, and
`f.origin` is the plane's stored origin, which can stand far from the
edge. Where the edge grazes the plane, `|r0 − r1|` is only a few `Kε`
(decided, or `pm_census_ef_pierce_lever` under `hypothetical`), so the
point moves by about `len·|p − f.origin|·u / |r0 − r1|`: `3·3·u / 2Kε`
≈ 5e-5 at ε = 1e-12. A pierce near the face's boundary can then be read
`Out` (a missed pierce) or `In` (a false one). `p1` is also rebuilt as
`p0 + dir·len` rather than read as `e.p1`.

## Owed

Measure it first: a grazing edge through a face whose origin is metres
away, at ε = 1e-12, read at `f64` against `Interval`. If it reproduces,
read the residuals from a point of the face near the edge, and charge
the pierce point's rounding before `contain` reads it.
