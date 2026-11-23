---
id: face-chart-folds-into-the-datum-once-a-datum-levers-its-pole
kind: issue
title: PoseDef::FaceChart folds into the datum once a datum levers its pole over what it carries
status: open
opened: 2026-11-22
priority: P3
cost: M
design: true
refs: [a-plane-other-than-a-face-cannot-be-authored-in-the-viewer, a-face-charts-turn-is-levered-over-none-of-what-it-carries, 7151, 7382]
---


The datum design (DESIGN `[placement.datums]`; the
`a-plane-other-than-a-face-cannot-be-authored-in-the-viewer` row's
"The design (2026-11-22)") says `PoseDef::FaceChart`, a frame placed
flush on a face by a fixed one-mate bundle, folds into the datum: draw
on a face would insert a datum mated flush on it and draw on its frame.
PR 7382 built that fold (`Recording::datum_on_face`), then reverted it
before landing, because PR 7151 (doctail,
`a-face-charts-turn-is-levered-over-none-of-what-it-carries`) had
reworked the chart while it was in flight:

- `PoseDef::FaceChart` stands only bare in a profile's frame seat, and
  refuses `PoseConstruction::FaceChartUnseated` anywhere else
  (`crates/editor-core/src/pose.rs`).
- It decides nothing where it is read. Its least turn's pole band is
  decided at each **carriage** of what stands in it (the profile's
  loops, each sweep of it, the faces it traces), levered over that
  thing's own reach. `FaceChartUnleverable` carries the
  `mate::LeverRefusal` of that lever (`mate::value::FaceChart`,
  `mate/solve.rs`).

A datum folds its bundle in its own evaluation, with a reach of 0 (it
copies nothing). Its pole verdict is therefore levered over the face's
and the slides' distance from the origin alone. That is exactly the
"levered over none of what it carries" defect 7151 fixed: a frame that
charts at a tilt where a wide boss on it should refuse. So a fold onto
today's datum would regress 7151.

Options, for the designer pass:

1. **A datum's pole is decided at each reader** over what that reader
   carries (7151's carriage rule, generalised). A mirror or a mate
   reading a datum carries no loops, so what its lever is there is the
   open question. This touches `[placement.datums]`.
2. **Fold, dropping the carriage semantics**: the datum levers over
   reach 0 as it does now. This regresses 7151's fix and its rows
   (`face_chart_and_signed_depth`), and is not recommended.

Until this is settled, draw on a face stays on `PoseDef::FaceChart`, and
a datum is authored only by New plane / New axis / New frame
(formwords/datum).
