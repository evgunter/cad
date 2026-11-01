---
id: the-other-orbit-readings-refuse-a-tie-the-walk-now-reads
kind: issue
title: strut_anchor, held_cut/tied_held, on_arc and holds_whole still refuse a tie of two germs along one edge's two sides that walk_order and leads now read
status: review
opened: 2026-10-27
priority: P1
cost: M
refs: [a-box-corner-at-a-rings-tip-reads-two-crossing-germs-along-one-direction]
branch: cleave/orbit-readings-read-the-beside-tie
pr: 6184
---

Disclosed by PR 5812 (`cleave/flush-partners-at-a-slit-build`) and its
claims review.

## What

PR 5812 reads one more tie of a vertex orbit: two germs along one
direction in one sector entry, each along one of the other solid's
coincident edges there (a slit's or ring's), ordered by the side of
the line each edge's flanker flush with the entry lies on
(`boolean/insert.rs` `beside_order`, through `walks_before_beside`).
`walk_order` and `leads` read it, and so does the strut facing
(`walk_faces_first`). The other readings of the same order still read
the tie bare (`walks_before`, `insert.rs:1528`) and refuse or take
their own tie rule there:

- `strut_anchor` (`:2077`): "two dangling null edges in one corner
  start along one direction";
- `held_cut` and `tied_held` (`:1432`, `:1497`): a cut along an end
  germ's own direction;
- `on_arc` (`:1554`) and `holds_whole` (`:1099`): an arc's end at a
  tie.

None is reached by a known witness with such a pair: the ringed-plate
and keel probes of PRs 5545 and 5812 build or refuse elsewhere. A pose
with two plans' struts at one shared vertex, each along one of a
ring's two edges, would reach them.

## Owed

Read every orbit position through `walks_before_beside`, so one
reading orders a vertex's cuts everywhere, or show a site cannot meet
the pair, with a witness for each that can.

