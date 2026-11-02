---
id: lily-walls-curve-walks-decide-24045-margins-in-band-under-k-lint
kind: issue
title: k-lint (dev-probe): the lily_walls curve walks leave 24,045 margins in the ambiguity band at 1e-6 and 1e-9
status: open
opened: 2026-10-29
priority: P2
cost: M
refs: [k-lint-dev-probe-large-k-gate-fails-on-51730-margins-once-the-sweep-runs]
---


Found by GREEN's nightly-76 lane, when the K-telemetry probe sweep
first ran to its end (`work/instr/k-lint-dev-probe-large-k-gate-fails-on-51730-margins-once-the-sweep-runs.md`).

## What

Every rule-1 sample of the large-K lint is the lily's wall probes
(`demos/tour/src/lily.rs` `wall_probes`, scene `demo/lily_walls`), in
the curve-pair walk's predicates (`crates/geom-brep/src/curve_pair/walk.rs`).
Each sample is decided `indeterminate`: its margin lands in the ambiguity
band. Counted from the sweep's CSVs (`outcome` column), on main
`c173123e` with GREEN's probe pins:

| ε | predicate | in band |
|---|---|---|
| 1e-6 | `curve_pair_walk_foot_clear` | 5532 |
| 1e-6 | `curve_pair_walk_foot_piece` | 3312 |
| 1e-6 | `curve_pair_walk_clear` | 347 |
| 1e-6 | `curve_pair_walk_piece` | 15 |
| 1e-9 | `curve_pair_walk_foot_clear` | 9349 |
| 1e-9 | `curve_pair_walk_foot_piece` | 5181 |
| 1e-9 | `curve_pair_walk_clear` | 290 |

At 1e-12 there are none, but only because nothing is recorded: the
`lily` and `lily_walls` scenes are pinned to stop there on the stem's
kernel limit (`work/offset/main-red-five-slow-rows-refuse-at-limb-2s-kernel-limit-since-5728.md`).

The lint's zero arm also tallies these predicates near band_zero. At
1e-6, `curve_pair_walk_clear` uses up to 100% of ε, and `…_foot_clear`
up to 76%.

**Unread.** Nobody has read these margins yet. This row does not claim
the decisions are wrong or right, and does not say whether the fix is
in the walk or in the scene. It is also unknown when the population
appeared: no earlier night's sweep reached this lint.

## What closing it takes

Read which wall pairs the walks decide in band. Is each one a true
near-touch that should escalate, or a margin that should not crowd the
band at all? Then fix the walk, or record the population
as expected, so INSTR's large-K lint can gate again.

Reproduce: `CAD_TOLERANCE_EPS=1e-9 cargo run --manifest-path demos/tour/Cargo.toml
--features probe -- k-probe /tmp/k.csv`, then
`awk -F, '$6=="indeterminate"{print $1":"$2}' /tmp/k.csv | sort | uniq -c`.

