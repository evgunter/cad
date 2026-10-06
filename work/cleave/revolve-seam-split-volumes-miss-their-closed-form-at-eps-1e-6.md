---
id: revolve-seam-split-volumes-miss-their-closed-form-at-eps-1e-6
kind: issue
title: the split-across-a-revolve-seam rows miss their volume closed form at CAD_TOLERANCE_EPS=1e-6 on main
status: open
opened: 2026-10-06
priority: P1
cost: E
refs: [4120, split-halves-volumes-sum-to-the-whole-only-within-their-pads]
---


Main is red at the 1e-6 row since PR 4120 (`cleave/tube-across-axis`) merged. Measured on `origin/main` at `78bee3ac` with `CAD_TOLERANCE_EPS=1e-6 cargo nextest run -p sweep -E 'test(split_across_a_revolve_seam) | test(m5_pr6_pcurves)'`. Four rows fail, and default eps and 1e-12 pass:

- `split_across_a_revolve_seam::a_tube_cut_across_its_axis_splits_into_two_annular_halves`: "about y, tilt 0.0997, azimuth π, s = 1": 1.1781107213234776, want 1.1780972450961724.
- `split_across_a_revolve_seam::a_section_touching_a_rim_splits_at_the_closed_form`: "rim 1, azimuth 0, s = 1": 0.4776255466374772, want 0.47762426877222874.
- `split_across_a_revolve_seam::a_counterbore_and_a_cone_socket_split_across_their_axes`: "counterbore at y = 0.3, tilt 0.1, s = 1": 0.8576549532572836, want 0.8576547944300136.
- `m5_pr6_pcurves::a_seam_closed_tube_split_mints_clean_halves` (`m5_pr6_pcurves.rs`, its volume assertion): reads 0.4523909410843751.

Each is a half's `mass_properties` volume against its closed form. The miss is 1e-7 to 1.3e-5 absolute, which is the size of the quadrature pad at that ε (`split-halves-volumes-sum-to-the-whole-only-within-their-pads`). It is not a topology failure: tier 3′ and the one-annular-section-face checks pass first. The same rows on `band/lamina-annulus-is-one-face` (PR 4136) fail with bit-identical numbers.

Recourse: compare each half against its closed form within the volume's own certified pad (`MassProperties`' bound) rather than within a fixed `by`, or tighten the quadrature these splits take at 1e-6. The first option is the row's own fix.
