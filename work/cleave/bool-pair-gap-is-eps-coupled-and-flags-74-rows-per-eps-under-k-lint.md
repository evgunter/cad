---
id: bool-pair-gap-is-eps-coupled-and-flags-74-rows-per-eps-under-k-lint
kind: issue
title: bool_pair_gap records 24·eps on touching pairs: an eps-coupled margin off the rule (4) roster, 74 k-lint rule-2/3 flags per eps row
status: dispatched
opened: 2026-10-10
priority: P2
cost: M
branch: cleave/pair-gap-eps-coupled
---

## What

`bool_pair_gap` (`PAIR_GAP` in `crates/topo/src/boolean/separating.rs`,
"two reaches' gap along a candidate direction, less both items' pads")
is the largest gating population in a fresh K sweep on main
(`f43f16b26`, 2026-10-10: editor-core `m4_pr8_k_probe` plus the tour's
`k-probe`, merged as `scripts/k_probe_sweep.sh` does). 3 632 rows per
ε row; 74 of them per row flag rule 2's definite arm and rule 3 (the
baseline floor, 4e-5), 148 flags per row:

| shape | rows per ε | \|m\| at 1e-6 / 1e-9 / 1e-12 |
| --- | --: | --- |
| `demo/tiltedcut` | 28 | 2.4e-5 / 2.4e-8 / 2.4e-11 |
| `demo/projectbox_cutaway` | 18 | 2.4e-5 / 2.4e-8 / 2.4e-11 |
| `demo/lily` | 14 | 2.4e-5 (8), 2.4000000003e-5 (6), scaling the same |
| `demo/lily_walls` | 14 | as `demo/lily` |

The margin is **24·ε at every row**: a pad-dominated gap, so the
statistic is ε-coupled — the shape `EPS_COUPLED_PREDICATES` and rule (4)
exist for (`tools/k-lint/src/lib.rs`, "The ε-coupled families") — and
the metre rules read it as a fine feature. It is not on that roster or
on `EPS_COUPLED_UNRULED`. The name entered the tree at `ae5765f42`
(REACH, 2026-10-06), after the last committed era.

## The question

Whether `bool_pair_gap`'s recorded margin is meant to be the gap less
the pads (then it is ε-coupled by construction, and the roster wants
it, with a floor cut per the K-REPORT runbook), or the gap itself
(then the recorder subtracts the pads in the wrong place). The source
decides it, not the lint.

## Home

CLEAVE (`scripts/work.py territory` puts `separating.rs` under cleave
and hone). Filed by GERM's k-lint zero-arm unit
(`germ/klint-zero-arm-tally`), which measured the sweep and does not own
the predicate.
