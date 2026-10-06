---
id: split-seam-closed-form-rows-fail-at-eps-1e-6
kind: issue
title: four split-across-a-revolve-seam closed-form rows fail at eps 1e-6 on main: their comparison is tighter than the 1e-6 band
status: closed
opened: 2026-10-06
priority: P0
cost: E
closed: 2026-10-06
pr: 4083
---


Found by SHELL's PR 4115 lane (2026-10-06). On its head `01e80431`,
merged with main, a full `--profile ci --workspace` run at ε = 1e-6
failed these four. They fail identically with `origin/main`'s
`shell.rs` and `containment.rs` swapped in, so they are main's, not
4115's:

- `split_across_a_revolve_seam::a_section_touching_a_rim_splits_at_the_closed_form`
  (closed-form mismatch, 0.4776255 vs 0.4776243);
- `split_across_a_revolve_seam::a_counterbore_and_a_cone_socket_split_across_their_axes`;
- `split_across_a_revolve_seam::a_tube_cut_across_its_axis_splits_into_two_annular_halves`;
- `m5_pr6_pcurves::a_seam_closed_tube_split_mints_clean_halves`.

`split_across_a_revolve_seam.rs` last changed on main in `bf06e9a0`
(CLEAVE's tube-across-axis, merged via PR 4120). The rows' comparison
appears tighter than the 1e-6 band: they pass at default ε and 1e-12.
Main's PR gate runs sweep's 1e-6 row only for diffs that touch a sweep
probe or golden path (CIW's
`a-new-test-file-outside-the-eps-crates-never-runs-at-the-extra-eps-rows-before-merge`),
so it could not see them. The closest existing row,
`a-rim-touching-split-escalates-on-the-side-of-plane-band`, is a
different symptom. Owed: the tolerance each row compares at should be
the band's, or the row says why it is tighter, and every sweep PR's
1e-6 row goes green. Filed by the SHELL orchestrator.

## Closed (PR 4083, 2026-10-06)

Fixed in PR 4083. Its merge with main ran these rows at the 1e-6 row, which main had never done. The
four `split_across_a_revolve_seam` rows and
`m5_pr6_pcurves::a_seam_closed_tube_split_mints_clean_halves` now hold each volume to 1e-8 plus its
own certified quadrature pad, and pass at 1e-6, 1e-9 and 1e-12. The two rows filed for this red are
closed together.
