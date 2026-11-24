---
id: a-wall-crossings-chart-point-reads-tens-of-metres-wide-with-its-root-bounded
kind: issue
title: With a wall crossing's root bounded, the winding about it still reads its chart point tens of metres wide, so side_at_infinity_winding stays undecided on a boxed hole
status: open
opened: 2026-11-24
---


Found by `boxfinish/wall-roots` (the unit
`a-holes-finish-climbs-the-ladder-on-eight-pose-disc-decisions-that-end-indeterminate`),
measured on the box-arm-off probe tree with that branch merged in. The
unit's row inferred that the winding's lost angle followed from the
unbounded wall root. It does not.

**Measured, the tour's first drive (1,023 replays).** Before → after the
roots were bounded: `side_at_infinity_winding` undecided 11,212 →
11,536, ladder time 200 → 361 s (the clearance, which no longer climbs,
used to pay for the forms the two share). The undecided reading did not
move (`[-0.0070, 0.0633]` `Trv`, the most common). No crossing parameter
is unbounded any more (the probe's `[m12-t]` print: 0 lines), and yet
15,280 turns still hold an angle of `[-π, π]` `Trv` (23,194 before).
One case, face 3v1 (the wall): `p = ([-67.1, 79.7], [-4.36, 4.36], 0)`
in the wall's chart, so `cross = [-745, 745]` and
`dot = [-5470, 6494]`. The point is tens of metres wide over a box of
0.1 mm with its root a few millimetres wide. Which operation between the
root and `solid_contain::loop_turn`'s chart point widens it is not
traced.

**A small fixture that shows it at `Interval`** (not committed; the
lane's scratch): a plate `[0, 4]² × [0, 1]` with a round hole about
`(2, 2)`, extruded at `Interval` by the extrude door, then
`topo::transform_rigid` by `RigidMotion::rotation(exact_z, 0.3 ± h)`.
At `h = 1e-5` and radius 0.5, `topo::test_support::each_aim` sets both
hole-wall rays aside on `side_at_infinity_winding`, with margin
`[-0.36, 19.8]` after the unit and `[-10.2, 3.4]` on main; every
`h ≤ 3e-6` reads `Side(Out)`. At radius 1.25 mm the wall rays are set
aside on `side_at_infinity_winding`, `curve_face_pose_disc` or
`side_at_infinity_denom` at every `h` from 1e-6 to 3e-5, on main and
after the unit alike.
