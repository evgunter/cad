---
id: torus-split-lead-escalates-a-legitimately-small-resolvent-root
kind: issue
title: line_torus_roots' rung 3 (bool_ray_torus_split_lead) escalates every ray whose odd coefficient is small but decided nonzero, not only a rounding-scale contradiction
status: open
opened: 2026-09-25
priority: P2
cost: D
---


Found by GERM while fixing the resolvent's one-real-root cancellation
(`work/germ/ray-torus-root-search-finds-a-counterexample-at-eps-1e-12.md`).

## The finding

`line_torus_roots` (`crates/topo/src/boolean/solid_contain.rs`) decides
`bool_ray_torus_split_lead` on the resolvent's largest root `z` over
`ext`, and its ladder doc (rung 3) says a non-Positive answer is "a
rounding-scale contradiction" of the constant term `−q̂² < 0`. That is
not what reaches it. In the one-real-root branch `z ≈ q̂²/c1`, so a ray
whose `q̂` clears `bool_ray_torus_odd` (`|q̂|/ext² > K·ε`) can still have
`z/ext ≈ q̂²/(c1·ext)` far inside the band. Every such ray escalates
`Uncertain` through rung 3, although its count is certified and its
roots are well defined.

Measured on the pinned pose
(`r1_the_near_perpendicular_ray_keeps_its_roots` in
`crates/topo/src/boolean/r1_probes.rs`: `R = 1`, `r = 0.9`,
`q̂ ≈ −1.16e-4`, `z ≈ 1.34e-9`, `z/ext ≈ 3.7e-10`). It certifies at
ε = 1e-12 and returns `Uncertain` at ε = 1e-9 and 1e-6
(replay `CAD_FUZZ_SEED=0x2ce3095461764e3a CAD_FUZZ_EFFORT=1` of
`r1_generic_poses`: "1 uncertain/escalated" at both).

So there is a band of `q̂` between the biquadratic arm's threshold and
roughly `√(K·ε·c1·ext)` where the torus door abstains on a
non-degenerate ray. That costs decidedness, not correctness: the
schedule retries another ray. The doc's rung-3 story is wrong about
which rays reach it. Decide on `α = √z` (a length) instead, or widen
the biquadratic arm to meet the resolvent's own threshold. Either
choice moves k-lint margins and wants re-baselining.
