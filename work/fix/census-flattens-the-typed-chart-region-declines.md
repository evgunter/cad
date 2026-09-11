---
id: census-flattens-the-typed-chart-region-declines
kind: issue
title: the census maps every chart-region refusal onto one CensusUnsupported, so a typed decline is legible at the chart door and invisible at the census
status: closed
opened: 2026-09-04
branch: fix/census-typed-chart-declines
pr: 2354
closed: 2026-09-11
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

## What landed

**Shape 1, the carried cause.** `ValidationError::CensusUnsupported`
gains `cause: CensusUnsupportedCause` — `ChartRegion(ChartRegionError)`
for the two exhaustive chart matches, `ContactLane(&'static str)` for
the two `ContactRefusal::NotCertifiable` arms whose `what` the census
was discarding, `FaceUnboundable` for the face-bounding sweep. The
`Display` renders the lane's own sentence, recourse included, exactly
as `CensusEscalated` renders `Indeterminate`; the blanket *"declare and
certify through a supported lane, or separate the geometry"* tail is
gone, because it was the inventory lanes' repair and the wrong
instruction for the rest.

**Why not shape 2 (a second variant).** The split it needs is not
clean. Of the twelve chart-region arms only `WitnessBudgetExhausted`
is "the schedule stopped". `RayExhausted` is named for exhaustion and
is not one — the schedule is FIXED and every ray of it grazed, so the
query is ill-conditioned at this ε and no extra budget decides it.
`MissingCache` is a fact about the BODY (re-mint the pcurves) and
`Corrupt` is a kernel-invariant violation; neither is a geometric
undecidability. A two-way split would have put four arms in buckets
that misname them — a refusal whose stated cause is not the true one,
which is the defect this item exists to fix. The cause carries the arm
itself and pre-judges nothing.

**It moves no answer.** `editor_core::assembly::attribute` dispatches
on the `ValidationError` variant and on the `CensusSubject` SHAPE; a
payload participates in neither, so every finding classifies exactly
as before and no `AssemblyError::AtRest`/`Uncertified` verdict changes.
That is right on the merits and not merely convenient: the decline
relation is "the census neither certified nor contradicted this
declaration", which holds whichever lane declined. The item's claim
that *"a new variant or a carried cause moves the `AtRest`/`Uncertified`
decision with it"* is true of the first and not of the second, and the
invariance is now a row
(`the_decline_relation_does_not_depend_on_which_lane_declined`).

**The carriers now owe the recourse, and pay it.** Dropping the
census's blanket tail exposed that `ChartRegionError`'s header claim —
*"every arm names its recourse"* — was false of NINE of its thirteen
arms; the census had been papering over it while naming the wrong
repair for every arm it did not describe. All nine have one now, and
`every_chart_region_arm_names_a_recourse` is the floor that keeps the
claim true. The one deliberate exception is `ContactRefusal::
NotCertifiable`, which carries none because `contact.rs` ratified that
it must not.

**The Python word is NOT here.** It is LIB's row: the prelude cannot
publish a new refusal payload without the CUR3 property row
(`pncad/tests/all.rs::carried_refusal_payloads_are_matchable_through_the_prelude`)
that makes "a refusal the prelude names is matchable through the
prelude" mechanical, and without repairing the six prose copies of the
premise this unit's own cause doc falsifies. The kernel carries the
fact; the Python door crosses it in a LIB-owned change.

`scripts/payload-rung-sweep.py` caught the consequence on the first
run — a new payload rung under a curated carrier, on no curated list —
and the non-carriage is argued beside the declaration in `validate.rs`
with its falsifier, registered `argued` in that script's disposition
table. The row is deleted when the facade carries the cause.
