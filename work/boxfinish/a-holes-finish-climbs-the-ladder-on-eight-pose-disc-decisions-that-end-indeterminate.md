---
id: a-holes-finish-climbs-the-ladder-on-eight-pose-disc-decisions-that-end-indeterminate
kind: issue
title: A hole's finish over a box reads four margins that are definitely nonzero as straddling zero (a curve's clearance from its carrier, a winding, a pose discriminant, a carrier gap), and each then climbs the symbolic ladder
status: open
opened: 2026-11-24
priority: P1
cost: M
---


Found by M10 and corrected by M11 (the probe lane, 2026-11-24;
`probe/M10-REPORT.md` and `probe/M11-REPORT.md` on
`boxfinish/probe-switch-off`, the second at `e37099a49a`), on main
after PR 7394 with the switch's box arm off. The id keeps M10's
headline, which was a sample of the drive's first 100 s (its widest
leaves) reported as the finish; M11 measured the whole drive.

**What is measured.** On the tour's certified study each hole
extrude's finish builds in every leaf at 523 ms (2,128 finishes; it
refused in 40 ms before PR 7384). Over the whole first drive (1,023
replays):

| predicate (home) | decided / undecided | the undecided reading | witness at `f64` |
|---|---|---|---|
| `curve_carrier_clear` (`geom::curves::on_carrier`, `Foot::on_carrier`) | 143,058 Positive / 26,948 | `[0, +inf]`, decorations `Trv`, `Def`, `Com` | 0.32 to 8.56 mm, all Positive |
| `side_at_infinity_winding` (`solid_contain`, `turned_inside`) | 17,208 definite / 11,436 | `[-0.0083, 0.0303]` `Trv` | ±0.024227, never 0 |
| `curve_face_pose_disc` (`geom_brep::curve_face::pose::discriminant`) | 15,196 Positive / 1,760 | `[-0.0013783, 0.0013783]` `Def` | 0.000489268 |
| `curve_carrier_gap` (the first's door) | 8,344 Positive / 9,710 | `[0, 0.001672]` `Com` | none taken |

None is in band and none is zero: every sub-box narrow enough to
decide decides as the witness does. The finish does not refuse on
them: the aimed ray is set aside (8,400 times against 1,057 sides
read over the drive) and another ray or the volume reads the side, so
every hole finish still ends `ok`.

**Why the pose discriminant is wide** (measured at the root replay):
`a` is exact and the radius carries the box's own 6e-5, but the ray's
offset from the wall's axis reads 5.6 mm wide over a box 0.1 mm wide
(`shift = [-3.13e-3, 2.49e-3]`, decoration `Def`), about 56 times the
box, so the discriminant straddles. That the box enters the offset
more than once, through a quotient, is inferred from the width and the
decoration; it is the shape PR 7480 removed from the turn and the
aim's step, here in a third reading, and
`a-walls-aim-hangs-from-its-moving-edge-so-its-ray-reads-a-box-a-tenth-as-wide`
is the nearest open row.

**Why the clearance reads `[0, +inf]`:** an operation inside it lost
its domain over the box (the `Trv` decoration is measured; which
operation is not traced).

**The a0 plate has no such decision undecided** (its eight
discriminants read a point, Positive, with no ladder asked): the
defect needs a hole whose ray's offset moves with the box.

**What it costs** is the other row,
`an-undecided-at-rest-decision-climbs-the-ladder-on-a-margin-that-is-not-zero`:
983 s of the drive's 1,306 s of leaf time is the tier climbing on
these four.

**First step for a lane:** on main with PRs 7370 and 7480 merged,
re-take the table (M12), then trace the operation that loses its
domain in the clearance and where the offset takes the box twice.

**What it holds.** The cost row and through it the switch's deletion.
(BOXFINISH orchestrator)
