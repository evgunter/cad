---
id: topo-poisoned-escalations-offer-unfollowable-endings
kind: issue
title: topo: PointInSolid, SplitReduce and census escalations offer declare/move recourses on poisoned or contradicted margins
status: closed
closed: 2026-10-10
branch: encl/topo-poisoned-endings
pr: 4497
opened: 2026-10-09
priority: P3
cost: M
---



(Unit 4 of `hand-minted-invalid-gates-in-topo`'s scope.)

## What

These topo escalations end in a recourse that a poisoned or contradicted margin cannot follow:
- **`PointInSolidError::Escalated`** always offers declare/move: `solid_contain` `:2756`, `:2762`, `:3530` (empty face) and `:5401`.
- **`SplitReduceError::SliverSector`** offers "move the split plane" for contradictions: `splitting/rules` `:319`, `:488` and `:500`.
- **`CensusEscalated`'s poison arm** offers declare: `census:2583`.
- **Still to check:** `ChartRegionError::Escalated`, `RegionRefusal::Escalated`, `SectionError::Escalated` and `ContainError::Escalated { Decided }`.

PR 4453 fixed this class for `Unsized::LastResort`, and `validate::own_close` already maps poison to the defect ending.

## Repair shape

Each error's text reads poison (`RefusedArm::unreadable` / `MarginDiag::is_invalid`, the one fact a margin may route on) and ends a poisoned or contradicted arm in the reading's defect ending. Pin a poisoned row per error. Coordinate with `sized-poisoned-ending-ignores-the-reading-and-the-file`, which gives the poison→note rule one home. `census:2583` and `rules:319` also sit on CLEAVE's parked `topo-mints-indeterminates-outside-the-funnel`; touch only their endings here.

## Closed

2026-10-10. PR 4497 merged at `4c7fb9b0fc` after a full review (verdict: merge) and a fix pass; hosted CI was green.
- **Census coincidence menu:** `validate::too_close` now goes through `own_close`, so a poisoned or contradicted margin at rest ends in the kernel-or-file defect ending.
- **`ChartRegionError::Escalated`:** its poisoned arm ends in `defect_ending(Build)`.
- **`PointInSolidError::Escalated` (accepted departure):** both arms end in the unnamed placement lever plus the reading's note. That is the same recourse its other three readers give, and moving the query point does cure a NaN point.
- **`SplitReduceError::SliverSector` is unchanged:** its mints are not all contradictions. `wall_graze` may straddle (ledger F11) and `split_sector_extent` is a decided Zero. They are filed on CLEAVE's `split-escalations-end-a-poisoned-margin-in-the-plane-lever`.
- **Filed on FLUX:** `indeterminate-display-offers-declare-on-a-poisoned-margin`.
- **Unit 2 residue:** `solid_contain`'s decided period/nappe verdicts now end in the placement lever with the note.
