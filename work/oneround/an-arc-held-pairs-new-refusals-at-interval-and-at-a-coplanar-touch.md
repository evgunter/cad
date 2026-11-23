---
id: an-arc-held-pairs-new-refusals-at-interval-and-at-a-coplanar-touch
kind: issue
title: the re-posed curve_pair readers refuse where the base answered rightly: the circle door's basin at Interval (a centre-held ellipse's too), a small ellipse 30 eps off an arc-held one
status: open
opened: 2026-11-23
priority: P2
cost: M
refs: [an-arc-held-ellipses-readers-decide-apart-from-its-derived-centre]
---


Left by `oneround/conic-pair-readers`, which re-posed `curve_pair`'s
readers of where a conic is from its stored point. No reading is wrong;
these are refusals where the base answered rightly, measured on the two
ladders under `analysis/oneround/conic-pair-readers/`.

- **The circle door's `curve_pair_pose_basin` at `Interval`**, and centre-held ellipses with it. On the unit's ladder (`analysis/oneround/conic-pair-readers/ladder.rs`), the readings right on the base and refused on the head at `Interval` are, by refusing row, with twins `20ε` apart (`analysis/oneround/conic-pair-readers/ladder/join-base-head.txt`): at 1e-9, `curve_pair_pose_basin` on the circle door 64, `curve_pair_pose_level_arc` 4 and `curve_pair_walk_pair_gap` 2 (70 in all); at 1e-12, basin 177 and `level_arc` 4 (181). Every one of them is past the session box; in the box, with the seam at the anchor, the head refuses nothing the base read rightly. The review's counts on the first head were 103 and 245 on basin, 27 and 42 on `walk_pair_gap`, 2 and 2 on `level_arc`. The loss is the circle door's polish basin. The line door's `curve_pair_pose_level_arc`, which this row named first, loses next to nothing: it refuses where the base also refused or read wrongly. The loss is not only the placement class. The review's probe (`analysis/oneround/conic-pair-readers/review/probe_window.rs` on PR 7543's review branch, `review/out/window-{base,head}-1e-12.txt`) reads a unit circle across an ellipse through its point at ε 1e-12, `Interval`. An ellipse held at its **centre**, semi-axis 100 m, θ = 0.5, goes from `Ok` with one meeting on the base to `curve_pair_pose_basin` on the head. An arc-held one of 10 m at θ = 2 goes the same way. Both are far below the regression floor. The likely cause, not bisected: `Window::holds_only` at `Interval` now reads the parts it used to pass over (`reaches`, PR 7543's fix of a wrong answer), and the basin meter refuses the extra parts. A refusal, never a wrong answer; but a centre-held verdict newly refused is a regression this row schedules, banded P2.
- **`Unsettled(Tangency)` for a small centre-held ellipse `30ε` off an
  arc-held one in its plane** (the suspects' family F, `clear±30`): every
  such pose refuses at both ε, where the base answered 14 (1e-9) and 45
  (1e-12) of 96 `apart` poses rightly (and 21 and 87 of the meeting ones
  wrongly). The coplanar subdivision now reads the big ellipse's level
  from its anchor (`Conic::level_at`), which resolves the near-touch the
  base's rounding hid; whether the touch rows can split it is not read.
