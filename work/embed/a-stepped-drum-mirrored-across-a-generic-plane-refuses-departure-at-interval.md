---
id: a-stepped-drum-mirrored-across-a-generic-plane-refuses-departure-at-interval
kind: issue
title: A stepped drum mirrored across a generic plane refuses Departure across a rim at Interval
status: open
opened: 2026-11-23
---


## What is seen

A stepped drum on its axis built off the dyadic grid — the profile
`(0, 0.13), (0.7, 0.13), (0.7, 0.13 + 1/3), (1/3, 0.13 + 1/3),
(1/3, 1.13), (0, 1.13)` revolved a whole turn about `y`
(`stepped_drum_off_the_grid`,
`crates/sweep/tests/self_intersection_proves_injectivity.rs`) — finished,
then mirrored by the finished body's own door across the plane through
`(0.1, −0.2, 0.3)` with normal `(1, 2, 3)/√14`, is refused by
`topo::clearance::embedded` at `Interval`:

`Uncertified { a: FaceKey(3v1), b: FaceKey(7v1), shared: Edge(EdgeKey(2v1)), arm: Departure }`

The two faces are both cylinders: the two faces of one wall, across
their seam. The receipt is `candidates: 28, discharged: 17013,
refused: 1, splits: 18256, outside: 5710, certified: 1086,
abandoned: 1270` — the walk runs out of depth along the seam. The same
body at `f64` holds (855 discharged), and at `Interval` it holds as
built (47), turned 0.7 rad about `(1, 2, 3)` (406) and a hundred units
out (57). The stepped ring built off the grid the same way holds in all
four poses at both scalars.

## Where it is from

It reads so on `embed/wave3-land` at `8d34076a9e` before the embedding
question reads tier 3's facts, with the same receipt, and after
(`embed/verdict-read`), so it is not the facts' reading. Not traced
further: a seam between two faces of one cylinder is tangent, the
mirrored `Interval` wall's frame is an enclosure with width in every
channel, and the likely cause is the seam's separator reading those
enclosures; that is a guess.

## Pinned

`bodies_built_off_the_dyadic_grid_hold_in_every_pose_but_one` pins the
exit as a finding and goes red the day the body holds.
