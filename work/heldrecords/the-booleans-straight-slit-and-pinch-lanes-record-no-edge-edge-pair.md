---
id: the-booleans-straight-slit-and-pinch-lanes-record-no-edge-edge-pair
kind: issue
title: The Boolean's straight slit and pinch lanes mint two coincident line edges and record no edge-edge pair; only the census's shared-bounds exemption passes them
status: open
opened: 2026-11-23
priority: P1
cost: M
---

Found by `heldrecords/split-on` (plan step 3), measuring the census with
the shared-bounds exemption off.

## What

`crates/topo/src/census.rs` `ee_collinear_lane` reads two line edges
whose ends share both point keys as one segment by structure
(`(low, high) == (Bound::Shared, Bound::Shared)`), and so owes them no
record. Plan step 3 retires that exemption once every door records the
coincident edges it mints (Ev on PR 5967: "coincident edges an op mints,
straight or curved, are recorded"). The split now records its slits'
and its straight pinches' pairs, and the Boolean records an arc's two
coincident edges (`boolean::slit::part_riders`). The Boolean's straight
lanes still record nothing:

- **The slit parting of a line** (`crates/topo/src/boolean/slit.rs`
  `part_riders`, the `backing` read is taken for arcs only): a line lying
  in a face parts into two coincident lines and no edge–edge record.
  Witnesses, refusing `UnrecordedContact { EdgeEdgeOverlap }` with the
  exemption off:
  `crates/topo/tests/a_face_inside_an_edges_wedge_refuses_at_tier_3_prime.rs`
  `the_booleans_slit_keel_passes_tier_3_prime` and
  `a_glued_tip_subtracted_with_its_records_passes_tier_3_prime`;
  `crates/topo/tests/a_figure_eight_hole_cut_through_its_pinch.rs`
  (the subtract `x + y = 0`);
  `crates/topo/tests/a_vertex_read_by_two_sector_passes.rs`
  `an_edge_through_two_blocks_glued_faces_builds_at_rest`; and the
  bore's slit forms, `crates/sweep/tests/a_keel_on_a_bores_ruling.rs`
  `the_slit_form_holds_its_closed_form` and
  `crates/sweep/tests/a_slit_tip_on_a_curved_face_reads_its_full_turn.rs`
  `the_block_carries_the_keel_as_a_slit_of_the_bore`.
- **Splits of a Boolean's unrecorded pair**: an operand the Boolean
  built with two coincident line edges and no record, which a later cut
  crosses, leaves each side two coincident halves the split cannot
  record (it carries what its operand holds, and mints only the pairs
  over an operand edge in its plane):
  `crates/editor-core/tests/split_pinch_keeps.rs`
  `a_diamond_moat_reaching_the_block_wall_holds_its_touches_on_the_pocket_wall_as_corners`,
  `a_moat_reaching_the_block_walls_at_its_corners_quarters_both_sections`
  and `a_split_along_a_diamond_pocket_floor_touching_the_block_walls_builds`,
  which refuse on their own records as well (measured 2026-11-23).
- **The union pinch along a line** (two solids touching along an edge,
  each end collapsed to one vertex): `crates/sweep/tests/common/touch_union.rs`
  through `review_4704_flush_round_oracle::a_same_side_tangent_touch_answers_every_op`,
  `verbs_cylcyl_r1_review_probes::externally_tangent_walls_touch_at_every_op`
  and `wedge_end_doors::two_discs_kissing_externally_touch_at_every_op`.

The exemption's own pins go red with it, by design:
`census::tests::a_tip_copy_on_its_own_bit_equal_point_is_an_unrecorded_contact`,
`crates/topo/tests/m3_pr6_tier3prime.rs` (`tangent_edge_promoted`,
`interval::tier3prime_interval`) and
`crates/topo/tests/union_flush_onto_edge_contact.rs`
`a_pinch_without_its_edge_edge_row_reads_its_shared_ends`. Their messages
cite the shared-ends reading as ratified on PR 4793 ("the record is the
lint's evidence, not the census's"), where `crates/topo/README.md`'s
tier 3′ text already says the census "infers no record from shared
bounds": the two have to be reconciled with Ev before the exemption
comes out.

Tests that hand the census no records, where the body holds its own,
also go red, and are a test-side change once the doors record:
`crates/topo/tests/m3_pr3_split.rs` (`pseudomanifold_door`, three rows),
and hand-built fixtures with no records
(`crates/topo/tests/a_doubled_line_does_not_revert.rs`
`the_slit_body_reverted_unpaired_refuses_its_wedges_at_tier_3_prime`;
`crates/topo/tests/a_slit_tip_at_a_cone_apex_reads_its_full_turn.rs`, the
cone's ruling slit and the two splits that carry it unrecorded).

## Direction

The parting records a line's two edges as it records an arc's, citing
the records that put the line or its ends on the face; the union pinch
records its coincident edges citing the vertex identities that collapsed
each end. Then the exemption comes out in one PR with the tests above
reading their bodies' records.
