---
id: a-near-tangent-coplanar-circle-pair-places-its-touch-off-both-circles
kind: issue
title: carrier_cross misplaces circle-pair candidates near the band (common_line for planes tilted under ~3e-4 rad; parallel_circles' touch arm at d ≈ Kε), so boundary_crossing reads Clear where the edges cross
status: closed
closed: 2026-10-14
branch: fuse/curve-pair-reader-c
opened: 2026-10-13
priority: P1
cost: M
---


## The finding

Measured by a FUSE designer lane weighing `two-curve-curve-meeting-finders` (closed, PRs 5154, 5162 and 5227). The probe ran in a throwaway worktree and was never committed.

The fault is in `carrier_cross::parallel_circles` (`crates/topo/src/boolean/carrier_cross.rs`), in the touch branch `if apart == Sign::Zero || nested == Sign::Zero { return Ok(vec![foot]); }`. It places the touch with the generic radical-line foot `c1 + e·(d² + r1² − r2²)/2d`. That formula multiplies the decision's residue (`d − |r1 − r2|`, or `|r1 + r2| − d`) by `(r1 + r2)/2d`, which is unbounded as `d → 0`.

- **Reproduction:**
  - the witness band (zero `1e-9`, escalate `1e-8`);
  - two coplanar circles: a unit circle at the origin, and a circle at `(1e-8, 0, 0)` of radius `1 + 9e-9`.
- **Expected:** the circles pass within `1e-9` of each other at `(−1, 0, 0)`, so the touch candidate should land there.
- **Got:** `nested` reads `Zero` and the candidate lands at `(−0.9, 0, 0)`, about 0.1 m off both circles. The confirm step then reads it off, so the boolean's crossing layer reports no meeting.
- **Cross-check:** the census walk (`census::curved::reads_meet`) on the same pair answers `Some(true)`.
- **Who can reach it:** two rims of nearly equal radius in one plane, for example a thin washer's caps on a shaft.

The same probe found no drift in the two-crossing arm (residuals ≤ 5e-13 over 106 rows) or in the tilted common-line arm (no drift down to `s = 1e-8`).

## What it needs

Place a candidate admitted under a `Zero` reading with the exactly degenerate formula, not the generic one: `foot = c1 + e·r1` when `apart == Zero`, and `c1 + e·r1·sign(r1 − r2)` when `nested == Zero`. That point lies exactly on the first circle and within the residue of the second. A ci row should pin the reproduction above.

Sweep the other closed-form arms for the same class: a generic formula evaluated at data a `Zero` reading has just called degenerate. The class is also a contract clause in the FUSE curve-meeting design (`two-curve-curve-meeting-finders`).

## A second measurement: `common_line` and the coplanar crossing arm

A second designer lane ran its own probe, also never committed: 2000 random pairs per row at ε = 1e-9, Kε = 1e-8. Its results widen the row above.

- **Tilted planes (`common_line`).** Two circles cut from one sphere, centred at (3.7, −2.1, 0.4), with their planes tilted by θ. For each θ: the worst distance from a candidate to a circle, and how many of the 2000 candidates sit more than ε off.

  | θ (rad) | worst candidate distance | candidates > ε off |
  |---|---|---|
  | 1e-5 | 1.8e-6 | 1980 |
  | 1e-6 | 3.3e-4 | 2000 |
  | 1e-7 | 2.3e-2 | 2000 |

  - The cause is that `common_line` divides by sin²θ after a subtraction that cancels.
  - On the same pairs, `splitting::conic_plane_candidates`, which solves in the circle's angle, stays within about 1e-9.
  - A candidate that misses its meeting fails the boundary pre-pass, so `boundary_crossing` answers **`Clear` while the edges cross: silent**. A sphere or cone with two cuts tilted less than a milliradian apart reaches this.
- **Coplanar crossing pairs (`parallel_circles`).** At d = 1e-8 = Kε, 92 of the 2000 definite answers are wrong.
  - Example: d = 1e-8, Δr = 0.9·d. The circles cross at (−0.9, ±0.436). `nested` = d − |Δr| reads Zero, so the arm returns one "touch" at (−0.9, 0), 0.44 m from both crossings.
  - Near-concentric circles stay within d + |Δr| of each other all the way round, so a centre-line margin that reads in band does not tell where they touch.
  - Above d = 3e-8 every definite answer is exact, to about 1e-15.

**What the fix needs, revised:**
- Route the conic×plane arms through the harmonic form in the conic's own angle. `conic_plane_candidates` already does this to about 1e-15.
- Decide a double root by the gap between its two roots along the curve, in metres, not by a centre-line margin.
- Rows: one red-then-green row per reproduction above.

FUSE's curve-meeting design (`two-curve-curve-meeting-finders` (closed, PRs 5154, 5162 and 5227), now with Ev) would absorb these arms. **This defect is silent today, so fix it now, independently of that design.**

## Closed — `carrier_cross` reads its meetings from `geom_brep::curve_pair::meet` (fuse/curve-pair-reader-c)

`parallel_circles`, `common_line` and `circle_meets_plane` are gone with
`carrier_cross::meetings`. PR 5058 left the touch branch's radical-line
foot in place; it only moved the one-circle decision onto
`one_curve_gap`. `boundary_crossing` now takes every meeting from
`meet`. There:

- **In one plane**, two circles' distance is a first harmonic in the
  first circle's angle with exact metre extremes, a double root is
  decided by the chord between its roots, and a touch is the extreme's
  own point, on the first circle exactly.
- **Across planes**, a crossing of the other's plane is solved in the
  conic's own angle and polished to the carriers' closest pair under a
  basin meter.
- **Rows:**
  - the reproduction and three neighbours,
    `geom_brep::curve_pair::meet_rows::a_near_tangent_coplanar_pair_is_placed_on_both_circles`;
  - inside, outside and negative-radius tangency,
    `a_decided_coplanar_touch_lies_on_both_circles`;
  - the tilted common-line table's poses,
    `circles_on_one_sphere_meet_on_both_at_any_tilt` and the oracle's
    tilted family.
- **Where the pose sits below the tangency floor** it refuses typed
  rather than placing a candidate
  (`work/cpair/curve-pair-polish-meters-refuse-shallow-meetings.md`),
  and `carrier_cross` answers `Unread`. The caller keeps its frontier
  door rather than reading `Clear`.

The row's second ask, a sweep of the other closed-form arms for a generic formula at data a `Zero` reading called degenerate, is not done here. It lives on `work/issues/decided-tangent-point-is-the-radical-foot.md`, with the two arms still of that shape (`splitting::conic_plane_candidates` and `rim_wedge::crossings`).

## The PR 5190 review's M-1 family through `curve_pair::meet` (2026-10-19)

PR 5190 (closed unmerged, superseded by the reader) was reviewed with a
near-tangent family that found silent misses in its own `conic_meets_plane`
at ε = 1e-12. The same family, run through `geom_brep::curve_pair::meet` on
main (f6dbcfe1f7):

- **The poses (601):** two sections of the unit sphere about
  `(3.7, −2.1, 0.4)`; their planes share a line `g` short of tangency to
  the first, with `g ∈ ±20·z`; turned `θ` apart about it, 30 log steps of
  `θ` from `1e-11` to `1e-3` by 20 gaps; plus the review's witness
  (`θ = 1e-6`, crossings 16.6 µm apart).
- **The method:** whole-circle spans, both orders. A double-double
  oracle on the stored records (`reach² − lift²`, each product exact)
  says where each conic crosses the other's plane, and a pose counts
  where both do.
- **Silent misses** (a definite no-meeting where the records cross):
  none at any ε.

| ε | readings that cross | meeting found | shared run | refused typed (`Escalated`) |
|---|---|---|---|---|
| 1e-12 | 636 | 606 | 0 | 30 |
| 1e-9 | 590 | 394 | 126 | 70 |
| 1e-6 | 610 | 160 | 370 | 80 |
