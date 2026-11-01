---
id: a-conic-on-a-cone-face-is-read-at-five-samples-unbounded-between
kind: issue
title: A conic is read on a cone face at five samples, with no bound on its offset between them
status: open
opened: 2026-11-01
priority: P3
cost: M
---


Found by PR 6177's fix pass, bounding `boolean/slit.rs`
`carrier_on_surface` between its samples.

## What

`carrier_on_surface` reads a conic on a quadric at five points
equispaced over its carrier's turn, each offset decided on the band. On
a sphere or a cylinder the offset along a conic is a trigonometric
polynomial of degree 2, so five samples bound it everywhere within
twice their largest (the Lebesgue constant at five points is `1.989`),
inside the band's `escalate / zero` ratio of 10. On a torus the bound
is carried explicitly (`torus_between`). On a cone the offset
`ρ cos α − s sin α` is not a polynomial along a conic: the cone's
equation `ρ² cos² α − s² sin² α` is one of degree 2, but the offset is
it over `ρ cos α + s sin α`, which falls toward the apex. So a conic
near a cone, close at the five samples and off between them, is not
bounded, and can read as lying on it.

Unmeasured: no near-miss witness is built yet. Both readers ask here:
the rider reading (`slit.rs`) and a closed germ edge's
(`sectors.rs` `closed_reach`).

## What a fix would be

Decide the equation's samples, bounded by the degree-2 Lebesgue
constant, over a certified lower bound of `ρ cos α + s sin α` along the
conic, as the torus's bound does with `(ρ + R)² + h² − r²`; or refuse
typed where that bound cannot meet the band.
