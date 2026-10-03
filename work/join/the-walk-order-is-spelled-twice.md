---
id: the-walk-order-is-spelled-twice
kind: issue
title: The order along a section conic is spelled twice: the boolean's walk_passes reads a sweep angle about the centre, the split's conic_pairs a parameter walk, tied by prose
status: open
opened: 2026-10-03
---


Found in the dual review of PR 3985 (`reach/arc-from-pairing`; both
lanes, style finding), filed by that PR's fix pass rather than fixed:
the PR's scope is the chord's arc, and either reconciliation reaches
into both lanes' pairing.

## What

Two lanes pair the crossings of one section conic by their order
along it, and each spells that order its own way:

- the boolean's `walk_passes` (`crates/topo/src/boolean/join.rs:1622`)
  reads the sweep angle about the conic's centre from the near site,
  in the near germ's sense (`bool_join_walk_order`), and rejects a
  facing pair when a third site of the locus lies strictly between;
- the split's `conic_pairs` (`crates/topo/src/splitting/join.rs:464`)
  sorts the crossings along the walk coordinate `w = ±θ`, the conic's
  eccentric anomaly, by `sort_along` with an arc-length gap
  (`crates/topo/src/splitting/join.rs:519`).

The angle about the centre and the eccentric anomaly order the points
of an ellipse the same way, so the two agree; nothing but the comment
at `crates/topo/src/boolean/join.rs:1024` ("the order
`splitting::join`'s `conic_pairs` pairs by") says so.

## Done when

One walk serves both lanes, or a row holds the two orders against each
other on the conics both reach (a tilted ellipse on a cylinder, a
circle on a sphere) so that a change to either goes red.
