---
id: the-support-test-reads-an-analytic-cell-by-its-box-so-the-klein-bulbs-walls-run-out-of-pairs
kind: issue
title: The support test reads a cone, torus, cylinder or sphere cell by its axis-aligned box, so the Klein bulb's inner and outer walls, 0.05 apart, split until the injectivity check runs out of its 65,536 pairs
status: open
opened: 2026-11-12
priority: P0
cost: M
---


## Finding

From CLEAR's lane clear-pairs
(`the-certificate-runs-out-of-pairs-on-eight-finished-tour-bodies`,
its Diagnosis). `klein_bulb` (12 faces: cylinder, torus, cone, torus,
cylinder and two annular caps, each run twice, once per wall face,
`WALL` = 0.05 apart) refuses `Budget(Pairs)` at depth 24. The
frontier is the distance arm, not the departure arm:

| pair | kinds | splits |
|---|---|---:|
| `6v1`–`12v1` | cone, outer wall against inner | 43,159 |
| `7v1`–`11v1` | torus, outer against inner | 21,779 |
| six more pairs of opposite walls | | 700–2,600 each |

These faces share nothing, so only the distance test decides them.
At depth 24 a cone cell is `0.049` rad by `0.09` m of slant, about
0.12 m across at the cone's wide end: more than twice the gap, and
the boxes of two parallel slanted patches still overlap.

DIRSUPPORT's second compare, `separated_by_support` (`clearance.rs`),
would separate them at cells of about `√(gap × radius)`, but its
`Hull::of` reads a support along a direction only off a plane's
first-order expansion or a spline chart's restricted net. Every other
carrier (cylinder, cone, sphere, torus) falls to `Hull::Box`, the
cell's box, so on an analytic wall the support test is the box test
again. `distance-test-separates-boxes-not-directions` (closed) was
measured on the Klein LOOP, whose walls are NURBS.

## What a fix needs

A support along a direction for an analytic cell: the mean-value
form of `⟨P, n⟩` over the cell, `⟨P(m), n⟩ + ⟨S_u, n⟩·[−h_u, h_u] +
⟨S_v, n⟩·[−h_v, h_v]`, with the partials enclosed over the whole cell
(`rect_jet`). Along the cell's own normal the two partial terms are
second order in the cell, which is what makes the support tight.
Pin: the Klein bulb (or a two-cone wall fixture of the same gap)
holds within the default budget.

Seam: `Hull` is beside lane A's `Sweep::run` and `min_separation`
(`clear/verdict-door`).
