---
id: main-reds-editor-core-slow-set-and-the-k-probe-sweep
kind: issue
title: Main carries 24 editor-core slow-set reds and two reds that abort scripts/k_probe_sweep.sh (sweep ray_wall_margin_twins, editor-core m4_pr8_k_probe content key)
status: open
opened: 2026-10-21
priority: P2
cost: M
---


Found by PR 5552's FULL review on main `d39fbfc` (merge base), and
re-measured by NURBS build A on main `3c342076` (2026-10-21). NURBS
build C sees the same set independently (24, `sym11` among them). None
is PR 5552's: every one fails identically on main. Filed on TCOST, which
owns every file below with TINT; SCALAR co-owns the `m10_*_interval`
files (`work.py territory`). One item for the lot, as ruled; split it if
the causes turn out to differ.

**The editor-core slow set** (`cargo nextest run -p editor-core`, debug,
the slow set included, which `--profile ci` filters out): 3018 run, 24
failed, 3309 s.

| file | failing rows |
|---|---|
| `docm9_range.rs` | `a_budget_too_small_is_indeterminate_and_never_a_bound`, `a_branch_change_is_a_decision_flip_and_names_the_predicate`, `a_decision_flips_within_contains_a_value_that_does_not_build`, `the_first_boundary_stops_the_walk_and_a_seed_on_the_island_certifies_it`, `the_certificate_is_inside_the_locally_valid_range` |
| `m10_bulge_interval.rs` | `m10_bulge_the_d_tabs_literal_split_at_the_nominal`, `m10_bulge_the_d_tabs_parameter_split_at_the_nominal` (the review saw `segment_gap [0, 6]`) |
| `m10_sym_profile_interval.rs` | `the_forms_the_walks_build_are_pinned_per_eps_row`, `the_plains_ledger_lines_are_the_same_under_every_dial_set` |
| `m10_9_pins_interval.rs` | `m10_9_the_pad_at_both_rule_f_dials`, `m10_9_what_the_registry_buys` |
| `m10_6_ci_rows_interval.rs` | `the_margin_thin_accounting_is_goldened_bit_exact` |
| `m10_3_driver_interval.rs` | `a_planted_flip_refuses_flip_crossing_and_names_what_flipped` |
| `m10_3_r2_probes_interval.rs` | `containment_fires_when_the_chamber_is_strictly_inside_the_box` (2095 s) |
| `revalue_corpus.rs` | `revalue_measured_web`, `revalue_loft_prism`, `revalue_declared_tangency`, `revalue_die_chamfer`, `revalue_heat_sink_fins`, `revalue_plate_param_apart_to_its_parting`, `revalue_boss_union_apart_to_its_parting` |
| `sym11_exact_channel_rows.rs` | `sym11_the_exact_channel_never_contradicts_past_the_ceiling` (C: `two_hole_plate` reads `[1253, 991, 1557]` against the stored `[1253, 979, 1557]`) |
| `sym_9_retry_interval.rs` | `sym_9_the_kept_atom_ladder_recovers_what_phase_1_measured`, `sym_9_the_drive_writes_the_ladders_receipt` |

Many read stored counts or goldens, so a decision-count re-bless that
skipped the slow set is the first suspect: bisect one row per file
before blessing anything.

**Two rows that abort `scripts/k_probe_sweep.sh`** (run as the script
runs them, `--features probe`, ε `1e-9`):
- `crates/sweep/tests/ray_wall_margin_twins.rs:143`,
  `the_wall_arms_margins_scale_linearly_with_the_model`: "predicates whose
  margins do not scale with the model: [curve_foot_branch: 6 pairs off,
  e.g. 9.990e2]".
- `crates/editor-core/tests/m4_pr8_k_probe.rs:353`,
  `probe_agrees_with_f64_bit_for_bit_over_the_corpus` (ignored, swept):
  "die: node … content key — `ContentBits for Probe` feeds the wrapped
  f64's bits, so the two lanes' memo keys are the same word", the keys
  differing.

Either aborts the sweep, so the corpus K-probe dump cannot be taken
through the script on main; PR 5552's review ran the dump directly.

**Done when** each row is green on main, or re-blessed with its cause
named.

## 2026-10-23 — first nightly, and the merge (INTENT nightly triage)

All 24 slow-set rows are red at every ε row (default, 1e-6, 1e-12) in
the nightly full suite from 2026-10-22 (run 39728496598, main
`0ffab3663d`) onward, and green on 2026-10-21 (run 39549991622,
`8c7db2ae37`). Bisected over the merges between `8c7db2ae37` and the
review's `d39fbfc`, nine rows (the five `docm9_range` rows, both
`m10_bulge` rows, `revalue_loft_prism`, `sym11_…ceiling`) run as the
nightly runs them (`--profile ci --ignore-default-filter`, default ε):
all nine pass at `42af7c0b48` (#5548, which already holds INTENT's
#5506) and eight of them fail at `fa901e4308`, the merge of #5542
(`nurbs/seg-one-line`, `profile::seg`'s `gaps_apart` reading
`segment_gap`); the ninth was still running at 400 s when stopped. The
revalue and `m10_bulge` messages name `segment_gap` themselves. So the
suspected merge is #5542. The rows are still red on 2026-10-23 (run
39906156937, `be9b907694`).

## 2026-10-24 — still red (INTENT nightly triage)

Nightly run 40066431545 (main `3dfb96b91c`) still reds 22 of the 24
rows at every ε, with the same panics: `docm9_range` ×5,
`m10_3_driver_interval`, `m10_3_r2_probes_interval`,
`m10_6_ci_rows_interval`, `m10_9_pins_interval` ×2,
`m10_sym_profile_interval` ×2, `revalue_corpus` ×7,
`sym11_exact_channel_rows`, `sym_9_retry_interval` ×2. The two
`m10_bulge_interval` rows pass tonight. Some receipt counts have
drifted since 10-23, though each row fails the same way:
`m10_9_the_pad_at_both_rule_f_dials` read `(1546, 1252, 3048)` on 10-23
and reads `(1546, 1251, 2420)` tonight, against the pin
`(1546, 1260, 3026)`. `sym_9_the_drive_writes_the_ladders_receipt`
went from `[1421, 1378, 34]` to `[1421, 1377, 34]`, against
`[1421, 1386, 34]`. So when a fix lands, re-derive the receipts on the
day. Do not take them from this file.

`k-lint (dev-probe)` still stops on
`ray_wall_margin_twins::the_wall_arms_margins_scale_linearly_with_the_model`:
`predicates whose margins do not scale with the model:
["curve_foot_branch: 6 pairs off, e.g. 9.990e2"]`.
