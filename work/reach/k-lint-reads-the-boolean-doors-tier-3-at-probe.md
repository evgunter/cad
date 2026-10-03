---
id: k-lint-reads-the-boolean-doors-tier-3-at-probe
kind: issue
title: k-lint (dev-probe) flags 108 rows with tier 3 at the boolean door: the gate asks chart_bound_outer_span and check 7's quadrature on results and finished operands at Probe
status: open
opened: 2026-10-03
priority: P1
cost: M
refs: [boolean-door-adopts-the-finished-body-type, chart-bound-outer-span-decides-a-poisoned-margin, quad-last-round-margin-has-no-sign-premise]
---


Measured on `reach/door-finished-body` (the boolean door gating its
result at tier 3, callers finishing operands through the at-rest gate):
`scripts/k_probe_sweep.sh` (run past `tilted_sphere_pair_k_rows`, red on
main as `work/topo/the-carved-balls-clearance-row-is-vacuous-on-main.md`;
the 1e-12 demo pass stops at main's `lily_leaf_b`), then `tools/k-lint`
over the 1e-6 and 1e-9 rows: GATE FAILED, 108 flags.

| shape:predicate | flags (both rows) |
|---|---|
| demo/projectbox_cutaway:chart_bound_outer_span | 32 |
| demo/tiltedcut:chart_bound_outer_span | 24 |
| demo/lily_walls:chart_bound_outer_span | 12 |
| demo/lily:chart_bound_outer_span | 6 |
| demo/bossplate:chart_bound_outer_span | 6 |
| corpus/boss_union:chart_bound_outer_span | 6 |
| demo/lily:props_quad_last_round | 8 |
| demo/lily_walls:props_quad_last_round | 4 |
| demo/projectbox_cutaway:props_quad_converged | 6 |
| demo/lily:props_quad_converged | 2 |
| demo/table:volume_backstop | 3 |

The nightly reading `chart-bound-outer-span-decides-a-poisoned-margin`
records is 9 `chart_bound_outer_span` rule-1 flags per ε row
(boss_union, bossplate, lily_walls, 3 each); this run has 43 per row.
That predicate's NaN is the filed mechanism (a point-scalar hull is
poison by design), and tier 3 asks it on every body it validates, so the
gate on results and on operands asks it on more bodies. The
`props_quad_*` rows are check 7's quadrature, which tier 3 runs on curved
results at `Probe`; `docs/K-REPORT.md` keeps `props_quad_last_round` off
rule (4) on the record that no sweep had recorded one, which this one
does. `demo/table:volume_backstop` (1.5e-5, below the 4e-5 floor) was
not attributed. Not measured against a main sweep of the same tree, so
the attribution of the increase to the gate is by mechanism, not by
difference.

The geometry is not to be changed (`docs/prompts/implementer-discipline.md`
§3, k-lint). What closes it: the chart item's fix (the predicate stops
recording poison), a ruling on `props_quad_last_round`'s rule-(4)
roster, and a baseline re-derivation per `docs/K-REPORT.md` against a
sweep with the gate in.
