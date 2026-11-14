---
id: refinement-walks-flag-rules-2-and-3-by-construction-in-the-gated-sweep
kind: issue
title: k-lint: three refinement walks on demo/lily_walls record every piece's reach from 4e-5 down to Keps, so they flag rules 2 and 3 by construction (754 at 1e-6, 2118 at 1e-9)
status: open
opened: 2026-11-10
priority: P2
cost: M
design: true
refs: [k-lint-dev-probe-large-k-gate-fails-on-51730-margins-once-the-sweep-runs, lily-walls-curve-walks-decide-24045-margins-in-band-under-k-lint, the-spline-region-carrier-walk-records-in-band-margins-by-construction-under-k-lint]
---


Found by this program's large-K lint lane (`instr/klint-large-k`),
reading the uncapped split of
`k-lint-dev-probe-large-k-gate-fails-on-51730-margins-once-the-sweep-runs`.

## What

The sweep's dump half on main `9fe94da35` (2026-11-10). Three
predicates on `demo/lily_walls` are halving walks that record a sample
at every step:

- `curve_pair_walk_foot_clear` and `curve_pair_walk_foot_piece`
  (`crates/geom-brep/src/curve_pair/walk.rs`, `near_pieces` and
  `stops`);
- `bool_spline_region_carrier` (`crates/topo/src/boolean/spline_region.rs`,
  `carrier_walk`).

`_foot_piece`'s in-band rows at 1e-6 are a pure ×2 ladder. Its values
are 5.262e-6 (18 rows), 2.631e-6 (36) and 1.316e-6 (72): each halving
doubles the pieces and halves the reach. Flags that are not rule 1:

| ε | `_foot_clear` | `_foot_piece` | `_carrier` | total | \|m\|/ε |
|---|--:|--:|--:|--:|---|
| 1e-6 | 144 | 72 | 538 | 754 | 10.0 … 38.9 |
| 1e-9 | 882 | 432 | 804 | 2118 | 10.0 … 3.9e4 (1428 under rule 2) |

That is 754 of the 1992 rule-2 and rule-3 flags at 1e-6 (38%), and
2118 of 3341 at 1e-9 (63%).

## Why no threshold fits them

A walk that halves a reach from above 4e-5 m down to the band records a
sample at every scale on the way. So it fills the decades between `Kε`
and the floor however the floor is cut:

- **`BASELINE_FLOOR_MARGIN`.** These rows are where the M7 addendum's
  P0 argument breaks. That argument rests on an empty gap between the
  zero cluster and the definite bottom, and these walks populate the
  gap.
- **Rule (4).** It is a floor at a fixed multiple of ε, and these rows
  reach 3.9e4·ε at 1e-9.
- **The E6 driver's demotion.** `--gate-rule-1-only` exists for a
  population that is refined toward zero by construction, but that
  population rides in a file of its own. These rows sit inside the
  gated demo dump, beside rows that rules 2 and 3 should keep judging.

## The fork

Separating them takes one of these:

- a roster of refinement predicates, judged by rule 1 alone (a
  name-keyed class like rule (4)'s);
- a scene or predicate split of the gated CSV (moving the gate's
  subject matter, which `scripts/k_probe_sweep.sh`'s header calls a
  geometry conversation);
- the kernel recording a walk's stop test some other way (the owners'
  rows in `refs`).

The first two change k-lint's ratified rule shapes, so this goes to
two designers and then Ev. It also waits on the owners' reading: if a
walk's samples turn out to be wrong, there is nothing to rule. The
lily's scenes are not recorded at 1e-12 (the stem's pinned stop), so
the walks' 1e-12 population is unknown.
