---
id: a-circle-of-radius-1500-certifies-miss-on-two-true-roots-at-eps-1e-12
kind: issue
title: The ρ = 1500 conic × quadric sweep certifies Miss on two true roots at ε 1e-12 (seed 0x8b36961a61c3e5d1)
status: open
opened: 2026-11-27
priority: P1
cost: M
---


Found by the LATHE teapot lane on PR 7824's hosted CI (run 45998636509,
`test (kernel)`, the ε 1e-12 step, 2026-11-27). PR 7824 does not touch
`geom`, `geom-core` or `geom-brep`, so this is a counterexample the
varying-seed fuzzer drew, not something that PR caused.

## What

`geom_brep::curve_face::conic_quadric::in_band_sweep`
`a_circle_of_radius_1500_places_its_roots_within_the_zero_band` drew
one wrong answer at ε 1e-12:

```
1 wrong answers (reproduce with CAD_FUZZ_SEED=0x8b36961a61c3e5d1 CAD_FUZZ_EFFORT=1):
ε 1e-12, depth 6.335210560086177e-4, g 3.9456287479666026, b 4.713041627663133: Miss on 2 true roots
```

A certified `Miss` where the oracle has two true roots is a wrong
answer, not a refusal. That is why this is P1. The depth, 6.3e-4 m, is
far outside the band, so it is not a pose at the band's edge. The
related rows are closed:
`oneround/a-conics-point-is-evaluated-from-its-far-centre-not-its-vertex`
and `germ/the-half-angle-ladder-certifies-in-band-configurations`.

## Reproduce

`CAD_TOLERANCE_EPS=1e-12 CAD_FUZZ_SEED=0x8b36961a61c3e5d1 CAD_FUZZ_EFFORT=1 cargo nextest run -p geom-brep -E 'test(a_circle_of_radius_1500_places_its_roots_within_the_zero_band)'`

This reproduces locally with that command (2026-11-27). Once the cause is known, pin the pose as a deterministic row beside the
fix.
