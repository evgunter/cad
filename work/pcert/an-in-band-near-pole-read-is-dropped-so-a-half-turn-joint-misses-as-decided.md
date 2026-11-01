---
id: an-in-band-near-pole-read-is-dropped-so-a-half-turn-joint-misses-as-decided
kind: issue
title: An in-band near-pole read is dropped, so a continuous half-turn joint at a band-sized lever misses as decided
status: closed
branch: pcert/near-pole-read-tri-state
opened: 2026-10-14
closed: 2026-10-14
priority: P2
cost: M
refs: [near-pole-read-tries-only-the-images-own-orbit-point]
---

Found by PR 5028's review (probe `review_5028_probes` on
`review/5028-r`); filed in that PR. The same class as its parent,
`near-pole-read-tries-only-the-images-own-orbit-point`: a joint that
can be continuous in metres answers a **decided** discontinuity.

**The drop.** `topo::pcurves::lever_periods`
(`crates/topo/src/pcurves.rs:4413`) asks the near-pole read where the
marks miss, as `near_pole_gap_closes(..).ok_or(miss)` (`:4424`).
`near_pole_gap_closes` (`:4372`) returns an `Option`: `Some` only for an
offset decided Zero, so an offset whose gap is **in band** reads the
same as one decided off. Where the marks' miss is `OnMark` (decided),
that decided miss is returned though the read never decided the joint
discontinuous. Under `AzimuthRule::Marks` (`:4188`) it becomes
`PinMiss::Discontinuity`: `PcurveMintError::LoopDiscontinuity` at rest,
`turn_element`'s `Err(None)`. Through `chord_join::nearest_branch`
(`crates/topo/src/chord_join.rs:2847`) it is a
`SplitJoinError::SectionInvariant`.

**The reach.** A gap of half a period, on a mark, at a lever `ℓ` with
`π·ℓ` in band, i.e. `ℓ` in `(ε/π, escalate/π)`. The two nearest
offsets (`m = 0`, `m = 1`) both sit `π·ℓ` off, in band. The two chart
points are `2ℓ` apart in metres, within the joint bound (`4ε`, see
`decide_joint`'s docs) for `ℓ ≤ 2ε`, so the joint can be continuous.
The honest answer there is an escalation.

- Pinned as it is today:
  `pcurves::stretch_meter::a_narrow_cone_loop_joint_clear_of_the_apex_takes_a_lift_of_its_point`,
  on the `α = 0.01` cone, gap `π`, at the levers in that range (`0.5ε`,
  `1ε`), reads `Discontinuity`.
- No suite joint reaches it today. The configuration is the 31 cone
  joints of PR 4983's measurement (`site_rows`' walk, gap exactly
  `±π`, levers 0.5–1 m), shrunk to a lever in `(ε/π, escalate/π)`.

**The fix shape.** `near_pole_gap_closes` returns a tri-state: closes
at `m`, every offset decided off, or some offset in band (with its
cause). `lever_periods` surfaces the in-band case as
`BranchMiss::Undecided(cause)` instead of the marks' `OnMark`, so a
decided miss is returned only where every offset tried is decided off.
This changes a decided answer to an escalation at the reach above and
nowhere else (the read runs only where a mark missed).

**Sweep for the shape** (a near read whose undecided result is folded
into "does not close"): `.ok_or(miss)` and `matches!(decide(..),
Ok(Sign::Zero))` over `crates/*/src`.
- `pcurves::near_pole_gap_closes` / `lever_periods`: this item.
- `boolean::circle_torus` (`bool_circle_torus_coaxial_tilt`),
  `boolean::circle_roots` (`rows.side`'s `on`), and
  `boolean::vtxfac` (`bool_held_carrier_datum`) fold an in-band reading
  into "not Zero" too. None is a near read of a chart orbit, so none is
  this item. Whether each fold is sound is not judged here.

**Second pass**, at that sweep's blind spot (a fold spelled
`== Ok(Sign::Zero)`, `if let Ok(Sign::Zero) = decide`, or
`.is_ok_and(..)`): `geom_core::sym` (`sym_test`, a test helper),
`profile::path` (`path_collinear_target`) and `boolean::circle_roots`
(`rows.coaxial`). None is a near read of a chart orbit. **What neither
pass matches:** a fold through a helper that returns `bool` from a
`decide` under an unrelated name.

## Closed

`near_pole_gap_closes` answers a `NearRead`: `Closes(m)` at the first
offset decided within ε, else `InBand(cause)` (the first in-band
offset's cause) if any offset was in band, else `Off`. The offsets it
tries are the joint's `Orbit` offsets about the image, then the two
branches astride the mark the marks' walk stopped on
(`geom_brep::whole_period_walk`'s `WalkStop::astride`). Where the walk
stops on a mark decided Zero and the read decides both branches astride
it off, the gap lies between them, so `π·ℓ ≥ escalate` and every other
orbit point is at least `3π·ℓ − ε` off: `Off` there means no orbit
point is within the band, at any `K > 1`.

`lever_periods` answers an in-band read over the marks' decided
`OnMark` with `BranchMiss::Undecided(cause)`; an undecided mark keeps
its own cause, and `Off` keeps the marks' miss. Both reaches follow:
`decide_joint` escalates (`PinMiss::Escalated`, `turn_element`'s
`Err(Some)`), and `chord_join::nearest_branch` answers
`SplitJoinError::Escalated` in place of `SectionInvariant`.

Rows:
- `pcurves::stretch_meter::a_narrow_cone_loop_joint_clear_of_the_apex_takes_a_lift_of_its_point`
  and `chord_join::nearest_branch_rows::a_narrow_cone_joint_clear_of_the_apex_takes_a_lift_of_its_point`
  pin a gap of `π` in all three regimes (within ε the image, in band
  an escalation, past the band the decided miss);
- `pcurves::stretch_meter::a_half_turn_gap_answers_by_the_band_at_every_turn_count`
  and its `nearest_branch_rows` twin pin gaps of `k + ½` turns, `|k|`
  up to 3, at levers between and just either side of the band edges.

**The kill turn.** A turn's ends meet within ε, not exactly, so a
stored gap of half a turn is admissible. No decided miss results: the
ends meet only where the chord `2ℓ` between them is within ε, so
`π·ℓ ≤ (π/2)·ε`, under the escalate edge wherever that edge is at
least `(π/2)·ε` (`K ≥ π/2`; the linear band's `K` is 10), and the read
answers `Closes` or `InBand`; where `π·ℓ` is past escalate the chord is
past ε and the ends do not meet. A band with `K < π/2` leaves a window,
filed as `a-half-turn-kill-under-a-band-tighter-than-pi-over-two-misses-as-decided`.
