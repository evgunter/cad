---
id: ssi-a-step-capped-by-a-neighbouring-crossing-can-spend-the-step-budget
kind: issue
title: ssi: a plane × NURBS branch's step is capped by the nearest unused crossing, which may be another branch's, so a long branch beside a close crossing can spend SSI_MAX_STEPS
status: spec
opened: 2026-10-02
priority: P2
cost: M
needs_ev: true
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

## Design (converged, 2026-10-04; `[ev]` PR for the C3 sentence)

Two designers weighed it over two rounds. The record is in `docs/DESIGN-FORK-LOG.md`, and the probes are on `analysis/design-fork/neighbour-cap-{a,b}`.

1. **Retire the crossing cap** (`StepCap::Crossing`, its "a fifth", and its ending). Between known crossings the march's step is the curvature's against ε, no longer than the domain's diagonal.
2. **Keep the candidate order Ev agreed on PR 3862.** The Hermite cubic goes to the nearest unused crossing first, then the march, uncapped, which pairs by its exit. A wrong guess costs one refused certificate: limb 3 (PR 3999) makes every certified candidate's pairing the locus's own, so the order never changes a certified answer. March-first was weighed and withdrawn. It loses a branch whose curvature step falls in the band, such as `a_semicircle_too_short_to_march_whose_cubic_misses_refuses_by_its_length`, unless it grows a second, nearest-crossing path.
3. **The fit's minimum is the fit's.** A polyline with fewer than `SSI_FIT_DEGREE + 1` samples has its gaps halved, each midpoint settled onto the locus, until it has four. Halving stops when half a gap falls in the band (`ssi_refine_halving`). `ShortBranchUncertified` is minted from that stop. The separate `ssi_short_branch` decision and `SHORT_BRANCH_STEPS` retire on this lane.
4. **Likely, to be confirmed in the build:** the ℝ³ lane's `march_both` re-march at `length/5` (`StepCap::ShortBranch`) is the same class, and the same minimum rule retires it. `Ends::through_seed`, which can hand the fit fewer than four states today, is covered by the same rule.

**Measured on the probes:**
- On the transversal two-arm witness, every gap from 1e-4 to 0.2 m certifies, and each branch takes its curvature's count: 262 + 262 samples at gap 1e-4, where main refuses `StepBudget`; 285 + 285 at gap 1e-3 and h = 2, where main takes 10 281 samples and 13 s.
- The bent-chart warp row certifies with the uncapped march.

**Done when:**
- the cap is retired and the fit minimum is in `Ends::finish`, or wherever the fit runs, for every polyline;
- the arms witness, at gaps 1e-4 / 1e-3 / 0.2 and ε 1e-6 / 1e-9 / 1e-12, certifies as a row; the `Cap` arm of `a_spent_step_budget…` moves to it;
- the rows the designers listed are re-pinned: the straight-marched row, the δ-short row and the rung-naming row;
- the semicircle keeps its sized refusal;
- C3 reads as on this PR.

**Not in scope, file separately:**
- `match_exit` picks the nearest crossing to the chord's meeting point. Uncapped chords make it worth settling by identity (Newton the exit onto the side).
- The march's per-state transversality lever reads the chart, not the geometry. This is covered by `ssi-transversality-at-a-point-is-spelled-three-ways`.
