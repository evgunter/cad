---
id: past-payload-word-and-negative-arm-collapsed-render-are-unguarded
kind: issue
title: geom-core/editor-core: Shows::Past's payload word has no pin, and the negative ArmCollapsed render has no concision row
status: open
opened: 2026-10-14
priority: P4
cost: E
---

(Filed by the ENCL orchestrator from PR 5023's delta review.)

## What

Two outputs on the sized-ending path go unguarded:

- **`Shows::Past`'s payload word** (geom-core `predicate.rs`, `Indeterminate::reading`). The delta review gave Past alone a different word, and all 946 geom-core tests stayed green. `Refused`'s word is pinned through `k_stats_doors::a_rejection_tells_the_decided_margin_it_carries`. Past may be unreachable through a gate rejection; nobody has confirmed either way. If it is reachable, it wants a pin. If it is not, it wants the reason stated where the word is chosen.
- **The negative `ArmCollapsed` render** (`CertifyError::ArmCollapsed` with `Refused::Negative`, ending in `DIHEDRAL_ARM`'s note). It has no editor-core `refusal_concision_chains` row; only the zero one does, at 73 of 75 words. It should come to the same word count ("margin -1e-3 m" against "0e0 m"), but nothing checks that.

## Ask

Pin or justify Past's word, and add the negative ArmCollapsed row to the concision chains.

## Progress (PR 5349)

The negative `ArmCollapsed` row is in `refusal_concision_chains::every_certify_refusal_ends_in_its_routed_sentence` ("negative lever arm"). `Shows::Past`'s payload word is still unpinned.
