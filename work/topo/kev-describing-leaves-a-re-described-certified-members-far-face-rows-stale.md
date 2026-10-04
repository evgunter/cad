---
id: kev-describing-leaves-a-re-described-certified-members-far-face-rows-stale
kind: issue
title: kev_describing leaves a listed certified member's rows on its far face spanning the interval its ends moved from
status: open
opened: 2026-10-04
priority: P3
---


Found by the review of PR 4010
(`kev-describing-a-null-member-leaves-its-face-missing-its-rows`),
its probe P3.

`Body::kev_describing` (`crates/topo/src/euler_kill.rs`, `fn
kev_describing`, :944) re-describes every member it lists under the
curve the kill installs. A listed NULL member's first description
re-mints the faces that member's halves are on, as the kill leaves
them (`Body::null_description_rows`, `crates/topo/src/attach.rs`
:1033, fed `kev_loops_after`, `euler_kill.rs` :1199). A listed
CERTIFIED member keeps its rows: that is the `Completes` posture as
declared (`pcurves::staleness_posture`'s `Completes` variant,
`crates/topo/src/pcurves.rs` :4286, "keeps the rows it finds … and
rests on the same tier-3 pass for what its write stales"). So where
the kill moves a certified member's end, the member's half on a face
no listed null member's halves are on keeps a row spanning the old
interval, and the tier-3 pcurve pass reports it.

The state predates PR 4010: the re-mint that PR adds covers only the
faces a null member's halves are on.

## Recipe (probe P3)

- A sheet with its side split at mid-height.
- A null strut (`mev_null`) at the split vertex.
- `kev_describing` killing the lower side segment toward the split:
  a `General` kill across two loops (`[2v1, 1v1]`), listing both
  merged members, the null strut and the upper side segment, each
  with the line between its merged ends.

## Findings

- **At PR 4010's head** (`c16baa855d`): `validate_pcurves` reports
  `RowInterval { half_edge: 7v1 }` and `LoopNotClosed { face: 1v1 }`,
  from the listed certified member's half on the far face 1v1. The
  null's face, 2v1, leaves clean.
- **At its base** (`0f8918c4c`): the same two, plus `RowInterval
  8v1` and `LoopNotClosed 2v1` on the null's face, and two
  `MissingCache` (11v1, 12v1) for the null's halves.

## Closing it

Either the kill re-mints (or re-derives the rows of) every face a
listed member's halves are on whose carrier it moved, as the null
member's faces already are, or the posture's declaration says in
words that a moved certified member's far-face rows are tier 3's to
report. The first is the one D1's atomic contract points at.
