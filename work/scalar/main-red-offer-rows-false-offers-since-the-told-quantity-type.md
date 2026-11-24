---
id: main-red-offer-rows-false-offers-since-the-told-quantity-type
kind: issue
title: Main red (slow set): offer_rows::every_offered_tolerance_passes_just_below_it makes false offers on three rows since PR 7159 (the told-quantity type)
status: closed
opened: 2026-11-20
priority: P1
cost: M
refs: [7159]
branch: scalar/main-red-since-the-told-quantity-type
closed: 2026-11-21
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

## 2026-11-21: dispatched (SCALAR)

One unit for this row and `work/green/main-red-three-source-censuses-red-since-the-told-quantity-type.md`. Difficulty letter **M**; review tier **single, FULL**: the census repairs are mechanical, but what the offer rows should pin under the told rule is a judgement a second reader should check.

## Evidence: sweep's sphere offer row (ONEROUND, 2026-11-22)

`sweep offer_rows::every_sphere_offer_passes_just_below_it` (slow set) is red on main the same way, on both sides of PR 7227's differential and in its review. `nested_off_axis_in_the_zero_band_union` and `…_subtract` each quote `None` and offer `Some(4.999e-11)` where the computed margin is `5e-10`: the offer is the margin over the band's ratio rendered at four significant figures, just below it, as in the three topo rows above. Read as the same cause, not bisected. Reproduce: `cargo nextest run -p sweep -E 'test(=offer_rows::every_sphere_offer_passes_just_below_it)'`.

- 2026-11-22 — **`a_corner_reads_only_its_own_sectors::an_unglued_pair_parallel_at_the_arm_refuses_on_the_arms_tilt` is still red on main** (EMBED, lane strip): on main `9bc9d6517`, under CI's build (`--profile ci`, test crates at opt-0, the rest at opt-1), with the same offer of 2.999e-11 against the text's "below 3e-11 m" at `a_corner_reads_only_its_own_sectors.rs:122`.

## Red at every ε row (GREEN U9, 2026-11-22)

(GREEN, `green/held-reds`) Two lanes cited this test to GREEN's closed
`main-red-two-offered-tolerances-fail-just-below-their-margin`; this row
holds it. The nightly of main `1942290ead` (run 44989234943) has it red
at the default ε, 1e-6 and 1e-12 with one message
(`offer_rows.rs:2044`, `lever_arm_sector_side_in_the_zero_band: offers
Some(3.535e-11), its margin gives Some(3.535533905932738e-11)`,
`conic_bend_short_of_the_band` and `tangent_side_in_band` 5.499e-10
against 5.5e-10, …), so PR 7267's fix is owed at every ε row.

On current main (`5ea1ac0a70`, hosted, throwaway run 45051677551, the
row out of the slow set) it is still red at all three ε, with the same
message.

## 2026-11-22: closed (SCALAR, PR 7267)

The rows were wrong, not the text. Every quoted offer is the told rule's offered bound of the kernel's own margin: four figures, cut to the side that works. The rows held the quote to the raw `|m|/K` at 1e-4 or 1e-6, a spelling the rule no longer makes.

The two spellings in one message come from two margins, either side of a grid point. The row's `want` was `offer_deciding` of the row's model margin (2.9999999999998656e-10, told 2.999e-11). The sentence's was of the measured margin (3.0000000000031634e-10, told 3e-11). The 5.499e-10 quotes are the same straddle: a measured margin at least a float below the case's `D = 5.5e-9`.

Each row now holds the quote with `geom_core::test_support::is_offer_of_a_margin_within`, which has a row of its own. It checks the quote two ways that answer independently:
- against the margin itself: at most `(|m| + slack)/K`, and within the grid's step of `(|m| − slack)/K`;
- against the rule's answer at the window's ends.

The corner row with the measured margin in hand also checks `is_told_offer_of`. `the_corners_offer_executes` runs just below the quoted number.

Sweep's `every_sphere_offer_passes_just_below_it` (the evidence above) takes the same check. It passes on PR 7267's head `2189e77282`, CI run 45073869052, slow-set step, beside the topo rows.
