---
id: two-spans-projections-on-one-line-are-read-four-ways
kind: issue
title: "The gap or overlap of two straight spans' projections on one line is read four ways: curve_pair's router, the census, check 9 and profile's seg"
status: closed
opened: 2026-10-18
priority: P1
cost: M
closed: 2026-10-21
branch: nurbs/seg-one-line
---

## What

Found in the review of PR 5286 (RESTREAD,
`restread/meet-apart-on-one-line`). That PR taught
`geom_brep::curve_pair::pose`'s router to read the gap between two
spans' projections on a line before any carrier reading. The same
question, where two straight spans lie along one line and whether
their projections overlap, has four implementations. Each has its own
margin, row name and reading order:

- `crates/geom-brep/src/curve_pair/pose.rs`: `overlap` (an `Overlap`,
  signed extent) and `route`'s `curve_pair_pose_gap`, read after the
  parallel screen and before the tilt and offset, charged with the
  projections' rounding.
- `crates/topo/src/census.rs`: `collinear_overlap`, through
  `projected_overlap` (`pm_census_ee_overlap`, its ends compared by
  `tri_cmp`) and `overlap_gap` (`pm_census_ee_line_gap`). The span is
  read before the gap.
- `crates/topo/src/validate.rs`: `lines_meet`'s `overlap` closure
  (`ring_outer_meet_overlap`). It reads the ends' sides of the other
  line first and escalates on them before the overlap
  (`work/restfront/check-9-lines-meet-reads-each-end-against-the-other-line-before-the-overlap.md`).
- `crates/profile/src/seg.rs`: `line_line`'s collinear arm
  (`collinear_overlap`). It is reached only after `chord_side` reads
  all four ends as zero, and escalates before that
  (`work/profile/validate-reads-in-band-carriers-before-spans-in-line-line-arc-arc.md`).

Two of the four still read the carrier before the span. That is the
drift a single logic would have prevented.

## The shape to give

One reading of "two straight spans on one line" (projections, signed
extent and gap, read before any end-against-carrier question), owned
by `geom_brep::curve_pair` and called by the other three. Check 9 is
already headed there
(`work/restfront/check-9-meeting-arms-silent-off-a-plane-and-on-ellipse-spiric-nurbs-edges.md`:
make check 9 a caller of `meet`). The census and `profile::seg` would
follow.

## Closed

The four readers now read one home, `geom_core::one_line`, each
before any end-against-carrier question:

- **`curve_pair::pose`'s router** (PR 5481, which made the home):
  `route`'s `curve_pair_pose_gap` reads `on_one_line`'s gap.
- **Check 9** (PR 5492): `lines_meet` reads the one-line gap first,
  symmetrically, and its overlap closure reads the charged home.
- **The census** (PR 5513): `collinear_overlap` reads the spans'
  projections through the home, its uncharged comparisons gated on
  their precision, and refuses past it on the precision with its own
  recourse (shorter edges).
- **`profile::seg`** (PR 5542, `nurbs/seg-one-line`): `line_line` reads
  `gaps_apart` (`on_one_line_2d`, predicate `segment_gap`) on either
  axis before any `chord_side`, as check 9 does, with `produced = 0` on
  the loop's stored vertices. Its collinear arm (`collinear_overlap`)
  reads the same home's `OnOneLine`, decided past its charge both ways,
  where it read its own uncharged `tc`/`td`/`lo`/`hi`.

PR 5504 took the home's charge at the spans' own scale
(`|a1−a0| + |b0−a0| + |b1−a0|`), which every caller reads.

The home is in `geom_core`, not `geom_brep::curve_pair` as "The shape
to give" proposed: `profile` and `topo` both read it, and `geom_core`
is the crate they share.
