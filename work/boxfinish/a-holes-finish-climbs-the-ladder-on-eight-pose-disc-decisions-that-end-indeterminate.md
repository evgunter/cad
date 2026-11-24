---
id: a-holes-finish-climbs-the-ladder-on-eight-pose-disc-decisions-that-end-indeterminate
kind: issue
title: A hole's finish over a box reads four margins that are definitely nonzero as straddling zero (a curve's clearance from its carrier, a winding, a pose discriminant, a carrier gap), and each then climbs the symbolic ladder
status: dispatched
opened: 2026-11-24
priority: P1
cost: M
branch: boxfinish/wall-roots
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

**M12 (2026-11-24, on main with PRs 7370 and 7480; `probe/M12-REPORT.md`
on `boxfinish/probe-switch-off` at `0a6d17f556`).** Neither PR moved
the table (the hole finish 517 ms; the drive's leaf time 1,346 s; the
same verdicts and counts), and the two untraced readings are traced to
ONE place:

- **The roots of a ray on a cylinder wall read `(−inf, +inf)`.**
  `geom_brep::quartic::quadratic_roots(a, b, c, disc)` spells the root
  whose numerator cancels as `c` over the other numerator, choosing by
  the sign of `b` through `select_le_zero`, a value selection that
  keeps both arms where `b` straddles zero. Its doc says neither
  denominator nears zero there ("it is `√disc` plus a sliver"). It
  does: one traced case has `b = [-9.48e-5, 1.10e-4]` and
  `√disc = [6.66e-5, 2.48e-4]`, so `√disc − b` and `−b − √disc` both
  contain 0, and `c / plus`, `c / minus` are unbounded. 320 of 320
  unbounded crossing parameters traced are a wall face's.
- **`b` straddles because the wall quadratic about its nearest point
  loses a cancellation.** `line_quadric::wall_quadratic` at
  `About::Nearest` forms `shift = clamp(−(w0p·dp)/a2, ±span)`,
  `w = w0p + dp·shift`, `b = w·dp`: where the clamp does not bind `b`
  is zero by construction (0.0 at `f64`), and over the box it reads
  `[-1.69e-4, 2.28e-4]`, because `w0p` enters `w` twice. The ray's
  start is itself 6 to 12 times the box before the wall is read (the
  aim's step and reach: PR 7480's follow-up rows).
- **Three of the four margins follow from it:** the clearance reads
  `[0, +inf]` because its seed `q + d·t` is unbounded (measured); the
  winding's angle is `[-π, π]` because the crossing point is hundreds
  of metres wide in the wall's chart (inferred from the same root);
  the gap likewise (inferred). The discriminant is `b² − a·c`, and with
  `b` at zero it is `−a·c`, whose `c` is definite here.

**The unit** (`boxfinish/wall-roots`): the roots' spelling where `b`
straddles zero, and the coefficient that is zero by construction at
the nearest point, each fixed where it is made; measured first, with
the four margins' table re-taken on the box-arm-off tree as the
acceptance.

**What it holds.** The cost row and through it the switch's deletion.
(BOXFINISH orchestrator)
