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

## Also on the near face (PR 4037's review fixes)

The same keep holds where the listed certified member's halves are both
on the kill's own face: `euler_site_pcurve_rows::a_kev_mirror_writes_the_sum_of_two_periods`
(`crates/topo/tests/euler_site_pcurve_rows.rs`) merges the tip of an arc
chain hung off a cylinder wall into its far end, listing the remaining
arc with the arc to the merged end; `validate_pcurves` reports
`RowInterval` on both of that arc's halves and nothing else. The row pins
that state, so closing this item turns its first assertion red.

## Later operators no longer repair it (PR 4039)

Until PR 4039 the site mint re-derived every image of a loop it rewired,
so a later `mev` or `mef` on the far face incidentally re-minted the
stale rows. It now keeps every image the door does not create
(`pcurves::site_rows`), so the stale rows outlive later operators, and
their stale interval enters the joints those operators decide beside
them. Tier 3 still reports them at rest.

A `debug_assert!` that a kept image's interval is its edge's
(`cache.params()` against the edge's) was considered there and not
added: this row is a reachable state in which the two differ, so the
assertion would panic a debug build where the posture hands the state
to tier 3. Closing this row makes it sound.
