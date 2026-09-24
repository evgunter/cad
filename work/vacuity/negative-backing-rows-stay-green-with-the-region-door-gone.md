---
id: negative-backing-rows-stay-green-with-the-region-door-gone
kind: issue
title: Seven 'backs no crossing' rows reach a Door-2 read and stay green with the region door gone: each wants a positive twin
status: open
priority: P4
cost: E
opened: 2026-09-21
---

## Finding

Measured on LANE-2's head (PR 3038, its fix pass) by two reviewers
independently: with `validate_pseudomanifold_certificate`'s
`Some(RegionLane::certified())` replaced by `None` and nothing else
changed, 24 `topo` + `sweep` rows reach a Door-2 read site in
`crates/topo/src/census.rs` (`pair_region_verified`'s consult,
`sweep_conformal_patches`' arm, `confirm_curve_and_patch_records`'
Door 2 — a `panic!` planted in each `None` arm fires them). Seventeen
go red; **seven stay green with the door gone**, because each asserts
only that a declared pair backs NO crossing — an assertion that holds
just as well when the consult that would back it is not made at all:

- `topo::m9_c1_r1_probes::the_swapped_graft_order_flush_seat_exercises_the_other_arm`
- `topo::mate9_crossing_rung::a_verified_pair_elsewhere_backs_no_crossing`
- `topo::mate9_crossing_rung::an_unverified_point_holding_pair_backs_no_crossing`
- `topo::r1_mate8_probes::r1_the_four_cells_of_the_table`
- `topo::review_mate9_r1_probes::r1_a_diving_edge_crossing_is_not_backed_by_the_seat_pair`
- `topo::review_mate9_r2_probes::r2_an_unverified_opposed_pair_backs_no_crossing`
- `sweep::verbs_pierce_r2_probes::r2_the_1032_declaration_measurement_reproduces`

The assertion is monotone the wrong way: the mutant that removes the
rung's whole backing mechanism makes every one of them MORE true.
Each row is still a real pin of its own claim (a wrongly-backed
crossing would red it); what it cannot see is the mechanism going
away, and a positive twin beside it — the same body with the pair
that DOES back, asserting the crossing is absorbed — is what would.
`mate9_crossing_rung::the_declared_crossing_seat_certifies_both_ways`
is the shape, and is one of the seventeen that red.

Not rewritten in the LANE-2 fix pass (out of that unit's fence and
not its finding to fix in passing); the red-first receipt in PR 3038's
body names the 17, the 7 and the 35 `editor-core` rows the same
mutant reds.

## Recourse

For each of the seven: a positive twin on the same fixture, or a
sentence in the row's doc saying which existing row is its twin. No
row is deleted; no assertion is weakened.
