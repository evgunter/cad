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

- topo, 2 645 tests: 102 vertices reach "outermost disagree" and 150
  "edge undecided", all in
  `crates/topo/tests/union_flush_onto_edge_contact.rs`:
  - outermost disagree:
    `a_dangling_null_edge_inside_another_along_one_end_builds_in_every_op`
    (12), `two_dangling_null_edges_with_one_segment_build_in_every_op`
    (12), `two_dangling_null_edges_with_one_segment_ending_in_the_cubes_face_build_in_every_op`
    (27), `every_tied_strut_witness_holds_with_its_vertex_pairs_reversed`
    (51);
  - edge undecided: `two_pinches_crossing_on_one_line_refuse_their_union_typed`
    (48), `every_order_folds_a_flush_partner_onto_the_edge_contact` (16),
    `carried_records_certify_every_order_of_the_fold` (16),
    `joined_folds_certify_and_build_one_body_in_every_order` (16),
    `four_crossings_at_one_corner_build_in_every_op` (16),
    `three_crossings_at_one_corner_build_in_every_op` (14),
    `a_vertex_crossing_both_sides_of_a_pinch_builds_in_every_op` (12),
    `a_four_row_remap_group_certifies_a_subtract_of_two_pinches` (8),
    `three_pieces_two_of_whose_cuts_tie_build_in_every_op` (2),
    `every_tied_strut_witness_holds_with_its_vertex_pairs_reversed` (2).
- editor-core: 48 "edge undecided", 16 in each of three
  `union_flush_onto_edge_contact` naming rows:
  `the_union_node_publishes_one_table_in_every_member_order`,
  `the_union_node_folds_a_flush_partner_onto_the_edge_contact` and
  `chained_pair_unions_fold_a_flush_partner_onto_the_edge_contact`.
- sweep: none.
- no outermost cone: no witness is known. No suite row reaches it,
  nor does `sectors::cone_fuzz`'s pair oracle.

`layered`'s `p.read.as_ref()?` arm, which also leaves an edge
undecided, is not reached from `pair_classes`: it refuses typed beside
a partner that reads nothing before it layers. `touch_classes` refuses
beside one too.

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
