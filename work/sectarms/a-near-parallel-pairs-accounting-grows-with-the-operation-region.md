---
id: a-near-parallel-pairs-accounting-grows-with-the-operation-region
kind: issue
title: A near-parallel ℝ³ pair's accounting sweep runs over the whole operation region, so small near-parallel faces in a large part refuse CellBudget instead of listing their arcs
status: open
opened: 2026-10-31
priority: P1
cost: H
design: true
---


Filed by the SECTARMS orchestrator (2026-10-31) from the cylinder × cylinder
unit's phase-0 measurement (lane `sectarms-cyl`; probe calling
`ssi::implicit_pair_r3` directly, release build).

## What

Fork row 224 / PR 6127 decided that an ℝ³ pair is read over its own ball cut
by one region per operation, and that seeding runs at the pair's own scale.
The **accounting** sweep, C3's exhaustiveness over the domain at the ε floor,
is a 3-D tree over that domain. Its cost therefore grows with the region, and
the decision says nothing about it.

Measured, 1 mm rods 1e-3 rad apart:

| region | 1e-9 | 1e-12 |
|---|---|---|
| 0.02 m | 28k cells | |
| 0.1 m | 194k cells, the budget being 200k | |
| 1 m | `CellBudget` | `RefinementExhausted` (hull_sup in band) |

PR 6127's own worked example (1 mm rods, 1 m, 1e-7 rad) reads correctly as
crossing at L. It then refuses typed instead of listing its open arcs, as the
PR text claimed it would.

## Open (design)

The candidate answers:

- **Accept the typed refusals.** Sound, but a near-parallel pair of small
  faces in a large part does not build.
- **Account on the thinner wall's chart, cut by the region.** This mirrors
  the seeding decision, but departs from C3's per-side rule (two residual
  sides trace in ℝ³), so it is Ev's.
- **Something else.** Designers first, then an `[ev]` PR if the answer
  changes C3.

The cylinder unit's stage 1 lands without it: its refusals are typed.
