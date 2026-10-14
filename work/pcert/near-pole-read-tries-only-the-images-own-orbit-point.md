---
id: near-pole-read-tries-only-the-images-own-orbit-point
kind: issue
title: The near-pole read tries only the image's own orbit point, so a continuous whole-turn joint at a band-sized lever misses as decided
status: closed
branch: pcert/near-pole-read-orbit-offsets
opened: 2026-10-13
closed: 2026-10-13
priority: P2
cost: E
---


Found on branch `pcert/cone-singular-gate-lever`, writing
the narrow-cone joint's rows; widened by that PR's review (4983).

`topo::pcurves::near_pole_gap_closes` (`crates/topo/src/pcurves.rs:4340`)
tries the orbit points `m = 0` and, with a sphere twin, `m = ±1` half
steps (`:4347`). It never tries a whole period off the image. So a
joint whose gap is a whole turn (`τ`) at a band-sized lever `ℓ` is not
closed whenever `τ·ℓ` is over ε, though the image one period over
meets the joint exactly.

**The refusal is decided, not merely loud.** Under
`AzimuthRule::Marks` (`:4198`) a miss the near-pole read does not close
returns the marks' own `BranchMiss::OnMark`, which is
`PinMiss::Discontinuity`: `PcurveMintError::LoopDiscontinuity` at rest,
and `turn_element`'s `Err(None)` (`:4269`), a decided miss per
`EulerOpError::KillTurnUndecided`'s doc (`crates/topo/src/euler.rs:1321`).
The joint is continuous, so that is the wrong kind of answer: the same
defect as this item's parent
(`cone-singular-gate-decides-distance-where-the-lever-is-distance-times-sin-alpha`),
one orbit point further out.

Two reaches, both pinned as they are today:

- **The narrow cone.** Row
  `pcurves::stretch_meter::a_narrow_cone_loop_joint_clear_of_the_apex_takes_a_lift_of_its_point`:
  at `α = 0.01`, `d = 2e-8` (lever `2e-10`), a gap of `τ` reads
  `Discontinuity`. The row admits any refusal there and requires a lift
  only where the marks decide; it does not pin the kind.
- **The kill turn on a band-sized cylinder, one orientation.** Row
  `pcurves::turn_miss::a_turn_whose_joint_misses_refuses`: a null strut
  described as the wall's circle once round. `he_minus` reads a gap of
  `+τ`, whose first mark is `π·r` off; for `r` in `(ε/τ, ε/π]` that
  mark decides Zero, the near-pole read meters `τ·r > ε` and does not
  close, and the turn is `Err(None)` (measured at `r/ε` = 0.16 to
  0.318). `he_plus` (gap `−τ`) escalates across the same range.

Both lifts name the one point, so taking `m = ±1` whole periods (`±2`
half steps with a twin) where its gap decides within ε is sound by the
same argument as `m = 0`.

**The knob.** `lever_periods`' and `near_pole_gap_closes`' `twin: bool`
conflates "the image has a twin" with "which orbit offsets are tried".
This fix varies exactly the second, so the parameter becomes the set
of offsets tried (in half steps where there is a twin), and the caller
says which.

Probe (six crates, default ε): no suite joint reaches either. The 31
cone joints whose marks missed at a lever with room were all gaps of
exactly `±π` at levers of 0.5–1 m, in `site_rows`' walk, cleared by its
designed fallback.

## Closed

`near_pole_gap_closes` and `lever_periods` take the joint's `Orbit` in
place of `twin: bool`; the orbit owns both its step and the offsets
tried, so an offset is never read in the other orbit's step.
`Orbit::Periods` tries 0, ±1 whole periods where the image has no twin;
`Orbit::HalfPeriodsWithTwin` tries 0, ±1, ±2 half periods where it has
one. Every offset tried is a lift of the one point: a whole period is a
deck transformation, and a half step lands on the twin, which the
caller's second-channel count then decides. `decide_joint` (both
`Marks` and `MarksNearPole`, and so `turn_element`) passes the orbit
its twin selects; `chord_join::nearest_branch` passes `Orbit::Periods`.

Both reaches now decide a lift:
- `pcurves::stretch_meter::a_narrow_cone_loop_joint_clear_of_the_apex_takes_a_lift_of_its_point`
  requires the lift at gaps 0, 1e-3 and ±τ at every `d`, and on a
  sphere within the band of a pole at 0 and ±τ (two half steps);
- `pcurves::turn_miss::a_sub_band_turn_decides_the_lift_its_gap_meets`
  (renamed from `a_turn_whose_joint_misses_refuses`): both
  orientations decide `u = ∓1` from `0.17ε` to `3ε` by the near-pole
  read, with `10ε` the control where the marks decide, and `u = 0`
  under `ε/τ`;
- `chord_join::nearest_branch_rows::a_narrow_cone_joint_clear_of_the_apex_takes_a_lift_of_its_point`
  gains gaps ±τ.

A sibling of this class, a half-turn joint whose in-band near read is
dropped, is filed as
`an-in-band-near-pole-read-is-dropped-so-a-half-turn-joint-misses-as-decided`.
