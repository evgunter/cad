---
id: the-largest-gram-eigenvalue-of-a-harmonic-is-spelled-in-three-files
kind: issue
title: The largest Gram eigenvalue of a harmonic's coefficients, (aa+bb)/2 + √(((aa−bb)/2)² + ab²), is spelled in implicit.rs, harmonic.rs and pcurve_cache/speed.rs
status: open
opened: 2026-11-23
priority: P1
cost: M
refs: [7509]
---


Found by CHARTTAIL's review of PR 7509 (`charttail/speed-census`), 2026-11-23.

## What

`sup_t |a·cos t + b·sin t|² = (aa + bb)/2 + √(((aa − bb)/2)² + ab²)` is the larger eigenvalue of the Gram matrix of `(a, b)`. It is written out in three files:

- `crates/geom-brep/src/harmonic.rs`, `harmonic_sup`: generic over `T: Real`, as `(m + (q.powi(2) + a.dot(b).powi(2)).sqrt()).sqrt()`, with the test mutants the harmonic-sup rows refuse.
- `crates/geom-brep/src/implicit.rs`, the conic-torus harmonics (`gram = (aa + bb) * half + (((aa - bb) * half).powi(2) + off.powi(2)).sqrt()`): generic, where `off = a·b·(û·v̂)`.
- `crates/geom-brep/src/pcurve_cache/speed.rs`, `gram_max`: on `Interval`. PR 7509 gave its two spellings there (`carrier_speed`, `pole_azimuth_speed`) one home, but it cannot reach the other two. `speed.rs` is a certification-doors importer with no `Real` in scope, and the other two are lane code over `T`.

## Owed

Give the formula one home that both the lane code and certification arithmetic read. One option is a generic `gram_max<T: Real>(aa, bb, ab)` that `harmonic_sup` and `implicit.rs` call, and that `speed.rs` reaches at `T = Interval`. Whether a certification importer may call a `Real`-generic helper at `Interval` is the certification-doors gate's question (its known gap on lane evaluation at `T = Interval`), and it decides the shape. The harmonic-sup mutants have to keep reaching the production path.
