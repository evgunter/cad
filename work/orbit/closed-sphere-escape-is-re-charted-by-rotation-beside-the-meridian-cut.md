---
id: closed-sphere-escape-is-re-charted-by-rotation-beside-the-meridian-cut
kind: issue
title: A closed sphere group's escape is re-charted by rotating the group while a trimmed face's is cut along its meridian: two answers to one escape
status: open
opened: 2026-10-05
priority: P1
cost: H
refs: [4044]
---


Found by the sweep of `reach/trimmed-sphere-escape`.

## What

`boolean::ops::sphere_extent_scan`'s plane arm answers one escape two
ways. A CLOSED sphere group is rotated about its own centre so its seam
meridians cross every escape plane (`apply_recuts`, `SphereRecut`),
which only works when all of the group's escape planes are parallel
(else `FallbackExtentUnsupported`, "one sphere group escapes through
NON-PARALLEL plane faces", with its `EscapeParallel` and `RecutAlign`
questions). A TRIMMED group's face is cut along its own chart's
meridian through each escaping circle (`apply_cut_ins`, `SphereCutIn`),
one cut per circle, any number of planes. An escape circle of a closed
ball lies inside one half-band too, so the cut serves both.

## Measured (branch `reach/trimmed-sphere-escape`)

Asking the section certificate of closed groups as well and cutting
every R-loop (the plane arm's `held` read for every group): topo and
sweep run 4392 rows, 21 move.

- `m5_s13_review_probes::probe_two_nonparallel_escapes_refuse_typed`
  and `probe_flipped_row_replays_bit_identical`;
- `m5_s13_pips::{die_pip_intersect_is_the_cap_and_additive,
  trimmed_sphere_group_operand_assembles_with_a_clear_partner,
  two_pips_cut_under_the_group_arm, die_pip_subtract_is_green}`;
- `m5_s12_curved_ops::{the_die_pip_sphere_shape_now_cuts_at_the_opened_door,
  finding_row_flipped_containment_fallback_now_sees_the_curved_extent}`
  and `m5_s12_curved_ops_interval::interval_sphere_subtract_decides_definitely_after_the_recut`;
- `m5_pr12_battery::{p1_radius_headroom_refuses_on_a_ball_tighter_than_the_blend,
  p3_spine_regularity_refuses_before_the_torus_is_minted,
  the_battery_passes_on_a_pip_rim_as_a_closed_chain}`;
- `review_pr12_probes::{probe_a_pipped_cube_all_edges,
  probe_d_rim_arc_orientation, probe_g_door_a_fields,
  probe_h_door_a_closed_tool}`;
- `review_fillet_e2_probes::the_ring_recourse_reaches_the_front_door_off_the_sample_lattice_and_is_followable`,
  `blend_recourse_followability::the_ring_recourse_is_screened_first_on_a_lattice_aligned_dimple`,
  `review_arceval_r1_probes::e1_ball_minus_cap_block_decides_definitely_at_interval`;
- `topo` `offer_rows::every_site_names_the_decision_it_raises` and
  `sweep` `offer_rows::every_sphere_offer_passes_just_below_it`.

Which of these move because the answer is now right (the non-parallel
pair builds both caps), which because a pinned shape moved (a pip's
seams now run where the cut put them), and which are wrong is the
work: none was read here.

## What a fix owes

One answer: the cut for every escape, `apply_recuts`, `SphereRecut`,
`recut_lean` and the two rotation questions retired with their offer
rows, each moved row read and re-baselined or fixed, and the
non-parallel pair under every op against its two caps.
