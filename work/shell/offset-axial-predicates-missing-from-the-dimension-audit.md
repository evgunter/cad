---
id: offset-axial-predicates-missing-from-the-dimension-audit
kind: issue
title: docs/predicate-dimension-audit.md lists a third of offset_axial.rs's decide names
status: open
opened: 2026-09-08
---


Found by the SHELL-7 R1 review lane (its NOTE-11): `docs/predicate-
dimension-audit.md` lists a third of the `decide(...)` names in
`crates/topo/src/offset_axial.rs` — R1 counted 11 of 33 at head
`4adb70dd`. The audit's own charter is that every predicate the funnel
takes has a row saying what its margin measures and in what unit, so
the module's other names (`offset_axial_request`, `_pole`,
`_pole_station`, `_pole_centre`, `_radius`, `_concurrence`,
`_azimuth_arm`, `_azimuth_amp`, `_azimuth`, `_azimuth_residual`,
`_branch`, `_side`, `_chart_motion`, `_edge_agreement`,
`_edge_on_surface`, `_seam_radial`, `_seam_concentric`,
`_seam_meridian`, `_alignment`, `_meridian`, `_meridian_through`,
`_latitude_tilt`, `_reauthor_plane`, and the ones SHELL-7 itself
added or folded) owe rows. A docs-tier unit: read each site, state its
quantity and lever, add the row.
