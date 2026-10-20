---
id: a-full-revolves-return-half-rim-misses-its-start-by-two-rounded-half-turns
kind: issue
title: a full revolve's return-half rim starts at fl(π) and runs fl(π) more, so it ends r·2·(π − fl(π)) short of its start vertex, past the band at ε = 1e-12 from r ≈ 4100
status: open
opened: 2026-10-14
priority: P3
cost: M
refs: [a-flat-frustums-revolve-refuses-its-rim-image-at-1e-12, conic-section-just-past-its-band-fails-certification-downstream]
---

Found closing `a-flat-frustums-revolve-refuses-its-rim-image-at-1e-12`:
its `(α, S) = (1.55, 100)` row is the one that row's fix leaves
refusing, at another door.

## Measured

The frustum `(0, 0), (r0, 0), (50, 100), (0, 100)`, `r0 = 50 + 100·tan
1.55 = 4857.85`, revolved a full turn about `y` at ε = 1e-12, refuses

> `Op { Certification { Escalated { EndpointEnd, sample 8, margin 1.19e-12, carrier_endpoint_end } } }`

rendered "There is no way through: this is a kernel defect; report it".
The edge is the far cap rim's return half, minted by
`crates/sweep/src/revolve/full.rs` `build_wire`'s band-2 `mef`: a
`Circle` of radius `r0` whose `u_ref` is the vertex `far.point(..)`
placed by `Placing::turned(.., half)` with `half = fl(τ)/2 = fl(π)`, and
`param_end = fl(π)`. The vertex sits at azimuth `fl(π) = π − 1.22e-16`
(its `z` is `−r0·sin fl(π) = −5.9e-13`), and the rim ends at azimuth
`2·fl(π) = 2π − 2.45e-16`: `r0·2.45e-16 = 1.19e-12` from the start
vertex it closes on (`eval(t1) = (4857.85, 0, 1.19e-12)` against
`(4857.85, 0, 0)`). The band-1 half from the start vertex ends exactly on
the far vertex, because both are placed at the same `fl(π)`.

So the return half carries both halves' angle rounding. With the far
vertex placed at an exact half turn (the radial part negated, which is
exact), each half would miss its far end by `r·(π − fl(π)) = r·1.22e-16`
and the floor would move from `r ≈ 4100` to `r ≈ 8200` at ε = 1e-12;
a span chosen as the `f64` nearest `τ − fl(π)` gains only 2.45 → 2.0.
Below the floor itself, `r·(π − fl(π))` is the angle parameter's own
resolution and no spelling of a `Circle` carrier removes it.

## What closes it

Either place the full revolve's far half-turn exactly (every full
revolve's angle-π vertices and meridians move by `r·1.22e-16`, so stored
bits and goldens move with it), or show the doubled floor is not worth
that and end the refusal as a kernel limitation of the angle's
resolution rather than "kernel defect; report it" (Ev's interim on
`conic-section-just-past-its-band-fails-certification-downstream`).

The same hunt (300 fresh flat frusta, α ∈ [0.9, 1.565], S ∈ [0.1, 100],
ε = 1e-12) refuses two more poses at r0 = 2245 and 2572 on
`MappedSource` (sample 3, margins 1.02e-12 and 1.12e-12) and one at r0
= 1882 on a cap rim's `MapResidual` (sample 3, 1.05e-12). Each is a
sampled `f64` evaluation of two formulas at coordinates near 2000,
about two ulps apart (the rim's image itself misses by 1.1e-14 in
exact arithmetic): the coordinates' own resolution at that ε, not this
row's mechanism, and the same ending applies.
