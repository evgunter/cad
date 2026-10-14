---
id: bool-pair-gap-is-eps-coupled-and-flags-74-rows-per-eps-under-k-lint
kind: issue
title: bool_pair_gap records 24·eps on touching pairs: an eps-coupled margin off the rule (4) roster, 74 k-lint rule-2/3 flags per eps row
status: open
opened: 2026-10-10
priority: P2
cost: M
design: true
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

## Measured (branch cleave/pair-gap-eps-coupled, origin/main `8354f5c03`)

**The recorder and the decision agree.** `separating::apart_along`
mints `PAIR_GAP` once, through `validate::decide` on
`Margin::of(gap − 2·sweep_pad)`, and reads only that decision's sign.
The funnel records the margin it classifies, so no recorder can
subtract the pads anywhere else. The recorded margin is the clearance
between the two padded reaches, which is what the verdict is made on.
`sweep_pad = escalate + 2·zero` makes the offset `2(K + 2)·ε`, which is
24·ε at K = 10.

**The sweep reproduces the finding.** These are the linted halves of
`scripts/k_probe_sweep.sh`: the corpus dump plus the tour's `k-probe`.
The plain-selection prepass is red on main at
`assemble_one_local_battery`, which
`work/tint/probe-sweep-prepass-reds-on-assemble-one-local-battery.md`
tracks. Results:

- `bool_pair_gap` has 3 632 rows per ε row: 3 628 negative and 4
  positive.
- 74 rows per ε row sit at `|m| = 24·ε` exactly (62 of them at 1e-12;
  the other 12 at 24.003·ε, an overlap of about 3e-15 m). Those are
  pairs whose reaches abut exactly. They are the rule-2 and rule-3
  flags: 74 + 74 per row.
- The next margin up is 1.9e-4 m. Every negative row carries the same
  −24·ε offset: the 1e-6 and 1e-9 rows differ by 2.4e-5 throughout.
- Across the whole stream, `bool_pair_gap` is the only predicate with
  samples whose `|m|/ε` is identical at all three rows.

**Why rostering is not an application of rule (4).** Rule (4)'s one
floor is 150·ε, the P0 of `props_quad_converged`. The contact rows
sit at 24·ε, so rostering the name turns 148 flags per row into 74
rule-4 flags. Either way out changes rule (4) as the M7 addendum
states it:

- re-cut the shared floor to 20·ε or below, which weakens the quad
  family's guard by 7.5×; or
- give each family its own floor.

Both also need a committed era that carries `bool_pair_gap` draws (the
`props_quad_last_round` ruling, `threshold_provenance.rs`). This same
sweep moves other witnesses as well: `chart_bound_outer_span` has 41
invalid rows per row, `volume_backstop` reads 1.5e-5 m on demo/table,
and `props_quad_last_round` now has draws.

**Sibling with the same shape.** `split_gate_box_side`
(`splitting/classify.rs`, `reach_clears`) decides on `gap − sweep_pad`.
It records no row in this sweep.

## Re-homed to INSTR (CLEAVE, 2026-10-11)

CLEAVE's lane (`cleave/pair-gap-eps-coupled`, measurement above)
answered the row's question: the recorded margin is the margin the
decision decides on, so `separating.rs` has nothing to fix. What is
left is a choice about k-lint's rule (4): a single shared floor vs
per-family floors, and the new committed era either needs. That is in
`tools/k-lint` and `docs/K-REPORT.md`, INSTR's territory, so the row
moves here. It is a design fork (two designers, then Ev if it changes
rule (4)'s ratified shape). The lane recommends per-family floors on a
new era.

The kernel side is final: `bool_pair_gap`'s lower edge is the closed
form `2(K+2)·ε`, with an empty gap above it up to 1.9e-4 m.
`splitting/classify.rs` `reach_clears` (`split_gate_box_side`) has the
same shape and is covered by the same ruling. Neighbours:
`k-lint-eps-coupled-criterion-unwritten`,
`work/tier/census-decides-under-bool-pair-gap-unaudited.md`.
