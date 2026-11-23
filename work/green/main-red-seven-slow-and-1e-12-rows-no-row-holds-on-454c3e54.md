---
id: main-red-seven-slow-and-1e-12-rows-no-row-holds-on-454c3e54
kind: issue
title: Seven rows are red on main 454c3e54 (one at the default eps, six at 1e-12) and no work row names them
status: open
opened: 2026-11-22
---


Measured by the BOXFINISH doors' lane (2026-11-22) on `origin/main`
`454c3e54` and on its branch merged onto it, as a slow-set
differential (`cargo nextest run -p <crate> --ignore-default-filter
--no-fail-fast`, `CARGO_PROFILE_{DEV,TEST}_OPT_LEVEL=1`): each row
below is red on both, with the same payload, and `grep -rl` of its
name over `work/` finds no row (`log.md` files aside).

- default ε, `editor-core`:
  `split_pinch_keeps::the_kite_moats_subtract_names_its_edges_where_its_result_stands`
  (`the kite moat's evaluation, bodies and name tables`: digest
  `6885395609920571358` against the pinned `11878713692985015632`);
- `CAD_TOLERANCE_EPS=1e-12`, `sweep`:
  - `band_closed_mixed_chain_shapes::a_four_arc_barrel_fillets_whole`,
  - `band_closed_mixed_chain_shapes::rounded_polygons_fillet_whole`,
  - `band_closed_mixed_chain_shapes::turned_rounded_outlines_fillet_whole`,
  - `band_open_mixed_chain::an_l_plates_rounded_corner_run_fillets_whole_at_interval`,
  - `band_open_mixed_chain_shapes::an_s_run_of_a_convex_and_a_reflex_arc_builds`,
  - `section_ellipse_seam_poses::planes_through_the_axis_nearly_along_it_split`.

Not read past the failing assertion; whose merge moved each is
unmeasured.
