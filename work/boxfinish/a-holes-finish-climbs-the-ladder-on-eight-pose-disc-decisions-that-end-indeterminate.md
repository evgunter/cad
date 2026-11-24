---
id: a-holes-finish-climbs-the-ladder-on-eight-pose-disc-decisions-that-end-indeterminate
kind: issue
title: A hole's finish over a box climbs the symbolic ladder on eight curve_face_pose_disc decisions that each end indeterminate: a second of every hole finish, and most of the tour's leaf time with the switch off
status: open
opened: 2026-11-24
priority: P1
cost: M
---


Found by M10 (the probe lane, 2026-11-24; `probe/M10-REPORT.md` on
`boxfinish/probe-switch-off` at `7901e1b078`), on main after PR 7394
with the switch's box arm off.

**What is measured.** On the tour's certified study, each hole
extrude's finish used to refuse at check 1 in about 40 ms (the `u_ref`
length, gone since PR 7384). It now builds in every leaf, at 520 ms
average: 2,204 hole finishes are 1,146 s of the drive's 1,304.9 s of
summed leaf time (M9: 177.6 s). Traced, the time is on the decision
path: `curve_face_pose_disc`, eight decisions per hole finish, about
1.0 to 1.3 s of a traced finish, **every one indeterminate after a
ladder climb** of 12,000 to 28,000 forms (the largest 173 terms). On
the two-parameter plate a refusing finish spends 263 ms of 998 ms
there.

**What is not known.** Why the numeric channel leaves these eight
undecided over the study's box (so that the tier is asked at all), and
whether the climb could ever prove one: a ladder that ends
indeterminate on every one of 17,600 asks is paying for an answer the
shape cannot have. Which of the two is the defect (the reading that is
indeterminate, or the climb's price on a question it cannot close) is
the first thing a lane measures.

**What it holds.** The switch's deletion: with the box arm off the
tour's first drive takes 328 s of wall time. Three quarters of the
drive's time is on node keys already seen (967 s of 1,304.9 s), which
the per-drive store
(`a-drive-keeps-no-node-result-from-one-leaf-to-the-next`) serves once
it exists; this row is what one such finish costs when it does run.
(BOXFINISH orchestrator)
