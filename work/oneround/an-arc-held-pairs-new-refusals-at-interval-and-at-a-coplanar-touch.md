---
id: an-arc-held-pairs-new-refusals-at-interval-and-at-a-coplanar-touch
kind: issue
title: the re-posed curve_pair readers refuse where the base answered rightly: the slanted arc's width at Interval, a small ellipse 30 eps off an arc-held one
status: open
opened: 2026-11-23
priority: P3
cost: M
refs: [an-arc-held-ellipses-readers-decide-apart-from-its-derived-centre]
---


Left by `oneround/conic-pair-readers`, which re-posed `curve_pair`'s
readers of where a conic is from its stored point. No reading is wrong;
these are refusals where the base answered rightly, measured on the two
ladders under `analysis/oneround/conic-pair-readers/`.

- **`curve_pair_pose_level_arc` at `Interval`** (the line door in the
  ellipse's plane, `ladder_rows.rs`): `Slant::read`
  (`crates/geom-brep/src/curve_pair/pose.rs`) takes the `|R| ≤ e` arc's
  ends from `Height::roots(±e)` and reads its width up `θ` within a turn,
  `up − τ·⌊up/τ⌋`; where `up`'s enclosure reaches below zero the floor
  hulls the width to a whole turn and the arc refuses. 146 readings at
  1e-9 and 306 at 1e-12, most of them poses the base read `apart`
  rightly; at 1e-12 the `Interval` lane's right answers fall from 468 to
  302 of 1008. A width read per arc case (the side arcs are at most half
  a turn by construction) would keep the enclosure narrow.
- **`Unsettled(Tangency)` for a small centre-held ellipse `30ε` off an
  arc-held one in its plane** (the suspects' family F, `clear±30`): every
  such pose refuses at both ε, where the base answered 14 (1e-9) and 45
  (1e-12) of 96 `apart` poses rightly (and 21 and 87 of the meeting ones
  wrongly). The coplanar subdivision now reads the big ellipse's level
  from its anchor (`Conic::level_at`), which resolves the near-touch the
  base's rounding hid; whether the touch rows can split it is not read.
