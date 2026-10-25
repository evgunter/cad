---
id: a-smooth-joint-at-a-near-concentric-crossing-validates-at-f64-and-escalates-at-interval
kind: issue
title: A smooth joint at a crossing of near-concentric carriers validates at f64 and escalates carrier_circles_chord at Interval, at 1e-12
status: open
opened: 2026-10-19
priority: P3
cost: M
---


Found by `paths/joint-reads-its-turn` while pinning the crossing-joint
rows of #5354's ruling; the same on its base `4378c864f8`, so it is
not that unit's.

`crates/profile/tests/rejections.rs`'s `crossing_loop` with
`near_concentric(d, 0.5ε)`, the joint at the carriers' crossing
(`at = −acos((1 + d² − r₂²)/2d)`) and the second arc on round
(`sweep = +0.8`), at `CAD_TOLERANCE_EPS=1e-12`:

| d | f64 | Interval |
|---|---|---|
| 0.1 | validates | `Escalated` at `SegmentPair(0/0, 0/1)` on `carrier_circles_chord` |
| 0.3 | validates | the same |

The cusp rows (`sweep = −0.8`) escalate on `carrier_circles_chord` at
both scalars (the f64 margin 4.75e-12 and 1.42e-12, in band), so the
two lanes disagree only on the smooth joint: the f64 lane settles it
and the Interval lane's enclosure does not. `validate_both` in that
file refuses a disagreement, which is why
`a_joint_at_a_crossing_of_near_concentric_carriers_is_a_corner` runs
`d = 0.1` and `0.3` only at 1e-9 and 1e-6.

Whether this is the Interval lane being honestly wider (the chord read
at a foot computed from a near-cancelling `d`) or the f64 lane
settling a chord the band cannot see is the open part. Start at
`seg::arc_arc`'s chord arm (`crates/profile/src/seg.rs`,
`carrier_circles_chord`).

## Re-homed at PATHS's close (2026-10-21)

Moved from `work/paths/` to PROFILEREST (`work/profilerest/`) when PATHS closed. PATHS's live rows went to four programs opened at this close and cut along the priority seam (PROFILE, PROFILEREST, PROFILETAIL, PROFILEPROSE; `docs/doc-ledger/paths-leaves-the-tracker.md`). The item's id, content and history are unchanged.
