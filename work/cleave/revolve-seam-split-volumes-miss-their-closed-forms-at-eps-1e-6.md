---
id: revolve-seam-split-volumes-miss-their-closed-forms-at-eps-1e-6
kind: issue
title: four revolve-seam split rows miss their closed-form volumes at eps 1e-6 on main
status: dispatched
opened: 2026-10-06
priority: P1
cost: E
branch: cleave/revseam-1e6
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

## Built (branch cleave/revseam-1e6)

No code change. The four asserts already hold the volume to its
certified bracket on main: commit `6ac174bf1a`, merged with PR #4083
after this row's `f71c22688` measurement. `split_across_a_revolve_seam`'s
`near_vol` asserts `|v − want| ≤ 1e-8 + volume_pad`. The
`m5_pr6_pcurves` assert is `|v − 0.144π| < 1e-8 + volume_pad`.

Measured on `8e5dc42c5e` at ε = 1e-6, the default and 1e-12. All
five tests in the two modules pass at all three rows. Before the
merge, the values were bit-identical on `44dced75f0`. At 1e-6 they
reproduce the row's failing numbers exactly.

Every row is verdict (a): the closed form lies inside `v ± volume_pad`.

| row (ε = 1e-6) | max abs(v − want) | its pad | err/pad |
|---|---|---|---|
| tube across its axis | 1.35e-5 | 1.20e-3 | 1.1 % |
| counterbore and socket | 8.99e-6 | 1.15e-3 | 0.9 % |
| section touching a rim | 2.98e-5 (sum), 2.44e-6 (cap) | 2.97e-3, 5.4e-4 | 1.0 % |
| m5 seam-closed tube | 1.60e-6 | 3.47e-4 | 0.5 % |

The error scales with ε. Its largest value is 1.1e-9 at the default
row and 1.6e-13 at 1e-12, at most 0.06 % of the pad. Square cuts and
the socket carry no pad and match the closed form to 4e-16.

The verdict was corroborated with a throwaway probe, which is not
committed:

- **Halves sum to the whole.** The tube halves sum to 0.75π within
  1e-14, because their pads are equal. The counterbore halves miss by
  8.8e-6 against a combined pad of 1.2e-3. The rim halves miss by up
  to 3.0e-5 against 3.0e-3. In both cases the two halves' pads differ,
  which is CLEAVE's
  `split-halves-volumes-sum-to-the-whole-only-within-their-pads`; the
  evidence is added there.
- **A second construction agrees.** The same solids were built about
  `z`: a two-seam cylinder less rods (`cut(turned_cylinder(0.3, 1), rod(…))`).
  They were split by each plane's image under the quarter-turn about
  `x` taking `y` to `z`. Every half lands inside its own pad. Body
  against body, the halves differ by at most 1.27e-5 (tube), 8.8e-6
  (counterbore) and 3.9e-6 (rim cap). Each difference is under 1 % of
the two bodies' combined pads.
- At the default ε and at 1e-12, `mass_properties` refused one
  second-construction half with an escalated `props_quad_converged`.
  The evidence is added to QUAD's
  `quadrature-convergence-test-escalates-instead-of-refining`.

The cross-check against QUAD's
`sweep-tests-hold-quadrature-midpoints-to-fixed-tolerances` found
nothing to fix here. None of that row's listed instances is in these
two files, and both files already read `volume_pad`, so that row is
unchanged.
