---
id: a-cylinder-rim-on-a-turned-frame-stores-a-rounding-in-its-axial-channel-at-f64
kind: issue
title: A circle square to a cylinder's axis is stored with a rounding residue in its axial channel where the outline is built off the axes, at f64, so the injectivity check reads a fillet strip's end arc as a curve and its corner by the ruling run on past the vertex (traced on one test)
status: open
opened: 2026-11-22
priority: P0
cost: E
refs: [a-cylinder-rims-axial-channel-holds-zero-only-to-an-enclosure-at-interval, a-line-tangent-to-an-arc-at-their-vertex-is-read-by-no-corner-presentation]
---

## Finding

From EMBED's lane `embed/tangent-corner` (2026-11-22), main `0c1eb97ad`
with the at-rest gate (`embed/gate-measure`) and the lane's change,
`f64`, a probe at `topo::injective::present_line_corner` printing the
corner's other edge's use.

`band_closed_mixed_chain_shapes::rounded_polygons_fillet_whole` (a
triangle of corner radius 0.3, every edge filleted at 0.1): the
cylinder strip `26v1` ends at the arc `26v3`, its section circle,
square to the strip's axis. The strip's chart image of that arc is
stored as

```
Harmonic { p0: [3.926990816987241, -0.3464101615137755],
           pa: [0, -6.938893903907228e-18], pb: [0, 0], pl: [-1, 0] }
```

a parallel `v = −0.34641…` but for a cosine term of `−6.9e−18` in the
axial channel: the residue of `u_ref · axis` on a frame turned off the
axes. `topo::clearance::iso_of` reads a coordinate line by exact
structure (the channel's coefficients lifting to the point zero), so
this use carries no `iso`. On an axis-aligned body
(`band_mixed_corner::an_inset_corner_with_all_three_edges_requested_builds_at_its_closed_form`)
the same arc is stored as a coordinate line.

**Where the class is decided, and why it is not here.** The mint does
decide this class: `geom_brep::pcurve_cache::derive_class`'s cylinder
arm, the moving case, passes a conic's axial `cos t` and `sin t` terms
through `cylinder_rim_axial`, which writes zero where both hold zero and
their sup is decided `Zero` at `pcurve_cylinder_chart_axial` (the fix
that closed
`a-cylinder-rims-axial-channel-holds-zero-only-to-an-enclosure-at-interval`).
It asks that only of a bracket that holds zero. Its own doc: "At `f64`
a value holds zero only when it is zero, so nothing there is decided or
moved." `−6.9e−18` is an `f64` point that is not zero, so it is handed
on as given: the rim's class is decided at the band for an enclosure
and not for a point a rounding off it. The cone arm beside it
(`pcurve_cone_chart_axial`) decides the amplitude itself
(`Margin::of(harmonic_sup_scalar(..))`), at every scalar.

Not measured: which door minted this pcurve. Its form
(`Angle { winding: −1 }` over a free axial channel) is
`Derivation::CylinderMoving`'s, but the blend builder may have stated
the image and had it certified rather than derived; a probe at the
mint would say.

Not the class of
`a-cell-whose-side-lies-a-rounding-off-a-curved-edge-reads-local-at-every-depth`
(a cell's side a rounding off an edge the cell does not hold): here the
stored image itself is a rounding off a coordinate line.

## What it costs

The injectivity check reads the strip's corner at the arc's end (the
ruling that is the strip's foot on its support plane, meeting the
section arc) as a coordinate line meeting a curve
(`present_line_corner`): its rectangle is not clipped at the arc, and
its zero side runs on along the ruling past the vertex, `6.8e−4` in
the traced cell. The support plane's cell at the same vertex is one
run of the straight foot and the next patch's arc
(`a-line-tangent-to-an-arc-at-their-vertex-is-read-by-no-corner-presentation`),
and the graph separator along that run cannot vanish on the ruling's
continuation as well. The pair refuses
`Uncertified { Departure }`, plane against cylinder across the straight
foot, at depth 40.

**Count.** One test is traced to the stored use
(`band_closed_mixed_chain_shapes::rounded_polygons_fillet_whole`,
`1v1 × 26v1`, edge `48v1`). A rigid move does not make it: the stored
images move with the body. The body has to be built off the axes; a
`2 × 1.2` rounded plate drawn turned 30° refuses this way and is
pinned (`self_intersection_proves_injectivity::a_support_face_c1_through_a_vertex_is_embedded_at_its_tangent_joints`).

Candidates, not traced: with the gate asking and the tangent joint
read, 11 `f64` tests of the two tangent-joint rows' 66 refuse plane
against cylinder across a line, every one an outline off the axes
(`turned_rounded_outlines_fillet_whole`,
`adversarial_outlines_build_where_their_bands_fit…`,
`a_hole_beside_a_corner_reaching_a_links_strip…`,
`the_foot_inside_a_face_recourse…`,
`every_edge_of_a_rounded_pocket_fillets_at_its_closed_form`,
`a_mixed_corner_at_a_non_right_plan_angle…`,
`a_feature_entering_the_patch_at_a_non_right_plan_angle…`,
`a_turned_and_moved_mixed_corner_reads_as_it_does_in_place`,
`a_hole_over_a_wide_arcs_band…`,
`an_s_run_of_a_convex_and_a_reflex_arc_builds`,
`arcs_of_a_half_turn_or_more_build`). That they share the cause is a
guess from the pair's shape.

## What is needed

`cylinder_rim_axial` deciding the axial amplitude at the band at every
scalar, as the cone arm does, so a section circle is a coordinate line
by variant in every pose; with what that drops stated (the drift is
within the band, and the certificate's envelope bounds what it moves,
as that function already says of the enclosure). Not a reading in
`topo::injective` that clips at a curve a rounding off a parallel:
that would be a concession.

## A second sighting, likely the same cause, untraced (2026-11-23)

`self_intersection_proves_injectivity::one_uneven_plate_is_embedded_in_every_rigid_pose_and_its_axis_mirror`
(PR 7320, the re-review's probe): a rounded plate of four different
corner radii, every edge filleted, is `Embedded` at `f64` as built, in
three rigid poses and mirrored across `x = 0`. Mirrored across a
generic plane (through `(0.2, −0.1, 0.3)`, normal `(0.3, −0.5, 0.81)`)
it reads `Uncertified Departure`, cylinder against plane across an
edge: the exit of this row's traced test. The mirror re-mints the rows
on a turned frame, where a rigid move keeps the stored images. Likely
the same cause; not traced. The row pins it at that exit.

## A third sighting, TRACED to the stored rows: a generic mirror (2026-11-23)

From EMBED's lane `embed/clip-vertex`, PR 7464's head `5b5ab29652`
(over `origin/embed/wave3-land` `8d34076a9e`), `f64`, a probe printing
every stored row of the body (`Body::pcurves`).

A ring revolved `τ − 10⁻³` about the y axis (profile
`[0.5, 1] × [0, 1]`; `near_full_ring` in
`crates/sweep/tests/self_intersection_proves_injectivity.rs`), then
`topo::mirror_body` across the plane through `(0.1, −0.2, 0.4)` with
normal `(1, 2, 3)`. Its four rim circles' rows on the cylinder walls:

| half-edge | as built | mirrored |
|---|---|---|
| `10v1` | `v: Free { c: 0, a: 0, b: 0, l: 0 }` | `v: Free { c: 0, a: 0, b: 1.3877787807814457e-17, l: 0 }` |
| `11v1` | `v: Free { c: 1, a: 0, b: 0, l: 0 }` | `v: Free { c: 1, a: 0, b: 1.3877787807814457e-17, l: 0 }` |
| `14v1` | `v: Free { c: 0, a: 0, b: 0, l: 0 }` | `v: Free { c: 0, a: 0, b: 2.7755575615628914e-17, l: 0 }` |
| `15v1` | `v: Free { c: −1, a: 0, b: 0, l: 0 }` | `v: Free { c: −1, a: 0, b: 2.7755575615628914e-17, l: 0 }` |

The same on a half rod united with a block (`half_rod_on_a_block`)
mirrored: its two rim half-circles store `a: 2.7755575615628914e-17`.
So the mirror's re-mint (`mirror_body` clears the rows and mints them
again on the mirrored frame; a rigid move carries them through) is a
second door into this row's class, with no outline built off the axes.

**What it costs there**, traced before on the lane's head `5421491599`:
each such rim use reads `iso = None, harmonic = Some`, `present_chart`
hands a cell with that circle in reach to `curved_walls`, and
`curved_walls` answers `Unpresented::Local` wherever the circle has a
second edge or a vertex beside it (47 exits on the block's two wall
pairs, 292 on the ring's refusing pair). On the landing branch the
mirrored ring reads `Uncertified { 1v1, 3v1, Edge(9v1), Departure }`,
a plane end face against the cylinder wall; the mirrored block is
embedded at `f64` (its `f64` blocker was the cover's joint, PR 7322).

**Controls**: a unit rod, a full ring, a 3-radian ring and a unit brick
hold mirrored across the same plane (a lone circle is read by
`present_curved`); every body above holds turned about a generic axis,
moved a hundred units, and mirrored across an axis plane.

**Not this row's, and not traced**: the same ring also refuses at
`Interval` (`Uncertified { Departure }`, the same pair), and the
mirrored block refuses at `Interval` (`Budget`) where it holds at
`f64`. This row's cause is an `f64` point off zero; what the `Interval`
rows store there was not printed.

Pinned as it reads:
`self_intersection_proves_injectivity::the_ring_and_the_interval_block_refuse_mirrored_across_a_generic_plane`.
