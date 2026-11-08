---
id: stored-description-adjacent-is-the-gates-reading-not-tier-3s
kind: issue
title: Body::stored_description_adjacent claims tier 3's at-rest adjacency reading but reads the write gate's surface-only one, so merge_faces' postcondition passes a non-wrap chart on a one-face edge tier 3 refuses
status: closed
opened: 2026-11-03
priority: P1
cost: E
branch: onedoor/at-rest-adjacency
closed: 2026-11-03
pr: 6440
---


Filed by the ONEDOOR orchestrator from the ladder fork (2026-11-03). Both designers found it independently.

`crates/topo/src/attach.rs` `Body::stored_description_adjacent` is documented as tier 3's `DescriptionNotAdjacent` reading. It actually calls `Named::adjacent_to`, which compares surfaces only and never asks which face. That surface-only reading is right for the write gate (`check_description_adjacent`), because a mint passes through states where both halves of a new edge lie on one face (`mev` before `mef`). It is wrong at rest. Tier 3's own hand-spelled reader in `crates/topo/src/validate.rs` requires wrap iff the two halves bound one face, and requires `!one_face` for the pair forms and a non-wrap `Chart`.

`merge_faces.rs` debug-asserts its postcondition with `stored_description_adjacent`. So it passes a non-wrap chart on a one-face edge that tier 3 then refuses.

**Fix:** one at-rest adjacency reading, with face identity, called by both tier 3 and `stored_description_adjacent`. The gate keeps the transient reading under a name that says so. This is the at-rest half of the ladder fork's `EdgeForm::of(sides)` (PR 6427); it lands now, independent of SELECT.

## Closed (PR 6440, 2026-11-03)

`Named::adjacent_at_rest` (attach.rs) is the one at-rest reading. It is one match per form, and it adds D1's face count to the surface reading. Five readers ask it:
- tier 3;
- `Body::stored_description_adjacent`, and through it `merge_faces`' postcondition;
- `tangent_struts`;
- `rest_parted_wrap_edges`.

The surface-only reading is `Named::adjacent_by_surfaces`, named for what it reads. Two callers take it:
- the write gate, because mints pass through one-face edges;
- the re-chart planner, because its sides are slots.

Tier 3's verdicts are unchanged by construction. A body-level row pins the refusal at the defect's site.

`describe_edges`' stale ladder still spells its own reading without face identity. That is disclosed on `description-staleness-ladder-three-spellings`, where `merge_faces`' postcondition is now its tripwire.
