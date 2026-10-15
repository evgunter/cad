---
id: a-near-tangent-coplanar-circle-pair-places-its-touch-off-both-circles
kind: issue
title: carrier_cross misplaces circle-pair candidates near the band (common_line for planes tilted under ~3e-4 rad; parallel_circles' touch arm at d ≈ Kε), so boundary_crossing reads Clear where the edges cross
status: open
opened: 2026-10-13
priority: P1
cost: M
branch: cleave/circle-pair-touch-on-both
---


## The finding

Measured by a FUSE designer lane weighing `work/fuse/two-curve-curve-meeting-finders.md`. The probe ran in a throwaway worktree and was never committed.

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

FUSE's curve-meeting design (`work/fuse/two-curve-curve-meeting-finders.md`, now with Ev) would absorb these arms. **This defect is silent today, so fix it now, independently of that design.**

## Built (branch `cleave/circle-pair-touch-on-both`)

Re-measured on main after PR 5058 with a 50-digit oracle at ε = 1e-9,
1e-6 and 1e-12. Every witness above survives 5058: its reorder and its
`Conic` input left the touch arm's foot and both `common_line` arms as
they were. The row's literal witness (`Δr = 0.9·d`, `d = Kε`) sits on
the zero threshold (`d − Δr = ε`), so it reads `Zero` only at 1e-9;
`Δr = 0.95·d` reproduces at all three. `common_line` drifts at 1e-9 and
1e-12 and holds at 1e-6. A third arm drifted too: the in-plane
crossings took their direction from the 3D centre offset, which a
`Zero` plane reading leaves up to the band.

- `conic_meets_plane` replaces `common_line`, `circle_meets_plane` and
  `ellipse_meets_plane`: each conic meets the other plane in its own
  angle, from `lift = n·(c_e − c_f)`, with the same rows and the same
  gap from tangency, now read from `lift`. A `Zero` touch is the
  clamped double root on the conic.
- `parallel_circles` reads `geom_core::Circles` in `e`'s plane, `f`'s
  centre projected into it: the real crossings wherever the half-chord
  exists, else the point of `e` on the line of centres.

The review (APPROVE-WITH-FIXES) found the gap's own rounding unread:
`x`, `y` and `lift` are dot products nearly square to `n`, each about
`u·(r + |Δc|)` off, and the gap that over the tilt. That rounding is
past Kε under a tilt of about `3e-5` at ε = 1e-12, where the gap read
bare took its sign: 47 of the review's 600 near-tangent poses read a
definite miss where the records cross. The gap now carries a running
bound (`geom_core::running`), and where `K` times the bound is past the
band's zero (new row `bool_carrier_cross_meets_resolution`):

- the gap is read as the interval the bound spans
  (`geom_brep::roots::extreme_sign`), and a definite sign there stands;
- a straddle escalates typed.

On the branch's own 601-pose family, against a double-double oracle on
the stored records, that is 24 definite misses at 1e-12 before and 0
after, and 0 and 0 at 1e-9 and 1e-6.

Placing the meeting accurately rather than charging it would need
compensated dot products at every scalar `T`, which is the
curve-pair reader's ground. The meeting that reader takes across
planes is the closest pair, which this arm does not read (evidence on
`fuse/the-curve-pair-reader`).
Pinned by `carrier_cross::meetings_rows::every_arm_places_its_meetings_on_both_curves`,
two `crossing_rows` (`a_section_turned_off_the_rims_plane_…`,
`a_nearly_concentric_circle_in_the_rims_plane_…`) and the boolean
row `sweep::a_rim_tilted_across_another_on_one_wall`, which main
refuses at 1e-12. No door reaches the coplanar witness: two circles
the band reads coplanar but not one curve leave a band-thin region
between their planes, which the boolean refuses elsewhere.
