---
id: a-near-parallel-pairs-accounting-grows-with-the-operation-region
kind: issue
title: A near-parallel ℝ³ pair's accounting sweep runs over the whole operation region, so small near-parallel faces in a large part refuse CellBudget instead of listing their arcs
status: closed
opened: 2026-10-31
closed: 2026-11-02
priority: P1
cost: H
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

## Decided (2026-10-31, Ev on `[ev]` PR 6137; fork row 227)

A pair with a ruled side (a cylinder or cone against a plane, cylinder,
cone or sphere) reads its pose on the ruling sweep. The sweep is a
certified 1-D pass over the ruling angle; each ruling meets the partner
at a closed-form quadratic's roots. It does four jobs:
- it decides the tangency;
- it counts the components;
- it gives the cut ends where each component crosses the domain the
  march ends at, plus a witness on each interior component;
- it seeds the march.

**Exhaustiveness closes by the count.** The certified branches are
matched one to one to the cut ends and witnesses; anything else is a
desync. No 3-D accounting sweep runs for these pairs.

**A 1-D coverage of the ruling angle is a `debug_assert`** (DESIGN's
error taxonomy, row 5): every cell's meetings lie outside the domain or
inside a found tube. Ev: "in production we don't want to waste time
double checking things".

**Build items:**
- one ruling-sweep module, shared with the cone pose;
- one-to-one end matching in `carry`'s desync check;
- the march ends at the region's ball, not the caller's cube;
- seeds come from the sweep;
- the tube ladder stays at the region's scale;
- the orchestrator's earlier "count only when the pose's ball lies
  inside the region" ruling is retired.

This row closes when the cylinder unit's stage that builds the sweep
lands.

## Closed (2026-11-02, branch `sectarms/cylinder-ruling-sweep`)

The build items, for two cylinders:
- `geom_brep::pair_section::ruling`: the sweep over the `Ruled` shape a
  cone pose will hand it too. It counts the loops, cuts them by the
  region (cut ends, witnesses) and reads branch points sheet-wise.
- `PairSection::matched` matches cut ends and witnesses one to one
  (`ended_by`).
- The march ends at the region's ball (`ssi::march::BallExit`), seeded
  from the count's witnesses, with no `seed_r3` or `account_r3`.
- The extent stays the region's, and the provisional
  inside-the-region split is retired.
- The 1-D coverage is a row-5 `debug_assert` read from the operands'
  general quadratic (`ruling::coverage`).

Two pieces of residue are filed:
- cylinder × sphere, the next ruled pair
  (`cylinder-sphere-reads-its-pose-on-the-ruling-sweep`);
- the thin-rod row's limb-3 refusal, which is the certificate's limit,
  not the count's
  (`work/ssimarch/limb-three-reads-no-margin-on-a-long-near-parallel-cylinder-arc.md`).

