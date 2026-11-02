---
id: extent-reads-made-outside-the-extent-home
kind: issue
title: consumed-extent reads still made outside geom_brep's extent home, and hidden levers outside Margin::levered
status: open
opened: 2026-11-02
priority: P2
cost: M
---


Found by the `onehome/one-extent` unit (2026-11-02). `Lever` now names
its source, but some consumed-extent reads are still made outside
`geom_brep::extent` and enter it only as bare lengths, and some
dimensionless readings are still levered without `Margin::levered`:

- `geom_brep::Reach::Face { below, above, across }` is built in topo
  from bare `T` measures (`splitting::rules::face_axial_range`,
  `face_reach_from(..).length()`, `chord_join::WallReach`); the
  variant's fields are lengths the caller vouches for.
- editor-core `eval::measure::curve_reach` repeats
  `Reach::Span::lever_from`'s per-carrier reads; sweep `blend::battery`
  `extent_of` (nine samples) overlaps `edge_extent_inf`; topo
  `splitting::containment`'s `step_ball`, `carrier_ball`,
  `conic_arc_ball` and `ruled_region::RuledFaceRegion::reach` are
  extent reads that return `T`.
- Hidden levers outside the door: `offset_derive` `holds_the_move` and
  `translates_along` (`Margin::of(dir·m · reach)`, at the requested move
  length), sweep `blend::arms`' `Meridian`/`Ruling` departures (a raw
  `sin · lever`). The gate cannot see these.
- Rate directions the conversion carried as found: `props::closure`
  `loop_turns` meters a definite claim at the MAX of the junction radii
  (`InfSpeed`); `pcurve_cache`'s fidelity twin and focal `beta` meter
  definite claims at a sup rate; `props::curved` `fold_torus_pieces`
  uses the tube radius as the rate of rim pieces too (`R ± r·cos v`).
