---
id: a-declared-seam-arrival-reads-its-turn-over-the-first-leg-and-names-it-the-arriving-legs
kind: issue
title: profile: the declared seam arrival levers its turn by the first leg too, so a real 64° arrival closes tangent and the off-direction refusal names the first leg's arm as the arriving leg's
status: open
opened: 2026-11-21
priority: P2
cost: M
refs: [a-real-junction-on-a-short-leg-refuses-as-tangent, a-real-near-reversal-on-a-short-leg-refuses-as-a-cusp]
---

(PROFILE lane, the sibling-arm audit asked by the review of PR 7260,
`profile/junction-turn-names-its-factors`.)

## Finding

`path.rs` `seam_arrival_check` (the `Start.arrives_tangent()` check) has
two parts:

- (i) It gates the ARRIVING leg's arm (`path_seam_arrival_lever`,
  `PathError::SeamArrivalLeverTooShort`).
- (ii) It reads the turn on `arriving_arm.min(leaving_arm)`, where the
  leaving arm is the entry's FIRST leg.

So a short first leg passes (i) and collapses (ii).

Measured at `Tol::witness()` (ε = 1e-9). The loop is a first leg of
`k·ε` along +x continued straight on by `.line(1.0)`, real corners at
`(1+kε, −2)` and `(−cos 64°, −sin 64°)`, then
`.line_to(Start.arrives_tangent())`: a metre-long closer arriving at
64° off the first side. Validation is `ConstructedProfile::validate`.

| k | door | validation |
|---|---|---|
| 0.3 | **closes**: the seam joint is constructed tangent at a real 64° turn | refuses segment 0 as degenerate |
| 1.05 | **closes**, constructed tangent | escalates on segment 0's length |
| 3 | escalates, `path_seam_arrival_turn` | — |
| 20 | `SeamArrivalOffDirection`: "misses by -1.798e-8 m over the arriving leg's 2e-8 m arm" | — |
| 50 | `SeamArrivalOffDirection`: "… over the arriving leg's 5e-8 m arm" | — |

There are two defects.

1. The door accepts a declaration it cannot check and constructs a
   tangent joint where the curve turns 64°. Today only validation's
   segment-length gate stands between that and a stored loop, and it
   stands there for a different reason.
2. `SeamArrivalOffDirection`'s sentence and its `arm` field doc say
   "the arriving leg's arm", but the arm is the shorter of the
   arriving leg and the first leg. At k = 20 the arriving leg is about
   1.2 m and the stated 2e-8 m is the first leg's.

## Shape question

One option is for (i) to gate the arm the turn is actually read on
(the shorter of the two), so `SeamArrivalLeverTooShort` answers on a
short first leg too. Its sentence would then name which leg. The
other is for the arrival check to carry a `LeveredTurn`, as the
undeclared seam now does. The two are not exclusive. Either way the
off-direction sentence must stop naming the arriving leg when the
first leg is the shorter.

The other sibling arms were checked too. `JunctionCusp` is filed
(`a-real-near-reversal-on-a-short-leg-refuses-as-a-cusp`).
`SeamArrivalLeverTooShort { arm }` states only the arriving arm, which
is what it gated. `ContinuationTargetOffRay { across, along }` is an
unlevered point miss. `CuspDoublesBackOverLeg { at }` carries no
margin. None of those three asserts a fact a short lever can falsify.

