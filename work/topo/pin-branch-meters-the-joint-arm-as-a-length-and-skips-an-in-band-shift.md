---
id: pin-branch-meters-the-joint-arm-as-a-length-and-skips-an-in-band-shift
kind: issue
title: pin_branch gates the period shift on the joint arm (m/rad) metered as a length, so a lever in (eps/|gap|, K eps] skips an exact shift and continuity refuses
status: open
opened: 2026-10-04
priority: P4
cost: E
refs: [a-kill-that-re-anchors-a-loops-first-leaves-its-rows-a-period-off-the-pass]
---


(TOPO orchestrator, from the probe of kill-kept rows against tier 3. The
report is `probe-report.md` on `analysis/probe/topo-kill-rows-tier3`, base
`0edbbecb`.)

## What

`pin_branch` gates the whole-period shift on `Margin::of(joint_arm.magnitude())`
(`crates/topo/src/pcurves.rs:3624-3628`). That meters the arm (metres per
radian) as a length. Continuity then meters arm·|gap| (`:3667-3671`). So for a
lever in (ε/|gap|, Kε] the shift is skipped, and continuity refuses with
`Discontinuity`, although the shifted branch is exact.

It was reproduced with a temporary unit test that calls `pin_branch` on a cone
(α = 0.5) with the joint at v:

| v | outcome |
|---|---|
| ≤ 1e-10 | fits |
| 5e-10, 1e-9, 2e-9 | `Escalated` |
| 5e-9, 1e-8, 2e-8 | `Discontinuity` |
| ≥ 3e-8 | shifts and fits |

It errs toward refusing and never certifies wrongly. The comment at
`:3600-3620` accepts that ("the skip defers; the margins decide"). There were
**0 live hits**: all 21,566 live skips in sweep's `ci` profile had an arm below
1e-12 or equal to 0, and all fit.

## Fix

Gate the skip on the quantity continuity meters, arm·|gap|, for the whole
period about to be skipped, instead of on the arm alone. Then a skipped shift is
one continuity would also have passed. Pin it with the cone row above:
v = 5e-9 must shift and fit.
