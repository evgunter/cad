---
id: the-ray-walks-curved-and-fallback-readings-subtract-an-aim-from-a-carriers-own-origin
kind: issue
title: The ray walk's curved-carrier readings and the fallback aim subtract two points that may share a boxed term; none is measured over a box
status: open
opened: 2026-11-15
priority: P2
cost: M
---


Found by the ray-readings unit's sweep
(`an-aimed-rays-other-readings-still-carry-a-moving-faces-box-twice`,
branch `boxfinish/ray-readings`). That unit re-posed the three
readings it measured on planar bodies (the loops' turn, the aim's step,
the advance to the planes the aim's anchor lies on). These have the
same shape, "a difference of two points that may share a boxed term,
then a three-component dot, cross or norm", and are **read from the
code, not measured**: no sweep door finishes a curved body over a box
today, so no ray reaches them there
(`work/inside/a-whole-turn-circle-cap-refuses-corrupt-loop-over-a-box-where-f64-answers.md`).

An aimed ray's line origin `q` is its aim, a point stepped from the
aimed face. Each reading below subtracts it from another stored point
of the same face or carrier.

**In the walk** (`crates/topo/src/boolean/solid_contain.rs`):

- the carrier roots: `line_wall_roots` (`q − origin`, `wall.offset(q)`),
  `line_cone_quadratic` (`q − apex`), `line_sphere_roots` and
  `line_torus_roots` (`q − center`), read `About::Nearest`
  (the unit's item 5), and `wall_elevation`;
- `wall_crosses_by_turn`: the wall's image (`param_and_offset` of each
  vertex and of the root against the axis origin), a circle edge's
  centre against the axis (`Line::distance(center)`), and
  `edge_clearance`'s `b − a` (the sibling of `clearance_in_plane` the
  planar step now reads);
- `root_outward` (`p − apex`), the partial arms' `(p − apex).norm()`
  and `(q + d·t − anchor).norm()`;
- `sphere_chart_trim` (a rim's centre against the sphere's, both on
  one axis), in `face_geo`'s setup;
- `curved_edges::passes`: `carrier.clearance(seed)`, `carrier.foot`,
  `ray.foot`, with `seed = q + d·t`;
- `ray_passes_beside`: `(v − q)·m` and `lo − base`, a face's vertices
  and cover against the aim. Reached only where a ray runs parallel to
  a plane in band;
- `ray_crossings`' parallel arm: `plane_offset(origin, normal, q)`
  does not read the aim's anchor as the crossing arm does. An aimed
  ray meets its own face at 10° from the normal, so this is reached
  for the face across the anchor edge only, where that face is
  parallel to the ray in band;
- `EdgeMeets::vertex`: a vertex's distance from the ray's line. Its
  width is harmless where the distance is read definitely positive,
  which is what was measured on the slab.

**On the fallback aim** (`crates/topo/src/stands.rs`,
`planar_interior_point`, taken where `loop_aim` offers none):
`line_meetings`' `(origin − m).cross(dir).dot(normal)` and
`r = m − center`.

**What closes it.** A curved body finished over a box by a door, and
per reading: its enclosure against members' `f64` values, as the
unit's rows hold the planar three
(`side_reader::boxed_brick_rows::a_moving_caps_turn_is_as_narrow_as_a_members`
and its two neighbours).
