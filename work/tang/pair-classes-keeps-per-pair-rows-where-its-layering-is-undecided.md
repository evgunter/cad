---
id: pair-classes-keeps-per-pair-rows-where-its-layering-is-undecided
kind: issue
title: A vertex in pairs alone whose layering leaves an edge undecided, or whose outermost cones disagree, keeps each pair's rows, silently
status: open
opened: 2026-10-08
priority: P2
cost: M
refs: [pair-classes-falls-back-to-per-pair-rows-beside-a-partner-that-reads-none]
---


Filed by the sweep of `pair-classes-falls-back-to-per-pair-rows-beside-a-partner-that-reads-none`,
which closes the fallback beside a partner that reads nothing and
leaves these.

## What

`crates/topo/src/boolean/vtxfac.rs` `pair_classes` layers a vertex's
partners where it is in pairs alone (`layered_alone`). Where
`layered_alone` returns `None`, each pair's rows stand as read
(`unwrap_or_else(alone)`), with no refusal, in three cases:

- **the outermost cones disagree**: one met and one joined
  (`base.is_some_and(|b| b != here)`);
- **an edge is left undecided**: it lies on two partners' boundaries,
  or a partner's rows miss it (`layered`'s `None`);
- **no outermost cone**: no partner is found outside every other.

The per-pair rows read each partner alone, so an edge can carry two
rows, or one that the other partner's cone would flip. Naming takes
them as given.

## Measured

An instrumented run (`pair_classes` logging each fallback, at `b7e31045`
plus the fix that filed this):

- topo, 2 645 tests: 114 vertices reach "outermost disagree", 150
  reach "edge undecided", all in
  `crates/topo/tests/union_flush_onto_edge_contact.rs`. The cases are
  `a_dangling_null_edge_inside_another_along_one_end_builds_in_every_op`,
  `two_dangling_null_edges_with_one_segment*`,
  `every_tied_strut_witness_holds_with_its_vertex_pairs_reversed`
  (disagree), and the flush-pinch, fold and crossing-at-a-corner rows
  (undecided).
- editor-core: 48 "edge undecided", in the three
  `union_flush_onto_edge_contact` naming rows of the union node.
- sweep: none.

None is checked against an oracle: those scenes build and their own
rows pass, but no germ reads these edges. The exact-oracle pair fuzz in
`sectors::cone_fuzz` (`check_pair`) counts what reaches the fallback
and its wrong rows, unasserted, and has reached none: its two partners
lie apart, so they neither disagree nor share a boundary.

## The shape to give

Measure first: read the scenes above against a germ (the
`a_vertex_read_again_classes_every_edge` tally would take them), and
count which kept rows are wrong. An edge on two partners' boundaries
is on the solid's boundary only where the two cones' sides there
agree. Outermost cones that disagree are a met and a joined cone side
by side, whose material between them a reading of both would decide.
Refuse typed for what no reading decides, as beside a partner that
reads nothing, and report any cell main built that would then refuse.
