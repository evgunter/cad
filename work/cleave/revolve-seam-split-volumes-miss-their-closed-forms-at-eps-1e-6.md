---
id: revolve-seam-split-volumes-miss-their-closed-forms-at-eps-1e-6
kind: issue
title: four revolve-seam split rows miss their closed-form volumes at eps 1e-6 on main
status: open
opened: 2026-10-06
---


## What

Found by SHELL's `shell/axial-closed` (PR 4151). Its diff touched an
unscopable doc, so CI's eps step ran the whole workspace. Four rows
fail at `CAD_TOLERANCE_EPS=1e-6` on `origin/main` at `f71c22688`,
measured locally in a clean worktree. They fail identically on the PR
branch:

- `split_across_a_revolve_seam::a_section_touching_a_rim_splits_at_the_closed_form`
  — `rim 1, azimuth 0, s = 1: 0.4776255466374772, want 0.47762426877222874`
  (`near`, the file's line-79 helper, called from line 247).
- `split_across_a_revolve_seam::a_counterbore_and_a_cone_socket_split_across_their_axes`
  — `counterbore at y = 0.3, tilt 0.1, s = 1: 0.8576549532572836, want 0.8576547944300136`
  (from line 140).
- `split_across_a_revolve_seam::a_tube_cut_across_its_axis_splits_into_two_annular_halves`
  — `about y, tilt 0.09966865249116204, azimuth 3.141592653589793, s = 1: 1.1781107213234776, want 1.1780972450961724`
  (from line 101).
- `m5_pr6_pcurves::a_seam_closed_tube_split_mints_clean_halves` —
  volume `0.4523909410843751` against `0.144π` within `1e-8`
  (`m5_pr6_pcurves.rs`, the closing assert).

The misses run from `1.6e-7` to `1.3e-5` m³. The tolerances are fixed
numbers that do not scale with ε. What is open is whether the split's
volume error is legitimately ε-scaled, so the tolerance should be
levered by ε, or whether the 1e-6 row builds a wrong body.

`sweep::pinch_faces_tessellate::a_face_through_two_vertices_on_one_point_tessellates`
fails in the same run. It is already JOIN's
`pinch-tessellate-row-escalates-at-eps-1e-6`, and this item does not
carry it.
