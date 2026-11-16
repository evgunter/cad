---
id: main-red-the-probe-station-counts-double-and-the-gather-pays-one-more
kind: issue
title: main red (--all-features, k-lint dev-probe): four must-carry station counts read double and assemble_one_local_battery's gather pays one sample more, since the 11-08 onehome merges
status: closed
opened: 2026-11-09
priority: P0
closed: 2026-11-14
---

Nightly run 42567046151 (sha `836426e07e`), jobs `suite under --all-features` and `k-lint (dev-probe)`. These rows count `k_stats` samples under the `probe` feature:

- `crates/sweep/tests/must_carry_rule.rs` `a_filleted_block_spends_the_rules_stations_once_per_smooth_strut`: 224, want 112;
- `crates/sweep/tests/contact_edge_must_carry.rs` `each_contact_edge_spends_the_rules_stations_once_beside_the_certificates`: 112, want 56;
- `crates/sweep/tests/review_contact_edge_must_carry_r1_probes.rs` `r1_the_die_spends_…`: 1344, want 672;
- `crates/sweep/tests/review_contact_edge_must_carry_r2_probes.rs` `r2_the_rules_stations_scale_with_the_contact_edge_count`: 56, want 28;
- `crates/editor-core/tests/assemble_one_local_battery.rs` `the_gather_pays_tier_3_prime_once_and_the_at_rest_check_nothing`: the gather recorded 2409 samples and tier 3′ 2408.

Each sweep count is exactly double the pin. That reads as the fillet's output paying `tangent_second_order`'s certificate twice; it is not yet measured.

All five were green in the 11-07 nightly (run 42342466395) and are already red at `7cbccda83d`, the first parent of BOXES' PR 6579 (built and run there with `--all-features`). So they arrived in the window after the 11-07 nightly (`45cae9d7a4`) and before PR 6579 (`3b7794bb5d^1` = `7cbccda83d`), whose code merges are PRs 6371 (`onehome/finished-body`), 6578 (`placing/place-live`), 6380 (`onehome/run-owns-rounding`) and 6586 (`onehome/fix-moved-row-cites`). Not BOXES'.

(BOXES nightly-reds triage, 2026-11-09)

## Closed (2026-11-14, tracker review)

Nothing here is left without a row. The gather half (2409 samples against 2408) is still red and is `work/onehome/main-red-the-gather-records-one-sample-tier-3-prime-does-not.md`. The four doubled station counts are no longer red: nightly 43161026932 (2026-11-12) ran `suite under --all-features` whole, 15710 tests, and its one failure was the gather row. That is read from the job's failure list, not from the four tests' own PASS lines. On 11-13 (run 43371638371) the all-features job did not finish its build and `k-lint (dev-probe)` stops at the gather row, so neither ran them.
