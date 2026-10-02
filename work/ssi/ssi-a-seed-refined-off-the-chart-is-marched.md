---
id: ssi-a-seed-refined-off-the-chart-is-marched
kind: issue
title: ssi: a seed Newton settles outside the march domain is marched from, and its out-of-chart state reaches the fit
status: closed
opened: 2026-10-02
priority: P1
cost: E
closed: 2026-10-02
pr: 3864
branch: ssi/seed-in-domain
---


Split out of `plane-nurbs-ssi-does-not-certify-a-curved-dome` (its
cause 1).

## What

`march` (`crates/geom-brep/src/ssi/march.rs`, `march`, ~:547) refines
its seed with min-norm Newton and marches from wherever it settles. It
never asks whether that state is inside `ctx.domain`. Every marched
state is decided (`ssi_branch_open_end`, ~:740); the seed alone is not.

Newton moves a seed across the domain's boundary as readily as a step.
Then `push_boundary` (~:987) bisects from the seed as the "inside" end,
and no bisection finds a state that is `within` the domain (~:1006). The
backward half stays a single out-of-chart state, and the fit
interpolates a wall point evaluated off its knot domain. When the
forward march also leaves at once, the trace has length 0 and the run
refuses `TraceUnresolved { samples: 1 }`.

## Repro

The dome `W(d)`: a clamped quadratic 3×3 net, control `(i/2, 0, j/2)`
with the centre moved to `y = −d`. It is cut by the tilt plane
`x + y = 0.5 − d/8`, in the domain `SsiDomain { center: (0.5, −d/8,
0.5), half_extent: 2, extent: 1, floor_scale: 1 }`. The ℝ⁴ seeder
(`plane_nurbs_ssi`, `crates/geom-brep/src/ssi.rs` ~:1906) hands
`(0.369, 0.00195)` at d = 1 and `(0.243, 0.00098)` at d = 2. These are
one seed cell from the `t = 0` edge, and Newton settles them at
`t = −1.15e-3` and `−1.87e-3`.

`plane_nurbs_ssi` on origin/main 3ee0e4b6e:

| d | ε 1e-6 | ε 1e-9 | ε 1e-12 |
|---|---|---|---|
| 1 | OnLocus 1.57e-3 | OnLocus 1.57e-3 | FitSampleBudget 2333 |
| 1.5 | OnLocus 4.19e-3 | OnLocus 4.19e-3 | TraceUnresolved 1 |
| 2 | OnLocus 3.36e-3 | TraceUnresolved 1 | TraceUnresolved 1 |
| 3 | OnLocus 5.55e-3 | TraceUnresolved 1 | TraceUnresolved 1 |

## Fix shape

Decide the settled seed inside the domain, with the decision every
marched state gets, before it is marched. A seed decided outside is no
branch, as one that will not settle is. Its cell is still accounted
for, because the exhaustiveness pass refuses at the floor any cell no
tube covers. Pin the d = 1 and d = 2 rows.

## Closed

`march` now decides the settled seed with `ssi_branch_open_end` before
it marches it. A seed decided outside refuses `SsiError::SeedOffDomain`,
which both doors treat as no branch. A seed in the zero band is
marched, and one in the escalation zone escalates. PR 3864 has the
before and after table. The cut still refuses at d = 1–3 for other
causes: `plane-nurbs-tube-straddles-a-curved-dome-at-coarse-eps` at
ε 1e-6, and causes 4 and 2 of
`plane-nurbs-ssi-does-not-certify-a-curved-dome` at 1e-9 and 1e-12.
`a_seed_settled_off_the_walls_chart_is_no_branch` pins those refusals.
