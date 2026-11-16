---
id: the-support-test-reads-an-analytic-cell-by-its-box-so-the-klein-bulbs-walls-run-out-of-pairs
kind: issue
title: The support test reads a cone, torus, cylinder or sphere cell by its axis-aligned box, so the Klein bulb's inner and outer walls, 0.05 apart, split until the injectivity check runs out of its 65,536 pairs
status: closed
opened: 2026-11-12
priority: P0
cost: M
closed: 2026-11-13
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

## Closed (2026-11-13)

- **Diagnosis confirmed.** On main the bulb refused `Budget(Pairs)`
  at depth 24, after 65,493 splits. With the fix, the cone pair
  `6v1`–`12v1` holds in 3,847 splits (was 43,159), and the torus pair
  `7v1`–`11v1` holds in 11,575 (was 21,779).
- **The fix.** `Hull::of` reads every carrier without a spline chart
  by its mean-value form about the cell's centre: the point there,
  plus each partial enclosed over the whole cell (`Surface::jet` at
  the cell's intervals) times the cell's offset from the centre. A
  plane is the form's affine case, so its own arm is gone. The
  enclosure proof is in `Hull::support`'s doc.
- **The bulb holds** in 23,531 splits (36% of the budget), 1.0 s.
- **The tour walk** holds 77 of 85 (76 before). No body that held
  moved. The certificates total 69.7 s (90.8 s before), and splits
  total 493k (639k before).
- **Pins.**
  - `support_rows::an_analytic_cells_support_holds_its_points_and_is_narrower_than_its_box`.
  - `self_intersection_proves_injectivity::a_conical_and_a_toroidal_wall_hold_within_the_default_budget`.
  - Both go red when an analytic cell is read by its box.
