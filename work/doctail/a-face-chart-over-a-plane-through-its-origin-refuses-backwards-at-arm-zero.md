---
id: a-face-chart-over-a-plane-through-its-origin-refuses-backwards-at-arm-zero
kind: issue
title: A face chart on a face whose plane passes through its space's origin, with zero slides, is levered over arm 0 and refuses Backwards, naming the orientation already written
status: closed
opened: 2026-11-18
priority: P1
cost: E
refs: [a-face-charts-turn-is-levered-over-none-of-what-it-carries]
closed: 2026-11-20
---


Found independently by both designers of the `lever` fork (2026-11-18), from the code; not yet run.

`mate::value::face_chart` forms `Arm::of(0 + slides, |foot|)` and never asks `Arm::decides_over`. That function's doc says: "every site that decides an angle over an arm asks this first". On a face whose plane passes through its space's origin, with zero slides, the arm is 0. `Margin::levered(chord, 0)` is then 0, the pole verdict reads Zero, and `least_turn_minted` returns `TurnError::Opposed`. So `FaceChartBackwards` fires, and `Orientation::toward` returns the orientation already written (it is already the maximum): the refusal names itself, on a face nowhere near backwards.

**Repro to write:** extrude a profile with a segment from (0,0) to (10,5); `draw_on_face` on that wall with zero slides. Expect a false Backwards. A below-the-band arm should answer `BelowZeroBand`. `Chart::read` does not ask `decides_over` for placements either; its only caller is `unobservable.rs`. Check whether a placement can reach it.

**Relation to the `[ev]` PR on the chart's band:** if Ev rules that the band is decided over what the frame carries, the frame stops deciding at all, and this defect goes with it, because a real carriage always has positive reach. Fix it first only if that ruling is slow. Pin the repro row either way.

## Closed

2026-11-20, with `a-face-charts-turn-is-levered-over-none-of-what-it-carries` (same PR): the frame alone no longer decides, and every carriage levers over a carried reach. The repro is pinned as `face_chart_and_signed_depth::a_tilted_face_through_its_spaces_origin_charts_at_zero_slides` (a tilted bottom cap through the origin, zero slides: the profile charts and its boss builds).
