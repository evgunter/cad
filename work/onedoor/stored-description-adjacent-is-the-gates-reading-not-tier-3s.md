---
id: stored-description-adjacent-is-the-gates-reading-not-tier-3s
kind: issue
title: Body::stored_description_adjacent claims tier 3's at-rest adjacency reading but reads the write gate's surface-only one, so merge_faces' postcondition passes a non-wrap chart on a one-face edge tier 3 refuses
status: dispatched
opened: 2026-11-03
priority: P1
cost: E
branch: onedoor/at-rest-adjacency
---


Filed by the ONEDOOR orchestrator from the ladder fork (2026-11-03). Both designers found it independently.

`crates/topo/src/attach.rs` `Body::stored_description_adjacent` is documented as tier 3's `DescriptionNotAdjacent` reading. It actually calls `Named::adjacent_to`, which compares surfaces only and never asks which face. That surface-only reading is right for the write gate (`check_description_adjacent`), because a mint passes through states where both halves of a new edge lie on one face (`mev` before `mef`). It is wrong at rest. Tier 3's own hand-spelled reader in `crates/topo/src/validate.rs` requires wrap iff the two halves bound one face, and requires `!one_face` for the pair forms and a non-wrap `Chart`.

`merge_faces.rs` debug-asserts its postcondition with `stored_description_adjacent`. So it passes a non-wrap chart on a one-face edge that tier 3 then refuses.

**Fix:** one at-rest adjacency reading, with face identity, called by both tier 3 and `stored_description_adjacent`. The gate keeps the transient reading under a name that says so. This is the at-rest half of the ladder fork's `EdgeForm::of(sides)` (PR 6427); it lands now, independent of SELECT.
