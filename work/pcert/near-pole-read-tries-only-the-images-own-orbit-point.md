---
id: near-pole-read-tries-only-the-images-own-orbit-point
kind: issue
title: The near-pole read tries only the image's own orbit point, so a continuous whole-turn joint at a band-sized lever misses as decided
status: open
opened: 2026-10-13
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
  `pcurves::stretch_meter::a_narrow_cone_loop_joint_clear_of_the_apex_takes_its_own_image`:
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
