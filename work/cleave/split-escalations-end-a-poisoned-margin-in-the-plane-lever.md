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
- `splitting/mod` `SplitReduceError::CrossingEscalated` where the fault names no decision: `lever_recourse(SPLIT_COINCIDENCE_RECOURSE, None)`, with no note.
- `splitting/mod` `SplitReduceError::SliverSector`: "move the split plane or the geometry".
- `chord_join` `SplitJoinError::render`: `Escalated` ("where a section runs across a face is too close to call"), `OrderEscalated`, and `RingHoming(PointInLoopError::Escalated)`.
- `splitting::section` `SectionError::WindingUndecided { diag: Some(_) }`.
- `splitting::finish` `SplitFinishError::{DescribeEscalated, DescribeBendEscalated}` and the arm after them.

On a poisoned margin (`MarginDiag::is_invalid`), each ends in the split's lever with no unreadable-margin note.

**None of these can route on `is_invalid` as it stands.** In each, one invalid margin may mean either of two things the ending must tell apart:
- **A straddle.** Two sound readings disagree, and the split lever reaches it. `RefusedArm::Straddle` documents this, and it is not poison.
- **A contradiction.** Two decided readings cannot both hold on a body that passed the gate, which makes it a kernel defect.

The sites that mint an invalid margin into these arms:
- **`chord_join::agreed_section`** (`pc_parallel_gap_disagreement`, `pc_axis_plane_parallel_disagreement`) mints into `SplitJoinError::Escalated`. Two readings of one wall's reach, each sound, serve sections of different classes. That is a straddle.
- **`rules::wall_graze`** (`wall_bend_order2`, "sectors disagree") mints into `SliverSector`. Each grazed wall's verdict is read over its own face extent, and ledger F11 leaves that arm an open policy question. Sound geometry may make two walls disagree, so this is a possible straddle.
- **`rules::split_sector_extent`** mints into `SliverSector` when the face extent decides Zero. That is a decided verdict (a collapsed face) reported as poison, not a contradiction. It is already listed on `topo-mints-indeterminates-outside-the-funnel`.
- **`rules::enters_material` / `wall_graze`** mint into `SliverSector` on `Exits`/`Tangent` after rule (a) read "enters". This is a contradiction if both reads ask the same question of the same data; confirm that before ending it as one.

ENCL left `SliverSector`'s ending as it was on main in PR 4497 for this reason: one `Display` cannot tell these sites apart.

## Which rule each arm takes, and the test

- **The defect ending** (`defect_ending(Build)`), for an invalid margin that only a contradiction of decided readings, or a NaN the kernel made, can mint on a body that passed the gate. Moving the plane or the geometry does not make the kernel's arithmetic right, so the report is the recourse.
- **The lever plus the reading's note** (PR 4475's rule: lever, then `unreadable_margin_note(reading)`), for an invalid margin that an input the user can move can poison: the caller's plane origin, or the operand geometry where no gate has checked it. Moving that input is a way through, and the note asks for the report in case it was the kernel.
- **The lever alone**, for a straddle.

So `CrossingEscalated` takes the lever plus the note unless its owner shows that the conic root lane mints an invalid margin only for a contradiction. The plane and conic are the user's, and the lane runs on the plane the caller passed. `SliverVertex` already reads as the second kind: its poison is a NaN plane origin, which is the caller's.

## Repair shape

Carry how the reading stands, as `splitting::containment::Escalation` does (`Margin` / `Straddle` / `Decided`), from each mint above to the error that renders it. Then end each arm through `RefusedArm`:
- the lever on a margin that was read, or on a straddle;
- the defect ending on a decided contradiction;
- the lever plus the note on poison an input can carry.

This moves mints, so `agreed_section`, `rules:197` and `rules:319` wait on `topo-mints-indeterminates-outside-the-funnel` where that row owns the sites.
