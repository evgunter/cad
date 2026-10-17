---
id: two-bisector-node-rows-exceed-the-fast-set-time-bar
kind: issue
title: Two bisector_nodes rows exceed the fast set's 1 s bar in CI
status: open
opened: 2026-10-17
---


Found on PCERT's PR 5225 (`pcert/azimuth-placement`), CI run
38996794002. The test job's fast-set time check (an existing fast-set
test that takes over 1 s fails it) flags two rows in
`crates/sweep/tests/bisector_nodes.rs`, which came with TANG's U2p
(`21b84729bc`, 2026-10-16):

- `bisector_nodes::the_near_node_poses_at_the_witness_band`: 7.276 s in CI;
- `bisector_nodes::an_oblique_seam_near_a_node_builds`: 1.620 s in CI.

They are main's, not PR 5225's. Release builds of main at `52b4501d12`
and of PR 5225's head time them alike: 2.42 s against 2.45 s, and
0.74 s against 0.80 s. CI builds tests at opt-level 1, which is about
three times slower.

The check's own recourse: make each row faster, or add
`binary_id(sweep::all) & test(=bisector_nodes::<name>)` to the slow set
in `.config/nextest.toml`. Three more rows in the file run over 2 s in
release (`a_rod_against_a_bore_builds_every_operation`,
`an_edge_near_a_node_crosses_through_the_bisector_planes`,
`slanted_bores_build_every_operation_one_difference_pinched_to_itself`),
and the check does not flag them, so they are presumably already in the
slow set. The two above probably belong beside them.
