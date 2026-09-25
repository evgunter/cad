---
id: circle-lowers-to-one-segment
kind: unit
title: circle and circle_split(n ≥ 1) lower to one full-turn segment per arc; lily migrates; re-baseline
status: parked
opened: 2026-09-25
priority: P1
cost: D
parent: lower-profiles-to-carrier-and-interval-not-vertex-and-bulge
blocked_on: [one-segment-loop-through-builders]
---


Unit 4 of the #3218 lowering. `circle(c, r)` lowers through `circle_split`'s kernel with n = 1, phase 0, and stays its own verb in the program (Ev, #3218 q3). `CircleSplitCount` refuses n = 0. The anchor's n = 2 paragraph goes. `lily::foot` drops its `circle_split(3)` workaround; bossplate and twopeg keep theirs, which are deliberate. Goldens, censuses, names and Python pins re-baseline. EMIT's `Piece(0)/Piece(1)` circle names become one `Carrier` (settled on #3202: second names break). Loft over a circle waits on TESS's `lofted-circle-sections-are-unmeshable…`.
