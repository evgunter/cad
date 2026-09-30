---
id: revert-anchors-trust-a-torn-next-or-prev
kind: issue
title: revert re-anchors every vertex at mate(emanating), whose new start it reads through next, and every loop at prev(first): a live-but-foreign next or prev carries off anchors through Ok
status: review
opened: 2026-09-29
refs: [kill-ops-anchor-emanating-on-an-unproven-next-mate-step]
priority: P3
cost: E
pr: 3546
branch: topo/revert-anchors
---

## What

Found by the receipt of
`kill-ops-anchor-emanating-on-an-unproven-next-mate-step`, whose
class is a write plan that takes `next(mate(x))`, `next(x)` or
`prev(x)` and writes it as an anchor trusting its start vertex or its
loop.

`Body::revert` (`crates/topo/src/revert.rs`) builds the reversed body
from three maps it reads off the source:

- every half-edge's new start is `half_edge_end(he)`
  (`crates/topo/src/body.rs`), which is `start(next(he))`;
- every vertex's new `emanating` is `mate(emanating)`, so its new
  start is `start(next(mate(emanating)))`, which the map never
  compares with the vertex;
- every loop's new `first` is `prev(first)`, resolved but never read
  for its `parent_loop`.

A live-but-foreign `next` at `mate(emanating)` gives the reversed
vertex an `emanating` that starts elsewhere
(`EmanatingStartMismatch`), and a live-but-foreign `prev(first)`
anchors the reversed loop in another loop, both through `Ok`. The
door's contract (`RevertError::Corrupt`) refuses only a link that
does not resolve, so this is the contract as written, not a
departure from it; the input is already tier-1-invalid. What the row
asks is whether the reversal should carry a live-but-wrong tear into
new anchor faults, or refuse it typed as the Euler plans now do.

## Measured

The kill-anchor review of PR 3483 planted every single tear on
`declined_cube` (24 half-edges, so 576 `next` tears and 576 `prev`
tears, one half-edge's link set to each half-edge in turn) and ran
`revert`. With debug assertions off, 168 of the 576 `next` tears give
`Ok` with an `EmanatingStartMismatch` the source did not have, and
120 of the 576 `prev` tears give `Ok` with a loop whose `first` lies
in another loop. In the dev profile `revert` panics at its tier-1
postcondition instead (`revert.rs`, the `debug_assert` after the
build). The probe was the review's own and is not committed; a row in
the style of `review_d18::kill_anchors_on_torn_bodies` would keep it.

## The shape to give

Either `revert`'s precondition pass proves each new anchor (the new
`emanating` starts at its vertex, the new `first` lies in its loop)
and refuses `RevertError::Corrupt` naming the link, or its docs say
that a live-but-wrong link is carried and why.

