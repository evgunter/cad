---
id: torus-villarceau-arm-reads-its-vertex-direction-off-a-small-vector
kind: issue
title: The torus chart's Villarceau arm reads its vertex direction off w_r, whose length is the tube radius, so its azimuth data lose u·|c|/r on thin tubes
status: closed
opened: 2026-10-13
priority: P3
cost: E
refs: [focal-section-vertex-azimuth-is-read-off-a-cancelling-sum]
closed: 2026-10-14
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

## Closed 2026-10-14: the premise is wrong

`w = c − center` is one correctly rounded subtraction of two stored
points. Each component errs by at most `u·|w_i|`, so `w` (and `w_r`,
whose axial part is gated to zero) is good to relative `u` whatever
`|c|` is, not to `u·|c|`, and `d = w_r/|w_r|` is good to about `u`.
`n × axis` enters only the tilt gate, never the image.

Measured with `torus_oblique_circle` on analytic Villarceau circles:
`R = 2`, `r ∈ {0.7, 1e-2, 1e-4}`, torus centre at distance `L` from
the origin, axis-aligned and skew frames, ε = 1e-9. The miss is
whole-turn and 4096-sampled against the stored carrier:

| frame, L | miss at r = 0.7 / 1e-2 / 1e-4 | carrier off the torus | certificate |
|---|---|---|---|
| aligned, 0 | 3.0e-15 / 2.6e-15 / 3.4e-15 | 8e-16 | ok |
| aligned, 1e3 | 1.71e-13 at every r | 9.5e-14 | ok |
| aligned, 1e6 | 1.75e-10 at every r | 1.0e-10 | ok (1.5e-12 to 3.9e-11) |
| skew, 1e6 | 2.67e-10 / 2.02e-10 / 2.02e-10 | 1.1e-10 to 1.4e-10 | ok |

The miss does not grow with `1/r`. It tracks the carrier's own
resolution `u·L`. `d` does differ from the construction's ideal
direction by about `0.1·u·|c|/r`, but that is the stored centre
`center + d·r` rounding by `u·|c|`, and the image follows the stored
carrier, as it must.

The table is reproduced by `probe_villarceau_small_vector`
(`geom-brep/tests/pcurve_villarceau.rs` on branch `pcert/villarceau-probe`
@ `d0b8357e1`, not for merge; run with `--no-capture`). The review of PR
5050 reproduced it independently (`review/5050-r` @ `5aefe397a`,
`pcurve_cache/review_5050_probe.rs`). Its adversarial row moves the
centre by `δ` and gets a miss of exactly `δ` at every `r`, so the image
follows the carrier rather than any ideal direction.

The suite's 260 torus `FocalSection` certificates agree: envelope /
span miss is at most 3.6, and nothing grows with a thin tube.
