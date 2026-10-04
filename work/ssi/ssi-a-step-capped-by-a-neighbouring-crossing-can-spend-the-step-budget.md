---
id: ssi-a-step-capped-by-a-neighbouring-crossing-can-spend-the-step-budget
kind: issue
title: ssi: a plane × NURBS branch's step is capped by the nearest unused crossing, which may be another branch's, so a long branch beside a close crossing can spend SSI_MAX_STEPS
status: open
opened: 2026-10-02
priority: P2
cost: M
---


(SSI implementer on PR 3862, from reviewer B's review of that PR,
2026-10-02.)

## What

`ssi/ends.rs`'s `Ends::branches` marches each open branch from a
crossing `A` with its step capped at `min(SSI_STEP_MAX·extent, d/5)`,
`d` the distance from `A` to the nearest crossing not yet used. The
partner is not known before the march, so `d ≤ |AB|` and the branch is
cut into at least five steps. When another branch's crossing lies close
to `A` (two branches converging on a side, or meeting it a little apart),
`d` is that gap rather than the branch's length, and a branch of length
`L` takes about `5L/d` steps. Past `SSI_MAX_STEPS` (20 000) the march
refuses `StepBudget`, though the branch is ordinary. Reachable by valid
geometry; not yet met by a fixture.

## Open

Whether the cap should be re-derived once the partner is known (a second
march at `|AB|/5`, which is the re-march this lane retired), read from
the crossings on the same branch's side pair only, or grow as the march
leaves `A`'s neighbourhood (the step rule's own curvature rungs already
bind far from the ends). A stepper choice, priced M.

## Met by a fixture (ssi/step-max-certify, 2026-10-03)

`SSI_STEP_MAX` is retired, so the cap is `d/5` alone, and a fixture now
reaches it. `m5_pr7_ssi.rs`'s `close_crossings_wall(gap)` is cut by the
plane `y = 0` in two straight metre-long branches that meet the bottom
side `gap` apart. At `gap = 1e-4` m each branch takes 50 000 steps and
the call refuses `StepBudget` with `StepBound::Cap`; at `gap = 0.2` m it
certifies both (`a_spent_step_budget_ends_by_the_rung_that_held_its_steps`).
The `Cap` ending now names this lever: move the geometry so no other
branch meets the boundary near this one.

With refinement by certification in place, the open question above has
a fourth option: the cap need only give the fit its samples, since the
certificate asks for more where it needs them.

## Re-measured on main after PRs 3998, 3983 and 4028 (ssi/neighbour-cap, 2026-10-04)

Still refuses. The Hermite does not carry these branches: it is tried
from `A` to the nearest unused crossing, which here is the other
branch's, so it is refused and the march runs at that gap's fifth.

**`close_crossings_wall(gap, h)`** (straight branches; ε 1e-6, 1e-9,
1e-12 give the same column):

| gap \ h | 0.25 m | 1 m | 2 m |
|---|---|---|---|
| 1e-4 | `CellBudget` | `StepBudget`, `Cap(Crossing)` | `StepBudget`, `Cap(Crossing)` |
| 1e-3, 1e-2 | `CellBudget` | `CellBudget` | `CellBudget` |
| 5e-2 | 26 + 4 samples | 101 + 4 | 201 + 4 |
| 0.2 | 7 + 4 | 26 + 4 | 51 + 4 |

This fixture is the wrong witness for the cap at gap ≤ 1e-2: the wall
crosses the plane at about `gap` radians and lies within `gap²/4` of it
between the branches, so once the march gets through, exhaustiveness
spends the cell budget. With any of the fixes below it refuses
`CellBudget` at every gap ≤ 1e-2.

**A transversal witness**: `y = z/h + 2·gap² − 8(x − ½)²` on the unit
square, extruded `h` tall in `z`, cut by `y = 0`. Its two parabola arms
meet the bottom side `gap` apart, and `∂y/∂z = 1/h` keeps the wall
transversal everywhere. Main, samples of the first and second branch:

| gap | h = 0.25 | h = 1 | h = 2 |
|---|---|---|---|
| 1e-4 | `StepBudget` `Cap(Crossing)`, all ε | same | same |
| 1e-3 (ε 1e-6) | 2246 + 34, 1.5 s | 5456 + 49, 4.1 s | 10281 + 54, 13 s |
| 1e-2 (ε 1e-6) | 224 + 34 | 544 + 46 | 1027 + 51 |
| 0.2 (ε 1e-6) | 22 + 22 | 35 + 30 | 55 + 30 |

The second branch, once the first has used the near crossing, takes
the curvature's samples (34 to 54 at ε 1e-6, up to 1601 at 1e-12).
The first takes `5L/gap`.

**Prototypes measured** (not built):

- *Scout, then the Hermite to the crossing reached.* March from `A`
  under the curvature and diagonal rungs alone, match its exit `B`, and
  try the Hermite `A → B` before anything else. `close_crossings` at
  gap ≥ 5e-2: both branches one Hermite span (4 samples) at every h
  and ε. Its gap ≤ 1e-2 cases go to `CellBudget`, as above.
- *Then certify the scout's own states* (no neighbour cap; refinement
  adds samples where the certificate refuses), or *re-march at
  `|AB|/5`* (the re-march once retired). Every arms case certifies at
  every gap, h and ε, both branches the curvature's count (34 to 55 at
  ε 1e-6, 115 to 286 at 1e-9, 627 to 1608 at 1e-12); h = 2, gap 1e-3,
  ε 1e-6 falls from 13 s to 0.27 s. The two give identical counts on
  the arms: the `|AB|/5` cap never binds a curved branch.
  Without either, a straight branch whose Hermite to `B` is refused
  would reach the fit with the scout's one or two steps, fewer samples
  than the cubic needs, and refinement halves only what a certificate
  refused; the re-march, taken once, is the fixed rule that covers it.

Each changes what C3 states of the march ("its step capped at a fifth
of the distance to the nearest crossing not yet used"), and the first
changes which crossing the Hermite is tried to, so the choice is a
design fork, not a defect fix.
