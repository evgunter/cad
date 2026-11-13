---
id: a-mixed-ring-inside-a-few-wide-arcs-refuses-at-the-support-meter
kind: issue
title: The support-boundary meter reads an arc or circle across a mixed link's windowed strip as straddling it, though it clears
status: closed
opened: 2026-11-12
priority: P1
cost: H
closed: 2026-11-12
pr: 6887
refs: [an-annulus-rim-around-a-mixed-rim-refuses-in-one-call]
---

Filed by the band-mixgates lane (2026-11-12).

`surgery::support_boundary_clearance` (`crates/sweep/src/blend/surgery.rs`) meters each piece of a mixed rim's support boundary through `StripLevel::clearance`. That takes the piece's range across a link's strip and its range along each window half-space separately, and the piece clears only if one of those ranges does.

A wide arc that passes the strip at a distance clears neither range. Its sweep across the strip runs from beside it to beyond it, and its sweep along the window runs through it.

Witness: a disc of radius 1.5, authored as TWO arcs and 0.5 thick, with the rounded rectangle `[−1, 1] × [−0.6, 0.6]` (corner radius 0.3) through it.
- The outline's two rims alone, filleted at 0.05, refuse `RingClearance` at margin −0.30, as does a four-arc disc. The disc's arcs come no nearer than 0.45 to any strip.
- Authored as eight arcs, the disc's outline builds.
- Reproduced on the lane's branch. The disc's arcs are stored edges, not requested ones, so the branch's change to the co-requested read does not reach them.

The fix reads a circle piece against the strip's REGION, its windowed slab or annular wedge, rather than against each half-space alone. That needs a certified distance from a circular arc to a rectangle, or to an annular sector, at the scalar.

**Widened (review of PR 6865, band-gates-r1).** The same ranged read misfires on a ladder hole beside a rounded corner. Each hole below is geometrically clear of every strip; every edge is filleted at 0.05. The read is non-monotone: a true intrusion at (0.851, 0.25) reads −0.001.

| hole | read |
|---|---|
| r 0.1 at (0.559, 0.159) | −0.0086 |
| r 0.1 at (0.6, 0.2) | −0.041 |
| r 0.05 at (0.845, 0.25) | −0.0034 |
| r 0.05 at (0.849, 0.25) | −0.0072 |

The hole's trim circle passes an arc link's wedge, or a straight link's slab end. PR 6865 keeps that false margin off the user: a ladder ring beside a mixed rim that does not read clear refuses `UnsupportedChain` with the sequential recourse, and the outline-first sequence builds at the closed form (`band_mixed_rim_shared_supports::holes_beside_a_corner_refuse_with_the_sequential_recourse_which_builds`). This row's fix turns those four into one-call builds.

**Priority P1.** A two-semicircle loop is how this repo authors a circle. For such a disc, even the gate's own sequential recourse is false: the outline alone refuses at −0.30.

**The exact read.** Clip the piece to the window's half-planes first: a circle against a half-plane is an arc, φ ± acos(−k/A). Then read the level over the clipped arcs, each exact by `piece_distance` / `piece_along`. A clipped arc is connected, so its level range overlapping the strip's means it meets the strip; the read is then exact both ways. Escalation is at the clip angles' acos near ±1.

## Resolution

`StripLevel::wedge_margin` clips a line or circle piece to each wedge
(`within`, `arc_within`) and reads the level over the parts inside, as
proposed above. Before → after, every edge filleted at 0.05, at `f64`
and `Interval`: the two- and four-arc discs' outline rims, −0.30 →
build at Pappus; the four holes beside a corner, the sequential
recourse → build in one call at the sequential volume; the true
intrusion at (0.851, 0.25), the sequential recourse → `RingClearance`
at −0.001. A clip angle in band is a touch, kept as its enclosure and
escalated only where it decides the read. PR 6865's `beside_rim`
fallback is retired: no case needs it.

