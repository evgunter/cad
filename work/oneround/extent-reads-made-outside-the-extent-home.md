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
`sphere_window`, `solid_contain` `sphere_chart_trim` (2),
`chord_join` `holds_outer`. Each should read the lever from its own
pivot (`lever_from(pivot)`), which moves its margin and wants the
row's own differential.

## Slate triage (2026-11-08, ONEHOME split)

- Moved to ONEROUND (`work/oneround/`) by `git mv`, id and body unchanged. ONEHOME measured 78 budget points against 35 after PRs 6371 and 6380 landed, and split along its seams (`work/README.md` Track size): the rows on f64 rounding, levers and the consumed extent that PR 6380 and PR 6391 filed went to ONEROUND.

## Added 2026-11-19 (`oneround/lever-doors`)

- First bullet: `Reach::Face` is still built in topo from bare measures. `scripts/gates/lever-doors.sh` pins `Reach::of_face`'s three production callers; nothing else about it changed.
- Second bullet: sweep `blend::battery::extent_of` is pinned by the same gate as a `points_diameter` site (`lever-doors-pinned-sites-await-a-geometry-read`).
- `edge_extent_inf` and `edge_extent_sup` lost their end-points argument: the chord is read off the carrier at the span's ends, so a caller no longer hands the extent home two points of its own.
- **Hidden levers, the `Margin` doors that take a bare arm** (asked by the ONEROUND orchestrator, checked in the tree):
  - `Margin::sagitta(κ_rel, arm)` took `arm: T` and squared it. It takes `Lever<T>` now. Its 11 production callers and the functions that carried the arm to them (`enters_material_order2`, `parting`, `parting_side`, `tied_germ_turn`, `bends_into_material`, certify's `tube_separation`, topo's census `separates`) are retyped; nearly every caller already held a `Lever` and stripped it with `.length()`. The census point lanes mint theirs from the face's sphere and the edge's circle.
  - `Margin::over_lever(measure, lever: T)` and `over_lever_down` are untouched: 32 and 2 production calls. The lever is a divisor, so it is a reciprocal arm as `levered_inv`'s `per_length` is, and a longer one shrinks the margin. The sites hold a perimeter, an area, a radius, a straddle height or a rate bound computed in place: `pcurve_cache` (8), `quartic` (7), `solid_contain` (4), `props::settle` (2), `loop_winding` (2), `geom::curves::compose` (2), `offset_meters` (2), and one each in `splitting::join`, `ray_walk`, `profile::validate`, `props::quad::run`, `props::curved`, `half_angle`, `edge_nurbs`. Few hold a `Lever`, and what the divisor's source should be is a design question, so it was not started.
  - `geom_brep::sector_shape::metering_arm(arm: T) -> Lever<T>` is a door: 9 production calls reach its one pinned `unsourced` reference (2 in its own file, 2 in topo's `sector_shape`, 3 in `star::walk`, 2 in `splitting::insert`). Left to `unsourced-levers-await-a-source`.
- **`Lever::max` is called where its two arms do not share a pivot** (PR 7143's delta review). Its doc states the rule: the larger of two arms is a read of the union of their extents where both are read from one pivot. Two callers break it:
  - `geom_brep::curve_pair::pose` (the `arm` of its two carriers' balls): `ball(a.carrier()).lever().max(ball(b.carrier()).lever())` maxes two carriers' cover balls, each a radius about its own centre. It should read one ball enclosing both (`ExtentBall::enclosing`) from the pivot the reading is taken at.
  - `geom_brep::intersect::plane_cone_section`: `ExtentBall::of_cover(&c).lever().max(extent)` maxes a rim's ball radius against an extent read elsewhere. It should read the rim's cover from the extent's own pivot (`lever_from`), or enclose both.
  - `topo::boolean::edge_join::joinable`: `[e1, e2]` mapped through `edge_wedge_reach` and reduced with `Lever::max`, each edge's wedge reach read from its own midpoint. It should read both edges from one point.
  - `step_import::build::choose`: an edge extent, `faces_reach`'s diameter and, per cone, `region.lever_from(apex)`, folded with `max`: three reads from three reference points. It should read the region's cover from the one pivot its verdict is taken at.
  Three more read ONE extent from two pivots and take the larger, which is a bound from either pivot and not the union's reach: `geom_brep::intersect::route_pose` (`reach.lever_from(anchor)` over each surface's anchor, reduced with `max`), and `topo::boolean::section_cert::cone_cone` twice (`reach.lever_from(one.apex).max(reach.lever_from(two.apex))`, and the same with `twice_lever_from`).
