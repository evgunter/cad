---
id: quadrature-interval-floor-grows-with-the-body-past-the-band
kind: issue
title: the quadrature's interval floor grows with the body, so at scale 10^3 the volume backstop cannot tell a correct zero-margin result from a wrong one
status: closed
opened: 2026-10-01
priority: P2
cost: M
pr: 5842
closed: 2026-10-27
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

## A seamless pipe a kilometre out (FLUX, PR 4839, 2026-10-12)

`topo/tests/a_seamless_pipe_cut_by_an_oblique_plane_measures.rs`, the
Interval row: the pipe in the frame at `(1e3, −7e2, 4e2)` with axis
`(0.3, −0.2, 0.9)`, its floor 100 m up the axis, runs its wall out of
budget at `CAD_TOLERANCE_EPS=1e-12` only: `QuadratureBudget { width_len
3.33e-9, target_len 1.024e-9, rounds 13 }`. The same pose converges at
`f64` and in the world frame at `Interval`, and every other pose and
frame converges at all three ε. Which term holds the width is not
measured; the position term `o·A⃗`, read at the interval scalar a
kilometre out, is the candidate. The row pins it by type
(`runs_out_of_budget_at_interval`).

**Measured (FLUX spiric-budget unit, `flux/spiric-budget-world-position`,
2026-10-20).** It was the position term. With each face's flux read about
a vertex of its shell (`topo::props`' `shell_reference`) instead of the
world origin, this pose converges at `Interval` and ε = 1e-12. The pin is
gone and the row asserts the enclosure for every pose. The scale-1 rod's
backstop floor above is not re-measured here, so this item stays open
for it: the ε share and the round-6 rule remainder are not position
terms.

## Re-measured on main 258dc6c7c2 (QUAD, 2026-10-21)

**Verdict: shrunk.** The scale-10³ plants now decide. What is left is
a scale dependence, not a position one. The backstop refuses the
correct answer from between scale 3×10³ and 4×10³ at ε = 1e-9, and
#5442's `shell_reference` did not move that threshold.

Probe: `crates/sweep/tests/quad_frame_remeasure.rs`
`row2_backstop_right_answer_by_scale`. It runs
`topo::test_support::volume_backstop(Subtract, A, B, ·)` on
`reach_volume_backstop.rs`'s scaled oblique rod pair, planting `A`
itself (the correct answer) and `A` fattened by 1e-12 relative:

| ε | scale s (m) | right answer | fattened 1e-12 |
|---|---|---|---|
| 1e-9 | 1, 10², 10³, 2·10³, 3·10³ | Validated | accepted |
| 1e-9 | 4·10³, 6·10³, 10⁴ | `VolumeUndecided` | `VolumeUndecided` |
| 1e-8 | 1, 10³, 4·10³, 10⁴, 3·10⁴ | Validated | accepted |
| 1e-12 | 1 | Validated | accepted |
| 1e-12 | 10³ | `VolumeUndecided` | `VolumeUndecided` |

The same probe on main before #5442 (`ec32861499`) gives the same
verdicts at 1e-9 (10³ to 3·10³ validate, 4·10³ and 10⁴ refuse) and at
1e-12 scale 1 (validates). So the change from this row's 2026-10-01
numbers came from an earlier change on main, and I cannot tell which.
Those numbers were: 10³ refuses at 1e-9, 10⁴ refuses at 1e-8, and the
scale-1 rod is undecidable at 1e-12. The row's own analysis still
fits: a rule remainder ∝ s³ against a lever ∝ s². The
`last_round_open` comment in `reach_volume_backstop.rs` already states
the 3e12·ε–4e12·ε flip.

At ε = 1e-12, scale 4·10³ does not build: the fixture's extrude
refuses `Pcurve(Certify … MapResidual)`. A 4 km body at 1e-12 is past
the model's range, so I filed nothing for it.

The kilometre pipe at 1e-12:
`a_seamless_pipe_cut_by_an_oblique_plane_encloses_its_closed_form_at_interval`,
run on main at `CAD_TOLERANCE_EPS=1e-12`, passes: every frame and pose, the (1e3, −7e2, 4e2) one included, encloses at `Interval`. **Gone**
(#5442), as the 2026-10-20 section says.

## Re-checked on main `9267d3ae` (QUAD tracker sweep, 2026-10-27)

Still holds, in kind. The premise of the first section is gone:
`LAST_ROUND_EVERY_LANE_RUNS` and `PastTarget::refine` went in PRs 5589
and 5628. Each face now refines to its own `Floor` or `Wall` in one
`walk_set` (`crates/topo/src/props.rs:826`), and `bound_holds` is at
`crates/topo/src/boolean/ops.rs:2802`. The scale dependence remains.
`crates/sweep/tests/reach_volume_backstop.rs:315`–`336`
(`last_round_open`) now states the open range as ≈ 1.2e-13·s. The
right answer stops deciding between 6e13·ε and 1e14·ε at ε = 1e-9,
where it was 3e12·ε to 4e12·ε under the round caps.
`an_open_sign_beyond_the_band_at_the_last_round_refuses` (`:594`)
still pins the refusal and cites this row. The threshold moved about
20× outward, but it still exists and still grows with the body, so
the band is kept at P2. The `row2_backstop_right_answer_by_scale` probe
(`crates/sweep/tests/quad_frame_remeasure.rs:165`) was not re-run.

## Closed (2026-10-27)

The volume backstop left the boolean's operation path (`[ev]` PR 5780: the volume relations are a test oracle the suites run on their own results, `topo::test_support::volume_relations`), so nothing refuses or passes on it. Its consequence on the operation path, the backstop's `VolumeUndecided` on a correct large body, is gone, and with it `an_open_sign_beyond_the_band_at_the_last_round_refuses` and the `row2_backstop_right_answer_by_scale` probe. Nothing on the operation path decides a relation between bodies' volumes now; a deciding predicate that meets the floor at scale is a new row.
