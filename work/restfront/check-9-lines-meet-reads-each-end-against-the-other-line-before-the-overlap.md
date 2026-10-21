---
id: check-9-lines-meet-reads-each-end-against-the-other-line-before-the-overlap
kind: issue
title: "Check 9's lines_meet escalates on two segments on one line far apart: it reads each end against the other's line before their overlap"
status: closed
opened: 2026-10-18
closed: 2026-10-20
priority: P3
cost: E
---

## What

Found by the class sweep of RESTREAD's
`census-meet-reads-two-segments-on-one-line-two-metres-apart-as-coincident-lines`
(branch `restread/meet-apart-on-one-line`), which fixed the same shape
in `geom_brep::curve_pair::pose`'s router: two segments on one line
far apart along it, an end of one in the band's gap of the other's
line, escalated though the segments are definitely apart.

`crates/topo/src/validate.rs` `lines_meet` (check 9's arm 5, under
`segments_meet`) reads the four ends' sides of the other's line
(`ring_outer_meet_side`) first. It answers `Apart` only where both of
one segment's ends are definitely on one side, and returns
`EdgePair::Unsure` on any in-band side before it reaches the collinear
overlap (`ring_outer_meet_overlap`). Two segments on one line, two
metres apart, with an end `1.06ε` off the other's line, are Unsure.
Found by reading; not measured on a body.

## The shape to give

Read the overlap of the two spans' projections first: projections
apart by a gap past the band hold no meeting, whatever the sides read
(a meeting's two points project within `ε` of each other). The
alternative route is the one
`check-9-meeting-arms-silent-off-a-plane-and-on-ellipse-spiric-nurbs-edges`
names: make check 9 a caller of `geom_brep::curve_pair::meet`, whose
router reads that gap since this branch.

## Resolution

`lines_meet` reads the gap between the two spans' projections on `a`'s
axis first, through `geom_core::one_line::on_one_line`, on its own row
`ring_outer_meet_gap`, and answers `Apart` past the band before any
side is read. Its points are the body's stored vertex positions
(`edge_endpoints`), so the reading is charged for the projections only
(`produced = 0`). Rows in `topo::validate::tests`:
`two_segments_on_one_line_far_apart_along_it_are_apart` (this item's
pose; `Unsure` on main) and
`an_end_touching_the_middle_of_a_square_segment_is_never_read_apart_by_the_gap`
(red with the gap's charge dropped). PR 2 of
`work/cpair/two-spans-projections-on-one-line-are-read-four-ways`.
