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
