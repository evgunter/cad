---
id: main-red-m10-bulge-d-tab-radial-moving-pcurve-chart-moved
kind: issue
title: main red (slow set): both m10_bulge d_tab rows read pcurve_chart_radial_moving [22, 20] -> [18, 24] at the nominal, since 11-03..11-07
status: open
opened: 2026-11-09
priority: P0
---

Nightly run 42567046151 (sha `836426e07e`), both slow-set rows: `crates/editor-core/tests/m10_bulge_interval.rs` `m10_bulge_the_d_tabs_literal_split_at_the_nominal` and `…_parameter_split_at_the_nominal` (through `m10_8_harness.rs`): `these moved at the nominal (symbolic/numeric); if a rule took them, re-pin and say which: ["pcurve_chart_radial_moving [22, 20] -> [18, 24]"]`.

Green in the 11-03 nightly (run 41583529337), red in the 11-07 one (run 42342466395) with the same reading. No BOXES merge falls between them, so this is not BOXES'.

(BOXES nightly-reds triage, 2026-11-09)

## Bisected (ONECOPY, `onecopy/main-reds-triage`, 2026-11-09)

Both rows move at PR 6504 (`pcert/one-ruling-reading`, merge `143d99ec29`, 2026-11-04), first-parent at ε 1e-9. They pass at `143d99ec29^1` and read `pcurve_chart_radial_moving [22, 20] -> [18, 24]` from `143d99ec29` on. PR 6504's description does not disclose it. The same merge moved `sym11_the_exact_channel_never_contradicts_past_the_ceiling` and the plate's walk ledger (`m10_sym_profile_interval`, re-pinned in PR 6663). The harness asks "if a rule took them, re-pin and say which". Whether the one ruling reading is the rule that took them is PCERT's to answer before re-pinning.
