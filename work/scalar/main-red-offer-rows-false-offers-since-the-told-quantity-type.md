---
id: main-red-offer-rows-false-offers-since-the-told-quantity-type
kind: issue
title: Main red (slow set): offer_rows::every_offered_tolerance_passes_just_below_it makes false offers on three rows since PR 7159 (the told-quantity type)
status: open
opened: 2026-11-20
priority: P1
cost: M
refs: [7159]
---


Filed by GERM's orchestrator (2026-11-20), from a bisect by a GERM lane.

## What

`topo boolean::refusal_routes::offer_rows::every_offered_tolerance_passes_just_below_it`
(nightly slow set) is red on `origin/main` at `a512d228e3`. It offers a tolerance
whose margin does not pass just below it:

- `lever_arm_sector_side_in_the_zero_band` offers 3.535e-11, where the margin gives 3.5355e-11;
- `conic_bend_short_of_the_band` and `tangent_side_in_band` each offer 5.499e-10, where the margin gives 5.5e-10.

## Culprit

A first-parent `git bisect run` over main's merges since 2026-11-20T08:00 names
`bd8b2c9bb5`, the merge of **PR 7159** (`scalar/told-quantity-type`). Its first
parent `4d2d994295` passes, and every bisect step after it fails. No GERM merge
(7171, 7174, 7179, 7183, 7184, 7209, 7217) turned the row red.

The offered values are the margins rounded to four significant figures, below
the margin, which reads like a told quantity rendering the offer at its stated
precision and not at the margin's value. Unverified.

## Owed

Make each offer pass just below it, never by weakening the row. Reproduce with
`cargo nextest run -p topo -E 'test(every_offered_tolerance_passes_just_below_it)'`
(slow set; it runs nightly, not on a PR gate).

## 2026-11-21: two more rows red by the same merge, and the row's count (ONEROUND)

Found by ONEROUND's lane on main `5341f713`, `cargo nextest run -p topo --all-features --ignore-default-filter`.

**Two rows of `crates/topo/tests/a_corner_reads_only_its_own_sectors.rs` (slow set) are red since the same merge.** Both pass on `4d2d9942` and fail on `bd8b2c9b` (PR 7159), and on every main since:

- `a_corner_reads_only_its_own_sectors::the_corners_offer_executes`: "generic_corner_six_runs: offers 9.999999999999951e-12, quoted 9.999e-12".
- `a_corner_reads_only_its_own_sectors::an_unglued_pair_parallel_at_the_arm_refuses_on_the_arms_tilt`: "generic, θ = 300ε, B ∩ A: offers the arm's tilt over K, 2.999e-11: … margin 3.0000000000031634e-10 lies within the zero band (±1e-9). Recourse: …, tighten the tolerance below 3e-11 m".

Each compares the offer the text quotes (`test_utils::offer::offered_below`) with the margin's offer to one part in 10⁶, and the text now carries four figures cut downward, one part in 10⁴ off. The second message holds both spellings: the sentence says "below 3e-11 m" and the number read back from it is 2.999e-11.

**`every_offered_tolerance_passes_just_below_it` names 16 false offers on main `5341f713` (and on `0ad2e0b5`)**, not the three above, all of the one shape (5.499e-10 against 5.5e-10, 5.756e-11 against 5.756955e-11, …): `lever_arm_sector_side_in_the_zero_band`, `conic_bend_short_of_the_band`, `tangent_side_in_band`, `thinner_wedge_tilted_past_the_band`, `tool_on_a_block_corner_tilted_at_its_arm`, `corner_tilted_in_band_at_its_arm`, the three `wedge_turned_on_a_block_*`, `tangent_side_curving_away`, `arc_root_just_inside_its_span`, `vertex_near_a_vertex_of_a_curved_face`, `neighbours_bent_at_the_band`, `roots_a_hair_apart`, `roots_a_hair_apart_across_the_window`, `seam_barely_bending_apart`.
