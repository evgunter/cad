---
id: two-arcs-of-one-cone-section-are-stated-as-two-anchored-ellipses-so-no-reading-knows-them-one-conic
kind: issue
title: A plane-and-cone section's two arcs are stated as two anchored ellipses a half turn apart, so no reading can know them one conic, and every cut cone's plane cap refuses the embedding certificate at f64
status: open
opened: 2026-11-23
priority: P0
cost: M
---


## Finding

From EMBED's lane `embed/cone-image` (PR 7605, 2026-11-23), on the
landing branch `embed/wave3-land` at `8d34076a9e` (main `50c8275810`
with PR 7203's anchored ellipse).

`crates/geom-core/README.md` on main (PR 7523): "A conic is the curve
its stored numbers state, and no reading replaces it with a rounded
copy … A conic's position is the point it stores plus two vectors times
two functions of its parameter … Its centre is a datum of the whole
conic, stored or derived." Two anchored ellipses that state one curve
twice are two statements, and no reading can prove them one from two
rounded statements. The embedding certificate compares two ellipses by
`same_content` and one `v` (`one_stated_conic`,
`crates/topo/src/injective.rs`), and that is right.

**A cut cone states its section twice.** The frustum of radii 1 and 0.5
over a height of 1 about `y`, split by the plane through `(0, 0.5, 0)`
whose normal is `y` tilted 0.25 toward `+x` (`cone_cut` in
`crates/sweep/tests/self_intersection_proves_injectivity.rs`,
`placed_cut(ON_GRID, (0.5, 0.25, 0.0), true, AsBuilt)`): the section's
two arcs, as the body stores them,

- `Ellipse { at: Frame { origin (−0.6650876241363457, 0.6698247517273087, 0), axis (0.24740395925452296, 0.9689124217106448, 0), u_ref (−0, 0, 1) }, held: Arc { phase: (0, −1) }, semi_u 0.7561882066647966, semi_v 0.7868899755863784 }`
- the same `at.origin`, axis and semi-axes, `u_ref (0, −0, −1)`, `phase (−0, 1)`.

What each comparison answers on that pair:

| comparison | answer |
|---|---|
| `same_content` | `false` (`u_ref` and the phase differ) |
| `same_content` and one `v` (`one_stated_conic`) | `false` |
| the derived `frame()`s | one origin to the bit, `(0.09733984772888238, 0.4751450562695438, 0)`, one axis, equal semi-axes, `u_ref` OPPOSED: one ellipse restated a half turn on |

**Which door.** `geom_brep::plane_cone_section`
(`crates/geom-brep/src/intersect.rs`) mints the section's ellipse once
per call through `Curve3::ellipse_anchored(vertex, n, toward, …)`, held
at its vertex nearer the apex's foot. The stored arcs are not that
statement: each is held at one anchor with `u_ref` along the MINOR axis
and its own phase. Which function restates the section per arc (the
split's per-face section, or the cut at the ruling's vertex) was not
identified: the lane read the stored values, not the path that wrote
them.

**Before the merge** (landing `ef332af96`, before PR 7203) the same
arcs were centre-held, `Ellipse { frame: Frame { origin, axis, u_ref },
semi_u, semi_v }`, their frames one up to the sign of `u_ref`, and
`one_stated_conic` read them through `one_stated_plane` on the frames
(each axis equal or opposed) with equal semi-axes: one conic. (The one
pre-merge pair that failed was stated a QUARTER turn apart with the
semi-axes exchanged: EMBED's
`two-arcs-of-one-ellipse-stated-a-quarter-turn-apart-are-not-one-conic`.)

**What it costs.** The cut's plane cap is bounded by those two arcs,
meeting at a vertex. Its corner there is read only as two arcs of one
conic (`Reading::Conic`, `continues`); with `one_conic` false the cap is
`Structural` at that corner at every depth, and the body refuses
`Uncertified { a: 5v1 (a half-cone), b: 8v3 (the cap), shared: Edge(11v1) (a section arc), arm: Departure }`.

## Count

The 24 cone/cone tests of the per-solid census
(`measure/gate/census-c2a0c41f40.json`), through the at-rest gate
(`embed/cone-image-measure`, `measure/cone-image/`):

| tree | pass |
|---|---|
| landing `3e433e678`, before `embed/cone-image` | 0 |
| `embed/cone-image` on landing `ef332af96`, before the merge | 12 |
| `embed/cone-image` merged with landing `8d34076a9e` | **1** |

The 11 lost each refuse on a Cone/Plane pair across a section arc:
`reach_cone_split::a_tilted_cone_cut_is_never_misread_by_containment`
(0.2 s, the cheapest), `…::tilted_cuts_across_the_elliptic_range_are_exact_on_both_nappes`,
`…::cuts_through_a_cap_and_the_apex_region`,
`near_circular_sections::a_near_square_split_of_a_frustum_validates_its_halves`,
`cone_operand_rows::the_tilted_planes_ellipse_builds_its_closed_forms`,
`cone_join_lane::each_poses_body_is_its_closed_form_in_every_op`,
`reach_probes::a_cone_face_bounded_by_a_section_ellipse_meshes_on_the_cone_row`,
`…::the_unit_funnel_cut_near_its_apex_meshes_at_every_delta`,
`…::a_cone_face_through_its_apex_meshes_on_the_cone_row`, and the two
`trimmed::tests`. Every cut cone the lane probed besides refuses the
same way (15 placements of five bodies), with the two halves of the
wall holding when asked alone.

Not checked: a plane-and-cylinder section. The rod less a tilted slab
holds at both scalars on the merged tree, so its two arcs are one
statement.

## The two ways out

- the door states both arcs by ONE stored conic, as a split's two
  children of one edge are a restatement verbatim; or
- it records that the two are one, as `embed/collinear-cover`'s lines
  are joined by key, and the corner reading reads the record.

Which, and where the record would live, is this slate's.

## Pin

`self_intersection_proves_injectivity::a_cut_cones_wall_holds_in_every_placement_and_its_body_refuses_on_a_cap`
and `…::a_needle_and_a_near_flat_cone_cut_read_as_any_cone` pin the
refusal on the cap (`refuses_on_a_cap_and_its_wall_holds`); they are
re-pinned to `Embedded` when this closes.
