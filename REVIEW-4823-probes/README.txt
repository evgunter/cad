PR 4823 review probes (frozen head 79953ad30c, base 34097ca6d6).
- crates/sweep/tests/join_pierce_runs_sweep.rs, appended rtk_* tests (ignored):
  P1 rtk_offset_across_a_kiss: offset_planes_together on a corner-kiss union, every chart.
  P2 rtk_fabricated_carried_record: a carried v-v record naming two vertices SIDE apart.
  P3/P4/P5 rtk_three_faces_*: faces through v of `three i=4 j=3 k=1` notch U pinch.
- crates/sweep/examples/r1_4139_probes.rs: PR 4207 review probe (touch, dbl3), ported to BooleanInputs.
- crates/editor-core/tests/intent_s2_c_world.rs, appended rtk_product_points.
- mutants.patch: RTK_MUT=free_none|free_all|no_point_classes (env-gated, applied to a copy of head).
Same probe source was appended to base for the diffs.
- crates/topo/tests/union_flush_onto_edge_contact.rs, appended rtk_fold_without_records (plain unions, records dropped).
