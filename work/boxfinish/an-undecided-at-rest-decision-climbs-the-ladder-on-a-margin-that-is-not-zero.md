---
id: an-undecided-at-rest-decision-climbs-the-ladder-on-a-margin-that-is-not-zero
kind: issue
title: An undecided at-rest decision climbs the symbolic ladder on a margin that is not zero: three quarters of the tour's leaf time with the box arm off
status: dispatched
opened: 2026-11-24
priority: P1
cost: H
design: true
branch: boxfinish/refute-first
---


Found by M11 (the probe lane, 2026-11-24; `probe/M11-REPORT.md` on
`boxfinish/probe-switch-off` at `e37099a49a`), on main after PR 7394
with the switch's box arm off.

**What is measured.** Over the tour's first drive (1,023 replays,
2,128 hole finishes at 523 ms), four predicates are left undecided by
the numeric channel and each then asks the symbolic tier, which climbs
its ladder and ends not-zero every time:

| predicate | undecided | ladder time, whole drive | per slow climb |
|---|---|---|---|
| `curve_carrier_clear` | 26,948 | 682.6 s | 3,117 climbs, 211 ms, 17,050 forms |
| `side_at_infinity_winding` | 11,436 | 167.2 s | 2,743 climbs, 57 ms |
| `curve_face_pose_disc` | 1,760 | 100.0 s | 880 climbs, 113 ms |
| `curve_carrier_gap` | 9,710 | 33.6 s | 620 climbs, 26 ms |

Together 983 s of the drive's 1,306 s of leaf time. None of the four
is ever decided `Zero` at the box scalar, and none is zero: the `f64`
witness is definite on each and every sub-box narrow enough to decide
decides the same way. A slow climb walks the plain form, the early
walk and the top reduction, then both retries the session offers;
every stage ends in a small form that is not zero, with no budget stop
and no poisoned form. It stops because it has run out of attempts.

**Skipping the climb changes no verdict** (scratch: an undecided
decision of a named predicate returns the numeric answer and asks
nothing): 211 certified, 301 refused, 511 splits each way. With all
four skipped the hole finish is 219 ms (−57%) and the remainder moves
to `azimuth_near_cut_side` (151 ms, all undecided): the cost is the
forms the undecided margins share, paid by whichever asks first.

**Two things are wrong, and this row is the second.** The first is the
readings: four margins that are definitely nonzero read as straddling
zero over a 0.1 mm box
(`a-holes-finish-climbs-the-ladder-on-eight-pose-disc-decisions-that-end-indeterminate`).
The second holds wherever a margin does straddle, for any reason,
which a wide box always can: the tier is asked a question whose only
answer is `Zero`, of a margin that is not zero, and pays the whole
ladder to say nothing. E12 (`docs/ERROR-DESIGN.md`) has a decision
read the value first and ask the tier only where the value reads no
definite sign; it does not say what an ask costs when the answer is
no. What an undecided at-rest decision should pay there is a design
question, weighed by designers before a lane builds it.

It holds the cost row
(`tier-3-at-every-box-replay-leaf-multiplies-the-decisions`).
(BOXFINISH orchestrator)
