---
id: main-red-coincident-wedges-overlap-refuses-flush-copies
kind: issue
title: main red in the CI profile since PR 5529: CoincidentWedgesOverlap fires on flush and abutting copies (7 rows, topo + editor-core)
status: open
opened: 2026-10-26
priority: P0
cost: M
---


## What

Since PR 5529 (merged as `3005261684`, "tier 3′ reads coincident edges' wedges on one line"), seven rows of the **CI profile** fail on main. Every one is the kernel's new `CoincidentWedgesOverlap` ("two edges lie along one line … the line holds material twice") firing on flush or abutting copies:

- `topo` `coincident_line_wedges`: `bricks_sharing_a_quadrant_refuse_on_their_lines`, `a_prism_leaning_into_a_brick_reads_as_passing_into_it`;
- editor-core:
  - `aligned_overlap_quiets::a_flush_sided_slab_is_quiet_under_its_gap`;
  - `asm_r2b_assembly::copies_filling_one_volume_are_one_interference`;
  - `mate6r2_probes::p6_carried_penetration_is_loud`;
  - `p2_gauge_offsets_and_spaces`: `a_document_of_copies_in_another_space_has_an_empty_world` and `the_gate_checks_each_new_space_whatever_the_world_holds`.

INTENT's D′3 lane (PR 5636) measured these on main at `4c341b9225^2`, and on its own merged head. A viewer row, `mate_door_and_badge::the_at_rest_badge_checks_a_space_of_its_own`, fails the same way on that head; it is being confirmed on main.

## Why it matters

These are fast-set rows, so every PR's `gate ok` that merges main inherits the red. The likely cause is 5529's lane landing beside the census's same-side interference (PR 5605's planar face pair, PR 5637's mated-seat recording) on a base that predated one of them. That is unverified. The nearest filed row, `a-confirmed-face-pair-swallows-the-crossing-lanes-side-verdict`, does not cover it.

## Owed

Decide whether a flush or abutting pair (a face pair at rest) should reach tier 3′'s line-wedge read at all. Then fix the reading, or re-pin the rows if 5529's verdict is right and the rows assumed the old one. Discipline §3 applies: say what moved and why. — (INTENT orchestrator), filed from D′3's measurement
