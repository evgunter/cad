---
id: census-face-rungs-for-a-collinear-overlap-are-unconfined
kind: issue
title: The census's face rungs for an edge–edge overlap, ee_face_backed and vv_face_backed, back by structural incidence and are scheduled for no migration
status: closed
opened: 2026-10-11
priority: P3
cost: M
refs: [ef-bound-backed-migrates-to-region-confinement, declared-only-meetings-clear-at-the-census-gate-unread]
closed: 2026-10-23
pr: 5605
branch: intent/census-planar-face-pair
---

## What

`ee_collinear_lane` (`crates/topo/src/census.rs`) certifies a collinear
edge–edge overlap when the pair holds its edge–edge record, or when
`Declared::ee_face_backed` finds a declared face pair naming a face each
edge bounds (PR 4740). `ee_face_backed` asks whether the pair holds the
two edges, never where the overlap lies, so it backs an overlap outside
the pair's own overlap region: `m9_c1_rest_face_rung::the_rung_backs_an_event_outside_the_declared_pairs_overlap_region`
(two coplanar faces touching along one line, no shared area) passes
through it.

That is the strength of the other grandfathered rungs (the census
module's GRANDFATHERED BY NAME list, `crates/topo/README.md`'s
Grandfathered rungs), which migrate to the unified strength one at a
time, measured. `ef_bound_backed`'s face-pair arms have migrated
(`Declared::vf_face_backed_confined`). No row schedules the rest.
`vv_face_backed`, which backs the overlap's two vertex–vertex bounds,
is the rung `ee_face_backed` pairs with: the same eight rows depend on
each, so the two migrate together.

## Done when

`ee_face_backed` and `vv_face_backed` back only where the pair passes
Door 1 and Door 2 reaches `PositiveArea` (as
`Declared::vf_face_backed_confined` does), each rung's dependent rows
measured before and after; or a row states, with that measurement, why
one of them keeps structural incidence.

## 2026-10-17 — parked on the D10 hold, moved from RESTREAD

RESTREAD read this row against the D10 hold (`docs/DESIGN.md`, State of
the build) and the open [ev] PR 4666 (the census records what it
decides): it stands on contact records, declared pairs or the shape of
at-rest findings, which INTENT stages 4 and 5 rebuild. Per RESTREAD's
plan (a row needing declared pairs or contact parks and moves to
CONTACTHOLD), it moved here by `git mv` with its id unchanged. Priced
`M` where it carried no cost. (RESTREAD orchestrator)

## Closed (2026-10-23, PR 5605)

Overtaken by INTENT's by-location confinement (`docs/DESIGN.md`
`[assertions]`, Ev on PR 5605). `ee_face_backed` and `vv_face_backed`
are gone. A collinear overlap and its bounds are now the confirmed
pair's only where they lie in the closure of the pair's overlap
(`census.rs` `Declared::confine` in `pair_edge_edge`, `Declared::holds`
in `ee_bound_backed`), and the pair is confirmed only where both doors
answer (`Declared::confirm`). That is stronger than the Done-when's
`PositiveArea` gate. The line-contact row it cites is now
`m9_c1_rest_face_rung::the_rung_backs_no_event_outside_the_declared_pairs_overlap_region`.
The location rows are `review_intent_census_r1_probes` and
`review_census_r2_probes`, including a collinear run outside a
non-convex overlap.

