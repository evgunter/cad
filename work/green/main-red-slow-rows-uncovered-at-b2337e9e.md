---
id: main-red-slow-rows-uncovered-at-b2337e9e
kind: issue
title: Four slow-set rows red on main at b2337e9e that no open row covers, and two open main-red rows that cite a state the tree no longer has
status: open
opened: 2026-11-03
priority: P1
cost: E
---


Found by the FULL review of ONECOPY's PR 6463. Each failure below is identical on main tip `b2337e9e` and on that PR's head, so none of them comes from the PR. Filed by the ONECOPY orchestrator.

## Red, and covered by no row
- `editor-core` `docm2_part_interval::a7_…`: red at ε/10, ε/16 and ε/32. `work/topo/a-cap-normal-reads-the-height-s-width` still says ε/32 is green.
- `editor-core` `m10_sym_profile_interval::…pinned_per_eps_row`: the slab now fails (2827 forms against 2807). The open row `work/green/main-red-two-slow-digest-rows-at-8ae2b9b` says the plate is what fails.
- `node_standing::every_node_result_reader…` (`viewer/session.rs`).
- `coincidence_door::two_copies_on_one_axis…`, at 1e-12 only.

## Partly covered
- `certified_endpoint_census::every_production_endpoint_read_is_counted_and_dispositioned`. `work/splice/main-red-certified-endpoint-census-drifted-in-geom-brep-after-6171` names 4 files, but today's drift is wider: carrier_net, ruling, winds, residual, ssi/boundary, clearance, net, and others.

Filed on GREEN, which carries main-red rows (work/issues/ is retired). Route each to the program that owns its ground (`python3 scripts/work.py territory --files -`), and correct the two open rows' stated states. This row closes once each failure has an owner's row.

## Added (2026-11-03, the PR 6454 fix pass, failing identically at its merge base)

- `editor-core` `m4_pr8_k_probe`'s corpus dump: 116 `<unnamed>` decisions, and the key-split pin.
- `editor-core` `node_standing::every_node_result_reader_goes_through_usable_or_is_listed`: `viewer/src/session.rs` is in READERS with a count of 3 that the scan no longer finds. Same row as above, now with its cause.
- topo `boolean::boxes::tests::every_door_that_reads_a_box_is_inventoried`: `ruled_region.rs`, `shell.rs` and `splitting/containment.rs` read a box with no entry. Covered by BOXES' own main-red row.
- `sweep` `ray_wall_margin_twins::the_wall_arms_margins_scale_linearly_with_the_model`: its pinned red now PASSES on main ("pinned to stop … and it passed"). Its owners (CPAIR or CHARTTAIL) should retire or re-pin it. Reported by the PR 6463 lane.
- `sweep` `bisector_nodes::the_near_node_poses_at_the_witness_band`: fails identically on main at c02eb40cfe with `QuadratureUnsupported`. Reported by the PR 6463 lane; no row covers it.
