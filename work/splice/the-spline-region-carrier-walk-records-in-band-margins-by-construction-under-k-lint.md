---
id: the-spline-region-carrier-walk-records-in-band-margins-by-construction-under-k-lint
kind: issue
title: k-lint (dev-probe): the spline-region carrier walk on demo/lily_walls records 1106 in-band margins at 1e-6 and 720 at 1e-9, halving its cells through the band
status: open
opened: 2026-11-10
priority: P2
cost: M
refs: [k-lint-dev-probe-large-k-gate-fails-on-51730-margins-once-the-sweep-runs]
---


Found by INSTR's large-K lint lane (`instr/klint-large-k`), reading the
uncapped split of
`work/instr/k-lint-dev-probe-large-k-gate-fails-on-51730-margins-once-the-sweep-runs.md`.

## What

`scripts/k_probe_sweep.sh`'s dump half on main `9fe94da35` (2026-11-10),
linted by `tools/k-lint` as the nightly's `large-K lint` step lints it.
`bool_spline_region_carrier` on `demo/lily_walls` is the largest rule-1
family in the gate:

| ε | rule 1 (in band) | rules 2 and 3 (definite, below 4e-5) | \|m\|/ε of the in-band rows |
|---|--:|--:|---|
| 1e-6 | 1106 | 538 | 1.18 … 9.998 |
| 1e-9 | 720 | 804 (654 of them under rule 2) | 1.07 … 10.0 |
| 1e-12 | not recorded | not recorded | the lily stops at its pinned stem limit |

The margins are spread over the whole band (553 distinct values at
1e-6), and the definite rows continue upward from `Kε` to 3.9e-5 m at
both rows.

## Reading (not yet the owner's)

Both decides are in `carrier_walk` (`crates/topo/src/boolean/spline_region.rs`,
`carrier_walk`, the two `decide(SPLINE_REGION_CARRIER, …)` calls, ~309
and ~342): the cell's floor against `q`, then the cell centre's distance
from `q`. An in-band reading there is kept as `pending` and the walk
quarters the cell and goes on. Only a walk that runs out of cells, or
ends with `foot` and `open`, reports it (`Reading::Unsettled(pending)`).
So a walk that quarters a cell down towards `q` passes through
`(ε, Kε)` by construction, and every such step records a rule-1
sample. The steps above `Kε` record the rule-2 and rule-3 samples.

If that reading holds, these samples are refinement steps, not refusals.
k-lint's rule 1 cannot tell the two apart (`work/instr/k-lint-rule-1-prose-assumes-every-in-band-site-refuses.md`),
and rule 1 is not demotable. Re-deriving k-lint's thresholds cannot
touch rule 1, because it reads only the sample's `outcome`.

## Owed

Read whether any of these walks ends `Unsettled` on the lily, or whether
every in-band sample is a step the walk passed through. Then decide
whether a refinement step's stop test should be a recorded `decide` at
all. If the walk is right and the record is the issue, say so on the
INSTR row named above so the lint's side can be ruled.

Reproduce: `CAD_TOLERANCE_EPS=1e-9 cargo run --manifest-path demos/tour/Cargo.toml --features probe -- k-probe /tmp/k.csv`,
then `awk -F, '$2=="bool_spline_region_carrier" && $6=="indeterminate"' /tmp/k.csv | wc -l`.
