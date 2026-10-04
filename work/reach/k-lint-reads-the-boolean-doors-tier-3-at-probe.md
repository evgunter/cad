---
id: k-lint-reads-the-boolean-doors-tier-3-at-probe
kind: issue
title: k-lint (dev-probe) flags 108 rows with tier 3 at the boolean door: the gate asks chart_bound_outer_span and check 7's quadrature on results and finished operands at Probe
status: closed
opened: 2026-10-03
priority: P1
cost: M
refs: [boolean-door-adopts-the-finished-body-type, chart-bound-outer-span-decides-a-poisoned-margin, quad-last-round-margin-has-no-sign-premise]
closed: 2026-10-03
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

## Closed (2026-10-03): red on main by the same flags, not this door's

**Current count (2026-10-04): 104 flags on both trees, identical.**
PR 3987's verifier (`analysis/reach-verify/3987`) re-ran
`delta3987_ksweep.sh` at 1e-6 and 1e-9 on head 149ed1091 and its base
main c860806e: both GATE FAILED with 104 margins (rule 1: 82, rule 2: 3,
rule 3: 20; 105 FLAG lines), identical once line numbers are stripped
(5.75 M samples on the head, 3.17 M on main). The 100 below is the
delta review's same measurement against the older base 46ce5d4d4.

The delta review of PR 3987 (`analysis/reach-delta/3987`, NOTE 3)
re-ran the measurement against the head's own main base. It ran the
nightly's dev-probe dumps (`m4_pr8_k_probe`, `demo-tour k-probe`) at
1e-6 and 1e-9, then `tools/k-lint` over both rows, on head 07ca5a8d
and on `origin/main` 46ce5d4d4. Both trees give GATE FAILED with 100
margins (rule 1: 82, rule 2: 3, rule 3: 16). One `table:volume_backstop`
margin trips two rules, so there are 101 FLAG lines. The flags are
identical once CSV line numbers are stripped. The head has 2.87 M
samples per row and main 1.58 M. The 104 below is the same measurement
against the older main 11d9c7a7f.

The attribution above was by mechanism, and a same-base measurement
disproves it. `scripts/k_probe_sweep.sh` (past
`tilted_sphere_pair_k_rows`, as above) then `tools/k-lint` over the 1e-6
and 1e-9 rows, on `origin/main` 11d9c7a7f and on this branch merged with
it: both GATE FAILED with **104** flags, identical line for line (shape,
predicate, margin, rule), and both 1e-12 demo passes stop at
`lily_leaf_b`. Main's rule-1 count is 41 per row, not the nightly's
recorded 9: the nine are old.

The branch's earlier 108 had four more `demo/tiltedcut:chart_bound_outer_span`
rows: the tour finished the tilted cut's two split halves, which no
boolean takes, so tier 3 ran on two more bodies with a periodic-chart
face at `Probe`. The halves are finished only where the walls use them
as operands (`demos/tour/src/curvedcut.rs`). The gate adds samples
(1.58 M to 2.87 M per row) and no flag.

The red row is main's: CHART's
`chart-bound-outer-span-decides-a-poisoned-margin` (whose readings may be
a design fork), the `props_quad_*` roster ruling, and
`demo/table:volume_backstop`.
