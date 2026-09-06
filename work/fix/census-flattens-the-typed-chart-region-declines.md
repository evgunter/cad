---
id: census-flattens-the-typed-chart-region-declines
kind: issue
title: the census maps every chart-region refusal onto one CensusUnsupported, so a typed decline is legible at the chart door and invisible at the census
status: open
opened: 2026-09-04
---


## The gap

`crates/topo/src/census.rs:1663` and `:2776` are the two deliberately
exhaustive `ChartRegionError` matches. Both map **eleven** typed
refusals — `ChartDivergence`, `NonPlanarTrim`, `MissingCache`,
`ArmUnbounded`, `SeamBranch`, `PeriodFold`, `CarrierTilt`,
`TouchingBoundary`, `DegenerateLoop`, `RayExhausted`,
`WitnessBudgetExhausted` — onto one `ValidationError::CensusUnsupported`
carrying only its subject. Whatever the chart-region lane said, the
census consumer reads "outside the certifiable inventory".

So the distinction PR 1750 built is legible to a caller of the public
chart-region door (`declared_pair_overlap`, `chart_region_overlap`) and
invisible to a caller of the census (`validate_geometric_declared`,
`editor_core::assemble`). A fat, decidable overlap whose search was cut
off by `WITNESS_BUDGET` still reaches `assemble` as the same finding a
genuinely thin overlap does.

## Why PR 1750 did not close it, and why that disposition is right

`CensusUnsupported`'s `Display` has never distinguished any of the
eleven, so singling out the newest one would be a half-measure — and
not a free one: `editor_core::assembly::attribute`'s classification is
a dispatch on the `ValidationError` variant, so a new variant or a
carried cause moves the `AtRest`/`Uncertified` decision with it. That
is a door-shape question, not a threading one.

Point 1 of issue 1478 asked for the arm to exist and to thread the
matches, and that is what landed. What is scheduled here is the
carry-through.

## Why this is a file and not a paragraph

Issue 1478 exists **because** MATE-8 left a deviation disclosed but
unscheduled, and PR 1750 disclosed this one in its body and in its
item's `## Closed` section — which is the same shape one deferral on.
A `## Closed` section lives in `work/fix/`, and `work/README.md`'s
closed-program rule deletes that directory when FIX closes: the
disclosure would go with it and only the PR body would remain. A
finding with no durable home cannot warn anyone.

## Shape of a fix, not a decision taken here

Either the census's refusal carries the chart-region cause (a payload,
with `attribute`'s dispatch re-decided for it), or a second variant
splits "cannot decide this geometry" from "the schedule stopped" and
every consumer classifies both. Both are door-shape work with their own
census rows; neither is a threading change.

## Home

`crates/topo/src/census.rs` — CURVED's and S-MATE's territory, edited
by FIX under the same recorded seam PR 1750 crossed.
