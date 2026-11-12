---
id: a-mixed-ring-inside-a-few-wide-arcs-refuses-at-the-support-meter
kind: issue
title: A mixed rim that is a ring of a face bounded by a few wide arcs refuses at the support-boundary meter
status: open
opened: 2026-11-12
priority: P2
cost: H
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
