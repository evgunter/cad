---
id: main-red-coincident-wedges-overlap-refuses-flush-copies
kind: issue
title: main red in the CI profile since PR 5529: CoincidentWedgesOverlap fires on flush and abutting copies (7 rows, topo + editor-core)
status: closed
opened: 2026-10-26
priority: P0
cost: M
closed: 2026-10-26
pr: 5804
branch: restread/flush-wedges-main-red
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

## Two more rows (INTENT, 2026-10-26 12:50)

The CI on PR 5748's main merge (`ee35bec40e`) shows the same refusal on two more rows:
- viewer `mate_door_and_badge::the_at_rest_badge_checks_a_space_of_its_own` ("placed, the two posts interpenetrate in the world: … `CoincidentWedgesOverlap`");
- python `test_world_pins::test_the_gate_checks_each_new_space_by_itself` (`AssemblyError: 12 finding(s)`).

That is nine rows in all. Every PR merging main inherits them: #5748 is held on exactly this set.

## Closed

ATREST's PR 5779 (`atrest/fix-ssi-vs-line-wedges`, merged 13:48Z) fixed the census side of this before the row was picked up. The cause it measured was PR 5745 (`SameSideInterference`, merged 11:34Z beside 5529 at 11:24Z), not 5605 or 5637. An aligned face pair *is* the doubled material, so the wedge read in the closure of its overlap reported the same overlap a second time, and that report refused. Pass 5 now skips the wedge read where an aligned confirmed pair holds the span's middle (`Declared::overlap_holds`). A cross-solid `CoincidentWedgesOverlap` elsewhere reads as the two solids' interference. Opposed and recorded pairs still reach the read, so the pinwheel still refuses. The two topo rows were renamed when they were restated (`bricks_sharing_a_quadrant_are_one_interference`, `a_prism_leaning_into_a_brick_refuses_the_axis_beside_its_caps`). The python row was renamed to `test_the_gate_checks_each_assemble_by_itself`. PR 5748's red merge `ee35bec40e` took main at 12:01Z, before 5779.

Picking the row up found a second break on main: PR 5704 added `BoundaryCrossesItself{,Undecided}` and editor-core's exhaustive `finding_solids` did not place them, so editor-core did not compile (the push CI's change filter selected only topo for that merge). ATREST's `863030db6b` placed them on main as `(s, s)`, as census `Named::of` reads them. With both fixes on main, all nine rows pass (the closing PR's body has the runs).
