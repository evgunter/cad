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

- **The circle door's `curve_pair_pose_basin` at `Interval`**, and centre-held ellipses with it. On the unit's ladder (`analysis/oneround/conic-pair-readers/ladder.rs`), the readings right on the base and refused on the head at `Interval` are, by refusing row, with twins `20ε` apart (`analysis/oneround/conic-pair-readers/ladder/join-base-head.txt`): at 1e-9, `curve_pair_pose_basin` on the circle door 64, `curve_pair_pose_level_arc` 4 and `curve_pair_walk_pair_gap` 2 (70 in all); at 1e-12, basin 177 and `level_arc` 4 (181). On the unit's ladder every one of them is past the session box, and in the box, with the seam at the anchor, the head refuses nothing the base read rightly. That is true of the ladder's poses, all within 3 m of the anchor, and not of the box. The review's counts on the first head were 103 and 245 on basin, 27 and 42 on `walk_pair_gap`, 2 and 2 on `level_arc`. The loss is the circle door's polish basin. The line door's `curve_pair_pose_level_arc`, which this row named first, loses next to nothing: it refuses where the base also refused or read wrongly. The loss is not only the placement class. The review's probe (`analysis/oneround/conic-pair-readers/review/probe_window.rs` on PR 7543's review branch, `review/out/window-{base,head}-1e-12.txt`) reads a unit circle across an ellipse through its point at ε 1e-12, `Interval`. An ellipse held at its **centre**, semi-axis 100 m, θ = 0.5, goes from `Ok` with one meeting on the base to `curve_pair_pose_basin` on the head. An arc-held one of 10 m at θ = 2 goes the same way. Both are far below the regression floor. The likely cause, not bisected: `Window::holds_only` at `Interval` now reads the parts it used to pass over (`reaches`, PR 7543's fix of a wrong answer), and the basin meter refuses the extra parts. A refusal, never a wrong answer; but a centre-held verdict newly refused is a regression this row schedules, banded P2.
- **`Unsettled(Tangency)` for a small centre-held ellipse `30ε` off an
  arc-held one in its plane** (the suspects' family F, `clear±30`): every
  such pose refuses at both ε, where the base answered 14 (1e-9) and 45
  (1e-12) of 96 `apart` poses rightly (and 21 and 87 of the meeting ones
  wrongly). The coplanar subdivision now reads the big ellipse's level
  from its anchor (`Conic::level_at`), which resolves the near-touch the
  base's rounding hid; whether the touch rows can split it is not read.

## In the box, refused where the base read rightly (2026-11-23, PR 7543's delta read)

The delta read of PR 7543 (`analysis/oneround/conic-pair-readers/delta-report.md` on `oneround/conic-pair-readers-review` at `33e51ba6d`, m4) found three groups in the box, the seam at the anchor, that the base read rightly and the head refuses. Each is a refusal, not a wrong answer. Counts are against the base, as that read measured them:

1. **The circle door's basin at `Interval`, far along the arc.** On the reviewer's wall ladder (`review2/ladder_wall.rs`, the anchor 950 m along the arc's tangent from the box's centre): 22 readings at ε 1e-12, none at 1e-9, none at `f64`. All are `curve_pair_pose_basin` on the circle door, the 20ε twin the base read apart, at 1,500 and 1,850 m of arc on fat sections from `a` = 1e3 to 1e9.
2. **Family F's apart twins at `f64`**, on the suspects' harness (every pose within 3 m of the anchor): 13 at 1e-9 (`a` ≥ 1e8) and 51 at 1e-12 (`a` ≥ 1e5), on `Unsettled(Tangency)`. This is the second bullet above, recounted on that read's base.
3. **One meeting of family G at `f64`** (two arc-held ellipses across planes): ε 1e-12, `a` = 1e5, on `curve_pair_pose_basin`.

**Reachable through a door, by the tree.** Nothing caps an arc-held conic's size:
- `Curve3::representability_margins` (`crates/geom/src/curves.rs`) bounds an ellipse's semi-axes from below only;
- `carrier_resolution::resolution` charges about `u·a` into margins and refuses nothing;
- `plane_cone_section` mints these sections at any size.

Validation's census reads two faces' edge pairs through `curve_pair::meet` (`topo/src/census.rs`'s `shared_run` and the witness pairs, through `census/curved.rs`'s `interiors_meeting`). It does so at `f64` on every validate, and at `Interval` in the editor's interval replay (`editor-core/src/eval/mod.rs`). A body with such an edge beside another edge standing as the twin does would have its validation refused where the base certified it. No construction found today builds such a pair as edges of one body; that is not shown either way. Reported to the orchestrator for Ev under PR 7140's standing condition.
