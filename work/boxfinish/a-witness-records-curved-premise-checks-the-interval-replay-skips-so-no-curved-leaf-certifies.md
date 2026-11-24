---
id: a-witness-records-curved-premise-checks-the-interval-replay-skips-so-no-curved-leaf-certifies
kind: issue
title: the f64 witness records a curved face's props premise checks that the interval leaf replay skips, so the drive refuses every curved leaf as a FlipCrossing with empty evidence
status: closed
opened: 2026-11-23
priority: P0
cost: M
branch: boxfinish/witness-premises
closed: 2026-11-24
---


Found by OFFER while merging `origin/main` into PR 7369 (the certified range search). A red row on that branch, `docm9_range::the_directed_search_proves_ground_on_the_plate_the_level_drive_cannot`, comes from main's own `drive`, not from the search.

## What is seen

On the corpus plate (`corpus::plate_param`), with `hole_r` banded `[0, ε/4]` or `[0, ε/8]`, `drive::drive` at depth 0 refuses its one leaf as `RefusalReason::FlipCrossing` whose `FlipEvidence` is EMPTY (`verdicts.nodes = {}`, `structure = []`). Before the merge these leaves certified. They were the plate's first proof, at about `ε/4`.

A leaf that holds the witness's own value cannot be decided definitely different. And a flip with nothing in it names no cause.

## Why

`classify_replay`'s exact comparison (`certifying_vector(doc, leaf) == witness_vector`) fails, but `diff_verdicts` finds no flip. The vectors differ in LENGTH, not in a sign:

- Node ordinal 64 of the plate records 12078 verdicts at the `f64` witness and 11854 in the leaf replay.
- The first difference is verdict #2169. The witness has `props_circle_axis_class` (Check, Positive) where the leaf has `positive_volume_exact`.
- So the `f64` witness decides the curved face's closed-form premises (`crates/geom-brep/src/props/curved.rs`, the `props_circle_axis_class` sites), and the interval leaf replay does not record them.

That lines up with #7344 (`boxfinish/volume-premises`, commit `c4240cc65`): "a curved face's closed-form premises are decided once, at the scalar its body lives at". It merged after this branch's previous base, and nothing else in the merge touches the comparison (`drive.rs`'s comparison and `diff_verdicts` are unchanged). I could not bisect for disk space; the attribution is by the predicate and the commit that added its sites (`git log -S'props_circle_axis_class'`).

## Consequence

Every curved document's leaves now compare unequal to their witness. So `drive` and the range search certify nothing on curved geometry, and report the refusal as a decided difference with empty evidence rather than as undecided. The range search reads that as "proven different at 0", which the viewer would render as a boundary at the current value.

## What would close it

Either the premise checks are recorded identically at both scalars (decided once, but logged in both runs), or `certifying_vector` leaves out premise checks that only one lane runs. A row: the plate's `hole_r` leaf `[0, ε/4]` certifies (the range row above is one).

## Dispatched (2026-11-24)

Re-banded P0: if the report holds, the drive and the range search
certify nothing on curved geometry since PR 7344, and a consumer reads
an empty flip as "proven different". Cost M.

**Letter M; tier single FULL** (it changes what a leaf's verdict vector
holds and what the driver says of a comparison that fails with no
differing decision; a mistake there certifies or refuses wrongly, so
the review carries the correctness claims). The implementer is PR
7344's arm continued, as a later escape's repair is; no fresh draw.
Branch `boxfinish/witness-premises`; measure first, with an interim
note before the build. (BOXFINISH orchestrator)

## Closed

PR PRNUM (2026-11-24). The rounding lane's interval re-read of a curved
closed-form face (`lifted_closed_form`, `crates/topo/src/props.rs`)
runs under `k_stats::unlogged` on ledger row F48, so the `f64` witness
states the decisions the leaf's replay states and no others; a refusal
inside it still refuses the face by its payload. The plate's
`hole_r` leaves certify: `docm9_range`'s
`the_plates_narrow_hole_radius_leaves_certify_against_their_witness`
and `the_directed_search_proves_ground_on_the_plate_the_level_drive_cannot`,
both red on the base tree; the two `props::lifted_pass_rows` rows in
`topo` are each red under a named mutant. No `f64` pin moved.

A parted path with no turn is still reported as an empty
`FlipCrossing`:
`a-parted-path-with-no-turn-is-reported-as-an-empty-flip-crossing`
holds it. The sweep's one hit is
`limb-2-logs-other-decisions-at-an-enclosure-scalar-than-at-the-witness`.
