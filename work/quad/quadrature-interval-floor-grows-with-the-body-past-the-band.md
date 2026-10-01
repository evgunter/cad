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

## More evidence (REACH, reach-eps lane, PR 3636, 2026-10-01)

**Measured:** the 5.5e-10 m³ "floor" above is ε-proportional, not
rounding. I instrumented `bound_holds` and `PastTarget::refine` (not
committed) and printed the oblique rod's per-round half-widths at three
ε rows:
- 1e-9: 9.2e-7, 1.16e-7, 1.49e-8, 2.34e-9, 7.7e-10;
- 1e-6: 6.0e-5, 8.0e-6, 1.47e-6, 6.6e-7, 5.6e-7, 5.5e-7, 5.49e-7;
- 1e-12: 1.79e-9, 2.25e-10.

Every round reads as `rule(round) + 0.549·ε`. Subtracting 0.549·ε from
the 1e-6 rounds gives the 1e-9 rounds' widths, and the last round's
rule share is ≈ 2.2e-10 m³ at every ε. So:
- past round 6 at the default ε the width stops at the ε share. The
  source of that share is not traced; it is presumably the stored trim
  geometry's own ε;
- at round 6 (`LAST_ROUND_EVERY_LANE_RUNS`) the rule remainder is
  still there. It is what grows with the body (∝ s³ against a lever
  ∝ s²), and it makes the backstop undecidable on the scale-1 rod at
  ε = 1e-12 (open range 1.8e-11 m against K·ε = 1e-11).

The fix shapes above (compensated sums, a face-local origin) target
rounding. Measure them against the round cap and the ε share before
building either.
