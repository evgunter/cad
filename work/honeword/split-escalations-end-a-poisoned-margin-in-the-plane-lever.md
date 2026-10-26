---
id: split-escalations-end-a-poisoned-margin-in-the-plane-lever
kind: issue
title: topo: split and section escalations end a poisoned margin in the plane lever, and one of them carries a straddle as poison
status: open
opened: 2026-10-10
priority: P3
cost: M
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

Carry how the reading stands, as `splitting::containment::Escalation` does (`Margin` / `Straddle`; a decided contradiction is no escalation and is typed as its own finding, as `contfp`'s `ContainError::ArcEndClearOfCorners` is), from each mint above to the error that renders it. Then end each arm through `RefusedArm`:
- the lever on a margin that was read, or on a straddle;
- the defect ending on a decided contradiction;
- the lever plus the note on poison an input can carry.

This moves mints, so `agreed_section`, `rules:197` and `rules:319` wait on `topo-mints-indeterminates-outside-the-funnel` where that row owns the sites.

## Built in part (branch `cleave/split-rules-mint-no-invalid`)

`SplitReduceError::SliverSector` is done. It carries
`SectorReading::{Escalated, GrazeAgainstNeighbours,
SmoothEdgeDisagrees}` in place of an `Indeterminate`, and ends through
`LeverOnly` at `Reading::Build`:

- the lever alone on a read margin, or on either of the two decided
  disagreements, which carry their margins;
- the lever plus `UNREADABLE_MARGIN_NOTE` on a poisoned funnel margin.

No `SliverSector` arm is a decided contradiction any more, so none
takes the defect ending.

Since PR 4745, `SectionError::WindingUndecided`, `SplitFinishError`
(`DescribeEscalated`, `DescribeBendEscalated`, `SectionWindingUndecided`)
and `SplitJoinError::OrderEscalated`'s frame decisions
(`split_join_frame_axis`, `_arm`) end the same way, with the section's,
the split's and the join's lever. `RingHoming` ends in the walk
decision's lever alone. `SplitJoinError::Escalated` and the order
decisions `split_join_order_u`/`_v` take `Indeterminate::ending`, the
ending geom-brep gives the same decision. Most of `Escalated`'s mints
are sized (a pose's gap, an apex's offset, a tangent chord's advance),
and so is a metred coordinate. Not all of them are, though (below).

Still open on this row:
- `CrossingEscalated` (no note on poison).
- At the sites above, an invalid margin minted from a contradiction
  of decided readings still takes the lever and the note rather than
  the defect ending until the mint carries its `Escalation`.
- **A straddle reads as a subdividable enclosure.**
  `pc_axis_plane_parallel_straddle` is a `k_stats::straddle`, but
  `SplitJoinError::Escalated` ends it through `Indeterminate::ending`,
  which puts "subdivide the parameter box for a tighter enclosure" in
  front of the lever. `RefusedArm::Straddle` and this row say a
  straddle ends in the lever alone. The escalation does not carry its
  `Escalation`, so the ending cannot tell.
- **Two unsized mints offer the tolerance.** `chord_arc_leave_germ` and
  `chord_arc_leave_section` (`chord_join.rs`, in the arc-leave reading)
  raise `SplitJoinError::Escalated` on a levered cosine or sine, which
  is no size the user chose. They take `Indeterminate::ending` with
  the sized mints, so they offer a tolerance arm. They want the lever
  alone, which needs `Escalated` to carry which kind of decision
  minted it.

## `agreed_section` no longer mints poison (branch `cleave/split-straddles-carry-reading`)

`agreed_section` now carries a straddle: the enclosure of the two
readings, through `k_stats::straddle`, under
`pc_axis_plane_parallel_straddle`. The
same holds for the walk's conic straddles (`*_conic_straddle`,
`bool_contact_arc_straddle`, `pm_census_ef_cross_arc_straddle`,
`bool_sphere_region_arc_straddle`).

Two readers still drop the straddle's type and end on the margin's
kind. With an enclosure in place of `INVALID`, both now give the lever
alone, which is what `RefusedArm::Straddle` gives:
- the Boolean germ frame (`join::frame_refusal` →
  `Coincide::Section`), whose unreadable-margin note is gone;
- the census crossing cut (`CrossRefusal::Escalated` →
  `ValidationError::CensusEscalated` → `too_close`), whose defect
  ending is now the coincidence menu.

The ending is still decided by the margin there, not by a typed
`Escalation`. Carrying the type to those two errors is this row's
repair.

## More sites of the same shape (ENCL, from the sweep of `switch-profile-and-editor-core-onto-indeterminate-lever-ending`)

These compose the lever by hand after an `Indeterminate`'s payload, so a poisoned margin also ends in the lever with no note:
- `splitting/mod.rs` `SplitReduceError::SectionGermsEscalated`, both readings: `"… ({diag.payload()}). Recourse: {SPLIT_COINCIDENCE_RECOURSE}"`.
- `chord_join.rs` `SplitJoinError::Section { source: SectionError::Carrier(EllipseInvalid::Escalated(diag)) }`: `"… is undecided for the section through a curved face: {payload}. Recourse: {JOIN_RECOURSE}"`.

This one is rendering-neutral:
- `boolean/refusal_routes.rs` `Ending::recourse`'s `Self::Lever(lever)` arm composes `LeverOnly { lever }.recourse(RefusedArm::Undecided(diag), Reading::Build)`. That is `diag.lever_ending(lever)` spelled through the table.

## Slate triage (2026-10-26, CLEAVE)

- Banded P3: endings that cannot tell a straddle from poison: library usability: the refusal names the wrong cause, recourse or payload, P3 by `work/README.md` Priority.
- Priced `M`, the effort the body describes.
- Moved to HONEWORD (`work/honeword/`) by `git mv`, id and body unchanged: whose charter is split and boolean refusal text, payloads and endings. CLEAVE measured 270.5 budget points against 30 and split on its priority seam (Ev, in chat, 2026-10-26).
