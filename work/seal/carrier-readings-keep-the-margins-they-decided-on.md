---
id: carrier-readings-keep-the-margins-they-decided-on
kind: unit
title: carrier and sector readings keep the Decided they decided on (CarrierVerdict, IdentityLadder::Apart, NotTangent, bounds_off), off_ring carries its TorusRefusal, and the census's mechanical survivors take their doors
status: closed
opened: 2026-11-09
priority: P1
cost: M
closed: 2026-11-09
pr: 6634
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

## Closed (2026-11-09, PR 6634)

Each reading on the row's list keeps the `geom_core::Decided` it decided on. The PR body has the per-site table (each ending before and after), the sweep, and the premise corrections.

- **`CarrierVerdict`** (`boolean/carrier_eq.rs`) is `One { relation, margin } | Apart(Decided) | TwoKinds`.
  - `TwoKinds` is the kind-mismatch arm: no datum decides it, so it carries no margin (the design's "two kinds" structural fact).
  - The glue door's "one carrier on no margin" invariant is gone, as are its two twins at the corner (`vtxfac`, `recl`) and `rest_pair_verdict`'s `unreachable!`.
- **geom-brep keeps its readings:**
  - `IdentityLadder::Apart { at, decided }`.
  - `TangentLocusError::NotTangent { apart, predicate, decided }`.
  - `TorusCoincidence::Apart { datum, by }`, the shape step 2 extends with `upper`.
  - `TorusCoincidence::OffRing { torus, refusal }`.
- **`CarrierEqError::OffRing { torus, refusal }`.** At the Boolean it ends as the pierce normal does (`boolean::off_ring_torus`: `DegenerateTorus` when decided, `Escalated { Torus(half) }` in band), in the half's lever.
- **`BoolSector::bounds_off`** returns its `Decided`.
- **I7** is decided through `decide_reported`, and its refusal on a decided reading is `k_stats::gate_rejection` under `SizedPass::Positive`, the gate `decide_positive` stands for. Deciding first keeps the deferred arm's log a verdict, as on main.
- **`seam_traversal_unread`** goes through `decide_nonzero`.
- **Typed structural arms** for the extent and cover reads:
  - The merge refuses `MergeCoplanarError::PairExtentUnread` (defect ending), and `MergeDecision::PairReach` is gone.
  - `seam_cover_unread` is now `RimUnread::Unplaced`, ending as the tangent line's own placement does: `PointInFaceRefused` / `CurvedBooleanUnsupported`.
- **Not done here:**
  - **I1/I2's evidence** (`contact_rest_senses_opposed` / `contact_rest_one_carrier` still mint `Contradicted{INVALID}`). Their evidence `None` has no spelling until step 3's Rest verdict replaces `ContactRefusal::Contradicted { diag: Indeterminate }`.
  - **The plane ladder's orientation rung** stays on `decide_nonzero`. The settled design gives its margin no reader (I1's evidence is `None`), and joining it into `One`'s margin would unprove every Sym Zero.
- **Filed from the review:**
  - `a-torus-refusals-ending-is-dispatched-by-hand-at-four-doors`
  - `a-one-carrier-verdict-can-still-hold-distinct`
  - `plane-only-readers-hold-a-dead-off-ring-arm`
  - `the-corners-senses-decide-the-plane-orientation-a-second-time`
