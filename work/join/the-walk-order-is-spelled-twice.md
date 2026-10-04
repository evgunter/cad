---
id: the-walk-order-is-spelled-twice
kind: issue
title: The order along a section conic is spelled twice: the boolean's partner ranking reads the side of an axis plane, the split's conic_pairs a parameter walk
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

- the boolean's partner ranking (`crates/topo/src/boolean/join.rs`
  `germ_arm` and `nearer_along`, through `turned_past`): a germ's
  partners in the half-turn it runs into come first, and within one
  half-turn the one on the near side of the incumbent's axis plane
  (`bool_join_arc_ahead`, `bool_join_arc_travel`);
- the split's `conic_pairs` (`crates/topo/src/splitting/join.rs:464`)
  sorts the crossings along the walk coordinate `w = ±θ`, the conic's
  eccentric anomaly, by `sort_along` with an arc-length gap
  (`crates/topo/src/splitting/join.rs:519`).

Both orders are the order along the conic, so they agree; nothing
holds them against each other. (PR 3985 first added a third spelling,
a walk filter in the boolean's `partners`; JOIN's ranking made it
redundant on every row, and it was removed before merging.)

## Done when

One walk serves both lanes, or a row holds the two orders against each
other on the conics both reach (a tilted ellipse on a cylinder, a
circle on a sphere) so that a change to either goes red.
