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

Route each to the program that owns its ground (`python3 scripts/work.py territory --files -`), and correct the two open rows' stated states. This row closes once each failure has an owner's row.
