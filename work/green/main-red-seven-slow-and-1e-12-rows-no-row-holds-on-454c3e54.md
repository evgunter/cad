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

## Seen again on main `6a7c5430d`, with one more row (2026-11-23, ONEROUND's `readings-not-levers` lane)

Six of these and a seventh, slow set, release, ε 1e-12, on a clean checkout of `origin/main` at `6a7c5430d`, with no seed and again with `CAD_FUZZ_SEED=12345` (the seed changes nothing), and identically on PR 7442's head `12d3bdf41`:

- the five band rows (`band_closed_mixed_chain_shapes::a_four_arc_barrel_fillets_whole`, `::rounded_polygons_fillet_whole`, `::turned_rounded_outlines_fillet_whole`, `band_open_mixed_chain::an_l_plates_rounded_corner_run_fillets_whole_at_interval`, `band_open_mixed_chain_shapes::an_s_run_of_a_convex_and_a_reflex_arc_builds`), each at `band_closed_mixed_chain_shapes.rs:445`: "at Interval at ε 1e-12 the section's shared row escalates its touch, got UnsupportedGeometry { … detail: "a junction's section does not certify as the shared row of its two bands" }";
- `section_ellipse_seam_poses::planes_through_the_axis_nearly_along_it_split` (`:169`): "poses off their table (…): θ 1e-6 φ 0.4 z 0.5: at-rest gate above URef Tilt Lower & URef Tilt Upper (want ok)";
- **not listed above:** `sweep::all section_ellipse_seam_poses::a_slab_face_nearly_along_a_ruling_builds` (`section_ellipse_seam_poses.rs:319`): "dd -5e-11 t 1e-2 ∪(cyl, slab): ResultInvalid { … VolumeUncomputable { solid: SolidKey(1v1), source: Face { face: FaceKey(4v1), source: Escalated { cause: Indeterminate { margin: MarginDiag(Value(7.960010676797084e-12, …)), band: Band { zero: 1e-12, escalate: 1e-11 }, predicate: Some("props_chart_junction_closed") }, check: Inventory } } } }" (and the ∩ line, face 3v1, margin 7.930562011068901e-12). The same predicate as the prism-corner row red at 1e-12.

```
CAD_TOLERANCE_EPS=1e-12 cargo nextest run --release -p sweep --ignore-default-filter --no-fail-fast -E 'test(=section_ellipse_seam_poses::a_slab_face_nearly_along_a_ruling_builds)'
```
