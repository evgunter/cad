---
id: a-band-seam-the-certificate-certifies-reads-under-determined-on-a-loose-drift-bound
kind: issue
title: Blend-band seams that main stores as TangentIntersection read under-determined since the walk demands the tube: kappa_drift for a non-coaxial cylinder-torus pair dwarfs kappa_rel
status: open
priority: P1
opened: 2026-11-24
---


Found by HELDRECORDS' elbow-seams fix pass (2026-11-24), on PR 7471
(`heldrecords/elbow-seams`), from its second reviews: seams across
about twenty blend and band suites rest conventional where main stored
`TangentIntersection`. Filed on SELECT, owner of the walk
(`crates/geom-brep/src/dihedral.rs`).

## What changed

`second_order_walk` answers `Determinate` only where `|κ_rel|` is
certified bounded away from zero over the span: the least `|κ_rel|` less
`TangentSpanBounds::kappa_drift`, decided under `tangent_tube_margin`
(`dihedral::tube_separation`, the tangency certificate's own tube). A
walk whose every station reads positive and whose tube does not is
`Separated`, and the must-carry rule and the fold store the
conventional description.

## Measured

An env-gated census of every walk that reads every station positive and
does not certify the tube, over sweep's and editor-core's rows matching
`band|blend|fillet|chamfer|round|contact_edge|must_carry` (1290 rows,
slow set included, default ε; the instrumentation was not committed):

- 7782 such walks. By pair: cylinder–torus 5432, torus–torus 66 (tube
  negative); plane–cone 2186 and cone–cone 96 (no drift bound: out of the
  certificate's lane, so demanded nothing on main either).
- The cylinder–torus walks read a least `|κ_rel|` of 1e-30 to 1e-14 at
  the span's ends, against a `kappa_drift` of 42.8 to 196 per span.
  Their pairs are not coaxial, so `circle_span_bounds`' Killing-field
  deviation is the clamp and the drift is enormous.
- The end samples decide nothing: a first census of walks whose tube
  differs with and without the end samples found none in the same 1290
  rows.
- By suite (walks; one row's walk runs many times):
  - 672 band_mixed_rim_shared_supports::rectangular_holes_ending_at_a_corners_window_build_at_pappus
  - 640 band_mixed_rim_shared_supports::holes_beside_a_corner_build_in_one_call_as_in_sequence
  - 544 band_closed_mixed_chain_shapes::adversarial_outlines_build_where_their_bands_fit_and_refuse_where_they_overlap
  - 456 band_closed_mixed_chain_shapes::rounded_outlines_of_other_shapes_chamfer_whole
  - 432 band_closed_mixed_chain_shapes::turned_rounded_outlines_fillet_whole
  - 384 band_closed_mixed_chain_shapes::turned_and_moved_rounded_rectangles_chamfer_whole
  - 384 band_closed_mixed_chain_shapes::arcs_flanking_a_short_flat_fillet_whole
  - 336 band_closed_mixed_chain_shapes::rounded_polygons_fillet_whole
  - 240 band_closed_mixed_chain_shapes::rounded_outlines_at_their_limits_chamfer_whole
  - 192 band_mixed_rim_shared_supports::an_outline_through_a_disc_of_a_few_arcs_fillets_at_pappus
  - 192 band_closed_mixed_chain_shapes::a_rounded_plate_with_a_rounded_through_hole_chamfers_every_edge
  - 192 band_closed_mixed_chain_shapes::a_lattice_drawn_rounded_plate_fillets_whole
  - 160 band_mixed_rim_shared_supports::a_rounded_plate_with_a_round_hole_fillets_every_edge_in_one_call
  - 144 band_closed_mixed_chain_shapes::rounded_outlines_drawn_in_arc_pieces_chamfer_whole
  - 144 band_closed_mixed_chain::a_rounded_plate_chamfers_all_round_at_pappuss_volume_at_interval

## Why it is a question, not a fix

On main these seams were stored `TangentIntersection` and certified,
but not through the tangency certificate's tube (`Resolved::Tangent`'s
`tube_separated`), which would refuse a drift this size. A blend
band's seam certifies through its section's own lane. So the demanded set
is the tube-certifiable set, as PR 7471 owed, but it is now narrower than
what the kernel certifies by its other lanes. Options:

- (a) The walk certifies through whichever lane the stored description
  would (the walk asks the certificate, not the tube).
- (b) A tighter drift bound for non-coaxial circle carriers, where the
  tube can certify them.
- (c) Keep the conventional description on these seams, since every body
  still validates at tier 3, and record that prefer-intrinsic (D2/OQ7)
  yields to certifiability here.

(c) is the state PR 7471 lands.
