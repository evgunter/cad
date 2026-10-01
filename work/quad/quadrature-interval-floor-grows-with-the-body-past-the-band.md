---
id: quadrature-interval-floor-grows-with-the-body-past-the-band
kind: issue
title: the quadrature's interval floor grows with the body, so at scale 10^3 the volume backstop cannot tell a correct zero-margin result from a wrong one
status: open
opened: 2026-10-01
---


## What

A face's certified flux enclosure stops narrowing at an interval
rounding floor, and the floor scales with the body. On the
oblique-capped rod (`r = 0.5`, cut 20° at `z = 3.5`) it is about
5.5e-10 m³ at scale 1. Rounds past the reporting target shrink the
half-width about 8× each (9.2e-7 at the target, 7.7e-10 at round 6),
and from round 6 on the width stops shrinking (measured to round 12).
Metered as a boundary displacement over the body's area, the floor
grows linearly with size.

REACH's volume backstop (`crates/topo/src/boolean/ops.rs`,
`bound_holds`) refines both compared bodies to
`LAST_ROUND_EVERY_LANE_RUNS` and then decides the open range against
the band. Measured on branch `reach/volume-backstop` (2026-10-01),
ε = 1e-9:
- At scale 10³ and 10⁴ (the rod 4 km and 40 km tall), the CORRECT
  result of `A ∖ B` planted as itself refuses `VolumeUndecided`.
- So does the reviewer's `A ∩ big → A` plant at those scales.
- Scales 1 and 10² decide.
- A looser tolerance helps only within limits: at ε = 1e-8 the scale-10³
  plants validate and scale 10⁴ still refuses (the PR 3611 review).

Pinned by `crates/sweep/tests/reach_volume_backstop.rs`
`an_open_sign_beyond_the_band_at_the_last_round_refuses`.

## The shape of a fix

The floor is the quadrature's rounding and not its rule. Lowering it
relative to the body would let the backstop decide large bodies, for
example by:
- accumulating the composite sums with compensated or wider interval
  arithmetic;
- forming the Green integrand about a face-local origin rather than the
  world one, so the `o·A⃗` term does not carry the body's absolute
  position into every piece.

Measure first which term dominates the floor.
