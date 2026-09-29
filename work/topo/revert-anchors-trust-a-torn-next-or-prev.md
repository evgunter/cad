---
id: revert-anchors-trust-a-torn-next-or-prev
kind: issue
title: revert re-anchors every vertex at mate(emanating), whose new start it reads through next, and every loop at prev(first): a live-but-foreign next or prev carries off anchors through Ok
status: open
opened: 2026-09-29
refs: [kill-ops-anchor-emanating-on-an-unproven-next-mate-step]
priority: P3
cost: E
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

Derived by reading, not measured: a probe in the style of
`review_d18::kill_anchors_on_torn_bodies` over `revert` would size
it.

## The shape to give

Either `revert`'s precondition pass proves each new anchor (the new
`emanating` starts at its vertex, the new `first` lies in its loop)
and refuses `RevertError::Corrupt` naming the link, or its docs say
that a live-but-wrong link is carried and why.

