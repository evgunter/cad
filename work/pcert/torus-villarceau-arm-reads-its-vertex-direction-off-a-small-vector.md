---
id: torus-villarceau-arm-reads-its-vertex-direction-off-a-small-vector
kind: issue
title: The torus chart's Villarceau arm reads its vertex direction off w_r, whose length is the tube radius, so its azimuth data lose u·|c|/r on thin tubes
status: open
opened: 2026-10-13
priority: P3
cost: E
refs: [focal-section-vertex-azimuth-is-read-off-a-cancelling-sum]
---

Found by the class sweep of `pcert/focal-section-vertex-azimuth` (PR
4960), which fixed a vertex azimuth read off a cancelling sum in the
cone arm. This is a different shape: a small vector, not a cancelling
sum.

## What happens

`torus_oblique_circle` (`geom-brep/src/pcurve_cache.rs`) mints the
Villarceau `FocalSection` image of a circle on a torus:

- `pcurve_cache.rs:7961` takes `w = c − center` and `w_r` its radial
  part; the `pcurve_torus_villarceau_offset` gate holds `|w_r| = r`, the
  tube radius.
- `pcurve_cache.rs:7981` reads the vertex direction `d = w_r / |w_r|`.
  The rounding in `w` is about `u·|c|` (the positions' magnitude), so
  `d`'s direction is off by about `u·|c|/r`.
- `d` then feeds the image's data: `sense` off `axis × d`
  (`pcurve_cache.rs:7998`), `t0` (`:8002`) and `u0` (`:8006`).
- The tilt gate (`pcurve_cache.rs:7968`) compares `|n × axis|` with
  `r/R`, so `n × axis` is a vector of length `r/R` and its direction
  conditions the same way, about `u·R/r`.

On a thin tube placed far from the origin (`r ≪ |c|`) the image's
`u0`/`t0` turn by about `u·|c|/r`, and the image misses by about
`R·u·|c|/r`. No row measures this; nothing in the suites is known to
reach it.

## What closes it

A row on a thin-tube torus far from the origin in a skew pose that
measures the whole-turn miss against `u·|c|/r`, and either a datum
that does not divide by `r` (for example, the direction read off a
vector of size `R`), or a note that the conditioning is inherent and
bounded by the incidence band.
