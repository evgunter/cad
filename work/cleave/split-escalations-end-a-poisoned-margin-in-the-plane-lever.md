---
id: split-escalations-end-a-poisoned-margin-in-the-plane-lever
kind: issue
title: topo: split and section escalations end a poisoned margin in the plane lever, and one of them carries a straddle as poison
status: open
opened: 2026-10-10
---


(Filed by ENCL from the sweep of `topo-poisoned-escalations-offer-unfollowable-endings`. Pre-existing.)

## What

These split-side escalations print a margin payload and then a constant lever, whatever the margin:
- `chord_join` `SplitJoinError::render`: `Escalated` ("where a section runs across a face is too close to call"), `OrderEscalated`, and `RingHoming(PointInLoopError::Escalated)`.
- `splitting::section` `SectionError::WindingUndecided { diag: Some(_) }`.
- `splitting::finish` `SplitFinishError::{DescribeEscalated, DescribeBendEscalated}` and the arm after them.

On a poisoned margin (`MarginDiag::is_invalid`), each ends in the split's lever with no unreadable-margin note. That differs from the rule PR 4475 gave a poisoned margin (lever plus `geom_brep::recourse::unreadable_margin_note(reading)`), and from `SplitReduceError::SliverSector`, which ends a poisoned margin in the defect ending since the ENCL PR for that row.

`SplitJoinError::Escalated` cannot route on `is_invalid` as it stands. `chord_join::agreed_section` mints `invalid_margin::invalid` (`pc_parallel_gap_disagreement`, `pc_axis_plane_parallel_disagreement`) for a **straddle**: two readings of one wall's reach, each sound, that serve sections of different classes. A straddle is no poison (`RefusedArm::Straddle`'s doc), and the split lever is a recourse it can follow. Routing on the margin would end it as a defect.

## Repair shape

Carry how the reading stands, as `splitting::containment::Escalation` does (`Margin` / `Straddle` / `Decided`), from `agreed_section` through `geom_brep::SectionError::Escalated` (or a topo-side wrapper) to `SplitJoinError::Escalated`. Then end each of the arms above through `RefusedArm`: the lever on a margin that was read, and the poison rule on one that was not. This moves `agreed_section`'s mint, so it waits on `topo-mints-indeterminates-outside-the-funnel` where that row owns the site.
