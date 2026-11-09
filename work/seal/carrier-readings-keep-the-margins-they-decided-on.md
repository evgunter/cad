---
id: carrier-readings-keep-the-margins-they-decided-on
kind: unit
title: carrier and sector readings keep the Decided they decided on (CarrierVerdict, IdentityLadder::Apart, NotTangent, bounds_off), off_ring carries its TorusRefusal, and the census's mechanical survivors take their doors
status: dispatched
opened: 2026-11-09
priority: P1
cost: M
parent: glue-disagreements-and-held-contradictions-after-stage-4
---


Step 1 of the parent's design (fork `glue-disagreements-carry`, 2026-11-09). Each reading below already decides through the funnel, and then drops the `geom_core::Decided` it decided on. This step makes each one keep it.

## What

- **`topo::CarrierVerdict::{One { relation, margin }, Apart(Decided)}`** replaces `(CarrierRelation, Option<MarginDiag>)`.
  - "One carrier on no margin" then stops typechecking. That retires the glue door's `ClassificationInvariant` for it and `rest_pair_verdict`'s `unreachable!`.
  - `Apart` carries the datum that decided it, as `TorusDatum` does.
- **geom-brep keeps its margins.** `IdentityLadder::Apart { at, decided }` and `TangentLocusError::NotTangent { apart, decided }` keep their decided margins. The plane ladder's orientation rung decides through `decide_nonzero_reported`.
- **`BoolSector::bounds_off` keeps its `Decided`.** Today `decide_nonzero` returns a bare sign. `a-corner-reads-only-its-own-sectors` reads it.
- **`off_ring` carries its torus refusal.** `TorusCoincidence::OffRing` says which torus. `CarrierEqError` gains `OffRing(TorusRefusal)`, which ends in the convention's lever as `BooleanError::DegenerateTorus` already does at the pierce normal, not as `PoisonedCarrierDatum`.
- **The census's mechanical survivors** (the parent's Re-census table):
  - I7 `contact_tangent_second_order` goes through `decide_positive`.
  - rim_wedge `seam_traversal_unread` goes through `decide_nonzero`.
  - merge_faces' two `merge_pair_extent` arms and rim_wedge's `seam_cover_unread` get typed structural arms, not `Escalated{INVALID}`. The precedent is `unreadable_extent` → `InvalidInput`.
- **I1, the Rest door's senses:** evidence `None` for both planes and curved kinds. A plane's sense is a cosine of about ±1: sign-certain, with no size anyone means.

## Done when

None of the sites above names `MarginDiag::INVALID` or builds an `Indeterminate` by hand. Each moved arm's ending is pinned before and after, per the parent row's rule.

## Dispatch (2026-11-09)

Branch `seal/carrier-readings-keep-margins`. Review tier: **single, FULL**. The step threads kept margins through public types in topo and geom-brep and moves arm endings, so believing it takes more than reading it. It is no design decision: the fork settled the shapes.
