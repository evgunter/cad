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

**Second pass** (a grep for `Margin::of(` over a product with an arm or a
trigonometric reading, production code): `topo::shell` `walls_cross`
(`Margin::of(cross.norm() * lever)`, the lever three wall spans summed
through `gate_measured`), `boolean::join` `SEAM_GERM_SIDE`
(`into_region.dot(n_f) * len`), `boolean::reduce` (`disc / (a2·reach)`,
a reciprocal arm), `geom_brep::half_angle` `root_reach`
(`2·speed_hi·reach`, a rate times a reach). Each levers a pure number
outside `Margin::levered`, so its arm names no source and the gate
cannot see it. What the grep cannot match: a product computed into a
variable first and handed to `Margin::of` by name.

**Pivot not the centre** (PR 6391's review, 2026-11-02). `ExtentBall::lever()`
is the ball's radius: its reach from its own centre. Where the reading it
meters pivots on the rim or elsewhere, the reach is up to `2r`, and the
arm under-states it (the values are main's; the type only names them).
Sites: `props::curved` (`of_circle(c_c, r_c).lever()` and kin at the
circle-axis class, the meridian rows and the rim levels: about 11
sites), `pcurve_cache` spiric `chart_tilt` (`of_torus`), `intersect`
`pt_axis_normal` (`of_torus`), `pair_section::gap` slope and
`torus_coincidence` (core circles), and in topo `boolean::arcs`
`arcs_along`, `boolean::trace` circle crossing, `boolean::boxes`
`sphere_window`, `boolean::solid_contain` `sphere_chart_trim` (2),
`chord_join` `holds_outer`. Each should read the lever from its own
pivot (`lever_from(pivot)`), which moves its margin and wants the
row's own differential.
